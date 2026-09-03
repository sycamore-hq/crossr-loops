# Migration

Clean copy from [`sycamore-hq/crossr-skills`](https://github.com/sycamore-hq/crossr-skills).
History stays on skills. This remote is a snapshot plus new work.

| Field | Value |
|-------|-------|
| Source remote | `sycamore-hq/crossr-skills` |
| Source SHA | `2c0b00976928c275e07c7ebc43b4b0e0f400b2ba` (`main` at copy) |
| Unit | split-03 |
| Method | clean copy (no filter-branch, no rewrite) |
| Skills copies | kept until split-07 (dual-publish, then delete) |

Do not treat this SHA as a lockfile pin. Pins are published tags (`loops = "v0"` as of split-08).

## Copied (26 files)

### Conductor skills

- `.agents/skills/avril/SKILL.md`
- `.agents/skills/axel/SKILL.md`
- `.agents/skills/brick/SKILL.md` (conductor only)
- `.agents/skills/rust-team-lead/SKILL.md`
- `.agents/skills/orchestrator-prompt/SKILL.md`
- `.agents/skills/orchestrator-prompt/assets/orchestrator-prompt-template.md`

### Personas

- `.agents/agents/planning-architect-agent.md`
- `.agents/agents/product-owner-agent.md`
- `.agents/agents/qa-architect-agent.md`
- `.agents/agents/visionary-cto-agent.md`
- `.agents/agents/axel-conductor-agent.md`
- `.agents/agents/rust-reviewer-agent.md`
- `.agents/agents/rust-tester-agent.md`
- `.agents/agents/rust-architect-agent.md`
- `.agents/agents/brick-specifier-agent.md`
- `.agents/agents/brick-coder-agent.md`
- `.agents/agents/brick-refactorer-agent.md`
- `.agents/agents/brick-mutator-agent.md`

### OpenCode prompt bodies

- `templates/harness/opencode/agent/avril.md`
- `templates/harness/opencode/agent/axel.md`
- `templates/harness/opencode/command/avril.md`
- `templates/harness/opencode/command/axel.md`

### Pipeline law

- `book/src/pipeline/overview.md`
- `book/src/pipeline/avril.md`
- `book/src/pipeline/axel.md`
- `book/src/pipeline/brick.md`

Copied byte-identical. Pipeline chapters still point at `scull7/crossr-skills` URLs and a book `getting-started` path that is not in this remote — left alone (no rewrite on copy day).

## Intentionally not copied

- `brick-specifier`, `brick-coder`, `brick-refactorer`, `brick-mutator` **skills** (stages stay in the catalog)
- `skill-evaluator-agent`, `skill-remediator-agent`, `skill-reviewer-agent`
- `dashboard-prompt`, `chief-of-staff` (harness, split-04)
- `HARNESS-SPEC.md`, bootstrap, dashboard, `/status` prompt bodies
- `.agents/agents/README.md` (mixed catalog + loop personas)

## Consumers

Install from [`sycamore-hq/crossr-harness`](https://github.com/sycamore-hq/crossr-harness) (`harness-bootstrap`). Current consumer pin: `loops = "v1-cards"` (split-08 first pin was `v0`). This repo's `AGENTS.md` / `features.json` / `progress.md` / `justfile` / `lockfile.toml` are a `--process-only` consumer instance (split-08), not loop law.

## Graphs (split-09)

JSON nodes+edges in `graphs/`. Topology only. Conductor `SKILL.md` files were not rewritten. Pin `v0` does not include graphs. Pin `v1-cards` does. Bootstrap still does not copy `graphs/`.

