use console::Style;

use crate::prelude::*;

pub struct ASTPrinter {
    indent: usize,
    result: String,
}

impl ASTPrinter {
    const LEVEL_INDENT: usize = 2;
    const NUMBER_COLOR: Style = Style::new().cyan();
    const TEXT_COLOR: Style = Style::new().white();

    pub fn new() -> Self {
        Self {
            indent: 0,
            result: String::new(),
        }
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }

    fn push_colored(&mut self, text: impl Into<String>, color: Style) {
        self.result
            .push_str(&format!("{}", color.apply_to(text.into())));
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
        self.push_colored(span.literal.clone(), Self::TEXT_COLOR);
    }

    fn visit_number(&mut self, expression: &ASTNumberExpression) {
        self.push_colored(expression.number().to_string(), Self::NUMBER_COLOR);
    }

    fn visit_binary_expression(&mut self, expression: &ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        self.add_whitespace();
        self.push_colored(
            expression.operator.token.span.literal.clone(),
            Self::TEXT_COLOR,
        );
        self.add_whitespace();
        self.visit_expression(&expression.right);
    }

    fn visit_parenthesized_expression(&mut self, expression: &ASTParenthesizedExpression) {
        self.push_colored("(", Self::TEXT_COLOR);
        self.visit_expression(&expression.expression);
        self.push_colored(")", Self::TEXT_COLOR);
    }
}
