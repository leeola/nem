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

    fs::create_dir_all(&pwa_dst_path).unwrap();

    println!("cargo:rustc-env=PWA_DIR={}", pwa_dst_path.to_str().unwrap());

    // rerun the build if any pwa related files change.
    [
      "../pwa",
      "../gui",
      // TODO: the above dirs are not reloading properly. Fix this..
      "../gui/src/lib.rs",
      "../pwa/index.html",
      "../pwa/manifest.json",
      "../pwa/sw.js",
    ]
    .iter()
    .for_each(|path| {
      println!("cargo:rerun-if-changed={}", path);
    });

    ["index.html", "manifest.json"].iter().for_each(|file| {
      fs::copy(pwa_crate_dir.join(file), pwa_dst_path.join(file)).unwrap();
    });

    // TODO: implement a better ident. Timestamp is just simple, but ideally it would be
    // a content hash of some files. At this moment in time though i didn't want to hash
    // all of the content in multiple crates :s. Perhaps i could base it off of cargo's
    // identifier for the crates? Eg, the hash suffix on built packages.
    {
      use std::{
        io::{Read, Write},
        time::SystemTime,
      };
      let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
      let mut src = fs::File::open(pwa_crate_dir.join("sw.js")).unwrap();
      let mut contents = String::new();
      src.read_to_string(&mut contents).unwrap();
      let contents = contents.replace("%%CACHE_IDENT%%", &format!("{}", now));
      let mut dst = fs::File::create(pwa_dst_path.join("sw.js")).unwrap();
      dst.write_all(contents.as_bytes()).unwrap();
    }

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
