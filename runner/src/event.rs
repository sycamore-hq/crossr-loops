//! Data: the events a walk feeds to the stepper.
//!
//! The vocabulary is the graphs' own (decision 3): the reserved `next` fires
//! an unlabeled edge, `BLESS` / `REJECT` are the only events an adversary
//! accepts, and any other token is a plain edge label (`pass`, `human`, ...).
//! The runner never decides a verdict — every event comes from the caller.

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::graph::Label;

/// An adversary's verdict. The only events an `adversary` node accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Verdict {
    /// `BLESS`
    Bless,
    /// `REJECT`
    Reject,
}

impl Verdict {
    /// Both verdicts, in the order error messages list them.
    pub const ALL: [Verdict; 2] = [Verdict::Bless, Verdict::Reject];

    /// The verdict as the graphs spell it.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Verdict::Bless => "BLESS",
            Verdict::Reject => "REJECT",
        }
    }

    /// The verdict `token` spells, if it spells one (case-sensitive).
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Verdict::ALL.into_iter().find(|v| v.as_str() == token)
    }

    /// `BLESS, REJECT` — the list an adversary error names.
    #[must_use]
    pub fn accepted() -> String {
        Verdict::ALL
            .iter()
            .map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One line of a walk.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Event {
    /// The reserved [`Label::NEXT`]: fires an unlabeled edge.
    Next,
    /// `BLESS` / `REJECT`: fires the matching edge of an adversary.
    Verdict(Verdict),
    /// Any other token: fires the edge whose `when` equals it.
    Label(Label),
}

impl Event {
    /// The event as a walk-file token — the same text [`FromStr`] reads.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Event::Next => Label::NEXT,
            Event::Verdict(v) => v.as_str(),
            Event::Label(l) => l.as_str(),
        }
    }

    /// `true` when this event fires an edge with the given `when`
    /// (`None` = unlabeled).
    #[must_use]
    pub fn matches(&self, when: Option<&Label>) -> bool {
        match (self, when) {
            (Event::Next, None) => true,
            (Event::Next, Some(_)) | (Event::Verdict(_) | Event::Label(_), None) => false,
            (Event::Verdict(v), Some(l)) => l.as_str() == v.as_str(),
            (Event::Label(want), Some(l)) => l == want,
        }
    }

    /// `true` for `BLESS` / `REJECT`.
    #[must_use]
    pub fn is_verdict(&self) -> bool {
        matches!(self, Event::Verdict(_))
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Why a token is not an event. One event per line: the token is trimmed
/// and may not be empty or contain whitespace.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ParseEventError {
    /// Nothing but whitespace.
    #[error("empty event")]
    Empty,
    /// Two tokens on one line.
    #[error("event '{text}' contains whitespace; one event per line")]
    Whitespace {
        /// The offending text, trimmed.
        text: String,
    },
}

impl FromStr for Event {
    type Err = ParseEventError;

    /// `next` → [`Event::Next`]; `BLESS` / `REJECT` → [`Event::Verdict`];
    /// anything else → [`Event::Label`]. Case-sensitive, as the graphs are.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = s.trim();
        if token.is_empty() {
            return Err(ParseEventError::Empty);
        }
        if token.chars().any(char::is_whitespace) {
            return Err(ParseEventError::Whitespace {
                text: token.to_owned(),
            });
        }
        if token == Label::NEXT {
            return Ok(Event::Next);
        }
        Ok(Verdict::from_token(token)
            .map_or_else(|| Event::Label(Label::new(token)), Event::Verdict))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grammar_reads_next_verdicts_and_labels() {
        assert_eq!("next".parse(), Ok(Event::Next));
        assert_eq!("BLESS".parse(), Ok(Event::Verdict(Verdict::Bless)));
        assert_eq!("REJECT".parse(), Ok(Event::Verdict(Verdict::Reject)));
        assert_eq!("pass".parse(), Ok(Event::Label(Label::new("pass"))));
        assert_eq!(
            "zero-survivors".parse(),
            Ok(Event::Label(Label::new("zero-survivors")))
        );
    }

    #[test]
    fn grammar_is_case_sensitive() {
        assert_eq!("bless".parse(), Ok(Event::Label(Label::new("bless"))));
        assert_eq!("Next".parse(), Ok(Event::Label(Label::new("Next"))));
    }

    #[test]
    fn grammar_trims_and_refuses_empty_or_split_tokens() {
        assert_eq!("  BLESS\t".parse(), Ok(Event::Verdict(Verdict::Bless)));
        assert_eq!("".parse::<Event>(), Err(ParseEventError::Empty));
        assert_eq!("   ".parse::<Event>(), Err(ParseEventError::Empty));
        assert_eq!(
            "BLESS BLESS".parse::<Event>(),
            Err(ParseEventError::Whitespace {
                text: "BLESS BLESS".to_owned()
            })
        );
    }

    #[test]
    fn display_round_trips_the_token() {
        for token in ["next", "BLESS", "REJECT", "pass", "human"] {
            let event: Event = token.parse().expect("token parses");
            assert_eq!(event.to_string(), token);
        }
    }

    #[test]
    fn matches_follows_decision_3() {
        let bless = Label::new("BLESS");
        let pass = Label::new("pass");
        assert!(Event::Next.matches(None));
        assert!(!Event::Next.matches(Some(&pass)));
        assert!(Event::Verdict(Verdict::Bless).matches(Some(&bless)));
        assert!(!Event::Verdict(Verdict::Reject).matches(Some(&bless)));
        assert!(!Event::Verdict(Verdict::Bless).matches(None));
        assert!(Event::Label(pass.clone()).matches(Some(&pass)));
        assert!(!Event::Label(pass.clone()).matches(Some(&bless)));
        assert!(!Event::Label(pass).matches(None));
    }

    #[test]
    fn accepted_lists_both_verdicts() {
        assert_eq!(Verdict::accepted(), "BLESS, REJECT");
        assert!(Event::Verdict(Verdict::Bless).is_verdict());
        assert!(!Event::Next.is_verdict());
        assert!(!Event::Label(Label::new("BLESS-ish")).is_verdict());
    }
}
