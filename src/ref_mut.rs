use core::marker::PhantomData;

/// A mutable reference type.
///
/// This type is purely an annotator for the [`crate::IntoAnyFn`] trait and
/// never constructed.
pub struct RefMut<T> {
    _data: PhantomData<T>,
}
