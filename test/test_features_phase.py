#!/usr/bin/env python3
"""A phase with every child commit completed must not stay in_progress.

gan-close-4b (work#6): gan-layer-separation was left open after 4a + the
lockfile pin. Dashboard then printed an active phase with 0 in-progress
commits.
"""

from __future__ import annotations

import json
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def all_commits_completed(phase: dict) -> bool:
    commits = phase.get("commits") or []
    return bool(commits) and all(c.get("status") == "completed" for c in commits)


def phase_left_open(phase: dict) -> bool:
    return all_commits_completed(phase) and phase.get("status") == "in_progress"


class Calculations(unittest.TestCase):
    def test_empty_phase_is_not_left_open(self):
        self.assertFalse(phase_left_open({"status": "in_progress", "commits": []}))

    def test_all_done_in_progress_is_left_open(self):
        phase = {
            "status": "in_progress",
            "commits": [{"id": "pr4a", "status": "completed"}],
        }
        self.assertTrue(phase_left_open(phase))

    def test_all_done_completed_is_closed(self):
        phase = {
            "status": "completed",
            "commits": [{"id": "pr4a", "status": "completed"}],
        }
        self.assertFalse(phase_left_open(phase))

    def test_one_open_child_keeps_the_phase_honestly_open(self):
        phase = {
            "status": "in_progress",
            "commits": [
                {"id": "a", "status": "completed"},
                {"id": "b", "status": "in_progress"},
            ],
        }
        self.assertFalse(phase_left_open(phase))


class LiveTree(unittest.TestCase):
    def test_no_phase_is_left_open(self):
        features = json.loads((ROOT / "features.json").read_text())
        for name, phase in features.items():
            with self.subTest(phase=name):
                self.assertFalse(
                    phase_left_open(phase),
                    f"{name} is in_progress with every child commit completed",
                )


if __name__ == "__main__":
    unittest.main()
