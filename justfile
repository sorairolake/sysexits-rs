# SPDX-FileCopyrightText: 2022-2023 Shun Sakai and Contributors
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

alias all := default
alias lint := clippy

# Run default recipe
default: build

# Build a package
@build:
    cargo build

# Remove generated artifacts
@clean:
    cargo clean

# Check a package
@check:
    cargo check

# Run tests
@test:
    cargo test

# Run the formatter
@fmt:
    cargo fmt

# Run the formatter with options
@fmt-with-options:
    cargo fmt -- --config "format_code_in_doc_comments=true,wrap_comments=true"

# Run the linter
@clippy:
    cargo clippy -- -D warnings

# Apply lint suggestions
@clippy-fix:
    cargo clippy --fix --allow-dirty --allow-staged --allow-no-vcs -- -D warnings

# Run the linter for GitHub Actions workflow files
@lint-github-actions:
    actionlint

# Run the formatter for the README
@fmt-readme:
    npx prettier -w README.md

# Increment the version
@bump part:
    bump2version {{part}}
