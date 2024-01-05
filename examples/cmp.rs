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

#[cfg(feature = "std")]
enum ExitCode {
    Same,
    Different,
    Trouble,
    Other(sysexits::ExitCode),
}

#[cfg(feature = "std")]
impl From<sysexits::ExitCode> for ExitCode {
    fn from(code: sysexits::ExitCode) -> Self {
        Self::Other(code)
    }
}

#[cfg(feature = "std")]
impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        use std::process::ExitCode;

        match self {
            Self::Same => ExitCode::from(u8::MIN),
            Self::Different => ExitCode::from(1),
            Self::Trouble => ExitCode::from(2),
            Self::Other(code) => ExitCode::from(code),
        }
    }
}

#[cfg(feature = "std")]
fn main() -> ExitCode {
    use std::{env, fs, io, path::PathBuf};

    let args: Vec<_> = env::args_os().skip(1).take(2).collect();

    let files = if let (Some(from), Some(to)) = (args.first(), args.get(1)) {
        (PathBuf::from(from), PathBuf::from(to))
    } else {
        eprintln!("Error: files are missing");
        return ExitCode::Trouble;
    };
    let contents: io::Result<Vec<_>> = args.into_iter().map(fs::read).collect();
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

#[cfg(not(feature = "std"))]
fn main() -> Result<(), &'static str> {
    Err("`std` feature is required")
}
