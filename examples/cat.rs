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

use std::io::Read;

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
            match err.kind() {
                std::io::ErrorKind::NotFound => return sysexits::ExitCode::NoInput.into(),
                std::io::ErrorKind::PermissionDenied => return sysexits::ExitCode::NoPerm.into(),
                std::io::ErrorKind::InvalidData => return sysexits::ExitCode::DataErr.into(),
                _ => return std::process::ExitCode::FAILURE,
            }
        }
    };

    for output in contents {
        print!("{output}");
    }
    std::process::ExitCode::SUCCESS
}
