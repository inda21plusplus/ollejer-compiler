use crate::position::Position;

#[derive(Debug, Clone)]
pub enum ErrorType {
    DisallowedCharError(DisallowedCharError),
    SyntaxError(SyntaxError),
    RunTimeError(RunTimeError),
    DivisionByZeroError(DivisionByZeroError),
}

#[derive(Debug, Clone)]
struct Error {
    pos_begin: Option<Position>,
    pos_end: Option<Position>,
    error_name: String,
    error_message: String,
}

impl Error {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_name: String,
        error_message: String,
    ) -> Self {
        Self {
            pos_begin,
            pos_end,
            error_name,
            error_message,
        }
    }
}

impl Error {
    fn as_string(&self) -> String {
        let mut error = format!("{}: {}", self.error_name, self.error_message);
        let mut file_name = String::new();
        let line: i64;
        let col: i64;

        match self.pos_begin.clone() {
            Some(pos) => {
                file_name = pos.file_name();
                line = pos.line() + 1;
                col = pos.column();
            }

            None => {
                file_name = "Unknown File".to_string();
                line = -1;
                col = -1
            }
        };

        let error_origin = format!(", File {}, line {}, col {}", file_name, line, col);
        let error_origin = &error_origin[..];
        error.push_str(error_origin);
        error
    }
}

#[derive(Debug, Clone)]
pub struct DisallowedCharError {
    error: Error,
}

impl DisallowedCharError {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_message: String,
    ) -> Self {
        Self {
            error: Error::new(
                pos_begin,
                pos_end,
                "Illegal Character".to_string(),
                error_message,
            ),
        }
    }
    pub fn as_string(&self) -> String {
        self.error.as_string()
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxError {
    error: Error,
}

impl SyntaxError {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_message: String,
    ) -> Self {
        Self {
            error: Error::new(
                pos_begin,
                pos_end,
                "Syntax Error".to_string(),
                error_message,
            ),
        }
    }
    pub fn as_string(&self) -> String {
        self.error.as_string()
    }
}

#[derive(Debug, Clone)]
pub struct RunTimeError {
    error: Error,
}

impl RunTimeError {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_message: String,
    ) -> Self {
        Self {
            error: Error::new(
                pos_begin,
                pos_end,
                "Runtime Error".to_string(),
                error_message,
            ),
        }
    }
    pub fn as_string(&self) -> String {
        self.error.as_string()
    }
}

#[derive(Debug, Clone)]
pub struct DivisionByZeroError {
    error: Error,
}

impl DivisionByZeroError {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_message: String,
    ) -> Self {
        Self {
            error: Error::new(
                pos_begin,
                pos_end,
                "DivisionByZero Error".to_string(),
                error_message,
            ),
        }
    }
    pub fn as_string(&self) -> String {
        self.error.as_string()
    }
}
