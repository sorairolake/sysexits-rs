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
//! ```
//! # #[cfg(feature = "std")]
//! # {
//! use std::str;
//!
//! use sysexits::ExitCode;
//!
//! fn main() -> ExitCode {
//!     let bytes = [0xF0, 0x9F, 0xA6, 0x80];
//!     match str::from_utf8(&bytes) {
//!         Ok(string) => {
//!             assert_eq!(string, "🦀");
//!             ExitCode::Ok
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!             ExitCode::DataErr
//!         }
//!     }
//! }
//! # }
//! ```
//!
//! [`<sysexits.h>`]: https://man.openbsd.org/sysexits

#![doc(html_root_url = "https://docs.rs/sysexits/0.10.0/")]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Lint levels of rustc.
#![deny(missing_docs)]

#[cfg(test)]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod error;
mod exit_code;

pub use crate::exit_code::{ExitCode, result::Result};
