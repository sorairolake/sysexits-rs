//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! The system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

use core::fmt;
#[cfg(feature = "std")]
use std::process::Termination;

/// `ExitCode` is a type that represents the system exit code constants as
/// defined by [`<sysexits.h>`][sysexits-man-url].
///
/// [sysexits-man-url]: https://man.openbsd.org/sysexits
#[derive(Clone, Copy, Debug)]
pub enum ExitCode {
    /// The successful exit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Ok as u8, 0);
    /// ```
    Ok,

    /// The command was used incorrectly, e.g., with the wrong number of
    /// arguments, a bad flag, bad syntax in a parameter, or whatever.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Usage as u8, 64);
    /// ```
    Usage = 64,

    /// The input data was incorrect in some way.
    /// This should only be used for user's data and not system files.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::DataErr as u8, 65);
    /// ```
    DataErr,

    /// An input file (not a system file) did not exist or was not readable.
    /// This could also include errors like "No message" to a mailer (if it
    /// cared to catch it).
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoInput as u8, 66);
    /// ```
    NoInput,

    /// The user specified did not exist.
    /// This might be used for mail addresses or remote logins.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoUser as u8, 67);
    /// ```
    NoUser,

    /// The host specified did not exist.
    /// This is used in mail addresses or network requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoHost as u8, 68);
    /// ```
    NoHost,

    /// A service is unavailable.
    /// This can occur if a support program or file does not exist.
    /// This can also be used as a catch-all message when something you wanted
    /// to do doesn't work, but you don't know why.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Unavailable as u8, 69);
    /// ```
    Unavailable,

    /// An internal software error has been detected.
    /// This should be limited to non-operating system related errors if
    /// possible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Software as u8, 70);
    /// ```
    Software,

    /// An operating system error has been detected.
    /// This is intended to be used for such things as "cannot fork", or "cannot
    /// create pipe". It includes things like
    /// [`getuid(2)`][getuid-2-man-url] returning a user that does not exist in
    /// the passwd file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::OsErr as u8, 71);
    /// ```
    ///
    /// [getuid-2-man-url]: https://man.openbsd.org/getuid.2
    OsErr,

    /// Some system file (e.g., `/etc/passwd`, `/var/run/utmp`) does not exist,
    /// cannot be opened, or has some sort of error (e.g., syntax error).
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::OsFile as u8, 72);
    /// ```
    OsFile,

    /// A (user specified) output file cannot be created.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::CantCreat as u8, 73);
    /// ```
    CantCreat,

    /// An error occurred while doing I/O on some file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::IoErr as u8, 74);
    /// ```
    IoErr,

    /// Temporary failure, indicating something that is not really an error.
    /// For example that a mailer could not create a connection, and the request
    /// should be reattempted later.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::TempFail as u8, 75);
    /// ```
    TempFail,

    /// The remote system returned something that was "not possible" during a
    /// protocol exchange.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Protocol as u8, 76);
    /// ```
    Protocol,

    /// You did not have sufficient permission to perform the operation.
    /// This is not intended for file system problems, which should use
    /// [`NoInput`](Self::NoInput) or [`CantCreat`](Self::CantCreat), but rather
    /// for higher level permissions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoPerm as u8, 77);
    /// ```
    NoPerm,

    /// Something was found in an unconfigured or misconfigured state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Config as u8, 78);
    /// ```
    Config,
}

impl ExitCode {
    /// Returns `true` if this system exit code represents successful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Ok.is_success(), true);
    /// assert_eq!(ExitCode::Usage.is_success(), false);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns `true` if this system exit code represents unsuccessful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Ok.is_failure(), false);
    /// assert_eq!(ExitCode::Usage.is_failure(), true);
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_failure(self) -> bool {
        !self.is_success()
    }

    /// Terminates the current process with the exit code defined by `ExitCode`.
    ///
    /// This method is equivalent to [`std::process::exit`] with a restricted
    /// exit code.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// fn main() {
    ///     ExitCode::Ok.exit();
    /// }
    /// ```
    #[cfg(feature = "std")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
    pub fn exit(self) -> ! {
        std::process::exit(self.into())
    }
}

