// SPDX-FileCopyrightText: 2022 Kevin Matthes
// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of concatenating files and print on the standard output. The
//! contents of the file must be valid UTF-8.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
    env, fs,
    io::{self, Read},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();

    let contents: io::Result<Vec<_>> = if args.is_empty() {
        let mut buf = String::new();
        vec![io::stdin().read_to_string(&mut buf).map(|_| buf)]
            .into_iter()
            .collect()
    } else {
        args.into_iter().map(fs::read_to_string).collect()
    };
    let contents = match contents {
        Ok(strings) => strings,
        Err(err) => {
            eprintln!("Error: {err}");
            return sysexits::ExitCode::from(err).into();
        }
    };

    for output in contents {
        print!("{output}");
    }
    ExitCode::SUCCESS
}
