use crate::prelude::*;

pub mod evaluator;
pub mod expressions;
pub mod lexer;
pub mod parser;
pub mod printer;
pub mod statements;
pub mod symbols;
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

pub trait ASTVisitor {
    fn default_visit_statement(&mut self, statement: &ASTStatement) {
        match statement.kind() {
            ASTStatementKind::Expression(expr) => self.visit_expression(expr),
            ASTStatementKind::LetStatement(let_stmt) => self.visit_let_statement(let_stmt),
        }
    }

    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.default_visit_statement(statement);
    }

    fn default_visit_expression(&mut self, expression: &ASTExpression) {
        match expression.kind() {
            ASTExpressionKind::Error(span) => self.visit_error_expression(span),
            ASTExpressionKind::Number(expr) => self.visit_number_expression(expr),
            ASTExpressionKind::Binary(expr) => self.visit_binary_expression(expr),
            ASTExpressionKind::Variable(expr) => self.visit_variable_expression(expr),
            ASTExpressionKind::Parenthesized(parenthesized) => {
                self.visit_parenthesized_expression(parenthesized)
            }
        }
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.default_visit_expression(expression);
    }

    fn visit_variable_expression(&mut self, _expression: &ASTVariableExpression) {}

    fn visit_let_statement(&mut self, _let_statement: &ASTLetStatement) {}

    fn visit_error_expression(&mut self, _span: &TextSpan) {}

    fn visit_number_expression(&mut self, _expression: &ASTNumberExpression) {}

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        self.visit_expression(&expression.right);
    }

    fn visit_parenthesized_expression(&mut self, expression: &ASTParenthesizedExpression) {
        self.visit_expression(&expression.inner);
    }
}
