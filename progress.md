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
