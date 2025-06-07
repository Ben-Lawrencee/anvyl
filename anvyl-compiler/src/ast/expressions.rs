use crate::prelude::{TextSpan, Token};

#[derive(Debug)]
pub enum ASTExpressionKind {
    Error(TextSpan),
    Number(ASTNumberExpression),
    Binary(ASTBinaryExpression),
    Parenthesized(ASTParenthesizedExpression),
}

#[derive(Debug)]
pub struct ASTExpression {
    kind: ASTExpressionKind,
}

impl ASTExpression {
    pub fn new(kind: ASTExpressionKind) -> Self {
        Self { kind }
    }

    pub fn number(value: i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression::new(value)))
    }

    pub fn binary(left: ASTExpression, operator: ASTBinaryOperator, right: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Binary(ASTBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: ASTExpression) -> Self {
        ASTExpression::new(ASTExpressionKind::Parenthesized(
            ASTParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }

    pub fn error(span: TextSpan) -> Self {
        ASTExpression::new(ASTExpressionKind::Error(span))
    }

    pub fn kind(&self) -> &ASTExpressionKind {
        &self.kind
    }
}

#[derive(Debug)]
pub struct ASTNumberExpression {
    value: i64,
}

impl ASTNumberExpression {
    pub fn new(value: i64) -> Self {
        Self { value }
    }

    pub fn number(&self) -> i64 {
        self.value
    }
}

#[derive(Debug)]
pub struct ASTBinaryExpression {
    pub(crate) left: Box<ASTExpression>,
    pub(crate) operator: ASTBinaryOperator,
    pub(crate) right: Box<ASTExpression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTBinaryOperatorKind {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub struct ASTBinaryOperator {
    pub(crate) kind: ASTBinaryOperatorKind,
    pub(crate) token: Token,
}

impl ASTBinaryOperator {
    pub fn new(kind: ASTBinaryOperatorKind, token: Token) -> Self {
        Self { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            ASTBinaryOperatorKind::Add | ASTBinaryOperatorKind::Subtract => 1,
            ASTBinaryOperatorKind::Multiply | ASTBinaryOperatorKind::Divide => 2,
        }
    }
}

#[derive(Debug)]
pub struct ASTParenthesizedExpression {
    pub(crate) expression: Box<ASTExpression>,
}
