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

macro_rules! bench_from_exit_code_to_integer {
    ($T:ty, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| <$T>::from(ExitCode::Ok));
        }
    };
}
bench_from_exit_code_to_integer!(i8, from_exit_code_to_i8);
bench_from_exit_code_to_integer!(i16, from_exit_code_to_i16);
bench_from_exit_code_to_integer!(i32, from_exit_code_to_i32);
bench_from_exit_code_to_integer!(i64, from_exit_code_to_i64);
bench_from_exit_code_to_integer!(i128, from_exit_code_to_i128);
bench_from_exit_code_to_integer!(isize, from_exit_code_to_isize);
bench_from_exit_code_to_integer!(u8, from_exit_code_to_u8);
bench_from_exit_code_to_integer!(u16, from_exit_code_to_u16);
bench_from_exit_code_to_integer!(u32, from_exit_code_to_u32);
bench_from_exit_code_to_integer!(u64, from_exit_code_to_u64);
bench_from_exit_code_to_integer!(u128, from_exit_code_to_u128);
bench_from_exit_code_to_integer!(usize, from_exit_code_to_usize);

#[cfg(feature = "std")]
#[bench]
fn from_exit_code_to_process_exit_code(b: &mut Bencher) {
    b.iter(|| std::process::ExitCode::from(ExitCode::Ok));
}

macro_rules! bench_try_from_integer_to_exit_code {
    ($T:ty, $name:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| ExitCode::try_from(0 as $T).unwrap());
        }
    };
}
bench_try_from_integer_to_exit_code!(i8, try_from_i8_to_exit_code);
bench_try_from_integer_to_exit_code!(i16, try_from_i16_to_exit_code);
bench_try_from_integer_to_exit_code!(i32, try_from_i32_to_exit_code);
bench_try_from_integer_to_exit_code!(i64, try_from_i64_to_exit_code);
bench_try_from_integer_to_exit_code!(i128, try_from_i128_to_exit_code);
bench_try_from_integer_to_exit_code!(isize, try_from_isize_to_exit_code);
bench_try_from_integer_to_exit_code!(u8, try_from_u8_to_exit_code);
bench_try_from_integer_to_exit_code!(u16, try_from_u16_to_exit_code);
bench_try_from_integer_to_exit_code!(u32, try_from_u32_to_exit_code);
bench_try_from_integer_to_exit_code!(u64, try_from_u64_to_exit_code);
bench_try_from_integer_to_exit_code!(u128, try_from_u128_to_exit_code);
bench_try_from_integer_to_exit_code!(usize, try_from_usize_to_exit_code);

#[cfg(feature = "std")]
#[bench]
fn from_io_error_to_exit_code(b: &mut Bencher) {
    use std::io::{Error, ErrorKind};

    b.iter(|| ExitCode::from(Error::from(ErrorKind::NotFound)));
}

#[cfg(feature = "std")]
#[bench]
fn from_io_error_kind_to_exit_code(b: &mut Bencher) {
    use std::io;

    b.iter(|| ExitCode::from(io::ErrorKind::NotFound));
}
