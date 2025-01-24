use core::{
    cell::{BorrowError, BorrowMutError},
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error.
#[derive(Debug)]
pub enum AnyFnError {
    /// A borrow failure.
    Borrow(BorrowError),
    /// A mutable borrow failure.
    BorrowMut(BorrowMutError),
    /// An object downcast failure.
    Downcast,
}

impl From<BorrowError> for AnyFnError {
    fn from(error: BorrowError) -> Self {
        Self::Borrow(error)
    }
}

impl From<BorrowMutError> for AnyFnError {
    fn from(error: BorrowMutError) -> Self {
        Self::BorrowMut(error)
    }
}

impl Error for AnyFnError {}

impl Display for AnyFnError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Downcast => write!(formatter, "cannot downcast object"),
            Self::Borrow(error) => write!(formatter, "{error}"),
            Self::BorrowMut(error) => write!(formatter, "{error}"),
        }
    }
}
