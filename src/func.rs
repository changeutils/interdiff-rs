//!
//! Helper functions.
//!

use std::mem;

use patch_rs::{Context, ContextHeader, PatchLine};

enum FlipState {
    StartContext,
    Context,
    Buffering,
}

pub fn flip(input: &Context) -> Vec<Context> {
    let mut results = Vec::new();

    let mut output = Context::default();
    output.header.file1_l = input.header.file1_l;
    output.header.file2_l = input.header.file2_l;

    let mut state = FlipState::StartContext;
    let mut deletes = Vec::new();
    let mut inserts = Vec::new();

    println!("START");
    for line in input.data.iter() {
        match line {
            PatchLine::Context(_) => {
                match state {
                    FlipState::StartContext => {
                        println!("Context StartContext");
                        if output.ends_with_context_lines() >= 1 {
                            output.data.pop();
                            output.header.file1_l += 1;
                            output.header.file2_l += 1;
                            println!("POP START");
                        }
                        output.data.push(line.clone());
                    },
                    FlipState::Context => {
                        println!("Context Context");
                        output.data.push(line.clone());
                        let lines = output.ends_with_context_lines();
                        if lines > 2 {
                            let mut last = None;
                            for _ in 1..lines {
                                last = output.data.pop();
                                println!("POP END");
                            }
                            output.set_s_values();

                            println!("PUSH OUTPUT");
                            let output_next = Context {
                                header: ContextHeader {
                                    file1_l: output.header.file1_l + output.header.file1_s + lines - 2,
                                    file1_s: Default::default(),
                                    file2_l: output.header.file2_l + output.header.file2_s + lines - 2,
                                    file2_s: Default::default(),
                                },
                                data: vec![last.unwrap()],
                            };

                            results.push(output);
                            output = output_next;

                            state = FlipState::Context;
                        }
                    },
                    FlipState::Buffering => {
                        println!("Context Buffering");
                        output.data.append(&mut deletes);
                        output.data.append(&mut inserts);
                        state = FlipState::Context;
                        output.data.push(line.clone());
                    },
                }
            },
            PatchLine::Delete(_) => {
                if let FlipState::StartContext = state {
                    println!("Delete StartContext");
                    state = FlipState::Buffering;
                }
                if let FlipState::Context = state {
                    println!("Delete Context");
                    state = FlipState::Buffering;
                }
                if let FlipState::Buffering = state {
                    println!("Delete Buffering");
                    inserts.push(line.flip());
                }
            },
            PatchLine::Insert(_) => {
                println!("INSERT");
                if let FlipState::StartContext = state {
                    println!("Insert StartContext");
                    state = FlipState::Buffering;
                }
                if let FlipState::Context = state {
                    println!("Insert Context");
                    state = FlipState::Buffering;
                }
                if let FlipState::Buffering = state {
                    println!("Insert Buffering");
                    deletes.push(line.flip());
                }
            },
        }
    }

    let lines = output.ends_with_context_lines();
    if lines > 2 {
        let mut last = None;
        for _ in 1..lines {
            last = output.data.pop();
            println!("POP END");
        }
    }

    output.set_s_values();
    results.push(output);

    println!("END");

    results
}
