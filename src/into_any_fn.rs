use crate::{error::AnyFnError, AnyCell, AnyFn, RefMut};
use alloc::boxed::Box;
use core::{any::Any, mem::size_of};

/// A trait to convert a statically-typed function into a dynamically-typed function.
pub trait IntoAnyFn<'a, T, S> {
    /// Converts itself into a dynamically-typed function.
    fn into_any_fn(self) -> AnyFn<'a>;
}

// macro_rules! annotate {
//     (0, $type:ident) => {
//         RefMut<$type>
//     };
//     (1, $type:ident) => {
//         $type
//     };
//     (2, &$type:ident) => {
//         $type
//     };
// }
//
// macro_rules! argument {
//     (0, $type:ident, $arguments:ident, $iter:ident) => {
//         $arguments[$iter.next().unwrap_or_default()]
//             .borrow()
//             .downcast_ref::<$type>()
//             .ok_or(AnyFnError::Downcast)?
//             .clone()
//     };
//     (1, $type:ident, $arguments:ident, $iter:ident) => {
//         $arguments[$iter.next().unwrap_or_default()]
//             .borrow_mut()
//             .downcast_mut::<$type>()
//             .ok_or(AnyFnError::Downcast)?
//     };
// }
//
// macro_rules! impl_function {
//     ([$($type:ty),*], [$(($kind:lit, $name:ty)),*]) => {
//         impl<'a, T1: FnMut($($type),*) -> T2 + 'a, T2: Any, $($name: Any + Clone),*> IntoAnyFn<'a, $(annotate!($type)),*, T2> for T1 {
//             #[allow(non_snake_case)]
//             fn into_any_fn(mut self) -> AnyFn<'a> {
//                 #[allow(unused, unused_mut)]
//                 AnyFn::new(
//                     (&[$(size_of::<$type>()),*] as &[usize]).len(),
//                     Box::new(move |arguments: &[AnyCell]| {
//                         let mut iter = 0..;
//
//                         Ok(Box::new(self($(argument!($type, arguments, iter)),*)))
//                     }),
//                 )
//             }
//         }
//     };
// }

macro_rules! impl_function_combination {
    ([$first_type:ident$(,)? $($type:ident),*], [$(($kind:literal, $name:ident)),* $(,)?]) => {
        impl_function_combination!([$($type),*], [(0, $first_type), $(($kind, $name)),*]);
        impl_function_combination!([$($type),*], [(1, $first_type), $(($kind, $name)),*]);
    };
    ([], [$(($kind:literal, $name:ident)),* $(,)?]) => {
        // impl_function!([$type], [$(($kind:literal, $name:ident)),*]);
        // impl_function!([&mut $type], [$(($kind:literal, $name:ident)),*]);
    }
}

macro_rules! impl_functions {
    ($first_type:ident, $($type:ident),*) => {
        impl_function_combination!([$first_type, $($type),*], []);
        impl_functions!($($type),*);
    };
    ($type:ident) => {
        impl_function_combination!([$type], []);
        impl_function_combination!([], []);
    }
}

impl_functions!(A, B, C);
