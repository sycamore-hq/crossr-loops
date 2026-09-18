#!/usr/bin/env python3
"""Every page under book/src is in SUMMARY.md, and every SUMMARY entry exists.

Before this book had a SUMMARY.md it had no index at all: twelve authored
pages and nothing rendering them, so eight of them — crossr-review and the
whole destinations chapter — were orphans no reader could reach and no tool
would notice. mdbook silently drops a page that is not in SUMMARY.md, so the
orphan state is the one that comes back by default when someone adds a page.

This checks both directions without needing mdbook on PATH, which CI and most
contributor machines do not have.

Calculations are pure. Reading the tree is the action.
"""

from __future__ import annotations

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BOOK_SRC = ROOT / "book" / "src"

# [Title](path/to/page.md) — mdbook links are always relative to src/.
LINK = re.compile(r"\[[^\]]*\]\(([^)#]+\.md)(?:#[^)]*)?\)")


def summary_targets(summary: str) -> set[str]:
    """Every page SUMMARY.md links to, as src-relative posix paths."""
    return {match.group(1).lstrip("./") for match in LINK.finditer(summary)}


def page_files(src: Path) -> set[str]:
    """Every authored page under src/, excluding SUMMARY.md itself."""
    return {
        path.relative_to(src).as_posix()
        for path in src.rglob("*.md")
        if path.name != "SUMMARY.md"
    }


class Calculations(unittest.TestCase):
    def test_reads_links_from_a_summary(self):
        text = "- [A](pipeline/a.md)\n  - [B](destinations/b.md)\n"
        self.assertEqual(summary_targets(text), {"pipeline/a.md", "destinations/b.md"})

    def test_ignores_part_headings_and_prose(self):
        self.assertEqual(summary_targets("# Pipeline\n\nnot a link\n"), set())

    def test_strips_a_leading_dot_slash(self):
        self.assertEqual(summary_targets("- [A](./pipeline/a.md)"), {"pipeline/a.md"})

    def test_ignores_an_anchor(self):
        self.assertEqual(summary_targets("- [A](pipeline/a.md#section)"), {"pipeline/a.md"})

    def test_does_not_match_a_non_markdown_link(self):
        self.assertEqual(summary_targets("- [A](https://example.com/x.html)"), set())


class LiveTree(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.summary = (BOOK_SRC / "SUMMARY.md").read_text()
        cls.linked = summary_targets(cls.summary)
        cls.pages = page_files(BOOK_SRC)

    def test_book_has_a_summary(self):
        self.assertTrue(self.linked, "SUMMARY.md links to no pages")

    def test_no_page_is_an_orphan(self):
        orphans = sorted(self.pages - self.linked)
        self.assertEqual(
            orphans, [], f"pages absent from SUMMARY.md, so mdbook drops them: {orphans}"
        )

    def test_no_summary_entry_is_dangling(self):
        missing = sorted(self.linked - self.pages)
        self.assertEqual(
            missing, [], f"SUMMARY.md links pages that do not exist: {missing}"
        )

    def test_book_toml_exists(self):
        self.assertTrue(
            (ROOT / "book" / "book.toml").is_file(),
            "book/book.toml is what makes this a book rather than loose markdown",
        )


if __name__ == "__main__":
    unittest.main()
