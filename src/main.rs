//!
//! The Interdiff binary.
//!

use std::{io, fs, num, collections::VecDeque};

use log::*;

use patch_rs::{Patch, PatchError, PatchProcessor};

#[derive(Debug)]
enum Error {
    Reading(io::Error),
    ContextRadius(num::ParseIntError),
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
        .arg(
            clap::Arg::with_name("context_radius")
                .help("The unidiff context radius")
                .short("c")
                .long("context")
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("3"))
        .get_matches();

    let patch_1 = args.value_of("patch_1").expect("Unreachable");
    let patch_2 = args.value_of("patch_2").expect("Unreachable");

    let patch_1 = fs::read_to_string(patch_1).map_err(Error::Reading)?;
    let mut patch_1 = PatchProcessor::convert(&patch_1).map_err(Error::Patch)?;

    let patch_2 = fs::read_to_string(patch_2).map_err(Error::Reading)?;
    let mut patch_2 = PatchProcessor::convert(&patch_2).map_err(Error::Patch)?;

    let context_radius = args.value_of("context_radius").expect("Unreachable");
    let _context_radius: usize = context_radius.parse().map_err(Error::ContextRadius)?;

//    let c1 = patch_1.contexts.front().unwrap();
//    let c2 = patch_2.contexts.front().unwrap();
//    let sum = c1.flip().pop_front().unwrap().merge(c2.reduce().pop_front().unwrap());
//    trace!("\n{}", sum);

//    let mut interdiff = Patch {
//        input: patch_1.output.to_owned(),
//        output: patch_2.output.to_owned(),
//        contexts: VecDeque::new(),
//    };
//    let mut patch_1_offset = 0;
//    let mut patch_2_offset = 0;
//
//    trace!("DRAINING BOTH PATCHES");
//    while !patch_1.contexts.is_empty() && !patch_2.contexts.is_empty() {
//        let p1 = patch_1.contexts.front().unwrap();
//        let p2 = patch_2.contexts.front().unwrap();
//        if p1.header.file1_l + (patch_1_offset as usize) <= p2.header.file1_l + (patch_2_offset as usize) {
//            let flipped = patch_1.contexts.pop_front().unwrap().flip();
//            for mut context in flipped.into_iter() {
////                patch_1_offset += context.offset();
////                trace!("PATCH 1 OFFSET: {}", patch_1_offset);
////                context.shift(patch_2_offset);
////                trace!("CONTEXT 1: {}", context);
//                interdiff.contexts.push_back(context);
//            }
//        } else {
//            let reduced = patch_2.contexts.pop_front().unwrap().reduce();
//            for mut context in reduced.into_iter() {
////                patch_2_offset += context.offset();
////                trace!("PATCH 2 OFFSET: {}", patch_2_offset);
////                context.shift(patch_1_offset);
////                trace!("CONTEXT 2: {}", context);
//                interdiff.contexts.push_back(context);
//            }
//        }
//    }
//    trace!("DRAINING FIRST PATCH");
//    while !patch_1.contexts.is_empty() {
//        let flipped = patch_1.contexts.pop_front().unwrap().flip();
//        for mut context in flipped.into_iter() {
////            patch_1_offset += context.offset();
////            trace!("PATCH 1 OFFSET: {}", patch_1_offset);
////            context.shift(patch_2_offset);
////            trace!("CONTEXT 1: {}", context);
//            interdiff.contexts.push_back(context);
//        }
//    }
//    trace!("DRAINING SECOND PATCH");
//    while !patch_2.contexts.is_empty() {
//        let reduced = patch_2.contexts.pop_front().unwrap().reduce();
//        for mut context in reduced.into_iter() {
////            patch_2_offset += context.offset();
////            trace!("PATCH 2 OFFSET: {}", patch_2_offset);
////            context.shift(patch_1_offset);
////            trace!("CONTEXT 2: {}", context);
//            interdiff.contexts.push_back(context);
//        }
//    }
//
//    trace!("{}", interdiff);

    Ok(())
}
