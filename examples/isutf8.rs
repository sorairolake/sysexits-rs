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

use std::io::Read;

fn main() -> sysexits::ExitCode {
    #[allow(clippy::option_if_let_else)]
    let input = if let Some(file) = std::env::args().nth(1) {
        std::fs::read(file).unwrap()
    } else {
        let mut buf = Vec::new();
        std::io::stdin().read_to_end(&mut buf).unwrap();
        buf
    };
    std::str::from_utf8(&input).map_or(sysexits::ExitCode::DataErr, |_| sysexits::ExitCode::Ok)
}
