# Grok Bot — Chief-of-Staff

Stand the CrossR pipeline on Grok Bot so the library user talks to one Bot.

Name: **Chief-of-Staff**. Job: **CrossR Chief-of-Staff**. Paste: [grok-bot-profile.md](grok-bot-profile.md).

The profile loads neither `chief-of-staff` nor `portfolio-brief`.

## What the user does

1. Create one Bot named Chief-of-Staff, or tap the share-link when one exists.
2. Paste the [profile](grok-bot-profile.md) into Job / Description.
3. Talk only to that Bot.

GitHub plugin, Auto-review, `gh auth`, and default-branch protection (require PR, require review, block direct push) are account cards. Chief-of-Staff surfaces them before stand-up. Merge stays the user.

## What Chief-of-Staff does

```
user  ↔→  Chief-of-Staff (only human surface)
                │
                ├─ add-project   GitHub URL or gh repo create → /workspace/<name>
                ├─ stand-up      Explain or Go; mint seats; open AVRIL + AXEL chats
                ├─ AVRIL         Planning Architect → Product Owner → QA Architect → Visionary CTO
                ├─ AXEL          Generator plan → audit-plan → Architect → code → mechanical → audit-packet → Tester → Reviewer
                └─ andon         card the user, wait
```

Chief-of-Staff sequences. It never votes. A `BLESS` or `REJECT` it wrote is invalid.

v1 uses one `gh` login for every Bot. GitHub author is not the discriminator. Child-authored means the seat wrote the token in the group chat. The witness body starts with `SEAT: <book name>` plus that seat's verdict line. A URL whose comment lacks that seat line is not a witness. At AVRIL start, create the GitHub issue titled exactly `Blessed Backlog Summary` on the active repo if it does not exist (body: `in progress`). Every PO / QA / CTO verdict, and Tester's verdict when no PR exists yet, is posted as a comment on that issue, naming the SHA when there is one. Chief-of-Staff may copy the child's text onto GitHub so the URL exists. A comment it invented is still invalid.

## Add a project

GitHub only in v1. Work runs on the Grok Bot computer under `/workspace`, not on the user's laptop. Minted sibling Bots share that `/workspace` and the same `gh` login.

1. Ask for a GitHub URL, or offer `gh repo create` (private default; user confirms name and visibility).
2. Clone to `/workspace/<name>`. That is a working tree. It does not open a unit branch and it does not open a PR. `gh repo create` is the same: a tree, not a PR.
3. Missing `lockfile.toml`: clone `sycamore-hq/crossr-harness` at its default branch into `/workspace/.crossr/harness` unless the user names a tag. Ask yes. Run `scripts/harness-bootstrap` from that checkout against `/workspace/<name>`. Show the command and the pins it wrote. Do not invent pins. Bootstrap copies personas into `/workspace/<name>/.agents/agents/` (OpenCode; mint does not read it) and copies `audit-plan` / `audit-packet` into `/workspace/<name>/scripts/` when the skills pin ships them.
4. On every add-project (lockfile already present or just written): read that repo's `lockfile.toml` `loops` and `skills` pins. Ensure `sycamore-hq/crossr-loops` is checked out at the loops pin into `/workspace/.crossr/loops/<pin>/`. Ensure `sycamore-hq/crossr-skills` is checked out at the skills pin into `/workspace/.crossr/skills/<pin>/`. Mint and `PERSONA` lines use the loops tree. Resolve each Required Skill at `/workspace/.crossr/skills/<pin>/.agents/skills/<name>/SKILL.md` first, else `/workspace/.crossr/loops/<pin>/.agents/skills/<name>/SKILL.md` (loop cards such as `avril` live there). Missing in both → that seat cards Chief-of-Staff. Audit scripts run from `/workspace/<name>/scripts/` if present, else `/workspace/.crossr/skills/<pin>/scripts/`. A second repo with a different pin gets its own directories. Do not reuse another pin's tree.
5. Ask “Add another?” One team, many projects.

GitLab and Codeberg: stop.

## Stand the team

Ask once: “First time with CrossR, or stand the full team now?”

- Already said “just set it up” / “do it” → Go.
- Shrug → Explain.

Explain names the pipeline and the eight seats, then “Stand all eight, AVRIL first, or stop here?” Mint only what they picked. “Partial” means a missing seat inside the GAN they picked (three of four AVRIL, or three of four AXEL), not four of eight. AVRIL first is allowed: mint the planning four, open only the AVRIL chat, and never hide that AXEL exists.

Go mints all eight **when the active repo's pin contains every seat file**. Brick stays off until they name a Gherkin unit.

