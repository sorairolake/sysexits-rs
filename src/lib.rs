//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

//! The `sysexits` crate provides the system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! The [`SysExits`] type implements the
//! [`Termination`](std::process::Termination) trait, so this can be returned
//! from the `main` function.
//!
//! # Examples
//!
//! Return the [`SysExits`] from the `main` function:
//!
//! ```no_run
//! # use std::io::{self, Read};
//! #
//! use sysexits::SysExits;
//!
//! fn main() -> SysExits {
//!     let mut buf = String::new();
//!
//!     match io::stdin().read_to_string(&mut buf) {
//!         Ok(_) => {
//!             print!("{buf}");
//!
//!             SysExits::Ok
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!
//!             SysExits::DataErr
//!         }
//!     }
//! }
//! ```
//!
//! Return the [`ExitCode`](std::process::ExitCode) from the `main` function:
//!
//! ```no_run
//! # use std::fs;
//! # use std::io;
//! # use std::process::ExitCode;
//! #
//! use sysexits::SysExits;
//!
//! fn main() -> ExitCode {
//! #     let path = "/path/to/file.txt";
//! #
//!     match fs::read_to_string(path) {
//!         Ok(contents) => {
//!             print!("{contents}");
//!
//!             ExitCode::SUCCESS
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!
//!             match err.kind() {
//!                 io::ErrorKind::NotFound => SysExits::NoInput.into(),
//!                 io::ErrorKind::PermissionDenied => SysExits::NoPerm.into(),
//!                 io::ErrorKind::InvalidData => SysExits::DataErr.into(),
//!                 _ => ExitCode::FAILURE,
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

#![doc(html_root_url = "https://docs.rs/sysexits/0.1.0/")]
// Lint levels of rustc.
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

mod sysexits;

pub use crate::sysexits::SysExits;
