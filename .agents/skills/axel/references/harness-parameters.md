## Harness Context (Stratified Disclosure)

This is a harness-layer execution orchestration skill. It coordinates delivery inside a project harness that supplies:

- A **blessed** backlog intake (AVRIL Blessed Backlog Summary, Pinto items labeled/authorized as blessed, or an explicit human-authorized PBI id set)
- Board backend (prefer **Pinto** when available)
- Tracking artifacts (`features.json` / `progress.md` or equivalent)
- Session ritual + verification commands (`just test`, `just clippy`, etc., as disclosed)
- Code GAN stack for the language in play (see Language stack below)
- Agent personas under the harness agents directory (typical code trio: `reviewer-agent`, `tester-agent`, `architect-agent`)
- Optional **decomposition mode** parameters (see Mitchell decomposition): `decomposition_mode` (bool, default false), `decomposition_loc_threshold` (number, default 1500)

Concrete artifact names, column names, commit message format, and CLI flags are harness parameters disclosed at activation. The invariants (blessed intake only, PETC per unit, three-adversary code BLESS, AC evidence gate, orchestrator never touches code, small reviewable commits) are enforced uniformly. Decomposition mode is **off by default** and adds no steps when off.

### Relationship to other skills

| Skill | Role vs AXEL |
|-------|----------------|
| `avril` | **Upstream.** Produces the blessed backlog. AXEL refuses unblessed work. |
| `agent-harness` | Supplies PETC, stacked-PR, and session-ritual discipline AXEL obeys. |
| `code-writer` (+ language/domain) | **Generator** stack for implementation. Disclosed to the Generator; the conductor never loads it. |
| Mitchell decomposition (opt-in) | When **decomposition mode** is on, oversize phase diffs are halted and forced through decompose→massage→chunk loops. Contract: `docs/plans/mitchell-decomposition-contract.html`. Full contract: `decomposition-mode.md`. |

### Board backend (stratified)

**When Pinto is available** (preferred):

1. Confirm `pinto --version` and the correct board (`.pinto/config.toml`).
2. Inspect with JSON: `pinto list --json`, `pinto show <id> --json`, `pinto next --json`, `pinto board --json`, `pinto dod`.
3. Select work with dependency-aware readiness (`pinto next` or equivalent filter: unstarted, deps in done column).
4. Transition deliberately: move to the harness `in-progress` column when execution starts; to `review` when code GAN is triple-blessed and AC evidence is attached; to `done_column` only after AC checkboxes are satisfied (Pinto warns on incomplete AC — treat incomplete AC as a hard stop even if move would succeed).
5. Optional: `pinto link add` / `pinto link sync` when the harness wants commit↔PBI linkage.
6. Multi-command plans: `pinto automate --plan … --dry-run --json` before real writes when user authorized board mutation.
7. Installed CLI help and any disclosed `pinto-workflow` skill are authoritative for flags.

**When Pinto is not available:**

- Read/write the portable PBI shape from `avril` in the harness-disclosed backlog path.
- Maintain status field: `todo` → `in-progress` → `review` → `done`.
- Do not invent a second tracker if the harness already discloses one.

### Language stack (stratified)

Disclose at session start which code GAN applies. The conductor discloses this stack to Generator and adversary subagents; it never loads the writer or adversary skills itself.

**Rust (default when the repo is Rust / harness says so):**

- Generator: `code-writer` + `rust-code-writer` + domain (`rust-axum-backend`, `rust-tui`, `rust-frontend`, `rust-errors`, …)
- Adversaries (fixed order): `rust-code-reviewer` → `rust-code-tester` → `architecture`
- Personas: `reviewer-agent` → `tester-agent` → `architect-agent`

**Other languages / mixed:**

- Generator: `code-writer` + harness-disclosed language/domain skills
- Adversaries: harness-disclosed reviewer → tester → architect skills/personas in that order
- If no code GAN is disclosed, **stop** and ask the human — do not invent gates
