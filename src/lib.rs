//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

//! The `sysexits` crate provides the system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! The [`ExitCode`] type implements the
//! [`Termination`](std::process::Termination) trait, so this can be returned
//! from the `main` function.
//!
//! # Examples
//!
//! Return the [`ExitCode`] from the `main` function:
//!
//! ```no_run
//! use std::io::{self, Read};
//!
//! use sysexits::ExitCode;
//!
//! fn main() -> ExitCode {
//!     let mut buf = String::new();
//!
//!     match io::stdin().read_to_string(&mut buf) {
//!         Ok(_) => {
//!             print!("{buf}");
//!
//!             ExitCode::Ok
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!
//!             ExitCode::DataErr
//!         }
//!     }
//! }
//! ```
//!
//! Return the [`std::process::ExitCode`] from the `main` function:
//!
//! ```no_run
//! use std::fs;
//! use std::io;
//! use std::process::ExitCode as StdExitCode;
//!
//! use sysexits::ExitCode;
//!
//! fn main() -> StdExitCode {
//!     let path = "/path/to/file.txt";
//!
//!     match fs::read_to_string(path) {
//!         Ok(contents) => {
//!             print!("{contents}");
//!
//!             StdExitCode::SUCCESS
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!
//!             match err.kind() {
//!                 io::ErrorKind::NotFound => ExitCode::NoInput.into(),
//!                 io::ErrorKind::PermissionDenied => ExitCode::NoPerm.into(),
//!                 io::ErrorKind::InvalidData => ExitCode::DataErr.into(),
//!                 _ => StdExitCode::FAILURE,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

#![doc(html_root_url = "https://docs.rs/sysexits/0.1.1/")]
// Lint levels of rustc.
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod exit_code;

pub use crate::exit_code::ExitCode;
