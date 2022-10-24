//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai and Contributors
//

//! The system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

use std::{
    fmt,
    process::{ExitCode as StdExitCode, Termination},
};

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
}

impl fmt::Display for ExitCode {
    /// Implements the `Display` trait.
    ///
    /// `sysexits::ExitCode` implements the `Display` trait such that it can be
    /// formatted using the given formatter.  Thereby, the respective variant
    /// will be casted to its integer representation `u8`, at first, before
    /// being processed by the given formatter.
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
        write!(f, "{}", *self as u8)
    }
}

impl From<ExitCode> for StdExitCode {
    #[inline]
    fn from(code: ExitCode) -> Self {
        code.report()
    }
}

impl Termination for ExitCode {
    #[inline]
    fn report(self) -> StdExitCode {
        StdExitCode::from(self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_from_sys_exits_to_exit_code() {
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Ok)),
            format!("{:?}", StdExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Usage)),
            format!("{:?}", StdExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::DataErr)),
            format!("{:?}", StdExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::NoInput)),
            format!("{:?}", StdExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::NoUser)),
            format!("{:?}", StdExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::NoHost)),
            format!("{:?}", StdExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Unavailable)),
            format!("{:?}", StdExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Software)),
            format!("{:?}", StdExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::OsErr)),
            format!("{:?}", StdExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::OsFile)),
            format!("{:?}", StdExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::CantCreat)),
            format!("{:?}", StdExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::IoErr)),
            format!("{:?}", StdExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::TempFail)),
            format!("{:?}", StdExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Protocol)),
            format!("{:?}", StdExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::NoPerm)),
            format!("{:?}", StdExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", StdExitCode::from(ExitCode::Config)),
            format!("{:?}", StdExitCode::from(78))
        );
    }

    #[test]
    fn test_report_status_code() {
        assert_eq!(
            format!("{:?}", ExitCode::Ok.report()),
            format!("{:?}", StdExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Usage.report()),
            format!("{:?}", StdExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", ExitCode::DataErr.report()),
            format!("{:?}", StdExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoInput.report()),
            format!("{:?}", StdExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoUser.report()),
            format!("{:?}", StdExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoHost.report()),
            format!("{:?}", StdExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Unavailable.report()),
            format!("{:?}", StdExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Software.report()),
            format!("{:?}", StdExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", ExitCode::OsErr.report()),
            format!("{:?}", StdExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", ExitCode::OsFile.report()),
            format!("{:?}", StdExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", ExitCode::CantCreat.report()),
            format!("{:?}", StdExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", ExitCode::IoErr.report()),
            format!("{:?}", StdExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", ExitCode::TempFail.report()),
            format!("{:?}", StdExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Protocol.report()),
            format!("{:?}", StdExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", ExitCode::NoPerm.report()),
            format!("{:?}", StdExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", ExitCode::Config.report()),
            format!("{:?}", StdExitCode::from(78))
        );
    }
}
