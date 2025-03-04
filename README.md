<!--
SPDX-FileCopyrightText: 2022 Shun Sakai

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# sysexits-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
![MSRV][msrv-badge]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**sysexits-rs** ([`sysexits`][version-url]) is a library that provides the
system exit code constants as defined by [`<sysexits.h>`].

This library implements the [`Termination`] trait, so this can be returned from
the `main` function.

## Usage

Run the following command in your project directory:

```sh
cargo add sysexits
```

### Crate features

#### `extended_io_error`

Enables features that depend on the `io_error_inprogress` and the
`io_error_more` features. This also enables `std`. This is implied by `nightly`.

#### `nightly`

Enables features that depend on the nightly Rust.

#### `std`

Enables features that depend on the standard library. This is enabled by
default.

### `no_std` support

This supports `no_std` mode. Disables the `default` feature to enable this.

### Documentation

See the [documentation][docs-url] for more details.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.85.0.

## Source code

The upstream repository is available at
<https://github.com/sorairolake/sysexits-rs.git>.

The source code is also available at:

- <https://gitlab.com/sorairolake/sysexits-rs.git>
- <https://codeberg.org/sorairolake/sysexits-rs.git>

## Changelog

Please see [CHANGELOG.adoc].

## Contributing

Please see [CONTRIBUTING.adoc].

## Similar projects

- <https://github.com/sorairolake/Sysexits.jl> (Julia)
- <https://github.com/sorairolake/sysexits-zig> (Zig)

You can discover more projects at
<https://github.com/sorairolake/awesome-sysexits>.

## License

Copyright (C) 2022 Shun Sakai and other contributors (see [AUTHORS.adoc])

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

This project is compliant with version 3.2 of the [_REUSE Specification_]. See
copyright notices of individual files for more details on copyright and
licensing information.

[ci-badge]: https://img.shields.io/github/actions/workflow/status/sorairolake/sysexits-rs/CI.yaml?branch=develop&style=for-the-badge&logo=github&label=CI
[ci-url]: https://github.com/sorairolake/sysexits-rs/actions?query=branch%3Adevelop+workflow%3ACI++
[version-badge]: https://img.shields.io/crates/v/sysexits?style=for-the-badge&logo=rust
[version-url]: https://crates.io/crates/sysexits
[msrv-badge]: https://img.shields.io/crates/msrv/sysexits?style=for-the-badge&logo=rust
[docs-badge]: https://img.shields.io/docsrs/sysexits?style=for-the-badge&logo=docsdotrs&label=Docs.rs
[docs-url]: https://docs.rs/sysexits
[license-badge]: https://img.shields.io/crates/l/sysexits?style=for-the-badge
[`<sysexits.h>`]: https://man.openbsd.org/sysexits
[`Termination`]: https://doc.rust-lang.org/std/process/trait.Termination.html
[CHANGELOG.adoc]: CHANGELOG.adoc
[CONTRIBUTING.adoc]: CONTRIBUTING.adoc
[AUTHORS.adoc]: AUTHORS.adoc
[_REUSE Specification_]: https://reuse.software/spec/
