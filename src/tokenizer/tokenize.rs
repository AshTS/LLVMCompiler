use super::{Token, FileLocation, Stream};

struct Tokenizer
{
    current_data: String,
    source: Stream,
    pos: FileLocation,
    tokens: Vec<Token>
}

impl Tokenizer
{
    fn new(source: Stream) -> Self
    {
        Self
        {
            current_data: String::new(),
            pos: source.current_location().clone(),
            source: source,
            tokens: vec![]
        }
    }

    fn push_current(&mut self)
    {
        if self.current_data.len() != 0
        {
            self.tokens.push(Token::new(self.pos.clone(), self.current_data.clone()));
            self.current_data = String::new();
        }

        self.pos = self.source.current_location();
        
    }

    fn push_char(&mut self, c: char)
    {
        self.current_data.push(c);
    }

    fn consume(&mut self)
    {
        self.source.consume();
        if self.current_data.len() == 0
        {
            self.pos = self.source.current_location();
        }
    }

    fn move_back(&mut self)
    {
        self.pos.col -= 1;
    }
}

pub fn tokenize(input: String, file_name: String) -> Vec<Token>
{
    let mut tokenizer = Tokenizer::new(Stream::new(input, file_name));

    let mut single_line_comment: bool = false;
    let mut multi_line_comment: bool = false;

    loop 
    {
        match tokenizer.source.current()
        {
            Some(current) =>
            {
                if single_line_comment
                {
                    if current.0 == '/' && tokenizer.source.check_next('/')
                    {
                        tokenizer.source.consume();
                        single_line_comment = false;
                    }
                    else if current.0 == '\n'
                    {
                        single_line_comment = false;
                    }
                }
                else if multi_line_comment
                {
                    if current.0 == '*' && tokenizer.source.check_next('/')
                    {
                        tokenizer.source.consume();
                        multi_line_comment = false;
                    }
                }
                else
                {
                    match current.0
                    {
                        ' ' | '\n' => {tokenizer.push_current();},
                        '{' | '}' | '(' | ')' | '[' | ']' | ';' | ',' | ':' | '.' | '?' | '~' => 
                            {
                                tokenizer.push_current();
                                tokenizer.push_char(current.0);
                                tokenizer.move_back();
                                tokenizer.push_current();
                            },
                        '+' | '-' | '&' | '|' | '<' | '>' | '=' =>
                        {
                            tokenizer.push_current();

                            // Repeated
                            if tokenizer.source.check_next_vec(vec![current.0, '=']) || 
                                    current.0 == '-' && tokenizer.source.check_next('>')
                            {
                                tokenizer.push_char(current.0);
                                tokenizer.move_back();
                                tokenizer.source.consume();
                                tokenizer.push_char(tokenizer.source.current().unwrap().0);
                                if vec!['<', '>'].contains(&current.0) && tokenizer.source.check_next('=')
                                {
                                    tokenizer.source.consume();
                                    tokenizer.push_char(tokenizer.source.current().unwrap().0);
                                }
                                tokenizer.push_current();
                            }
                            else
                            {
                                tokenizer.move_back();
                                tokenizer.push_char(current.0);
                                tokenizer.push_current();
                            }
                        },
                        '*' | '%' | '^' | '!' =>
                        {
                            tokenizer.push_current();

                            if tokenizer.source.check_next('=')
                            {
                                tokenizer.push_char(current.0);
                                tokenizer.move_back();
                                tokenizer.source.consume();
                                tokenizer.push_char('=');
                                tokenizer.push_current();
                            }
                            else
                            {
                                tokenizer.move_back();
                                tokenizer.push_char(current.0);
                                tokenizer.push_current();
                            }
                        }
                        '/' =>
                        {
                            tokenizer.push_current();

                            if tokenizer.source.check_next('*')
                            {
                                tokenizer.consume();
                                multi_line_comment = true;
                            }
                            else if tokenizer.source.check_next('/')
                            {
                                tokenizer.consume();
                                single_line_comment = true;
                            }
                            else
                            {
                                if tokenizer.source.check_next('=')
                                {
                                    tokenizer.push_char(current.0);
                                    tokenizer.move_back();
                                    tokenizer.source.consume();
                                    tokenizer.push_char('=');
                                    tokenizer.push_current();
                                }
                                else
                                {
                                    tokenizer.move_back();
                                    tokenizer.push_char(current.0);
                                    tokenizer.push_current();
                                }
                            }
                        }
                        default => {tokenizer.push_char(default);}
                    }
                }

                tokenizer.source.consume();
            },
            None => {break;}
        }
    }

    tokenizer.push_current();

    tokenizer.tokens
}