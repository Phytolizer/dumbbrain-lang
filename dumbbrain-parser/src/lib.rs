use std::iter::Peekable;

use ast::ExpressionSyntax;
use dumbbrain_lexer::syntax::SyntaxKind;
use dumbbrain_lexer::token::Token;
use dumbbrain_lexer::Lexer;

pub mod ast;

pub struct Parser<'s> {
    lexer: Peekable<Lexer<'s>>,
    expected_kinds: Vec<SyntaxKind>,
    errors: Vec<String>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            lexer: Lexer::new(source).peekable(),
            expected_kinds: vec![],
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> ExpressionSyntax {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, parent_precedence: usize) -> ExpressionSyntax {
        let unary_operator_precedence = self
            .peek()
            .map(|tok| tok.kind.unary_precedence())
            .unwrap_or(0);
        let mut left =
            if unary_operator_precedence != 0 && unary_operator_precedence >= parent_precedence {
                let operator_token = self.bump().unwrap();
                let operand = self.parse_expression(unary_operator_precedence);
                ExpressionSyntax::Unary {
                    operator_token,
                    right: Box::new(operand),
                }
            } else {
                self.parse_primary_expression()
            };

        loop {
            let precedence = self
                .peek()
                .map(|tok| tok.kind.binary_precedence())
                .unwrap_or(0);
            if precedence == 0 || precedence <= parent_precedence {
                break left;
            }
            let operator_token = self.bump().unwrap();
            let right = self.parse_expression(precedence);
            left = ExpressionSyntax::Binary {
                left: Box::new(left),
                operator_token,
                right: Box::new(right),
            };
        }
    }

    fn parse_primary_expression(&mut self) -> ExpressionSyntax {
        if self.check(&[
            SyntaxKind::NumberToken,
            SyntaxKind::TrueKeyword,
            SyntaxKind::FalseKeyword,
        ]) {
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
            panic!("unexpected: {:#?}", self.peek());
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.eat_whitespace();
        self.lexer.peek()
    }

    fn check(&mut self, kinds: &[SyntaxKind]) -> bool {
        self.eat_whitespace();
        for kind in kinds {
            self.expected_kinds.push(*kind);
        }
        self.lexer.peek().map_or(false, |t| kinds.contains(&t.kind))
    }

    fn expect(&mut self, kind: SyntaxKind) -> Option<Token> {
        let token = self.bump()?;

        (token.kind == kind).then(|| token).or_else(|| {
            self.error();
            None
        })
    }

    fn bump(&mut self) -> Option<Token> {
        self.eat_whitespace();
        self.expected_kinds.clear();
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

    fn error(&mut self) {
        let token = self.bump().unwrap();
        let mut message = format!(
            "at {}:{}: expected ",
            token.span.first_line, token.span.first_column
        );
        for (i, kind) in self.expected_kinds.iter().enumerate() {
            if i == 0 {
                message.push_str(&format!("{:?}", kind));
            } else if i == self.expected_kinds.len() - 1 {
                message.push_str(&format!(" or {:?}", kind));
            } else {
                message.push_str(&format!(", {:?}", kind));
            }
        }

        self.errors.push(message);
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use expect_test::Expect;
    use ptree::TreeBuilder;

    use crate::ast::traits::SyntaxNode;
    use crate::ast::ExpressionSyntax;

    use super::Parser;

    fn format_node(node: &dyn SyntaxNode) -> String {
        let mut s = format!("{:?}", node.kind());
        if let Some(value) = node.value() {
            s.push_str(&format!(" {}", value));
        }
        s
    }

    fn build_tree(builder: &mut TreeBuilder, node: &dyn SyntaxNode) {
        let children = node.children();
        if children.is_empty() {
            builder.add_empty_child(format_node(node));
        } else {
            builder.begin_child(format_node(node));
            for child in children {
                build_tree(builder, child);
            }
            builder.end_child();
        }
    }

    fn format_tree(expression: &ExpressionSyntax) -> String {
        let mut builder = TreeBuilder::new("ParseTree".into());
        build_tree(&mut builder, expression);
        let mut output = Vec::<u8>::new();
        ptree::write_tree(&builder.build(), &mut output).unwrap();
        String::from_utf8(output).unwrap()
    }

    fn check(input: &str, expected: Expect) {
        let expression = Parser::new(input).parse();
        let pretty_tree = format_tree(&expression);
        expected.assert_eq(&pretty_tree);
    }

    #[test]
    fn parse_number() {
        check(
            "3",
            expect![[r#"
            ParseTree
            └─ LiteralExpression
               └─ NumberToken 3
        "#]],
        )
    }

    #[test]
    fn parse_addition() {
        check(
            "1+2",
            expect![[r#"
            ParseTree
            └─ BinaryExpression
               ├─ LiteralExpression
               │  └─ NumberToken 1
               ├─ PlusToken
               └─ LiteralExpression
                  └─ NumberToken 2
        "#]],
        )
    }

    #[test]
    fn parse_number_with_leading_whitespace() {
        check(
            "  1",
            expect![[r#"
            ParseTree
            └─ LiteralExpression
               └─ NumberToken 1
        "#]],
        )
    }

    #[test]
    fn parse_number_with_trailing_whitespace() {
        check(
            "123   ",
            expect![[r#"
            ParseTree
            └─ LiteralExpression
               └─ NumberToken 123
        "#]],
        )
    }

    #[test]
    fn parse_number_with_leading_and_trailing_whitespace() {
        check(
            "  4 ",
            expect![[r#"
            ParseTree
            └─ LiteralExpression
               └─ NumberToken 4
        "#]],
        )
    }

    #[test]
    fn plus_is_left_associative() {
        check(
            "1 + 2 + 3",
            expect![[r#"
            ParseTree
            └─ BinaryExpression
               ├─ BinaryExpression
               │  ├─ LiteralExpression
               │  │  └─ NumberToken 1
               │  ├─ PlusToken
               │  └─ LiteralExpression
               │     └─ NumberToken 2
               ├─ PlusToken
               └─ LiteralExpression
                  └─ NumberToken 3
        "#]],
        )
    }

    #[test]
    fn minus_is_left_associative() {
        check(
            "1 - 2 - 3",
            expect![[r#"
            ParseTree
            └─ BinaryExpression
               ├─ BinaryExpression
               │  ├─ LiteralExpression
               │  │  └─ NumberToken 1
               │  ├─ MinusToken
               │  └─ LiteralExpression
               │     └─ NumberToken 2
               ├─ MinusToken
               └─ LiteralExpression
                  └─ NumberToken 3
        "#]],
        )
    }

    #[test]
    fn operator_precedence_is_respected() {
        check(
            "1 + 2 * 3",
            expect![[r#"
            ParseTree
            └─ BinaryExpression
               ├─ LiteralExpression
               │  └─ NumberToken 1
               ├─ PlusToken
               └─ BinaryExpression
                  ├─ LiteralExpression
                  │  └─ NumberToken 2
                  ├─ StarToken
                  └─ LiteralExpression
                     └─ NumberToken 3
        "#]],
        )
    }

    #[test]
    fn parse_parentheses() {
        check(
            "(((((10)))))",
            expect![[r#"
            ParseTree
            └─ ParenthesizedExpression
               ├─ LeftParenthesisToken
               ├─ ParenthesizedExpression
               │  ├─ LeftParenthesisToken
               │  ├─ ParenthesizedExpression
               │  │  ├─ LeftParenthesisToken
               │  │  ├─ ParenthesizedExpression
               │  │  │  ├─ LeftParenthesisToken
               │  │  │  ├─ ParenthesizedExpression
               │  │  │  │  ├─ LeftParenthesisToken
               │  │  │  │  ├─ LiteralExpression
               │  │  │  │  │  └─ NumberToken 10
               │  │  │  │  └─ RightParenthesisToken
               │  │  │  └─ RightParenthesisToken
               │  │  └─ RightParenthesisToken
               │  └─ RightParenthesisToken
               └─ RightParenthesisToken
        "#]],
        )
    }

    #[test]
    fn parentheses_override_precedence() {
        check(
            "(1 + 2) * 3",
            expect![[r#"
            ParseTree
            └─ BinaryExpression
               ├─ ParenthesizedExpression
               │  ├─ LeftParenthesisToken
               │  ├─ BinaryExpression
               │  │  ├─ LiteralExpression
               │  │  │  └─ NumberToken 1
               │  │  ├─ PlusToken
               │  │  └─ LiteralExpression
               │  │     └─ NumberToken 2
               │  └─ RightParenthesisToken
               ├─ StarToken
               └─ LiteralExpression
                  └─ NumberToken 3
        "#]],
        )
    }

    #[test]
    fn unary_binds_stronger() {
        check(
            "-1 * -2",
            expect![[r#"
                ParseTree
                └─ BinaryExpression
                   ├─ UnaryExpression
                   │  ├─ MinusToken
                   │  └─ LiteralExpression
                   │     └─ NumberToken 1
                   ├─ StarToken
                   └─ UnaryExpression
                      ├─ MinusToken
                      └─ LiteralExpression
                         └─ NumberToken 2
            "#]],
        )
    }
}
