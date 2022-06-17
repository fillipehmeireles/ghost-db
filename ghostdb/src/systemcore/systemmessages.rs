use std::fmt;

#[derive(Debug)]
pub enum ErrorsMessages {
    ErrorOnInsert,
    ErrorOnUpdate,
    ErrorOnDelete,
}

#[derive(Debug)]
pub enum SuccessMessages {
    SuccesOnInsert,
    SuccesOnUpdate,
    SuccesOnDelete,
}

impl fmt::Display for ErrorsMessages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorsMessages::ErrorOnInsert => write!(f, "ErrorOnInsert"),
            ErrorsMessages::ErrorOnUpdate => write!(f, "ErrorOnUpdate"),
            ErrorsMessages::ErrorOnDelete => write!(f, "ErrorOnDelete"),
        }
    }
}

impl fmt::Display for SuccessMessages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuccessMessages::SuccesOnInsert => write!(f, "SuccessOnInsert"),
            SuccessMessages::SuccesOnUpdate => write!(f, "SuccessOnUpdate"),
            SuccessMessages::SuccesOnDelete => write!(f, "SuccessOnDelete"),
        }
    }
}
