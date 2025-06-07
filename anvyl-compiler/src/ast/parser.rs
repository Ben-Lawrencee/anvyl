use std::cell::Cell;

use crate::prelude::*;

pub struct Counter {
    value: Cell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }

    pub fn increment(&self) -> usize {
        let current = self.value.get();
        self.value.set(current + 1);
        current
    }

    pub fn get_value(&self) -> usize {
        self.value.get()
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: Counter,
    diagnostics_bag: DiagnosticsBagCell,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, diagnostics_bag: DiagnosticsBagCell) -> Self {
        Self {
            tokens: tokens
                .iter()
                .filter(|t| t.kind != TokenKind::Whitespace)
                .cloned()
                .collect(),
            current: Counter::new(),
            diagnostics_bag,
        }
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current.get_value() as isize + offset) as usize;

        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }

        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_and_expect(&self, expected: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != expected {
            self.diagnostics_bag
                .borrow_mut()
                .report_unexpected_token(&expected, token);
        }
        token
    }

    fn is_at_end(&self) -> bool {
        self.current.get_value() >= self.tokens.len()
    }

    pub fn next_statement(&mut self) -> Option<ASTStatement> {
        if self.is_at_end() {
            return None;
        }

        Some(self.parse_statement())
    }
}

impl Parser {
    fn parse_statement(&mut self) -> ASTStatement {
        let expr = self.parse_expression();
        ASTStatement::expression(expr)
    }

    fn parse_expression(&mut self) -> ASTExpression {
        self.parse_binary_expression(0)
    }

    fn parse_primary_expression(&mut self) -> ASTExpression {
        let token = self.consume();

        match token.kind {
            TokenKind::Number(value) => ASTExpression::number(value),
            TokenKind::LeftParen => {
                let expr = self.parse_expression();
                self.consume_and_expect(TokenKind::RightParen);

                ASTExpression::parenthesized(expr)
            }
            _ => {
                self.diagnostics_bag
                    .borrow_mut()
                    .report_expected_expression(&token);

                ASTExpression::error(token.span.clone())
            }
        }
    }

    fn parse_binary_operator(&mut self) -> Option<ASTBinaryOperator> {
        let token = self.current();

        let kind = match token.kind {
            TokenKind::Plus => ASTBinaryOperatorKind::Add,
            TokenKind::Minus => ASTBinaryOperatorKind::Subtract,
            TokenKind::Asterisk => ASTBinaryOperatorKind::Multiply,
            TokenKind::Slash => ASTBinaryOperatorKind::Divide,
            _ => return None,
        };

        Some(ASTBinaryOperator::new(kind, token.clone()))
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> ASTExpression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            self.consume();
            let op_precedence = operator.precedence();
            if op_precedence < precedence {
                break;
            }

            let right = self.parse_binary_expression(op_precedence);
            left = ASTExpression::binary(left, operator, right);
        }

        left
    }
}
