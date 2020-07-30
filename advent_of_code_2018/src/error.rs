use std::error::Error;

pub type TError = Box<dyn Error>;
pub type TResult<T> = Result<T, TError>;
