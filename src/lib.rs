//!
//! The Interdiff library.
//!

use std::collections::VecDeque;

use log::*;

use patch_rs::Patch;

pub fn interdiff(mut patch_1: Patch, mut patch_2: Patch, context_radius: usize) -> Patch {
    let mut interdiff = Patch {
        input: patch_1.output.to_owned(),
        output: patch_2.output.to_owned(),
        contexts: VecDeque::new(),
    };
    let mut patch_1_offset = 0;
    let mut patch_2_offset = 0;

    trace!("DRAINING BOTH PATCHES");
    while !patch_1.contexts.is_empty() && !patch_2.contexts.is_empty() {
        let p1 = patch_1.contexts.front().unwrap();
        let p2 = patch_2.contexts.front().unwrap();
        if p1.header.file1_l <= p2.header.file1_l {
            let mut context = patch_1.contexts.pop_front().unwrap();
            let flipped = context.flip(context_radius);
            for mut context in flipped.into_iter() {
                context.shift(patch_2_offset);
                patch_1_offset += context.offset();
                interdiff.contexts.push_back(context);
            }
        } else {
            let mut context = patch_2.contexts.pop_front().unwrap();
            let reduced = context.reduce(context_radius);
            for mut context in reduced.into_iter() {
                context.shift(-patch_1_offset);
                patch_2_offset += context.offset();
                interdiff.contexts.push_back(context);
            }
        }
    }
    trace!("DRAINING FIRST PATCH");
    while !patch_1.contexts.is_empty() {
        let mut context = patch_1.contexts.pop_front().unwrap();
        let flipped = context.flip(context_radius);
        for mut context in flipped.into_iter() {
            context.shift(patch_2_offset);
            patch_1_offset += context.offset();
            interdiff.contexts.push_back(context);
        }
    }
    trace!("DRAINING SECOND PATCH");
    while !patch_2.contexts.is_empty() {
        let mut context = patch_2.contexts.pop_front().unwrap();
        let reduced = context.reduce(context_radius);
        for mut context in reduced.into_iter() {
            context.shift(-patch_1_offset);
            patch_2_offset += context.offset();
            interdiff.contexts.push_back(context);
        }
    }

    interdiff
}
