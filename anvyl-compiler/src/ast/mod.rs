use crate::prelude::*;

pub mod expressions;
pub mod lexer;
pub mod parser;
pub mod printer;
pub mod syntax;

pub struct AST {
    pub statements: Vec<ASTStatement>,
}

impl AST {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&mut self, visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&mut self) {
        let mut printer = ASTPrinter::new();
        self.visit(&mut printer);
        println!("{}", printer.result());
    }
}

#[derive(Debug)]
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

#[derive(Debug)]
pub struct ASTStatement {
    kind: ASTStatementKind,
}

impl ASTStatement {
    pub fn new(kind: ASTStatementKind) -> Self {
        Self { kind }
    }

    pub fn expression(expr: ASTExpression) -> Self {
        ASTStatement::new(ASTStatementKind::Expression(expr))
    }

    pub fn kind(&self) -> &ASTStatementKind {
        &self.kind
    }
}

pub trait ASTVisitor {
    fn default_visit_statement(&mut self, statement: &ASTStatement) {
        match statement.kind() {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.default_visit_statement(statement);
    }

    fn default_visit_expression(&mut self, expression: &ASTExpression) {
        match expression.kind() {
            ASTExpressionKind::Error(span) => self.visit_error(span),
            ASTExpressionKind::Number(expr) => self.visit_number(expr),
            ASTExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ASTExpressionKind::Parenthesized(parenthesized) => {
                self.visit_parenthesized_expression(parenthesized)
            }
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.default_visit_expression(expression);
    }

    fn visit_error(&mut self, span: &TextSpan);

    fn visit_number(&mut self, expression: &ASTNumberExpression);

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        self.visit_expression(&expression.right);
    }

    fn visit_parenthesized_expression(&mut self, expression: &ASTParenthesizedExpression) {
        self.visit_expression(&expression.expression);
    }
}
