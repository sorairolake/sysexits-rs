// SPDX-FileCopyrightText: 2022 Kevin Matthes
// SPDX-FileCopyrightText: 2022 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for formatting and printing [`ExitCode`].

use core::fmt;

use super::ExitCode;

impl fmt::Display for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sysexits::ExitCode;
    ///
    /// assert_eq!(format!("{}", ExitCode::Ok), "0");
    /// assert_eq!(format!("{}", ExitCode::Usage), "64");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug() {
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

    #[test]
    fn display() {
        assert_eq!(format!("{}", ExitCode::Ok), "0");
        assert_eq!(format!("{}", ExitCode::Usage), "64");
        assert_eq!(format!("{}", ExitCode::DataErr), "65");
        assert_eq!(format!("{}", ExitCode::NoInput), "66");
        assert_eq!(format!("{}", ExitCode::NoUser), "67");
        assert_eq!(format!("{}", ExitCode::NoHost), "68");
        assert_eq!(format!("{}", ExitCode::Unavailable), "69");
        assert_eq!(format!("{}", ExitCode::Software), "70");
        assert_eq!(format!("{}", ExitCode::OsErr), "71");
        assert_eq!(format!("{}", ExitCode::OsFile), "72");
        assert_eq!(format!("{}", ExitCode::CantCreat), "73");
        assert_eq!(format!("{}", ExitCode::IoErr), "74");
        assert_eq!(format!("{}", ExitCode::TempFail), "75");
        assert_eq!(format!("{}", ExitCode::Protocol), "76");
        assert_eq!(format!("{}", ExitCode::NoPerm), "77");
        assert_eq!(format!("{}", ExitCode::Config), "78");
    }
}
