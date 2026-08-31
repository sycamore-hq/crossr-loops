# crossr-loops

AVRIL, AXEL, BRICK, and orchestrator-prompt live here.

These are the loops (and later, the graphs) that compose skills from
[`crossr-skills`](https://github.com/sycamore-hq/crossr-skills) by name.
This remote does not own skill text. It is pinned from a harness lockfile.

**Dogfood is live (split-08).** First published pin: [`v0`](https://github.com/sycamore-hq/crossr-loops/releases/tag/v0). Current lockfile: `skills = "v1-gan-layers"`, `loops = "v0"`. Process files came from `crossr-harness --process-only` — they are a consumer instance, not loop law.

**Graphs (split-09).** JSON nodes+edges under [`graphs/`](graphs/). Map, not a runtime. SKILL.md stays the law. Human view: [`graphs/index.html`](graphs/index.html).

Charter: [`skills-loops-harness-split.html`](https://github.com/sycamore-hq/crossr-skills/blob/main/docs/plans/skills-loops-harness-split.html).

## What's here

### Conductors

| Skill | Role |
|-------|------|
| `avril` | Planning GAN — blessed PBIs (PO → QA → CTO) |
| `axel` | Execution loop — blessed PBI → PETC + code GAN |
| `brick` | BRICK conductor only — stages stay in the catalog |
| `orchestrator-prompt` | Generate a stateless loop-runner prompt |

### Personas

- AVRIL: `avril-conductor`, `planning-architect`, `product-owner`, `qa-architect`, `visionary-cto`
- AXEL: `axel-conductor`
- Code GAN: `reviewer`, `tester`, `architect`
- BRICK stages: `brick-specifier`, `brick-coder`, `brick-refactorer`, `brick-mutator` *agents* (the stage *skills* stay in skills)

### OpenCode prompt bodies

Conductor agent entrypoints are generated from `axel-conductor-agent` and `avril-conductor-agent` (`<role>-conductor-agent` → `<role>`). Command bodies: `templates/harness/opencode/command/{avril,axel}.md`.

A target bootstrapped before the harness loops pin moves to `v1-cards` may still carry an unmarked `.opencode/agent/avril.md`. Delete that file by hand before regen — the never-overwrite rule will otherwise protect it forever.

### Pipeline law

`book/src/pipeline/{overview,avril,axel,brick}.md` is the SSOT for those chapters.

### Graphs (topology)

[`graphs/`](graphs/) — `avril`, `axel`, `brick`, `code-gan`, `flagship`. Catalog skills by name. No Rhai. `just graphs-verify`.

## Not here

- BRICK stage skills (`brick-specifier`, `brick-coder`, `brick-refactorer`, `brick-mutator`) — catalog
- `skill-evaluator` + skill GAN agents — catalog
- `dashboard-prompt`, `chief-of-staff` — harness
- `HARNESS-SPEC.md`, bootstrap, dashboard — harness

## Sibling remotes

- [crossr-skills](https://github.com/sycamore-hq/crossr-skills) — catalog
- [crossr-harness](https://github.com/sycamore-hq/crossr-harness) — spec, bootstrap, dashboard
- [crossr-web-landing](https://github.com/sycamore-hq/crossr-web-landing) — public front door
