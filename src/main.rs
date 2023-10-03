use anyhow::Result;
use clap::Parser;
use passthrough::Passthrough;
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Args {
    /// The path of filesystem's mount
    #[clap(short, long, default_value = "/tmp/fsmnt")]
    mount: PathBuf,

    /// The directory to pass VFS calls through to
    #[clap(short, long, default_value = "/tmp/fsdata")]
    passthrough: PathBuf,

    /// Run filesystem in debug mode
    #[clap(short, long, default_value_t = false)]
    debug: bool,

    /// Run filesystem in foreground
    #[clap(short, long, default_value_t = false)]
    foreground: bool,

    /// Run filesystem in multithreaded mode
    #[clap(short, long, default_value_t = false)]
    multithreaded: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if fs::metadata(&args.mount).is_err() {
        fs::create_dir_all(&args.mount)?;
    }

    if fs::metadata(&args.passthrough).is_err() {
        fs::create_dir_all(&args.passthrough)?;
    }

    Passthrough::options()
        .debug(args.debug)
        .foreground(args.foreground)
        .multithreaded(args.multithreaded)
        .mount(args.mount, args.passthrough)
}
