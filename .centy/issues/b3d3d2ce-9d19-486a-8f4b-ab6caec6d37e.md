---
displayNumber: 134
status: open
priority: 2
createdAt: 2026-02-22T17:00:00.000000+00:00
updatedAt: 2026-02-22T17:00:00.000000+00:00
---

# Implement clippy::unnecessary_literal_bound lint

## Overview

Implement the [`clippy::unnecessary_literal_bound`](https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_literal_bound) pedantic lint as a Dylint lint.

## Tasks

- Study the clippy reference implementation for `clippy::unnecessary_literal_bound`
- Implement the lint in a new Dylint lint crate (or extend an existing one)
- Write UI tests covering the lint's detection and allowed cases
- Ensure the lint passes `cargo clippy`, `cargo fmt`, and `cargo test`

## Notes

- Lint group: **pedantic**
- Clippy reference: <https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_literal_bound>

## Acceptance Criteria

- Lint correctly detects the pattern described by `clippy::unnecessary_literal_bound`
- False positives are handled (lint does not fire on allowed code)
- UI test `.stderr` file matches actual output
