---
displayNumber: 93
status: open
priority: 2
createdAt: 2026-02-22T17:00:00.000000+00:00
updatedAt: 2026-02-22T17:00:00.000000+00:00
---

# Implement clippy::needless_raw_string_hashes lint

## Overview

Implement the [`clippy::needless_raw_string_hashes`](https://rust-lang.github.io/rust-clippy/master/index.html#needless_raw_string_hashes) pedantic lint as a Dylint lint.

## Tasks

- Study the clippy reference implementation for `clippy::needless_raw_string_hashes`
- Implement the lint in a new Dylint lint crate (or extend an existing one)
- Write UI tests covering the lint's detection and allowed cases
- Ensure the lint passes `cargo clippy`, `cargo fmt`, and `cargo test`

## Notes

- Lint group: **pedantic**
- Clippy reference: <https://rust-lang.github.io/rust-clippy/master/index.html#needless_raw_string_hashes>

## Acceptance Criteria

- Lint correctly detects the pattern described by `clippy::needless_raw_string_hashes`
- False positives are handled (lint does not fire on allowed code)
- UI test `.stderr` file matches actual output
