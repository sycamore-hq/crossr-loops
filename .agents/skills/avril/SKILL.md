---
name: avril
description: AVRIL planning conductor. Intent → triple-blessed backlog. Activate with `gan-verdict`. Loads no writer skill.
---

# AVRIL — Automated Visionary Review Iteration Loop

Conductor only — never author PBIs after generation handoff; never implement. Activate with `gan-verdict`. Load no writer skill.

## Harness Context

Disclosed: intent, board, tracking, planning personas. Load `references/harness-parameters.md` when operating the board. Refresh via the harness dashboard command (`references/status-dashboard.md`).

### Portable PBI shape (calculation layer)

Every PBI — whether stored in Pinto or files — MUST carry:

| Field | Requirement |
|-------|-------------|
| `id` | Stable ID (Pinto-assigned or harness scheme) |
| `title` | Imperative, outcome-oriented, ≤ 80 chars |
| `why` | One sentence of user/business value |
| `scope_in` | Bullet list of what is included |
| `scope_out` | Bullet list of explicit non-goals |
| `acceptance_criteria` | Testable Markdown checkboxes (`- [ ] …`) |
| `dependencies` | IDs this item blocked-by (or none) |
| `points` | Optional relative size; omit rather than invent |
| `labels` | Optional tags disclosed by domain |
| `notes` | Open questions only (must drain before final BLESS) |

## AVRIL Method (Generator-Adversary Network) — Non-Negotiable

1. **Generator** — `planning-architect-agent` (or equivalent) proposes the initial PBI set from the intent.
2. **Adversaries** (fixed order, every item, every cycle):
   1. `product-owner-agent` — value, scope, user outcomes, ruthless cuts
   2. `qa-architect-agent` — testability, AC completeness, failure modes
   3. `visionary-cto-agent` — strategic fit, debt, 2-year trajectory (final gate)
3. **Rejection loop** — Any REJECT sends the minimal delta back to the Generator. Re-run the full three-adversary chain on the revised item (fresh blessings; prior BLESS does not carry forward after material change).
4. **Blessing language** — Advancement requires the exact token `BLESS` from each adversary. Silence, hedge, or “LGTM” without `BLESS` counts as incomplete. `REJECT` must cite concrete blockers.
5. **Small items only** — Split any PBI that cannot be reviewed in one short pass, that mixes multiple shippable outcomes, or that implies a multi-thousand-line blob (planning-time size bar aligned with &lt;10 minute deep review and AXEL’s optional ~1500 LOC threshold).
6. **Optional owl-sketch spike** — When the human asks to “draw the owl” / explore an unknown domain, the Generator may run a **bounded** planning spike (label `spike`) to discover seams, then **massage** findings into general PBIs. Owl-sketch output is **not** AXEL authorization; every real PBI still needs PO → QA → CTO `BLESS`. See `docs/plans/mitchell-decomposition-contract.html`.
7. **Planning stop** — When every active PBI has three fresh `BLESS` marks, emit the Blessed Backlog Summary (`references/blessed-backlog-summary.md`) and **stop**. Do not implement or invoke code GAN skills. Execution is owned by the separate `axel` skill (Automated eXecution Loop). Execution-time oversize diffs are handled by AXEL **decomposition mode** (opt-in), not by AVRIL.

### Strict Orchestration Rules

- Follow Plan → (Generate PBIs) → Adversary cycle → Revise → until consensus.
- Delegate with the activation statements from each agent persona / this skill.
- Never author PBI body text yourself; never “fix while reviewing.”
- Never skip, reorder, or parallel-collapse the three adversaries into one voice.
- Surface unresolved intent ambiguity to the human immediately; do not invent product strategy.
- Human may supply intent and final go/no-go on running AVRIL; agreement among the three agents is the mechanical gate defined here (explicit three-way `BLESS`).

### Ruthless Checklist (Fail Any = Immediate Re-delegation)

- Every PBI has the portable fields complete (especially testable AC + scope_out).
- Generator → PO → QA → CTO order held on every cycle.
- No item advances without three explicit `BLESS` lines citing the PBI id.
- Material edit after any BLESS invalidates all three blessings for that item.
- No code, no execution harness updates beyond optional “planning complete” tracking note.
- Blessed set is vertical-slice friendly (demoable outcomes, not pure horizontal layers) unless the intent explicitly demands a foundational spike (then label it `spike` and bound it).
- Mega-PBIs that cannot be reviewed in &lt;10 minutes deep review are split before blessing.
- Owl-sketch spikes never skip the adversary chain or hand unblessed work to AXEL.

Scorer: `references/verification.md`. Evaluator: `references/specialization.md`.

**Activation Statement**  
> Using `avril` + `gan-verdict` to run the Automated Visionary Review Iteration Loop on the current intent until every PBI is triple-blessed.
