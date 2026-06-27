---
# These are optional metadata elements. Feel free to remove any of them.
status: "{draft | reviewed | approved | implemented | verified | deferred | waived}"
date: {YYYY-MM-DD when the use case was last updated}
priority: "{critical | high | medium | low}"
frequency: "{once | daily | weekly | monthly | on-demand | continuous}"
source: {SRS requirement ID or user story reference}
---

# {verb-phrase use case name, representing the actor's goal}

| Use Case ID | {UC-NNN} |
|-------------|----------|

## Scope
💬 _The system boundary — the "box" this use case describes._

➥ State what system or subsystem is under design. This should match the System Boundary defined in the UCS (Section 2.1).

💡 Tips:
- Use the same system name and boundary consistently across all use cases.
- Example: "The Acme e-commerce platform" or "The Order Processing subsystem."

## Level
💬 _The abstraction level of this use case, per Cockburn's hierarchy._

➥ Choose one:
- **Summary** — A high-level business process spanning multiple user goals, often referencing other use cases in its steps.
- **User-goal** — The primary unit of work, completed in a single sitting by one actor. This is the default level for most use cases.
- **Subfunction** — A reusable sub-step called by other use cases; not a standalone user goal.

💡 Tips:
- Most use cases should be user-goal. Summaries provide context; subfunctions extract shared behavior.
- If you find yourself with 20+ steps in the main success scenario, you may be mixing user-goal and subfunction steps.

## Primary Actor
💬 _The actor who initiates the interaction to achieve the stated goal._

➥ Name the primary actor — a role, not a specific person or system instance.

💡 Tips:
- Use the actor name from the Actor Catalog (UCS Section 2.2), which derives from the SRS Section 2.4 User Characteristics. Keep actor names consistent across documents to maintain traceability.
- If the trigger is time-based (e.g., a scheduler), treat the scheduler as the primary actor.

## Secondary Actors
💬 _Actors the system interacts with during the use case, but who do not initiate it._

➥ List any supporting actors (external systems, services, or human roles) that participate.

💡 Tips:
- This is optional if no secondary actors are involved.
- Each secondary actor should appear in at least one step of the main success scenario or extensions.

## Stakeholders & Interests
💬 _Everyone with a vested interest in this use case, and what they care about. This is a Cockburn signature — it surfaces conflicting expectations before design begins._

➥ List each stakeholder group and their primary interest/concern regarding this use case. Be specific.

💡 Tips:
- Go beyond the obvious (e.g., Marketing may want analytics, Privacy Officer may want minimal data collection, CFO may want fraud prevention).
- This table often reveals hidden requirements — don't skip it for high-stakes use cases.

| Stakeholder       | Interest / Concern                                       |
|-------------------|----------------------------------------------------------|
| Customer          | Quick, error-free checkout; order confirmation           |
| Marketing         | Conversion analytics; abandoned-cart tracking            |
| Payment Gateway   | Clear error codes; retry limits; fraud signals           |
| Privacy Officer   | PII minimized; consent captured; data retention honored  |
| CFO               | No double charges; accurate reconciliation               |

## Preconditions
💬 _What must be true before the use case can begin._

➥ List the preconditions — states, data, or conditions guaranteed by the system at the start. These are enforced by the system, not assumed of the actor.

💡 Tips:
- Preconditions are system-enforced guarantees, not actor actions (e.g., "Customer is authenticated" ✓; "Customer opens the app" ✗).
- Avoid listing the trigger as a precondition.

## Minimal Guarantees
💬 _What the system promises even when the use case fails._

➥ State the guarantees the system upholds regardless of outcome. This is critical for audit, compliance, and trust.

💡 Tips:
- Examples: "No money changes hands," "The attempt is logged for audit," "No partial updates," "The user is returned to a safe state."
- If the use case has no meaningful failure guarantees, state "None" explicitly.

## Success Guarantees
💬 _What is true after a successful completion._

➥ Describe the postconditions — the state of the system, the data, and the actor's situation after the goal is achieved.

