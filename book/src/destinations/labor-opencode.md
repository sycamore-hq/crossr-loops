# Labor — OpenCode Go setup

Hand this page to a setup Bot. Do not paste it into the Chief-of-Staff profile.

**Setup OpenCode Go on my computer (Grok Bot box)**

You are wiring this box so the agent can spawn `opencode run` as labor. Do the work. Do not invent paths. Do not print the key.

OpenCode-the-destination (`/avril`, `/axel` in the TUI) is not this job. Do not treat a TUI slash as a spawn.

### 1. CLI
- Check: `which opencode && opencode --version`
- If missing: official install into `~/.local/bin` (`curl -fsSL https://opencode.ai/install | bash` or the box-standard path). Confirm `opencode` is on `PATH`.
- Target shape: `/home/box/.local/bin/opencode` with a real version string.

### 2. Auth = OpenCode Go API key, not a chat paste
- Secret name `OPENCODE_API_KEY`, label "OpenCode Go API key".
- User gets it from https://opencode.ai/auth after subscribing to Go, or from an existing TUI `/connect` → OpenCode Go.
- Never ask them to paste the key into chat. Secret-request only.
- Do not print, echo, log, or `cat` the key. Verify presence and length only.

### 3. Persist into the box secret store
```bash
python3 <<'PY'
import json, os
tok = os.environ.get("OPENCODE_API_KEY", "")
path = "/home/box/agent-data/box-secrets.json"
assert tok, "OPENCODE_API_KEY not in env"
data = {"version": 1, "secrets": {}}
if os.path.exists(path):
    try:
        data = json.load(open(path))
    except Exception:
        pass
if not isinstance(data.get("secrets"), dict):
    data["secrets"] = {}
data["secrets"]["OPENCODE_API_KEY"] = tok
with open(path, "w") as f:
    json.dump(data, f)
os.chmod(path, 0o600)
stored = json.load(open(path))["secrets"].get("OPENCODE_API_KEY", "")
print("store_set", bool(stored), "len", len(stored), "match", stored == tok)
PY
```

Also write creds into OpenCode's own store without echoing. Prefer the CLI (`opencode auth`) if it can take a key non-interactively. Fallback: write `~/.local/share/opencode/auth.json` mode 600 using the provider id the CLI already uses after `/connect` (confirm with `opencode auth list` — names only, no keys). Official Go docs write `opencode-go`. The CLI may print `opencode`. Use whatever `opencode auth list` prints. Do not invent a second store format. Do not print the file.

### 4. Loader
Ensure `/home/box/.config/opencode/load-go.sh` exists and sources the store without echoing the value:

```bash
# Load OpenCode Go key from Grok Bot secret store into the environment.
# Do not echo the value.
_oc_secrets="/home/box/agent-data/box-secrets.json"
if [ -f "$_oc_secrets" ]; then
  _tok="$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["secrets"].get("OPENCODE_API_KEY",""))' "$_oc_secrets" 2>/dev/null || true)"
  if [ -n "$_tok" ]; then
    export OPENCODE_API_KEY="$_tok"
  fi
  unset _tok
fi
unset _oc_secrets
```

Before every spawn:

```bash
. /home/box/.config/opencode/load-go.sh
python3 -c 'import os; t=os.environ.get("OPENCODE_API_KEY",""); print("token_set", bool(t), "len", len(t))'
opencode auth list
opencode models
```

Expect Go models on the list. Report the provider prefix those commands print (`opencode/` or `opencode-go/`). If the list is empty, re-request the secret. Do not guess IDs.

### 5. Roles, not IDs
- LABOR_CHEAP — generation, verification, test authoring, draft diffs
- LABOR_SMART — architecture, planning, judgement review

Dated examples 2026-09-10. Refresh from `opencode models` on this box. Skip any id that is not listed.

- cheap: `glm-5.3-flash`, `deepseek-v4.1-flash`, `mimo-v2.5`, `qwen3.8-flash`
- smart: `kimi-k3`, `glm-5.3`, `qwen3.8-max`, `kimi-k2.7-code`

Never route a Grok model through Go from a Grok Bot (`grok-*` on whatever prefix the list printed).

### 6. Spawn
New empty process every job. Load first. Confirm live flag names with `opencode run --help` on this box. If `--dir` exists, point it at `/workspace/<repo>`. If it does not, `cd` into that tree first. Do not invent `--yolo`. If a permission flag exists and is required for headless (`--auto` is the documented one), name it in the report and use it only when the brief needs writes.

`-m` takes the exact `provider/id` string `opencode models` printed.

```bash
. /home/box/.config/opencode/load-go.sh
cd /workspace/<repo>
opencode run -m <provider>/<id> "<brief>"
```

Optional smoke (no secrets in output). Pick a cheap id that `opencode models` actually listed:

```bash
opencode run -m <provider>/<cheap-id> "Reply with exactly: AUTH_OK"
```

Labor does not BLESS, REJECT, or merge. The seat that hired you emits the token.

### 7. Skills
OpenCode reads `.agents/skills` from the repo working tree. Do not copy `github-pr-review` into `~/.claude`. Reviewer briefs that need the slash still prefer Claude Code when that backend is green. OpenCode labor may load the same skill from the pin checkout if the user asked for OpenCode on that review.

### 8. Done when
- `opencode` on PATH with a real version
- `OPENCODE_API_KEY` in `/home/box/agent-data/box-secrets.json` (chmod 600)
- `load-go.sh` loads it; `opencode models` lists Go models
- One AUTH_OK-style smoke
- Report paths + versions + one cheap id + one smart id actually listed. Never the key.

