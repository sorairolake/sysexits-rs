// SPDX-FileCopyrightText: 2023 Kevin Matthes
// SPDX-FileCopyrightText: 2023 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for this crate.

use core::fmt;

/// The error type indicating that [`ExitCode`](crate::ExitCode) was out of
/// range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ExitCodeRangeError;

impl fmt::Display for ExitCodeRangeError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value is out of range for `ExitCode`")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ExitCodeRangeError {}

#[cfg(feature = "std")]
/// An error which can be returned when converting an
/// [`ExitCode`](crate::ExitCode) from an
/// [`ExitStatus`](std::process::ExitStatus).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct TryFromExitStatusError(Option<i32>);

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
impl fmt::Display for TryFromExitStatusError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(f, "invalid exit code `{code}`")
        } else {
            write!(f, "exit code is unknown")
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromExitStatusError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_exit_code_range_error() {
        assert_eq!(ExitCodeRangeError.clone(), ExitCodeRangeError);
    }

    #[test]
    fn copy_exit_code_range_error() {
        let a = ExitCodeRangeError;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn debug_exit_code_range_error() {
        assert_eq!(format!("{ExitCodeRangeError:?}"), "ExitCodeRangeError");
    }

    #[test]
    fn exit_code_range_error_equality() {
        assert_eq!(ExitCodeRangeError, ExitCodeRangeError);
    }

    #[test]
    fn display_exit_code_range_error() {
        assert_eq!(
            format!("{ExitCodeRangeError}"),
            "value is out of range for `ExitCode`"
        );
    }

    #[cfg(feature = "std")]
    #[test]
    fn source_exit_code_range_error() {
        use std::error::Error;

        assert!(ExitCodeRangeError.source().is_none());
    }

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
    #[test]
    fn code_try_from_exit_status_error() {
        assert_eq!(TryFromExitStatusError::new(Some(1)).code(), Some(1));
        assert_eq!(TryFromExitStatusError::new(None).code(), None);
    }

    #[cfg(feature = "std")]
    #[test]
    const fn code_try_from_exit_status_error_is_const_fn() {
        const _: Option<i32> = TryFromExitStatusError::new(None).code();
    }

    #[cfg(feature = "std")]
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

    #[cfg(feature = "std")]
    #[test]
    fn source_try_from_exit_status_error() {
        use std::error::Error;

        assert!(TryFromExitStatusError::new(Some(1)).source().is_none());
        assert!(TryFromExitStatusError::new(None).source().is_none());
    }
}
