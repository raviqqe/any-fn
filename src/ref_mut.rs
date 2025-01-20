use core::marker::PhantomData;

/// A mutable reference.
///
/// This type is purely an annotator for the [IntoAnyFn][crate::IntoAnyFn] trait
/// and never constructed.
pub struct RefMut<T> {
    _data: PhantomData<T>,
}
