//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

//! The system exit code constants as defined by
//! [`<sysexits.h>`][sysexits-man-url].
//!
//! [sysexits-man-url]: https://man.openbsd.org/sysexits

use std::process::{ExitCode, Termination};

/// `SysExits` is a type that represents the system exit code constants as
/// defined by [`<sysexits.h>`][sysexits-man-url].
///
/// [sysexits-man-url]: https://man.openbsd.org/sysexits
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum SysExits {
    /// The successful exit.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Ok as u8, 0);
    /// ```
    Ok,

    /// The command was used incorrectly, e.g., with the wrong number of
    /// arguments, a bad flag, bad syntax in a parameter, or whatever.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Usage as u8, 64);
    /// ```
    Usage = 64,

    /// The input data was incorrect in some way.
    /// This should only be used for user's data and not system files.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::DataErr as u8, 65);
    /// ```
    DataErr,

    /// An input file (not a system file) did not exist or was not readable.
    /// This could also include errors like "No message" to a mailer (if it
    /// cared to catch it).
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::NoInput as u8, 66);
    /// ```
    NoInput,

    /// The user specified did not exist.
    /// This might be used for mail addresses or remote logins.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::NoUser as u8, 67);
    /// ```
    NoUser,

    /// The host specified did not exist.
    /// This is used in mail addresses or network requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::NoHost as u8, 68);
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
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Unavailable as u8, 69);
    /// ```
    Unavailable,

    /// An internal software error has been detected.
    /// This should be limited to non-operating system related errors if
    /// possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Software as u8, 70);
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
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::OsErr as u8, 71);
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
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::OsFile as u8, 72);
    /// ```
    OsFile,

    /// A (user specified) output file cannot be created.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::CantCreat as u8, 73);
    /// ```
    CantCreat,

    /// An error occurred while doing I/O on some file.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::IoErr as u8, 74);
    /// ```
    IoErr,

    /// Temporary failure, indicating something that is not really an error.
    /// For example that a mailer could not create a connection, and the request
    /// should be reattempted later.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::TempFail as u8, 75);
    /// ```
    TempFail,

    /// The remote system returned something that was "not possible" during a
    /// protocol exchange.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Protocol as u8, 76);
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
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::NoPerm as u8, 77);
    /// ```
    NoPerm,

    /// Something was found in an unconfigured or misconfigured state.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Config as u8, 78);
    /// ```
    Config,
}

impl SysExits {
    /// Returns `true` if this system exit code represents successful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Ok.is_success(), true);
    /// assert_eq!(SysExits::Usage.is_success(), false);
    /// ```
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// Returns `true` if this system exit code represents unsuccessful
    /// termination.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::SysExits;
    ///
    /// assert_eq!(SysExits::Ok.is_failure(), false);
    /// assert_eq!(SysExits::Usage.is_failure(), true);
    /// ```
    #[must_use]
    pub const fn is_failure(self) -> bool {
        !self.is_success()
    }
}

impl From<SysExits> for ExitCode {
    fn from(code: SysExits) -> Self {
        code.report()
    }
}

impl Termination for SysExits {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_success_for_successful_termination() {
        assert!(SysExits::Ok.is_success());
    }

    #[test]
    fn test_is_success_for_unsuccessful_termination() {
        assert!(!SysExits::Usage.is_success());
        assert!(!SysExits::DataErr.is_success());
        assert!(!SysExits::NoInput.is_success());
        assert!(!SysExits::NoUser.is_success());
        assert!(!SysExits::NoHost.is_success());
        assert!(!SysExits::Unavailable.is_success());
        assert!(!SysExits::Software.is_success());
        assert!(!SysExits::OsErr.is_success());
        assert!(!SysExits::OsFile.is_success());
        assert!(!SysExits::CantCreat.is_success());
        assert!(!SysExits::IoErr.is_success());
        assert!(!SysExits::TempFail.is_success());
        assert!(!SysExits::Protocol.is_success());
        assert!(!SysExits::NoPerm.is_success());
        assert!(!SysExits::Config.is_success());
    }

    #[test]
    fn test_is_failure_for_successful_termination() {
        assert!(!SysExits::Ok.is_failure());
    }

    #[test]
    fn test_is_failure_for_unsuccessful_termination() {
        assert!(SysExits::Usage.is_failure());
        assert!(SysExits::DataErr.is_failure());
        assert!(SysExits::NoInput.is_failure());
        assert!(SysExits::NoUser.is_failure());
        assert!(SysExits::NoHost.is_failure());
        assert!(SysExits::Unavailable.is_failure());
        assert!(SysExits::Software.is_failure());
        assert!(SysExits::OsErr.is_failure());
        assert!(SysExits::OsFile.is_failure());
        assert!(SysExits::CantCreat.is_failure());
        assert!(SysExits::IoErr.is_failure());
        assert!(SysExits::TempFail.is_failure());
        assert!(SysExits::Protocol.is_failure());
        assert!(SysExits::NoPerm.is_failure());
        assert!(SysExits::Config.is_failure());
    }

    #[test]
    fn test_from_sys_exits_to_exit_code() {
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Ok)),
            format!("{:?}", ExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Usage)),
            format!("{:?}", ExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::DataErr)),
            format!("{:?}", ExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::NoInput)),
            format!("{:?}", ExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::NoUser)),
            format!("{:?}", ExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::NoHost)),
            format!("{:?}", ExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Unavailable)),
            format!("{:?}", ExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Software)),
            format!("{:?}", ExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::OsErr)),
            format!("{:?}", ExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::OsFile)),
            format!("{:?}", ExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::CantCreat)),
            format!("{:?}", ExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::IoErr)),
            format!("{:?}", ExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::TempFail)),
            format!("{:?}", ExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Protocol)),
            format!("{:?}", ExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::NoPerm)),
            format!("{:?}", ExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", ExitCode::from(SysExits::Config)),
            format!("{:?}", ExitCode::from(78))
        );
    }

    #[test]
    fn test_report_status_code() {
        assert_eq!(
            format!("{:?}", SysExits::Ok.report()),
            format!("{:?}", ExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", SysExits::Usage.report()),
            format!("{:?}", ExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", SysExits::DataErr.report()),
            format!("{:?}", ExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", SysExits::NoInput.report()),
            format!("{:?}", ExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", SysExits::NoUser.report()),
            format!("{:?}", ExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", SysExits::NoHost.report()),
            format!("{:?}", ExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", SysExits::Unavailable.report()),
            format!("{:?}", ExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", SysExits::Software.report()),
            format!("{:?}", ExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", SysExits::OsErr.report()),
            format!("{:?}", ExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", SysExits::OsFile.report()),
            format!("{:?}", ExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", SysExits::CantCreat.report()),
            format!("{:?}", ExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", SysExits::IoErr.report()),
            format!("{:?}", ExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", SysExits::TempFail.report()),
            format!("{:?}", ExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", SysExits::Protocol.report()),
            format!("{:?}", ExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", SysExits::NoPerm.report()),
            format!("{:?}", ExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", SysExits::Config.report()),
            format!("{:?}", ExitCode::from(78))
        );
    }
}
