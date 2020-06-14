use crate::window::integration::scene::Scene;
use {
    crate::window::{integration::proxy::Proxy, WindowEvent},
    iced::{Command, Element, Subscription},
    iced_futures::Executor,
    iced_winit::{Color, Runtime},
    std::fmt::Debug,
    winit::event_loop::{EventLoop, EventLoopProxy},
};

use iced_wgpu::{wgpu, window::SwapChain, Primitive, Renderer, Settings, Target};
use iced_winit::{futures, mouse, winit, Cache, Clipboard, Size, UserInterface};

use objc::{msg_send, sel, sel_impl};
use winit::{
    event::{Event, ModifiersState, WindowEvent as WinitWindowEvent},
    event_loop::ControlFlow,
};

pub trait Application: iced::Application {
    fn run(flags: Self::Flags) -> !
    where
        Self: 'static,
    {
        log::error!("custom run");
        Self::run_with_window_events(flags, |_| None)
    }
    fn run_with_window_events<F>(flags: Self::Flags, window_event_map: F) -> !
    where
        Self: 'static,
        F: Fn(&Self::Message) -> Option<WindowEvent> + 'static,
    {
        // Initialize winit
        // let event_loop = EventLoop::with_user_event();
        let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        let mut logical_size = window.inner_size().to_logical(window.scale_factor());
        let mut modifiers = ModifiersState::default();

        let mut runtime = {
            let executor = <Self as iced::Application>::Executor::new().expect("Create executor");
            Runtime::new(executor, Proxy::new(event_loop.create_proxy()))
        };

        let (mut application, init_command) =
            runtime.enter(|| <Self as iced::Application>::new(flags));
        runtime.spawn(init_command);

        let subscription = application.subscription();
        runtime.track(subscription);

        // Initialize WGPU

        let surface = wgpu::Surface::create(&window);
        let (mut device, queue) = futures::executor::block_on(async {
            let adapter = wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .expect("Request adapter");

            adapter
                .request_device(&wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                })
                .await
        });

        let format = wgpu::TextureFormat::Bgra8UnormSrgb;

        let mut swap_chain = {
            let size = window.inner_size();

            SwapChain::new(&device, &surface, format, size.width, size.height)
        };
        let mut resized = false;

        // Initialize iced
        let mut events = Vec::new();
        let mut external_messages = Vec::new();
        let mut cache = Some(Cache::default());
        let mut renderer = Renderer::new(&mut device, Settings::default());
        let mut output = (Primitive::None, mouse::Interaction::default());
        let clipboard = Clipboard::new(&window);

        // Initialize scene and GUI controls
        let mut scene = Scene::new(&device);

