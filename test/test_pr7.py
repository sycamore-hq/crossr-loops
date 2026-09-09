#!/usr/bin/env python3
"""PR 7b acceptance: AVRIL set review + AXEL packet ritual.

Brief VALIDATE (work#12 / gan-layer-separation-plan §4 PR 7):
batch topology on po/qa/cto; per-item verdict declarations; no blanket
token; set-size cycle line; AXEL packets audited; review prose dropped.

Calculations are pure. Loading the tree is the action.
"""

from __future__ import annotations

import importlib.machinery
import importlib.util
import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

_vp_spec = importlib.util.spec_from_loader(
    "verify_protocol",
    importlib.machinery.SourceFileLoader(
        "verify_protocol", str(ROOT / "scripts" / "verify-protocol")
    ),
)
vp = importlib.util.module_from_spec(_vp_spec)
assert _vp_spec.loader is not None
_vp_spec.loader.exec_module(vp)

_vg_spec = importlib.util.spec_from_loader(
    "verify_graphs",
    importlib.machinery.SourceFileLoader(
        "verify_graphs", str(ROOT / "scripts" / "verify-graphs")
    ),
)
vg = importlib.util.module_from_spec(_vg_spec)
assert _vg_spec.loader is not None
_vg_spec.loader.exec_module(vg)

DECISION_3 = "A bare BLESS over a set is not a verdict."

AVRIL_MANDATE_5448096 = (
    "“Run AVRIL — Architect proposes PBIs, Product Owner then QA Architect then "
    "Visionary CTO each explicitly BLESS or REJECT, revise until unanimous — and "
    "stop at a blessed backlog without writing implementation yourself.”"
)

AXEL_MANDATE_5448096 = (
    "“Drive only blessed PBIs through PETC and a full code GAN until every "
    "acceptance criterion is evidenced and the board matches reality — without "
    "writing a line of implementation.”"
)

REVIEWER_VERDICT_5448096 = (
    "**Verdict format** (per `gan-verdict`): `code-review: BLESS | REJECT`"
)
TESTER_VERDICT_5448096 = (
    "**Verdict format** (per `gan-verdict`): `testing: BLESS | REJECT`"
)
ARCHITECT_VERDICT_5448096 = (
    "**Verdict format** (per `gan-verdict`): `architecture: BLESS | REJECT`"
)

CYCLE_LINE_RE = re.compile(
    r"^- cycle (\d+): set (\d+) · PO (\d+)/(\d+) · QA (\d+)/(\d+) · CTO (\d+)/(\d+)$"
)

PACKET_FIELD_NAMES = (
    "phase id",
    "k of n",
    "gate",
    "files",
    "diff",
    "AC",
    "claims",
    "prior verdicts",
    "envelope",
)


def batch_nodes(graph: dict) -> set[str]:
    return {
        node["id"]
        for node in graph.get("nodes") or []
        if isinstance(node, dict) and node.get("batch") is True
    }


class Calculations(unittest.TestCase):
    def test_batch_nodes_returns_batch_true_ids(self):
        graph = {
            "nodes": [
                {"id": "po", "batch": True},
                {"id": "qa", "batch": True},
                {"id": "cto", "batch": True},
                {"id": "reviewer"},
            ]
        }
        self.assertEqual(batch_nodes(graph), {"po", "qa", "cto"})

    def test_batch_persona_with_only_testing_reject_fails(self):
        text = f"BLESS <id>\nREJECT <id>\n{DECISION_3}\ntesting: REJECT\n"
        check = vp.Check()
        vp.check_batch_adversary("fixture", "qa", "qa-architect-agent", text, check)
        self.assertTrue(
            any("single-gate" in msg for msg in check.bad),
            check.bad,
        )

    def test_mixed_protocol_failure_names_surplus_not_absence(self):
        text = f"BLESS <id>\nREJECT <id>\n{DECISION_3}\n<gate>: BLESS\n"
        check = vp.Check()
        vp.check_batch_adversary("fixture", "qa", "qa-architect-agent", text, check)
        self.assertEqual(len(check.bad), 1, check.bad)
        self.assertNotIn("lacks", check.bad[0])
        self.assertIn(
            "declares both per-item and single-gate verdict shapes (`<gate>: BLESS`)",
            check.bad[0],
        )

    def test_verify_missing_batch_persona_is_one_failure(self):
        graph = {
            "nodes": [
                {
                    "id": "po",
                    "role": "adversary",
                    "batch": True,
                    "uses": {"persona": "no-such-agent"},
                }
            ],
            "edges": [
                {"from": "po", "to": "qa", "when": "BLESS"},
                {"from": "po", "to": "generator", "when": "REJECT"},
            ],
        }
        check = vp.Check()
        vp.verify("fixture", graph, check)
        persona_fails = [m for m in check.bad if "missing persona" in m]
        batch_fails = [m for m in check.bad if "batch adversary" in m]
        self.assertEqual(len(persona_fails), 1, check.bad)
        self.assertEqual(batch_fails, [])


