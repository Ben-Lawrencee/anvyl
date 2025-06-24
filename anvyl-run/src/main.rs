use std::rc::Rc;

use anvyl_compiler::{ast::evaluator::ASTEvaluator, prelude::*};

fn main() {
    let input = "let a = 10 + 11; let b = 20; let c = a * b;";

    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    println!("Tokens:");
    for token in &tokens {
        println!("  {:?}", token);
    }
    println!("");

    let diagnostics: DiagnosticsBagCell = DiagnosticsBag::new_ref_cell();
    let mut ast = AST::new();
    let mut parser = Parser::new(tokens, Rc::clone(&diagnostics));

    println!("Parsing statements...");
    while let Some(stmt) = parser.next_statement() {
        println!("  {:?}", stmt);
        ast.add_statement(stmt);
    }
    println!("");

    ast.visualize();

    let diagnostics_binding = diagnostics.borrow();
    if !diagnostics_binding.is_empty() {
        println!("");
        println!("Diagnostics found:");
        let source_text = SourceText::new(input);
        let printer = Diagnostic::printer(&source_text, &diagnostics_binding.diagnostics);

        for diagnostic in diagnostics_binding.diagnostics.iter() {
            println!("{}", printer.stringify_diagnostic(diagnostic));
        }
        return;
    }

    println!("No diagnostics found, proceeding to evaluation...");

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);

    println!("Result: {:?}", eval.last_value);
}
