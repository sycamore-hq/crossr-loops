# Handoff packet (AXEL fill-in)

AXEL fills `gan-verdict` item 8. Grammar lives there: `gan-verdict` `references/handoff-packet.md`. Do not copy it. This page says **which field comes from where**.

Write the file to the disclosed scratch path (`references/harness-parameters.md`). Never inside the repo. Never committed.

| Field | Source |
|-------|--------|
| phase id | Blessed plan `## Phases` — this phase's id |
| k of n | Blessed plan `## Phases` — this phase's index and the count |
| gate | The adversary about to read: `testing` / `code-review` / `architecture` |
| files | `git diff --stat` of this phase |
| diff | `git diff` of this phase (fenced, or a ref) |
| AC | AC subset this phase claims, from the plan |
| claims | Claim ids from the plan that this phase covers |
| prior verdicts | This phase's earlier gates, transcribed to the prior-verdict line shape |
| envelope | `gan-verdict` item 5 (conductor selects fields) |

`audit-packet brief` before the adversary reads. On reply: `audit-packet verdict --gate <gate>`. Transcribe the verdict into the next packet's prior verdicts as `- <gate> BLESS` or `- <gate> REJECT: <≤140 chars>` (gan-verdict `references/handoff-packet.md`), not the reply line verbatim.

Never: a sibling SKILL.md, a previous-phase essay, the whole board, dashboard HTML.
