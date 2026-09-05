#!/usr/bin/env python3
"""Pure checks for verify-skill-refs calculations."""

from __future__ import annotations

import importlib.machinery
import importlib.util
import os
import tempfile
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

    def test_multiline_empty_is_empty_list(self):
        text = "books = [\n]\n"
        self.assertEqual(vsr.parse_lockfile_books(text), [])

    def test_multiline_named(self):
        text = 'books = [\n  "rust",\n]\n'
        self.assertEqual(vsr.parse_lockfile_books(text), ["rust"])

    def test_string_value_is_raw_string(self):
        self.assertEqual(vsr.parse_lockfile_books('books = "rust"\n'), "rust")

    def test_loops_self_pin_has_no_books_key(self):
        text = (ROOT / "lockfile.toml").read_text()
        self.assertIsNone(vsr.parse_lockfile_books(text))


def _gate(text: str) -> vsr.Check:
    """Run verify_consumer_books against one lockfile. Action: tempfile + env."""
    graphs = {"code-gan": {"requires": {"book": True}}}
    check = vsr.Check()
    with tempfile.NamedTemporaryFile("w", suffix=".toml", delete=False) as handle:
        handle.write(text)
        path = handle.name
    previous = os.environ.get(vsr.CONSUMER_LOCKFILE_ENV)
    os.environ[vsr.CONSUMER_LOCKFILE_ENV] = path
    try:
        vsr.verify_consumer_books(graphs, check)
    finally:
        if previous is None:
            os.environ.pop(vsr.CONSUMER_LOCKFILE_ENV, None)
        else:
            os.environ[vsr.CONSUMER_LOCKFILE_ENV] = previous
        Path(path).unlink()
    return check


class ConsumerBooksGate(unittest.TestCase):
    def test_multiline_empty_fails_requires_book(self):
        check = _gate("books = [\n]\n")
        self.assertTrue(check.bad)
        self.assertTrue(any("books = []" in msg for msg in check.bad))

    def test_multiline_named_is_reported(self):
        check = _gate('books = [\n  "rust",\n]\n')
        self.assertFalse(check.bad)
        self.assertTrue(any("['rust']" in msg for msg in check.ok))

    def test_string_value_fails(self):
        check = _gate('books = "rust"\n')
        self.assertTrue(check.bad)
        self.assertTrue(any("array of strings" in msg for msg in check.bad))


PERSONA_WITH_TICKS = """# reviewer-agent

## Required Skills (must be active)

- `code-review`
- `gan-verdict`

## Personality
"""

PERSONA_WITH_PROSE = """# brick-coder-agent

## Required Skills (must be active)

- `code-writer`
- `brick-coder`
- the disclosed book

## Contract
"""

PERSONA_NO_SECTION = """# architect-agent

## Personality
"""


class RequiredSkills(unittest.TestCase):
    def test_extracts_backtick_names(self):
        self.assertEqual(
            vsr.extract_required_skills(PERSONA_WITH_TICKS),
            ["code-review", "gan-verdict"],
        )

    def test_skips_prose_without_backticks(self):
        self.assertEqual(
            vsr.extract_required_skills(PERSONA_WITH_PROSE),
            ["code-writer", "brick-coder"],
        )

    def test_missing_section_is_empty(self):
        self.assertEqual(vsr.extract_required_skills(PERSONA_NO_SECTION), [])

    def test_does_not_read_verdict_format_line(self):
        text = PERSONA_WITH_TICKS + (
            "\n**Verdict format** (per `gan-verdict`): "
            "`code-review: BLESS | REJECT`\n"
        )
        self.assertEqual(
            vsr.extract_required_skills(text),
            ["code-review", "gan-verdict"],
        )


if __name__ == "__main__":
    unittest.main()
