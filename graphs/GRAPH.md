# Graphs (v0)

Explicit topology for the conductors. **Not a runtime.**

- Authoring format: JSON. `apiVersion: crossr-loops/v0`.
- Nodes + edges. Catalog skills are referenced **by name** (`catalog: true`). This remote does not own their text.
- Local personas and conductor skills are files in this repo.
- **If a graph and a `SKILL.md` disagree, `SKILL.md` wins.** The graph is a map.

Schema: [`schema.json`](schema.json). Gates: `just graphs-verify`, `just verify-skill-refs` (requires `CROSSR_SKILLS_PATH` at the lockfile skills pin; a sibling checkout is never substituted).

## Runner

`graph-runner` (`runner/`, binary of this repo) **replays a walk against a graph; it never runs one.** It reads `graphs/*.json` and nothing else: no `SKILL.md`, no model, no process, no socket. Every event comes from the caller; the runner never decides a verdict. No Rhai. No OpenCode-native executor. No interpreter in v0.

- **Start** is the graph's `start` key. Document order and in-degree are not the rule.
- **Sink** is a node with no out-edges (`role: terminal` by construction; `code-gan`'s `commit` gate too). A walk is complete iff it stands on a sink at depth 0 with every event consumed; events left over are `trailing events`, events run out are `incomplete at <graph>:<node>`.
- An unlabeled edge fires on the reserved event `next`; any other edge fires on its `when`. An **adversary** accepts `BLESS` / `REJECT` and nothing else. An event no out-edge accepts fails loud, naming the node and what it accepts.
- **Descent**: a `role: graph` node opens `uses.graph` at that graph's `start`; the subgraph's sink returns to the parent node and the next event selects the parent's out-edge. The trace indents two spaces per level and prints `<graph>: <node> >> <sub>` / `<sub>: <sink> << <graph>:<node>`. `--flat` treats the node as opaque.
- **Walks** live in [`walks/`](walks/) as `<graph>.<story>.walk`: one event per line, `#` comments, blank lines ignored; the prefix before the first `.` names the graph. Each file tells one story (`avril.happy`, `code-gan.tester-rejects`, `axel.missing-evidence`, …).
- `just graphs-check` loads every graph; `just graphs-cover` replays every committed walk with descent on and lists every `(graph, edge)` no walk took — `cargo test` and `just graphs-verify` (which now shells out to `cover`) are red while that count is above zero.
- **`SKILL.md` wins over the graph; the graph wins over the runner.** The runner reads the graph; nothing reads the runner.

| File | What |
|------|------|
| `avril.json` | Planning GAN — generator → PO → QA → CTO → stop |
| `code-gan.json` | Inner code GAN (generate → mechanical → tester → reviewer → commit; architect on unsatisfiable-claim only) |
| `axel.json` | Execution loop; plan-write → plan-audit → plan-architect → `code-gan` |
| `brick.json` | Stage pipeline; stage *skills* are catalog |
| `flagship.json` | Intent → AVRIL → AXEL → Done. BRICK is the alternative, not a node here. |
