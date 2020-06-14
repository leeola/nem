use {
    iced::{
        futures::{
            self,
            stream::{self, BoxStream, StreamExt},
        },
        Subscription,
    },
    iced_native::subscription::Recipe,
    std::{
        any::TypeId,
        hash::{Hash, Hasher},
    },
};

#[derive(Hash)]
pub struct GlobalHotkey {
    id: usize,
}

impl GlobalHotkey {
    pub fn new() -> Self {
        Self::with_id(0)
    }
    pub fn with_id(id: usize) -> Self {
        Self { id }
    }
    pub fn subscription() -> Subscription<Event> {
        Subscription::from_recipe(Self::new())
    }
}

impl<H, I> Recipe<H, I> for GlobalHotkey
where
    H: Hasher,
{
    type Output = Event;

    fn hash(&self, state: &mut H) {
        TypeId::of::<Self>().hash(state);
        Hash::hash(self, state);
    }
    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        Box::pin(stream::unfold(None, |state| async move {
            log::error!("foo");
            let state = state.unwrap_or_else(State::new);
            let event = loop {
                if let Some(event) = state.maybe_event() {
                    break event;
                }
                // yield the future for this iteration.
                futures::future::pending::<()>().await;
            };
            Some((event, Some(state)))
        }))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    A,
    B,
    C,
}
pub struct State {}
impl State {
    pub fn new() -> Self {
        Self {}
    }
    pub fn maybe_event(&self) -> Option<Event> {
        Some(Event::A)
    }
}
