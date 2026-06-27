# Architecture Decision Record Log
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
* [2. Decision Areas](#2-decision-areas)
    * [2.1 Architecture Principles](#21-architecture-principles)
    * [2.2 Decision Drivers](#22-decision-drivers)
* [3. Decision Log](#3-decision-log)
* [4. Decision Template](#4-decision-template)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
|      |      |                    |         |
|      |      |                    |         |

## 1. Introduction
💬 _Introduction to the Architecture Decision Record Log and how it relates to the project documentation set. Orients the reader to the ADR process and its role in the project lifecycle._

➥ Briefly summarize the ADR Log’s purpose, scope, relationship to other documents (SRS, architecture docs, roadmaps), and how the document is organized. Do not include details here; reference the relevant sections instead.

### 1.1 Document Purpose
💬 _Clarifies why this ADR Log exists, what it contains, and who should use it._

➥ State the purpose in 2–4 sentences. Name the primary audiences (e.g., architecture, engineering, product, operations, security) and how they use it throughout the project lifecycle.

💡 Tips:
- Emphasize that the ADR Log captures and indexes architectural decisions, making decision history and rationale traceable.
- Mention that individual decision records are authored using the Decision Template (Section 4).
- Explain how decisions are classified and when they should be revisited.

### 1.2 Product Scope
💬 _Defines the system(s) governed by these architectural decisions._

➥ Identify the product/system by name and version/release. In 3–5 sentences, describe the scope of the architecture being governed: subsystems, components, integrations, or cross-cutting concerns. Clarify boundaries — what is in scope vs. out of scope for this ADR Log.

💡 Tips:
- Reference the SRS or vision/scope document for broader product scope.
- If a separate ADR Log governs a subsystem, note it here.

### 1.3 Definitions, Acronyms, and Abbreviations
💬 _Help readers understand specialized terms used in the decision log._

➥ Provide a glossary of terms, acronyms, and abbreviations used across the ADRs.

💡 Tips:
- Include terms like ADR (Architecture Decision Record), cross-cutting concern, quality attribute, trade-off, etc.
- Keep entries alphabetized and consistent with the SRS glossary.

| Term | Definition                                                                                               |
|------|----------------------------------------------------------------------------------------------------------|
| ADR  | Architecture Decision Record — A document that captures a significant architectural decision             |
| ADRL | Architecture Decision Record Log — This document, the index and catalog of all ADRs                      |
| ASAP | As Soon As Possible — A measure of priority                                                             |
| SRS  | Software Requirements Specification — Document describing intended purpose, requirements, and nature     |

### 1.4 References
💬 _Lists external sources that are normative or informative for the ADR process._

➥ Cite standards, architecture frameworks, SRS, roadmap, design documents, or style guides. For each, include title, author/owner, version, date, and location/URL. Indicate whether normative (binding) or informative (guidance).

💡 Tips:
- Reference the architecture decision template and decision-making framework used.

### 1.5 Document Overview
💬 _Brief guide to navigating the ADR Log._

➥ Summarize what each major section covers (Decision Areas, Decision Log, Template, Appendixes), note any document conventions, and mention how updates and revision history are managed.

💡 Tips:
- Keep to 3–5 sentences focusing on navigation and conventions.

## 2. Decision Areas
💬 _Provides background and context that frame the architectural decisions. Think of it as the architecture team's "working agreements."_

### 2.1 Architecture Principles
💬 _Overarching principles that guide and constrain the architectural decisions._

➥ Define 5–15 architecture principles that serve as guardrails. For each, state the principle itself and a brief rationale. Principles should be actionable, testable, and guide decision-making.

💡 Tips:
- Use the format: "[Principle] — [Rationale]."
- Examples: "API-first — All capabilities are exposed through versioned APIs." "Prefer boring over clever — Choose proven patterns and well-understood technologies."
- Align architecture principles with the SRS Product Constraints (Section 2.3).

| # | Principle | Rationale | Source |
|---|-----------|-----------|--------|
| 1 |           |           |        |
| 2 |           |           |        |

### 2.2 Decision Drivers
💬 _Recurring forces, constraints, or quality attributes that influence architectural decisions._

➥ List the key decision drivers that apply across many ADRs (e.g., scalability, cost, time-to-market, compliance, team skill set, vendor lock-in). These drivers inform the trade-off analysis in individual ADRs.

💡 Tips:
- Link each driver to the relevant SRS QoS section (3.3) or Product Constraint (2.3) when applicable.
- Distinguish mandatory constraints from strongly-desired qualities.

## 3. Decision Log
💬 _The index of all architectural decisions made for this product/system. This is the core of the ADR Log._

➥ Maintain a table linking to each individual ADR. Each entry should provide enough context for readers to understand the decision at a glance and navigate to the full record.

💡 Tips:
- Assign sequential IDs (e.g., ADR-0001, ADR-0002) or a hierarchical scheme by domain/component.
- Keep the log sorted by ID or date, and mark superseded decisions with a pointer to the replacement ADR.
- Archive rather than delete decisions that are no longer applicable.

| ID       | Title                           | Status     | Date       | Decision Area | Supersedes / Superseded By |
|----------|---------------------------------|------------|------------|---------------|----------------------------|
| ADR-0001 | Use PostgreSQL as primary DB    | accepted   | 2025-01-15 | Data Storage  |                            |
| ADR-0002 | Adopt Event-driven architecture | proposed   | 2025-02-01 | Messaging     |                            |
| ADR-0003 | Migrate auth from JWT to OAuth  | deprecated | 2024-11-20 | Security      | superseded by ADR-0005    |

### 3.1 Status Lifecycle
💬 _Defines the allowed status values and their meaning._

➥ Describe the lifecycle of a decision record (e.g., proposed → accepted → deprecated or superseded). Define what each status means and who may move a decision between statuses.

💡 Tips:
- Keep the lifecycle simple — resist adding too many states.
- A visual state diagram may help.

## 4. Decision Template
💬 _Template for individual architecture decision records. Each ADR is a standalone document authored using this template._

➥ Reference the template file and describe when and how to use it. Templates exist at multiple levels of detail — choose the one appropriate for the decision's complexity and impact.

| Template | Description |
|----------|-------------|
| `adr-template.md` | Full template — for significant, high-impact decisions |
| `adr-template-minimal.md` | Minimal — for medium-impact decisions that need a record without full analysis |
| `adr-template-bare.md` | Bare — for quicker drafting with all sections present |
| `adr-template-bare-minimal.md` | Bare-minimal — for lightweight, low-stakes decisions |

💡 Tips:
- Choose template level based on decision complexity, reversibility, and stakeholder impact.
- When in doubt, start with the full template — it's easier to trim than to add missing analysis later.

## 5. Appendixes
💬 _Supporting material such as glossaries, diagrams, or architectural style references._

➥ Include any supplementary information that supports the ADRs, such as architectural diagrams, technology radar, decision-making frameworks, or domain-specific terminology.

💡 Tips:
- Reference material here rather than duplicating it. Link to the SRS, design docs, or architecture wiki.
- Include a decision-timing calendar or roadmap if decisions are tied to releases or milestones.
