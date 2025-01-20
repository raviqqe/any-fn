#[cfg(doc)]
use crate::IntoAnyFn;
use core::marker::PhantomData;

/// An immutable reference.
///
/// This type is purely an annotator for the [IntoAnyFn] trait
/// and never constructed.
pub struct Ref<T> {
    _data: PhantomData<T>,
}
