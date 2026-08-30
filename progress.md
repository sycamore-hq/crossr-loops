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
- `scripts/verify-skill-refs` (`just verify-skill-refs`): every graph `uses.skill` / `requires.skills` resolves to a SKILL.md in this checkout or the pinned catalog; every `uses.persona` resolves to `.agents/agents/<name>.md`. Scope addition beyond the plan — loops#4 shipped dangling `rust-architect` that only a human grep caught.

## Verification Status
- `./scripts/verify-graphs`: PASS
- `./scripts/verify-protocol`: PASS
- `./scripts/verify-skill-refs`: PASS
