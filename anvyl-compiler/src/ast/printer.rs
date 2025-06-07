use crate::{ast::syntax::SyntaxColors, prelude::*};

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

impl ASTPrinter {
    const LEVEL_INDENT: usize = 2;

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: String::new(),
        }
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }

    fn push(&mut self, text: impl Into<String>) {
        self.result.push_str(&text.into());
    }

    fn add_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn add_newline(&mut self) {
        self.result.push('\n');
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_error(&mut self, span: &TextSpan) {
        self.push(
            SyntaxColors::text()
                .apply_to(span.literal.clone())
                .to_string(),
        );
    }

    fn visit_number(&mut self, expression: &ASTNumberExpression) {
        self.push(
            SyntaxColors::number()
                .apply_to(expression.number())
                .to_string(),
        );
    }

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        self.add_whitespace();
        self.push(
            SyntaxColors::text()
                .apply_to(expression.operator.token.span.literal.clone())
                .to_string(),
        );
        self.add_whitespace();
        self.visit_expression(&expression.right);
    }

    fn visit_parenthesized_expression(&mut self, expression: &ASTParenthesizedExpression) {
        self.push(SyntaxColors::text().apply_to("(").to_string());
        self.visit_expression(&expression.expression);
        self.push(SyntaxColors::text().apply_to(")").to_string());
    }
}
