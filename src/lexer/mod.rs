use super::ParserError;

#[derive(Debug, Clone)]
pub struct TextSpan {
    start: usize,
    end: usize,
    literal: String,
}
impl TextSpan {
    pub fn new(start: usize, end: usize, literal: String) -> Self {
        TextSpan {
            start,
            end,
            literal,
        }
    }
    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    HttpGet,
    HttpPost,
    HttpPut,
    HttpPatch,
    HttpDelete,

    Http401,
    Http403,
    Http200,
    Http201,
    Http202,
    Http429,
    Http500,

    LeftParen,
    RightParen,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftSquareBrace,
    RightSquareBrace,
    Colon,
    Comma,
    Quotes,

    TempStopObjectToken,

    Bad,
    Eof,
    LiteralStringsKeyOrValue,
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    //once we get path...

    // we should extract all ENDPOINTS -> each represented by it's HTTPMETHOD... get, post, put, patch, delete...

    //first - extract the get and it's items -> URL, Responses (200, 400, 500, etc) -> We are just interested in whether they are
    //defined or not...
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos == self.input.len() {
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0, 0, "\0".to_string()),
            ));
        }

        let c = self.current_char();
        return c.map(|c| {
            let start = self.current_pos;
            let mut kind = TokenKind::Bad;

            // if Self::is_whitespace(&c){
            //     //skip it...
            //     self.consume_whitespace();
            // }else if c == ':'{
            //     kind = TokenKind::Colon
            // }else if c == ','{
            //     kind = TokenKind::Comma
            // }

            match &c {
                ':' => kind = TokenKind::Colon,
                ',' => kind = TokenKind::Comma,
                '(' => kind = TokenKind::LeftParen,
                ')' => kind = TokenKind::RightParen,
                '{' => kind = TokenKind::LeftCurlyBrace,
                '}' => kind = TokenKind::RightCurlyBrace,
                '[' => kind = TokenKind::LeftSquareBrace,
                ']' => kind = TokenKind::RightSquareBrace,
                '"' => {
                    //make a literal... meaning "key": "value" or "key": {}
                    //consume string...
                    self.consume_literal_strings_key_or_value();
                    kind = TokenKind::LiteralStringsKeyOrValue;
                }
                _ => {
                    if Self::is_whitespace(&c) {
                        self.consume_whitespace()
                    }else if c.is_numeric(){
                        self.consume_numbers().unwrap_or_else(|e| {
                            panic!("{}", e)
                        });
                    }else{
                        unimplemented!()
                    }
                }
            }

            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start, end, literal);

            Token::new(kind, span)
        });
    }

    fn current_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }
    fn next_char(&mut self) -> Option<char> {
        //self.current_pos +=1;
        self.input.chars().nth(self.current_pos + 1)
    }
    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;
        c
    }
    fn consume_whitespace(&mut self) {
        self.current_pos += 1;
    }

    pub fn extract_open_api_path() -> Option<()> {
        //OUTPUTS... -> An Object that has -> endpointUrl, method, REsponses -> 401, 400, 200, 429 each must be defined for it...

        Some(())
    }
    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn consume_literal_strings_key_or_value(&mut self) {
        self.current_pos += 1;
        while let Some(c) = self.input.chars().nth(self.current_pos) {
            if c == '"' {
                break;
            }
            self.current_pos += 1;
        }
    }

    fn consume_numbers(&mut self) -> Result<(), ParserError> {
        self.current_pos += 1;
        let mut dots: u8 = 0;
        while let Some(c) = self.input.chars().nth(self.current_pos){
            if c == '.'{
                dots += 1;
            }
            if !c.is_numeric() && c != '.'{
                break;
            }
            self.current_pos += 1; //next_char method will be created to make this easier...
        }
        if dots > 1{
            return Err(ParserError::NumberFormatError("dots in a decimal number cannot be greater than 1".into()))
        }

        Ok(())
    }
}
