# Graphs (v0)

Explicit topology for the conductors. **Not a runtime.**

- Authoring format: JSON. `apiVersion: crossr-loops/v0`.
- Nodes + edges. Catalog skills are referenced **by name** (`catalog: true`). This remote does not own their text.
- Local personas and conductor skills are files in this repo.
- **If a graph and a `SKILL.md` disagree, `SKILL.md` wins.** The graph is a map.
- No Rhai. No OpenCode-native executor. No interpreter in v0.

Schema: [`schema.json`](schema.json). Gates: `just graphs-verify`, `just verify-skill-refs` (requires `CROSSR_SKILLS_PATH` at the lockfile skills pin; a sibling checkout is never substituted).

| File | What |
|------|------|
| `avril.json` | Planning GAN — generator → PO → QA → CTO → stop |
| `code-gan.json` | Inner code GAN (generate → mechanical → tester → reviewer → commit; architect on unsatisfiable-claim only) |
| `axel.json` | Execution loop; plan-write → plan-audit → plan-architect → `code-gan` |
| `brick.json` | Stage pipeline; stage *skills* are catalog |
| `flagship.json` | Intent → AVRIL → AXEL → Done. BRICK is the alternative, not a node here. |
