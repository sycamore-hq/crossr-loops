# Destinations

A destination is a host that runs the CrossR pipeline. It is not the pipeline.

Pipeline law stays in [`book/src/pipeline/`](../pipeline/overview.md):

```
Intent → AVRIL → Blessed Backlog → AXEL → Done
```

This book part teaches how to stand that pipeline on a machine. Grok Bot is chapter 1. OpenCode (`/avril`, `/axel`) is another destination and stays in getting-started until that chapter moves here. Do not treat Grok Bot as the product.

Labor backends are optional workers on a destination. They are not a destination and not the pipeline. OpenCode as a TUI destination is not OpenCode as `opencode run` labor.

## Chapters

- [Grok Bot — Chief-of-Staff](grok-bot.md)
- [Chief-of-Staff profile](grok-bot-profile.md) — the paste
- [Labor backends](labor.md)
- [Labor — Cursor Cloud Agent](labor-cursor.md) — setup paste
- [Labor — Claude Code](labor-claude.md) — setup paste
- [Labor — OpenCode Go](labor-opencode.md) — setup paste

## Hard rules

1. No fifth remote. Destinations live in `crossr-loops`.
2. No catalog skill. This is not `crossr-skills`.
3. Personas stay in `.agents/agents/`. Load them. Do not paraphrase them.
4. Do not mint `avril-conductor-agent` or `axel-conductor-agent` as Grok Bots. Chief-of-Staff is the chair.
5. Labor never votes, never merges. Seats emit the SEAT token.
