# Grok Bot — Chief-of-Staff

Stand the CrossR pipeline on Grok Bot so the library user talks to one Bot.

Name: **Chief-of-Staff**. Job: **CrossR Chief-of-Staff**. Paste: [grok-bot-profile.md](grok-bot-profile.md).

The colliding harness skill is `chief-of-staff` (renamed to `portfolio-brief` in a pending harness PR). The profile loads neither.

## What the user does

1. Create one Bot named Chief-of-Staff, or tap the share-link when one exists.
2. Paste the [profile](grok-bot-profile.md) into Job / Description.
3. Talk only to that Bot.

GitHub plugin, Auto-review, and `gh auth` are account cards. Chief-of-Staff surfaces them. Merge stays the user.

## What Chief-of-Staff does

```
user  ↔→  Chief-of-Staff (only human surface)
                │
                ├─ add-project   GitHub URL or gh repo create → /workspace/<name>
                ├─ stand-up      Explain or Go; mint seats; open AVRIL + AXEL chats
                ├─ AVRIL         Planning Architect → Product Owner → QA Architect → Visionary CTO
                ├─ AXEL          Generator (plan → Architect) → code → mechanical → Tester → Reviewer
                └─ andon         card the user, wait
```

Chief-of-Staff sequences. It never votes. A `BLESS` or `REJECT` it wrote is invalid.

## Add a project

GitHub only in v1. Work runs on the Grok Bot computer under `/workspace`, not on the user's laptop.

1. Ask for a GitHub URL, or offer `gh repo create` (private default; user confirms name and visibility).
2. Clone to `/workspace/<name>`.
3. Missing `lockfile.toml`: name `sycamore-hq/crossr-harness` `scripts/harness-bootstrap`, ask yes, run the official tool against that repo, show the pins it wrote. Do not invent pins. Bootstrap also copies personas into `/workspace/<name>/.agents/agents/`. That copy is the project's OpenCode tree. Mint does not read it.
4. Clone `sycamore-hq/crossr-loops` at the `loops` pin into `/workspace/.crossr/loops`. This is the only persona root mint reads: `/workspace/.crossr/loops/.agents/agents/`.
5. Ask “Add another?” One team, many projects.

GitLab and Codeberg: stop. A clone is not a draft PR.

## Stand the team

Ask once: “First time with CrossR, or stand the full team now?”

- Already said “just set it up” / “do it” → Go.
- Shrug → Explain.

Explain names the pipeline and the eight seats, then “Stand all eight, AVRIL first, or stop here?” Mint only what they picked.

Go mints all eight **when the pin contains every seat file**. Brick stays off until they name a Gherkin unit.

Eight seats require a `loops` pin at or after the commit that adds `generator-agent.md`. The current published pin `v1-cards` does not have that file. On a pin missing any seat file: card the user. Do not mint a partial team. Do not invent a writer brief to paper over the hole.

At mint, each sibling Description is the persona file body from `/workspace/.crossr/loops/.agents/agents/<seat>-agent.md`. First line is that absolute path plus the pin. Do not paraphrase. Missing file → card; do not create that Bot; do not stand the rest of that GAN.

Two chats: **AVRIL** (Chief-of-Staff + planning four) and **AXEL** (Chief-of-Staff + execution four). User stays in the Chief-of-Staff DM. Briefs go in the group chat, @ the seat. Receipts are quoted back to the DM.

## After stand-up

Report the roster. Read the board.

v1 Grok Bot board is a deliberate subset of the AXEL intake gate in [`axel.md`](../pipeline/axel.md). It reads only:

- a Blessed Backlog Summary in the repo, or
- GitHub comments with child-authored `BLESS <id>` from the AVRIL seats.

It does not honor `avril-blessed` board markers or a human-authorized id set. README, raw issues, `progress.md`, and `features.json` are empty.

- Blessed ready id on that subset → offer AXEL. Wait for yes.
- Nothing blessed → offer AVRIL. Wait for Intent.
- Silence = stop.

## PRs (Destinations law)

Not pipeline law. Do not back-port into `avril.md` / `axel.md`.

- Unit start = branch only. No PR.
- Generator opens a Draft PR only as a durable save point.
- Generator marks Ready for review when handing the write to Reviewer. That is the Generator → Reviewer handoff, not permission to merge. Ready requires Tester `BLESS` on that SHA (or a brief that names the handoff). A Tester `REJECT` is a fix cycle, not a handoff.
- v1: Chief-of-Staff @ Reviewer in the AXEL chat. Listening for `ready_for_review` is later.
- After Reviewer `BLESS` plus the repo's named check transcript on that SHA, Chief-of-Staff announces the PR is ready to merge and nudges. The user merges.
- Chief-of-Staff does not create, ready, or merge a PR unless the user names that verb this turn.

Mechanical green is whatever that repo's CrossR tooling names (usually `just check`). No recipe → card. Do not invent a check.

## Seats

AVRIL: Planning Architect, Product Owner, QA Architect, Visionary CTO.

AXEL: Generator, Architect, Tester, Reviewer.

Personas: `/workspace/.crossr/loops/.agents/agents/<seat>-agent.md` only. Do not mint the conductor agents.
