// SPDX-FileCopyrightText: 2023 Shun Sakai and other contributors
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for this crate.

use std::fmt;

/// An error which can be returned when converting an
/// [`ExitCode`](crate::ExitCode) from an
/// [`ExitStatus`](std::process::ExitStatus).
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TryFromExitStatusError(Option<i32>);

impl TryFromExitStatusError {
    #[inline]
    pub(crate) const fn new(code: Option<i32>) -> Self {
        Self(code)
    }

    /// Returns the corresponding exit code for this error.
    ///
    /// Returns [`None`] if the process was terminated by a signal.
    #[must_use]
    #[inline]
    pub const fn code(&self) -> Option<i32> {
        self.0
    }
}

impl fmt::Display for TryFromExitStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "invalid exit code `{code}`")
        } else {
            write!(f, "exit code is unknown")
        }
    }
}

impl std::error::Error for TryFromExitStatusError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_try_from_exit_status_error() {
        assert_eq!(
            TryFromExitStatusError::new(Some(1)).clone(),
            TryFromExitStatusError::new(Some(1))
        );
        assert_eq!(
            TryFromExitStatusError::new(None).clone(),
            TryFromExitStatusError::new(None)
        );
    }

    #[test]
    fn copy_try_from_exit_status_error() {
        {
            let a = TryFromExitStatusError::new(Some(1));
            let b = a;
            assert_eq!(a, b);
        }
        {
            let a = TryFromExitStatusError::new(None);
            let b = a;
            assert_eq!(a, b);
        }
    }

    #[test]
    fn debug_try_from_exit_status_error() {
        assert_eq!(
            format!("{:?}", TryFromExitStatusError::new(Some(1))),
            "TryFromExitStatusError(Some(1))"
        );
        assert_eq!(
            format!("{:?}", TryFromExitStatusError::new(None)),
            "TryFromExitStatusError(None)"
        );
    }

    #[test]
    fn try_from_exit_status_error_equality() {
        assert_eq!(
            TryFromExitStatusError::new(Some(1)),
            TryFromExitStatusError::new(Some(1))
        );
        assert_ne!(
            TryFromExitStatusError::new(Some(1)),
            TryFromExitStatusError::new(None)
        );
        assert_ne!(
            TryFromExitStatusError::new(None),
            TryFromExitStatusError::new(Some(1))
        );
        assert_eq!(
            TryFromExitStatusError::new(None),
            TryFromExitStatusError::new(None)
        );
    }

    #[test]
    fn code_try_from_exit_status_error() {
        assert_eq!(TryFromExitStatusError::new(Some(1)).code(), Some(1));
        assert_eq!(TryFromExitStatusError::new(None).code(), None);
    }

    #[test]
    fn display_try_from_exit_status_error() {
        assert_eq!(
            format!("{}", TryFromExitStatusError::new(Some(1))),
            "invalid exit code `1`"
        );
        assert_eq!(
            format!("{}", TryFromExitStatusError::new(None)),
            "exit code is unknown"
        );
    }

    #[test]
    fn source_try_from_exit_status_error() {
        use std::error::Error;

        assert!(TryFromExitStatusError::new(Some(1)).source().is_none());
        assert!(TryFromExitStatusError::new(None).source().is_none());
    }
}
