# Labor — Claude Code setup

Hand this page to a setup Bot. Do not paste it into the Chief-of-Staff profile.

**Setup Claude Code on my computer (Grok Bot box)**

You are setting up the local Claude Code CLI so this agent can spawn `claude -p` sessions (bridge reviews, write-ups). Do the work yourself. Do not invent paths. Do not print the token.

### 1. CLI
- Check: `which claude && claude --version`
- If missing, install the official Claude Code native CLI into `~/.local/bin` (or the box-standard location) and confirm `claude` is on `PATH`.
- Target shape that works: `/home/box/.local/bin/claude`, version like `2.1.266 (Claude Code)`.

### 2. Auth = setup-token, NOT an API key
- Auth is a long-lived token from `claude setup-token` (Claude subscription), exported as `CLAUDE_CODE_OAUTH_TOKEN`.
- Never ask the user to paste the token into chat.
- Request it with a secure secret field: secret name `CLAUDE_CODE_OAUTH_TOKEN`, label like "Claude Code setup-token".
- Do not print, echo, log, or `cat` the token. Verify only with presence/length and `claude auth status`.

### 3. Persist into the box secret store
The secret may appear in `process.env` for new shells, but sandbox restores can unset it. Persist it (without printing) into `/home/box/agent-data/box-secrets.json`:

```bash
python3 <<'PY'
import json, os
tok = os.environ.get("CLAUDE_CODE_OAUTH_TOKEN", "")
path = "/home/box/agent-data/box-secrets.json"
assert tok, "CLAUDE_CODE_OAUTH_TOKEN not in env"
data = {"version": 1, "secrets": {}}
if os.path.exists(path):
    try:
        data = json.load(open(path))
    except Exception as e:
        raise SystemExit(f"box-secrets.json unreadable, not overwriting: {e}")
if not isinstance(data.get("secrets"), dict):
    data["secrets"] = {}
data["secrets"]["CLAUDE_CODE_OAUTH_TOKEN"] = tok
if os.path.exists(path):
    os.chmod(path, 0o600)
fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
with os.fdopen(fd, "w") as f:
    json.dump(data, f)
stored = json.load(open(path))["secrets"].get("CLAUDE_CODE_OAUTH_TOKEN", "")
print("store_set", bool(stored), "len", len(stored), "match", stored == tok)
PY
```

### 4. Loader for every spawn
Ensure `/home/box/.config/claude/load-oauth.sh` exists and sources the store without echoing the value:

```bash
# Load Claude Code OAuth token from Grok Bot secret store into the environment.
# Do not echo the value.
_claude_secrets="/home/box/agent-data/box-secrets.json"
if [ -f "$_claude_secrets" ]; then
  _tok="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["secrets"].get("CLAUDE_CODE_OAUTH_TOKEN",""))' "$_claude_secrets" 2>/dev/null || true)"
  if [ -n "$_tok" ]; then
    export CLAUDE_CODE_OAUTH_TOKEN="$_tok"
  fi
  unset _tok
fi
unset _claude_secrets
```

Before every `claude` spawn:

```bash
. /home/box/.config/claude/load-oauth.sh
# confirm without leaking:
python3 -c 'import os; t=os.environ.get("CLAUDE_CODE_OAUTH_TOKEN",""); print("token_set", bool(t), "len", len(t))'
claude auth status
```

Expect `loggedIn: true`, `authMethod: "oauth_token"`. Optional smoke (no secrets in output):

```bash
claude -p "Reply with exactly: AUTH_OK" --output-format text </dev/null
```

If the store is empty later, re-request the secret securely and re-persist. Do not retry bridge work until `claude auth status` is green.

### 5. Required for the Reviewer / Fix job classes
Install `github-pr-review` and `github-pr-fix` so the slashes resolve. Prefer the consumer's skills pin. Clone `main` only when the pin tree lacks the skill; say so in the report.

`<repo>` is the active add-project. No lockfile → stop and card; do not clone.

