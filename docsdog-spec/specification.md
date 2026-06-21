---
author: Hugo0Vaz
version: 1.0.0
draft: true
---

# DocDog Specification

# 1. Introduction

DocDog is an open specification for expressing semantic relationships between 
source code and external artifacts.

It defines a language-independent metadata model that enables traceability
across software systems, documentation, architecture, requirements, APIs,
infrastructure, and other engineering artifacts.

DocDog intentionally defines only relationships.

It does not define how relationships are discovered, indexed, stored, resolved,
or visualized.

# 2. Design Principles

DocDog follows six principles.

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

DocDog must never alter program behavior.

# 3. Core Model

Every DocDog annotation represents a directed relationship.

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

The source is implicit.

Only the predicate and target are mandatory.

# 4. Relationship

Every relationship consists of:

| Field     | Required | Type              |
| --------- | -------- | ----------------- |
| predicate | Yes      | string            |
| target    | Yes      | DocDog Identifier |
| metadata  | No       | map<string, any>  |

Example:

```json
{
  "predicate": "implements",
  "target": "docdog:usecase:UC-001"
}
```

# 5. DocDog Identifier

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
docdog:usecase:UC-001
```

---

# 6. Built-in Namespace

The specification reserves the namespace:

```
docdog
```

Artifacts within this namespace have standardized meanings.

---

# 7. Standard Artifact Kinds

## Requirements

```
docdog:requirement:REQ-014
```

## Use Cases

```
docdog:usecase:UC-001
```

## User Stories

```
docdog:userstory:US-032
```

## Business Rules

```
docdog:rule:BR-008
```

## ADRs

```
docdog:adr:ADR-004
```

## Events

```
docdog:event:InvoiceCreated
```

## Commands

```
docdog:command:CreateInvoice
```

## Queries

```
docdog:query:GetInvoice
```

## Aggregates

```
docdog:aggregate:Invoice
```

## Entities

```
docdog:entity:Customer
```

## APIs

```
docdog:api:POST:/invoices
```

```
docdog:api:GET:/users/{id}
```

## Database Objects

```
docdog:database:invoice

docdog:table:users

docdog:view:active_users

docdog:index:idx_customer_email
```

## Infrastructure

```
docdog:queue:payments

docdog:topic:invoice-events

docdog:bucket:documents

docdog:lambda:process-payment
```

## Standards

```
docdog:rfc:RFC-9110

docdog:iso:27001

docdog:soc2:CC6.1
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
→ docdog:usecase:UC-001

requires
→ docdog:requirement:REQ-014

decision
→ docdog:adr:ADR-003

emits
→ docdog:event:InvoiceCreated

persists
→ docdog:table:invoice
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

DocDog intentionally delegates all processing.

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
docdog:usecase:UC-001
```

Another relationship:

```
Source:
CreateInvoiceService

Relationship:
decision

Target:
docdog:adr:ADR-004
```

Another:

```
Source:
CreateInvoiceService

Relationship:
emits

Target:
docdog:event:InvoiceCreated
```

Together they describe a semantic graph that tooling can analyze independently of any programming language.

# 16. Versioning

Future versions of DocDog MUST preserve backward compatibility whenever possible.

Implementations MUST ignore unknown predicates.

Implementations MUST ignore unknown namespaces.

Implementations MUST preserve unknown metadata.

This guarantees that new ecosystem features can be introduced without breaking existing implementations.
