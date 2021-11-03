#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    index: i64,
    line: i64,
    column: i64,
    file_name: String,
    file_text: String,
}

impl Position {
    pub fn new(index: i64, line: i64, column: i64, file_name: String, file_text: String) -> Self {
        Self {
            index,
            line,
            column,
            file_name,
            file_text,
        }
    }

    pub fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.column += 1;

        if let Some(c) = current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            }
        }
    }

    pub fn index(&self) -> i64 {
        self.index
    }

    pub fn line(&self) -> i64 {
        self.line
    }

    pub fn column(&self) -> i64 {
        self.column
    }

    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
}
