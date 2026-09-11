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
    except Exception:
        pass
if not isinstance(data.get("secrets"), dict):
    data["secrets"] = {}
data["secrets"]["CLAUDE_CODE_OAUTH_TOKEN"] = tok
with open(path, "w") as f:
    json.dump(data, f)
os.chmod(path, 0o600)
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

### 5. Optional but required for CrossR-style PR reviews
Install the `github-pr-review` skill so `/github-pr-review` resolves:

```bash
rm -rf /tmp/crossr-skills-clone
git clone --depth 1 --filter=blob:none --sparse https://github.com/sycamore-hq/crossr-skills.git /tmp/crossr-skills-clone
cd /tmp/crossr-skills-clone
git sparse-checkout set .agents/skills/github-pr-review
mkdir -p /home/box/.claude/skills
rm -rf /home/box/.claude/skills/github-pr-review
cp -a .agents/skills/github-pr-review /home/box/.claude/skills/
```

Prefer the consumer's skills pin when a `lockfile.toml` exists. The clone above is the fallback when this box has no pin checkout yet.

Probe:

```bash
claude -p "/github-pr-review reply with exactly SKILL_OK and stop" --permission-mode bypassPermissions --output-format text </dev/null
```

### 6. Spawn pattern
New empty process every job. Load oauth first. For PR review:

```bash
. /home/box/.config/claude/load-oauth.sh
claude -p "/github-pr-review {brief}" --permission-mode bypassPermissions --output-format text </dev/null
```

Redirect stdin with `</dev/null` so `-p` does not hang waiting for pipe input.

Labor does not BLESS, REJECT, or merge. The seat that hired you emits the token.

### 7. Done when
- `claude` on PATH with a real version
- `CLAUDE_CODE_OAUTH_TOKEN` in `/home/box/agent-data/box-secrets.json` (chmod 600)
- `load-oauth.sh` loads it; `claude auth status` shows oauth login
- (If needed) `/github-pr-review` returns `SKILL_OK`
- Report paths + versions to the user; never report the token

