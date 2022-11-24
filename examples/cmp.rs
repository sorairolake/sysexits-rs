//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! An example of comparing two files byte by byte.

// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

use std::process::Termination;

enum ExitCode {
    Same,
    Different,
    Trouble,
    Other(sysexits::ExitCode),
}

impl From<sysexits::ExitCode> for ExitCode {
    fn from(code: sysexits::ExitCode) -> Self {
        Self::Other(code)
    }
}

impl Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        match self {
            Self::Same => std::process::ExitCode::from(u8::MIN),
            Self::Different => std::process::ExitCode::from(1),
            Self::Trouble => std::process::ExitCode::from(2),
            Self::Other(code) => std::process::ExitCode::from(code),
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os().skip(1).take(2).collect();

    let files = if let (Some(from), Some(to)) = (args.get(0), args.get(1)) {
        (std::path::PathBuf::from(from), std::path::PathBuf::from(to))
    } else {
        eprintln!("Error: files are missing");
        return ExitCode::Trouble;
    };
    let contents: std::io::Result<Vec<_>> = args.into_iter().map(std::fs::read).collect();
    let contents = match contents {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Error: {err}");
            match err.kind() {
                std::io::ErrorKind::NotFound => return sysexits::ExitCode::NoInput.into(),
                std::io::ErrorKind::PermissionDenied => return sysexits::ExitCode::NoPerm.into(),
                std::io::ErrorKind::InvalidData => return sysexits::ExitCode::DataErr.into(),
                _ => return ExitCode::Trouble,
            }
        }
    };

    if contents[0] == contents[1] {
        println!(
            "Files {} and {} are identical",
            files.0.display(),
            files.1.display()
        );
        ExitCode::Same
    } else {
        println!(
            "Files {} and {} are different",
            files.0.display(),
            files.1.display()
        );
        ExitCode::Different
    }
}
