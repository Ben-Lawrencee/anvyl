use crate::{ast::syntax::SyntaxColors, prelude::*};

pub struct ASTPrinter {
    result: String,
}

impl ASTPrinter {
    pub fn new() -> Self {
        Self {
            result: String::new(),
        }
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }

    fn push(&mut self, text: impl Into<String>) {
        self.result.push_str(&text.into());
    }

    fn push_whitespace(&mut self) {
        self.result.push_str(" ");
    }

    fn push_newline(&mut self) {
        self.result.push('\n');
    }
}

impl ASTVisitor for ASTPrinter {
    fn visit_error_expression(&mut self, span: &TextSpan) {
        self.push(
            SyntaxColors::error()
                .apply_to(span.clone_text())
                .to_string(),
        );
    }

    fn visit_number_expression(&mut self, expression: &ASTNumberExpression) {
        self.push(
            SyntaxColors::number()
                .apply_to(expression.number())
                .to_string(),
        );
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.push(SyntaxColors::keyword().apply_to("let").to_string());
        self.push_whitespace();
        self.push(
            SyntaxColors::text()
                .apply_to(let_statement.identifier.span.clone_text())
                .to_string(),
        );
        self.push(SyntaxColors::text().apply_to(" = ").to_string());
        self.visit_expression(&let_statement.initializer);
        self.push(SyntaxColors::text().apply_to(";").to_string());
        self.push_newline();
    }

    fn visit_binary_expression(&mut self, bin_expr: &ASTBinaryExpression) {
        self.visit_expression(&bin_expr.left);
        self.push_whitespace();
        self.push(
            SyntaxColors::text()
                .apply_to(bin_expr.operator.token.span.clone_text())
                .to_string(),
        );
        self.push_whitespace();
        self.visit_expression(&bin_expr.right);
    }

    fn visit_parenthesized_expression(&mut self, paren_expr: &ASTParenthesizedExpression) {
        self.push(SyntaxColors::text().apply_to("(").to_string());
        self.visit_expression(&paren_expr.inner);
        self.push(SyntaxColors::text().apply_to(")").to_string());
    }

    fn visit_variable_expression(&mut self, var_expr: &ASTVariableExpression) {
        self.push(
            SyntaxColors::variable()
                .apply_to(var_expr.identifier())
                .to_string(),
        );
    }
}
