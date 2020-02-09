pub mod electron;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct OptWithCmd {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Command {
    Electron(electron::OptWithCmd),
}

fn main() {
    OptWithCmd::from_args().run_cmd().unwrap();
}

impl OptWithCmd {
    pub fn run_cmd(self) -> Result<()> {
        match self.cmd {
            Command::Electron(opt) => opt.run_cmd(),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unhandled(String),
}
