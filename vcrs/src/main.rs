use std::error::Error;

use clap::Clap;

use gitrs::{hash_object, init, show};

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init,
    Show(Show),
    HashObject(HashObject),
}

#[derive(Clap)]
struct Show {
    kind: String,
    object: String,
}

#[derive(Clap)]
struct HashObject {
    kind: String,
    file: String,
    #[clap(short, takes_value = false)]
    write: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Init => init(),
        SubCommand::Show(args) => show(args.kind, args.object),
        SubCommand::HashObject(args) => hash_object(args.kind, args.file, args.write),
    }
}
