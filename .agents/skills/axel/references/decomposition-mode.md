## Mitchell decomposition mode (opt-in)

**Default: off.** When off, skip this entire section — zero new mandatory steps.

**On when:** human requests “mitchell”, “decomposition mode”, or “draw the owl”, **or** harness discloses `decomposition_mode: true`.

**Contract (normative detail):** `docs/plans/mitchell-decomposition-contract.html` (issue #43).

### Threshold

- **T** = harness `decomposition_loc_threshold` or **1500**.
- **LOC** = added + deleted (`git diff --numstat` columns; same definition as a phase's `est. LOC`).
- **Plan time:** each phase states `est. LOC`. `audit-plan --loc-threshold T` compares those estimates. The plan Architect rejects a phase whose stated size exceeds T.
- **Code time:** measure the phase (or uncommitted) diff with `git diff --numstat`.

### Over-threshold path (mandatory when mode on)

**Plan time**

1. Architect **REJECT**. Rewrite the plan (smaller phases or superseding claims). Do not start implementation.

**Code time**

1. **Do not commit** the oversize blob.
2. **Return to the plan** with superseding claims (ids append-only). The plan Architect re-blesses the new phase boundaries.
3. Nobody splits the phase at code time. A re-split after BLESS creates unblessed phase boundaries.
4. If the measure reveals new product scope, route through `avril`.

### Draw-the-owl spike

Allowed only as a **time-boxed spike** to discover seams. Spike output is learning + candidate phase cuts for the plan — not a license to merge a mega-diff or to split a blessed phase at code time. If the spike invents new product scope, route through `avril` before treating it as blessed work.

### HITL

Human-in-the-loop remains required for UI, public APIs, and architectural invariants. Decomposition mode does not auto-merge.
