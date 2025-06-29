use std::collections::HashMap;

use crate::ast::ASTVisitor;
use crate::prelude::*;

pub struct ASTEvaluator {
    pub last_value: Option<i64>,
    pub variables: HashMap<String, i64>,
}

impl ASTEvaluator {
    pub fn new() -> Self {
        Self {
            last_value: None,
            variables: HashMap::new(),
        }
    }
}

impl ASTVisitor for ASTEvaluator {
    fn visit_variable_expression(&mut self, expression: &crate::prelude::ASTVariableExpression) {
        self.last_value = Some(
            *self
                .variables
                .get(&expression.identifier.span.clone_text())
                .unwrap(),
        );
    }

    fn visit_let_statement(&mut self, let_statement: &crate::prelude::ASTLetStatement) {
        self.visit_expression(&let_statement.initializer);
        self.variables.insert(
            let_statement.identifier.span.clone_text(),
            self.last_value.unwrap(),
        );
    }

    fn visit_number_expression(&mut self, expression: &crate::prelude::ASTNumberExpression) {
        self.last_value = Some(expression.number());
    }

    fn visit_binary_expression(&mut self, expression: &crate::prelude::ASTBinaryExpression) {
        self.visit_expression(&expression.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&expression.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match expression.operator.kind {
            ASTBinaryOperatorKind::Add => left + right,
            ASTBinaryOperatorKind::Subtract => left - right,
            ASTBinaryOperatorKind::Multiply => left * right,
            ASTBinaryOperatorKind::Divide => left / right,
        });
    }

    fn visit_parenthesized_expression(
        &mut self,
        expression: &crate::prelude::ASTParenthesizedExpression,
    ) {
        self.visit_expression(&expression.inner);
    }
}
