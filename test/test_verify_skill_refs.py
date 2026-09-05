#!/usr/bin/env python3
"""Pure checks for verify-skill-refs calculations."""

from __future__ import annotations

import importlib.machinery
import importlib.util
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
_spec = importlib.util.spec_from_loader(
    "verify_skill_refs",
    importlib.machinery.SourceFileLoader(
        "verify_skill_refs", str(ROOT / "scripts" / "verify-skill-refs")
    ),
)
vsr = importlib.util.module_from_spec(_spec)
assert _spec.loader is not None
_spec.loader.exec_module(vsr)


BOOK_TRUE = """---
name: rust
description: language book
metadata:
  book: "true"
---

# Rust Book
"""

BOOK_YES = """---
name: ocaml
book: yes
---
"""

NOT_A_BOOK = """---
name: code-review
description: gate
---

# Gate
"""

METADATA_WITHOUT_BOOK = """---
name: github-pr-review
metadata:
  author: scull7
  short-description: review
---
"""

RULES_HEADING_ONLY = """---
name: architecture
---

# Architecture

## Rules

Do not infer book-ness from this heading.
"""


class BookMarker(unittest.TestCase):
    def test_metadata_book_true_is_a_book(self):
        fields = vsr.parse_frontmatter(BOOK_TRUE)
        self.assertTrue(vsr.is_book_frontmatter(fields))

    def test_top_level_book_yes_is_a_book(self):
        fields = vsr.parse_frontmatter(BOOK_YES)
        self.assertTrue(vsr.is_book_frontmatter(fields))

    def test_gate_card_is_not_a_book(self):
        fields = vsr.parse_frontmatter(NOT_A_BOOK)
        self.assertFalse(vsr.is_book_frontmatter(fields))

    def test_other_metadata_is_not_a_book(self):
        fields = vsr.parse_frontmatter(METADATA_WITHOUT_BOOK)
        self.assertFalse(vsr.is_book_frontmatter(fields))

    def test_rules_heading_does_not_make_a_book(self):
        fields = vsr.parse_frontmatter(RULES_HEADING_ONLY)
        self.assertFalse(vsr.is_book_frontmatter(fields))


if __name__ == "__main__":
    unittest.main()
