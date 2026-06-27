# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-06-27

### Added

- DocsDog specification v0.1.0 defining the core metadata model: directed
  relationships between source code and external artifacts (`source → predicate → target`).
- JSON schemas for scan documents and relationships (`scan.schema.json`,
  `relationship.schema.json`, `scan-example.json`).
- DocsDog CLI (`docsdog-cli/`) for scaffolding and generating documentation
  templates (requirements, use cases, ADRs, user stories) with `init` and `make`
  subcommands.
- MkDocs site with Material theme (`mkdocs/`) and Nix package for building the
  documentation site.
- Nix flake with dev shell providing Rust toolchain (`cargo`, `rustc`,
  `rustfmt`, `clippy`, `rust-analyzer`) and MkDocs with plugins.
- Template variants for all four document types: full (with guidance), bare
  (placeholders only), and minimal (fewer sections, where applicable).
- Project mascot (`assets/docdoc.png`).

### Changed

- Namespace renamed from `docdog`/`docref` to `docsdog`.
- PHP implementation removed from this repository; the spec and CLI are now the
  only artifacts in this repo.
- Documentation restructured: spec moved to `docsdog-spec/`, CLI to
  `docsdog-cli/`, and mkdocs site to `mkdocs/`.
- MkDocs now copies spec files and assets at build time via `mkdocs-gen-files`
  instead of relying on symlinks.

### Fixed

- Prevented copy error when Nix resolves committed symlinks in the store.
- Image path fixes for the mkdocs documentation build.
