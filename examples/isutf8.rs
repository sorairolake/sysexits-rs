// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of checking whether the input is valid UTF-8. The input is a file
//! or the standard input.

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
    use std::{env, fs, io, str};

    use sysexits::ExitCode;

    let input = env::args_os()
        .nth(1)
        .map_or_else(
            || {
                let mut buf = Vec::new();
                io::stdin().read_to_end(&mut buf).map(|_| buf)
            },
            fs::read,
        )
        .unwrap_or_else(|err| panic!("{err}"));
    if let Err(err) = str::from_utf8(&input) {
        eprintln!("Error: {err}");
        ExitCode::DataErr
    } else {
        println!("OK");
        ExitCode::Ok
    }
}

#[cfg(not(feature = "std"))]
fn main() -> Result<(), &'static str> {
    Err("`std` feature is required")
}
