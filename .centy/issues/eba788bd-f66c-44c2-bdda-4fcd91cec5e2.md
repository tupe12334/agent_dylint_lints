---
displayNumber: 15
status: open
priority: 2
createdAt: 2026-02-22T17:00:00.000000+00:00
updatedAt: 2026-02-22T17:00:00.000000+00:00
---

# Implement clippy::cast_precision_loss lint

## Overview

Implement the [`clippy::cast_precision_loss`](https://rust-lang.github.io/rust-clippy/master/index.html#cast_precision_loss) pedantic lint as a Dylint lint.

## Tasks

- Study the clippy reference implementation for `clippy::cast_precision_loss`
- Implement the lint in a new Dylint lint crate (or extend an existing one)
- Write UI tests covering the lint's detection and allowed cases
- Ensure the lint passes `cargo clippy`, `cargo fmt`, and `cargo test`

## Notes

- Lint group: **pedantic**
- Clippy reference: <https://rust-lang.github.io/rust-clippy/master/index.html#cast_precision_loss>

## Acceptance Criteria

- Lint correctly detects the pattern described by `clippy::cast_precision_loss`
- False positives are handled (lint does not fire on allowed code)
- UI test `.stderr` file matches actual output
