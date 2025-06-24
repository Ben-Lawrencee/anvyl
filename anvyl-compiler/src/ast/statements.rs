use crate::prelude::*;

#[derive(Debug)]
pub enum ASTStatementKind {
    Expression(ASTExpression),
    LetStatement(ASTLetStatement),
}

#[derive(Debug)]
pub struct ASTStatement {
    pub(crate) kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }

    pub fn let_statement(identifier: Token, initializer: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::LetStatement(ASTLetStatement::new(
            identifier,
            initializer,
        )))
    }

    pub fn kind(&self) -> &ASTStatementKind {
        &self.kind
    }
}

#[derive(Debug)]
pub struct ASTLetStatement {
    pub(crate) identifier: Token,
    pub(crate) initializer: ASTExpression,
}

impl ASTLetStatement {
    pub fn new(identifier: Token, initializer: ASTExpression) -> Self {
        Self {
            identifier,
            initializer,
        }
    }
}
