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


class ConsumerBooks(unittest.TestCase):
    def test_missing_key_is_none(self):
        text = 'skills = "v1-one-law"\nloops = "v1-cards"\n'
        self.assertIsNone(vsr.parse_lockfile_books(text))

    def test_empty_array_is_empty_list(self):
        text = 'skills = "v1-one-law"\nbooks = []\n'
        self.assertEqual(vsr.parse_lockfile_books(text), [])

    def test_empty_array_with_comment(self):
        text = 'books = []     # disclosed language books\n'
        self.assertEqual(vsr.parse_lockfile_books(text), [])

    def test_named_books(self):
        text = 'books = ["rust", "ocaml"]\n'
        self.assertEqual(vsr.parse_lockfile_books(text), ["rust", "ocaml"])

    def test_loops_self_pin_has_no_books_key(self):
        text = (ROOT / "lockfile.toml").read_text()
        self.assertIsNone(vsr.parse_lockfile_books(text))


if __name__ == "__main__":
    unittest.main()
