// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]
// Lint levels of rustc.
#![forbid(unsafe_code)]
#![deny(missing_debug_implementations)]
#![warn(rust_2018_idioms)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

extern crate test;

use sysexits::ExitCode;
use test::Bencher;

#[bench]
fn from_result_type_to_exit_code(b: &mut Bencher) {
    b.iter(|| ExitCode::from(Ok::<(), ExitCode>(())));
}
