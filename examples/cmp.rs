// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of comparing two files byte by byte.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::{
    fs, io,
    path::PathBuf,
    process::{self, Termination},
};

use clap::Parser;

enum ExitCode {
    Same,
    Different,
    Other(sysexits::ExitCode),
}

impl From<sysexits::ExitCode> for ExitCode {
    fn from(code: sysexits::ExitCode) -> Self {
        Self::Other(code)
    }
}

impl Termination for ExitCode {
    fn report(self) -> process::ExitCode {
        match self {
            Self::Same => process::ExitCode::SUCCESS,
            Self::Different => process::ExitCode::FAILURE,
            Self::Other(code) => process::ExitCode::from(code),
        }
    }
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// File to compare.
    #[arg(value_name("FILE1"))]
    pub input_1: PathBuf,

    /// File to compare.
    #[arg(value_name("FILE2"))]
    pub input_2: PathBuf,
}

fn main() -> ExitCode {
    let opt = Opt::parse();

    let files = [opt.input_1, opt.input_2];
    let contents: io::Result<Vec<_>> = files.iter().map(fs::read).collect();
    let contents = match contents {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Error: {err}");
            return sysexits::ExitCode::from(err).into();
        }
    };

    if contents[0] == contents[1] {
        println!(
            "Files {} and {} are identical",
            files[0].display(),
            files[1].display()
        );
        ExitCode::Same
    } else {
        println!(
            "Files {} and {} are different",
            files[0].display(),
            files[1].display()
        );
        ExitCode::Different
    }
}
