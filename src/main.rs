use clap::Parser;
use fuse_sys::prelude::*;
use passthru::Passthru;
use std::{env, fs, io::ErrorKind};

#[derive(Parser)]
struct Args {
    /// The path of filesystem's mount
    #[clap(short, long, default_value = "/tmp/fsmnt")]
    mount: String,
    /// The directory that backs mount
    #[clap(short = 'a', long, default_value = "/tmp/fsdata")]
    data: String,
    /// Whether or not to run fuse in debug mode
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let bin = env::args().next().unwrap();
    let Args { mount, data, debug } = Args::parse();

    let mut fuse_args = vec![bin.as_str(), mount.as_str(), "-f", "-s"];
    if debug {
        fuse_args.push("-d");
    }

    match fs::read_dir(&mount) {
        Err(e) if e.kind() == ErrorKind::NotFound => fs::create_dir(&mount).unwrap(),
        r => {
            r.unwrap();
        }
    }
    match fs::read_dir(&data) {
        Err(e) if e.kind() == ErrorKind::NotFound => fs::create_dir(&data).unwrap(),
        r => {
            r.unwrap();
        }
    }

    println!("Mounting {mount} as mirror of {data}...");
    Passthru::new().run(&fuse_args).unwrap();
}
