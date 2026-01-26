// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![feature(test)]

extern crate test;

#[cfg(feature = "std")]
use std::{io, process};

use sysexits::ExitCode;
use test::Bencher;

#[bench]
fn from_exit_code_to_u8(b: &mut Bencher) {
    b.iter(|| u8::from(ExitCode::Ok));
}

#[cfg(feature = "std")]
#[bench]
fn from_exit_code_to_process_exit_code(b: &mut Bencher) {
    b.iter(|| process::ExitCode::from(ExitCode::Ok));
}

#[cfg(feature = "std")]
#[bench]
fn from_io_error_to_exit_code(b: &mut Bencher) {
    b.iter(|| ExitCode::from(io::Error::from(io::ErrorKind::NotFound)));
}

#[cfg(feature = "std")]
#[bench]
fn from_io_error_kind_to_exit_code(b: &mut Bencher) {
    b.iter(|| ExitCode::from(io::ErrorKind::NotFound));
}
