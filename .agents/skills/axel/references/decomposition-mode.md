## Mitchell decomposition mode (opt-in)

**Default: off.** When off, skip this entire section — zero new mandatory steps.

**On when:** human requests “mitchell”, “decomposition mode”, or “draw the owl”, **or** harness discloses `decomposition_mode: true`.

**Contract (normative detail):** `docs/plans/mitchell-decomposition-contract.html` (issue #43).

### Threshold

- Measure phase (or uncommitted) diff with `git diff --numstat`.
- **LOC** = sum of added + deleted numeric columns (default definition).
- **Threshold T** = harness `decomposition_loc_threshold` or **1500**.
- If LOC **> T** (or the blob is clearly unreviewable in &lt;10 minutes): **halt the commit path**.

### Over-threshold path (mandatory when mode on)

1. **Do not commit** the oversize blob.
2. **Decompose** into atomic incremental tasks (still under the current blessed PBI or return to AVRIL if new product scope appears).
3. **Massage** task titles/AC into general maintainable slices — not “the hack shape we just produced.”
4. Execute each chunk through PETC + code GAN; **re-measure** each chunk; recurse if still over T.
5. **Parallel** Task subagents are optional when available; **sequential fallback is always valid and required** if parallel is unavailable or unclear.
6. Optional **bounded re-owl**: another exploratory spike to rediscover a thinner path. Results still need intake (no new unblessed scope), GAN BLESS, and AC evidence. Re-owl **never** bypasses the intake gate or adversary chain.

### Draw-the-owl spike

Allowed only as a **time-boxed spike** to discover seams. Spike output is learning + candidate decomposition — not a license to merge a mega-diff. If the spike invents new product scope, route through `avril` before treating it as blessed work.

### HITL

Human-in-the-loop remains required for UI, public APIs, and architectural invariants. Decomposition mode does not auto-merge.