impl fmt::Display for ExitCode {
    /// Implements the [`Display`](fmt::Display) trait.
    ///
    /// `ExitCode` implements the [`Display`](fmt::Display) trait such that it
    /// can be formatted using the given formatter. Thereby, the respective
    /// variant will be casted to its integer representation [`u8`], at first,
    /// before being processed by the given formatter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{}", ExitCode::Ok), "0");
    /// assert_eq!(format!("{}", ExitCode::Usage), "64");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl From<std::io::ErrorKind> for ExitCode {
    /// Converts an [`ErrorKind`](std::io::ErrorKind) into an `ExitCode`.
    ///
    /// This method returns [`ExitCode::IoErr`] if there is not a suitable
    /// `ExitCode` to represent an [`ErrorKind`](std::io::ErrorKind).
    fn from(kind: std::io::ErrorKind) -> Self {
        use std::io::ErrorKind;

        match kind {
            ErrorKind::NotFound => Self::OsFile,
            ErrorKind::PermissionDenied => Self::NoPerm,
            ErrorKind::ConnectionRefused
            | ErrorKind::ConnectionReset
            | ErrorKind::ConnectionAborted
            | ErrorKind::NotConnected => Self::Protocol,
            ErrorKind::AddrInUse | ErrorKind::AddrNotAvailable => Self::Unavailable,
            ErrorKind::AlreadyExists => Self::CantCreat,
            ErrorKind::InvalidInput | ErrorKind::InvalidData | ErrorKind::UnexpectedEof => {
                Self::DataErr
            }
            _ => Self::IoErr,
        }
    }
}

