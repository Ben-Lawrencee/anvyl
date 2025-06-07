use std::cmp;

use console::style;

use crate::prelude::*;

pub struct DiagnosticsPrinter<'a> {
    text: &'a SourceText,
    diagnostics: &'a [Diagnostic],
}

const PREFIX_LENGTH: usize = 8;

impl<'a> DiagnosticsPrinter<'a> {
    pub fn new(text: &'a SourceText, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    /// Stringifies the diagnostic .
    ///
    /// It uses the following format:
    ///
    /// <file>:<line>:<column>: <kind>: <message>
    /// let <red>x<reset> = 5;
    ///          ^
    ///          |
    ///          +-- This is the error message (<line>:<column>)
    ///
    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let span_length = diagnostic.span.length();

        let line_index = self.text.line_index(diagnostic.span.start);
        let line = self.text.get_line(line_index);
        let line_start = self.text.line_start(line_index);

        let column = diagnostic.span.start - line_start;

        let (prefix, span, suffix) = Self::get_line_window(span_length, line, column);

        let indent = cmp::min(PREFIX_LENGTH, column);
        let (arrow_pointers, arrow_line) = Self::format_arrow(span_length, indent);

        let error_message = Self::format_message(diagnostic, indent);

        let colored = Self::apply_diagnostic_color(&diagnostic.kind, span);

        format!(
            "{}{}{}\n{}\n{}\n{}",
            prefix, colored, suffix, arrow_pointers, arrow_line, error_message,
        )
    }

    pub fn print(&self) {
        for diagnostic in self.diagnostics {
            let output = self.stringify_diagnostic(diagnostic);
            println!("{}", output);
        }
    }

    fn get_line_window(span_length: usize, line: &str, column: usize) -> (&str, &str, &str) {
        let prefix_start = cmp::max(0, column as isize - PREFIX_LENGTH as isize) as usize;
        let prefix_end = column;

        let suffix_start = cmp::min(column + span_length + 1, line.len());
        let suffix_end = cmp::min(suffix_start + PREFIX_LENGTH, line.len());

        let prefix = &line[prefix_start..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];

        (prefix, span, suffix)
    }

    fn format_arrow(span_length: usize, indent: usize) -> (String, String) {
        let arrow_pointers = format!("{:indent$}{}", "", "^".repeat(span_length), indent = indent);
        let arrow_line = format!("{:indent$}|", "", indent = indent);
        (arrow_pointers, arrow_line)
    }

    fn apply_diagnostic_color(
        kind: &DiagnosticKind,
        message: impl Into<String>,
    ) -> console::StyledObject<String> {
        match kind {
            DiagnosticKind::Error => style(message.into()).red(),
            DiagnosticKind::Warning => style(message.into()).yellow(),
        }
    }

    fn format_message(diagnostic: &Diagnostic, indent: usize) -> String {
        format!("{:indent$}+-- {}", "", diagnostic.message, indent = indent)
    }
}
