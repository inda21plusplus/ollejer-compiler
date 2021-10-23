pub trait Error {
    fn as_string(&self) -> String;
}

pub struct DisallowedCharError {
    error_name: String,
    error_message: String,
}

impl DisallowedCharError {
    pub fn new(error_name: String, error_message: String) -> Self {
        Self {
            error_name,
            error_message,
        }
    }
}

impl Error for DisallowedCharError {
    fn as_string(&self) -> String {
        format!("{}: {}", self.error_name, self.error_message)
    }
}
