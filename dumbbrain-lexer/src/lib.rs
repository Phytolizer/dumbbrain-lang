use std::iter::Peekable;
use std::str::CharIndices;

use dumbbrain::object::DumbBrainObject;
use span::Span;
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
