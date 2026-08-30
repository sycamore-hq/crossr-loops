# crossr-loops

AVRIL, AXEL, BRICK, rust-team-lead, and orchestrator-prompt live here.

These are the loops (and later, the graphs) that compose skills from
[`crossr-skills`](https://github.com/sycamore-hq/crossr-skills) by name.
This remote does not own skill text. It is pinned from a harness lockfile.

**Dogfood is live (split-08).** First published pin: [`v0`](https://github.com/sycamore-hq/crossr-loops/releases/tag/v0). Lockfile: `skills = "v0-last-monolith"`, `loops = "v0"`. Process files came from `crossr-harness --process-only` — they are a consumer instance, not loop law.

**Graphs (split-09).** JSON nodes+edges under [`graphs/`](graphs/). Map, not a runtime. SKILL.md stays the law. Human view: [`graphs/index.html`](graphs/index.html).

Charter: [`skills-loops-harness-split.html`](https://github.com/sycamore-hq/crossr-skills/blob/main/docs/plans/skills-loops-harness-split.html).

## What's here

### Conductors

| Skill | Role |
|-------|------|
| `avril` | Planning GAN — blessed PBIs (PO → QA → CTO) |
| `axel` | Execution loop — blessed PBI → PETC + code GAN |
| `brick` | BRICK conductor only — stages stay in the catalog |
| `rust-team-lead` | Inner GAN for Rust plan execution |
| `orchestrator-prompt` | Generate a stateless loop-runner prompt |

### Personas

- AVRIL: `planning-architect`, `product-owner`, `qa-architect`, `visionary-cto`
- AXEL: `axel-conductor`
- Rust GAN: `reviewer`, `tester`, `architect`
- BRICK stages: `brick-specifier`, `brick-coder`, `brick-refactorer`, `brick-mutator` *agents* (the stage *skills* stay in skills)

### OpenCode prompt bodies

`templates/harness/opencode/{agent,command}/{avril,axel}.md` — loops owns the text; harness bootstrap installs them.

### Pipeline law

`book/src/pipeline/{overview,avril,axel,brick}.md` is the SSOT for those chapters.

### Graphs (topology)

[`graphs/`](graphs/) — `avril`, `axel`, `brick`, `rust-team-lead`, `flagship`. Catalog skills by name. No Rhai. `just graphs-verify`.

## Not here

- BRICK stage skills (`brick-specifier`, `brick-coder`, `brick-refactorer`, `brick-mutator`) — catalog
- `skill-evaluator` + skill GAN agents — catalog
- `dashboard-prompt`, `chief-of-staff` — harness
- `HARNESS-SPEC.md`, bootstrap, dashboard — harness

## Sibling remotes

- [crossr-skills](https://github.com/sycamore-hq/crossr-skills) — catalog
- [crossr-harness](https://github.com/sycamore-hq/crossr-harness) — spec, bootstrap, dashboard
- [crossr-web-landing](https://github.com/sycamore-hq/crossr-web-landing) — public front door
