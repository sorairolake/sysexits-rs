// SPDX-FileCopyrightText: 2023 Kevin Matthes
// SPDX-FileCopyrightText: 2023 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error handling with the [`Result`] type based on [`ExitCode`].

use super::ExitCode;

/// A [`Result`](std::result::Result) type based on [`ExitCode`].
///
/// In case of an error, an appropriate variant of [`ExitCode`] can describe the
/// exact cause in further detail.
pub type Result<T> = std::result::Result<T, ExitCode>;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
