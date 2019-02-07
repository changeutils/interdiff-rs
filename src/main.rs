//!
//! The Interdiff binary.
//!

use std::{io, fs, num};

use patch_rs::{Patch, PatchError, PatchProcessor};

use interdiff_rs::*;

#[derive(Debug)]
enum Error {
    Reading(io::Error),
    ContextRadius(num::ParseIntError),
    BothEmpty,
    Patch(PatchError),
}

#[cfg(target_os = "windows")]
const EMPTY_PATCH: &str = "nul";
#[cfg(target_os = "linux")]
const EMPTY_PATCH: &str = "/dev/null";

type InterdiffResult = Result<(), Error>;

fn main() -> InterdiffResult {
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();

    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("patch_1")
                .help("The first patch")
                .index(1)
                .value_name("PATH")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("patch_2")
                .help("The second patch")
                .index(2)
                .value_name("PATH")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("context_radius")
                .help("The unidiff context radius")
                .short("U")
                .long("context")
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("3"))
        .get_matches();

    let patch_1 = args.value_of("patch_1").expect("Unreachable");
    let patch_2 = args.value_of("patch_2").expect("Unreachable");
    let context_radius = args.value_of("context_radius").expect("Unreachable");

    let context_radius: usize = context_radius.parse().map_err(Error::ContextRadius)?;

    if patch_1 == EMPTY_PATCH && patch_2 == EMPTY_PATCH {
        return Err(Error::BothEmpty);
    }

    let patch_1 = if patch_1 != EMPTY_PATCH {
        let patch_1 = fs::read_to_string(patch_1).map_err(Error::Reading)?;
        PatchProcessor::convert(&patch_1).map_err(Error::Patch)?
    } else {
        Patch::default()
    };

    let patch_2 = if patch_2 != EMPTY_PATCH {
        let patch_2 = fs::read_to_string(patch_2).map_err(Error::Reading)?;
        PatchProcessor::convert(&patch_2).map_err(Error::Patch)?
    } else {
        Patch::default()
    };

    println!("{}", interdiff(patch_1, patch_2, context_radius));

    Ok(())
}
