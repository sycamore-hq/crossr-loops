## Harness Context (Stratified Disclosure)

This is a harness-layer planning orchestration skill. It coordinates backlog consensus inside a project harness that supplies:

- Intent source (PRD, conversation, prototype notes, ADR set, or equivalent)
- The **board** — which one, and by what channel you reach it (see Board I/O below). The blessed backlog lands there; there is no tracking file beside it.
- Agent personas under the harness agents directory (typical: `planning-architect-agent`, `product-owner-agent`, `qa-architect-agent`, `visionary-cto-agent`)
- Verdict scratch path — disclosed by the harness (HARNESS-SPEC §12); fallback `${TMPDIR:-/tmp}/crossr-packets/<cycle-or-set-id>/<adversary>.verdict.md`; never in the tree.

The skill definition itself is portable and harness-agnostic. Concrete board identity, access channel, ID prefixes, and the exact pre-flight ritual are parameters of the invoking harness and are disclosed at activation. The invariants (strict Generator → three-adversary order, explicit BLESS language, planning-only stop, no silent scope creep) are enforced uniformly.

### Board I/O (stratified)

You need four capabilities from the board. Which product provides them, and
whether you reach it through MCP tools, a CLI, an HTTP API, or a human relaying
for you, is disclosed — never assumed, never required to be any one of them.

| Capability | What you do with it |
|---|---|
| **Read items** | the existing backlog, so you revise rather than duplicate |
| **Create and revise** | one item per PBI, carrying every field in the portable shape from the AVRIL card |
| **Order** | rank, and dependency links between items |
| **Record** | the blessing chain on the item, so a later session can see what was blessed and by whom |

Rules that hold whatever the channel:

- **Confirm which board and which project** before the first write. A blessed
  backlog filed against the wrong project is worse than none.
- **Stable ids across revise cycles.** An id that moves breaks every reference
  the plan, the commits, and the adversaries already made to it.
- **Never invent a second competing tracker.** Not a file in the repo, not a
  list in the transcript. If the board is unreachable this session, emit the
  portable shape for a human to file and say clearly that nothing was filed.
- **Dry-run multi-step mutations** where the channel offers it, and only once
  the user has authorized writes.
