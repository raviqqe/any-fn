use crate::{error::AnyFnError, AnyCell, AnyFn, RefMut};
use alloc::boxed::Box;
use core::{any::Any, mem::size_of};

/// A trait to convert a statically-typed function into a dynamically-typed function.
pub trait IntoAnyFn<'a, T, S> {
    /// Converts itself into a dynamically-typed function.
    fn into_any_fn(self) -> AnyFn<'a>;
}

macro_rules! impl_function {
    ([$($type:ident),*], [$($ref_mut:ident),*]) => {
        impl<'a, T1: FnMut($($type,)* $(&mut $ref_mut,)*) -> T2 + 'a, T2: Any, $($type: Any + Clone,)* $($ref_mut: Any,)*> IntoAnyFn<'a, ($($type,)* $(RefMut<$ref_mut>,)*), T2> for T1 {
            #[allow(non_snake_case)]
            fn into_any_fn(mut self) -> AnyFn<'a> {
                #[allow(unused, unused_mut)]
                AnyFn::new(
                    (&[$(size_of::<$type>(),)* $(size_of::<$ref_mut>(),)*] as &[usize]).len(),
                    Box::new(move |arguments: &[AnyCell]| {
                        let mut iter = 0..;

                        Ok(Box::new(self(
                            $(
                                arguments[iter.next().unwrap_or_default()]
                                .borrow()
                                .downcast_ref::<$type>()
                                .ok_or(AnyFnError::Downcast)?
                                .clone(),
                            )*
                            $(
                                arguments[iter.next().unwrap_or_default()]
                                .borrow_mut()
                                .downcast_mut::<$ref_mut>()
                                .ok_or(AnyFnError::Downcast)?,
                            )*
                        )))
                    }),
                )
            }
        }
    };
}

macro_rules! impl_ref_mut_functions {
    ([$($type:ident),*], [$first_ref_mut:ident, $($ref_mut:ident),*]) => {
        impl_function!([$($type),*], [$first_ref_mut, $($ref_mut),*]);
        impl_ref_mut_functions!([$($type),*], [$($ref_mut),*]);
    };
    ([$($type:ident),*], [$ref_mut:ident]) => {
        impl_function!([$($type),*], [$ref_mut]);
        impl_function!([$($type),*], []);
    }
}

macro_rules! impl_functions {
    ([$first_type:ident, $($type:ident),*], [$($ref_mut:ident),*]) => {
        impl_ref_mut_functions!([$first_type, $($type),*], [$($ref_mut),*]);
        impl_functions!([$($type),*], [$($ref_mut),*]);
    };
    ([$type:ident], [$($ref_mut:ident),*]) => {
        impl_ref_mut_functions!([$type], [$($ref_mut),*]);
        impl_ref_mut_functions!([], [$($ref_mut),*]);
    }
}

impl_functions!(
    [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z],
    [い, ろ, は, に, お, へ, と, ち, り, ぬ, る, を]
);