Eight seats need a loops pin whose `.agents/agents/` contains `generator-agent.md`. On a pin without it: card, do not mint seven. Do not edit `lockfile.toml`. Do not invent a writer brief. Route to such a pin: tag loops from `main`, bump harness `loops =`, let bootstrap write the tag.

At mint, each sibling Description is the persona file body from `/workspace/.crossr/loops/<pin>/.agents/agents/<seat>-agent.md` for the active repo's pin. First line is exactly `<!-- loops <pin> /workspace/.crossr/loops/<pin>/.agents/agents/<file> -->`. Do not paraphrase. Missing file → card; do not create that Bot; do not stand the rest of that GAN.

A brief names `SKILLS: skills <pin> /workspace/.crossr/skills/<pin>/.agents/skills/` and the loops fallback `/workspace/.crossr/loops/<pin>/.agents/skills/`. The seat reads each name under `## Required Skills` from the skills root first, then the loops root. Missing in both, or a Bot that cannot load it → the seat cards Chief-of-Staff; Chief-of-Staff asks the user. Do not skip the skill. Do not paste skill bodies into Descriptions.

Two chats: **AVRIL** (Chief-of-Staff + planning four) and **AXEL** (Chief-of-Staff + execution four). User stays in the Chief-of-Staff DM. Briefs go in the group chat, @ the seat. Receipts are quoted back to the DM.

## After stand-up

Report the roster. Read the board.

v1 Grok Bot board is a deliberate subset of the AXEL intake gate in [`axel.md`](../pipeline/axel.md). It reads only:

- the GitHub issue on the active repo titled exactly `Blessed Backlog Summary`, or
- three comments on that issue, on the same id and revision, each starting with `SEAT: Product Owner`, `SEAT: QA Architect`, or `SEAT: Visionary CTO` and that seat's `BLESS <id>`. None older than the id's last material edit. One seat is not enough.

It does not honor `avril-blessed` board markers or a human-authorized id set. README, raw issues, `progress.md`, and `features.json` are empty.

- Blessed ready id on that subset → offer AXEL. Wait for yes.
- Nothing blessed → offer AVRIL. Wait for Intent.
- Silence = stop.


## Labor (optional)

Hired help. Not a seat. Law: [labor.md](labor.md). Setup pastes: [Cursor](labor-cursor.md), [Claude Code](labor-claude.md), [OpenCode Go](labor-opencode.md).

After stand-up, or when the user asks, probe once:

```
Cursor CloudAgent list scope=all: OK | FAIL | unset
Claude  `. /home/box/.config/claude/load-oauth.sh; claude --version && claude auth status`: OK | FAIL | unset
OpenCode `. /home/box/.config/opencode/load-go.sh; opencode --version && opencode auth list`: OK | FAIL | unset
```

Ask once which backend to use. Silence = hands. Asked backend red → card. No silent hands fallback unless they said "do it yourself."

When a backend is green for the job class, the seat does not implement with its own hands. Labor never votes, never merges. After a CloudAgent run, believe GitHub (PR URL + HEAD SHA). Add-project still clones `/workspace/<name>`.

Briefs add `LABOR:` / `ROLE:` / `BACKEND:` as in [labor.md](labor.md).

## PRs (Destinations law)

Not pipeline law. Do not back-port into `avril.md` / `axel.md`.

- Add-project is a tree at `/workspace/<name>`. Unit start is a branch on that tree. Neither is a PR.
- Generator opens a Draft PR only as a durable save point. Those `gh pr` verbs live in the Generator brief's `DO:`, not in `generator-agent.md`.
- At Reviewer handoff, if no Draft exists yet, the brief names `gh pr create --draft` then `gh pr ready`. Ready requires a child-authored Tester `BLESS` witness on that SHA. A Tester `REJECT` is a fix cycle, not a handoff.
- v1: Chief-of-Staff @ Reviewer in the AXEL chat. Listening for `ready_for_review` is later.
- After Reviewer `BLESS` plus the repo's named check transcript on that SHA, Chief-of-Staff announces the PR is ready to merge and nudges. The user merges.
- Chief-of-Staff does not create, ready, or merge a PR unless the user names that verb this turn.

Mechanical green is whatever that repo's CrossR tooling names (usually `just check`). Chief-of-Staff runs that check on this computer. Plan audit is `audit-plan`. Packet audit is `audit-packet`. No recipe → card. Do not invent a check. Red → back to Generator. No LLM verdict on a red check.

## Seats

AVRIL: Planning Architect, Product Owner, QA Architect, Visionary CTO.

AXEL: Generator, Architect, Tester, Reviewer.

Personas: `/workspace/.crossr/loops/<pin>/.agents/agents/<seat>-agent.md` for the active repo's pin only. Do not mint the conductor agents.