💡 Tips:
- Be specific: "Order is persisted with status 'confirmed,' inventory is decremented, a confirmation email is queued for delivery."

## Trigger
💬 _The event that causes the use case to begin._

➥ State the trigger as a simple event (e.g., "Customer taps 'Place Order'," "Scheduler fires at midnight UTC," "Support agent selects 'Initiate Refund'").

## Main Success Scenario
💬 _The primary path — the sequence of steps when everything goes right._

➥ Write a numbered list of steps, 3–9 ideally. Each step describes a single interaction between an actor and the system, phrased as a complete sentence.

💡 Tips:
- Cockburn insists on short scenarios. If you exceed 9 steps, consider splitting into a summary use case that calls subfunctions.
- Use the format: "<number>. <Actor> <action>. The System <response>." — always making the actor and system explicit.
- Keep steps at the user-goal level of abstraction. Don't dive into field-level detail here (that's what extensions and subfunctions are for).

1. Customer selects "Checkout" from the cart.
2. System presents the order summary, shipping options, and saved payment methods.
3. Customer confirms shipping address and selects a payment method.
4. System validates the payment method and calculates the final total.
5. Customer confirms the order.
6. System processes the payment, creates the order, decrements inventory, and displays a confirmation page with the order number.

## Extensions
💬 _Every alternate, failure, or edge case branching from the main success scenario. Numbered by the step they extend._

➥ For each step in the main success scenario, list all possible extensions. Use hierarchical numbering (e.g., 3a, 3a1, 3b).

💡 Tips:
- Extensions are first-class — every interesting thing that can go differently lives here. This is where the real specification work happens.
- For complex extensions that warrant their own sub-use case, reference it (e.g., "→ See UC-004 Validate Payment Method").
- Don't forget timeouts, cancellations, and data validation failures.

- **3a. Shipping address is invalid:**
    1. System highlights the invalid fields and prompts Customer to correct them.
    2. Customer corrects the address.
    3. Return to step 3.
- **3b. Customer wants to add a new shipping address:**
    1. System presents the address entry form.
    2. Customer fills in and saves the new address.
    3. Return to step 3 with the new address selected.
- **4a. Selected payment method is expired:**
    1. System marks the method as expired and notifies Customer.
    2. Return to step 3.
- **4b. Payment validation fails (declined):**
    1. System displays the decline reason without exposing sensitive gateway data.
    2. Customer may retry with a different payment method or cancel.
    3a. Customer retries → Return to step 3.
    3b. Customer cancels → Use case ends. Minimal guarantees apply.
- **6a. Payment succeeds but order creation fails (inventory race condition):**
    1. System voids the payment and notifies Customer.
    2. System logs the inconsistency for ops investigation.
- **6b. Payment times out:**
    1. System notifies Customer of the timeout.
    2. System schedules a payment status reconciliation job.
    3. Customer is advised to check order status within 5 minutes.

## Related Information
💬 _Additional context that doesn't fit elsewhere but affects implementation, testing, or prioritization._

➥ Capture notes, performance targets, business rules, UI/UX references, or open questions.

💡 Tips:
- Cross-reference related use cases (included or extended).
- Note any non-functional requirements relevant to this use case (e.g., "Step 6 must complete within 2 seconds p95").
- Flag open decisions or assumptions.

| Category       | Detail                                                                 |
|----------------|------------------------------------------------------------------------|
| Performance    | Payment processing (step 6) must complete within 2 seconds p95         |
| Business Rule  | Orders above R$ 10,000 require manual fraud review before confirmation |
| UI Reference   | See checkout wireframes v2.1 in Figma                                   |
| Open Question  | Should we support buy-now-pay-later in the initial release?            |
| Related UCs    | UC-004 (Validate Payment Method), UC-007 (View Order History)          |

## More Information
💬 _Supporting links, design notes, assumptions, and traceability references._

➥ Link to related artifacts: SRS requirements, ADRs that constrain this use case, UI specs, API contracts, test suites, or architecture diagrams.

💡 Tips:
- Use this as a catch-all for anything that helps future readers understand context.
- Keep it linking-oriented rather than duplicating information.
