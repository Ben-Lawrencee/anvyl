use crate::prelude::{TextSpan, Token};

pub mod evaluator;
pub mod lexer;
pub mod parser;

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

pub struct ASTPrinter {
    indent: usize,
}

impl ASTPrinter {
    const LEVEL_INDENT: usize = 2;

    pub fn new() -> Self {
        Self { indent: 0 }
    }

    fn print_with_indent(&mut self, text: &str) {
        println!("{}{}", " ".repeat(self.indent), text);
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_statement(&mut self, statement: &ASTStatement) {
        self.print_with_indent("Statement:");
        self.indent += Self::LEVEL_INDENT;
        ASTVisitor::default_visit_statement(self, statement);
        self.indent -= Self::LEVEL_INDENT;
    }

    fn visit_expression(&mut self, expression: &ASTExpression) {
        self.print_with_indent("Expression:");
        self.indent += Self::LEVEL_INDENT;
        ASTVisitor::default_visit_expression(self, expression);
        self.indent -= Self::LEVEL_INDENT;
    }

    fn visit_error(&mut self, span: &TextSpan) {
        self.print_with_indent(&format!("Error at {}:{}", span.start, span.end));
        self.print_with_indent(&format!("Literal: {}", span.literal));
    }

    fn visit_number(&mut self, expression: &ASTNumberExpression) {
        self.print_with_indent(&format!("Number: {}", expression.number()));
    }

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.print_with_indent("Binary Expression:");
        self.indent += Self::LEVEL_INDENT;
        self.print_with_indent(&format!("Operator: {:?}", expression.operator.kind));
        self.print_with_indent("Left:");
        self.indent += Self::LEVEL_INDENT;
        ASTVisitor::default_visit_expression(self, &expression.left);
        self.indent -= Self::LEVEL_INDENT;
        self.print_with_indent("Right:");
        self.indent += Self::LEVEL_INDENT;
        ASTVisitor::default_visit_expression(self, &expression.right);
        self.indent -= Self::LEVEL_INDENT;
        self.indent -= Self::LEVEL_INDENT;
    }

    fn visit_parenthesized_expression(&mut self, expression: &ASTParenthesizedExpression) {
        self.print_with_indent("Parenthesized Expression:");
        self.indent += Self::LEVEL_INDENT;
        ASTVisitor::default_visit_expression(self, &expression.expression);
        self.indent -= Self::LEVEL_INDENT;
    }
}

#[derive(Debug)]
pub enum ASTStatementKind {
    Expression(ASTExpression),
}

#[derive(Debug)]
pub struct ASTStatement {
    kind: ASTStatementKind,
    // span: TextSpan,
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
    left: Box<ASTExpression>,
    operator: ASTBinaryOperator,
    right: Box<ASTExpression>,
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
    expression: Box<ASTExpression>,
}