```bash
test -f /workspace/<repo>/lockfile.toml || { echo "no lockfile; card"; exit 1; }
PIN="$(sed -n 's/^skills *= *"\([^"]*\)".*/\1/p' /workspace/<repo>/lockfile.toml)"
test -n "$PIN" || { echo "no skills pin; card"; exit 1; }
SRC="/workspace/.crossr/skills/$PIN/.agents/skills/github-pr-review"
FIX="/workspace/.crossr/skills/$PIN/.agents/skills/github-pr-fix"
if [ ! -d "$SRC" ] || [ ! -d "$FIX" ]; then
  rm -rf /tmp/crossr-skills-clone
  git clone --depth 1 --filter=blob:none --sparse https://github.com/sycamore-hq/crossr-skills.git /tmp/crossr-skills-clone
  cd /tmp/crossr-skills-clone
  git sparse-checkout set .agents/skills/github-pr-review .agents/skills/github-pr-fix
  SRC=/tmp/crossr-skills-clone/.agents/skills/github-pr-review
  FIX=/tmp/crossr-skills-clone/.agents/skills/github-pr-fix
fi
mkdir -p /home/box/.claude/skills
rm -rf /home/box/.claude/skills/github-pr-review /home/box/.claude/skills/github-pr-fix
cp -a "$SRC" /home/box/.claude/skills/
cp -a "$FIX" /home/box/.claude/skills/
```

Probe. A box without the skill directory must fail. Review probes stay non-mutating.

```bash
test -f /home/box/.claude/skills/github-pr-review/SKILL.md || { echo SKILL_MISSING; exit 1; }
. /home/box/.config/claude/load-oauth.sh
claude -p "/github-pr-review Do not review anything. Reply with the exact first heading of the skill body you loaded for this command, or NO_SKILL if none loaded." --disallowedTools "Edit,Write,NotebookEdit,Bash(git push*),Bash(gh pr merge*)" --output-format text </dev/null
# expect: PR Review
```

Without that heading, report Claude as `BACKEND: red` for Reviewer and Fix even when `claude auth status` is OK.

### 6. Spawn pattern
New empty process every job. Load oauth first. Write the brief to a file; do not inline it. Review spawns stay non-mutating. Fix spawn is mutating: edits and pushes the unit branch only; still no merge. A fix run ends with a commit pushed on the unit branch; report the HEAD SHA (rule 5).

Review:

```bash
. /home/box/.config/claude/load-oauth.sh
cat > /tmp/labor-brief.md <<'BRIEF'
<paste the brief verbatim>
BRIEF
claude -p "/github-pr-review $(cat /tmp/labor-brief.md)" --disallowedTools "Edit,Write,NotebookEdit,Bash(git push*),Bash(gh pr merge*)" --output-format text </dev/null
```

Fix:

```bash
. /home/box/.config/claude/load-oauth.sh
cat > /tmp/labor-brief.md <<'BRIEF'
<paste the brief verbatim>
BRIEF
claude -p "/github-pr-fix $(cat /tmp/labor-brief.md)" --permission-mode acceptEdits --allowedTools "Bash(git add *),Bash(git commit *),Bash(git push origin <unit-branch>*)" --disallowedTools "Bash(gh pr merge*),Bash(git push origin main*)" --output-format text </dev/null
```

Redirect stdin with `</dev/null` so `-p` does not hang waiting for pipe input.

Labor does not BLESS, REJECT, or merge. The seat that hired you emits the token. A GitHub review labor submitted is an artifact; the Reviewer seat posts the witness.

### 7. Done when
- `claude` on PATH with a real version
- `CLAUDE_CODE_OAUTH_TOKEN` in `/home/box/agent-data/box-secrets.json` (chmod 600)
- `load-oauth.sh` loads it; `claude auth status` shows oauth login
- `github-pr-review/SKILL.md` present; probe first heading is `PR Review`
- Report paths + versions to the user; never report the token

