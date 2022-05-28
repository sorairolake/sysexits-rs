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
//! ```
//! use sysexits::SysExits;
//!
//! # fn is_input_exist() -> bool {
//! #     true
//! # }
//! #
//! fn main() -> SysExits {
//!     if !is_input_exist() {
//!         return SysExits::NoInput;
//!     }
//!
//!     SysExits::Ok
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
