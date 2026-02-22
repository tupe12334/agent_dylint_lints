---
displayNumber: 7
status: in-progress
priority: 2
createdAt: 2026-02-22T15:47:59.780996+00:00
updatedAt: 2026-02-22T15:49:32.814302+00:00
---

# Pre-push hook: enforce 100% test coverage with cargo-llvm-cov

## Overview

Add a pre-push hook that measures test coverage and fails if it falls below 100%.

## Implementation

Install `cargo-llvm-cov`:

```sh
cargo install cargo-llvm-cov
```

Add to `.husky/pre-push`:

```sh
cargo llvm-cov --fail-under-lines 100
```

## Notes

- Uses LLVM source-based coverage instrumentation (more accurate than `grcov`)
- `--fail-under-lines 100` exits non-zero if line coverage < 100%
- Uses the pinned nightly toolchain from `rust-toolchain.toml`
- Add `cargo-llvm-cov` to the development prerequisites in `README.md`
- Uncoverable lines (e.g. `unreachable!`) can be excluded with `#[cfg(not(coverage))]` or `// coverage:ignore`

## Acceptance Criteria

- Hook blocks push when line coverage is below 100%
- Hook passes when all reachable lines are exercised by tests
