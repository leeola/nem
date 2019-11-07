use {
  std::path::Path,
  std::{env, fs},
  wasm_pack,
};

fn main() {
  if cfg!(feature = "pwa-assets") {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pwa_dst_path = Path::new(&out_dir).join("pwa_build");
    let server_crate_dir = env::current_dir().unwrap();
    let pwa_crate_dir = server_crate_dir.join("..").join("pwa");

    println!("cargo:rustc-env=PWA_DIR={}", pwa_dst_path.to_str().unwrap());

    ["index.html", "manifest.json", "sw.js"]
      .iter()
      .for_each(|file| {
        fs::copy(pwa_crate_dir.join(file), pwa_dst_path.join(file)).unwrap();
      });

    use wasm_pack::command::{
      build::{BuildMode, BuildOptions, Target},
      Command,
    };

    println!("building PWA WASM assets..");
    wasm_pack::command::run_wasm_pack(Command::Build(BuildOptions {
      path: Some(pwa_crate_dir),
      scope: None,
      mode: BuildMode::Normal,
      disable_dts: true,
      target: Target::Web,
      // deprecated
      debug: false,
      // TODO: tie dev to !--release?
      dev: false,
      // TODO: tie dev to --release?
      release: true,
      profiling: false,
      out_dir: pwa_dst_path.to_str().unwrap().to_owned(),
      out_name: Some("index".to_owned()),
      extra_options: vec![],
    }))
    .unwrap();
  }
}
