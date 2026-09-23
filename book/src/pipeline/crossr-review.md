# CrossR Review — sidecar

Skill: `crossr-review` · Persona: `crossr-review-conductor-agent`

This is a sidecar, not a flagship node. It does not sit on the Intent → AVRIL → AXEL path. Topology: [`graphs/crossr-review.json`](https://github.com/sycamore-hq/crossr-loops/blob/main/graphs/crossr-review.json). If the graph and the conductor `SKILL.md` disagree, the skill wins.

## Purpose

Run Review Agent then Fix Agent on one named GitHub pull request until the review returns zero issues and zero questions, then stamp APPROVE.

The conductor **never reviews the diff** and **never edits the PR branch**. It resolves inputs, launches the two sub-agents, and stamps a clean result.

## Loop

Slash-only: `/crossr-review`. Resolve a PR from the invocation or conversation (URL, `owner/repo#N`, `pr N`, `#N`). None → ask and stop.

Review Agent (`github-pr-review`) then Fix Agent (`github-pr-fix`; the user asked for nits). No `--model-review` or `--model-fix` → ask which model runs that agent and wait. Do not pick one. `q` threads are not fixes.

- Only-`q` → stop, no Fix, no stamp.
- Dirty / max-rounds (default 8) → no stamp.
- Clean → GitHub APPROVE, or a conversation comment containing `APPROVED` if self-approve is refused.

`--model-review` and `--model-fix` are harness parameters. A provided id is a raw session slug. There is no default model and no short-id map.

## Related

- [Pipeline overview](overview.md) — flagship stays AVRIL → AXEL
- Graph: [`graphs/crossr-review.json`](https://github.com/sycamore-hq/crossr-loops/blob/main/graphs/crossr-review.json)
