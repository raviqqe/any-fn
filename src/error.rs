use core::{
    error,
    fmt::{self, Display, Formatter},
};

/// An error.
#[derive(Debug)]
pub enum AnyFnError {
    /// A object downcast error.
    Downcast,
    /// A object index error.
    ObjectIndex,
}

impl error::Error for AnyFnError {}

impl Display for AnyFnError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::Downcast => write!(formatter, "cannot downcast object"),
            Self::ObjectIndex => write!(formatter, "invalid object index"),
        }
    }
}
