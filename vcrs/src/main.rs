use std::error::Error;

use clap::Clap;

use gitrs::{init, show};

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init,
    Show(Show),
}

#[derive(Clap)]
struct Show {
    hash: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init => init(),
        SubCommand::Show(hash) => show(hash.hash),
    }
}
