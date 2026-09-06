## Verification

In a fresh activation the following seven behaviors are directly observable and scorable:

- The agent recites the One-Sentence Mandate verbatim before selecting work or moving a board item.
- The agent enforces the intake gate (AVRIL blessing, blessed marker, or explicit human ids) and refuses unblessed scope; board/tracking details are treated as harness-disclosed parameters with Pinto preferred when present.
- The agent runs plan-first PETC: Generator + `plan-writer`, mechanical audit, Architect at plan time, then Generate → Mechanical → Tester → Reviewer per phase.
- The agent requires explicit `BLESS` from the plan Architect before implementation, and from Tester and Reviewer before commit; on `REJECT` or silence it follows the card's matrix (not a full-chain restart).
- The agent itself emits zero code, zero edits, and zero adversary review content; it only sequences, records evidence, updates board/tracking via delegation or explicit post-bless ritual direction, and gates.
- The agent blocks `done` until every acceptance criterion has recorded evidence and the disclosed verification matrix is green, then emits a PBI Completion Record before advancing.

- The agent refreshes the status dashboard at each disclosed checkpoint via the harness's dashboard command, never hand-writes the artifact, and never reports a state it has not read from the board or tracking artifacts.

**Additionally, when decomposition mode is on** (opt-in; scorers may treat these as eighth/ninth observables):

- At plan time, when mode is on, the agent runs `audit-plan --loc-threshold T` (default 1500) and the plan Architect rejects a phase whose stated size exceeds T.
- Before any commit of Generator output, the agent measures phase LOC via `git diff --numstat` (added+deleted) against T and **halts commit** when LOC &gt; T, returning to the plan with superseding claims; it does not split the phase at code time.
- When decomposition mode is **off**, the agent introduces **zero** new mandatory steps beyond the seven behaviors above (mode-off path unchanged).

Violations against any of these observable criteria during fresh activation indicate the skill was not followed and must be corrected before the work can be considered complete.
