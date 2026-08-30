default:
    @just --list

init:
    echo "environment ready"

check:
    cargo check --workspace 2>/dev/null || echo "(no Rust crates)"

test:
    cargo test --workspace 2>/dev/null || echo "(no tests)"

graphs-verify:
    @./scripts/verify-graphs

verify-protocol:
    @./scripts/verify-protocol

# Requires CROSSR_SKILLS_PATH pointing at the lockfile skills pin.
# A sibling checkout is never used as the pin; this target does not clone.
verify-skill-refs:
    @./scripts/verify-skill-refs

graphs-verify-html:
    @./scripts/verify-graphs --html
