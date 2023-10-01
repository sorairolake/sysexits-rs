// SPDX-FileCopyrightText: 2022 Kevin Matthes
// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

use core::fmt;
#[cfg(feature = "std")]
use std::process::Termination;

/// A [`Result`](std::result::Result) type based on [`ExitCode`].
///
/// In case of an error, an appropriate variant of [`ExitCode`] can describe the
/// exact cause in further detail.
#[cfg(feature = "std")]
pub type Result<T> = std::result::Result<T, ExitCode>;

/// `ExitCode` is a type that represents the system exit code constants as
/// defined by [`<sysexits.h>`][sysexits-man-url].
///
/// [sysexits-man-url]: https://man.openbsd.org/sysexits
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    /// The input data was incorrect in some way. This should only be used for
    /// user's data and not system files.
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

    /// The user specified did not exist. This might be used for mail addresses
    /// or remote logins.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoUser as u8, 67);
    /// ```
    NoUser,

    /// The host specified did not exist. This is used in mail addresses or
    /// network requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::NoHost as u8, 68);
    /// ```
    NoHost,

    /// A service is unavailable. This can occur if a support program or file
    /// does not exist. This can also be used as a catch-all message when
    /// something you wanted to do doesn't work, but you don't know why.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Unavailable as u8, 69);
    /// ```
    Unavailable,

    /// An internal software error has been detected. This should be limited to
    /// non-operating system related errors if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::Software as u8, 70);
    /// ```
    Software,

    /// An operating system error has been detected. This is intended to be used
    /// for such things as "cannot fork", or "cannot create pipe". It includes
    /// things like [`getuid(2)`][getuid-2-man-url] returning a user that does
    /// not exist in the passwd file.
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

    /// Temporary failure, indicating something that is not really an error. For
    /// example that a mailer could not create a connection, and the request
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

    /// You did not have sufficient permission to perform the operation. This is
    /// not intended for file system problems, which should use
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
    pub const fn is_success(&self) -> bool {
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
    pub const fn is_failure(&self) -> bool {
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
    pub fn exit(self) -> ! {
        std::process::exit(self.into())
    }
}

impl fmt::Display for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
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

macro_rules! impl_from_exit_code_for_integer {
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
        impl_from_exit_code_for_integer!(
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
impl_from_exit_code_for_integer!(i32);
impl_from_exit_code_for_integer!(u8);
impl_from_exit_code_for_integer!(u32);

#[cfg(feature = "std")]
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
impl<T> From<Result<T>> for ExitCode {
    /// Converts a [`Result<T>`] into an `ExitCode`.
    ///
    /// This method returns [`ExitCode::Ok`] if the result is [`Ok`], otherwise
    /// returns the appropriate variant of `ExitCode` contained in the [`Err`]
    /// variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::from(Ok::<(), ExitCode>(())), ExitCode::Ok);
    /// assert_eq!(ExitCode::from(Ok::<u8, ExitCode>(42)), ExitCode::Ok);
    ///
    /// assert_eq!(
    ///     ExitCode::from(Err::<(), ExitCode>(ExitCode::Usage)),
    ///     ExitCode::Usage
    /// );
    /// assert_eq!(
    ///     ExitCode::from(Err::<u8, ExitCode>(ExitCode::Usage)),
    ///     ExitCode::Usage
    /// );
    /// ```
    fn from(result: Result<T>) -> Self {
        result.map_or_else(|code| code, |_| Self::Ok)
    }
}

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
    /// This method returns [`Err`] in the following situations:
    ///
    /// - The exit code is not `0` or `64..=78`.
    /// - The exit code is unknown (e.g., the process was terminated by a
    ///   signal).
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

#[cfg(feature = "std")]
impl std::error::Error for ExitCode {}

#[cfg(feature = "std")]
impl Termination for ExitCode {
    #[inline]
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(u8::from(self))
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

    #[test]
    fn clone_exit_code() {
        assert_eq!(ExitCode::Ok.clone(), ExitCode::Ok);
    }

    #[test]
    fn copy_exit_code() {
        let a = ExitCode::Ok;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn debug_exit_code() {
        assert_eq!(format!("{:?}", ExitCode::Ok), "Ok");
        assert_eq!(format!("{:?}", ExitCode::Usage), "Usage");
        assert_eq!(format!("{:?}", ExitCode::DataErr), "DataErr");
        assert_eq!(format!("{:?}", ExitCode::NoInput), "NoInput");
        assert_eq!(format!("{:?}", ExitCode::NoUser), "NoUser");
        assert_eq!(format!("{:?}", ExitCode::NoHost), "NoHost");
        assert_eq!(format!("{:?}", ExitCode::Unavailable), "Unavailable");
        assert_eq!(format!("{:?}", ExitCode::Software), "Software");
        assert_eq!(format!("{:?}", ExitCode::OsErr), "OsErr");
        assert_eq!(format!("{:?}", ExitCode::OsFile), "OsFile");
        assert_eq!(format!("{:?}", ExitCode::CantCreat), "CantCreat");
        assert_eq!(format!("{:?}", ExitCode::IoErr), "IoErr");
        assert_eq!(format!("{:?}", ExitCode::TempFail), "TempFail");
        assert_eq!(format!("{:?}", ExitCode::Protocol), "Protocol");
        assert_eq!(format!("{:?}", ExitCode::NoPerm), "NoPerm");
        assert_eq!(format!("{:?}", ExitCode::Config), "Config");
    }

    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    #[test]
    fn exit_code_equality() {
        assert_eq!(ExitCode::Ok, ExitCode::Ok);
        assert_ne!(ExitCode::Ok, ExitCode::Usage);
        assert_ne!(ExitCode::Ok, ExitCode::DataErr);
        assert_ne!(ExitCode::Ok, ExitCode::NoInput);
        assert_ne!(ExitCode::Ok, ExitCode::NoUser);
        assert_ne!(ExitCode::Ok, ExitCode::NoHost);
        assert_ne!(ExitCode::Ok, ExitCode::Unavailable);
        assert_ne!(ExitCode::Ok, ExitCode::Software);
        assert_ne!(ExitCode::Ok, ExitCode::OsErr);
        assert_ne!(ExitCode::Ok, ExitCode::OsFile);
        assert_ne!(ExitCode::Ok, ExitCode::CantCreat);
        assert_ne!(ExitCode::Ok, ExitCode::IoErr);
        assert_ne!(ExitCode::Ok, ExitCode::TempFail);
        assert_ne!(ExitCode::Ok, ExitCode::Protocol);
        assert_ne!(ExitCode::Ok, ExitCode::NoPerm);
        assert_ne!(ExitCode::Ok, ExitCode::Config);
        assert_ne!(ExitCode::Usage, ExitCode::Ok);
        assert_eq!(ExitCode::Usage, ExitCode::Usage);
        assert_ne!(ExitCode::Usage, ExitCode::DataErr);
        assert_ne!(ExitCode::Usage, ExitCode::NoInput);
        assert_ne!(ExitCode::Usage, ExitCode::NoUser);
        assert_ne!(ExitCode::Usage, ExitCode::NoHost);
        assert_ne!(ExitCode::Usage, ExitCode::Unavailable);
        assert_ne!(ExitCode::Usage, ExitCode::Software);
        assert_ne!(ExitCode::Usage, ExitCode::OsErr);
        assert_ne!(ExitCode::Usage, ExitCode::OsFile);
        assert_ne!(ExitCode::Usage, ExitCode::CantCreat);
        assert_ne!(ExitCode::Usage, ExitCode::IoErr);
        assert_ne!(ExitCode::Usage, ExitCode::TempFail);
        assert_ne!(ExitCode::Usage, ExitCode::Protocol);
        assert_ne!(ExitCode::Usage, ExitCode::NoPerm);
        assert_ne!(ExitCode::Usage, ExitCode::Config);
        assert_ne!(ExitCode::DataErr, ExitCode::Ok);
        assert_ne!(ExitCode::DataErr, ExitCode::Usage);
        assert_eq!(ExitCode::DataErr, ExitCode::DataErr);
        assert_ne!(ExitCode::DataErr, ExitCode::NoInput);
        assert_ne!(ExitCode::DataErr, ExitCode::NoUser);
        assert_ne!(ExitCode::DataErr, ExitCode::NoHost);
        assert_ne!(ExitCode::DataErr, ExitCode::Unavailable);
        assert_ne!(ExitCode::DataErr, ExitCode::Software);
        assert_ne!(ExitCode::DataErr, ExitCode::OsErr);
        assert_ne!(ExitCode::DataErr, ExitCode::OsFile);
        assert_ne!(ExitCode::DataErr, ExitCode::CantCreat);
        assert_ne!(ExitCode::DataErr, ExitCode::IoErr);
        assert_ne!(ExitCode::DataErr, ExitCode::TempFail);
        assert_ne!(ExitCode::DataErr, ExitCode::Protocol);
        assert_ne!(ExitCode::DataErr, ExitCode::NoPerm);
        assert_ne!(ExitCode::DataErr, ExitCode::Config);
        assert_ne!(ExitCode::NoInput, ExitCode::Ok);
        assert_ne!(ExitCode::NoInput, ExitCode::Usage);
        assert_ne!(ExitCode::NoInput, ExitCode::DataErr);
        assert_eq!(ExitCode::NoInput, ExitCode::NoInput);
        assert_ne!(ExitCode::NoInput, ExitCode::NoUser);
        assert_ne!(ExitCode::NoInput, ExitCode::NoHost);
        assert_ne!(ExitCode::NoInput, ExitCode::Unavailable);
        assert_ne!(ExitCode::NoInput, ExitCode::Software);
        assert_ne!(ExitCode::NoInput, ExitCode::OsErr);
        assert_ne!(ExitCode::NoInput, ExitCode::OsFile);
        assert_ne!(ExitCode::NoInput, ExitCode::CantCreat);
        assert_ne!(ExitCode::NoInput, ExitCode::IoErr);
        assert_ne!(ExitCode::NoInput, ExitCode::TempFail);
        assert_ne!(ExitCode::NoInput, ExitCode::Protocol);
        assert_ne!(ExitCode::NoInput, ExitCode::NoPerm);
        assert_ne!(ExitCode::NoInput, ExitCode::Config);
        assert_ne!(ExitCode::NoUser, ExitCode::Ok);
        assert_ne!(ExitCode::NoUser, ExitCode::Usage);
        assert_ne!(ExitCode::NoUser, ExitCode::DataErr);
        assert_ne!(ExitCode::NoUser, ExitCode::NoInput);
        assert_eq!(ExitCode::NoUser, ExitCode::NoUser);
        assert_ne!(ExitCode::NoUser, ExitCode::NoHost);
        assert_ne!(ExitCode::NoUser, ExitCode::Unavailable);
        assert_ne!(ExitCode::NoUser, ExitCode::Software);
        assert_ne!(ExitCode::NoUser, ExitCode::OsErr);
        assert_ne!(ExitCode::NoUser, ExitCode::OsFile);
        assert_ne!(ExitCode::NoUser, ExitCode::CantCreat);
        assert_ne!(ExitCode::NoUser, ExitCode::IoErr);
        assert_ne!(ExitCode::NoUser, ExitCode::TempFail);
        assert_ne!(ExitCode::NoUser, ExitCode::Protocol);
        assert_ne!(ExitCode::NoUser, ExitCode::NoPerm);
        assert_ne!(ExitCode::NoUser, ExitCode::Config);
        assert_ne!(ExitCode::NoHost, ExitCode::Ok);
        assert_ne!(ExitCode::NoHost, ExitCode::Usage);
        assert_ne!(ExitCode::NoHost, ExitCode::DataErr);
        assert_ne!(ExitCode::NoHost, ExitCode::NoInput);
        assert_ne!(ExitCode::NoHost, ExitCode::NoUser);
        assert_eq!(ExitCode::NoHost, ExitCode::NoHost);
        assert_ne!(ExitCode::NoHost, ExitCode::Unavailable);
        assert_ne!(ExitCode::NoHost, ExitCode::Software);
        assert_ne!(ExitCode::NoHost, ExitCode::OsErr);
        assert_ne!(ExitCode::NoHost, ExitCode::OsFile);
        assert_ne!(ExitCode::NoHost, ExitCode::CantCreat);
        assert_ne!(ExitCode::NoHost, ExitCode::IoErr);
        assert_ne!(ExitCode::NoHost, ExitCode::TempFail);
        assert_ne!(ExitCode::NoHost, ExitCode::Protocol);
        assert_ne!(ExitCode::NoHost, ExitCode::NoPerm);
        assert_ne!(ExitCode::NoHost, ExitCode::Config);
        assert_ne!(ExitCode::Unavailable, ExitCode::Ok);
        assert_ne!(ExitCode::Unavailable, ExitCode::Usage);
        assert_ne!(ExitCode::Unavailable, ExitCode::DataErr);
        assert_ne!(ExitCode::Unavailable, ExitCode::NoInput);
        assert_ne!(ExitCode::Unavailable, ExitCode::NoUser);
        assert_ne!(ExitCode::Unavailable, ExitCode::NoHost);
        assert_eq!(ExitCode::Unavailable, ExitCode::Unavailable);
        assert_ne!(ExitCode::Unavailable, ExitCode::Software);
        assert_ne!(ExitCode::Unavailable, ExitCode::OsErr);
        assert_ne!(ExitCode::Unavailable, ExitCode::OsFile);
        assert_ne!(ExitCode::Unavailable, ExitCode::CantCreat);
        assert_ne!(ExitCode::Unavailable, ExitCode::IoErr);
        assert_ne!(ExitCode::Unavailable, ExitCode::TempFail);
        assert_ne!(ExitCode::Unavailable, ExitCode::Protocol);
        assert_ne!(ExitCode::Unavailable, ExitCode::NoPerm);
        assert_ne!(ExitCode::Unavailable, ExitCode::Config);
        assert_ne!(ExitCode::Software, ExitCode::Ok);
        assert_ne!(ExitCode::Software, ExitCode::Usage);
        assert_ne!(ExitCode::Software, ExitCode::DataErr);
        assert_ne!(ExitCode::Software, ExitCode::NoInput);
        assert_ne!(ExitCode::Software, ExitCode::NoUser);
        assert_ne!(ExitCode::Software, ExitCode::NoHost);
        assert_ne!(ExitCode::Software, ExitCode::Unavailable);
        assert_eq!(ExitCode::Software, ExitCode::Software);
        assert_ne!(ExitCode::Software, ExitCode::OsErr);
        assert_ne!(ExitCode::Software, ExitCode::OsFile);
        assert_ne!(ExitCode::Software, ExitCode::CantCreat);
        assert_ne!(ExitCode::Software, ExitCode::IoErr);
        assert_ne!(ExitCode::Software, ExitCode::TempFail);
        assert_ne!(ExitCode::Software, ExitCode::Protocol);
        assert_ne!(ExitCode::Software, ExitCode::NoPerm);
        assert_ne!(ExitCode::Software, ExitCode::Config);
        assert_ne!(ExitCode::OsErr, ExitCode::Ok);
        assert_ne!(ExitCode::OsErr, ExitCode::Usage);
        assert_ne!(ExitCode::OsErr, ExitCode::DataErr);
        assert_ne!(ExitCode::OsErr, ExitCode::NoInput);
        assert_ne!(ExitCode::OsErr, ExitCode::NoUser);
        assert_ne!(ExitCode::OsErr, ExitCode::NoHost);
        assert_ne!(ExitCode::OsErr, ExitCode::Unavailable);
        assert_ne!(ExitCode::OsErr, ExitCode::Software);
        assert_eq!(ExitCode::OsErr, ExitCode::OsErr);
        assert_ne!(ExitCode::OsErr, ExitCode::OsFile);
        assert_ne!(ExitCode::OsErr, ExitCode::CantCreat);
        assert_ne!(ExitCode::OsErr, ExitCode::IoErr);
        assert_ne!(ExitCode::OsErr, ExitCode::TempFail);
        assert_ne!(ExitCode::OsErr, ExitCode::Protocol);
        assert_ne!(ExitCode::OsErr, ExitCode::NoPerm);
        assert_ne!(ExitCode::OsErr, ExitCode::Config);
        assert_ne!(ExitCode::OsFile, ExitCode::Ok);
        assert_ne!(ExitCode::OsFile, ExitCode::Usage);
        assert_ne!(ExitCode::OsFile, ExitCode::DataErr);
        assert_ne!(ExitCode::OsFile, ExitCode::NoInput);
        assert_ne!(ExitCode::OsFile, ExitCode::NoUser);
        assert_ne!(ExitCode::OsFile, ExitCode::NoHost);
        assert_ne!(ExitCode::OsFile, ExitCode::Unavailable);
        assert_ne!(ExitCode::OsFile, ExitCode::Software);
        assert_ne!(ExitCode::OsFile, ExitCode::OsErr);
        assert_eq!(ExitCode::OsFile, ExitCode::OsFile);
        assert_ne!(ExitCode::OsFile, ExitCode::CantCreat);
        assert_ne!(ExitCode::OsFile, ExitCode::IoErr);
        assert_ne!(ExitCode::OsFile, ExitCode::TempFail);
        assert_ne!(ExitCode::OsFile, ExitCode::Protocol);
        assert_ne!(ExitCode::OsFile, ExitCode::NoPerm);
        assert_ne!(ExitCode::OsFile, ExitCode::Config);
        assert_ne!(ExitCode::CantCreat, ExitCode::Ok);
        assert_ne!(ExitCode::CantCreat, ExitCode::Usage);
        assert_ne!(ExitCode::CantCreat, ExitCode::DataErr);
        assert_ne!(ExitCode::CantCreat, ExitCode::NoInput);
        assert_ne!(ExitCode::CantCreat, ExitCode::NoUser);
        assert_ne!(ExitCode::CantCreat, ExitCode::NoHost);
        assert_ne!(ExitCode::CantCreat, ExitCode::Unavailable);
        assert_ne!(ExitCode::CantCreat, ExitCode::Software);
        assert_ne!(ExitCode::CantCreat, ExitCode::OsErr);
        assert_ne!(ExitCode::CantCreat, ExitCode::OsFile);
        assert_eq!(ExitCode::CantCreat, ExitCode::CantCreat);
        assert_ne!(ExitCode::CantCreat, ExitCode::IoErr);
        assert_ne!(ExitCode::CantCreat, ExitCode::TempFail);
        assert_ne!(ExitCode::CantCreat, ExitCode::Protocol);
        assert_ne!(ExitCode::CantCreat, ExitCode::NoPerm);
        assert_ne!(ExitCode::CantCreat, ExitCode::Config);
        assert_ne!(ExitCode::IoErr, ExitCode::Ok);
        assert_ne!(ExitCode::IoErr, ExitCode::Usage);
        assert_ne!(ExitCode::IoErr, ExitCode::DataErr);
        assert_ne!(ExitCode::IoErr, ExitCode::NoInput);
        assert_ne!(ExitCode::IoErr, ExitCode::NoUser);
        assert_ne!(ExitCode::IoErr, ExitCode::NoHost);
        assert_ne!(ExitCode::IoErr, ExitCode::Unavailable);
        assert_ne!(ExitCode::IoErr, ExitCode::Software);
        assert_ne!(ExitCode::IoErr, ExitCode::OsErr);
        assert_ne!(ExitCode::IoErr, ExitCode::OsFile);
        assert_ne!(ExitCode::IoErr, ExitCode::CantCreat);
        assert_eq!(ExitCode::IoErr, ExitCode::IoErr);
        assert_ne!(ExitCode::IoErr, ExitCode::TempFail);
        assert_ne!(ExitCode::IoErr, ExitCode::Protocol);
        assert_ne!(ExitCode::IoErr, ExitCode::NoPerm);
        assert_ne!(ExitCode::IoErr, ExitCode::Config);
        assert_ne!(ExitCode::TempFail, ExitCode::Ok);
        assert_ne!(ExitCode::TempFail, ExitCode::Usage);
        assert_ne!(ExitCode::TempFail, ExitCode::DataErr);
        assert_ne!(ExitCode::TempFail, ExitCode::NoInput);
        assert_ne!(ExitCode::TempFail, ExitCode::NoUser);
        assert_ne!(ExitCode::TempFail, ExitCode::NoHost);
        assert_ne!(ExitCode::TempFail, ExitCode::Unavailable);
        assert_ne!(ExitCode::TempFail, ExitCode::Software);
        assert_ne!(ExitCode::TempFail, ExitCode::OsErr);
        assert_ne!(ExitCode::TempFail, ExitCode::OsFile);
        assert_ne!(ExitCode::TempFail, ExitCode::CantCreat);
        assert_ne!(ExitCode::TempFail, ExitCode::IoErr);
        assert_eq!(ExitCode::TempFail, ExitCode::TempFail);
        assert_ne!(ExitCode::TempFail, ExitCode::Protocol);
        assert_ne!(ExitCode::TempFail, ExitCode::NoPerm);
        assert_ne!(ExitCode::TempFail, ExitCode::Config);
        assert_ne!(ExitCode::Protocol, ExitCode::Ok);
        assert_ne!(ExitCode::Protocol, ExitCode::Usage);
        assert_ne!(ExitCode::Protocol, ExitCode::DataErr);
        assert_ne!(ExitCode::Protocol, ExitCode::NoInput);
        assert_ne!(ExitCode::Protocol, ExitCode::NoUser);
        assert_ne!(ExitCode::Protocol, ExitCode::NoHost);
        assert_ne!(ExitCode::Protocol, ExitCode::Unavailable);
        assert_ne!(ExitCode::Protocol, ExitCode::Software);
        assert_ne!(ExitCode::Protocol, ExitCode::OsErr);
        assert_ne!(ExitCode::Protocol, ExitCode::OsFile);
        assert_ne!(ExitCode::Protocol, ExitCode::CantCreat);
        assert_ne!(ExitCode::Protocol, ExitCode::IoErr);
        assert_ne!(ExitCode::Protocol, ExitCode::TempFail);
        assert_eq!(ExitCode::Protocol, ExitCode::Protocol);
        assert_ne!(ExitCode::Protocol, ExitCode::NoPerm);
        assert_ne!(ExitCode::Protocol, ExitCode::Config);
        assert_ne!(ExitCode::NoPerm, ExitCode::Ok);
        assert_ne!(ExitCode::NoPerm, ExitCode::Usage);
        assert_ne!(ExitCode::NoPerm, ExitCode::DataErr);
        assert_ne!(ExitCode::NoPerm, ExitCode::NoInput);
        assert_ne!(ExitCode::NoPerm, ExitCode::NoUser);
        assert_ne!(ExitCode::NoPerm, ExitCode::NoHost);
        assert_ne!(ExitCode::NoPerm, ExitCode::Unavailable);
        assert_ne!(ExitCode::NoPerm, ExitCode::Software);
        assert_ne!(ExitCode::NoPerm, ExitCode::OsErr);
        assert_ne!(ExitCode::NoPerm, ExitCode::OsFile);
        assert_ne!(ExitCode::NoPerm, ExitCode::CantCreat);
        assert_ne!(ExitCode::NoPerm, ExitCode::IoErr);
        assert_ne!(ExitCode::NoPerm, ExitCode::TempFail);
        assert_ne!(ExitCode::NoPerm, ExitCode::Protocol);
        assert_eq!(ExitCode::NoPerm, ExitCode::NoPerm);
        assert_ne!(ExitCode::NoPerm, ExitCode::Config);
        assert_ne!(ExitCode::Config, ExitCode::Ok);
        assert_ne!(ExitCode::Config, ExitCode::Usage);
        assert_ne!(ExitCode::Config, ExitCode::DataErr);
        assert_ne!(ExitCode::Config, ExitCode::NoInput);
        assert_ne!(ExitCode::Config, ExitCode::NoUser);
        assert_ne!(ExitCode::Config, ExitCode::NoHost);
        assert_ne!(ExitCode::Config, ExitCode::Unavailable);
        assert_ne!(ExitCode::Config, ExitCode::Software);
        assert_ne!(ExitCode::Config, ExitCode::OsErr);
        assert_ne!(ExitCode::Config, ExitCode::OsFile);
        assert_ne!(ExitCode::Config, ExitCode::CantCreat);
        assert_ne!(ExitCode::Config, ExitCode::IoErr);
        assert_ne!(ExitCode::Config, ExitCode::TempFail);
        assert_ne!(ExitCode::Config, ExitCode::Protocol);
        assert_ne!(ExitCode::Config, ExitCode::NoPerm);
        assert_eq!(ExitCode::Config, ExitCode::Config);
    }

    #[test]
    fn display_exit_code() {
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
    fn is_success_for_successful_termination() {
        assert!(ExitCode::Ok.is_success());
    }

    #[test]
    fn is_success_for_unsuccessful_termination() {
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
    fn is_failure_for_successful_termination() {
        assert!(!ExitCode::Ok.is_failure());
    }

    #[test]
    fn is_failure_for_unsuccessful_termination() {
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

    macro_rules! from_exit_code_to_integer {
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
    from_exit_code_to_integer!(i32, from_exit_code_to_i32);
    from_exit_code_to_integer!(u8, from_exit_code_to_u8);
    from_exit_code_to_integer!(u32, from_exit_code_to_u32);

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

    #[cfg(feature = "std")]
    #[test]
    fn result_type() {
        assert_eq!(
            std::any::type_name::<Result<()>>(),
            std::any::type_name::<std::result::Result<(), ExitCode>>()
        );
        assert_eq!(
            std::any::type_name::<Result<u8>>(),
            std::any::type_name::<std::result::Result<u8, ExitCode>>()
        );
    }

    #[cfg(feature = "std")]
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    #[test]
    fn from_result_type_to_exit_code() {
        assert_eq!(ExitCode::from(Ok::<(), ExitCode>(())), ExitCode::Ok);
        assert_eq!(ExitCode::from(Ok::<u8, ExitCode>(42)), ExitCode::Ok);

        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::Usage)),
            ExitCode::Usage
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::DataErr)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::NoInput)),
            ExitCode::NoInput
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::NoUser)),
            ExitCode::NoUser
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::NoHost)),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::Unavailable)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::Software)),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::OsErr)),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::OsFile)),
            ExitCode::OsFile
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::CantCreat)),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::IoErr)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::TempFail)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::Protocol)),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::NoPerm)),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::from(Err::<(), ExitCode>(ExitCode::Config)),
            ExitCode::Config
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::Usage)),
            ExitCode::Usage
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::DataErr)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::NoInput)),
            ExitCode::NoInput
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::NoUser)),
            ExitCode::NoUser
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::NoHost)),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::Unavailable)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::Software)),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::OsErr)),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::OsFile)),
            ExitCode::OsFile
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::CantCreat)),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::IoErr)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::TempFail)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::Protocol)),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::NoPerm)),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::from(Err::<u8, ExitCode>(ExitCode::Config)),
            ExitCode::Config
        );
    }

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
        assert_eq!(
            ExitCode::from(Error::from(ErrorKind::Other)),
            ExitCode::IoErr
        );
    }

    #[cfg(feature = "std")]
    #[allow(clippy::cognitive_complexity)]
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
        assert_eq!(
            ExitCode::from(io::ErrorKind::InvalidInput),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::InvalidData),
            ExitCode::DataErr
        );
        assert_eq!(ExitCode::from(io::ErrorKind::TimedOut), ExitCode::TempFail);
        assert_eq!(ExitCode::from(io::ErrorKind::WriteZero), ExitCode::Software);
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
        assert_eq!(ExitCode::from(io::ErrorKind::Other), ExitCode::IoErr);
    }

    #[cfg(feature = "std")]
    #[cfg(any(unix, windows))]
    #[allow(clippy::cognitive_complexity)]
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

    #[cfg(feature = "std")]
    #[test]
    fn source_exit_code() {
        use std::error::Error;

        assert!(ExitCode::Ok.source().is_none());
        assert!(ExitCode::Usage.source().is_none());
        assert!(ExitCode::DataErr.source().is_none());
        assert!(ExitCode::NoInput.source().is_none());
        assert!(ExitCode::NoUser.source().is_none());
        assert!(ExitCode::NoHost.source().is_none());
        assert!(ExitCode::Unavailable.source().is_none());
        assert!(ExitCode::Software.source().is_none());
        assert!(ExitCode::OsErr.source().is_none());
        assert!(ExitCode::OsFile.source().is_none());
        assert!(ExitCode::CantCreat.source().is_none());
        assert!(ExitCode::IoErr.source().is_none());
        assert!(ExitCode::TempFail.source().is_none());
        assert!(ExitCode::Protocol.source().is_none());
        assert!(ExitCode::NoPerm.source().is_none());
        assert!(ExitCode::Config.source().is_none());
    }

    #[cfg(feature = "std")]
    #[test]
    fn report_exit_code() {
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
}
