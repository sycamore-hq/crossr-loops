#!/usr/bin/env python3
"""PR 6 acceptance: plan-first AXEL. Architect blesses plans, not diffs.

Brief VALIDATE (work#11 / gan-layer-separation-plan §4 PR 6):
the plan node is the code Generator with plan-writer, not the conductor;
architect sits at plan time; code-gan happy path is generate → mechanical
→ tester → reviewer → commit; code-time architect is escalation only;
code-writer is absent from the plan-time window.

Calculations are pure. Loading the tree is the action.
"""

from __future__ import annotations

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def node_by_id(graph: dict, nid: str) -> dict | None:
    for node in graph.get("nodes") or []:
        if node.get("id") == nid:
            return node
    return None


def uses(graph: dict, nid: str) -> dict:
    node = node_by_id(graph, nid) or {}
    return node.get("uses") or {}


def edges_from(graph: dict, nid: str) -> list[dict]:
    return [e for e in graph.get("edges") or [] if e.get("from") == nid]


def edge(graph: dict, frm: str, to: str) -> dict | None:
    for e in graph.get("edges") or []:
        if e.get("from") == frm and e.get("to") == to:
            return e
    return None


def happy_successors(graph: dict, start: str, stop: str) -> list[str]:
    """Walk unlabeled / pass / BLESS edges from start until stop."""
    ok = {None, "pass", "BLESS"}
    path = [start]
    here = start
    seen = {start}
    while here != stop:
        nxt = [
            e["to"]
            for e in edges_from(graph, here)
            if e.get("when") in ok
        ]
        if len(nxt) != 1:
            return path
        here = nxt[0]
        if here in seen:
            return path
        seen.add(here)
        path.append(here)
    return path


def conductor_writes_the_plan(graph: dict) -> bool:
    plan = uses(graph, "plan")
    write = uses(graph, "plan-write")
    persona = plan.get("persona") or write.get("persona")
    return persona == "axel-conductor-agent"


def plan_window_loads_code_writer(graph: dict) -> bool:
    for nid in ("plan", "plan-write"):
        if uses(graph, nid).get("skill") == "code-writer":
            return True
    return False


class GraphCalculations(unittest.TestCase):
    def test_conductor_as_plan_persona_is_the_bug(self):
        g = {
            "nodes": [
                {
                    "id": "plan",
                    "role": "generator",
                    "uses": {"persona": "axel-conductor-agent"},
                }
            ]
        }
        self.assertTrue(conductor_writes_the_plan(g))

    def test_plan_writer_skill_is_not_the_bug(self):
        g = {
            "nodes": [
                {
                    "id": "plan-write",
                    "role": "generator",
                    "catalog": True,
                    "uses": {"skill": "plan-writer"},
                }
            ]
        }
        self.assertFalse(conductor_writes_the_plan(g))
        self.assertFalse(plan_window_loads_code_writer(g))

    def test_happy_path_follows_bless_and_pass(self):
        g = {
            "edges": [
                {"from": "generate", "to": "mechanical"},
                {"from": "mechanical", "to": "tester", "when": "pass"},
                {"from": "mechanical", "to": "generate", "when": "fail"},
                {"from": "tester", "to": "reviewer", "when": "BLESS"},
                {"from": "tester", "to": "generate", "when": "REJECT"},
                {"from": "reviewer", "to": "commit", "when": "BLESS"},
            ]
        }
        self.assertEqual(
            happy_successors(g, "generate", "commit"),
            ["generate", "mechanical", "tester", "reviewer", "commit"],
        )


