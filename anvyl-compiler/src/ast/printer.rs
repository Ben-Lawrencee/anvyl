use crate::prelude::*;

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
