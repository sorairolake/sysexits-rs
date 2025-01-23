// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2023 Kevin Matthes
// SPDX-FileCopyrightText: 2023 zSchoen
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`ExitCode`] and other types.

use super::ExitCode;
use crate::error::ExitCodeRangeError;

macro_rules! impl_from_exit_code_to_integer {
    ($T:ty, $ok:expr, $usage:expr) => {
        impl From<ExitCode> for $T {
            /// Converts an `ExitCode` into the raw underlying integer value.
            ///
            /// The resulting value is `0` or `64..=78`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sysexits::ExitCode;
            /// #
            #[doc = $ok]
            #[doc = $usage]
            /// ```
            #[inline]
            fn from(code: ExitCode) -> Self {
                code as Self
            }
        }
    };
    ($T:ty) => {
        impl_from_exit_code_to_integer!(
            $T,
            concat!("assert_eq!(", stringify!($T), "::from(ExitCode::Ok), 0);"),
            concat!(
                "assert_eq!(",
                stringify!($T),
                "::from(ExitCode::Usage), 64);"
            )
        );
    };
}
impl_from_exit_code_to_integer!(i8);
impl_from_exit_code_to_integer!(i16);
impl_from_exit_code_to_integer!(i32);
impl_from_exit_code_to_integer!(i64);
impl_from_exit_code_to_integer!(i128);
impl_from_exit_code_to_integer!(isize);
impl_from_exit_code_to_integer!(u8);
impl_from_exit_code_to_integer!(u16);
impl_from_exit_code_to_integer!(u32);
impl_from_exit_code_to_integer!(u64);
impl_from_exit_code_to_integer!(u128);
impl_from_exit_code_to_integer!(usize);

#[cfg(feature = "std")]
impl From<ExitCode> for std::process::ExitCode {
    /// Converts an `sysexits::ExitCode` into an [`std::process::ExitCode`].
    #[inline]
    fn from(code: ExitCode) -> Self {
        use std::process::Termination;

        code.report()
    }
}

macro_rules! impl_try_from_integer_to_exit_code {
    ($T:ty, $ok:expr, $usage:expr, $err:expr) => {
        impl TryFrom<$T> for ExitCode {
            type Error = ExitCodeRangeError;

            /// Converts an integer value into an `ExitCode`.
            ///
            /// # Errors
            ///
            /// Returns [`Err`] if `value` is not `0` or `64..=78`.
            ///
            /// # Examples
            ///
            /// ```
            /// # use sysexits::ExitCode;
            /// #
            #[doc = $ok]
            #[doc = $usage]
            ///
            #[doc = $err]
            /// ```
            #[inline]
            fn try_from(value: $T) -> core::result::Result<Self, Self::Error> {
                match value {
                    0 => Ok(Self::Ok),
                    64 => Ok(Self::Usage),
                    65 => Ok(Self::DataErr),
                    66 => Ok(Self::NoInput),
                    67 => Ok(Self::NoUser),
                    68 => Ok(Self::NoHost),
                    69 => Ok(Self::Unavailable),
                    70 => Ok(Self::Software),
                    71 => Ok(Self::OsErr),
                    72 => Ok(Self::OsFile),
                    73 => Ok(Self::CantCreat),
                    74 => Ok(Self::IoErr),
                    75 => Ok(Self::TempFail),
                    76 => Ok(Self::Protocol),
                    77 => Ok(Self::NoPerm),
                    78 => Ok(Self::Config),
                    _ => Err(ExitCodeRangeError),
                }
            }
        }
    };
    ($T:ty) => {
        impl_try_from_integer_to_exit_code!(
            $T,
            concat!(
                "assert_eq!(ExitCode::try_from(0_",
                stringify!($T),
                "), Ok(ExitCode::Ok));"
            ),
            concat!(
                "assert_eq!(ExitCode::try_from(64_",
                stringify!($T),
                "), Ok(ExitCode::Usage));"
            ),
            concat!(
                "assert!(ExitCode::try_from(79_",
                stringify!($T),
                ").is_err());"
            )
        );
    };
}
impl_try_from_integer_to_exit_code!(i8);
impl_try_from_integer_to_exit_code!(i16);
impl_try_from_integer_to_exit_code!(i32);
impl_try_from_integer_to_exit_code!(i64);
impl_try_from_integer_to_exit_code!(i128);
impl_try_from_integer_to_exit_code!(isize);
impl_try_from_integer_to_exit_code!(u8);
impl_try_from_integer_to_exit_code!(u16);
impl_try_from_integer_to_exit_code!(u32);
impl_try_from_integer_to_exit_code!(u64);
impl_try_from_integer_to_exit_code!(u128);
impl_try_from_integer_to_exit_code!(usize);

