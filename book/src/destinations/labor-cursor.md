# Labor — Cursor Cloud Agent setup

Hand this page to a setup Bot. Do not paste it into the Chief-of-Staff profile.

**Setup Cursor for this Grok Bot environment. Do the work; report results with URLs/status only. Do not paste secrets into chat.**

### Goal
Make this bot able to create and run Cursor cloud agents on the signed-in Cursor account, hand off repo-mutate work, and verify GitHub access used with those agents.

Repo work goes to Cursor cloud agents. Do not clone a repo onto this box for the purpose of implementing it. A `/workspace/<name>` clone that already exists for board / `gh` / `just check` stays. That clone is not the implementer.

If `/home/box/agent-data/managed-skills/skills/code-changes/SKILL.md` exists, read it and follow it.

### Steps
1. **Verify the CloudAgent tool**
   Inspect the `cursor` namespace tool `CloudAgent`. Run `action: list` with `scope: "all"` and `limit: 5`.
   - Success = auth is already wired through the signed-in Cursor user. Prefer this tool for launch/list/get/reply/watch.
   - Do not ask the user to paste an API key into chat.

2. **Optional raw API key (only if needed)**
   If a workflow requires `POST https://api.cursor.com/v1/agents` with `Authorization: Bearer $CURSOR_API_KEY` and `CURSOR_API_KEY` is unset:
   - Request it with a secret-request named `CURSOR_API_KEY` (masked input → env var).
   - Never echo, log, or paste the key.
   - Persist into `/home/box/agent-data/box-secrets.json` chmod 600 the same way other box secrets persist. Print only `store_set` / `len` / `match`.
   - Smoke-test with a read-only `GET https://api.cursor.com/v1/agents?limit=1` using the env var. Expect `200`.

3. **Verify source control**
   Confirm `gh auth status` (or the SCM the user uses with Cursor: GitHub / GitLab / Bitbucket / Azure DevOps).
   Run CloudAgent `action: repositories` (or `gh repo list`) and confirm the orgs/repos this bot should touch are visible.
   v1 CrossR Destinations is GitHub only. Other forges: report them, do not launch.

4. **Smoke test (non-destructive)**
   - CloudAgent list (done above).
   - Optionally `get` on one finished agent if any exist.
   - Do **not** launch an implement agent unless asked this turn.

5. **Operating rules**
   - Launch with a clear prompt, repo URL (or `new_repo: true` for greenfield), optional `starting_ref`.
   - Review / fix prompts start with `/github-pr-review` or `/github-pr-fix` (nits on). No freehand review essay.
   - Prefer CloudAgent over hand-rolling curl when the tool can do the job.
   - After a run finishes: believe GitHub (PR URL + HEAD SHA), not the agent's story.
   - Never paste API keys, tokens, or cookies into chat.
   - Never implement repo changes with your own hands when CloudAgent is green.
   - Labor does not BLESS, REJECT, or merge. The seat that hired you emits the token.

### Done-when
Reply with:
- CloudAgent list: OK / FAIL (+ error)
- `CURSOR_API_KEY`: set / unset / requested
- `gh` (or SCM): account + OK / FAIL
- One line: "Cursor ready" or the blocker and what you need
