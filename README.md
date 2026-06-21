# DocsDog

![docdoc](/assets/docdoc.png)

DocsDog is an open specification for expressing semantic relationships between
source code and external artifacts.

It defines a language-independent metadata model that enables traceability
across software systems, documentation, architecture, requirements, APIs,
infrastructure, and other engineering artifacts.

## Core Model

Every DocsDog annotation represents a directed relationship:

```
Source ──Predicate──▶ Target
```

- **Source**: a code element (class, method, function, interface, trait, enum, module, …).
- **Predicate**: the semantic meaning of the link.
- **Target**: an external artifact identified by a URI-like string.

## Identifier Format

Targets use a three-part URI-like syntax:

```
<namespace>:<kind>:<identifier>
```

| Component  | Description                   |
| ---------- | ----------------------------- |
| namespace  | Registry owner                |
| kind       | Artifact category             |
| identifier | Namespace-specific identifier |

The specification reserves the **`docsdog`** namespace with standard artifact kinds
(see the [full specification](docsdog-spec/specification.md)).

External namespaces are also supported: `jira:issue:ERP-123`,
`github:repo:company/project#15`, `openapi:spec:billing:v1`, etc.

## Standard Predicates

| Category        | Predicates                                            |
|-----------------|-------------------------------------------------------|
| Traceability    | `implements`, `traces-to`, `requires`, `validates`, `tests` |
| Architecture    | `depends-on`, `owned-by`, `decision`, `replaces`, `deprecated-by` |
| Messaging       | `emits`, `consumes`                                   |
| Persistence     | `persists`, `maps-to`                                 |
| API             | `exposes`                                             |
| Security        | `authenticated-by`, `authorized-by`, `secured-by`     |
| Configuration   | `configured-by`, `feature-flag`                       |

Custom predicates are supported. Unknown predicates MUST be accepted by
compliant implementations.

## Optional Metadata

Each relationship may carry arbitrary key-value metadata:

```json
{
  "predicate": "requires",
  "target": "docsdog:requirement:REQ-014",
  "metadata": {
    "since": "2.1",
    "critical": true,
    "tags": ["payments", "security"]
  }
}
```

## Design Principles

- **Language Agnostic** — independent of any programming language.
- **Storage Agnostic** — targets identify artifacts, not locations.
- **Semantic** — relationships describe meaning, not implementation.
- **Extensible** — new predicates and namespaces without modifying the spec.
- **Minimal** — only the minimum to express a relationship.
- **Non-Intrusive** — must never alter program behavior.

## Implementations

| Language | Repo                                                         |
|----------|--------------------------------------------------------------|
| PHP      | [Hugo0Vaz/docsdog-php](https://github.com/Hugo0Vaz/docsdog-php) |

Language-specific docs (installation, annotation syntax, CLI usage) live in each
implementation's own repository.

## Specification

The formal specification is at [docsdog-spec/specification.md](docsdog-spec/specification.md).

JSON schemas:

- [Scan document](docsdog-spec/scan.schema.json) — top-level container.
- [Relationship](docsdog-spec/relationship.schema.json) — individual edge.
- [Scan example](docsdog-spec/scan-example.json) — complete example output.
