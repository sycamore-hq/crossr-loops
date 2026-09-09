# Batch review

AVRIL delegates a **set**, not one id at a time. Details live here; the card stays small.

## Delegation shape

One prompt, this order:

1. The adversary persona
2. The `avril` card (gate)
3. The active set (portable PBI fields for each id, in set order)
4. The envelope **last** (`gan-verdict` item 5)

Persona + card are the stable prefix. The set and envelope vary.

## Reply grammar

Do not copy the grammar. One law, one home: `gan-verdict` `references/batch-verdict.md`.

Each id gets one line: `BLESS <id>` or `REJECT <id> — <blockers>`. A bare BLESS over a set is not a verdict.

The conductor runs `audit-packet verdict --items <ids>` before reading a reply. Red → re-delegate that adversary.

## Per-item loop

`REJECT <id>` loops **that id** alone. Unchanged siblings keep their BLESS (card item 3; Ruthless Checklist “Material edit … for that item”).

Worked set `{T-1, T-2, T-3}` — PO replies:

```
BLESS T-1 — thinnest slice that ships the tag
REJECT T-2 — no observable done; cut or add AC
BLESS T-3 — scope_out names the extras
```

T-2 returns to the Generator. T-1 and T-3 keep the PO BLESS. QA and CTO have not yet spoken on this cycle; when they do, they review the set then in play. A later material edit to T-1 invalidates T-1's three blessings only.

## Decision 9

No cap; the set-size log is the instrument; a cap is a later measured decision.
