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

impl fmt::Octal for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:#o}", ExitCode::Ok), "0o0");
    /// assert_eq!(format!("{:o}", ExitCode::Usage), "100");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl fmt::LowerHex for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:#x}", ExitCode::Ok), "0x0");
    /// assert_eq!(format!("{:x}", ExitCode::Usage), "40");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl fmt::UpperHex for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:#X}", ExitCode::Ok), "0x0");
    /// assert_eq!(format!("{:X}", ExitCode::Usage), "40");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl fmt::Binary for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:#b}", ExitCode::Ok), "0b0");
    /// assert_eq!(format!("{:b}", ExitCode::Usage), "1000000");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl fmt::LowerExp for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:e}", ExitCode::Ok), "0e0");
    /// assert_eq!(format!("{:e}", ExitCode::Usage), "6.4e1");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        u8::from(*self).fmt(f)
    }
}

impl fmt::UpperExp for ExitCode {
    /// Shows the integer representation of this `ExitCode`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(format!("{:E}", ExitCode::Ok), "0E0");
    /// assert_eq!(format!("{:E}", ExitCode::Usage), "6.4E1");
    /// ```
    #[inline]
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

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn octal() {
        assert_eq!(format!("{:o}", ExitCode::Ok), "0");
        assert_eq!(format!("{:#o}", ExitCode::Ok), "0o0");
        assert_eq!(format!("{:03o}", ExitCode::Ok), "000");
        assert_eq!(format!("{:#05o}", ExitCode::Ok), "0o000");
        assert_eq!(format!("{:o}", ExitCode::Usage), "100");
        assert_eq!(format!("{:#o}", ExitCode::Usage), "0o100");
        assert_eq!(format!("{:03o}", ExitCode::Usage), "100");
        assert_eq!(format!("{:#05o}", ExitCode::Usage), "0o100");
        assert_eq!(format!("{:o}", ExitCode::DataErr), "101");
        assert_eq!(format!("{:#o}", ExitCode::DataErr), "0o101");
        assert_eq!(format!("{:03o}", ExitCode::DataErr), "101");
        assert_eq!(format!("{:#05o}", ExitCode::DataErr), "0o101");
        assert_eq!(format!("{:o}", ExitCode::NoInput), "102");
        assert_eq!(format!("{:#o}", ExitCode::NoInput), "0o102");
        assert_eq!(format!("{:03o}", ExitCode::NoInput), "102");
        assert_eq!(format!("{:#05o}", ExitCode::NoInput), "0o102");
        assert_eq!(format!("{:o}", ExitCode::NoUser), "103");
        assert_eq!(format!("{:#o}", ExitCode::NoUser), "0o103");
        assert_eq!(format!("{:03o}", ExitCode::NoUser), "103");
        assert_eq!(format!("{:#05o}", ExitCode::NoUser), "0o103");
        assert_eq!(format!("{:o}", ExitCode::NoHost), "104");
        assert_eq!(format!("{:#o}", ExitCode::NoHost), "0o104");
        assert_eq!(format!("{:03o}", ExitCode::NoHost), "104");
        assert_eq!(format!("{:#05o}", ExitCode::NoHost), "0o104");
        assert_eq!(format!("{:o}", ExitCode::Unavailable), "105");
        assert_eq!(format!("{:#o}", ExitCode::Unavailable), "0o105");
        assert_eq!(format!("{:03o}", ExitCode::Unavailable), "105");
        assert_eq!(format!("{:#05o}", ExitCode::Unavailable), "0o105");
        assert_eq!(format!("{:o}", ExitCode::Software), "106");
        assert_eq!(format!("{:#o}", ExitCode::Software), "0o106");
        assert_eq!(format!("{:03o}", ExitCode::Software), "106");
        assert_eq!(format!("{:#05o}", ExitCode::Software), "0o106");
        assert_eq!(format!("{:o}", ExitCode::OsErr), "107");
        assert_eq!(format!("{:#o}", ExitCode::OsErr), "0o107");
        assert_eq!(format!("{:03o}", ExitCode::OsErr), "107");
        assert_eq!(format!("{:#05o}", ExitCode::OsErr), "0o107");
        assert_eq!(format!("{:o}", ExitCode::OsFile), "110");
        assert_eq!(format!("{:#o}", ExitCode::OsFile), "0o110");
        assert_eq!(format!("{:03o}", ExitCode::OsFile), "110");
        assert_eq!(format!("{:#05o}", ExitCode::OsFile), "0o110");
        assert_eq!(format!("{:o}", ExitCode::CantCreat), "111");
        assert_eq!(format!("{:#o}", ExitCode::CantCreat), "0o111");
        assert_eq!(format!("{:03o}", ExitCode::CantCreat), "111");
        assert_eq!(format!("{:#05o}", ExitCode::CantCreat), "0o111");
        assert_eq!(format!("{:o}", ExitCode::IoErr), "112");
        assert_eq!(format!("{:#o}", ExitCode::IoErr), "0o112");
        assert_eq!(format!("{:03o}", ExitCode::IoErr), "112");
        assert_eq!(format!("{:#05o}", ExitCode::IoErr), "0o112");
        assert_eq!(format!("{:o}", ExitCode::TempFail), "113");
        assert_eq!(format!("{:#o}", ExitCode::TempFail), "0o113");
        assert_eq!(format!("{:03o}", ExitCode::TempFail), "113");
        assert_eq!(format!("{:#05o}", ExitCode::TempFail), "0o113");
        assert_eq!(format!("{:o}", ExitCode::Protocol), "114");
        assert_eq!(format!("{:#o}", ExitCode::Protocol), "0o114");
        assert_eq!(format!("{:03o}", ExitCode::Protocol), "114");
        assert_eq!(format!("{:#05o}", ExitCode::Protocol), "0o114");
        assert_eq!(format!("{:o}", ExitCode::NoPerm), "115");
        assert_eq!(format!("{:#o}", ExitCode::NoPerm), "0o115");
        assert_eq!(format!("{:03o}", ExitCode::NoPerm), "115");
        assert_eq!(format!("{:#05o}", ExitCode::NoPerm), "0o115");
        assert_eq!(format!("{:o}", ExitCode::Config), "116");
        assert_eq!(format!("{:#o}", ExitCode::Config), "0o116");
        assert_eq!(format!("{:03o}", ExitCode::Config), "116");
        assert_eq!(format!("{:#05o}", ExitCode::Config), "0o116");
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn lower_hex() {
        assert_eq!(format!("{:x}", ExitCode::Ok), "0");
        assert_eq!(format!("{:#x}", ExitCode::Ok), "0x0");
        assert_eq!(format!("{:02x}", ExitCode::Ok), "00");
        assert_eq!(format!("{:#04x}", ExitCode::Ok), "0x00");
        assert_eq!(format!("{:x}", ExitCode::Usage), "40");
        assert_eq!(format!("{:#x}", ExitCode::Usage), "0x40");
        assert_eq!(format!("{:02x}", ExitCode::Usage), "40");
        assert_eq!(format!("{:#04x}", ExitCode::Usage), "0x40");
        assert_eq!(format!("{:x}", ExitCode::DataErr), "41");
        assert_eq!(format!("{:#x}", ExitCode::DataErr), "0x41");
        assert_eq!(format!("{:02x}", ExitCode::DataErr), "41");
        assert_eq!(format!("{:#04x}", ExitCode::DataErr), "0x41");
        assert_eq!(format!("{:x}", ExitCode::NoInput), "42");
        assert_eq!(format!("{:#x}", ExitCode::NoInput), "0x42");
        assert_eq!(format!("{:02x}", ExitCode::NoInput), "42");
        assert_eq!(format!("{:#04x}", ExitCode::NoInput), "0x42");
        assert_eq!(format!("{:x}", ExitCode::NoUser), "43");
        assert_eq!(format!("{:#x}", ExitCode::NoUser), "0x43");
        assert_eq!(format!("{:02x}", ExitCode::NoUser), "43");
        assert_eq!(format!("{:#04x}", ExitCode::NoUser), "0x43");
        assert_eq!(format!("{:x}", ExitCode::NoHost), "44");
        assert_eq!(format!("{:#x}", ExitCode::NoHost), "0x44");
        assert_eq!(format!("{:02x}", ExitCode::NoHost), "44");
        assert_eq!(format!("{:#04x}", ExitCode::NoHost), "0x44");
        assert_eq!(format!("{:x}", ExitCode::Unavailable), "45");
        assert_eq!(format!("{:#x}", ExitCode::Unavailable), "0x45");
        assert_eq!(format!("{:02x}", ExitCode::Unavailable), "45");
        assert_eq!(format!("{:#04x}", ExitCode::Unavailable), "0x45");
        assert_eq!(format!("{:x}", ExitCode::Software), "46");
        assert_eq!(format!("{:#x}", ExitCode::Software), "0x46");
        assert_eq!(format!("{:02x}", ExitCode::Software), "46");
        assert_eq!(format!("{:#04x}", ExitCode::Software), "0x46");
        assert_eq!(format!("{:x}", ExitCode::OsErr), "47");
        assert_eq!(format!("{:#x}", ExitCode::OsErr), "0x47");
        assert_eq!(format!("{:02x}", ExitCode::OsErr), "47");
        assert_eq!(format!("{:#04x}", ExitCode::OsErr), "0x47");
        assert_eq!(format!("{:x}", ExitCode::OsFile), "48");
        assert_eq!(format!("{:#x}", ExitCode::OsFile), "0x48");
        assert_eq!(format!("{:02x}", ExitCode::OsFile), "48");
        assert_eq!(format!("{:#04x}", ExitCode::OsFile), "0x48");
        assert_eq!(format!("{:x}", ExitCode::CantCreat), "49");
        assert_eq!(format!("{:#x}", ExitCode::CantCreat), "0x49");
        assert_eq!(format!("{:02x}", ExitCode::CantCreat), "49");
        assert_eq!(format!("{:#04x}", ExitCode::CantCreat), "0x49");
        assert_eq!(format!("{:x}", ExitCode::IoErr), "4a");
        assert_eq!(format!("{:#x}", ExitCode::IoErr), "0x4a");
        assert_eq!(format!("{:02x}", ExitCode::IoErr), "4a");
        assert_eq!(format!("{:#04x}", ExitCode::IoErr), "0x4a");
        assert_eq!(format!("{:x}", ExitCode::TempFail), "4b");
        assert_eq!(format!("{:#x}", ExitCode::TempFail), "0x4b");
        assert_eq!(format!("{:02x}", ExitCode::TempFail), "4b");
        assert_eq!(format!("{:#04x}", ExitCode::TempFail), "0x4b");
        assert_eq!(format!("{:x}", ExitCode::Protocol), "4c");
        assert_eq!(format!("{:#x}", ExitCode::Protocol), "0x4c");
        assert_eq!(format!("{:02x}", ExitCode::Protocol), "4c");
        assert_eq!(format!("{:#04x}", ExitCode::Protocol), "0x4c");
        assert_eq!(format!("{:x}", ExitCode::NoPerm), "4d");
        assert_eq!(format!("{:#x}", ExitCode::NoPerm), "0x4d");
        assert_eq!(format!("{:02x}", ExitCode::NoPerm), "4d");
        assert_eq!(format!("{:#04x}", ExitCode::NoPerm), "0x4d");
        assert_eq!(format!("{:x}", ExitCode::Config), "4e");
        assert_eq!(format!("{:#x}", ExitCode::Config), "0x4e");
        assert_eq!(format!("{:02x}", ExitCode::Config), "4e");
        assert_eq!(format!("{:#04x}", ExitCode::Config), "0x4e");
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn upper_hex() {
        assert_eq!(format!("{:X}", ExitCode::Ok), "0");
        assert_eq!(format!("{:#X}", ExitCode::Ok), "0x0");
        assert_eq!(format!("{:02X}", ExitCode::Ok), "00");
        assert_eq!(format!("{:#04X}", ExitCode::Ok), "0x00");
        assert_eq!(format!("{:X}", ExitCode::Usage), "40");
        assert_eq!(format!("{:#X}", ExitCode::Usage), "0x40");
        assert_eq!(format!("{:02X}", ExitCode::Usage), "40");
        assert_eq!(format!("{:#04X}", ExitCode::Usage), "0x40");
        assert_eq!(format!("{:X}", ExitCode::DataErr), "41");
        assert_eq!(format!("{:#X}", ExitCode::DataErr), "0x41");
        assert_eq!(format!("{:02X}", ExitCode::DataErr), "41");
        assert_eq!(format!("{:#04X}", ExitCode::DataErr), "0x41");
        assert_eq!(format!("{:X}", ExitCode::NoInput), "42");
        assert_eq!(format!("{:#X}", ExitCode::NoInput), "0x42");
        assert_eq!(format!("{:02X}", ExitCode::NoInput), "42");
        assert_eq!(format!("{:#04X}", ExitCode::NoInput), "0x42");
        assert_eq!(format!("{:X}", ExitCode::NoUser), "43");
        assert_eq!(format!("{:#X}", ExitCode::NoUser), "0x43");
        assert_eq!(format!("{:02X}", ExitCode::NoUser), "43");
        assert_eq!(format!("{:#04X}", ExitCode::NoUser), "0x43");
        assert_eq!(format!("{:X}", ExitCode::NoHost), "44");
        assert_eq!(format!("{:#X}", ExitCode::NoHost), "0x44");
        assert_eq!(format!("{:02X}", ExitCode::NoHost), "44");
        assert_eq!(format!("{:#04X}", ExitCode::NoHost), "0x44");
        assert_eq!(format!("{:X}", ExitCode::Unavailable), "45");
        assert_eq!(format!("{:#X}", ExitCode::Unavailable), "0x45");
        assert_eq!(format!("{:02X}", ExitCode::Unavailable), "45");
        assert_eq!(format!("{:#04X}", ExitCode::Unavailable), "0x45");
        assert_eq!(format!("{:X}", ExitCode::Software), "46");
        assert_eq!(format!("{:#X}", ExitCode::Software), "0x46");
        assert_eq!(format!("{:02X}", ExitCode::Software), "46");
        assert_eq!(format!("{:#04X}", ExitCode::Software), "0x46");
        assert_eq!(format!("{:X}", ExitCode::OsErr), "47");
        assert_eq!(format!("{:#X}", ExitCode::OsErr), "0x47");
        assert_eq!(format!("{:02X}", ExitCode::OsErr), "47");
        assert_eq!(format!("{:#04X}", ExitCode::OsErr), "0x47");
        assert_eq!(format!("{:X}", ExitCode::OsFile), "48");
        assert_eq!(format!("{:#X}", ExitCode::OsFile), "0x48");
        assert_eq!(format!("{:02X}", ExitCode::OsFile), "48");
        assert_eq!(format!("{:#04X}", ExitCode::OsFile), "0x48");
        assert_eq!(format!("{:X}", ExitCode::CantCreat), "49");
        assert_eq!(format!("{:#X}", ExitCode::CantCreat), "0x49");
        assert_eq!(format!("{:02X}", ExitCode::CantCreat), "49");
        assert_eq!(format!("{:#04X}", ExitCode::CantCreat), "0x49");
        assert_eq!(format!("{:X}", ExitCode::IoErr), "4A");
        assert_eq!(format!("{:#X}", ExitCode::IoErr), "0x4A");
        assert_eq!(format!("{:02X}", ExitCode::IoErr), "4A");
        assert_eq!(format!("{:#04X}", ExitCode::IoErr), "0x4A");
        assert_eq!(format!("{:X}", ExitCode::TempFail), "4B");
        assert_eq!(format!("{:#X}", ExitCode::TempFail), "0x4B");
        assert_eq!(format!("{:02X}", ExitCode::TempFail), "4B");
        assert_eq!(format!("{:#04X}", ExitCode::TempFail), "0x4B");
        assert_eq!(format!("{:X}", ExitCode::Protocol), "4C");
        assert_eq!(format!("{:#X}", ExitCode::Protocol), "0x4C");
        assert_eq!(format!("{:02X}", ExitCode::Protocol), "4C");
        assert_eq!(format!("{:#04X}", ExitCode::Protocol), "0x4C");
        assert_eq!(format!("{:X}", ExitCode::NoPerm), "4D");
        assert_eq!(format!("{:#X}", ExitCode::NoPerm), "0x4D");
        assert_eq!(format!("{:02X}", ExitCode::NoPerm), "4D");
        assert_eq!(format!("{:#04X}", ExitCode::NoPerm), "0x4D");
        assert_eq!(format!("{:X}", ExitCode::Config), "4E");
        assert_eq!(format!("{:#X}", ExitCode::Config), "0x4E");
        assert_eq!(format!("{:02X}", ExitCode::Config), "4E");
        assert_eq!(format!("{:#04X}", ExitCode::Config), "0x4E");
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn binary() {
        assert_eq!(format!("{:b}", ExitCode::Ok), "0");
        assert_eq!(format!("{:#b}", ExitCode::Ok), "0b0");
        assert_eq!(format!("{:07b}", ExitCode::Ok), "0000000");
        assert_eq!(format!("{:#09b}", ExitCode::Ok), "0b0000000");
        assert_eq!(format!("{:b}", ExitCode::Usage), "1000000");
        assert_eq!(format!("{:#b}", ExitCode::Usage), "0b1000000");
        assert_eq!(format!("{:07b}", ExitCode::Usage), "1000000");
        assert_eq!(format!("{:#09b}", ExitCode::Usage), "0b1000000");
        assert_eq!(format!("{:b}", ExitCode::DataErr), "1000001");
        assert_eq!(format!("{:#b}", ExitCode::DataErr), "0b1000001");
        assert_eq!(format!("{:07b}", ExitCode::DataErr), "1000001");
        assert_eq!(format!("{:#09b}", ExitCode::DataErr), "0b1000001");
        assert_eq!(format!("{:b}", ExitCode::NoInput), "1000010");
        assert_eq!(format!("{:#b}", ExitCode::NoInput), "0b1000010");
        assert_eq!(format!("{:07b}", ExitCode::NoInput), "1000010");
        assert_eq!(format!("{:#09b}", ExitCode::NoInput), "0b1000010");
        assert_eq!(format!("{:b}", ExitCode::NoUser), "1000011");
        assert_eq!(format!("{:#b}", ExitCode::NoUser), "0b1000011");
        assert_eq!(format!("{:07b}", ExitCode::NoUser), "1000011");
        assert_eq!(format!("{:#09b}", ExitCode::NoUser), "0b1000011");
        assert_eq!(format!("{:b}", ExitCode::NoHost), "1000100");
        assert_eq!(format!("{:#b}", ExitCode::NoHost), "0b1000100");
        assert_eq!(format!("{:07b}", ExitCode::NoHost), "1000100");
        assert_eq!(format!("{:#09b}", ExitCode::NoHost), "0b1000100");
        assert_eq!(format!("{:b}", ExitCode::Unavailable), "1000101");
        assert_eq!(format!("{:#b}", ExitCode::Unavailable), "0b1000101");
        assert_eq!(format!("{:07b}", ExitCode::Unavailable), "1000101");
        assert_eq!(format!("{:#09b}", ExitCode::Unavailable), "0b1000101");
        assert_eq!(format!("{:b}", ExitCode::Software), "1000110");
        assert_eq!(format!("{:#b}", ExitCode::Software), "0b1000110");
        assert_eq!(format!("{:07b}", ExitCode::Software), "1000110");
        assert_eq!(format!("{:#09b}", ExitCode::Software), "0b1000110");
        assert_eq!(format!("{:b}", ExitCode::OsErr), "1000111");
        assert_eq!(format!("{:#b}", ExitCode::OsErr), "0b1000111");
        assert_eq!(format!("{:07b}", ExitCode::OsErr), "1000111");
        assert_eq!(format!("{:#09b}", ExitCode::OsErr), "0b1000111");
        assert_eq!(format!("{:b}", ExitCode::OsFile), "1001000");
        assert_eq!(format!("{:#b}", ExitCode::OsFile), "0b1001000");
        assert_eq!(format!("{:07b}", ExitCode::OsFile), "1001000");
        assert_eq!(format!("{:#09b}", ExitCode::OsFile), "0b1001000");
        assert_eq!(format!("{:b}", ExitCode::CantCreat), "1001001");
        assert_eq!(format!("{:#b}", ExitCode::CantCreat), "0b1001001");
        assert_eq!(format!("{:07b}", ExitCode::CantCreat), "1001001");
        assert_eq!(format!("{:#09b}", ExitCode::CantCreat), "0b1001001");
        assert_eq!(format!("{:b}", ExitCode::IoErr), "1001010");
        assert_eq!(format!("{:#b}", ExitCode::IoErr), "0b1001010");
        assert_eq!(format!("{:07b}", ExitCode::IoErr), "1001010");
        assert_eq!(format!("{:#09b}", ExitCode::IoErr), "0b1001010");
        assert_eq!(format!("{:b}", ExitCode::TempFail), "1001011");
        assert_eq!(format!("{:#b}", ExitCode::TempFail), "0b1001011");
        assert_eq!(format!("{:07b}", ExitCode::TempFail), "1001011");
        assert_eq!(format!("{:#09b}", ExitCode::TempFail), "0b1001011");
        assert_eq!(format!("{:b}", ExitCode::Protocol), "1001100");
        assert_eq!(format!("{:#b}", ExitCode::Protocol), "0b1001100");
        assert_eq!(format!("{:07b}", ExitCode::Protocol), "1001100");
        assert_eq!(format!("{:#09b}", ExitCode::Protocol), "0b1001100");
        assert_eq!(format!("{:b}", ExitCode::NoPerm), "1001101");
        assert_eq!(format!("{:#b}", ExitCode::NoPerm), "0b1001101");
        assert_eq!(format!("{:07b}", ExitCode::NoPerm), "1001101");
        assert_eq!(format!("{:#09b}", ExitCode::NoPerm), "0b1001101");
        assert_eq!(format!("{:b}", ExitCode::Config), "1001110");
        assert_eq!(format!("{:#b}", ExitCode::Config), "0b1001110");
        assert_eq!(format!("{:07b}", ExitCode::Config), "1001110");
        assert_eq!(format!("{:#09b}", ExitCode::Config), "0b1001110");
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn lower_exp() {
        assert_eq!(format!("{:e}", ExitCode::Ok), "0e0");
        assert_eq!(format!("{:05e}", ExitCode::Ok), "000e0");
        assert_eq!(format!("{:e}", ExitCode::Usage), "6.4e1");
        assert_eq!(format!("{:05e}", ExitCode::Usage), "6.4e1");
        assert_eq!(format!("{:e}", ExitCode::DataErr), "6.5e1");
        assert_eq!(format!("{:05e}", ExitCode::DataErr), "6.5e1");
        assert_eq!(format!("{:e}", ExitCode::NoInput), "6.6e1");
        assert_eq!(format!("{:05e}", ExitCode::NoInput), "6.6e1");
        assert_eq!(format!("{:e}", ExitCode::NoUser), "6.7e1");
        assert_eq!(format!("{:05e}", ExitCode::NoUser), "6.7e1");
        assert_eq!(format!("{:e}", ExitCode::NoHost), "6.8e1");
        assert_eq!(format!("{:05e}", ExitCode::NoHost), "6.8e1");
        assert_eq!(format!("{:e}", ExitCode::Unavailable), "6.9e1");
        assert_eq!(format!("{:05e}", ExitCode::Unavailable), "6.9e1");
        assert_eq!(format!("{:e}", ExitCode::Software), "7e1");
        assert_eq!(format!("{:05e}", ExitCode::Software), "007e1");
        assert_eq!(format!("{:e}", ExitCode::OsErr), "7.1e1");
        assert_eq!(format!("{:05e}", ExitCode::OsErr), "7.1e1");
        assert_eq!(format!("{:e}", ExitCode::OsFile), "7.2e1");
        assert_eq!(format!("{:05e}", ExitCode::OsFile), "7.2e1");
        assert_eq!(format!("{:e}", ExitCode::CantCreat), "7.3e1");
        assert_eq!(format!("{:05e}", ExitCode::CantCreat), "7.3e1");
        assert_eq!(format!("{:e}", ExitCode::IoErr), "7.4e1");
        assert_eq!(format!("{:05e}", ExitCode::IoErr), "7.4e1");
        assert_eq!(format!("{:e}", ExitCode::TempFail), "7.5e1");
        assert_eq!(format!("{:05e}", ExitCode::TempFail), "7.5e1");
        assert_eq!(format!("{:e}", ExitCode::Protocol), "7.6e1");
        assert_eq!(format!("{:05e}", ExitCode::Protocol), "7.6e1");
        assert_eq!(format!("{:e}", ExitCode::NoPerm), "7.7e1");
        assert_eq!(format!("{:05e}", ExitCode::NoPerm), "7.7e1");
        assert_eq!(format!("{:e}", ExitCode::Config), "7.8e1");
        assert_eq!(format!("{:05e}", ExitCode::Config), "7.8e1");
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn upper_exp() {
        assert_eq!(format!("{:E}", ExitCode::Ok), "0E0");
        assert_eq!(format!("{:05E}", ExitCode::Ok), "000E0");
        assert_eq!(format!("{:E}", ExitCode::Usage), "6.4E1");
        assert_eq!(format!("{:05E}", ExitCode::Usage), "6.4E1");
        assert_eq!(format!("{:E}", ExitCode::DataErr), "6.5E1");
        assert_eq!(format!("{:05E}", ExitCode::DataErr), "6.5E1");
        assert_eq!(format!("{:E}", ExitCode::NoInput), "6.6E1");
        assert_eq!(format!("{:05E}", ExitCode::NoInput), "6.6E1");
        assert_eq!(format!("{:E}", ExitCode::NoUser), "6.7E1");
        assert_eq!(format!("{:05E}", ExitCode::NoUser), "6.7E1");
        assert_eq!(format!("{:E}", ExitCode::NoHost), "6.8E1");
        assert_eq!(format!("{:05E}", ExitCode::NoHost), "6.8E1");
        assert_eq!(format!("{:E}", ExitCode::Unavailable), "6.9E1");
        assert_eq!(format!("{:05E}", ExitCode::Unavailable), "6.9E1");
        assert_eq!(format!("{:E}", ExitCode::Software), "7E1");
        assert_eq!(format!("{:05E}", ExitCode::Software), "007E1");
        assert_eq!(format!("{:E}", ExitCode::OsErr), "7.1E1");
        assert_eq!(format!("{:05E}", ExitCode::OsErr), "7.1E1");
        assert_eq!(format!("{:E}", ExitCode::OsFile), "7.2E1");
        assert_eq!(format!("{:05E}", ExitCode::OsFile), "7.2E1");
        assert_eq!(format!("{:E}", ExitCode::CantCreat), "7.3E1");
        assert_eq!(format!("{:05E}", ExitCode::CantCreat), "7.3E1");
        assert_eq!(format!("{:E}", ExitCode::IoErr), "7.4E1");
        assert_eq!(format!("{:05E}", ExitCode::IoErr), "7.4E1");
        assert_eq!(format!("{:E}", ExitCode::TempFail), "7.5E1");
        assert_eq!(format!("{:05E}", ExitCode::TempFail), "7.5E1");
        assert_eq!(format!("{:E}", ExitCode::Protocol), "7.6E1");
        assert_eq!(format!("{:05E}", ExitCode::Protocol), "7.6E1");
        assert_eq!(format!("{:E}", ExitCode::NoPerm), "7.7E1");
        assert_eq!(format!("{:05E}", ExitCode::NoPerm), "7.7E1");
        assert_eq!(format!("{:E}", ExitCode::Config), "7.8E1");
        assert_eq!(format!("{:05E}", ExitCode::Config), "7.8E1");
    }
}
