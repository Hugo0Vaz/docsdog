# Use Case Specification
## For {{project name}}

Version 0.1  
Prepared by {{author}}  
{{organization}}  
{{date_modified}}

## Table of Contents
<!-- TOC -->
* [1. Introduction](#1-introduction)
    * [1.1 Document Purpose](#11-document-purpose)
    * [1.2 Product Scope](#12-product-scope)
    * [1.3 Definitions, Acronyms, and Abbreviations](#13-definitions-acronyms-and-abbreviations)
    * [1.4 References](#14-references)
    * [1.5 Document Overview](#15-document-overview)
* [2. System Context](#2-system-context)
    * [2.1 System Boundary](#21-system-boundary)
    * [2.2 Actor Catalog](#22-actor-catalog)
    * [2.3 Use Case Diagram](#23-use-case-diagram)
* [3. Use Case Inventory](#3-use-case-inventory)
* [4. Use Case Templates](#4-use-case-templates)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
|      |      |                    |         |
|      |      |                    |         |

## 1. Introduction
💬 _Provides an overview of the Use Case Specification and orients the reader to the system’s behavioral requirements._

➥ Briefly summarize the UCS’s purpose, product scope, intended audience, and how the document is organized. Do not include details here; reference the relevant sections instead.

### 1.1 Document Purpose
💬 _Clarifies why this UCS exists, what it contains, and who should use it._

➥ State the purpose of the UCS in 2–4 sentences. Name the primary audiences (e.g., product, engineering, QA, UX, stakeholders) and how they use it across the software lifecycle.

💡 Tips:
- Emphasize that use cases describe externally observable system behavior in the form of actor-goal interactions, not internal implementation.
- Mention that individual use cases are authored using one of the templates referenced in Section 4.
- Clarify the relationship to the SRS (use cases serve as structured functional requirements) and the ADR Log (architectural decisions may constrain how use cases are realized).

### 1.2 Product Scope
💬 _Defines the system whose behavior is specified by these use cases._

➥ Identify the product/system by name and version/release. In 3–5 sentences, describe the system boundary — what is inside vs. outside — and which subsystems or functionality areas are covered. Note any intentionally deferred use cases.

💡 Tips:
- Reference the SRS Section 1.2 for broader product scope.
- The system boundary should map directly to the Scope field in individual use cases.

### 1.3 Definitions, Acronyms, and Abbreviations
💬 _Help readers understand domain terms and use case terminology._

➥ Provide a glossary of domain-specific terms and use case formalism used in this document.

💡 Tips:
- Define Cockburn-specific terms if the full template is used (e.g., scope, level, extensions, minimal guarantees, stakeholders & interests).
- Keep entries alphabetized and consistent with the SRS glossary.

| Term          | Definition                                                                                           |
|---------------|------------------------------------------------------------------------------------------------------|
| Actor         | A role played by a person, system, or device that interacts with the system to achieve a goal        |
| Extension     | An alternate flow of events branching from a step in the main success scenario (Cockburn)            |
| Precondition  | A condition that must be true before the use case can begin                                          |
| Primary Actor | The actor that initiates the use case to achieve a goal                                              |
| Scope         | The boundary of the system under design — the "black box" the use case describes (Cockburn)          |
| UCS           | Use Case Specification — This document, the catalog of use cases for a system                        |

### 1.4 References
💬 _Lists external sources that are normative or informative for this UCS._

➥ Cite the SRS, ADR Log, UX design specs, business process documents, or domain standards. For each, include title, author/owner, version, date, and location/URL. Indicate whether normative (binding) or informative (guidance).

💡 Tips:
- If a vision/scope document or product roadmap exists, reference it for the user goals driving the use cases.

### 1.5 Document Overview
💬 _Brief guide to navigating the UCS._

➥ Summarize what each major section covers (System Context, Use Case Inventory, Templates, Appendixes), note any document conventions, and mention how updates and revision history are managed.

💡 Tips:
- Keep to 3–5 sentences focusing on navigation and conventions.

## 2. System Context
💬 _Defines the boundary and external actors around the system._

### 2.1 System Boundary
💬 _Clarifies what is inside vs. outside the system under design._

➥ In 2–4 sentences, describe the system boundary — the set of responsibilities and behaviors the system owns. This boundary defines the Scope of every use case in this specification. Reference any context or system architecture diagrams.

💡 Tips:
- Use the same boundary definition across use cases for consistency.
- If the system is part of a larger ecosystem, describe its role and key integrations.

### 2.2 Actor Catalog
💬 _Surveys all actors that interact with the system._

➥ List every actor (person, external system, device, or time-based trigger) and their goals or responsibilities. Group actors by type (primary vs. supporting). For each actor, note the use cases they participate in.

💡 Tips:
- Actors are roles, not specific people or systems. Name them accordingly (e.g., "Customer," not "John," and "Payment Gateway," not "Stripe").
- Include external systems, scheduled jobs, and sensors/devices as actors.
- Derive actors from the SRS Section 2.4 User Characteristics — the SRS defines *who* the users are and their attributes (expertise, access levels, frequency, accessibility needs); the Actor Catalog maps those same roles to the use cases they participate in. Keep actor names consistent across both documents.

| Actor            | Type     | Description / Responsibilities | Participates In (UC IDs) |
|------------------|----------|--------------------------------|--------------------------|
| Customer         | Primary  | Places orders, tracks status  | UC-001, UC-002, UC-005   |
| Payment Gateway  | Secondary | Processes payment transactions | UC-001, UC-004           |
| Admin            | Primary  | Manages catalog and users     | UC-010, UC-011           |
| Scheduler        | Primary  | Generates daily reports       | UC-020                   |

### 2.3 Use Case Diagram
💬 _Visual overview of actors and their relationships to use cases._

➥ Place or reference a UML use case diagram that shows all actors, use cases, and relationships (include/extend). If a diagram is large, split by functional area or actor group.

💡 Tips:
- Keep diagrams at a high level — use case diagrams are about context, not control flow.
- Reference individual use case documents for detailed flows; don't duplicate them here.

## 3. Use Case Inventory
💬 _The index of all use cases for this system. This is the core of the UCS._

➥ Maintain a table linking to each individual use case document. Each entry should provide enough context to navigate the inventory at a glance.

💡 Tips:
- Use a consistent ID scheme (e.g., UC-001, UC-002) mapped to the SRS requirement IDs.
- Sort by priority or functional area to help readers prioritize implementation.
- Mark deferred or planned use cases clearly.

| ID      | Use Case Name           | Primary Actor | Level        | Priority | Status  | SRS Reference |
|---------|-------------------------|---------------|--------------|----------|---------|---------------|
| UC-001  | Place an Order          | Customer      | User-goal    | High     | done    | REQ-FUNC-010  |
| UC-002  | Track Order Status      | Customer      | User-goal    | High     | done    | REQ-FUNC-011  |
| UC-003  | Process Refund          | Support Agent | User-goal    | Medium   | draft   | REQ-FUNC-015  |
| UC-004  | Validate Payment Method | Payment GW    | Subfunction  | High     | done    | REQ-FUNC-020  |
| UC-005  | Generate Sales Report   | Scheduler     | Summary      | Low      | planned | REQ-FUNC-030  |

### 3.1 Status Lifecycle
💬 _Defines the allowed status values and their meaning._

➥ Describe the lifecycle of a use case (e.g., draft → reviewed → approved → implemented → verified). Define what each status means and who may transition between statuses.

💡 Tips:
- Align with the project's workflow (e.g., agile board columns, SRS verification status).

### 3.2 Level Convention
💬 _Defines the abstraction levels used in this specification (Cockburn)._

➥ Describe the use case levels and how to apply them to prevent mixing abstraction layers:

| Level       | Purpose | Typical scope | Example |
|-------------|---------|---------------|---------|
| Summary     | High-level business process, spans multiple user goals | System or enterprise | "Process an Order from Placement to Fulfillment" |
| User-goal   | The primary unit of work — a single sitting, single goal | Feature or workflow | "Place an Order" |
| Subfunction | A reusable sub-step called by multiple user-goal use cases | Component or service | "Validate Payment Method" |

💡 Tips:
- Most use cases in a specification should be at the user-goal level. Summaries provide context; subfunctions extract reusable behavior.
- A summary use case may reference its constituent user-goal use cases in its main success scenario.

## 4. Use Case Templates
💬 _Templates for individual use case documents. Each use case is a standalone document authored using one of these templates._

➥ Reference the templates and describe when to use each. The Cockburn template is the comprehensive standard; the UML Simplified template is suitable for lower-risk or well-understood use cases.

| Template | Style | Description |
|----------|-------|-------------|
| `uc-template.md` | Cockburn (full) | Comprehensive — for complex, multi-stakeholder, or high-risk use cases where edge cases and stakeholder interests matter |
| `uc-template-bare.md` | Cockburn (bare) | Same sections as the full Cockburn template, with empty placeholders for quicker drafting |
| `uc-template-minimal.md` | UML Simplified | Streamlined — for standard or well-understood use cases where full stakeholder analysis is unnecessary |
| `uc-template-bare-minimal.md` | UML Simplified (bare) | Same sections as the UML Simplified template, with empty placeholders for rapid authoring |

💡 Tips:
- Default to the Cockburn template for new or high-impact features. Downgrade to UML Simplified for internal tools, CRUD operations, or use cases with a single actor and few extensions.
- When in doubt, start with Cockburn — the act of filling out stakeholders & interests often surfaces hidden requirements.

## 5. Appendixes
💬 _Optional supporting material such as diagrams, business process models, or actor personas._

➥ Include any supplementary information that supports the use cases. Reference rather than duplicate when possible.

💡 Tips:
- Actor personas or journey maps can be included here to provide richer context for the use cases.
- Cross-reference the SRS Appendixes for broader system documentation.
