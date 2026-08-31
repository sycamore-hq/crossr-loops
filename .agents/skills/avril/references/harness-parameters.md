## Harness Context (Stratified Disclosure)

This is a harness-layer planning orchestration skill. It coordinates backlog consensus inside a project harness that supplies:

- Intent source (PRD, conversation, prototype notes, ADR set, or equivalent)
- Board / backlog backend (prefer **Pinto** when `pinto` is installed and a `.pinto/` board exists or may be initialized)
- Tracking artifacts (features.json / progress.md or equivalent) for recording that planning completed
- Agent personas under the harness agents directory (typical: `planning-architect-agent`, `product-owner-agent`, `qa-architect-agent`, `visionary-cto-agent`)

The skill definition itself is portable and harness-agnostic. Concrete artifact names, board commands, ID prefixes, and the exact pre-flight ritual are parameters of the invoking harness and are disclosed at activation. The invariants (strict Generator → three-adversary order, explicit BLESS language, planning-only stop, no silent scope creep) are enforced uniformly.

### Board backend (stratified)

**When Pinto is available** (preferred):

1. Confirm `pinto --version`.
2. Enter the correct board (nearest `.pinto/config.toml`, or `pinto init` only when the user wants a new board).
3. Prefer machine-readable inspection: `pinto list --json`, `pinto show <id> --json`, `pinto board --json`.
4. Create/revise via `pinto add`, `pinto edit`, `pinto reorder`, `pinto dep add`, bodies with Markdown acceptance-criteria checkboxes.
5. Multi-step board mutations: `pinto automate --plan … --dry-run --json` first when the user authorized writes.
6. Treat the installed CLI help and any disclosed `pinto-workflow` skill as authoritative for command flags.

**When Pinto is not available**:

- Persist PBIs in the portable shape from the AVRIL card as Markdown (or the harness-disclosed backlog path).
- Keep the same fields and IDs stable across revise cycles.
- Do not invent a second competing tracker if the harness already discloses one.
