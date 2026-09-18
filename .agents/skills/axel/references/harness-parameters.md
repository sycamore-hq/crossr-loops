## Harness Context (Stratified Disclosure)

This is a harness-layer execution orchestration skill. It coordinates delivery inside a project harness that supplies:

- A **blessed** backlog intake (AVRIL Blessed Backlog Summary, board items marked blessed, or an explicit human-authorized PBI id set)
- The **board** — which one, and by what channel you reach it (see Board I/O below). The board is the work log; there is no tracking file beside it.
- Session ritual + verification commands (`just test`, `just clippy`, etc., as disclosed)
- Code GAN stack for the language in play (see Language stack below)
- Agent personas under the harness agents directory (typical code trio: `reviewer-agent`, `tester-agent`, `architect-agent`)
- Optional **decomposition mode** parameters (see Mitchell decomposition): `decomposition_mode` (bool, default false), `decomposition_loc_threshold` (number, default 1500)
- Packet scratch path — disclosed by the harness (HARNESS-SPEC §12); fallback `${TMPDIR:-/tmp}/crossr-packets/<pbi-id>/`; never inside the repo, never committed.

Concrete board identity, column names, commit message format, and access channel are harness parameters disclosed at activation. The invariants (blessed intake only, PETC per unit, plan-time Architect then per-phase mechanical → `testing` → `code-review`, AC evidence gate, orchestrator never touches code, small reviewable commits) are enforced uniformly. Decomposition mode is **off by default** and adds no steps when off.

### Relationship to other skills

| Skill | Role vs AXEL |
|-------|----------------|
| `avril` | **Upstream.** Produces the blessed backlog. AXEL refuses unblessed work. |
| `agent-harness` | Supplies PETC, stacked-PR, and session-ritual discipline AXEL obeys. |
| `code-writer` (+ language/domain) | **Generator** stack for implementation. Disclosed to the Generator; the conductor never loads it. |
| Mitchell decomposition (opt-in) | When **decomposition mode** is on, the plan Architect rejects an over-threshold phase; a code-time measure over T halts the commit and returns to the plan. Nobody splits at code time. Contract: `docs/plans/mitchell-decomposition-contract.html`. Full contract: `decomposition-mode.md`. |

### Board I/O (stratified)

You need five capabilities from the board. Which product provides them, and
whether you reach it through MCP tools, a CLI, an HTTP API, or a human relaying
for you, is disclosed — never assumed, never required to be any one of them.

| Capability | What you do with it |
|---|---|
| **Read items** | id, title, status, acceptance criteria, and whatever the last session recorded |
| **Read ordering** | dependency or blocking links, so "ready" means every dependency is done |
| **Move an item** | `in-progress` when execution starts · `review` when Tester and Reviewer have BLESS and AC evidence is attached · `done` only after every AC is satisfied |
| **Record on an item** | what changed, the verification run, the adversary verdicts, the commit carrying it |
| **Link** | commit or PR ↔ item, when the harness wants it |

Rules that hold whatever the channel:

- **Incomplete AC is a hard stop**, even when the board would happily accept the
  move. A board that permits a transition has not blessed it; you have not either.
- **Confirm which board and which project** before the first move, the way you
  confirm which branch before the first commit. Guessing costs a session.
- **Never invent a second tracker.** Not a file in the repo, not a list in the
  transcript. If the board is unreachable this session, say so plainly, keep
  working, and record what is owed — an unreachable board is a reporting duty,
  never a licence to start one of your own.
- **Board writes are deliberate.** Batch nothing, dry-run first where the channel
  offers it, and never move an item you did not just do the work for.

### Language stack (stratified)

The harness discloses `books` from the consumer repo's `lockfile.toml` at session start (pre-flight step 4). The conductor discloses this stack to Generator and adversary subagents; it never loads the writer or adversary skills itself.

**When `books` names one book:**

- Generator (plan) loads: `plan-writer` + `<book>/RULES.md` + the PBI. Do not load `code-writer`.
- Generator (execute) loads: `code-writer` + `<book>` (card + the references for the situation) + domain skills
- Adversaries load: the gate card + `<book>/RULES.md`. Never `<book>/references/`
- Test verifier: rules tagged `test` in that same `RULES.md`
- Plan gate: `architecture` + `architect-agent`. Per-phase: mechanical → `testing` → `code-review`
- Personas: `tester-agent` → `reviewer-agent`; `architect-agent` at plan time (code time only on an unsatisfiable claim)

**When `books` names more than one book:**

Disclose which book applies **per PBI**. If unspecified, stop and ask. Do not default to first-listed.

**When `books` is missing or empty:**

**Stop** and ask the human — do not invent gates.
