use {crate::Result, std::process, structopt::StructOpt, structopt_wasm_pack::StructOpt as _};

#[derive(StructOpt)]
pub struct OptWithCmd {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    Build(BuildOpt),
    Run(RunOpt),
}

impl OptWithCmd {
    pub fn run_cmd(self) -> Result<()> {
        match self.cmd {
            Command::Build(opt) => opt.run_cmd(),
            Command::Run(opt) => opt.run_cmd(),
        }
    }
}

#[derive(StructOpt)]
pub struct BuildOpt {}

impl BuildOpt {
    fn run_cmd(self) -> Result<()> {
        log::info!("building wasm..");
        let cli = wasm_pack::Cli::from_iter_safe(&[
            "wasm-pack",
            "build",
            "--target",
            "web",
            "gui_electron",
        ])
        .expect("wasm-pack build")
        .cmd;
        wasm_pack::command::run_wasm_pack(cli).expect("wasm-pack build");

        Ok(())
    }
}

#[derive(StructOpt)]
pub struct RunOpt {}

impl RunOpt {
    fn run_cmd(self) -> Result<()> {
        BuildOpt {}.run_cmd()?;

        // TODO: impl a simple check to auto install nodejs deps?
        // Command::new("yarn")
        //         .current_dir("electron_gui")
        //           .args(&["install"])
        //           .output()
        //           .expect("yarn install");

        log::info!("running electron..");
        process::Command::new("node_modules/.bin/electron")
            .current_dir("gui_electron")
            .args(&["."])
            .output()
            .expect("run electron");

        Ok(())
    }
}
