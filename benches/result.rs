// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

use sysexits::ExitCode;
use test::Bencher;

#[bench]
fn from_result_type_to_exit_code(b: &mut Bencher) {
    b.iter(|| ExitCode::from(Ok::<(), ExitCode>(())));
}
