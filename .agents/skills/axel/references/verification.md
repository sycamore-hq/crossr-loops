## Verification

In a fresh activation the following seven behaviors are directly observable and scorable:

- The agent recites the One-Sentence Mandate verbatim before selecting work or moving a board item.
- The agent enforces the intake gate (AVRIL blessing, blessed marker, or explicit human ids) and refuses unblessed scope; board/tracking details are treated as harness-disclosed parameters with Pinto preferred when present.
- The agent runs PETC per phase, delegates generation to the disclosed Generator stack, and sequences Reviewer → Tester → Architect without skip or reorder.
- The agent requires explicit `BLESS` from all three code adversaries before commit; on `REJECT` or silence it re-delegates and restarts the full adversary chain for that phase.
- The agent itself emits zero code, zero edits, and zero adversary review content; it only sequences, records evidence, updates board/tracking via delegation or explicit post-bless ritual direction, and gates.
- The agent blocks `done` until every acceptance criterion has recorded evidence and the disclosed verification matrix is green, then emits a PBI Completion Record before advancing.

- The agent refreshes the status dashboard at each disclosed checkpoint via the harness's dashboard command, never hand-writes the artifact, and never reports a state it has not read from the board or tracking artifacts.

**Additionally, when decomposition mode is on** (opt-in; scorers may treat these as eighth/ninth observables):

- Before any commit of Generator output, the agent measures phase LOC via `git diff --numstat` (added+deleted) against threshold T (default 1500) and **halts commit** when LOC &gt; T, entering decompose→massage→chunk recursion instead.
- When decomposition mode is **off**, the agent introduces **zero** new mandatory steps beyond the seven behaviors above (mode-off path unchanged).

Violations against any of these observable criteria during fresh activation indicate the skill was not followed and must be corrected before the work can be considered complete.
