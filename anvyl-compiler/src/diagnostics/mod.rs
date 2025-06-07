use std::vec::IntoIter;

pub use crate::prelude::*;
use crate::{ast::syntax::SyntaxColors, prelude::printer::DiagnosticsPrinter};

pub mod printer;

pub enum DiagnosticKind {
    Error,
    Warning,
}

pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: TextSpan,
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, message: String, span: TextSpan) -> Self {
        Self {
            kind,
            message,
            span,
        }
    }

    pub fn printer<'a>(
        text: &'a SourceText,
        diagnostics: &'a [Diagnostic],
    ) -> DiagnosticsPrinter<'a> {
        DiagnosticsPrinter::new(text, diagnostics)
    }
}

pub struct DiagnosticsBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticsBag {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn report_error(&mut self, message: String, span: TextSpan) {
        let error = Diagnostic::new(DiagnosticKind::Error, message, span);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: TextSpan) {
        let warning = Diagnostic::new(DiagnosticKind::Warning, message, span);
        self.diagnostics.push(warning);
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, found: &Token) {
        let message = format!("Expected {}, found {}", expected, found.kind);
        self.report_error(message, found.span.clone());
    }

    pub fn report_expected_expression(&mut self, found: &Token) {
        let message = match found.kind.is_keyword() {
            true => format!(
                "Expected expression, found '{}' keyword",
                SyntaxColors::keyword().apply_to(found.kind.to_string())
            ),
            false => format!("Expected expression, found '{}'", found.kind),
        };

        self.report_error(message, found.span.clone());
    }
}

impl IntoIterator for DiagnosticsBag {
    type IntoIter = IntoIter<Diagnostic>;
    type Item = Diagnostic;

    fn into_iter(self) -> Self::IntoIter {
        self.diagnostics.into_iter()
    }
}
