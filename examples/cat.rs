// SPDX-FileCopyrightText: 2022 Kevin Matthes
// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! An example of concatenating files and print on the standard output. The
//! contents of the file must be valid UTF-8.

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    process::ExitCode,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Opt {
    /// Files to print.
    ///
    /// If [FILE] is not specified, data will be read from standard input.
    #[arg(value_name("FILE"))]
    pub input: Option<Vec<PathBuf>>,
}

fn main() -> ExitCode {
    let opt = Opt::parse();

    #[allow(clippy::option_if_let_else)]
    let contents: io::Result<Vec<_>> = if let Some(files) = opt.input {
        files.into_iter().map(fs::read_to_string).collect()
    } else {
        let mut buf = String::new();
        vec![io::stdin().read_to_string(&mut buf).map(|_| buf)]
            .into_iter()
            .collect()
    };
    let contents = match contents {
        Ok(strings) => strings,
        Err(err) => {
            eprintln!("Error: {err}");
            return sysexits::ExitCode::from(err).into();
        }
    };

    for output in contents {
        print!("{output}");
    }
    ExitCode::SUCCESS
}
