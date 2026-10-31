#!/usr/bin/env python3
"""Cross-cutting books ride alongside the per-PBI language book (SYC-19).

A cross-cutting book (`metadata.scope: "cross-cutting"`, e.g. system-design)
is disclosed every session from the pinned catalog. It never enters the
one-language-book-per-PBI choice. The planner gets its card, Rules, and routed
references; the architecture gate gets Rules only; execute, review, and test
get nothing.

Calculations are pure. Reading the tree is the action.
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PARAMS = ROOT / ".agents" / "skills" / "axel" / "references" / "harness-parameters.md"


def section(text: str, heading: str) -> str:
    start = text.index(heading)
    rest = text[start + len(heading):]
    end = re.search(r"(?m)^\*\*When |^### |^## ", rest)
    return rest[: end.start()] if end else rest


class CrossCuttingDisclosure(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.params = PARAMS.read_text()
        cls.cross = section(cls.params, "**Cross-cutting books")
        cls.multi = section(cls.params, "**When `books` names more than one book:**")

    def test_language_selection_ignores_cross_cutting_books(self):
        self.assertIn("Only language books count here", self.multi)
        self.assertIn("never enter this choice", self.multi)
        self.assertIn("not chosen per PBI", self.cross)

    def test_disclosed_from_the_pinned_catalog_with_resolved_identity(self):
        self.assertIn('metadata.scope: "cross-cutting"', self.cross)
        self.assertIn("pinned skills catalog", self.cross)
        self.assertIn("not listed in `lockfile.toml`", self.cross)
        self.assertRegex(self.cross, r"names each one with the skills pin")

    def test_planner_gets_card_rules_and_routed_references(self):
        line = next(l for l in self.cross.splitlines() if l.startswith("- Generator (plan)"))
        for needle in ("card", "RULES.md", "references its card routes", "plan-writer"):
            self.assertIn(needle, line)

    def test_gate_gets_rules_only(self):
        line = next(l for l in self.cross.splitlines() if l.startswith("- `architecture` gate"))
        self.assertIn("RULES.md", line)
        self.assertIn("Never its `references/`", line)

    def test_execution_agents_get_nothing(self):
        line = next(l for l in self.cross.splitlines() if l.startswith("- Generator (execute)"))
        for agent in ("testing", "code-review", "test verifier"):
            self.assertIn(agent, line)
        self.assertIn("no cross-cutting book", line)

    def test_audit_uses_the_same_disclosed_set(self):
        self.assertIn("audit-plan", self.cross)
        self.assertIn("same disclosed set", self.cross)


if __name__ == "__main__":
    unittest.main()
