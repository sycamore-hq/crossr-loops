# Labor backends

Labor is optional hired help on a destination. It is not a seat, not a pipeline stage, not a destination, and not a catalog skill.

A destination hosts the pipeline. Labor produces artifacts. Seats vote.

```
user  ↔→  Chief-of-Staff
                │
                ├─ AVRIL / AXEL seats     (vote, emit SEAT tokens)
                └─ Labor backends         (produce artifacts; never vote)
                        ├─ Cursor Cloud Agent   repo-mutate off-box
                        ├─ Claude Code CLI      `claude -p` on the box
                        └─ OpenCode Go CLI      `opencode run -m` on the box
```

Hands — the Bot editing `/workspace` itself — stay the last-resort engine. Silence at the probe = hands.

The official Destinations labor backends are Cursor Cloud Agent, Claude Code, and OpenCode Go. Those three have contracts and setup pastes in this chapter. The user may name additional backends for this team. An extra uses the name the user gave. It does not get an official contract unless one is written here. Extra labor still never votes, never merges.

OpenCode-as-destination (`/avril`, `/axel` in the OpenCode TUI) is not OpenCode-as-labor (`opencode run`). This chapter names the second only.

Setup pastes, handed to a setup Bot, not dumped into the Chief-of-Staff profile:

- [Cursor Cloud Agent](labor-cursor.md)
- [Claude Code](labor-claude.md)
- [OpenCode Go](labor-opencode.md)

## Hard rules

1. Labor never writes `BLESS`, `REJECT`, or merge. The seat that hired it emits the SEAT token. A token the labor process wrote is invalid. A GitHub review that labor submitted is an artifact, not a witness. The Reviewer seat reads it, then posts its own review starting with `SEAT: Reviewer` and its `code-review:` line. Only that URL is the witness.
2. When a backend is green for that job class, the seat does not implement with its own hands.
3. Asked backend red → card. No silent fallback to hands unless the user said "do it yourself" this turn.
4. Add-project still clones `/workspace/<name>`. That tree is for board, `gh`, `audit-plan`, `audit-packet`, and the named check. Cursor "never clone" is labor-host law for CloudAgent runs, not working-tree law.
5. After any mutating labor run (CloudAgent, on-box `--auto`, or a Claude fix spawn), believe GitHub (PR URL + HEAD SHA), not the agent story. Uncommitted labor edits are not a result.
6. Model law is roles, not IDs. `ROLE CHEAP` = generation, verification, test authoring, draft diffs. `ROLE SMART` = architecture, planning, judgement review. Confirm live OpenCode ids with `opencode models`.
7. Do not route Grok-via-OpenCode-Go from a Grok Bot.
8. Secrets: secret-request by name, persist `/home/box/agent-data/box-secrets.json` chmod 600, never print. `CURSOR_API_KEY` only if CloudAgent cannot list. Claude is `CLAUDE_CODE_OAUTH_TOKEN` from `claude setup-token`, not an API key. OpenCode is `OPENCODE_API_KEY` from the Go sub.
9. Deck-review law is unchanged. Reviewer briefs start with `/github-pr-review` (nits on). Fix briefs start with `/github-pr-fix` (nits on).
10. Lights-off merge still refused. Chief-of-Staff still never votes.
11. No fifth remote. No catalog skill. No pin move.
12. Destinations uses Cursor Cloud Agent, Claude Code, OpenCode Go. No other names for these engines.

## Stand-up probe

After the team exists, or when the user asks for labor, Chief-of-Staff probes once. Report status, never secrets. The report is how everyone knows what is available.

Official three:

```
Cursor CloudAgent list scope=all: OK | FAIL | unset
Claude  `. /home/box/.config/claude/load-oauth.sh; claude --version && claude auth status`: OK | FAIL | unset
OpenCode `. /home/box/.config/opencode/load-go.sh; opencode --version && opencode auth list`: OK | FAIL | unset
```

Then each extra the user named. Probe only the way the user said. Do not invent a probe, a secret name, or a spawn. Missing any of those → `unset` and ask how; do not enable that extra.

`AVAILABLE` = official backends that reported OK + extras that reported OK.

```
AVAILABLE: cursor claude
```

Missing Cursor tool auth → try CloudAgent first. Only if that fails, secret-request `CURSOR_API_KEY`.
Missing Claude → secret-request `CLAUDE_CODE_OAUTH_TOKEN`. Hand [labor-claude.md](labor-claude.md) to a setup Bot if the CLI is missing.
Missing OpenCode → secret-request `OPENCODE_API_KEY`. Hand [labor-opencode.md](labor-opencode.md) if the CLI is missing.

Then ask once, listing `AVAILABLE` and hands. Do not offer a backend that is not on `AVAILABLE`. Silence = hands. Do not enable labor uninvited.

## Brief line

Add to every seat brief when labor is on:

```
LABOR: <one of AVAILABLE, or hands>
AVAILABLE: <OK names from the last probe>
ROLE: CHEAP|SMART
BACKEND: green|red|unset
```

`LABOR` is not a closed three-name enum. Official tokens are `cursor`, `claude`, `opencode`. Any other token is an extra the user named. `hands` is always allowed.

`red` is a Chief-of-Staff probe token, not a seat dispatch token. Rule 3 cards before a brief goes out.

