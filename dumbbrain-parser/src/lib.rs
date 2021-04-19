use std::iter::Peekable;

use ast::ExpressionSyntax;
use dumbbrain_lexer::syntax::SyntaxKind;
use dumbbrain_lexer::token::Token;
use dumbbrain_lexer::Lexer;

pub mod ast;

pub struct Parser<'s> {
    lexer: Peekable<Lexer<'s>>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
        }
    }

    pub fn parse(&mut self) -> ExpressionSyntax {
        self.parse_term()
    }

    fn parse_term(&mut self) -> ExpressionSyntax {
        let mut left = self.parse_factor();

        while self.check(&[SyntaxKind::PlusToken, SyntaxKind::MinusToken]) {
            let operator_token = self.bump().unwrap();
            let right = self.parse_factor();
            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_factor(&mut self) -> ExpressionSyntax {
        let mut left = self.parse_unary_expression();

        while self.check(&[SyntaxKind::SlashToken, SyntaxKind::StarToken]) {
            let operator_token = self.bump().unwrap();
            let right = self.parse_unary_expression();
            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_unary_expression(&mut self) -> ExpressionSyntax {
        if self.check(&[SyntaxKind::PlusToken, SyntaxKind::MinusToken]) {
            // unary!
            let operator_token = self.bump().unwrap();
            let right = self.parse_unary_expression();
            ExpressionSyntax::Unary {
                operator_token,
                right: Box::new(right),
            }
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax {
        if self.check(&[SyntaxKind::NumberToken]) {
            let literal_token = self.bump().unwrap();
            ExpressionSyntax::Literal { literal_token }
        } else if self.check(&[SyntaxKind::LeftParenthesisToken]) {
            let left_parenthesis_token = self.bump().unwrap();
            let expression = self.parse();
            let right_parenthesis_token = self.expect(SyntaxKind::RightParenthesisToken).unwrap();
            ExpressionSyntax::Parenthesized {
                left_parenthesis_token,
                expression: Box::new(expression),
                right_parenthesis_token,
            }
        } else {
            todo!()
        }
    }

    fn check(&mut self, kinds: &[SyntaxKind]) -> bool {
        self.eat_whitespace();
        self.lexer.peek().map_or(false, |t| kinds.contains(&t.kind))
    }

    fn expect(&mut self, kind: SyntaxKind) -> Option<Token> {
        let token = self.bump()?;

        (token.kind == kind).then(|| token)
    }

    fn bump(&mut self) -> Option<Token> {
        self.eat_whitespace();
        self.lexer.next()
    }

    fn eat_whitespace(&mut self) {
        while let Some(Token {
            kind: SyntaxKind::WhitespaceToken,
            ..
        }) = self.lexer.peek()
        {
            self.lexer.next();
        }
    }
}
