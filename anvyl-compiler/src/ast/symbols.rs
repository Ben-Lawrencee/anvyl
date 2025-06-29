use std::collections::HashMap;

use crate::ast::ASTVisitor;
use crate::prelude::*;

pub struct SymbolChecker {
    symbols: HashMap<String, ()>,
    diagnostics: DiagnosticsBagCell,
}

impl SymbolChecker {
    pub fn new(diagnostics: DiagnosticsBagCell) -> Self {
        Self {
            symbols: HashMap::new(),
            diagnostics,
        }
    }
}

impl ASTVisitor for SymbolChecker {
    fn visit_variable_expression(&mut self, expression: &ASTVariableExpression) {
        if self
            .symbols
            .get(&expression.identifier.span.clone_text())
            .is_none()
        {
            let mut diagnostics_binding = self.diagnostics.borrow_mut();
            diagnostics_binding.report_undeclared_variable(
                expression.identifier.span.clone_text(),
                expression.identifier.span.clone(),
            );
        }
    }

    fn visit_let_statement(&mut self, let_statement: &ASTLetStatement) {
        self.symbols
            .insert(let_statement.identifier.span.clone_text(), ());
        self.visit_expression(&let_statement.initializer);
    }
}