- `green` — seat may dispatch; no hands
- `unset` — labor off; seat uses hands
- `red` — Chief-of-Staff already carded; seat waits; no hands unless the user said "do it yourself" this turn

A seat should not see `red`. If one arrives anyway, wait. Do not invent a fallback.

Default assignment when LABOR is not named in the user turn. A default applies only if that backend is on `AVAILABLE`. If it is not, ask from `AVAILABLE`. Named LABOR this turn wins if it is on `AVAILABLE`; if named and not available → card (rule 3).

| Job class | ROLE | Backend if on AVAILABLE |
| --- | --- | --- |
| Generator code / draft diff / test authoring | CHEAP | Cursor if mutate; else OpenCode |
| Mechanical verify write-up | CHEAP | OpenCode |
| Generator plan / Architect / Planning Architect / CTO | SMART | Ask from AVAILABLE. Named LABOR this turn wins. Silence = hands. |
| Reviewer (`/github-pr-review`) | SMART | Claude (artifact); seat posts the witness review |
| Fix (`/github-pr-fix`) | CHEAP | Claude |

Mechanical green (`just check`, `audit-plan`, `audit-packet`) stays Chief-of-Staff on `/workspace`. Labor does not replace those recipes.

## Dated pin table (examples, 2026-09-10, not law)

OpenCode Go models change. Confirm provider string and ids with `opencode auth list` and `opencode models` on the box. Do not freeze `opencode` vs `opencode-go`. Official Go docs write `opencode-go/<id>`; the CLI may print `opencode/<id>`. Use whatever those two commands print.

ROLE CHEAP examples: `glm-5.3-flash`, `deepseek-v4.1-flash`, `mimo-v2.5`, `qwen3.8-flash`.
ROLE SMART examples: `kimi-k3`, `glm-5.3`, `qwen3.8-max`, `kimi-k2.7-code`.

Do not pin Grok 4.6, GPT 5.6 Luna, or any Grok-via-Go id from a Grok Bot.

Claude Code and Cursor pick their own models. Do not route those through OpenCode Go.

## Engine contracts

### Cursor Cloud Agent

Follow the managed `code-changes` skill when `/home/box/agent-data/managed-skills/skills/code-changes/SKILL.md` exists. Prefer the `CloudAgent` tool. `CURSOR_API_KEY` only if the tool cannot list. Launch with a clear prompt, repo URL (or `new_repo: true`), optional `starting_ref`. Do not clone onto the box for the implement pass. Do not launch an implement agent unless this turn asked. After a run: PR URL + HEAD SHA from GitHub.

Setup: [labor-cursor.md](labor-cursor.md).

### Claude Code CLI

Binary on PATH, target shape `/home/box/.local/bin/claude`. Auth is `claude setup-token` → `CLAUDE_CODE_OAUTH_TOKEN`. Load `/home/box/.config/claude/load-oauth.sh` before every spawn. New empty process every job. Redirect stdin `</dev/null`. Write the brief to a file; do not inline it. Plan and review spawns stay non-mutating on `/workspace`. Each spawn gets its own `mktemp -d /tmp/labor.XXXXXX` for the brief and, on review, the scratch worktree and draft; no push, no merge. Fix spawn: `--permission-mode acceptEdits`, edit and push the unit branch only, `gh pr merge` and the default branch stay denied. Ends with a pushed commit; report the HEAD SHA. Commands in [labor-claude.md](labor-claude.md).

Install `github-pr-review` and `github-pr-fix` from the skills pin into `~/.claude/skills/`. Probe: `SKILL.md` present and the first heading of the loaded skill is `PR Review`. Without that, Claude is `BACKEND: red` for the Reviewer and Fix job classes even when `claude auth status` is OK.

Setup: [labor-claude.md](labor-claude.md).

### OpenCode Go CLI

Binary on PATH via the official install. Auth is the Go API key from https://opencode.ai/auth. Secret name `OPENCODE_API_KEY`. Loader `/home/box/.config/opencode/load-go.sh`. Spawn from `/workspace/<repo>`:

```
. /home/box/.config/opencode/load-go.sh
cat > /tmp/labor-brief.md <<'BRIEF'
<paste the brief verbatim>
BRIEF
opencode run -m <provider>/<id> "$(cat /tmp/labor-brief.md)"
```

Confirm `--dir` / `--format` / `--auto` against `opencode run --help` on that box. `--auto` only when the brief needs writes, and only after `/home/box/.config/opencode/opencode.json` denies `git push` to the default branch, `git push --force`, `gh pr merge`, and `gh pr ready`. `--auto` still honors those deny rules. A write job ends with a commit on the unit branch, pushed. Report the HEAD SHA. Mechanical runs on that SHA, not on the working tree. Uncommitted labor edits are not a result. Plan and review jobs stay non-mutating. Confirm IDs with `opencode models`. Do not invent an id. `/avril` and `/axel` are destination commands, not this spawn.

Setup: [labor-opencode.md](labor-opencode.md).

## Refuse

- Fifth remote, catalog skill, pin move.
- Labor emitting BLESS / REJECT / merge / `gh pr merge`.
- Silent hands fallback.
- Printing tokens or keys.
- Treating `/avril` `/axel` as a labor spawn.
- Rewriting add-project to "never clone."
- Routing Grok-via-Go from a Grok Bot.
- Lights-off merge.
- Freehand review when deck-review law applies.
- Other names for Cursor Cloud Agent, Claude Code, or OpenCode Go.
