---
# These are optional metadata elements. Feel free to remove any of them.
status: "{draft | ready | in-progress | done | deferred | waived}"
date: {YYYY-MM-DD when the story was last updated}
priority: "{critical | high | medium | low}"
effort: "{XS | S | M | L | XL}"
source: {SRS requirement ID or epic reference}
---

# {short title, representative of the user story}

## User Story

💬 The classic user-story format: who, what, and why.

➥ Write a single sentence in the form:

> As a **{role}**, I want **{feature/capability}** so that **{benefit/reason}**.

💡 Tips:
- Keep the role concrete — a persona or user class, not a job title.
- The benefit must be the real why, not a restatement of the feature.
- If you can't articulate the benefit, the story may not be ready.

## Acceptance Criteria

💬 Observable, testable conditions that must be met for the story to be considered done.

➥ List each criterion as a bullet, phrased as a testable statement.

💡 Tips:
- Use the format: "Given {precondition}, When {action}, Then {expected outcome}".
- Each criterion should be independently verifiable — answerable with yes/no.
- If a criterion requires a specific environment, dataset, or role, state it explicitly.

* Given ..., When ..., Then ...
* Given ..., When ..., Then ...

## Notes

💬 Implementation hints, design constraints, dependencies, risks, or open questions that affect this story.

➥ Capture anything the team needs to know beyond the story statement and acceptance criteria. Keep it brief — prefer links to detailed specs or ADRs.

💡 Tips:
- Note dependencies on other stories, APIs, or external systems.
- Flag assumptions that, if proven wrong, would change scope or approach.

## More Information

💬 Supporting links, references, and traceability.

➥ Link to related artifacts: SRS requirements, ADRs, use cases, UI mockups, API specs, or test cases.

💡 Tips:
- Keep this section link-oriented rather than duplicating information.
- Reference the parent epic or feature if this story is part of a larger effort.
