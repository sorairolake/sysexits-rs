//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! An example of concatenating files and print on the standard output.
//! The contents of the file must be valid UTF-8.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[cfg(feature = "std")]
use std::io::Read;

#[cfg(feature = "std")]
fn main() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).collect();

    let contents: std::io::Result<Vec<_>> = if args.is_empty() {
        let mut buf = String::new();
        vec![std::io::stdin().read_to_string(&mut buf).map(|_| buf)]
            .into_iter()
            .collect()
    } else {
        args.into_iter().map(std::fs::read_to_string).collect()
    };
    let contents = match contents {
        Ok(strings) => strings,
        Err(err) => {
            eprintln!("Error: {err}");
            return sysexits::ExitCode::try_from(err.kind()).map_or(
                std::process::ExitCode::FAILURE,
                std::process::ExitCode::from,
            );
        }
    };

    for output in contents {
        print!("{output}");
    }
    std::process::ExitCode::SUCCESS
}

#[cfg(not(feature = "std"))]
fn main() -> Result<(), &'static str> {
    Err("`std` feature is required")
}
