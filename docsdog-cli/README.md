# DocsDog CLI

DocsDog CLI scaffolds and generates project documentation templates following the
docsdog specification. It handles both index documents (the container catalogs)
and individual tracked documents (requirements, use cases, ADRs, user stories).

## Quick Start

```bash
# Scaffold the docs/ directory with index documents
docsdog init

# Generate individual tracked documents
docsdog make req      # → docs/req/REQ-001.md
docsdog make uc       # → docs/uc/UC-001.md
docsdog make adr      # → docs/adr/ADR-001.md
docsdog make us       # → docs/us/US-001.md
```

## Directory Structure

After `docsdog init`, the following is created under `./docs/` in the current
working directory:

```
docs/
├── adr/
│   └── adrs.md          # ADR Log (index)
├── req/
│   └── srs.md           # Software Requirements Specification (index)
├── uc/
│   └── ucs.md           # Use Case Specification (index)
└── us/
    └── uss.md           # User Story Specification (index)
```

Individual documents land next to their index:

```
docs/
├── adr/
│   ├── adrs.md          # index
│   ├── ADR-001.md       # tracked
│   └── ADR-002.md
├── req/
│   ├── srs.md
│   ├── REQ-001.md
│   └── REQ-002.md
├── uc/
│   ├── ucs.md
│   └── UC-001.md
└── us/
    ├── uss.md
    └── US-001.md
```

## Filename Numbering

Tracked documents follow the pattern `PREFIX-NNN.md` where `NNN` is a
zero-padded, auto-incrementing number:

| Command | Prefix | Example |
|---------|--------|---------|
| `make adr` | `ADR` | `ADR-001.md` |
| `make req` | `REQ` | `REQ-001.md` |
| `make uc`  | `UC`  | `UC-001.md` |
| `make us`  | `US`  | `US-001.md` |

The CLI scans the target directory, finds the highest existing number, and
emits the next one. Gaps in the sequence are left unfilled (deleting file 001
does not cause the next generated file to reuse that number).

## CLI Reference

### `docsdog init`

```
docsdog init [--bare]
```

Creates the `docs/` directory with four subdirectories (`adr`, `req`, `uc`,
`us`) and one index document per directory.

| Flag | Effect |
|------|--------|
| (none) | Full index templates with guidance comments (💬) and tips (💡) |
| `--bare` | Bare index templates — placeholders only, no guidance |

### `docsdog make`

```
docsdog make <TYPE> [--bare] [--minimal]
```

Generates a new tracked document of the given type with an auto-incremented ID.

`<TYPE>` is one of: `adr`, `req`, `uc`, `us`.

| Flag | Effect |
|------|--------|
| (none) | Full template with guidance comments and tips |
| `--bare` | Bare template — placeholders only |
| `--minimal` | Minimal template — fewer/simpler sections |

Both flags can be combined (`--bare --minimal`).

### Template Variants Per Type

Not every type has every variant. The table below shows which flags are valid
for each type:

| Command | `--bare` | `--minimal` | `--bare --minimal` |
|---------|----------|-------------|---------------------|
| `make adr` | ✓ | ✓ — fewer sections | ✓ — bare-minimal |
| `make req` | ✓ | ✗ | ✗ |
| `make uc`  | ✓ | ✓ — UML Simplified instead of Cockburn | ✓ — bare-minimal |
| `make us`  | ✓ | ✗ | ✗ |

Passing `--minimal` on a type that has no minimal template prints an error
message and exits with code 1 without generating a file:

```
Error: --minimal is not available for 'req'. No minimal template exists.
```

### Template Semantics

| Variant | What it provides |
|---------|-----------------|
| Full _(no flags)_ | Complete template with section descriptions (💬), writing tips (💡), example values, and structured placeholders. Best for complex or high-impact documents. |
| Bare (`--bare`) | Same sections as the full template but with empty placeholders and minimal inline comments. For quick drafting when you know what to write. |
| Minimal (`--minimal`) | Fewer sections than the full template (e.g., Cockburn → UML Simplified for use cases). For lower-risk or well-understood documents. |
| Bare-minimal (`--bare --minimal`) | Fewer sections, empty placeholders, no guidance. For rapid authoring of straightforward documents. |

## Build

```bash
cargo build --release
```

The binary embeds all template files at compile time — no runtime file-system
dependency on the templates directory.
