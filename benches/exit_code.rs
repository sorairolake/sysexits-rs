// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use core::error::Error;

use sysexits::ExitCode;
use test::Bencher;

#[bench]
fn default(b: &mut Bencher) {
    b.iter(ExitCode::default);
}

#[bench]
fn equality(b: &mut Bencher) {
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

#[bench]
fn source(b: &mut Bencher) {
    b.iter(|| ExitCode::Ok.source());
}

#[cfg(feature = "std")]
#[bench]
fn report(b: &mut Bencher) {
    use std::process::Termination;

    b.iter(|| ExitCode::Ok.report());
}
