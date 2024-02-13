use std::error::Error;
use std::fmt;

/// UnsupportApiError
#[derive(Debug, Clone)]
pub struct UnsupportApiError;
impl fmt::Display for UnsupportApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "unsupported api")
    }
}
impl Error for UnsupportApiError {}

/// UnsupportOsError
#[derive(Debug, Clone)]
pub struct UnsupportOsError;
impl fmt::Display for UnsupportOsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "unsupported os")
    }
}
impl Error for UnsupportOsError {}

/// UnsupportOsError
#[derive(Debug, Clone)]
pub struct DeepLEmptyAuthKeyError;
impl fmt::Display for DeepLEmptyAuthKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "please privode a deepl auth key")
    }
}
impl Error for DeepLEmptyAuthKeyError {}