class LiveTree(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads((ROOT / "graphs" / "schema.json").read_text())
        cls.avril = json.loads((ROOT / "graphs" / "avril.json").read_text())
        cls.axel = json.loads((ROOT / "graphs" / "axel.json").read_text())
        cls.code = json.loads((ROOT / "graphs" / "code-gan.json").read_text())
        cls.avril_card = (ROOT / ".agents" / "skills" / "avril" / "SKILL.md").read_text()
        cls.axel_card = (ROOT / ".agents" / "skills" / "axel" / "SKILL.md").read_text()
        cls.po = (ROOT / ".agents" / "agents" / "product-owner-agent.md").read_text()
        cls.qa = (ROOT / ".agents" / "agents" / "qa-architect-agent.md").read_text()
        cls.cto = (ROOT / ".agents" / "agents" / "visionary-cto-agent.md").read_text()
        cls.reviewer = (ROOT / ".agents" / "agents" / "reviewer-agent.md").read_text()
        cls.tester = (ROOT / ".agents" / "agents" / "tester-agent.md").read_text()
        cls.architect = (ROOT / ".agents" / "agents" / "architect-agent.md").read_text()
        cls.avril_conductor = (
            ROOT / ".agents" / "agents" / "avril-conductor-agent.md"
        ).read_text()
        cls.axel_conductor = (
            ROOT / ".agents" / "agents" / "axel-conductor-agent.md"
        ).read_text()
        cls.summary = (
            ROOT / ".agents" / "skills" / "avril" / "references" / "blessed-backlog-summary.md"
        ).read_text()
        cls.params = (
            ROOT / ".agents" / "skills" / "axel" / "references" / "harness-parameters.md"
        ).read_text()
        cls.handoff = (
            ROOT / ".agents" / "skills" / "axel" / "references" / "handoff-packet.md"
        ).read_text()
        cls.completion = (
            ROOT / ".agents" / "skills" / "axel" / "references" / "completion-record.md"
        ).read_text()

    def test_schema_allows_batch(self):
        props = self.schema["properties"]["nodes"]["items"]["properties"]
        self.assertIn("batch", props)
        self.assertEqual(props["batch"]["type"], "boolean")

    def test_verify_graphs_allowed_node_contains_batch(self):
        self.assertIn("batch", vg.ALLOWED_NODE)

    def test_verify_graphs_rejects_batch_on_non_adversary(self):
        graph = {
            "apiVersion": "crossr-loops/v0",
            "kind": "Graph",
            "name": "fixture",
            "nodes": [{"id": "intake", "role": "stage", "batch": True}],
            "edges": [],
        }
        check = vg.Check()
        vg.validate("fixture", graph, {"fixture"}, check)
        self.assertTrue(
            any(
                "fixture: node intake sets batch on role 'stage' — "
                "batch is adversary-only" in msg
                for msg in check.bad
            ),
            check.bad,
        )

    def test_avril_batch_nodes_are_exactly_po_qa_cto(self):
        self.assertEqual(batch_nodes(self.avril), {"po", "qa", "cto"})

    def test_no_code_gan_or_axel_node_is_batch(self):
        self.assertEqual(batch_nodes(self.code), set())
        self.assertEqual(batch_nodes(self.axel), set())

    def test_each_batch_persona_declares_per_item(self):
        for text in (self.po, self.qa, self.cto):
            self.assertEqual(vp.batch_declaration_gaps(text), [])
            self.assertFalse(vp.declares_single_gate(text))

    def test_avril_card_set_review_law(self):
        self.assertIn(DECISION_3, self.avril_card)
        self.assertRegex(self.avril_card, r"(?i)one verdict line per id")
        self.assertIn("audit-packet verdict", self.avril_card)
        self.assertIn("siblings", self.avril_card)
        self.assertIn("harness-parameters", self.avril_card)
        self.assertNotRegex(
            self.avril_card,
            r"(?i)max(imum)? (batch|set) size|at most \d+ (items|PBIs)",
        )

    def test_blessed_backlog_summary_has_cycle_template(self):
        template = next(
            line for line in self.summary.splitlines() if line.startswith("- cycle ")
        )
        filled = (
            template.replace("<n>", "1")
            .replace("<size>", "2")
            .replace("<bless>", "3")
            .replace("<reject>", "4")
        )
        self.assertRegex(filled, CYCLE_LINE_RE)

    def test_card_byte_caps(self):
        avril_n = (ROOT / ".agents" / "skills" / "avril" / "SKILL.md").stat().st_size
        axel_n = (ROOT / ".agents" / "skills" / "axel" / "SKILL.md").stat().st_size
        self.assertLessEqual(avril_n, 5400, f"avril card is {avril_n} B (wc -c)")
        self.assertLessEqual(axel_n, 6100, f"axel card is {axel_n} B (wc -c)")

    def test_axel_card_packet_ritual(self):
        self.assertIn("audit-packet brief", self.axel_card)
        self.assertIn("audit-packet verdict", self.axel_card)
        self.assertIn("scratch", self.axel_card)
        self.assertIn("drop", self.axel_card)
        self.assertIn("review prose", self.axel_card)
        self.assertNotIn("paste the board", self.axel_card)
        self.assertNotIn("pinto list --json", self.axel_card)

    def test_harness_parameters_names_scratch_path(self):
        self.assertIn("Packet scratch path", self.params)
        self.assertIn("${TMPDIR:-/tmp}/", self.params)
        avril_params = (
            ROOT / ".agents" / "skills" / "avril" / "references" / "harness-parameters.md"
        ).read_text()
        self.assertIn("Verdict scratch path", avril_params)
        self.assertIn("${TMPDIR:-/tmp}/", avril_params)
        self.assertIn(".verdict.md", avril_params)

    def test_handoff_packet_transcribes_prior_verdicts(self):
        self.assertIn("transcribed to the prior-verdict line shape", self.handoff)
        self.assertIn("- <gate> REJECT:", self.handoff)

    def test_no_packet_path_under_docs_or_pinto(self):
        hits = []
        roots = (
            ROOT / ".agents",
            ROOT / "book",
            ROOT / "templates",
        )
        for base in roots:
            for path in base.rglob("*"):
                if not path.is_file():
                    continue
                for lineno, line in enumerate(
                    path.read_text(errors="ignore").splitlines(), 1
                ):
                    if "packet" not in line.lower():
                        continue
                    for raw in re.findall(r"`([^`]+)`|(\S+)", line):
                        token = raw[0] or raw[1]
                        if re.match(r"^(docs/|\.pinto/)", token):
                            rel = path.relative_to(ROOT)
                            hits.append(f"{rel}:{lineno}:{token}")
        self.assertEqual(hits, [])

    def test_handoff_packet_names_item_8_fields(self):
        for name in PACKET_FIELD_NAMES:
            self.assertIn(name, self.handoff, name)

    def test_completion_record_drops_review_prose(self):
        self.assertIn("review prose dropped", self.completion)

    def test_conductor_mandates_unchanged_from_5448096(self):
        self.assertIn(AVRIL_MANDATE_5448096, self.avril_conductor)
        self.assertIn(AXEL_MANDATE_5448096, self.axel_conductor)

    def test_axel_verdict_format_lines_unchanged(self):
        self.assertIn(REVIEWER_VERDICT_5448096, self.reviewer)
        self.assertIn(TESTER_VERDICT_5448096, self.tester)
        self.assertIn(ARCHITECT_VERDICT_5448096, self.architect)

    def test_verify_protocol_batch_reports_missing_sentence(self):
        graph = {
            "nodes": [
                {
                    "id": "reviewer",
                    "role": "adversary",
                    "batch": True,
                    "uses": {"persona": "reviewer-agent"},
                }
            ]
        }
        text = "BLESS <id>\nREJECT <id>\n"
        node = graph["nodes"][0]
        check = vp.Check()
        vp.check_batch_adversary(
            "fixture",
            node["id"],
            node["uses"]["persona"],
            text,
            check,
        )
        self.assertTrue(
            any(
                "fixture: batch adversary reviewer persona reviewer-agent "
                "lacks per-item verdict declaration" in msg
                and DECISION_3 in msg
                for msg in check.bad
            ),
            check.bad,
        )


if __name__ == "__main__":
    unittest.main()
