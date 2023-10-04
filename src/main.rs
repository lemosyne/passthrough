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
    #[clap(short = 't', long, default_value_t = false)]
    multithreaded: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let _ = fs::create_dir_all(&args.mount);
    let _ = fs::create_dir_all(&args.passthrough);

    pretty_env_logger::init();

    Passthrough::options()
        .debug(args.debug)
        .foreground(args.foreground)
        .multithreaded(args.multithreaded)
        .build(args.passthrough)
        .mount(args.mount)
}
