use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "nem")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}
#[derive(Debug, StructOpt)]
enum Command {
    Serve {},
}
#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    nem_server::run().await;
}