class LiveTree(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.axel = json.loads((ROOT / "graphs" / "axel.json").read_text())
        cls.code = json.loads((ROOT / "graphs" / "code-gan.json").read_text())
        cls.card = (ROOT / ".agents" / "skills" / "axel" / "SKILL.md").read_text()
        cls.conductor = (
            ROOT / ".agents" / "agents" / "axel-conductor-agent.md"
        ).read_text()
        cls.architect = (
            ROOT / ".agents" / "agents" / "architect-agent.md"
        ).read_text()
        cls.reviewer = (
            ROOT / ".agents" / "agents" / "reviewer-agent.md"
        ).read_text()
        cls.tester = (ROOT / ".agents" / "agents" / "tester-agent.md").read_text()
        cls.completion = (
            ROOT / ".agents" / "skills" / "axel" / "references" / "completion-record.md"
        ).read_text()
        cls.verification = (
            ROOT / ".agents" / "skills" / "axel" / "references" / "verification.md"
        ).read_text()
        cls.graph_md = (ROOT / "graphs" / "GRAPH.md").read_text()
        cls.book = (ROOT / "book" / "src" / "pipeline" / "axel.md").read_text()
        cls.command = (
            ROOT / "templates" / "harness" / "opencode" / "command" / "axel.md"
        ).read_text()
        cls.features = json.loads((ROOT / "features.json").read_text())
        cls.progress = (ROOT / "progress.md").read_text()

    def test_plan_node_is_plan_writer_not_the_conductor(self):
        self.assertFalse(conductor_writes_the_plan(self.axel))
        self.assertEqual(uses(self.axel, "plan-write").get("skill"), "plan-writer")
        self.assertTrue(node_by_id(self.axel, "plan-write").get("catalog"))

    def test_plan_window_does_not_load_code_writer(self):
        self.assertFalse(plan_window_loads_code_writer(self.axel))
        req = (self.axel.get("requires") or {}).get("skills") or []
        self.assertIn("plan-writer", req)
        self.assertNotIn("code-writer", req)

    def test_plan_audit_sits_before_the_architect(self):
        self.assertIsNotNone(edge(self.axel, "plan-write", "plan-audit"))
        passed = edge(self.axel, "plan-audit", "plan-architect")
        self.assertIsNotNone(passed)
        self.assertEqual(passed.get("when"), "pass")
        failed = edge(self.axel, "plan-audit", "plan-write")
        self.assertIsNotNone(failed)
        self.assertEqual(failed.get("when"), "fail")

    def test_plan_architect_blesses_into_code_gan(self):
        node = node_by_id(self.axel, "plan-architect")
        self.assertEqual(node.get("role"), "adversary")
        self.assertEqual(uses(self.axel, "plan-architect").get("skill"), "architecture")
        self.assertEqual(
            uses(self.axel, "plan-architect").get("persona"), "architect-agent"
        )
        bless = edge(self.axel, "plan-architect", "code-gan")
        self.assertEqual(bless.get("when"), "BLESS")
        reject = edge(self.axel, "plan-architect", "plan-write")
        self.assertEqual(reject.get("when"), "REJECT")

    def test_code_gan_happy_path_is_mechanical_then_tester_then_reviewer(self):
        self.assertEqual(
            happy_successors(self.code, "generate", "commit"),
            ["generate", "mechanical", "tester", "reviewer", "commit"],
        )
        self.assertNotIn("architect", happy_successors(self.code, "generate", "commit"))

    def test_code_gan_architect_is_escalation_only(self):
        incoming = [
            e for e in self.code.get("edges") or [] if e.get("to") == "architect"
        ]
        self.assertTrue(incoming, "escalation node has no inbound edge")
        for e in incoming:
            self.assertEqual(e.get("when"), "unsatisfiable-claim")
        fail = edge(self.code, "mechanical", "generate")
        self.assertEqual(fail.get("when"), "fail")

    def test_axel_card_moves_decompose_into_the_plan(self):
        self.assertRegex(self.card, r"(?i)plan-writer")
        self.assertRegex(self.card, r"(?i)inside the plan|plan content|Phases")
        self.assertNotIn("restart the full three-adversary chain", self.card)

    def test_axel_card_caps_the_plan_loop(self):
        self.assertRegex(self.card, r"(?i)three.*REJECT")

    def test_axel_card_keeps_scope_on_avril(self):
        self.assertRegex(self.card, r"(?i)scope change.*avril")
        self.assertRegex(self.card, r"(?i)does not re-bless|never re-bless")

    def test_axel_card_orders_mechanical_before_llm(self):
        self.assertRegex(self.card, r"(?i)mechanical")
        self.assertRegex(self.card, r"(?i)zero (LLM )?token|no LLM")

    def test_completion_record_names_the_plan(self):
        self.assertRegex(self.completion, r"(?m)^## Plan")

    def test_conductor_persona_delegates_the_plan(self):
        self.assertRegex(self.conductor, r"(?i)plan-writer")
        self.assertNotIn("full re-chain", self.conductor)

    def test_architect_persona_is_plan_time(self):
        self.assertRegex(self.architect, r"(?i)plan time")
        self.assertRegex(self.architect, r"(?i)underspecif")

    def test_reviewer_persona_is_conformance_plus_risk(self):
        self.assertRegex(self.reviewer, r"(?i)conformance")
        self.assertRegex(self.reviewer, r"(?i)unanticipated")

    def test_tester_persona_is_ac_and_regressions(self):
        self.assertRegex(self.tester, r"(?i)AC coverage|acceptance crit")
        self.assertRegex(self.tester, r"(?i)regression")

    def test_docs_describe_the_v2_chain(self):
        for text in (self.graph_md, self.book, self.command, self.verification):
            self.assertNotIn("Reviewer → Tester → Architect", text)
            self.assertNotIn("reviewer → tester → architect", text)

    def test_features_records_pr6b(self):
        phase = self.features["gan-layer-separation"]
        ids = {
            c["id"]
            for c in phase.get("commits") or []
            if c.get("status") == "completed"
        }
        self.assertIn("pr6b", ids)

    def test_progress_records_pr6b(self):
        self.assertRegex(
            self.progress,
            r"(?m)^### gan-layer-separation — PR 6b \(COMPLETED\)",
        )


if __name__ == "__main__":
    unittest.main()
