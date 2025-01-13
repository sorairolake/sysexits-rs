// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constants for [`ExitCode`].

use super::ExitCode;

impl ExitCode {
    /// The base value for `ExitCode`.
    ///
    /// See [`sysexits.h(3head)`] for details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::BASE, ExitCode::Usage);
    /// ```
    ///
    /// [`sysexits.h(3head)`]: https://man7.org/linux/man-pages/man3/sysexits.h.3head.html
    pub const BASE: Self = Self::Usage;

    /// The maximum value for `ExitCode`.
    ///
    /// See [`sysexits.h(3head)`] for details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sysexits::ExitCode;
    /// #
    /// assert_eq!(ExitCode::MAX, ExitCode::Config);
    /// ```
    ///
    /// [`sysexits.h(3head)`]: https://man7.org/linux/man-pages/man3/sysexits.h.3head.html
    pub const MAX: Self = Self::Config;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(ExitCode::BASE, ExitCode::Usage);
    }

    #[test]
    fn max() {
        assert_eq!(ExitCode::MAX, ExitCode::Config);
    }
}
