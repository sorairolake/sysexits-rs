// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of checking whether the input is valid UTF-8. The input is a file
//! or the standard input.

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    str,
};

use clap::Parser;
use sysexits::ExitCode;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// File to check.
    ///
    /// If [FILE] is not specified, data will be read from standard input.
    #[arg(value_name("FILE"))]
    input: Option<PathBuf>,
}

fn main() -> ExitCode {
    let opt = Opt::parse();

    let input = if let Some(file) = opt.input {
        match fs::read(file) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error: {err}");
                return ExitCode::from(err);
            }
        }
    } else {
        let mut buf = Vec::new();
        if let Err(err) = io::stdin().read_to_end(&mut buf) {
            eprintln!("Error: {err}");
            return ExitCode::from(err);
        }
        buf
    };
    if let Err(err) = str::from_utf8(&input) {
        eprintln!("Error: {err}");
        ExitCode::DataErr
    } else {
        println!("OK");
        ExitCode::Ok
    }
}
