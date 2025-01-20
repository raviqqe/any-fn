use core::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// An error.
#[derive(Debug)]
pub enum AnyFnError {
    /// An object downcast failure.
    Downcast,
}

impl Error for AnyFnError {}

impl Display for AnyFnError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Downcast => write!(formatter, "cannot downcast object"),
        }
    }
}