#[cfg(feature = "std")]
impl From<std::io::Error> for ExitCode {
    /// Converts an [`Error`](std::io::Error) into an `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::{Error, ErrorKind};
    /// #
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(
    ///     ExitCode::from(Error::from(ErrorKind::NotFound)),
    ///     ExitCode::NoInput
    /// );
    /// ```
    #[inline]
    fn from(error: std::io::Error) -> Self {
        error.kind().into()
    }
}

#[cfg(feature = "std")]
impl From<std::io::ErrorKind> for ExitCode {
    /// Converts an [`ErrorKind`](std::io::ErrorKind) into an `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// #
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::from(io::ErrorKind::NotFound), ExitCode::NoInput);
    /// ```
    #[inline]
    fn from(kind: std::io::ErrorKind) -> Self {
        use std::io::ErrorKind;

        match kind {
            ErrorKind::NotFound => Self::NoInput,
            ErrorKind::PermissionDenied => Self::NoPerm,
            ErrorKind::ConnectionRefused | ErrorKind::OutOfMemory => Self::OsErr,
            ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected
            | ErrorKind::BrokenPipe
            | ErrorKind::TimedOut
            | ErrorKind::Interrupted => Self::TempFail,
            #[cfg(feature = "extended_io_error")]
            ErrorKind::HostUnreachable | ErrorKind::NetworkUnreachable => Self::NoHost,
            ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable => Self::Unavailable,
            #[cfg(feature = "extended_io_error")]
            ErrorKind::NetworkDown => Self::Unavailable,
            ErrorKind::AlreadyExists => Self::CantCreat,
            ErrorKind::WouldBlock | ErrorKind::Unsupported => Self::Protocol,
            #[cfg(feature = "extended_io_error")]
            ErrorKind::ReadOnlyFilesystem => Self::CantCreat,
            ErrorKind::InvalidInput | ErrorKind::InvalidData => Self::DataErr,
            ErrorKind::WriteZero | ErrorKind::UnexpectedEof => Self::Software,
            _ => Self::IoErr,
        }
    }
}

#[cfg(feature = "std")]
impl TryFrom<std::process::ExitStatus> for ExitCode {
    type Error = crate::error::TryFromExitStatusError;

