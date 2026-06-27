---
author: Hugo0Vaz
version: 0.1.0
---

# DocsDog Specification

# 1. Introduction

DocsDog is an open specification for expressing semantic relationships between
source code and external artifacts.

It defines a language-independent metadata model that enables traceability
across software systems, documentation, architecture, requirements, APIs,
infrastructure, and other engineering artifacts.

DocsDog intentionally defines only relationships.

It does not define how relationships are discovered, indexed, stored, resolved,
or visualized.

# 2. Design Principles

DocsDog follows six principles.

## Language Agnostic

The specification is independent of any programming language.

## Storage Agnostic

Targets identify artifacts, not locations.

A target may resolve to a Markdown document, a Jira issue, a database schema,
an OpenAPI document, or nothing at all.

## Semantic

Relationships describe meaning rather than implementation.

## Extensible

Organizations may define new predicates and namespaces without modifying the 
specification.

## Minimal

The specification defines only the minimum information required to express a
relationship.

## Non-Intrusive

DocsDog must never alter program behavior.

# 3. Core Model

Every DocsDog annotation represents a directed relationship.

```
Source
    │
    ▼
Predicate
    │
    ▼
Target
```

Where:

* Source is the annotated code element.
* Predicate defines the semantic meaning.
* Target identifies another artifact.

The source is mandatory in the scan document but SHOULD be inferred
automatically by language implementations from the annotated code element.

# 4. Relationship

Every relationship consists of:

| Field     | Required | Type                |
| --------- | -------- | ------------------- |
| source    | Yes      | Source Identifier   |
| predicate | Yes      | string              |
| target    | Yes      | DocsDog Identifier  |
| metadata  | No       | map<string, any>    |

Example:

```json
{
  "source": "php://src/Application/CreateInvoiceService.php#L12",
  "predicate": "implements",
  "target": "docsdog:usecase:UC-001"
}
```

# 5. DocsDog Identifier

Targets are identified using a URI-like syntax.

```
<namespace>:<kind>:<identifier>
```

Where:

| Component  | Description                   |
| ---------- | ----------------------------- |
| namespace  | Registry owner                |
| kind       | Artifact category             |
| identifier | Namespace-specific identifier |

Example:

```
docsdog:usecase:UC-001
```

---

# 6. Built-in Namespace

The specification reserves the namespace:

```
docsdog
```

Artifacts within this namespace have standardized meanings.

---

# 7. Standard Artifact Kinds

## Requirements

```
docsdog:requirement:REQ-014
```

## Use Cases

```
docsdog:usecase:UC-001
```

## User Stories

```
docsdog:userstory:US-032
```

## Business Rules

```
docsdog:rule:BR-008
```

## ADRs

```
docsdog:adr:ADR-004
```

## Events

```
docsdog:event:InvoiceCreated
```

## Commands

```
docsdog:command:CreateInvoice
```

## Queries

```
docsdog:query:GetInvoice
```

## Aggregates

```
docsdog:aggregate:Invoice
```

## Entities

```
docsdog:entity:Customer
```

## APIs

```
docsdog:api:POST:/invoices
```

```
docsdog:api:GET:/users/{id}
```

## Database Objects

```
docsdog:database:invoice

docsdog:table:users

docsdog:view:active_users

docsdog:index:idx_customer_email
```

## Infrastructure

```
docsdog:queue:payments

docsdog:topic:invoice-events

docsdog:bucket:documents

docsdog:lambda:process-payment
```

## Standards

```
docsdog:rfc:RFC-9110

docsdog:iso:27001

docsdog:soc2:CC6.1
```

# 8. External Namespaces

Organizations are encouraged to define additional namespaces.

Examples:

```
jira:ERP-123

github:my-org/payment-service#14

gitlab:backend/issues/20

company:policy:SEC-003

aws:lambda:generate-report

azure:function:invoice-generator

terraform:module:network

kubernetes:deployment:payments

openapi:billing:v1

asyncapi:invoice-events:v2
```

The specification does not interpret external namespaces.

# 9. Predicates

Predicates describe semantic meaning.

The following predicates are standardized.

## Traceability

* implements
* traces-to
* requires
* validates
* tests

## Architecture

* depends-on
* owned-by
* decision
* replaces
* deprecated-by

## Messaging

* emits
* consumes

## Persistence

* persists
* maps-to

## API

* exposes

## Security

* authenticated-by
* authorized-by
* secured-by

## Configuration

* configured-by
* feature-flag

Unknown predicates MUST be accepted.

# 10. Metadata

Metadata is optional. Schema-level validation of metadata is not yet
enforced; it may be added once the corresponding test suite is in place.

Metadata is implementation-defined.

Recommended fields include:

```json
{
    "since": "2.1",
    "critical": true,
    "author": "Architecture Team",
    "tags": [
        "payments",
        "security"
    ]
}
```

Unknown metadata MUST be preserved.

# 11. Multiple Relationships

Any source element may declare any number of relationships.

Example:

```
CreateInvoiceService

implements
→ docsdog:usecase:UC-001

requires
→ docsdog:requirement:REQ-014

decision
→ docsdog:adr:ADR-003

emits
→ docsdog:event:InvoiceCreated

persists
→ docsdog:table:invoice
```

Relationships are unordered.

# 12. Source Elements

Implementations should support any language construct capable of carrying metadata.

Examples include:

* class
* method
* function
* interface
* enum
* module
* namespace
* package
* trait
* record
* struct
* controller
* service
* endpoint

Languages may extend this list.

# 13. Compliance

A compliant implementation MUST:

* expose predicate
* expose target
* preserve metadata
* allow multiple relationships
* support arbitrary namespaces
* support arbitrary predicates
* resolve identifiers
* validate identifiers

A compliant implementation MUST NOT:

* contact remote services
* generate graphs
* interpret predicates
* require runtime infrastructure

# 14. Tool Responsibilities

DocsDog intentionally delegates all processing.

External tools may:

* build knowledge graphs
* compute documentation coverage
* perform impact analysis
* generate dependency diagrams
* integrate with CI
* integrate with IDEs
* integrate with Language Servers

These capabilities are outside the scope of this specification.

# 15. Example

Conceptually:

```
Source:
CreateInvoiceService

Relationship:
implements

Target:
docsdog:usecase:UC-001
```

Another relationship:

```
Source:
CreateInvoiceService

Relationship:
decision

Target:
docsdog:adr:ADR-004
```

Another:

```
Source:
CreateInvoiceService

Relationship:
emits

Target:
docsdog:event:InvoiceCreated
```

Together they describe a semantic graph that tooling can analyze independently of any programming language.

# 16. Versioning

Future versions of DocsDog MUST preserve backward compatibility whenever possible.

Implementations MUST ignore unknown predicates.

Implementations MUST ignore unknown namespaces.

Implementations MUST preserve unknown metadata.

This guarantees that new ecosystem features can be introduced without breaking existing implementations.
