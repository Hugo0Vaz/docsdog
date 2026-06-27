# User Story Specification
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
* [2. Product Overview](#2-product-overview)
    * [2.1 Product Perspective](#21-product-perspective)
    * [2.2 User Classes](#22-user-classes)
* [3. Story Inventory](#3-story-inventory)
* [4. Story Template](#4-story-template)
* [5. Appendixes](#5-appendixes)
<!-- TOC -->

## Revision History

| Name | Date | Reason For Changes | Version |
|------|------|--------------------|---------|
|      |      |                    |         |
|      |      |                    |         |

## 1. Introduction
💬 _Provides an overview of the User Story Specification and orients the reader to the system's intended behaviors from the user's perspective._

➥ Briefly summarize the USS's purpose, product scope, intended audience, and how the document is organized. Do not include details here; reference the relevant sections instead.

### 1.1 Document Purpose
💬 _Clarifies why this USS exists, what it contains, and who should use it._

➥ State the purpose in 2–4 sentences. Name the primary audiences (e.g., product, engineering, QA, design, stakeholders) and how they use it throughout the delivery lifecycle.

💡 Tips:
- Emphasize that the USS captures intended system behavior from the user's perspective, expressed as user stories.
- Mention that individual stories are authored using the Story Template (Section 4).
- Clarify the relationship to the SRS (stories refine functional requirements) and the UCS (stories and use cases describe complementary views of behavior).

### 1.2 Product Scope
💬 _Defines the system whose behaviors are captured by these stories._

➥ Identify the product/system by name and version/release. In 3–5 sentences, describe which features, workflows, or subsystems are covered. Note any deferred or excluded areas.

💡 Tips:
- Reference the SRS Section 1.2 for broader product scope.
- If separate story specifications exist for subsystems, note them here.

### 1.3 Definitions, Acronyms, and Abbreviations
💬 _Help readers understand domain terms and story terminology._

➥ Provide a glossary of terms used in this specification.

💡 Tips:
- Keep entries alphabetized and consistent with the SRS glossary.

| Term | Definition                                                                                                                   |
|------|------------------------------------------------------------------------------------------------------------------------------|
| SRS  | Software Requirements Specification — Document describing intended purpose, requirements, and nature of a software           |
| UCS  | Use Case Specification — Catalog of use cases describing actor-goal interactions with the system                             |
| US   | User Story — A short, simple description of a feature told from the perspective of the person who desires the new capability |
| USS  | User Story Specification — This document, the catalog of user stories for a system                                           |

### 1.4 References
💬 _Lists external sources that are normative or informative for this USS._

➥ Cite the SRS, UCS, ADR Log, UX designs, or business process documents. For each, include title, author/owner, version, date, and location/URL. Indicate whether normative (binding) or informative (guidance).

💡 Tips:
- Prefer stable links or repository paths over volatile URLs.

### 1.5 Document Overview
💬 _Brief guide to navigating the USS._

➥ Summarize what each major section covers (Product Overview, Story Inventory, Template, Appendixes), note any document conventions, and mention how updates and revision history are managed.

💡 Tips:
- Keep to 3–5 sentences focusing on navigation and conventions.

## 2. Product Overview
💬 _Background and context shaping the stories._

### 2.1 Product Perspective
💬 _Places the system within its larger context._

➥ Describe context and origin of the system — whether new, a replacement, or part of a family. Reference related systems or dependencies that affect story scope.

💡 Tips:
- Highlight upstream/downstream systems and ownership boundaries.

### 2.2 User Classes
💬 _The roles and personas for whom stories are written._

➥ List user classes, roles, or personas referenced in the stories. Note their goals, expertise, frequency of use, and any distinguishing attributes.

💡 Tips:
- Derive from the SRS Section 2.4 User Characteristics. Keep names consistent across documents.
- Stories use these roles in the "As a..." clause.

| User Class | Description / Goals | Frequency |
|------------|---------------------|-----------|
|            |                     |           |
|            |                     |           |

## 3. Story Inventory
💬 _The index of all user stories for this system. This is the core of the USS._

➥ Maintain a table linking to each individual story document. Each entry should provide enough context to navigate the inventory at a glance.

💡 Tips:
- Use a consistent ID scheme (e.g., US-001, US-002).
- Sort by priority or functional area.
- Mark deferred or planned stories clearly.

| ID     | Title | User Class | Priority | Effort | Status | SRS Reference |
|--------|-------|------------|----------|--------|--------|---------------|
| US-001 |       |            |          |        |        |               |
| US-002 |       |            |          |        |        |               |

### 3.1 Status Lifecycle
💬 _Defines the allowed status values and their meaning._

➥ Describe the lifecycle of a story (e.g., draft → ready → in-progress → done). Define what each status means and who may transition between statuses.

💡 Tips:
- Align with the team's workflow (e.g., Kanban board columns).

### 3.2 Priority Convention
💬 _Defines the priority levels used across stories._

| Priority | Description |
|----------|-------------|
| critical | Must be delivered; blocking progress on core functionality |
| high     | Important; should be delivered soon |
| medium   | Valuable; deliver when capacity permits |
| low      | Nice to have; deliver if and when possible |

### 3.3 Effort Convention
💬 _Defines the effort sizing scale used across stories._

| Effort | Description |
|--------|-------------|
| XS     | Trivial — a few hours |
| S      | Small — a day or two |
| M      | Medium — a few days |
| L      | Large — a week or more |
| XL     | Extra large — must be split into smaller stories |

## 4. Story Template
💬 _Templates for individual user story documents. Each story is a standalone document authored using one of these templates._

➥ Reference the template files and describe when to use each.

| Template | Description |
|----------|-------------|
| `us-template.md` | Full — with guidance comments (💬) and tips (💡) for thoughtful authoring |
| `us-template-bare.md` | Bare — placeholders only, for quick drafting when guidance is not needed |

💡 Tips:
- Start with the full template when the story involves multiple stakeholders or dependencies.
- Use the bare template for straightforward, well-understood stories.

## 5. Appendixes
💬 _Optional supporting material such as personas, journey maps, or design references._

➥ Include any supplementary information that supports the stories. Reference rather than duplicate when possible.

💡 Tips:
- Personas or user journey maps can provide richer context for the stories.
- Cross-reference the SRS and UCS Appendixes for broader system documentation.
