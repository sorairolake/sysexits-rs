//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! An example of checking whether the input is valid UTF-8.
//! The input is a file or the standard input.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[cfg(feature = "std")]
use std::io::Read;

#[cfg(feature = "std")]
fn main() -> sysexits::ExitCode {
    let input = std::env::args_os()
        .nth(1)
        .map_or_else(
            || {
                let mut buf = Vec::new();
                std::io::stdin().read_to_end(&mut buf).map(|_| buf)
            },
            std::fs::read,
        )
        .unwrap_or_else(|err| panic!("{err}"));
    if let Err(err) = std::str::from_utf8(&input) {
        eprintln!("Error: {err}");
        sysexits::ExitCode::DataErr
    } else {
        println!("OK");
        sysexits::ExitCode::Ok
    }
}

#[cfg(not(feature = "std"))]
fn main() -> Result<(), &'static str> {
    Err("`std` feature is required")
}
