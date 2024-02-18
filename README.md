<!--
SPDX-FileCopyrightText: 2022 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# sysexits-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**sysexits-rs** ([`sysexits`][version-url]) is a library that provides the
system exit code constants as defined by [`<sysexits.h>`].

This library implements the [`Termination`] trait, so this can be returned from
the `main` function.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sysexits = "0.7.11"
```

### Example

```rust
use std::str;

use sysexits::ExitCode;

fn main() -> ExitCode {
    let bytes = [0xf0, 0x9f, 0x92, 0x96];
    match str::from_utf8(&bytes) {
        Ok(string) => {
            println!("{string}");
            ExitCode::Ok
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::DataErr
        }
    }
}
```

### Crate features

#### `std`

Enables features that depend on the standard library. This is enabled by
default.

#### `nightly`

Enables features that depend on the nightly Rust.

#### `extended_io_error`

Enables features that depend on the `io_error_more` feature. This also enables
`std`. This is implied by `nightly`.

### `no_std` support

This supports `no_std` mode. Disables the `default` feature to enable this.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.61.0.

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## License

Copyright &copy; 2022&ndash;2024 Shun Sakai and other contributors (see
[AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.0 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/sysexits-rs/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/sysexits-rs/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/sysexits?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/sysexits
[docs-badge]: https://img.shields.io/docsrs/sysexits?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/sysexits
[license-badge]: https://img.shields.io/crates/l/sysexits?style=for-the-badge
[`<sysexits.h>`]: https://man.openbsd.org/sysexits
[`Termination`]: https://doc.rust-lang.org/std/process/trait.Termination.html
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
