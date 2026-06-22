# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Three MUD game examples demonstrating all 75 branchless pattern kernels in narrative game contexts:
  - `dungeon_crawler` — roguelike dungeon crawler; exercises 75/75 patterns, emits receipt chain
  - `space_station` — sci-fi space station defense; exercises 75/75 patterns, anti-cheat PASS, quality gate PASS
  - `siege_defense` — castle siege battle log; exercises all patterns across 8-link receipt chain

## [26.6.13] - 2026-06-13

### Added

- 75 branchless pattern kernels covering core algorithmic primitives
- C-ABI staticlib (`cdylib`/`staticlib`) for FFI consumption
- `wasm32-unknown-unknown` build target with verified portability
- GitHub Actions CI pipeline (lint, test, build, audit on every push)
- `cargo-deny` for license and supply chain policy enforcement
- `cargo audit` integration for CVE scanning of dependencies
- GGEN-only covenant: all generated source is first-class; no hand-edited generated files
- Portability proof: identical behaviour verified across `x86_64`, `aarch64`, and `wasm32` targets

[Unreleased]: https://github.com/sac/wasm4games/compare/v26.6.13...HEAD
[26.6.13]: https://github.com/sac/wasm4games/releases/tag/v26.6.13