macro_rules! impl_from_exit_code_for_integer {
    ($T:ty) => {
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
            #[doc = concat!("assert_eq!(", stringify!($T), "::from(ExitCode::Ok), 0);")]
            #[doc = concat!("assert_eq!(", stringify!($T), "::from(ExitCode::Usage), 64);")]
            /// ```
            #[inline]
            fn from(code: ExitCode) -> Self {
                code as Self
            }
        }
    };
}
impl_from_exit_code_for_integer!(i32);
impl_from_exit_code_for_integer!(u8);
impl_from_exit_code_for_integer!(u32);

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl From<ExitCode> for std::process::ExitCode {
    /// Converts an `sysexits::ExitCode` into an [`std::process::ExitCode`].
    ///
    /// This method is equivalent to [`ExitCode::report`].
    #[inline]
    fn from(code: ExitCode) -> Self {
        code.report()
    }
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl TryFrom<std::process::ExitStatus> for ExitCode {
    type Error = FromExitStatusError;

    /// Converts an [`ExitStatus`](std::process::ExitStatus) into an `ExitCode`.
    ///
    /// # Errors
    ///
    /// This method returns [`Err`] in the following situations:
    ///
    /// - The raw underlying integer value is not `0` or `64..=78`.
    /// - The raw underlying integer value is unknown.
    fn try_from(status: std::process::ExitStatus) -> Result<Self, Self::Error> {
        #[cfg(unix)]
        fn code(status: std::process::ExitStatus) -> Option<i32> {
            use std::os::unix::process::ExitStatusExt;

            status.code().or_else(|| status.signal())
        }

        #[cfg(not(unix))]
        fn code(status: std::process::ExitStatus) -> Option<i32> {
            status.code()
        }

        match code(status) {
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
            Some(code) => Err(FromExitStatusError(Some(code))),
            None => Err(FromExitStatusError(None)),
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
impl Termination for ExitCode {
    #[inline]
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(u8::from(self))
    }
}

/// An error which can be returned when converting an [`ExitCode`] from an
/// [`ExitStatus`](std::process::ExitStatus).
#[cfg(feature = "std")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "std")))]
#[derive(Debug)]
pub struct FromExitStatusError(Option<i32>);

#[cfg(feature = "std")]
impl fmt::Display for FromExitStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.0 {
            write!(f, "invalid exit code `{code}`")
        } else {
            write!(f, "exit code is unknown")
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromExitStatusError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(feature = "std", unix))]
    fn code(raw: i32) -> std::process::ExitStatus {
        use std::os::unix::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(raw)
    }

    #[cfg(all(feature = "std", windows))]
    fn code(raw: u32) -> std::process::ExitStatus {
        use std::os::windows::process::ExitStatusExt;

        std::process::ExitStatus::from_raw(raw)
    }

    #[test]
    fn test_display_trait_implementation() {
        assert_eq!(format!("{}", ExitCode::Ok), format!("{}", 0));
        assert_eq!(format!("{}", ExitCode::Usage), format!("{}", 64));
        assert_eq!(format!("{}", ExitCode::DataErr), format!("{}", 65));
        assert_eq!(format!("{}", ExitCode::NoInput), format!("{}", 66));
        assert_eq!(format!("{}", ExitCode::NoUser), format!("{}", 67));
        assert_eq!(format!("{}", ExitCode::NoHost), format!("{}", 68));
        assert_eq!(format!("{}", ExitCode::Unavailable), format!("{}", 69));
        assert_eq!(format!("{}", ExitCode::Software), format!("{}", 70));
        assert_eq!(format!("{}", ExitCode::OsErr), format!("{}", 71));
        assert_eq!(format!("{}", ExitCode::OsFile), format!("{}", 72));
        assert_eq!(format!("{}", ExitCode::CantCreat), format!("{}", 73));
        assert_eq!(format!("{}", ExitCode::IoErr), format!("{}", 74));
        assert_eq!(format!("{}", ExitCode::TempFail), format!("{}", 75));
        assert_eq!(format!("{}", ExitCode::Protocol), format!("{}", 76));
        assert_eq!(format!("{}", ExitCode::NoPerm), format!("{}", 77));
        assert_eq!(format!("{}", ExitCode::Config), format!("{}", 78));
    }

    #[test]
    fn test_is_success_for_successful_termination() {
        assert!(ExitCode::Ok.is_success());
    }

    #[test]
    fn test_is_success_for_unsuccessful_termination() {
        assert!(!ExitCode::Usage.is_success());
        assert!(!ExitCode::DataErr.is_success());
        assert!(!ExitCode::NoInput.is_success());
        assert!(!ExitCode::NoUser.is_success());
        assert!(!ExitCode::NoHost.is_success());
        assert!(!ExitCode::Unavailable.is_success());
        assert!(!ExitCode::Software.is_success());
        assert!(!ExitCode::OsErr.is_success());
        assert!(!ExitCode::OsFile.is_success());
        assert!(!ExitCode::CantCreat.is_success());
        assert!(!ExitCode::IoErr.is_success());
        assert!(!ExitCode::TempFail.is_success());
        assert!(!ExitCode::Protocol.is_success());
        assert!(!ExitCode::NoPerm.is_success());
        assert!(!ExitCode::Config.is_success());
    }

    #[test]
    fn test_is_failure_for_successful_termination() {
        assert!(!ExitCode::Ok.is_failure());
    }

    #[test]
    fn test_is_failure_for_unsuccessful_termination() {
        assert!(ExitCode::Usage.is_failure());
        assert!(ExitCode::DataErr.is_failure());
        assert!(ExitCode::NoInput.is_failure());
        assert!(ExitCode::NoUser.is_failure());
        assert!(ExitCode::NoHost.is_failure());
        assert!(ExitCode::Unavailable.is_failure());
        assert!(ExitCode::Software.is_failure());
        assert!(ExitCode::OsErr.is_failure());
        assert!(ExitCode::OsFile.is_failure());
        assert!(ExitCode::CantCreat.is_failure());
        assert!(ExitCode::IoErr.is_failure());
        assert!(ExitCode::TempFail.is_failure());
        assert!(ExitCode::Protocol.is_failure());
        assert!(ExitCode::NoPerm.is_failure());
        assert!(ExitCode::Config.is_failure());
    }

    #[cfg(feature = "std")]
    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_from_io_error_kind_for_exit_code() {
        use std::io::ErrorKind;

        assert!(matches!(
            ExitCode::from(ErrorKind::NotFound),
            ExitCode::OsFile
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::PermissionDenied),
            ExitCode::NoPerm
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::ConnectionRefused),
            ExitCode::Protocol
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::ConnectionReset),
            ExitCode::Protocol
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::ConnectionAborted),
            ExitCode::Protocol
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::NotConnected),
            ExitCode::Protocol
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::AddrInUse),
            ExitCode::Unavailable
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::AddrNotAvailable),
            ExitCode::Unavailable
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::BrokenPipe),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::AlreadyExists),
            ExitCode::CantCreat
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::WouldBlock),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::InvalidInput),
            ExitCode::DataErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::InvalidData),
            ExitCode::DataErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::TimedOut),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::WriteZero),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::Interrupted),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::Unsupported),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::UnexpectedEof),
            ExitCode::DataErr
        ));
        assert!(matches!(
            ExitCode::from(ErrorKind::OutOfMemory),
            ExitCode::IoErr
        ));
        assert!(matches!(ExitCode::from(ErrorKind::Other), ExitCode::IoErr));
    }

    macro_rules! test_from_exit_code_for_integer {
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
    test_from_exit_code_for_integer!(i32, test_from_exit_code_for_i32);
    test_from_exit_code_for_integer!(u8, test_from_exit_code_for_u8);
    test_from_exit_code_for_integer!(u32, test_from_exit_code_for_u32);

    #[cfg(feature = "std")]
    #[test]
    fn test_from_exit_code_for_process_exit_code() {
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

    #[cfg(feature = "std")]
    #[cfg(any(unix, windows))]
    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_try_from_process_exit_status_for_exit_code() {
        assert!(matches!(ExitCode::try_from(code(0)).unwrap(), ExitCode::Ok));
        assert!(matches!(
            ExitCode::try_from(code(64)).unwrap(),
            ExitCode::Usage
        ));
        assert!(matches!(
            ExitCode::try_from(code(65)).unwrap(),
            ExitCode::DataErr
        ));
        assert!(matches!(
            ExitCode::try_from(code(66)).unwrap(),
            ExitCode::NoInput
        ));
        assert!(matches!(
            ExitCode::try_from(code(67)).unwrap(),
            ExitCode::NoUser
        ));
        assert!(matches!(
            ExitCode::try_from(code(68)).unwrap(),
            ExitCode::NoHost
        ));
        assert!(matches!(
            ExitCode::try_from(code(69)).unwrap(),
            ExitCode::Unavailable
        ));
        assert!(matches!(
            ExitCode::try_from(code(70)).unwrap(),
            ExitCode::Software
        ));
        assert!(matches!(
            ExitCode::try_from(code(71)).unwrap(),
            ExitCode::OsErr
        ));
        assert!(matches!(
            ExitCode::try_from(code(72)).unwrap(),
            ExitCode::OsFile
        ));
        assert!(matches!(
            ExitCode::try_from(code(73)).unwrap(),
            ExitCode::CantCreat
        ));
        assert!(matches!(
            ExitCode::try_from(code(74)).unwrap(),
            ExitCode::IoErr
        ));
        assert!(matches!(
            ExitCode::try_from(code(75)).unwrap(),
            ExitCode::TempFail
        ));
        assert!(matches!(
            ExitCode::try_from(code(76)).unwrap(),
            ExitCode::Protocol
        ));
        assert!(matches!(
            ExitCode::try_from(code(77)).unwrap(),
            ExitCode::NoPerm
        ));
        assert!(matches!(
            ExitCode::try_from(code(78)).unwrap(),
            ExitCode::Config
        ));
    }

    #[cfg(feature = "std")]
    #[cfg(any(unix, windows))]
    #[test]
    fn test_try_from_process_exit_status_for_exit_code_when_out_of_range() {
        assert!(matches!(
            ExitCode::try_from(code(1)).unwrap_err(),
            FromExitStatusError(Some(1))
        ));
        assert!(matches!(
            ExitCode::try_from(code(63)).unwrap_err(),
            FromExitStatusError(Some(63))
        ));
        assert!(matches!(
            ExitCode::try_from(code(79)).unwrap_err(),
            FromExitStatusError(Some(79))
        ));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_report_status_code() {
        assert_eq!(
            format!("{:?}", ExitCode::Ok.report()),
            format!("{:?}", std::process::ExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Usage.report()),
            format!("{:?}", std::process::ExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", ExitCode::DataErr.report()),
            format!("{:?}", std::process::ExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoInput.report()),
            format!("{:?}", std::process::ExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoUser.report()),
            format!("{:?}", std::process::ExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoHost.report()),
            format!("{:?}", std::process::ExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Unavailable.report()),
            format!("{:?}", std::process::ExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Software.report()),
            format!("{:?}", std::process::ExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", ExitCode::OsErr.report()),
            format!("{:?}", std::process::ExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", ExitCode::OsFile.report()),
            format!("{:?}", std::process::ExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", ExitCode::CantCreat.report()),
            format!("{:?}", std::process::ExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", ExitCode::IoErr.report()),
            format!("{:?}", std::process::ExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", ExitCode::TempFail.report()),
            format!("{:?}", std::process::ExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Protocol.report()),
            format!("{:?}", std::process::ExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoPerm.report()),
            format!("{:?}", std::process::ExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Config.report()),
            format!("{:?}", std::process::ExitCode::from(78))
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_display_for_from_exit_status_error() {
        assert_eq!(
            format!("{}", FromExitStatusError(Some(1))),
            "invalid exit code `1`"
        );
        assert_eq!(
            format!("{}", FromExitStatusError(None)),
            "exit code is unknown"
        );
    }
}
