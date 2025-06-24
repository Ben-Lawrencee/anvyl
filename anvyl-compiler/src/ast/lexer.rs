use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Represents a failure in syntax.
    Bad,

    // Punctuation
    Colon,
    Semicolon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftChevron,
    RightChevron,
    Amperstand,
    Percent,
    Comma,
    Period,
    Hashtag,
    Exclamation,
    Question,
    Tilde,
    Pipe,
    Backslash,
    SingleQuote,
    DoubleQuote,
    Equals,

    // Arithmetic Operators
    Plus,
    Minus,
    Slash,
    Asterisk,

    // Keywords
    Let,
    If,
    Else,

    // Literals
    Number(i64),
    Identifier,

    // Hidden tokens
    /// Spaces, tabs.
    Whitespace,
    /// Represents a newline character.
    Newline,
    /// End of file token.
    EOF,
}

impl TokenKind {
    pub fn to_string(&self) -> String {
        let str = match self {
            // Error token
            TokenKind::Bad => "Bad",

            // Punctuation
            TokenKind::Colon => ":",
            TokenKind::Semicolon => ";",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftChevron => "<",
            TokenKind::RightChevron => ">",
            TokenKind::Amperstand => "&",
            TokenKind::Percent => "%",
            TokenKind::Comma => ",",
            TokenKind::Period => ".",
            TokenKind::Hashtag => "#",
            TokenKind::Exclamation => "!",
            TokenKind::Question => "?",
            TokenKind::Tilde => "~",
            TokenKind::Pipe => "|",
            TokenKind::Backslash => "\\",
            TokenKind::SingleQuote => "'",
            TokenKind::DoubleQuote => "\"",
            TokenKind::Equals => "=",

            // Arithmetic Operators
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Asterisk => "*",
            TokenKind::Slash => "/",

            // Keywords
            TokenKind::Let => "let",
            TokenKind::If => "if",
            TokenKind::Else => "else",

            // Literals
            TokenKind::Number(n) => &n.to_string(),
            TokenKind::Identifier => "Identifier",

            // Hidden tokens
            TokenKind::Whitespace => "Whitespace",
            TokenKind::Newline => "Newline",
            TokenKind::EOF => "EOF",
        };

        str.to_string()
    }

    pub fn is_bad(&self) -> bool {
        matches!(self, TokenKind::Bad)
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self, TokenKind::Let | TokenKind::If | TokenKind::Else)
    }

    pub fn is_punctuation(&self) -> bool {
        matches!(
            self,
            TokenKind::Colon
                | TokenKind::Semicolon
                | TokenKind::LeftBrace
                | TokenKind::RightBrace
                | TokenKind::LeftParen
                | TokenKind::RightParen
                | TokenKind::LeftChevron
                | TokenKind::RightChevron
                | TokenKind::Amperstand
                | TokenKind::Percent
                | TokenKind::Comma
                | TokenKind::Period
                | TokenKind::Hashtag
                | TokenKind::Exclamation
                | TokenKind::Question
                | TokenKind::Tilde
                | TokenKind::Pipe
                | TokenKind::Backslash
                | TokenKind::SingleQuote
                | TokenKind::DoubleQuote
                | TokenKind::Equals
        )
    }

    pub fn is_arithmetic_operator(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Slash | TokenKind::Asterisk
        )
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(
            self,
            TokenKind::Whitespace | TokenKind::Newline | TokenKind::EOF
        )
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<char> for TokenKind {
    fn from(c: char) -> Self {
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '|' => TokenKind::Pipe,
            '"' => TokenKind::DoubleQuote,
            '&' => TokenKind::Amperstand,
            '%' => TokenKind::Percent,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Period,
            '#' => TokenKind::Hashtag,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semicolon,
            '!' => TokenKind::Exclamation,
            '?' => TokenKind::Question,
            '~' => TokenKind::Tilde,
            '=' => TokenKind::Equals,
            '>' => TokenKind::RightChevron,
            '<' => TokenKind::LeftChevron,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '\\' => TokenKind::Backslash,
            '\'' => TokenKind::SingleQuote,
            '\n' => TokenKind::Newline,
            _ => TokenKind::Bad,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextSpan {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) literal: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        Self {
            start,
            end,
            literal,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn get_text(&self) -> &str {
        &self.literal
    }

    pub fn clone_text(&self) -> String {
        self.literal.clone()
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos >= self.input.len() {
            return None;
        }

        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::EOF,
                TextSpan::new(0, 0, '\0'.to_string()),
            ));
        }

        let start = self.current_pos;

        let kind = self.read_token();

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Some(Token::new(kind, span))
    }

    fn read_token(&mut self) -> TokenKind {
        let c = self.peek_unchecked();

        if Lexer::is_digit(c) {
            let number = self.consume_number();
            return TokenKind::Number(number);
        }

        if c == '\n' {
            self.consume();
            return TokenKind::Newline;
        }

        if Lexer::is_whitespace(c) {
            self.consume_whitespace();
            return TokenKind::Whitespace;
        }

        if Lexer::is_identifier(c) {
            let identifier = self.consume_identifier();
            return match identifier.as_str() {
                "let" => TokenKind::Let,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                _ => TokenKind::Identifier,
            };
        }

        self.consume_punctuation()
    }

    fn peek(&self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }

        Some(self.peek_unchecked())
    }

    fn peek_unchecked(&self) -> char {
        self.input.chars().nth(self.current_pos).unwrap()
    }

    fn is_whitespace(c: char) -> bool {
        c != '\n' && c.is_whitespace()
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn is_identifier(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(Lexer::is_whitespace)
    }

    fn consume_identifier(&mut self) -> String {
        self.consume_while(Lexer::is_identifier)
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();

        TokenKind::from(c)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }

        let c = self.peek_unchecked();
        self.current_pos += 1;

        Some(c)
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;

        while let Some(c) = self.peek() {
            if c.is_digit(10) {
                number = number * 10 + (c.to_digit(10).unwrap() as i64);
                self.current_pos += 1;
                continue;
            }

            break;
        }

        number
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.current_pos;
        while let Some(c) = self.peek() {
            if test(c) {
                self.current_pos += 1;
                continue;
            }

            break;
        }
        self.input[start..self.current_pos].to_string()
    }
}
