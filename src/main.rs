//!
//! The Interdiff binary.
//!

use std::{io, fs};

use patch_rs::{Patch, PatchError, PatchProcessor};

use interdiff_rs::*;

#[derive(Debug)]
enum Error {
    Reading(io::Error),
    Patch(PatchError),
}

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
        .get_matches();

    let patch_1 = args.value_of("patch_1").expect("Unreachable");
    let patch_1 = fs::read_to_string(patch_1).map_err(Error::Reading)?;
    let patch_1 = PatchProcessor::convert(&patch_1).map_err(Error::Patch)?;

    let patch_2 = args.value_of("patch_2").expect("Unreachable");
    let patch_2 = fs::read_to_string(patch_2).map_err(Error::Reading)?;
    let patch_2 = PatchProcessor::convert(&patch_2).map_err(Error::Patch)?;

    let _interdiff = Patch {
        input: patch_1.output.to_owned(),
        output: patch_2.output.to_owned(),
        contexts: Vec::new(),
    };

    for context in patch_1.contexts {
        println!(
            "{}",
            context,
        );
        for flipped in flip(&context).iter() {
            println!(
                "{}",
                flipped,
            );
        }
    }
//    for context in patch_2.contexts {
//        println!(
//            "{}",
//            context
//        );
//    }

    Ok(())
}
