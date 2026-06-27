---
# These are optional metadata elements. Feel free to remove any of them.
status: "{draft | reviewed | approved | implemented | verified | deferred | waived}"
date: {YYYY-MM-DD when the use case was last updated}
priority: "{critical | high | medium | low}"
source: {SRS requirement ID or user story reference}
---

# {verb-phrase use case name, representing the actor's goal}

| Use Case ID | {UC-NNN} |
|-------------|----------|

## Description
💬 _A concise summary of the use case — what the actor achieves._

➥ In 1–3 sentences, describe the goal and the context. This is the "elevator pitch" of the use case.

💡 Tips:
- Focus on the outcome, not the steps.
- Mention the primary actor and the value delivered.

## Actors
💬 _Who participates in this use case._

➥ List the primary actor first, followed by any secondary actors (external systems, services, or supporting roles).

💡 Tips:
- Primary actor: the one who initiates the use case to achieve a goal.
- Secondary actors: those the system interacts with during execution but who don't initiate.

- **Primary:** Customer
- **Secondary:** Payment Gateway, Email Service

## Preconditions
💬 _What must be true before the use case can begin._

➥ List system-enforced preconditions. These are guarantees, not assumptions about the actor's behavior.

💡 Tips:
- "Customer is authenticated" ✓
- "Customer opens the app" ✗ (that's a step, not a precondition)

## Postconditions
💬 _What is true after the use case completes successfully._

➥ Describe the system state, data changes, and the actor's situation after the goal is achieved.

💡 Tips:
- For error cases, rely on the exception flows to describe partial outcomes. Postconditions here are about success.

## Basic Flow
💬 _The happy path — the sequence of steps when everything goes right._

➥ Write a numbered list of steps describing actor actions and system responses. Keep it concise — 5–10 steps.

💡 Tips:
- Use the format: "<number>. <Actor> <action>. The System <response>."
- Stay at a consistent level of abstraction. Avoid diving into UI details or implementation.
- Each step should represent one interaction, not a subroutine.

1. Customer selects "Checkout" from the cart.
2. System presents the order summary, shipping options, and saved payment methods.
3. Customer confirms shipping address and selects a payment method.
4. System validates the payment method and calculates the final total.
5. Customer confirms the order.
6. System processes the payment, creates the order, and displays a confirmation page with the order number.

## Alternative Flows
💬 _Valid branches or variations of the basic flow — different ways to accomplish the same step._

➥ List alternative paths the actor can take that still lead to success. These are not errors — they're choices.

💡 Tips:
- Reference which step they branch from.
- Keep each alternative flow self-contained or return to a step in the basic flow.

- **At step 3 — Customer adds a new shipping address:**
    1. System presents the address entry form.
    2. Customer fills in and saves the new address.
    3. Return to step 3 with the new address selected.
- **At step 3 — Customer switches saved payment methods:**
    1. System displays the list of saved payment methods.
    2. Customer selects a different method.
    3. Return to step 3.

## Exception Flows
💬 _Error conditions, failures, and recovery paths._

➥ List things that can go wrong, how the system detects them, and how it responds or recovers.

💡 Tips:
- Include timeouts, validation failures, external system errors, and business rule violations.
- Specify whether the use case can continue, retry, or must terminate.

- **At step 4 — Payment method is expired:**
    1. System marks the method as expired and notifies Customer.
    2. Customer must select or add a different payment method.
- **At step 4 — Payment validation fails (declined):**
    1. System displays the decline reason (no sensitive data exposed).
    2. Customer may retry with a different method or cancel the use case.
- **At step 6 — Payment succeeds but order creation fails:**
    1. System voids the payment and notifies Customer.
    2. System logs the inconsistency for ops investigation.
- **At step 6 — Payment times out:**
    1. System notifies Customer of the timeout.
    2. System schedules a payment status reconciliation job.
    3. Customer is advised to check order status within 5 minutes.

## More Information
💬 _Supporting context, cross-references, and open items._

➥ Link to related SRS requirements, ADRs, UI designs, test suites, or note any open questions or assumptions.

💡 Tips:
- Keep it link-oriented rather than duplicating content.
- Flag assumptions or pending decisions that could affect this use case.
