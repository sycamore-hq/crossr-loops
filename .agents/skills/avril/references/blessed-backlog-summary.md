### Output: Blessed Backlog Summary

When the loop completes, produce a concise summary (HTML preferred when the audience is human; Markdown OK for agent handoff):

```markdown
# Blessed Backlog Summary

## Intent
…

## Blessed PBIs (ordered)
- ID — title — points? — deps

## Explicit cuts (scope_out / rejected ideas)
…

## Open questions for execution (must be empty or human-accepted)
…

## Blessing log
- <id>: PO BLESS | QA BLESS | CTO BLESS
```

Optional: append a single line to the harness progress log that planning is blessed. Do not mark implementation features complete.
