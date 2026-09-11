# Implementation Progress

## split-08 — consume harness (COMPLETED)

`--process-only` consumer files. Pin `v0` peels to `4bc52bd`.

## split-09 — graphs (COMPLETED)

Explicit JSON graphs in `graphs/`. SKILL.md untouched. Not a runtime.

- Schema `crossr-loops/v0`: nodes + edges. Catalog skills by name.
- Conductors: `avril`, `axel`, `brick`, `rust-team-lead`. Flagship: intent → AVRIL → AXEL.
- `scripts/verify-graphs` (+ `--html` → `graphs/index.html`).
- No Rhai. No OpenCode-native executor.

**Next:** a runner, if ever, is a later loops backlog. Not this PR.

## Verification Status
- `./scripts/verify-graphs`: PASS
- SKILL.md files: unchanged

## gan-layer-separation — PR 1b (COMPLETED)

Stacked on crossr-skills `pr1-peel-persona-protocol` (plan: crossr-skills `docs/plans/gan-layer-separation-plan.md` §4 PR 1).

- Adversary personas (`rust-reviewer-agent`, `rust-tester-agent`, `rust-architect-agent` — `rust-` prefix drops in PR 2 with the graph referrers): received their `Agent Personality` blocks from the gate skills, dropped `code-writer`/`rust-code-writer` from Required Skills, gained the gate card (`rust-code-reviewer` / `rust-code-tester` / `architecture`) + `gan-verdict`, and each declares its verdict format: `<gate>: BLESS | REJECT`. The architect's "NACK." verdict style was retired — `verify-protocol` forbids it.
- `scripts/verify-protocol` (+ `just verify-protocol`): every graph adversary node must have a persona, BLESS/REJECT out-edges only (both present), and a persona that declares both tokens and no retired token (`BLESSED`/`PASSED`/`REJECTED`/`NACK`). Enforces the `gan-verdict` catalog contract; catches the old personas' silence.
- `lockfile.toml`: `skills = "v1-gan-layers"` — the tag to cut from crossr-skills `main` at the 1a merge commit, before this PR merges. No `rust-architect` shim: `v0-last-monolith` is a frozen tag, so the rename never reaches consumers until the pin moves.

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS (6 adversary nodes)

## gan-layer-separation — PR 2a (COMPLETED)

Plan: crossr-skills `docs/plans/gan-layer-separation-plan.md` §4 PR 2 / §3.2. Load by role, not by stack.

