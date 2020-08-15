use std::error::Error;

use clap::Clap;

use gitrs::{hash_object, init, log, cat_file};

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Init,
    Log(Log),
    CatFile(CatFile),
    HashObject(HashObject),
}

#[derive(Clap)]
struct Log {
    hash: String,
}

#[derive(Clap)]
struct CatFile {
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
        SubCommand::Log(args) => log(args.hash),
        SubCommand::CatFile(args) => cat_file(args.kind, args.object),
        SubCommand::HashObject(args) => hash_object(args.kind, args.file, args.write),
    }
}
