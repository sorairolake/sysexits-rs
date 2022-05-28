//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

//! An example of concatenating files and print on the standard output.
//! The contents of the file must be valid UTF-8.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

use sysexits::SysExits;

fn main() -> ExitCode {
    let args: Vec<_> = env::args().skip(1).collect();

    let contents = if args.is_empty() {
        let mut buf = String::new();

        match io::stdin().read_to_string(&mut buf) {
            Ok(_) => vec![buf],
            Err(err) => {
                eprintln!("{err}");

                return SysExits::DataErr.into();
            }
        }
    } else {
        match args
            .into_iter()
            .map(|file| fs::read_to_string(file))
            .collect()
        {
            Ok(strings) => strings,
            Err(err) => {
                eprintln!("{err}");

                match err.kind() {
                    io::ErrorKind::NotFound => return SysExits::NoInput.into(),
                    io::ErrorKind::PermissionDenied => return SysExits::NoPerm.into(),
                    io::ErrorKind::InvalidData => return SysExits::DataErr.into(),
                    _ => return ExitCode::FAILURE,
                }
            }
        }
    };

    for output in contents {
        print!("{output}");
    }

    ExitCode::SUCCESS
}
