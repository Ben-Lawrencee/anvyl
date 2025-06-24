use console::Style;

use crate::prelude::*;

pub struct SyntaxColors;

impl SyntaxColors {
    pub fn error() -> Style {
        Style::new().red()
    }

    pub fn warning() -> Style {
        Style::new().yellow()
    }

    pub fn number() -> Style {
        Style::new().cyan()
    }

    pub fn variable() -> Style {
        Style::new().green()
    }

    pub fn text() -> Style {
        Style::new().bright().white()
    }

    pub fn string() -> Style {
        Style::new().green()
    }

    pub fn keyword() -> Style {
        Style::new().magenta()
    }

    pub fn comment() -> Style {
        Style::new().green().italic()
    }

    pub fn get_token_color(token: &TokenKind) -> Style {
        if token.is_keyword() {
            return Self::keyword();
        }

        if token.is_punctuation() {
            return Self::text();
        }

        match token {
            TokenKind::Number(_) => Self::number(),
            _ => Self::text(),
        }
    }
}
