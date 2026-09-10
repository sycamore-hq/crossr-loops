//! Calculations: the `.walk` file grammar (decision 6).
//!
//! `graphs/walks/<graph>.<name>.walk`: one event per line, `#` comment
//! lines, blank lines ignored. The file-stem prefix before the first `.`
//! names the graph. Reading the file is the caller's action; this module
//! parses text.

use std::ffi::OsStr;
use std::path::Path;

use thiserror::Error;

use crate::event::{Event, ParseEventError};

/// The walk-file extension.
pub const EXTENSION: &str = "walk";

/// A line that is not a comment, blank, or event.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("line {line}: {source}")]
pub struct WalkFileError {
    /// 1-based line number.
    pub line: usize,
    /// What was wrong with the token.
    #[source]
    pub source: ParseEventError,
}

/// Parse walk-file text into events, in order.
///
/// A line whose first non-blank character is `#` is a comment. Blank lines
/// are skipped. Every other line is one event token (see
/// [`Event::from_str`](std::str::FromStr)).
///
/// # Errors
///
/// [`WalkFileError`] naming the first line that is not an event.
pub fn parse(text: &str) -> Result<Vec<Event>, WalkFileError> {
    text.lines()
        .enumerate()
        .map(|(i, raw)| (i + 1, raw.trim()))
        .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
        .map(|(line, token)| {
            token
                .parse::<Event>()
                .map_err(|source| WalkFileError { line, source })
        })
        .collect()
}

/// The graph a walk file belongs to: the file-name prefix before the first
/// `.` (`avril.happy.walk` → `avril`). `None` when the path has no UTF-8
/// file name.
#[must_use]
pub fn graph_name(path: &Path) -> Option<&str> {
    path.file_name()
        .and_then(OsStr::to_str)
        .and_then(|name| name.split('.').next())
        .filter(|prefix| !prefix.is_empty())
}

/// `true` for `*.walk`.
#[must_use]
pub fn is_walk_file(path: &Path) -> bool {
    path.extension().and_then(OsStr::to_str) == Some(EXTENSION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Verdict;
    use crate::graph::Label;

    #[test]
    fn parse_skips_comments_and_blank_lines() {
        let text =
            "# avril — happy path\n\nnext\n  BLESS  \n\n# a comment in the middle\nREJECT\npass\n";
        assert_eq!(
            parse(text).expect("parses"),
            vec![
                Event::Next,
                Event::Verdict(Verdict::Bless),
                Event::Verdict(Verdict::Reject),
                Event::Label(Label::new("pass")),
            ]
        );
    }

    #[test]
    fn parse_of_only_comments_is_empty() {
        assert_eq!(parse("# nothing\n\n   # indented comment\n"), Ok(vec![]));
        assert_eq!(parse(""), Ok(vec![]));
    }

    #[test]
    fn parse_names_the_bad_line() {
        let err = parse("# story\nnext\nBLESS BLESS\n").expect_err("two tokens on one line");
        assert_eq!(err.line, 3);
        assert_eq!(
            err.source,
            ParseEventError::Whitespace {
                text: "BLESS BLESS".to_owned()
            }
        );
        assert_eq!(
            err.to_string(),
            "line 3: event 'BLESS BLESS' contains whitespace; one event per line"
        );
    }

    #[test]
    fn a_hash_is_a_comment_only_at_line_start() {
        let err = parse("BLESS # po\n").expect_err("inline hash is not a comment");
        assert_eq!(err.line, 1);
        assert!(
            matches!(err.source, ParseEventError::Whitespace { .. }),
            "{err}"
        );
    }

    #[test]
    fn graph_name_is_the_prefix_before_the_first_dot() {
        assert_eq!(
            graph_name(Path::new("graphs/walks/avril.happy.walk")),
            Some("avril")
        );
        assert_eq!(
            graph_name(Path::new("code-gan.reject.tester.walk")),
            Some("code-gan")
        );
        assert_eq!(graph_name(Path::new("brick.walk")), Some("brick"));
        assert_eq!(graph_name(Path::new(".walk")), None);
        assert_eq!(graph_name(Path::new("")), None);
    }

    #[test]
    fn is_walk_file_checks_the_extension() {
        assert!(is_walk_file(Path::new("graphs/walks/avril.happy.walk")));
        assert!(!is_walk_file(Path::new("graphs/avril.json")));
        assert!(!is_walk_file(Path::new("graphs/walks/README.md")));
    }
}
