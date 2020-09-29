use std::fmt;

static DEFAULT_FILE_NAME: &'static str = "[unknown]";

#[derive(Debug, Clone)]
pub struct FileLocation
{
    name: String,
    pub col: usize,
    pub row: usize
}

impl FileLocation
{
    pub fn new() -> Self
    {
        FileLocation
        {
            name: String::from(DEFAULT_FILE_NAME),
            col: 1,
            row: 1
        }
    }

    pub fn from_name(name: &str) -> Self
    {
        FileLocation
        {
            name: String::from(name),
            col: 1,
            row: 1
        }
    }

    pub fn consume_char(&mut self, value: char)
    {
        self.col += 1;
        match value
        {
            '\n' => {self.row += 1; self.col = 1;},
            _ => {}
        };
    }
}

impl fmt::Display for FileLocation
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Line {}:{} in file '{}'", self.row, self.col, self.name)
    }
}

pub struct Stream
{
    index: usize,
    data: String,
    location: FileLocation
}

impl Stream
{
    pub fn new(data: String, file_name: String) -> Self
    {
        Self
        {
            index: 0,
            data: data.clone(),
            location: FileLocation::from_name(&file_name)
        }
    }

    pub fn peek(&self) -> Option<(char, FileLocation)>
    {
        if self.index + 1 >= self.data.len()
        {
            None
        }
        else
        {
            let v = self.data.chars().nth(self.index + 1);
            if v.is_none()
            {
                None
            }
            else
            {
                Some((v.unwrap(), self.location.clone()))
            }
        }
    }

    pub fn current(&self) -> Option<(char, FileLocation)>
    {
        if self.index >= self.data.len()
        {
            None
        }
        else
        {
            let v = self.data.chars().nth(self.index);
            if v.is_none()
            {
                None
            }
            else
            {
                Some((v.unwrap(), self.location.clone()))
            }
        }
    }

    pub fn consume(&mut self) -> bool
    {
        self.index += 1;

        match self.current()
        {
            Some(value) =>
            {
                self.location.consume_char(value.0);
                true
            },
            None => false
        }
    }

    pub fn current_location(&self) -> FileLocation
    {
        self.location.clone()
    }

    pub fn check_next(&self, c: char) -> bool
    {
        match self.peek()
        {
            Some(v) => v.0 == c,
            None => false
        }
    }

    pub fn check_next_vec(&self, v: Vec<char>) -> bool
    {
        match self.peek()
        {
            Some(c) => v.contains(&c.0),
            None => false
        }
    }
}