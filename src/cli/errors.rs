use std::fmt;

/// Error severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity
{
    Warning,
    Error,
    FatalError
}

/// Cli Error Structure
#[derive(Debug, Clone)]
pub struct Error
{
    message: String,
    severity: Severity
}

impl Error
{
    /// Generate a new warning
    pub fn warning(msg: &str) -> Self
    {
        Self
        {
            message: String::from(msg),
            severity: Severity::Warning
        }
    }

    /// Generate a new error
    pub fn error(msg: &str) -> Self
    {
        Self
        {
            message: String::from(msg),
            severity: Severity::Error
        }
    }

    /// Generate a new fatal error
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
        // Render an error with the given colorings
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

/// Wrapper for an error recorder
pub struct ErrorRecorder
{
    recorded_errors: Vec<Error>
}

impl ErrorRecorder
{
    /// Generate a new error recorder
    pub fn new() -> Self
    {
        Self
        {
            recorded_errors: vec![]
        }
    }

    /// Report a new error
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

    /// Unwrap a return value while reporting an error
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

    /// Display any recorded errors
    pub fn dump(&self)
    {
        for error in &self.recorded_errors
        {
            eprintln!("{}", error);
        }
    }
}