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


def persona_declares_per_item(text: str) -> bool:
    return (
        "BLESS <id>" in text
        and "REJECT <id>" in text
        and DECISION_3 in text
    )


def persona_mixes_protocols(text: str) -> bool:
    return persona_declares_per_item(text) and vp.declares_single_gate(text)


def cycle_line(
    n: int,
    size: int,
    po: tuple[int, int],
    qa: tuple[int, int],
    cto: tuple[int, int],
) -> str:
    return (
        f"- cycle {n}: set {size} · PO {po[0]}/{po[1]} · "
        f"QA {qa[0]}/{qa[1]} · CTO {cto[0]}/{cto[1]}"
    )


def parse_cycle_line(line: str) -> dict:
    match = CYCLE_LINE_RE.match(line)
    if not match:
        raise ValueError(f"not a cycle line: {line!r}")
    n, size, pb, pr, qb, qr, cb, cr = (int(g) for g in match.groups())
    return {
        "n": n,
        "size": size,
        "po": (pb, pr),
        "qa": (qb, qr),
        "cto": (cb, cr),
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

    def test_persona_declares_per_item_needs_bless_id(self):
        text = f"REJECT <id>\n{DECISION_3}\n"
        self.assertFalse(persona_declares_per_item(text))

    def test_persona_declares_per_item_needs_reject_id(self):
        text = f"BLESS <id>\n{DECISION_3}\n"
        self.assertFalse(persona_declares_per_item(text))

    def test_persona_declares_per_item_needs_decision_3_sentence(self):
        text = "BLESS <id>\nREJECT <id>\n"
        self.assertFalse(persona_declares_per_item(text))

    def test_persona_declares_per_item_when_all_three_present(self):
        text = f"BLESS <id>\nREJECT <id>\n{DECISION_3}\n"
        self.assertTrue(persona_declares_per_item(text))

    def test_batch_persona_with_single_gate_is_flagged(self):
        text = f"BLESS <id>\nREJECT <id>\n{DECISION_3}\n<gate>: BLESS\n"
        self.assertTrue(persona_mixes_protocols(text))

    def test_cycle_line_renders_decision_9_shape(self):
        line = cycle_line(2, 3, (2, 1), (3, 0), (1, 2))
        self.assertEqual(
            line,
            "- cycle 2: set 3 · PO 2/1 · QA 3/0 · CTO 1/2",
        )

    def test_cycle_line_parser_round_trips(self):
        args = {
            "n": 4,
            "size": 7,
            "po": (5, 2),
            "qa": (6, 1),
            "cto": (4, 3),
        }
        self.assertEqual(parse_cycle_line(cycle_line(**args)), args)


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

    def test_avril_batch_nodes_are_exactly_po_qa_cto(self):
        self.assertEqual(batch_nodes(self.avril), {"po", "qa", "cto"})

    def test_no_code_gan_or_axel_node_is_batch(self):
        self.assertEqual(batch_nodes(self.code), set())
        self.assertEqual(batch_nodes(self.axel), set())

    def test_each_batch_persona_declares_per_item(self):
        for text in (self.po, self.qa, self.cto):
            self.assertTrue(persona_declares_per_item(text))
            self.assertFalse(persona_mixes_protocols(text))

    def test_avril_card_set_review_law(self):
        self.assertIn(DECISION_3, self.avril_card)
        self.assertRegex(self.avril_card, r"(?i)one verdict line per id")
        self.assertIn("audit-packet verdict", self.avril_card)
        self.assertIn("siblings", self.avril_card)
        self.assertNotRegex(
            self.avril_card,
            r"(?i)max(imum)? (batch|set) size|at most \d+ (items|PBIs)",
        )

    def test_blessed_backlog_summary_has_cycle_template(self):
        self.assertIn("- cycle <n>: set <size>", self.summary)

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
