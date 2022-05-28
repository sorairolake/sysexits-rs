//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

#![doc(html_root_url = "https://docs.rs/sysexits/0.1.0/")]
// Lint levels of rustc.
#![warn(rust_2018_idioms)]
#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]
// Lint levels of Clippy.
#![warn(clippy::cargo, clippy::nursery, clippy::pedantic)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