        // Run event loop
        event_loop.run(move |event, _, control_flow| {
            // You should change this if you want to render continuosly
            *control_flow = ControlFlow::Wait;

            match event {
                Event::DeviceEvent { .. } => {
                    // TODO: use device events to provide background keybinds on non-mac OSs.
                }
                Event::WindowEvent { event, .. } => {
                    match event {
                        WinitWindowEvent::ModifiersChanged(new_modifiers) => {
                            modifiers = new_modifiers;
                        }
                        WinitWindowEvent::Resized(new_size) => {
                            logical_size = new_size.to_logical(window.scale_factor());
                            resized = true;
                        }
                        WinitWindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    }

                    // Map window event to iced event
                    if let Some(event) = iced_winit::conversion::window_event(
                        &event,
                        window.scale_factor(),
                        modifiers,
                    ) {
                        events.push(event);
                    }
                }
                Event::MainEventsCleared => {
                    // If no relevant events happened, we can simply skip this
                    if events.is_empty() && external_messages.is_empty() {
                        return;
                    }

                    // We need to:
                    // 1. Process events of our user interface.
                    // 2. Update state as a result of any interaction.
                    // 3. Generate a new output for our renderer.

                    // First, we build our user interface.
                    let mut user_interface = UserInterface::build(
                        application.view(),
                        Size::new(logical_size.width, logical_size.height),
                        cache.take().unwrap(),
                        &mut renderer,
                    );

                    // Then, we process the events, obtaining messages in return.
                    let mut messages = user_interface.update(
                        events.drain(..),
                        clipboard.as_ref().map(|c| c as _),
                        &renderer,
                    );
                    // append any externally generated messages
                    messages.append(&mut external_messages);

                    let user_interface = if messages.is_empty() {
                        // If there are no messages, no interactions we care about have
                        // happened. We can simply leave our user interface as it is.
                        user_interface
                    } else {
                        // If there are messages, we need to update our state
                        // accordingly and rebuild our user interface.
                        // We can only do this if we drop our user interface first
                        // by turning it into its cache.
                        cache = Some(user_interface.into_cache());

                        // In this example, `Controls` is the only part that cares
                        // about messages, so updating our state is pretty
                        // straightforward.
                        for message in messages {
                            // match message_map(&message) {
                            //     Some(WindowEvent::HiddenOrFocused(hidden)) => {
                            //         // use ::winit::platform::macos::WindowExtMacOS;
                            //         // dbg!("event loop window target?", visible);
                            //         // if visible {
                            //         //     window.show_application();
                            //         // } else {
                            //         //     window.hide_application();
                            //         // }
                            //         let cls = objc::runtime::Class::get("NSApplication").unwrap();
                            //         let app: cocoa::base::id =
                            //             unsafe { msg_send![cls, sharedApplication] };
                            //         let is_active: bool = unsafe { msg_send![app, isActive] };
                            //         if is_active {
                            //             dbg!("hiding");
                            //             let cls =
                            //                 objc::runtime::Class::get("NSApplication").unwrap();
                            //             let app: cocoa::base::id =
                            //                 unsafe { msg_send![cls, sharedApplication] };
                            //             unsafe { msg_send![app, hide: 0] }
                            //         } else {
                            //             dbg!("making active");
                            //             let cls =
                            //                 objc::runtime::Class::get("NSApplication").unwrap();
                            //             let app: cocoa::base::id =
                            //                 unsafe { msg_send![cls, sharedApplication] };
                            //             let ignore_other_apps = true;
                            //             unsafe {
                            //                 msg_send![
                            //                     app,
                            //                     activateIgnoringOtherApps: ignore_other_apps
                            //                 ]
                            //             }
                            //         }
                            //     }
                            //     Some(super::WindowEvent::Title(title)) => {
                            //         window.set_title(title.as_str());
                            //     }
                            //     Some(super::WindowEvent::BackgroundColor(color)) => {
                            //         scene.background_color = color;
                            //     }
                            //     None => {}
                            // }
                            application.update(message);
                        }

                        // Once the state has been changed, we rebuild our updated
                        // user interface.
                        UserInterface::build(
                            application.view(),
                            Size::new(logical_size.width, logical_size.height),
                            cache.take().unwrap(),
                            &mut renderer,
                        )
                    };

                    // Finally, we just need to draw a new output for our renderer,
                    output = user_interface.draw(&mut renderer);

                    // update our cache,
                    cache = Some(user_interface.into_cache());

                    // and request a redraw
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    if resized {
                        let size = window.inner_size();

                        swap_chain =
                            SwapChain::new(&device, &surface, format, size.width, size.height);
                    }

                    let (frame, viewport) = swap_chain.next_frame().expect("Next frame");

                    let mut encoder = device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                    // We draw the scene first
                    scene.draw(&mut encoder, &frame.view);

                    // And then iced on top
                    let mouse_interaction = renderer.draw(
                        &mut device,
                        &mut encoder,
                        Target {
                            texture: &frame.view,
                            viewport,
                        },
                        &output,
                        window.scale_factor(),
                        &["Some debug information!"],
                    );

                    // Then we submit the work
                    queue.submit(&[encoder.finish()]);

                    // And update the mouse cursor
                    window.set_cursor_icon(iced_winit::conversion::mouse_interaction(
                        mouse_interaction,
                    ));
                }
                _ => {}
            }
        })
    }
}

pub struct Instance<A: iced::Application>(A);
impl<A> Application for Instance<A> where A: iced::Application {}
impl<A> iced::Application for Instance<A>
where
    A: iced::Application,
{
    type Executor = A::Executor;
    type Flags = A::Flags;
    type Message = A::Message;
    fn new(flags: Self::Flags) -> (Self, Command<A::Message>) {
        let (app, command) = A::new(flags);
        (Instance(app), command)
    }

    fn title(&self) -> String {
        self.0.title()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.0.update(message)
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.0.subscription()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        self.0.view()
    }
}
