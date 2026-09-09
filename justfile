default:
    @just --list

init:
    echo "environment ready"

check:
    cargo check --workspace --all-targets

test:
    python3 -m unittest discover -s test -v
    cargo test --workspace

runner-check:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings -W clippy::pedantic
    cargo test --workspace

graphs-check:
    cargo run -q -p graph-runner -- check graphs

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
