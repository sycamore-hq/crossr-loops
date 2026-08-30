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
