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
fn exit_code_equality(b: &mut Bencher) {
    b.iter(|| ExitCode::Ok == ExitCode::Ok);
}

#[bench]
fn is_success_for_successful_termination(b: &mut Bencher) {
    b.iter(|| ExitCode::Ok.is_success());
}

#[bench]
fn is_success_for_unsuccessful_termination(b: &mut Bencher) {
    b.iter(|| ExitCode::Usage.is_success());
}

#[bench]
fn is_failure_for_successful_termination(b: &mut Bencher) {
    b.iter(|| ExitCode::Ok.is_failure());
}

#[bench]
fn is_failure_for_unsuccessful_termination(b: &mut Bencher) {
    b.iter(|| ExitCode::Usage.is_failure());
}

#[cfg(feature = "std")]
#[bench]
fn source_exit_code(b: &mut Bencher) {
    use std::error::Error;

    b.iter(|| ExitCode::Ok.source());
}

#[cfg(feature = "std")]
#[bench]
fn report_exit_code(b: &mut Bencher) {
    use std::process::Termination;

    b.iter(|| ExitCode::Ok.report());
}
