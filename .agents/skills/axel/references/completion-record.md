## Outputs

### PBI Completion Record

```markdown
# PBI Completion — <id>

## Title
…

## Plan
- path: docs/plans/pbi/<id>.plan.md
- C-01: BLESS | REJECT — evidence: …
- C-02: …

## Phases
- phase-1: BLESS tester | reviewer — commit <sha> (architect: <claim id> BLESS | REJECT, only if escalated)
- …

## AC Evidence
- [x] AC1 — evidence: …
- [x] AC2 — evidence: …

## Verification matrix
- <command>: PASS

## Board
- status: done (or review)

## Follow-ups / non-goals honored
…
```

### Session Completion Summary

When the authorized set is finished (or session stops):

- List completed PBI ids + commits
- List remaining ready / blocked items
- Explicit statement: execution complete for scope X (not a silent partial)

HTML preferred for human-facing summaries; Markdown OK for agent handoff.
