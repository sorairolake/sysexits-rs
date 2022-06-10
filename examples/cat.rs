//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! An example of concatenating files and print on the standard output.
//! The contents of the file must be valid UTF-8.

use std::io::Read;

fn main() -> std::process::ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();

    let contents = if args.is_empty() {
        let mut buf = String::new();

        if let Err(err) = std::io::stdin().read_to_string(&mut buf) {
            eprintln!("{err}");

            return sysexits::ExitCode::DataErr.into();
        } else {
            vec![buf]
        }
    } else {
        match args.into_iter().map(std::fs::read_to_string).collect() {
            Ok(strings) => strings,
            Err(err) => {
                eprintln!("{err}");

                match err.kind() {
                    std::io::ErrorKind::NotFound => return sysexits::ExitCode::NoInput.into(),
                    std::io::ErrorKind::PermissionDenied => {
                        return sysexits::ExitCode::NoPerm.into()
                    }
                    std::io::ErrorKind::InvalidData => return sysexits::ExitCode::DataErr.into(),
                    _ => return std::process::ExitCode::FAILURE,
                }
            }
        }
    };

    for output in contents {
        print!("{output}");
    }

    std::process::ExitCode::SUCCESS
}
