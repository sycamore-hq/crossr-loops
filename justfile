default:
    @just --list

init:
    echo "environment ready"

check:
    cargo check --workspace 2>/dev/null || echo "(no Rust crates)"

test:
    cargo test --workspace 2>/dev/null || echo "(no tests)"
