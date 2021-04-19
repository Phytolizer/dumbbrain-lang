use std::iter::Peekable;
use std::str::CharIndices;

use dumbbrain::object::DumbBrainObject;
use syntax::SyntaxKind;
use token::Token;

pub mod syntax;
pub mod token;

pub struct Lexer<'s> {
    source: Peekable<CharIndices<'s>>,
}

impl<'s> Lexer<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            source: text.char_indices().peekable(),
        }
    }
}

impl<'s> Iterator for Lexer<'s> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next()? {
            (start, c) if c.is_ascii_digit() => {
                let mut lexeme = c.to_string();
                while let Some(&(_, c)) = self.source.peek() {
                    if !c.is_ascii_digit() {
                        break;
                    }
                    self.source.next();
                    lexeme.push(c);
                }

                let value = DumbBrainObject::Number(lexeme.parse().unwrap());
                Some(Token::new(SyntaxKind::NumberToken, start, lexeme, value))
            }
            (start, c) if c.is_whitespace() => {
                let mut lexeme = c.to_string();
                while let Some(&(_, c)) = self.source.peek() {
                    if !c.is_whitespace() {
                        break;
                    }
                    self.source.next();
                    lexeme.push(c);
                }

                Some(Token::new(SyntaxKind::WhitespaceToken, start, lexeme, None))
            }
            (pos, '+') => Some(Token::new(
                SyntaxKind::PlusToken,
                pos,
                String::from("+"),
                None,
            )),
            (pos, '-') => Some(Token::new(
                SyntaxKind::MinusToken,
                pos,
                String::from("-"),
                None,
            )),
            (pos, '*') => Some(Token::new(
                SyntaxKind::StarToken,
                pos,
                String::from("*"),
                None,
            )),
            (pos, '/') => Some(Token::new(
                SyntaxKind::SlashToken,
                pos,
                String::from("/"),
                None,
            )),
            (pos, '(') => Some(Token::new(
                SyntaxKind::LeftParenthesisToken,
                pos,
                String::from("("),
                None,
            )),
            (pos, ')') => Some(Token::new(
                SyntaxKind::RightParenthesisToken,
                pos,
                String::from(")"),
                None,
            )),
            (pos, c) => Some(Token::new(SyntaxKind::BadToken, pos, c.to_string(), None)),
        }
    }
}
