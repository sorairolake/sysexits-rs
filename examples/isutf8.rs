//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! An example of checking whether the input is valid UTF-8.
//! The input is a file or the standard input.

use std::io::Read;

fn main() -> sysexits::ExitCode {
    let input = if let Some(file) = std::env::args().skip(1).next() {
        std::fs::read(file).unwrap()
    } else {
        let mut buf = Vec::new();

        std::io::stdin().read_to_end(&mut buf).unwrap();

        buf
    };

    std::str::from_utf8(&input).map_or(sysexits::ExitCode::DataErr, |_| sysexits::ExitCode::Ok)
}
