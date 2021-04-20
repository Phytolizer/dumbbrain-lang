use std::iter::Peekable;
use std::str::CharIndices;

use dumbbrain::object::DumbBrainObject;
use span::Span;
use syntax::check_keyword;
use syntax::SyntaxKind;
use token::Token;

pub mod span;
pub mod syntax;
pub mod token;

pub struct Lexer<'s> {
    source: Peekable<CharIndices<'s>>,
    line_number: usize,
    column_offset: usize,
}

impl<'s> Lexer<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            source: text.char_indices().peekable(),
            line_number: 1,
            column_offset: 1,
        }
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        let (i, c) = self.source.next()?;
        match c {
            '\n' => {
                self.line_number += 1;
                self.column_offset = 1;
            }
            _ => {
                self.column_offset += 1;
            }
        }
        Some((i, c))
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let first_line = self.line_number;
        let first_column = self.column_offset;
        match self.advance()? {
            (start, c) if c.is_ascii_digit() => {
                let mut lexeme = c.to_string();
                while let Some(&(_, c)) = self.source.peek() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    self.advance();
                    lexeme.push(c);
                }

                let last_line = self.line_number;
                let last_column = self.column_offset;
                let value = DumbBrainObject::Number(lexeme.parse().unwrap());
                Some(Token::new(
                    SyntaxKind::NumberToken,
                    start,
                    lexeme,
                    value,
                    Span {
                        first_line,
                        first_column,
                        last_line,
                        last_column,
                    },
                ))
            }
            (start, c) if c.is_alphabetic() => {
                let mut lexeme = c.to_string();
                while let Some(&(_, c)) = self.source.peek() {
                    if !c.is_alphanumeric() {
                        break;
                    }
                    self.advance();
                    lexeme.push(c);
                }
                let last_line = self.line_number;
                let last_column = self.column_offset;
                let kind = check_keyword(&lexeme);
                let value = match kind {
                    SyntaxKind::TrueKeyword | SyntaxKind::FalseKeyword => {
                        Some(DumbBrainObject::Boolean(kind == SyntaxKind::TrueKeyword))
                    }
                    _ => None,
                };
                Some(Token::new(
                    kind,
                    start,
                    lexeme,
                    value,
                    Span {
                        first_line,
                        first_column,
                        last_line,
                        last_column,
                    },
                ))
            }
            (start, c) if c.is_whitespace() => {
                let mut lexeme = c.to_string();
                while let Some(&(_, c)) = self.source.peek() {
                    if !c.is_whitespace() {
                        break;
                    }
                    self.advance();
                    lexeme.push(c);
                }

                let last_line = self.line_number;
                let last_column = self.column_offset;
                Some(Token::new(
                    SyntaxKind::WhitespaceToken,
                    start,
                    lexeme,
                    None,
                    Span {
                        first_line,
                        first_column,
                        last_line,
                        last_column,
                    },
                ))
            }
            (pos, '+') => Some(Token::new(
                SyntaxKind::PlusToken,
                pos,
                String::from("+"),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, '-') => Some(Token::new(
                SyntaxKind::MinusToken,
                pos,
                String::from("-"),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, '*') => Some(Token::new(
                SyntaxKind::StarToken,
                pos,
                String::from("*"),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, '/') => Some(Token::new(
                SyntaxKind::SlashToken,
                pos,
                String::from("/"),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, '(') => Some(Token::new(
                SyntaxKind::LeftParenthesisToken,
                pos,
                String::from("("),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, ')') => Some(Token::new(
                SyntaxKind::RightParenthesisToken,
                pos,
                String::from(")"),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
            (pos, '=') if matches!(self.source.peek(), Some((_, '='))) => {
                self.advance();
                Some(Token::new(
                    SyntaxKind::EqualsEqualsToken,
                    pos,
                    "==".into(),
                    None,
                    Span {
                        first_line,
                        first_column,
                        last_line: self.line_number,
                        last_column: self.column_offset,
                    },
                ))
            }
            (pos, '!') if matches!(self.source.peek(), Some((_, '='))) => {
                self.advance();
                Some(Token::new(
                    SyntaxKind::BangEqualsToken,
                    pos,
                    "!=".into(),
                    None,
                    Span {
                        first_line,
                        first_column,
                        last_line: self.line_number,
                        last_column: self.column_offset,
                    },
                ))
            }
            (pos, '<') => {
                let (kind, literal) = if let Some((_, '=')) = self.source.peek() {
                    self.advance();
                    (SyntaxKind::LessEqualsToken, "<=")
                } else {
                    (SyntaxKind::LessToken, "<")
                };
                Some(Token::new(
                    kind,
                    pos,
                    literal.into(),
                    None,
                    Span {
                        first_line,
                        first_column,
                        last_line: self.line_number,
                        last_column: self.column_offset,
                    },
                ))
            }
            (pos, '>') => {
                let (kind, literal) = if let Some((_, '=')) = self.source.peek() {
                    self.advance();
                    (SyntaxKind::GreaterEqualsToken, ">=")
                } else {
                    (SyntaxKind::GreaterToken, ">")
                };
                Some(Token::new(
                    kind,
                    pos,
                    literal.into(),
                    None,
                    Span {
                        first_line,
                        first_column,
                        last_line: self.line_number,
                        last_column: self.column_offset,
                    },
                ))
            }
            (pos, c) => Some(Token::new(
                SyntaxKind::BadToken,
                pos,
                c.to_string(),
                None,
                Span {
                    first_line,
                    first_column,
                    last_line: self.line_number,
                    last_column: self.column_offset,
                },
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use expect_test::Expect;

    use crate::Lexer;
    use itertools::join;

    fn check_single_token(input: &str, expected: Expect) {
        let mut l = Lexer::new(input);
        let tok = l.next().unwrap();
        assert!(l.next().is_none());
        expected.assert_eq(&format!("{:#?}", tok));
    }

    #[test]
    fn lex_equals_equals() {
        check_single_token(
            "==",
            expect![[r#"
                Token {
                    kind: EqualsEqualsToken,
                    position: 0,
                    text: "==",
                    value: None,
                    span: Span {
                        first_line: 1,
                        first_column: 1,
                        last_line: 1,
                        last_column: 3,
                    },
                }"#]],
        )
    }

    #[test]
    fn lex_bad_token() {
        check_single_token(
            "@",
            expect![[r#"
            Token {
                kind: BadToken,
                position: 0,
                text: "@",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_number() {
        check_single_token(
            "123",
            expect![[r#"
            Token {
                kind: NumberToken,
                position: 0,
                text: "123",
                value: Some(
                    Number(
                        123.0,
                    ),
                ),
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 4,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_whitespace() {
        check_single_token(
            "  \t ",
            expect![[r#"
            Token {
                kind: WhitespaceToken,
                position: 0,
                text: "  \t ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 5,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_plus() {
        check_single_token(
            "+",
            expect![[r#"
            Token {
                kind: PlusToken,
                position: 0,
                text: "+",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_minus() {
        check_single_token(
            "-",
            expect![[r#"
            Token {
                kind: MinusToken,
                position: 0,
                text: "-",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_star() {
        check_single_token(
            "*",
            expect![[r#"
            Token {
                kind: StarToken,
                position: 0,
                text: "*",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_slash() {
        check_single_token(
            "/",
            expect![[r#"
            Token {
                kind: SlashToken,
                position: 0,
                text: "/",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_left_parenthesis() {
        check_single_token(
            "(",
            expect![[r#"
            Token {
                kind: LeftParenthesisToken,
                position: 0,
                text: "(",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_right_parenthesis() {
        check_single_token(
            ")",
            expect![[r#"
            Token {
                kind: RightParenthesisToken,
                position: 0,
                text: ")",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 2,
                },
            }"#]],
        )
    }

    #[test]
    fn lex_lots_of_things() {
        check_tokens(
            "123 + 456 - @#$ *\n5+2+3/4();",
            expect![[r##"
            Token {
                kind: NumberToken,
                position: 0,
                text: "123",
                value: Some(
                    Number(
                        123.0,
                    ),
                ),
                span: Span {
                    first_line: 1,
                    first_column: 1,
                    last_line: 1,
                    last_column: 4,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 3,
                text: " ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 4,
                    last_line: 1,
                    last_column: 5,
                },
            }
            Token {
                kind: PlusToken,
                position: 4,
                text: "+",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 5,
                    last_line: 1,
                    last_column: 6,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 5,
                text: " ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 6,
                    last_line: 1,
                    last_column: 7,
                },
            }
            Token {
                kind: NumberToken,
                position: 6,
                text: "456",
                value: Some(
                    Number(
                        456.0,
                    ),
                ),
                span: Span {
                    first_line: 1,
                    first_column: 7,
                    last_line: 1,
                    last_column: 10,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 9,
                text: " ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 10,
                    last_line: 1,
                    last_column: 11,
                },
            }
            Token {
                kind: MinusToken,
                position: 10,
                text: "-",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 11,
                    last_line: 1,
                    last_column: 12,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 11,
                text: " ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 12,
                    last_line: 1,
                    last_column: 13,
                },
            }
            Token {
                kind: BadToken,
                position: 12,
                text: "@",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 13,
                    last_line: 1,
                    last_column: 14,
                },
            }
            Token {
                kind: BadToken,
                position: 13,
                text: "#",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 14,
                    last_line: 1,
                    last_column: 15,
                },
            }
            Token {
                kind: BadToken,
                position: 14,
                text: "$",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 15,
                    last_line: 1,
                    last_column: 16,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 15,
                text: " ",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 16,
                    last_line: 1,
                    last_column: 17,
                },
            }
            Token {
                kind: StarToken,
                position: 16,
                text: "*",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 17,
                    last_line: 1,
                    last_column: 18,
                },
            }
            Token {
                kind: WhitespaceToken,
                position: 17,
                text: "\n",
                value: None,
                span: Span {
                    first_line: 1,
                    first_column: 18,
                    last_line: 2,
                    last_column: 1,
                },
            }
            Token {
                kind: NumberToken,
                position: 18,
                text: "5",
                value: Some(
                    Number(
                        5.0,
                    ),
                ),
                span: Span {
                    first_line: 2,
                    first_column: 1,
                    last_line: 2,
                    last_column: 2,
                },
            }
            Token {
                kind: PlusToken,
                position: 19,
                text: "+",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 2,
                    last_line: 2,
                    last_column: 3,
                },
            }
            Token {
                kind: NumberToken,
                position: 20,
                text: "2",
                value: Some(
                    Number(
                        2.0,
                    ),
                ),
                span: Span {
                    first_line: 2,
                    first_column: 3,
                    last_line: 2,
                    last_column: 4,
                },
            }
            Token {
                kind: PlusToken,
                position: 21,
                text: "+",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 4,
                    last_line: 2,
                    last_column: 5,
                },
            }
            Token {
                kind: NumberToken,
                position: 22,
                text: "3",
                value: Some(
                    Number(
                        3.0,
                    ),
                ),
                span: Span {
                    first_line: 2,
                    first_column: 5,
                    last_line: 2,
                    last_column: 6,
                },
            }
            Token {
                kind: SlashToken,
                position: 23,
                text: "/",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 6,
                    last_line: 2,
                    last_column: 7,
                },
            }
            Token {
                kind: NumberToken,
                position: 24,
                text: "4",
                value: Some(
                    Number(
                        4.0,
                    ),
                ),
                span: Span {
                    first_line: 2,
                    first_column: 7,
                    last_line: 2,
                    last_column: 8,
                },
            }
            Token {
                kind: LeftParenthesisToken,
                position: 25,
                text: "(",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 8,
                    last_line: 2,
                    last_column: 9,
                },
            }
            Token {
                kind: RightParenthesisToken,
                position: 26,
                text: ")",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 9,
                    last_line: 2,
                    last_column: 10,
                },
            }
            Token {
                kind: BadToken,
                position: 27,
                text: ";",
                value: None,
                span: Span {
                    first_line: 2,
                    first_column: 10,
                    last_line: 2,
                    last_column: 11,
                },
            }"##]],
        )
    }

    fn check_tokens(input: &str, expected: Expect) {
        let l = Lexer::new(input);
        let mut str_repr = vec![];
        for tok in l {
            str_repr.push(format!("{:#?}", tok));
        }

        expected.assert_eq(&join(str_repr.into_iter(), "\n"));
    }
}
