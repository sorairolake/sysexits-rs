// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2023 Kevin Matthes
// SPDX-FileCopyrightText: 2023 zSchoen
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The `sysexits` crate provides the system exit code constants as defined by
//! [`<sysexits.h>`].
//!
//! The [`ExitCode`] type implements the
//! [`Termination`](std::process::Termination) trait, so this can be returned
//! from the `main` function.
//!
//! # Examples
//!
//! ## Returns the exit code as defined by <sysexits.h>
//!
//! If you only use the exit code as defined by `<sysexits.h>`, you can return
//! this from the `main` function.
//!
//! ```
//! # #[cfg(feature = "std")]
//! use std::str;
//!
//! # #[cfg(feature = "std")]
//! use sysexits::ExitCode;
//!
//! # #[cfg(feature = "std")]
//! fn main() -> ExitCode {
//!     let bytes = [0xf0, 0x9f, 0x92, 0x96];
//!     match str::from_utf8(&bytes) {
//!         Ok(string) => {
//!             println!("{string}");
//!             ExitCode::Ok
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!             ExitCode::DataErr
//!         }
//!     }
//! }
//! #
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//!
//! ## Combine with other exit codes
//!
//! [`ExitCode`] can be converted to [`std::process::ExitCode`] by the [`From`]
//! trait, so you can combine it with your own exit codes or
//! [`std::process::ExitCode`].
//!
//! ```
//! # #[cfg(feature = "std")]
//! use std::{
//!     io::{self, Read},
//!     process::ExitCode,
//! };
//!
//! # #[cfg(feature = "std")]
//! fn main() -> ExitCode {
//!     let mut buf = String::new();
//!     if let Err(err) = io::stdin().read_to_string(&mut buf) {
//!         eprintln!("{err}");
//!         sysexits::ExitCode::from(err).into()
//!     } else {
//!         print!("{buf}");
//!         ExitCode::SUCCESS
//!     }
//! }
//! #
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//!
//! [`<sysexits.h>`]: https://man.openbsd.org/sysexits

#![cfg_attr(feature = "extended_io_error", feature(io_error_more))]
#![doc(html_root_url = "https://docs.rs/sysexits/0.7.9/")]
#![no_std]
#![cfg_attr(doc_cfg, feature(doc_auto_cfg, doc_cfg))]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, missing_docs)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[cfg(test)]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod error;
mod exit_code;

pub use crate::exit_code::ExitCode;
#[cfg(feature = "std")]
pub use crate::{error::TryFromExitStatusError, exit_code::Result};
