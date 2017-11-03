use std::error;
use std::fmt;

use route;

#[derive(Debug)]
pub struct Error {
    details: String,
    cause: Option<Box<error::Error>>,
}

impl Error {
    pub fn ident_type(ident: &str, type_: &str) -> Error {
        Error {
            details: format!("{} is not a valid {} identifier", ident, type_),
            cause: None,
        }
    }

    pub fn undefined(ident: &str) -> Error {
        Error {
            details: format!("Variable {} not defined", ident),
            cause: None,
        }
    }

    pub fn macro_args(id: &str, got: usize, expected: usize) -> Error {
        let details = format!("Incorrect number of arguments to macro {} \
                          (got {}, expected {})",
                          id, got, expected);
        Error {
            details,
            cause: None,
        }
    }

    pub fn macro_arg_type(macro_id: &str,
                          arg_id: &str,
                          type_: &str,
                          val: &str) -> Error {
        let details = format!("Argument {} to macro {} is not a {} (got {})",
                          arg_id, macro_id, type_, val);
        Error {
            details,
            cause: None,
        }
    }

    pub fn segment(seg: route::Segment) -> Error {
        Error {
            details: format!("Segment {:?} not defined", seg),
            cause: None,
        }
    }

    pub fn from_val<T: fmt::Debug>(val: T) -> Error {
        Error {
            details: format!("{:?}", val),
            cause: None,
        }
    }

    pub fn from_str(details: &str) -> Error {
        Error {
            details: String::from(details),
            cause: None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }

    fn cause(&self) -> Option<&error::Error> {
        self.cause.as_ref().map(|e| e.as_ref())
    }
}
