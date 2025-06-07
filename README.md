# Anvyl (.avyl)

A rust powered MXL-like file format which is compiled into normal `.tsx` files.

## Lexer / Tokenizer

Break the source code (`.avyl`) into tokens for parsing into an AST.

## Parser

Takes a token stream from the `Tokenizer`, and constructs an AST (Abstract Syntax Tree).

## Core

Constructs contexts from the AST and file structure for the plugins / addons to use.
Applies changes to the AST from plugins / addons to be passed to the compiler.

## Compiler

Runs compile-time checks on the final AST and file structure.
Determines and compiles the AST + plugins / addons into `.tsx` files.
