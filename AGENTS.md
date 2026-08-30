## Plan Mode

- Make every plan extremely concise. Sacrifice grammar for scannability.
- At the end of each plan, give a bulleted list of unresolved questions.
- Always follow the Plan → Execute → Test → Commit loop.

## Skills

Pinned from the harness lockfile (`skills = <tag>`, `loops = <tag>`). Catalog skills live in `.agents/skills/`. Loop conductors come from `crossr-loops`.

Graphs (topology, not a runtime) live in `graphs/`. `just graphs-verify` must PASS. Do not rewrite conductor `SKILL.md` to match a graph — the skill wins.

- `code-writer`
- `rust-code-writer`
- `agent-harness`
- [add project-specific skills here]

See [HARNESS-SPEC.md](https://github.com/sycamore-hq/crossr-harness/blob/main/HARNESS-SPEC.md) for the full harness rules, artifacts, and rituals.
