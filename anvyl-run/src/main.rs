use std::{cell::RefCell, rc::Rc};

use anvyl_compiler::prelude::*;

fn main() {
    let input = "7 + 3 * (2 - 1)";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("Tokens: {:?}", tokens);

    let diagnostics: DiagnosticsBagCell = Rc::new(RefCell::new(DiagnosticsBag::new()));
    let mut ast = AST::new();
    let mut parser = Parser::new(tokens, Rc::clone(&diagnostics));

    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }

    println!("Added statements");
    ast.visualize();

    let diagnostics_binding = diagnostics.borrow();
    if !diagnostics_binding.is_empty() {
        println!("Diagnostics found:");
        let source_text = SourceText::new(input);
        let printer = Diagnostic::printer(&source_text, &diagnostics_binding.diagnostics);

        for diagnostic in diagnostics_binding.diagnostics.iter() {
            println!("{}", printer.stringify_diagnostic(diagnostic));
        }
        return;
    }

    println!("No diagnostics found, proceeding to evaluation...");
}
