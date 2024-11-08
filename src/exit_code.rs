// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2023 Kevin Matthes
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The system exit code constants as defined by [`<sysexits.h>`].
//!
//! [`<sysexits.h>`]: https://man.openbsd.org/sysexits

mod consts;
mod convert;
mod fmt;
pub mod result;

/// `ExitCode` is a type that represents the system exit code constants as
/// defined by [`<sysexits.h>`].
///
/// [`<sysexits.h>`]: https://man.openbsd.org/sysexits
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ExitCode {
    /// The successful exit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::Ok), 0);
    /// ```
    #[default]
    Ok,

    /// The command was used incorrectly, e.g., with the wrong number of
    /// arguments, a bad flag, bad syntax in a parameter, or whatever.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::Usage), 64);
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
    /// assert_eq!(u8::from(ExitCode::DataErr), 65);
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
    /// assert_eq!(u8::from(ExitCode::NoInput), 66);
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
    /// assert_eq!(u8::from(ExitCode::NoUser), 67);
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
    /// assert_eq!(u8::from(ExitCode::NoHost), 68);
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
    /// assert_eq!(u8::from(ExitCode::Unavailable), 69);
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
    /// assert_eq!(u8::from(ExitCode::Software), 70);
    /// ```
    Software,

    /// An operating system error has been detected. This is intended to be used
    /// for such things as "cannot fork", or "cannot create pipe". It includes
    /// things like [`getuid(2)`] returning a user that does not exist in the
    /// passwd file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::OsErr), 71);
    /// ```
    ///
    /// [`getuid(2)`]: https://man.openbsd.org/getuid.2
    OsErr,

    /// Some system file (e.g., `/etc/passwd`, `/var/run/utmp`) does not exist,
    /// cannot be opened, or has some sort of error (e.g., syntax error).
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::OsFile), 72);
    /// ```
    OsFile,

    /// A (user specified) output file cannot be created.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::CantCreat), 73);
    /// ```
    CantCreat,

    /// An error occurred while doing I/O on some file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::IoErr), 74);
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
    /// assert_eq!(u8::from(ExitCode::TempFail), 75);
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
    /// assert_eq!(u8::from(ExitCode::Protocol), 76);
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
    /// assert_eq!(u8::from(ExitCode::NoPerm), 77);
    /// ```
    NoPerm,

    /// Something was found in an unconfigured or misconfigured state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::Config), 78);
    /// ```
    Config,
}

impl ExitCode {
    /// Returns [`true`] if this system exit code represents successful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert!(ExitCode::Ok.is_success());
    /// assert!(!ExitCode::Usage.is_success());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns [`true`] if this system exit code represents unsuccessful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert!(!ExitCode::Ok.is_failure());
    /// assert!(ExitCode::Usage.is_failure());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_failure(&self) -> bool {
        !self.is_success()
    }

    /// Terminates the current process with the exit code defined by `ExitCode`.
    ///
    /// Equivalent to [`std::process::exit`] with a restricted exit code.
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
    #[inline]
    pub fn exit(self) -> ! {
        std::process::exit(self.into())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ExitCode {}

#[cfg(feature = "std")]
impl std::process::Termination for ExitCode {
    #[inline]
    fn report(self) -> std::process::ExitCode {
        u8::from(self).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code() {
        assert_eq!(ExitCode::Ok as u8, 0);
        assert_eq!(ExitCode::Usage as u8, 64);
        assert_eq!(ExitCode::DataErr as u8, 65);
        assert_eq!(ExitCode::NoInput as u8, 66);
        assert_eq!(ExitCode::NoUser as u8, 67);
        assert_eq!(ExitCode::NoHost as u8, 68);
        assert_eq!(ExitCode::Unavailable as u8, 69);
        assert_eq!(ExitCode::Software as u8, 70);
        assert_eq!(ExitCode::OsErr as u8, 71);
        assert_eq!(ExitCode::OsFile as u8, 72);
        assert_eq!(ExitCode::CantCreat as u8, 73);
        assert_eq!(ExitCode::IoErr as u8, 74);
        assert_eq!(ExitCode::TempFail as u8, 75);
        assert_eq!(ExitCode::Protocol as u8, 76);
        assert_eq!(ExitCode::NoPerm as u8, 77);
        assert_eq!(ExitCode::Config as u8, 78);
    }

    #[test]
    fn clone() {
        assert_eq!(ExitCode::Ok.clone(), ExitCode::Ok);
    }

    #[test]
    fn copy() {
        let a = ExitCode::Ok;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn default() {
        assert_eq!(ExitCode::default(), ExitCode::Ok);
    }

    #[test]
    #[allow(clippy::cognitive_complexity, clippy::too_many_lines)]
    fn equality() {
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
    const fn is_success_is_const_fn() {
        const _: bool = ExitCode::Ok.is_success();
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

    #[test]
    const fn is_failure_is_const_fn() {
        const _: bool = ExitCode::Ok.is_failure();
    }

    #[cfg(feature = "std")]
    #[test]
    fn source() {
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
    fn report() {
        use std::process::Termination;

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
