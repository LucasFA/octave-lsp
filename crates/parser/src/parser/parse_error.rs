use std::fmt;
use syntax::SyntaxKind;
use text_size::TextRange;

#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(super) expected: Vec<SyntaxKind>,
    pub(super) found: Option<SyntaxKind>,
    pub(super) range: TextRange,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error at {}..{}: expected",
            u32::from(self.range.start()),
            u32::from(self.range.end()),
        )?;

        let num_expected = self.expected.len();
        let is_first = |idx| idx == 0;
        let is_last = |idx| idx == num_expected - 1;

        for (idx, expected_kind) in self.expected.iter().enumerate() {
            if is_first(idx) {
                write!(f, " {}", expected_kind)?;
            } else if is_last(idx) {
                write!(f, " or {}", expected_kind)?;
            } else {
                write!(f, ", {}", expected_kind)?;
            }
        }

        if let Some(found) = self.found {
            write!(f, ", but found {}", found)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range as StdRange;
    use expect_test::expect;

    fn check(
        expected: Vec<SyntaxKind>,
        found: Option<SyntaxKind>,
        range: StdRange<u32>,
        output: expect_test::Expect,
    ) {
        let error = ParseError {
            expected,
            found,
            range: {
                let start = range.start.into();
                let end = range.end.into();
                TextRange::new(start, end)
            },
        };

        output.assert_eq(format!("{}", error).as_str());
        // assert_eq!(format!("{}", error), output);
    }

    #[test]
    fn one_expected_did_find() {
        check(
            vec![SyntaxKind::Equals],
            Some(SyntaxKind::Identifier),
            10..20,
            expect!["error at 10..20: expected '=', but found identifier"],
        );
    }

    #[test]
    fn one_expected_did_not_find() {
        check(
            vec![SyntaxKind::RParen],
            None,
            5..6,
            expect!["error at 5..6: expected ')'"],
        );
    }

    #[test]
    fn multiple_expected_did_find() {
        check(
            vec![
                SyntaxKind::Number,
                SyntaxKind::Identifier,
                SyntaxKind::Minus,
                SyntaxKind::LParen,
            ],
            Some(SyntaxKind::Semicolon),
            100..105,
            expect![r#"error at 100..105: expected number, identifier, '-' or '(', but found ';'"#],
        );
    }

     #[test]
    fn two_expected_did_find() {
        check(
            vec![SyntaxKind::Plus, SyntaxKind::Minus],
            Some(SyntaxKind::Equals),
            0..1,
            expect!["error at 0..1: expected '+' or '-', but found '='"],
        );
    }
}
