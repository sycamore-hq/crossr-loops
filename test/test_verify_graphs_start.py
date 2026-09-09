#!/usr/bin/env python3
"""R0 acceptance: every graph names its entry node.

Decision 3: start is a required key that names a node. Document order
and in-degree are not the rule.

Calculations are pure. Loading the script is the action.
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


def fixture(**overrides: object) -> dict:
    graph = {
        "apiVersion": "crossr-loops/v0",
        "kind": "Graph",
        "name": "fixture",
        "start": "intake",
        "nodes": [{"id": "intake", "role": "gate"}],
        "edges": [],
    }
    graph.update(overrides)
    return graph


class Calculations(unittest.TestCase):
    def test_missing_start_fails(self):
        graph = fixture()
        del graph["start"]
        check = vg.Check()
        vg.validate("fixture", graph, {"fixture"}, check)
        self.assertTrue(
            any("fixture: missing start" in msg for msg in check.bad),
            check.bad,
        )

    def test_dangling_start_fails(self):
        graph = fixture(start="nope")
        check = vg.Check()
        vg.validate("fixture", graph, {"fixture"}, check)
        self.assertTrue(
            any("fixture: start 'nope' is not a node" in msg for msg in check.bad),
            check.bad,
        )

    def test_present_start_passes(self):
        graph = fixture()
        check = vg.Check()
        vg.validate("fixture", graph, {"fixture"}, check)
        self.assertEqual(check.bad, [])
        self.assertTrue(
            any("start intake" in msg for msg in check.ok),
            check.ok,
        )


if __name__ == "__main__":
    unittest.main()
