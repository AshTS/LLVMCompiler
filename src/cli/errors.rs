use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity
{
    Warning,
    Error,
    FatalError
}

#[derive(Debug, Clone)]
pub struct Error
{
    message: String,
    severity: Severity
}

impl Error
{
    pub fn warning(msg: &str) -> Self
    {
        Self
        {
            message: String::from(msg),
            severity: Severity::Warning
        }
    }

    pub fn error(msg: &str) -> Self
    {
        Self
        {
            message: String::from(msg),
            severity: Severity::Error
        }
    }

    pub fn fatal_error(msg: &str) -> Self
    {
        Self
        {
            message: String::from(msg),
            severity: Severity::FatalError
        }
    }
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "compiler: ")?;
        match self.severity
        {
            Severity::Warning =>
            {
                write!(f, "\x1b[1m\x1b[33mwarning\x1b[0m")?;
            },
            Severity::Error =>
            {
                write!(f, "\x1b[1m\x1b[31merror\x1b[0m")?;
            },
            Severity::FatalError =>
            {
                write!(f, "\x1b[1m\x1b[31mfatal error\x1b[0m")?;
            }
        }

        write!(f, ": {}", self.message)
    }
}

pub struct ErrorRecorder
{
    recorded_errors: Vec<Error>
}

impl ErrorRecorder
{
    pub fn new() -> Self
    {
        Self
        {
            recorded_errors: vec![]
        }
    }

    pub fn report_error(&mut self, error: Error) -> Result<(), Error>
    {
        if error.severity == Severity::FatalError
        {
            Err(error)
        }
        else
        {
            eprintln!("{}", error);
            self.recorded_errors.push(error);
            Ok(())
        }
    }

    pub fn wrap_return<T>(&mut self, result: Result<T, Error>) -> Result<Option<T>, Error>
    {
        match result
        {
            Ok(v) => Ok(Some(v)),
            Err(error) => 
            {
                self.report_error(error)?;
                Ok(None)
            }
        }
    }

    pub fn dump(&self)
    {
        for error in &self.recorded_errors
        {
            eprintln!("{}", error);
        }
    }
}