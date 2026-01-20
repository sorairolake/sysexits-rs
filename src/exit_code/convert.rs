// SPDX-FileCopyrightText: 2022 Shun Sakai
// SPDX-FileCopyrightText: 2023 Kevin Matthes
// SPDX-FileCopyrightText: 2023 zSchoen
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Implementations of conversions between [`ExitCode`] and other types.

#[cfg(feature = "std")]
use std::{
    io,
    process::{self, ExitStatus, Termination},
};

use super::ExitCode;
#[cfg(feature = "std")]
use crate::error::TryFromExitStatusError;

impl From<ExitCode> for u8 {
    /// Converts an `ExitCode` into the raw underlying integer value.
    ///
    /// The resulting value is `0` or `64..=78`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(u8::from(ExitCode::Ok), 0);
    /// assert_eq!(u8::from(ExitCode::Usage), 64);
    /// ```
    fn from(code: ExitCode) -> Self {
        code as Self
    }
}

#[cfg(feature = "std")]
impl From<ExitCode> for process::ExitCode {
    /// Converts an `sysexits::ExitCode` into an [`process::ExitCode`].
    fn from(code: ExitCode) -> Self {
        code.report()
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for ExitCode {
    /// Converts an [`io::Error`] into an `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// #
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(
    ///     ExitCode::from(io::Error::from(io::ErrorKind::NotFound)),
    ///     ExitCode::NoInput
    /// );
    /// ```
    fn from(error: io::Error) -> Self {
        error.kind().into()
    }
}

#[cfg(feature = "std")]
impl From<io::ErrorKind> for ExitCode {
    /// Converts an [`io::ErrorKind`] into an `ExitCode`.
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
    fn from(kind: io::ErrorKind) -> Self {
        match kind {
            io::ErrorKind::NotFound => Self::NoInput,
            io::ErrorKind::PermissionDenied => Self::NoPerm,
            io::ErrorKind::ConnectionRefused | io::ErrorKind::OutOfMemory => Self::OsErr,
            io::ErrorKind::ConnectionReset
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::NotConnected
            | io::ErrorKind::BrokenPipe
            | io::ErrorKind::TimedOut
            | io::ErrorKind::Interrupted => Self::TempFail,
            io::ErrorKind::HostUnreachable | io::ErrorKind::NetworkUnreachable => Self::NoHost,
            io::ErrorKind::AddrInUse
            | io::ErrorKind::AddrNotAvailable
            | io::ErrorKind::NetworkDown => Self::Unavailable,
            io::ErrorKind::AlreadyExists | io::ErrorKind::ReadOnlyFilesystem => Self::CantCreat,
            io::ErrorKind::WouldBlock | io::ErrorKind::Unsupported => Self::Protocol,
            io::ErrorKind::InvalidInput | io::ErrorKind::InvalidData => Self::DataErr,
            io::ErrorKind::WriteZero | io::ErrorKind::UnexpectedEof => Self::Software,
            _ => Self::IoErr,
        }
    }
}

#[cfg(feature = "std")]
impl TryFrom<ExitStatus> for ExitCode {
    type Error = TryFromExitStatusError;

    /// Converts an [`ExitStatus`] into an `ExitCode`.
    ///
    /// # Errors
    ///
    /// Returns [`Err`] if any of the following are true:
    ///
    /// - The exit code is not `0` or `64..=78`.
    /// - The exit code is unknown (e.g., the process was terminated by a
    ///   signal).
    fn try_from(status: ExitStatus) -> Result<Self, Self::Error> {
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
    #[cfg(feature = "std")]
    use std::process::Command;
    #[cfg(all(feature = "std", unix))]
    use std::process::Stdio;

    use super::*;

    #[cfg(all(feature = "std", unix))]
    fn get_exit_status(status: i32) -> ExitStatus {
        Command::new("sh")
            .arg("-c")
            .arg(format!("exit {status}"))
            .status()
            .unwrap()
    }

    #[cfg(all(feature = "std", windows))]
    fn get_exit_status(status: u32) -> ExitStatus {
        Command::new("cmd")
            .arg("/c")
            .arg(format!("exit {status}"))
            .status()
            .unwrap()
    }

    #[test]
    fn from_exit_code_to_u8() {
        assert_eq!(u8::from(ExitCode::Ok), 0);
        assert_eq!(u8::from(ExitCode::Usage), 64);
        assert_eq!(u8::from(ExitCode::DataErr), 65);
        assert_eq!(u8::from(ExitCode::NoInput), 66);
        assert_eq!(u8::from(ExitCode::NoUser), 67);
        assert_eq!(u8::from(ExitCode::NoHost), 68);
        assert_eq!(u8::from(ExitCode::Unavailable), 69);
        assert_eq!(u8::from(ExitCode::Software), 70);
        assert_eq!(u8::from(ExitCode::OsErr), 71);
        assert_eq!(u8::from(ExitCode::OsFile), 72);
        assert_eq!(u8::from(ExitCode::CantCreat), 73);
        assert_eq!(u8::from(ExitCode::IoErr), 74);
        assert_eq!(u8::from(ExitCode::TempFail), 75);
        assert_eq!(u8::from(ExitCode::Protocol), 76);
        assert_eq!(u8::from(ExitCode::NoPerm), 77);
        assert_eq!(u8::from(ExitCode::Config), 78);
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_exit_code_to_process_exit_code() {
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Ok)),
            format!("{:?}", process::ExitCode::from(0))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Usage)),
            format!("{:?}", process::ExitCode::from(64))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::DataErr)),
            format!("{:?}", process::ExitCode::from(65))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::NoInput)),
            format!("{:?}", process::ExitCode::from(66))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::NoUser)),
            format!("{:?}", process::ExitCode::from(67))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::NoHost)),
            format!("{:?}", process::ExitCode::from(68))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Unavailable)),
            format!("{:?}", process::ExitCode::from(69))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Software)),
            format!("{:?}", process::ExitCode::from(70))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::OsErr)),
            format!("{:?}", process::ExitCode::from(71))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::OsFile)),
            format!("{:?}", process::ExitCode::from(72))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::CantCreat)),
            format!("{:?}", process::ExitCode::from(73))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::IoErr)),
            format!("{:?}", process::ExitCode::from(74))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::TempFail)),
            format!("{:?}", process::ExitCode::from(75))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Protocol)),
            format!("{:?}", process::ExitCode::from(76))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::NoPerm)),
            format!("{:?}", process::ExitCode::from(77))
        );
        assert_eq!(
            format!("{:?}", process::ExitCode::from(ExitCode::Config)),
            format!("{:?}", process::ExitCode::from(78))
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_io_error_to_exit_code() {
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NotFound)),
            ExitCode::NoInput
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::PermissionDenied)),
            ExitCode::NoPerm
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ConnectionRefused)),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ConnectionReset)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::HostUnreachable)),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NetworkUnreachable)),
            ExitCode::NoHost
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ConnectionAborted)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NotConnected)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::AddrInUse)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::AddrNotAvailable)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NetworkDown)),
            ExitCode::Unavailable
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::BrokenPipe)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::AlreadyExists)),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::WouldBlock)),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NotADirectory)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::IsADirectory)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::DirectoryNotEmpty)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ReadOnlyFilesystem)),
            ExitCode::CantCreat
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::StaleNetworkFileHandle)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::InvalidInput)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::InvalidData)),
            ExitCode::DataErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::TimedOut)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::WriteZero)),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::StorageFull)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::NotSeekable)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::QuotaExceeded)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::FileTooLarge)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ResourceBusy)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ExecutableFileBusy)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::Deadlock)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::CrossesDevices)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::TooManyLinks)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::ArgumentListTooLong)),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::Interrupted)),
            ExitCode::TempFail
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::Unsupported)),
            ExitCode::Protocol
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::UnexpectedEof)),
            ExitCode::Software
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::OutOfMemory)),
            ExitCode::OsErr
        );
        assert_eq!(
            ExitCode::from(io::Error::from(io::ErrorKind::Other)),
            ExitCode::IoErr
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn from_io_error_kind_to_exit_code() {
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
            ExitCode::from(io::ErrorKind::HostUnreachable),
            ExitCode::NoHost
        );
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
        assert_eq!(
            ExitCode::from(io::ErrorKind::NotADirectory),
            ExitCode::IoErr
        );
        assert_eq!(ExitCode::from(io::ErrorKind::IsADirectory), ExitCode::IoErr);
        assert_eq!(
            ExitCode::from(io::ErrorKind::DirectoryNotEmpty),
            ExitCode::IoErr
        );
        assert_eq!(
            ExitCode::from(io::ErrorKind::ReadOnlyFilesystem),
            ExitCode::CantCreat
        );
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
        assert_eq!(ExitCode::from(io::ErrorKind::TimedOut), ExitCode::TempFail);
        assert_eq!(ExitCode::from(io::ErrorKind::WriteZero), ExitCode::Software);
        assert_eq!(ExitCode::from(io::ErrorKind::StorageFull), ExitCode::IoErr);
        assert_eq!(ExitCode::from(io::ErrorKind::NotSeekable), ExitCode::IoErr);
        assert_eq!(
            ExitCode::from(io::ErrorKind::QuotaExceeded),
            ExitCode::IoErr
        );
        assert_eq!(ExitCode::from(io::ErrorKind::FileTooLarge), ExitCode::IoErr);
        assert_eq!(ExitCode::from(io::ErrorKind::ResourceBusy), ExitCode::IoErr);
        assert_eq!(
            ExitCode::from(io::ErrorKind::ExecutableFileBusy),
            ExitCode::IoErr
        );
        assert_eq!(ExitCode::from(io::ErrorKind::Deadlock), ExitCode::IoErr);
        assert_eq!(
            ExitCode::from(io::ErrorKind::CrossesDevices),
            ExitCode::IoErr
        );
        assert_eq!(ExitCode::from(io::ErrorKind::TooManyLinks), ExitCode::IoErr);
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
        fn get_exit_status() -> ExitStatus {
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
