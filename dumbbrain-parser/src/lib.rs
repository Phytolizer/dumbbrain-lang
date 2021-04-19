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
        let mut left = self.parse_primary_expression();

        while self.check(&[SyntaxKind::SlashToken, SyntaxKind::StarToken]) {
            let operator_token = self.bump().unwrap();
            let right = self.parse_primary_expression();
            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax {
        let literal_token = self.expect(SyntaxKind::NumberToken).unwrap();
        ExpressionSyntax::Literal { literal_token }
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
