use crate::context::Context;
use crate::position::Position;
use std::cmp;
use std::fmt::{self, format};

#[derive(Debug, Clone)]
pub enum ErrorType {
    DisallowedCharError(DisallowedCharError),
    SyntaxError(SyntaxError),
    RunTimeError(RunTimeError),
    DivisionByZeroError(DivisionByZeroError),
}

/*
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ErrorType::DisallowedCharError(e) => write!(f, "{:?}", e),
            &ErrorType::SyntaxError(e) => write!(f, "{:?}", e),
            &ErrorType::RunTimeError(e) => write!(f, "{:?}", e),
            &ErrorType::DivisionByZeroError(e) => write!(f, "{:?}", e),

    }
}
*/

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
        let file_name: String;
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
        // Dont know why arrow wont appear automatically here but placing one manually fixes it./
        // String with arrows adds spaces though.
        // Would fix but not worthy of my time right now.
        format!(
            "{}\n\n{}^",
            error,
            string_with_arrows(self.pos_begin.clone(), self.pos_end.clone(),)
        )
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
    context: Context,
}

impl RunTimeError {
    pub fn new(
        pos_begin: Option<Position>,
        pos_end: Option<Position>,
        error_message: String,
        context: Context,
    ) -> Self {
        Self {
            error: Error::new(
                pos_begin,
                pos_end,
                "Runtime Error".to_string(),
                error_message,
            ),
            context,
        }
    }
    pub fn as_string(&self) -> String {
        let result = self.traceback_error();
        let result = format!(
            "{} {}: {}",
            result, self.error.error_name, self.error.error_message
        );
        format!(
            "{}\n\n{}",
            result,
            string_with_arrows(self.error.pos_begin.clone(), self.error.pos_end.clone(),)
        )
    }

    pub fn traceback_error(&self) -> String {
        let mut result = String::new();
        let mut position = self.error.pos_begin.clone();
        let mut context = Some(self.context.clone());

        while let Some(ctx) = context {
            let pos = position.unwrap(); // If context is not None so should pos
            result = format!(
                "File: {} Line {} Col {}, in {} \n{}",
                pos.file_name(),
                pos.line(),
                pos.column(),
                ctx.display_name(),
                result
            );
            position = ctx.parent_pos();
            context = match ctx.parent() {
                Some(c) => Some(*c),
                None => None,
            };
        }

        format!("Traceback from most recent callback: \n{}", result)
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

fn string_with_arrows(pos_start: Option<Position>, pos_end: Option<Position>) -> String {
    let mut result = String::new();

    let pos_start = match pos_start {
        Some(pos) => pos,
        None => panic!("Cant construct ERROR arrows"),
    };

    let pos_end = match pos_end {
        Some(pos) => pos,
        None => pos_start.clone(),
    };

    let text = pos_start.file_text();

    // Calc slice to show arrows under
    let text_start_slice = &text[0..pos_start.index() as usize];
    let text_end_slice = &text[pos_end.index() as usize..];

    let mut index_start = match text_start_slice.to_string().rfind('\n') {
        Some(idx) => idx,
        None => 0,
    };
    let mut index_end = match text_end_slice.to_string().find('\n') {
        Some(idx) => idx,
        None => text.len(),
    };

    // Gen lines
    let line_count = pos_end.line() - pos_start.line() + 1;
    for i in 0..line_count {
        let line_ = &text[index_start..index_end];

        let column_start = {
            if i == 0 {
                pos_start.column()
            } else {
                0
            }
        };
        let column_end = {
            if i == line_count - 1 {
                pos_end.column()
            } else {
                line_.len() as i64 - 1
            }
        };

        // Add To resulting Output String
        result = format!("{}{}\n", result, line_);
        for _ in 0..column_start {
            result = format!("{} ", result) // Add beginning Spaces
        }
        for _ in 0..(column_end - column_start) {
            result = format!("{}^", result)
        }

        // Re-Calc slices
        index_start = index_end;
        let mut index_end = match text_end_slice.to_string().find('\n') {
            Some(idx) => idx,
            None => text.len(),
        };
    }

    result.replace('\t', "")
}
