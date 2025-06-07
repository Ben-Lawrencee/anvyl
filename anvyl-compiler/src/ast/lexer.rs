use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Represents a failure in syntax.
    Bad,

    // Literals
    Asterisk,
    Colon,
    SemiColon,
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
    Plus,
    Minus,
    Slash,
    Equals,

    Number(i64),

    // Hidden tokens
    /// Spaces, tabs, and newlines.
    Whitespace,
    /// End of file token.
    EOF,
}

impl TokenKind {
    fn to_string(&self) -> String {
        match self {
            TokenKind::Bad => "Bad".to_string(),
            TokenKind::Asterisk => "*".to_string(),
            TokenKind::Colon => ":".to_string(),
            TokenKind::SemiColon => ";".to_string(),
            TokenKind::LeftBrace => "{".to_string(),
            TokenKind::RightBrace => "}".to_string(),
            TokenKind::LeftParen => "(".to_string(),
            TokenKind::RightParen => ")".to_string(),
            TokenKind::LeftChevron => "<".to_string(),
            TokenKind::RightChevron => ">".to_string(),
            TokenKind::Amperstand => "&".to_string(),
            TokenKind::Percent => "%".to_string(),
            TokenKind::Comma => ",".to_string(),
            TokenKind::Period => ".".to_string(),
            TokenKind::Hashtag => "#".to_string(),
            TokenKind::Exclamation => "!".to_string(),
            TokenKind::Question => "?".to_string(),
            TokenKind::Tilde => "~".to_string(),
            TokenKind::Pipe => "|".to_string(),
            TokenKind::Backslash => "\\".to_string(),
            TokenKind::SingleQuote => "'".to_string(),
            TokenKind::DoubleQuote => "\"".to_string(),
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Equals => "=".to_string(),
            TokenKind::Number(n) => n.to_string(),
            TokenKind::Whitespace => "Whitespace".to_string(),
            TokenKind::EOF => "EOF".to_string(),
        }
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
            ';' => TokenKind::SemiColon,
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

        if Lexer::is_whitespace(c) {
            self.consume_whitespace();
            return TokenKind::Whitespace;
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
        c.is_whitespace()
    }

    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(Lexer::is_whitespace)
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
