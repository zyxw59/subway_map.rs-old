use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IdentError {
    details: String
}

impl IdentError {
    pub fn new(ident: &str, type_: &str) -> IdentError {
        IdentError{
            details: format!("{} is not a valid {} identifier", ident, type_),
        }
    }
}

impl fmt::Display for IdentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for IdentError {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

