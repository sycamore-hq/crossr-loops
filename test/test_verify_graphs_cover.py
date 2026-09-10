#!/usr/bin/env python3
"""R3 acceptance: verify-graphs reads `graph-runner cover` output.

Decision 6: when runner/Cargo.toml exists, verify-graphs shells out to
`cargo run -q -p graph-runner -- cover graphs` and fails on any uncovered
edge. The subprocess call is the only action; these tests exercise the pure
parse and judgement over captured output. No cargo call here.
"""

from __future__ import annotations

import importlib.machinery
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

_vg_spec = importlib.util.spec_from_loader(
    "verify_graphs",
    importlib.machinery.SourceFileLoader(
        "verify_graphs", str(ROOT / "scripts" / "verify-graphs")
    ),
)
vg = importlib.util.module_from_spec(_vg_spec)
assert _vg_spec.loader is not None
_vg_spec.loader.exec_module(vg)

FULL = "taken 37/37 edges\nuncovered edges: 0\n"
PARTIAL = (
    "taken 35/37 edges\n"
    "avril: cto --REJECT--> generator\n"
    "code-gan: architect --REJECT--> generate\n"
    "uncovered edges: 2\n"
)


class ParseCover(unittest.TestCase):
    def test_count_found(self):
        self.assertEqual(vg.parse_cover(FULL), 0)
        self.assertEqual(vg.parse_cover(PARTIAL), 2)
        self.assertEqual(vg.parse_cover("uncovered edges: 14\n"), 14)

    def test_count_missing(self):
        self.assertIsNone(vg.parse_cover(""))
        self.assertIsNone(vg.parse_cover("taken 37/37 edges\n"))
        self.assertIsNone(vg.parse_cover("uncovered edges: many\n"))
        self.assertIsNone(vg.parse_cover("  uncovered edges: 0\n"))

    def test_line_must_be_whole(self):
        self.assertIsNone(vg.parse_cover("uncovered edges: 0 (approx)\n"))
        self.assertEqual(vg.parse_cover("noise\nuncovered edges: 3\nmore\n"), 3)


class JudgeCover(unittest.TestCase):
    def test_zero_count_and_exit_zero_pass(self):
        ok, bad = vg.judge_cover(0, FULL, "")
        self.assertEqual(bad, [])
        self.assertEqual(len(ok), 1)
        self.assertIn("uncovered edges: 0", ok[0])
        self.assertIn("taken 37/37 edges", ok[0])

    def test_non_zero_count_reported_with_the_uncovered_lines(self):
        ok, bad = vg.judge_cover(1, PARTIAL, "")
        self.assertEqual(ok, [])
        self.assertEqual(
            bad,
            [
                "cover: uncovered edges: 2",
                "cover: avril: cto --REJECT--> generator",
                "cover: code-gan: architect --REJECT--> generate",
            ],
        )

    def test_zero_count_with_non_zero_exit_fails(self):
        ok, bad = vg.judge_cover(1, FULL, "")
        self.assertEqual(ok, [])
        self.assertEqual(bad, ["cover: uncovered edges: 0"])

    def test_missing_count_reports_exit_and_stderr(self):
        stderr = "✗ graphs/walks/axel.happy.walk: walk 1 (axel): incomplete at code-gan:commit\n"
        ok, bad = vg.judge_cover(1, "axel: intake --blessed--> plan-write\n", stderr)
        self.assertEqual(ok, [])
        self.assertEqual(
            bad,
            [
                "cover: graph-runner exited 1 without an `uncovered edges:` line",
                "cover: axel: intake --blessed--> plan-write",
                "cover: ✗ graphs/walks/axel.happy.walk: walk 1 (axel): "
                "incomplete at code-gan:commit",
            ],
        )

    def test_cover_command_is_the_documented_one(self):
        self.assertEqual(
            vg.COVER_CMD,
            ["cargo", "run", "-q", "-p", "graph-runner", "--", "cover", "graphs"],
        )
        self.assertEqual(vg.TOOLCHAIN, "rust-toolchain.toml")


if __name__ == "__main__":
    unittest.main()
