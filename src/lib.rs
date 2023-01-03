//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022-2023 Shun Sakai and Contributors
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
//! ## Returns the exit code as defined by <sysexits.h>
//!
//! If you only use the exit code as defined by `<sysexits.h>`, you can return
//! this from the `main` function.
//!
//! ```
//! # #[cfg(feature = "std")]
//! fn main() -> sysexits::ExitCode {
//!     let bytes = [0xf0, 0x9f, 0x92, 0x96];
//!     match std::str::from_utf8(&bytes) {
//!         Ok(string) => {
//!             println!("{string}");
//!             sysexits::ExitCode::Ok
//!         }
//!         Err(err) => {
//!             eprintln!("{err}");
//!             sysexits::ExitCode::DataErr
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
//! use std::io::Read;
//!
//! # #[cfg(feature = "std")]
//! fn main() -> std::process::ExitCode {
//!     let mut buf = String::new();
//!     if let Err(err) = std::io::stdin().read_to_string(&mut buf) {
//!         eprintln!("{err}");
//!         sysexits::ExitCode::try_from(err.kind()).map_or(
//!             std::process::ExitCode::FAILURE,
//!             std::process::ExitCode::from,
//!         )
//!     } else {
//!         print!("{buf}");
//!         std::process::ExitCode::SUCCESS
//!     }
//! }
//! #
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! ```
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

#![doc(html_root_url = "https://docs.rs/sysexits/0.4.0/")]
#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
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

mod exit_code;

pub use crate::exit_code::ExitCode;
#[cfg(feature = "std")]
pub use crate::exit_code::{FromErrorKindError, FromExitStatusError};
