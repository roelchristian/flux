// src/lexer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Keyword(String),
    Symbol(String),
    KeywordSym(String),   // :name
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Arrow,                // =>
    Quote,
    Eval,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment(); // Skip comments before processing tokens

        if self.position >= self.input.len() {
            return Some(Token::EOF);
        }

        let ch = self.peek_char()?;

        match ch {
            '(' => self.consume_single(Token::LParen),
            ')' => self.consume_single(Token::RParen),
            '[' => self.consume_single(Token::LBracket),
            ']' => self.consume_single(Token::RBracket),
            '{' => self.consume_single(Token::LBrace),
            '}' => self.consume_single(Token::RBrace),
            ':' => self.read_keyword_symbol(),
            '"' => self.read_string(),
            '=' if self.peek_next_char() == Some('>') => {
                self.position += 2;
                Some(Token::Arrow)
            },
            '+' | '-' | '*' | '/' | '<' | '>' | '=' => self.read_symbol(),
            ch if ch.is_ascii_digit() => self.read_number(),
            ch if ch.is_alphabetic() => self.read_word(),
            _ => {
                self.position += 1;
                None
            }
        }
    }

    pub fn skip_comment(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch == ';' {
                // Skip until the end of the line
                while let Some(ch) = self.peek_char() {
                    self.position += 1;
                    if ch == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }
    
    pub fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    pub fn peek_next_char(&self) -> Option<char> {
        self.input.get(self.position + 1).cloned()
    }

    fn consume_single(&mut self, tok: Token) -> Option<Token> {
        self.position += 1;
        Some(tok)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() || ch == '.' {
                self.position += 1;
            } else {
                break;
            }
        }
        let value: String = self.input[start..self.position].iter().collect();
        value.parse::<f64>().ok().map(Token::Number)
    }

    fn read_string(&mut self) -> Option<Token> {
        self.position += 1; // skip initial "
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch == '"' {
                let content: String = self.input[start..self.position].iter().collect();
                self.position += 1;
                return Some(Token::String(content));
            }
            self.position += 1;
        }
        None
    }

    fn read_symbol(&mut self) -> Option<Token> {
        let ch = self.peek_char()?;
        self.position += 1;
        Some(Token::Symbol(ch.to_string()))
    }

    fn read_keyword_symbol(&mut self) -> Option<Token> {
        self.position += 1;
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.position += 1;
            } else {
                break;
            }
        }
        let value: String = self.input[start..self.position].iter().collect();
        Some(Token::KeywordSym(value))
    }

    fn read_word(&mut self) -> Option<Token> {
        let start = self.position;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '-' || ch == '_' {
                self.position += 1;
            } else {
                break;
            }
        }
        let word: String = self.input[start..self.position].iter().collect();

        match word.as_str() {
            "true" => Some(Token::Boolean(true)),
            "false" => Some(Token::Boolean(false)),
            "def" | "let" | "if" | "match" | "macro" => Some(Token::Keyword(word)),
            "quote" => Some(Token::Quote),
            "eval" => Some(Token::Eval),
            _ => Some(Token::Ident(word)),
        }
    }
}
