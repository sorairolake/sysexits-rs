# sysexits-rs

[![CI][ci-badge]][ci-url]
[![Version][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
![License][license-badge]

**sysexits-rs** ([`sysexits`][version-url]) is a library that provides the
system exit code constants as defined by [`sysexits.h`][sysexits-man-url].

This library implements the [`Termination`][termination-docs-url] trait, so
this can be returned from the `main` function.

## Minimum supported Rust version

The minimum supported Rust version (MSRV) of this library is v1.61.0 or later.

## Changelog

Please see [CHANGELOG.adoc](CHANGELOG.adoc).

## Contributing

Please see [CONTRIBUTING.adoc](CONTRIBUTING.adoc).

## License

Copyright (C) 2022 Shun Sakai (see [AUTHORS.adoc](AUTHORS.adoc))

This library is distributed under the terms of either the _Apache License 2.0_
or the _MIT License_.

See [COPYING](COPYING), [APACHE-2.0](license/APACHE-2.0) and [MIT](license/MIT)
for more details.

[ci-badge]: https://github.com/sorairolake/sysexits-rs/workflows/CI/badge.svg
[ci-url]: https://github.com/sorairolake/sysexits-rs/actions?query=workflow%3ACI
[version-badge]: https://img.shields.io/crates/v/sysexits-rs
[version-url]: https://crates.io/crates/sysexits-rs
[docs-badge]: https://img.shields.io/docsrs/sysexits-rs
[docs-url]: https://docs.rs/sysexits-rs
[license-badge]: https://img.shields.io/crates/l/sysexits-rs
[sysexits-man-url]: https://man.openbsd.org/sysexits
[termination-docs-url]: https://doc.rust-lang.org/std/process/trait.Termination.html
