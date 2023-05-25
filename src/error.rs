pub struct SunError {
    line: usize,
    message: String,
}

impl SunError {
    pub fn error(line: usize, message: String) -> SunError {
        SunError { line, message }
    }

    pub fn report(&self, location: String) {
        eprintln!("[line {}] Error{}: {}", self.line, location, self.message);
    }
}