- Personas renamed (`git mv`): `reviewer-agent`, `tester-agent`, `architect-agent`. Title lines updated. Personality / Required Skills / protocol steps untouched — Rust-flavored voice is PR 5.
- `axel-conductor-agent` load set is now `axel` + `gan-verdict`. Language stack is disclosed to subagents, never loaded by the conductor. `rust-team-lead` stays as an inner-GAN delegation option until PR 3.
- `graphs/rust-team-lead.json` `uses.persona` retargeted. `graphs/axel.json` `requires.skills` reduced to the conductor window. Plan node still `axel-conductor-agent` (PR 6).
- Template `templates/harness/opencode/agent/axel.md` prose only: new persona names + matching load set. Files kept (deletion is PR 3, after harness generation in 2b).
- `scripts/verify-skill-refs` (`just verify-skill-refs`): every graph `uses.skill` / `requires.skills` / `uses.persona` / `uses.graph` resolves. Catalog is `CROSSR_SKILLS_PATH` only — a sibling checkout is never the pin; no clone fallback. Failure text names the trees actually checked.
- `axel/SKILL.md` adversary line: `rust-architect` → `architecture` (one word; same class as loops#4). Writer-stack activation prose left for PR 3/4.
- `axel-conductor-agent` records load-set bytes at session start (plan §7; persona, not law). Baseline 73,031.

**Named window (PR 2 → 3/4) — do not expand 2a.** The persona and graph load by role. These still teach the old writer-stack activation and will put the 73k stack back if a conductor obeys SKILL.md-as-law:

- `axel/SKILL.md` frontmatter “Always activate together with `code-writer`”; body MUST-apply; activation statement `code-writer` + `axel` + `rust-team-lead`
- `templates/harness/opencode/command/axel.md` — “Load the `axel` and `code-writer` skills”
- `book/src/pipeline/axel.md:39` — `code-writer` + `axel` + `rust-code-writer` + `rust-team-lead`

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS
- `./scripts/verify-skill-refs`: PASS

## gan-layer-separation — PR 4a (COMPLETED)

Plan: crossr-skills `docs/plans/gan-layer-separation-plan.md` §4 PR 4 / §3.1 / §3.4. Card + `references/` split. Writer-stack window closed. `avril-conductor-agent` created (parked dual-source gap).

- `axel/SKILL.md` 18,148 → 5,998. `avril/SKILL.md` 12,627 → 4,902. Gates stayed on the card.
- Conductor load set is conductor card + `gan-verdict`. Language/domain stack disclosed to subagents.
- One mandate per conductor role, on the persona. Skill-layer mandates deleted.
- `templates/harness/opencode/agent/avril.md` deleted; generation produces `avril.md`.

Review follow-ups (4b / harness, not this merge):
- §7 row correction must carry the 5,443-byte irreducible arithmetic, not just swap 3KB → 6KB.
- Dashboard: re-litigate "lift into `agent-harness`". No conductor loads that skill since 2a; the parameterized pair may be the end state.
- Harness loops pin must move to `v1-cards`. 4b as scoped (skills pin + plan record) does not cover it.
- Stale-target remedy: an unmarked `.opencode/agent/avril.md` copied before that bump must be deleted by hand before regen.

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS
- `./scripts/verify-skill-refs`: PASS (`CROSSR_SKILLS_PATH` set to a checkout of tag `v1-gan-layers`; empty var fails by design)

## hygiene — loops-lockfile (COMPLETED)

`lockfile.toml` is a consumer pin (harness-bootstrap / `--process-only` dogfood), same contract as harness and skills. It is not a self-advertisement of this remote's tag. Left at split-08 `v0` after harness and skills moved to `v1-cards`.

- `lockfile.toml`: `loops = "v1-cards"` (skills pin unchanged).
- README + MIGRATION current-pin lines moved with it. Split-08 history (`loops = "v0"` as of that cut) stays.
- Graphs note: `v0` has no graphs; `v1-cards` does. Bootstrap still does not copy `graphs/`.

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS
- README current pins == `lockfile.toml`

## gan-layer-separation — close 0–4b (COMPLETED)

Every recorded child commit was already completed (`pr1b`..`pr4a`, `loops-lockfile`). Phase left `in_progress`, so the dashboard counted an active phase with 0 in-progress commits. Status is `completed`. 5d is a later unit and already on main.

### gan-layer-separation — PR 6b (COMPLETED)

Plan-first AXEL. Per crossr-skills `docs/plans/gan-layer-separation-plan.md` §4 PR 6 / work#11.

- `graphs/axel.json`: `plan-write` uses catalog `plan-writer` (not the conductor). Mechanical `plan-audit` before `plan-architect`. Architect BLESS → `code-gan`.
- `graphs/code-gan.json`: generate → mechanical → tester → reviewer → commit. Architect only on `unsatisfiable-claim`.
- `axel/SKILL.md`: decompose lives inside the plan; three-REJECT trip; mechanical before LLM; scope change → AVRIL; AXEL does not re-bless.
- Personas, completion record `## Plan`, book + `/axel` command retargeted.

### gan-layer-separation — PR 7b (COMPLETED)

AVRIL set review + AXEL packet ritual. Per crossr-skills `docs/plans/gan-layer-separation-plan.md` §3.3 / §3.8 / §4 PR 7 / work#12. Decisions 1, 3, 4, 5, 8, 9, 10.

- Pin `skills = v1-packets` (lockfile + README).
- `batch: true` on avril po / qa / cto. Schema + `verify-graphs` + `verify-protocol` per-item verdict declaration.
- AVRIL: one verdict line per id; `REJECT <id>` waits at PO; siblings keep BLESS; QA/CTO review the full set after it is PO-complete; cycle set-size line in the blessing log. No batch-size cap.
- AXEL: audited handoff packet before each adversary; drop review prose after commit.

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS (batch lines for po / qa / cto)
- `./scripts/verify-skill-refs`: PASS (`CROSSR_SKILLS_PATH` = tag `v1-packets`)
- `python3 -m unittest discover -s test -v`: OK

## graph-runner

R0 — explicit `start` in schema, graphs, verify-graphs.

Every graph names its entry node. `avril` and `code-gan` have no in-degree-0 node, so document order was never a rule a runner could recover. Schema requires the key; `verify-graphs` fails it missing or dangling.

### R1 (COMPLETED)

Crate scaffold, typed graph model, `check`. Per crossr-skills `docs/plans/graph-runner-prompt-set.md` decisions 1, 3, 7, 9. No stepping yet.

- `runner/` crate `graph-runner` (edition 2021, `rust-version = "1.94"`), workspace `Cargo.toml`, `Cargo.lock` committed, `rust-toolchain.toml` pins `1.94.1` with rustfmt + clippy, `target/` ignored. Dependencies: `serde`, `serde_json`, `thiserror`. Argv hand-parsed.
- `graph.rs` (data): `Graph` / `Node` / `Role` / `Edge` / `NodeId` / `Label` / `Uses` / `Requires`; `deny_unknown_fields` on every struct; `Label::NEXT` reserved. `role_enum_matches_schema` reads the schema's role enum and compares it to `Role::ALL`, in order.
- `load.rs` (checks): typed `LoadError`, every variant names the graph and the node/edge. Refuses wrong `apiVersion` / `kind`, empty nodes, duplicate ids, dangling edge endpoints, a `start` naming no node, two out-edges of one node with the same `when` (or both unlabeled), `when: "next"`, `uses.graph` with `uses.skill`, self-reference. Mirrors `verify-graphs`; does not replace it. Persona / skill existence stays with `verify-skill-refs`.
- `graph-runner check <dir>`: one `✓` line per graph (nodes, edges, start, sinks), `✓ 5 graphs OK`; `LoadError` → stderr, exit 1; bad argv → usage, exit 2.
- `justfile`: both `|| echo "(no Rust crates)"` fallbacks deleted; `check` / `test` run cargo for real; `runner-check` (fmt, pedantic clippy `-D warnings`, test) and `graphs-check` added.
- Graphs, `schema.json`, `.agents/`, `lockfile.toml` byte-identical. `verify-graphs` PASS, `verify-protocol` PASS, `verify-skill-refs` PASS (`v1-packets`), Python tests OK, Rust matrix green.

### R2 (COMPLETED)

Stepper, `walk`, happy-path walks. Per crossr-skills `docs/plans/graph-runner-prompt-set.md` decisions 3, 4, 6. No descent: a `role: graph` node is opaque, so walks are committed only for `avril`, `code-gan`, `brick` (decision 5).

- `event.rs` (data): `Event` = `Next` | `Verdict(Bless | Reject)` | `Label`; `FromStr` reads the walk-file token grammar (`next`, `BLESS`, `REJECT`, anything else a label; case-sensitive; one token per line).
- `step.rs` (pure): `step(&Graph, &NodeId, &Event)` — `Next` fires the unlabeled edge, a verdict fires `BLESS` / `REJECT`, a label fires `when == label`; no match → `NoEdge { accepted }` listing the node's labels (`next` for unlabeled); an adversary given a non-verdict → `NotAVerdict` even when a label would have matched. `walk(&Graph, &[Event])` from `graph.start`: `Complete` on a sink with events exhausted; `TrailingEvents` / `Incomplete` / `Step` otherwise, each carrying the trace so far. No ambiguity variant — the loader forbids it.
- `trace.rs` (data): `Trace { steps, end }`; renders `<graph>: <from> --<event>--> <to>` one line per step, the graphs' own vocabulary.
- `walkfile.rs` (pure): `.walk` text → events; `#` comment lines, blank lines skipped; file-stem prefix before the first `.` names the graph.
- `main.rs`: `walk <graph.json> <walk-file>` — trace to stdout; exit 0 complete, 1 any error (trace so far on stdout, error on stderr), 2 usage. File reads stay here.
- `graphs/walks/{avril,code-gan,brick}.happy.walk`, each opening with a `#` story line. `tests/step.rs` (the six C-08 tests + grammar), `tests/walks.rs` `committed_walks` (every descent-free walk replays to a sink).
- Graphs, `schema.json`, `.agents/`, `lockfile.toml` byte-identical. `verify-graphs` PASS, `verify-protocol` PASS, `verify-skill-refs` PASS (`v1-packets`), Python tests OK, Rust matrix green.

### R3 (COMPLETED)

Descent, `cover`, full edge coverage, docs. Per crossr-skills `docs/plans/graph-runner-prompt-set.md` decisions 5, 6, 10. Closes the loops side of the chain; R4 (work ledger) follows. 16 walks under `graphs/walks/`; `cargo run -q -p graph-runner -- cover graphs` → `taken 37/37 edges` / `uncovered edges: 0`.

- `step.rs` (pure): `walk(&Graph, &dyn Resolve, &[Event], flat)`. A `role: graph` node opens `uses.graph` at that graph's `start` (one `Descend` step); a subgraph standing on a sink closes when the next event arrives (one `Return` step) and that event selects the parent's out-edge. Complete iff on a sink at depth 0 with events exhausted; `Incomplete` names the innermost frame and its depth (`code-gan:commit`, depth 2). `flat` keeps R2 behaviour. Two new `StepError`s: `UnresolvedSubgraph` (resolver lacks the graph), `RecursiveDescent` (graph already open).
- `trace.rs` (data): `Step` = `Edge { depth, graph, index, from, event, to }` | `Descend` | `Return`; two spaces per level; `<graph>: <node> >> <sub>` and `<sub>: <sink> << <graph>:<node>`.
- `cover.rs` (pure): `cover(&Graphs, &[(graph, events)]) -> Coverage { taken: BTreeSet<(name, edge index)>, uncovered, total }`; edges counted by `(graph, index)`; a walk that does not complete is `CoverError`, not partial coverage.
- `load.rs`: `load_dir` returns `Graphs` (`BTreeMap<stem, Graph>`, a `Resolve`); a `uses.graph` with no file in the directory → `LoadError::MissingSubgraph`.
- `main.rs`: `walk [--flat] <graph.json> <walk-file>` (subgraphs resolved beside the graph file), `cover <dir>` (`taken k/n edges`, one line per uncovered edge, `uncovered edges: u`; exit 0 iff 0).
- `graphs/walks/`: `axel.happy`, `flagship.happy` (two levels of descent), and the stories that take every remaining edge — `avril.{po,qa,cto}-rejects`, `code-gan.{mechanical-fails,tester-rejects,reviewer-rejects,architect-blesses-claim,architect-rejects-claim}`, `axel.{plan-audit-fails,plan-architect-rejects,missing-evidence}`. `brick.happy` already takes every brick edge.
- `tests/walks.rs` `committed_walks` replays every walk with descent on and asserts `cover(...).uncovered` is empty; `tests/descent.rs` (flagship trace, innermost-frame incompleteness, `--flat`, re-entry, `MissingSubgraph`, unresolved and recursive descent).
- `scripts/verify-graphs`: when `runner/Cargo.toml` exists, shells out to `cargo run -q -p graph-runner -- cover graphs`; cargo missing from PATH fails naming `cargo` and `rust-toolchain.toml`; a non-zero exit or count fails with the runner's uncovered lines. Pure `parse_cover` / `judge_cover`, tested in `test/test_verify_graphs_cover.py` (no cargo call). `index.html` regenerated; the cover line is in the report.
- Docs (decision 10): `graphs/GRAPH.md` `## Runner`; one sentence in `README.md` Graphs (topology); one sentence in `book/src/pipeline/overview.md` after "Topology (not law)". `justfile` `graphs-cover`.
- Graphs, `schema.json`, `.agents/`, `lockfile.toml` byte-identical. `verify-graphs` PASS (incl. `uncovered edges: 0`), `verify-protocol` PASS, `verify-skill-refs` PASS (`v1-packets`), Python tests OK, Rust matrix green.

## destinations — Grok Bot (COMPLETED)

`book/src/destinations/{overview,grok-bot,grok-bot-profile}.md`. AXEL writer persona `generator-agent.md`. #17 landed those on loops main. "Go cards until a loops tag containing `generator-agent.md` is what harness bootstrap writes" is a harness pin move, not an open dest-grok-bot unit. This PR excludes pin moves.

## destinations — labor (IN PROGRESS)

Optional labor backends on the Grok Bot destination. Law: `book/src/destinations/labor.md`. Setup pastes: `labor-cursor.md`, `labor-claude.md`, `labor-opencode.md`. Pointers on overview, grok-bot, grok-bot-profile. No pin move. No catalog skill. Labor never votes.