    /// Converts an [`ExitStatus`](std::process::ExitStatus) into an `ExitCode`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any of the following are true:
    ///
    /// - The exit code is not `0` or `64..=78`.
    /// - The exit code is unknown (e.g., the process was terminated by a
    ///   signal).
    #[inline]
    fn try_from(status: std::process::ExitStatus) -> std::result::Result<Self, Self::Error> {
        match status.code() {
            Some(0) => Ok(Self::Ok),
            Some(64) => Ok(Self::Usage),
            Some(65) => Ok(Self::DataErr),
            Some(66) => Ok(Self::NoInput),
            Some(67) => Ok(Self::NoUser),
            Some(68) => Ok(Self::NoHost),
            Some(69) => Ok(Self::Unavailable),
            Some(70) => Ok(Self::Software),
            Some(71) => Ok(Self::OsErr),
            Some(72) => Ok(Self::OsFile),
            Some(73) => Ok(Self::CantCreat),
            Some(74) => Ok(Self::IoErr),
            Some(75) => Ok(Self::TempFail),
            Some(76) => Ok(Self::Protocol),
            Some(77) => Ok(Self::NoPerm),
            Some(78) => Ok(Self::Config),
            Some(code) => Err(Self::Error::new(Some(code))),
            None => Err(Self::Error::new(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(feature = "std", unix))]
    fn get_exit_status(status: i32) -> std::process::ExitStatus {
        use std::process::Command;

        Command::new("sh")
            .arg("-c")
            .arg(format!("exit {status}"))
            .status()
            .unwrap()
    }

    #[cfg(all(feature = "std", windows))]
    fn get_exit_status(status: u32) -> std::process::ExitStatus {
        use std::process::Command;

        Command::new("cmd")
            .arg("/c")
            .arg(format!("exit {status}"))
            .status()
            .unwrap()
    }

    macro_rules! test_from_exit_code_to_integer {
        ($T:ty, $name:ident) => {
            #[test]
            fn $name() {
                assert_eq!(<$T>::from(ExitCode::Ok), 0);
                assert_eq!(<$T>::from(ExitCode::Usage), 64);
                assert_eq!(<$T>::from(ExitCode::DataErr), 65);
                assert_eq!(<$T>::from(ExitCode::NoInput), 66);
                assert_eq!(<$T>::from(ExitCode::NoUser), 67);
                assert_eq!(<$T>::from(ExitCode::NoHost), 68);
                assert_eq!(<$T>::from(ExitCode::Unavailable), 69);
                assert_eq!(<$T>::from(ExitCode::Software), 70);
                assert_eq!(<$T>::from(ExitCode::OsErr), 71);
                assert_eq!(<$T>::from(ExitCode::OsFile), 72);
                assert_eq!(<$T>::from(ExitCode::CantCreat), 73);
                assert_eq!(<$T>::from(ExitCode::IoErr), 74);
                assert_eq!(<$T>::from(ExitCode::TempFail), 75);
                assert_eq!(<$T>::from(ExitCode::Protocol), 76);
                assert_eq!(<$T>::from(ExitCode::NoPerm), 77);
                assert_eq!(<$T>::from(ExitCode::Config), 78);
            }
        };
    }
    test_from_exit_code_to_integer!(i8, from_exit_code_to_i8);
    test_from_exit_code_to_integer!(i16, from_exit_code_to_i16);
    test_from_exit_code_to_integer!(i32, from_exit_code_to_i32);
    test_from_exit_code_to_integer!(i64, from_exit_code_to_i64);
    test_from_exit_code_to_integer!(i128, from_exit_code_to_i128);
    test_from_exit_code_to_integer!(isize, from_exit_code_to_isize);
    test_from_exit_code_to_integer!(u8, from_exit_code_to_u8);
    test_from_exit_code_to_integer!(u16, from_exit_code_to_u16);
    test_from_exit_code_to_integer!(u32, from_exit_code_to_u32);
    test_from_exit_code_to_integer!(u64, from_exit_code_to_u64);
    test_from_exit_code_to_integer!(u128, from_exit_code_to_u128);
    test_from_exit_code_to_integer!(usize, from_exit_code_to_usize);

    #[cfg(feature = "std")]
    #[test]
    fn from_exit_code_to_process_exit_code() {
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Ok)),
            format!("{:?}", std::process::ExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Usage)),
            format!("{:?}", std::process::ExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::DataErr)),
            format!("{:?}", std::process::ExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::NoInput)),
            format!("{:?}", std::process::ExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::NoUser)),
            format!("{:?}", std::process::ExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::NoHost)),
            format!("{:?}", std::process::ExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Unavailable)),
            format!("{:?}", std::process::ExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Software)),
            format!("{:?}", std::process::ExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::OsErr)),
            format!("{:?}", std::process::ExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::OsFile)),
            format!("{:?}", std::process::ExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::CantCreat)),
            format!("{:?}", std::process::ExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::IoErr)),
            format!("{:?}", std::process::ExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::TempFail)),
            format!("{:?}", std::process::ExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Protocol)),
            format!("{:?}", std::process::ExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::NoPerm)),
            format!("{:?}", std::process::ExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", std::process::ExitCode::from(ExitCode::Config)),
            format!("{:?}", std::process::ExitCode::from(78))
        );
    }

    macro_rules! test_try_from_integer_to_exit_code {
        ($T:ty, $name:ident) => {
            #[test]
            fn $name() {
                assert_eq!(ExitCode::try_from(0 as $T).unwrap(), ExitCode::Ok);
                assert_eq!(ExitCode::try_from(64 as $T).unwrap(), ExitCode::Usage);
                assert_eq!(ExitCode::try_from(65 as $T).unwrap(), ExitCode::DataErr);
                assert_eq!(ExitCode::try_from(66 as $T).unwrap(), ExitCode::NoInput);
                assert_eq!(ExitCode::try_from(67 as $T).unwrap(), ExitCode::NoUser);
                assert_eq!(ExitCode::try_from(68 as $T).unwrap(), ExitCode::NoHost);
                assert_eq!(ExitCode::try_from(69 as $T).unwrap(), ExitCode::Unavailable);
                assert_eq!(ExitCode::try_from(70 as $T).unwrap(), ExitCode::Software);
                assert_eq!(ExitCode::try_from(71 as $T).unwrap(), ExitCode::OsErr);
                assert_eq!(ExitCode::try_from(72 as $T).unwrap(), ExitCode::OsFile);
                assert_eq!(ExitCode::try_from(73 as $T).unwrap(), ExitCode::CantCreat);
                assert_eq!(ExitCode::try_from(74 as $T).unwrap(), ExitCode::IoErr);
                assert_eq!(ExitCode::try_from(75 as $T).unwrap(), ExitCode::TempFail);
                assert_eq!(ExitCode::try_from(76 as $T).unwrap(), ExitCode::Protocol);
                assert_eq!(ExitCode::try_from(77 as $T).unwrap(), ExitCode::NoPerm);
                assert_eq!(ExitCode::try_from(78 as $T).unwrap(), ExitCode::Config);
            }
        };
    }
    test_try_from_integer_to_exit_code!(i8, try_from_i8_to_exit_code);
    test_try_from_integer_to_exit_code!(i16, try_from_i16_to_exit_code);
    test_try_from_integer_to_exit_code!(i32, try_from_i32_to_exit_code);
    test_try_from_integer_to_exit_code!(i64, try_from_i64_to_exit_code);
    test_try_from_integer_to_exit_code!(i128, try_from_i128_to_exit_code);
    test_try_from_integer_to_exit_code!(isize, try_from_isize_to_exit_code);
    test_try_from_integer_to_exit_code!(u8, try_from_u8_to_exit_code);
    test_try_from_integer_to_exit_code!(u16, try_from_u16_to_exit_code);
    test_try_from_integer_to_exit_code!(u32, try_from_u32_to_exit_code);
    test_try_from_integer_to_exit_code!(u64, try_from_u64_to_exit_code);
    test_try_from_integer_to_exit_code!(u128, try_from_u128_to_exit_code);
    test_try_from_integer_to_exit_code!(usize, try_from_usize_to_exit_code);

    macro_rules! test_try_from_integer_to_exit_code_when_out_of_range {
        ($T:ty, $name:ident) => {
            #[test]
            fn $name() {
                assert_eq!(
                    ExitCode::try_from(79 as $T).unwrap_err(),
                    ExitCodeRangeError
                );
            }
        };
    }
    test_try_from_integer_to_exit_code_when_out_of_range!(
        i8,
        try_from_i8_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        i16,
        try_from_i16_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        i32,
        try_from_i32_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        i64,
        try_from_i64_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        i128,
        try_from_i128_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        isize,
        try_from_isize_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        u8,
        try_from_u8_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        u16,
        try_from_u16_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        u32,
        try_from_u32_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        u64,
        try_from_u64_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        u128,
        try_from_u128_to_exit_code_when_out_of_range
    );
    test_try_from_integer_to_exit_code_when_out_of_range!(
        usize,
        try_from_usize_to_exit_code_when_out_of_range
    );

    macro_rules! test_try_from_integer_to_exit_code_when_negative_integer_roundtrip {
        ($T:ty, $name:ident) => {
            #[cfg(feature = "std")]
            #[test_strategy::proptest]
            fn $name(#[strategy(..<$T>::default())] v: $T) {
                use proptest::prop_assert_eq;

                prop_assert_eq!(ExitCode::try_from(v).unwrap_err(), ExitCodeRangeError);
            }
        };
    }
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        i8,
        try_from_i8_to_exit_code_when_negative_i8_roundtrip
    );
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        i16,
        try_from_i16_to_exit_code_when_negative_i16_roundtrip
    );
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        i32,
        try_from_i32_to_exit_code_when_negative_i32_roundtrip
    );
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        i64,
        try_from_i64_to_exit_code_when_negative_i64_roundtrip
    );
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        i128,
        try_from_i128_to_exit_code_when_negative_i128_roundtrip
    );
    test_try_from_integer_to_exit_code_when_negative_integer_roundtrip!(
        isize,
        try_from_isize_to_exit_code_when_negative_isize_roundtrip
    );

    macro_rules! test_try_from_integer_to_exit_code_when_middle_integer_roundtrip {
        ($T:ty, $name:ident) => {
            #[cfg(feature = "std")]
            #[test_strategy::proptest]
            fn $name(#[strategy(1..(64 as $T))] v: $T) {
                use proptest::prop_assert_eq;

                prop_assert_eq!(ExitCode::try_from(v).unwrap_err(), ExitCodeRangeError);
            }
        };
    }
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        i8,
        try_from_i8_to_exit_code_when_middle_i8_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        i16,
        try_from_i16_to_exit_code_when_middle_i16_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        i32,
        try_from_i32_to_exit_code_when_middle_i32_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        i64,
        try_from_i64_to_exit_code_when_middle_i64_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        i128,
        try_from_i128_to_exit_code_when_middle_i128_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        isize,
        try_from_isize_to_exit_code_when_middle_isize_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        u8,
        try_from_u8_to_exit_code_when_middle_u8_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        u16,
        try_from_u16_to_exit_code_when_middle_u16_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        u32,
        try_from_u32_to_exit_code_when_middle_u32_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        u64,
        try_from_u64_to_exit_code_when_middle_u64_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        u128,
        try_from_u128_to_exit_code_when_middle_u128_roundtrip
    );
    test_try_from_integer_to_exit_code_when_middle_integer_roundtrip!(
        usize,
        try_from_usize_to_exit_code_when_middle_usize_roundtrip
    );

    macro_rules! test_try_from_integer_to_exit_code_when_positive_integer_roundtrip {
        ($T:ty, $name:ident) => {
            #[cfg(feature = "std")]
            #[test_strategy::proptest]
            fn $name(#[strategy((79 as $T)..)] v: $T) {
                use proptest::prop_assert_eq;

                prop_assert_eq!(ExitCode::try_from(v).unwrap_err(), ExitCodeRangeError);
            }
        };
    }
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        i8,
        try_from_i8_to_exit_code_when_positive_i8_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        i16,
        try_from_i16_to_exit_code_when_positive_i16_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        i32,
        try_from_i32_to_exit_code_when_positive_i32_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        i64,
        try_from_i64_to_exit_code_when_positive_i64_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        i128,
        try_from_i128_to_exit_code_when_positive_i128_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        isize,
        try_from_isize_to_exit_code_when_positive_isize_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        u8,
        try_from_u8_to_exit_code_when_positive_u8_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        u16,
        try_from_u16_to_exit_code_when_positive_u16_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        u32,
        try_from_u32_to_exit_code_when_positive_u32_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        u64,
        try_from_u64_to_exit_code_when_positive_u64_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        u128,
        try_from_u128_to_exit_code_when_positive_u128_roundtrip
    );
    test_try_from_integer_to_exit_code_when_positive_integer_roundtrip!(
        usize,
        try_from_usize_to_exit_code_when_positive_usize_roundtrip
    );

    #[cfg(feature = "std")]
    #[test]
    fn from_io_error_to_exit_code() {
        use std::io::{Error, ErrorKind};

        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NotFound)),
            ExitCode::NoInput
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::PermissionDenied)),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ConnectionRefused)),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ConnectionReset)),
            ExitCode::TempFail
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::HostUnreachable)),
            ExitCode::NoHost
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NetworkUnreachable)),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ConnectionAborted)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NotConnected)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::AddrInUse)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::AddrNotAvailable)),
            ExitCode::Unavailable
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NetworkDown)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::BrokenPipe)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::AlreadyExists)),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::WouldBlock)),
            ExitCode::Protocol
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NotADirectory)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::IsADirectory)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::DirectoryNotEmpty)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ReadOnlyFilesystem)),
            ExitCode::CantCreat
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::FilesystemLoop)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::StaleNetworkFileHandle)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::InvalidInput)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::InvalidData)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::TimedOut)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::WriteZero)),
            ExitCode::Software
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::StorageFull)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::NotSeekable)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::QuotaExceeded)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::FileTooLarge)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ResourceBusy)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ExecutableFileBusy)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::Deadlock)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::CrossesDevices)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::TooManyLinks)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::InvalidFilename)),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::ArgumentListTooLong)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::Interrupted)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::Unsupported)),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::UnexpectedEof)),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::OutOfMemory)),
            ExitCode::OsErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::InProgress)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::Other)),
            ExitCode::IoErr
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_io_error_kind_to_exit_code() {
        use std::io;

        assert_eq!(ExitCode::from(io::ErrorKind::NotFound), ExitCode::NoInput);
        assert_eq!(
            ExitCode::from(io::ErrorKind::PermissionDenied),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::ConnectionRefused),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::ConnectionReset),
            ExitCode::TempFail
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::HostUnreachable),
            ExitCode::NoHost
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::NetworkUnreachable),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::ConnectionAborted),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::NotConnected),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::AddrInUse),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::AddrNotAvailable),
            ExitCode::Unavailable
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::NetworkDown),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::BrokenPipe),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::AlreadyExists),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::WouldBlock),
            ExitCode::Protocol
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::NotADirectory),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::IsADirectory), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::DirectoryNotEmpty),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::ReadOnlyFilesystem),
            ExitCode::CantCreat
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::FilesystemLoop),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::StaleNetworkFileHandle),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::InvalidInput),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::InvalidData),
            ExitCode::DataErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::TimedOut), ExitCode::TempFail);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::WriteZero), ExitCode::Software);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::StorageFull), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::NotSeekable), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::QuotaExceeded),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::FileTooLarge), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::ResourceBusy), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::ExecutableFileBusy),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::Deadlock), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::CrossesDevices),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::TooManyLinks), ExitCode::IoErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::InvalidFilename),
            ExitCode::IoErr
        );
        #[cfg(feature = "extended_io_error")]
        assert_eq!(
            ExitCode::from(io::ErrorKind::ArgumentListTooLong),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::Interrupted),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::Unsupported),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::UnexpectedEof),
            ExitCode::Software
        );
        assert_eq!(ExitCode::from(io::ErrorKind::OutOfMemory), ExitCode::OsErr);
        #[cfg(feature = "extended_io_error")]
        assert_eq!(ExitCode::from(io::ErrorKind::InProgress), ExitCode::IoErr);
        assert_eq!(ExitCode::from(io::ErrorKind::Other), ExitCode::IoErr);
    }

    #[cfg(feature = "std")]
    #[cfg(any(unix, windows))]
    #[test]
    fn try_from_process_exit_status_to_exit_code() {
        assert_eq!(
            ExitCode::try_from(get_exit_status(0)).unwrap(),
            ExitCode::Ok
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(64)).unwrap(),
            ExitCode::Usage
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(65)).unwrap(),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(66)).unwrap(),
            ExitCode::NoInput
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(67)).unwrap(),
            ExitCode::NoUser
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(68)).unwrap(),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(69)).unwrap(),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(70)).unwrap(),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(71)).unwrap(),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(72)).unwrap(),
            ExitCode::OsFile
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(73)).unwrap(),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(74)).unwrap(),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(75)).unwrap(),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(76)).unwrap(),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(77)).unwrap(),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(78)).unwrap(),
            ExitCode::Config
        );
    }

    #[cfg(feature = "std")]
    #[cfg(any(unix, windows))]
    #[test]
    fn try_from_process_exit_status_to_exit_code_when_out_of_range() {
        use crate::error::TryFromExitStatusError;

        assert_eq!(
            ExitCode::try_from(get_exit_status(1)).unwrap_err(),
            TryFromExitStatusError::new(Some(1))
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(63)).unwrap_err(),
            TryFromExitStatusError::new(Some(63))
        );
        assert_eq!(
            ExitCode::try_from(get_exit_status(79)).unwrap_err(),
            TryFromExitStatusError::new(Some(79))
        );
    }

    #[cfg(all(feature = "std", unix))]
    #[test]
    fn try_from_process_exit_status_to_exit_code_when_terminated_by_signal() {
        use crate::error::TryFromExitStatusError;

        fn get_exit_status() -> std::process::ExitStatus {
            use std::process::{Command, Stdio};

            let mut child = Command::new("sh")
                .arg("-c")
                .arg("read a")
                .stdin(Stdio::piped())
                .spawn()
                .unwrap();
            child.kill().unwrap();
            child.wait().unwrap()
        }

        assert_eq!(
            ExitCode::try_from(get_exit_status()).unwrap_err(),
            TryFromExitStatusError::new(None)
        );
    }
}
