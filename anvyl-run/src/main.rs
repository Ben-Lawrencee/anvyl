use std::rc::Rc;

use anvyl_compiler::{
    ast::{evaluator::ASTEvaluator, symbols::SymbolChecker},
    prelude::*,
};

fn main() -> Result<(), ()> {
    let input = "let a = 10 + 11; let b = 20; let c = (a * b) + e;";
    let text = SourceText::new(input);

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

    check_diagnostics(&text, Rc::clone(&diagnostics))?;

    let mut symbol_checker = SymbolChecker::new(Rc::clone(&diagnostics));
    ast.visit(&mut symbol_checker);

    check_diagnostics(&text, Rc::clone(&diagnostics))?;

    let mut eval = ASTEvaluator::new();
    ast.visit(&mut eval);

    println!("Result: {:?}", eval.last_value);
    Ok(())
}

fn check_diagnostics(text: &SourceText, diagnostics_bag: DiagnosticsBagCell) -> Result<(), ()> {
    let diagnostics_binding = diagnostics_bag.borrow();
    if !diagnostics_binding.is_empty() {
        println!("");
        println!("Diagnostics found:");
        let printer = Diagnostic::printer(&text, &diagnostics_binding.diagnostics);

        printer.print();
        return Err(());
    }

    Ok(())
}
