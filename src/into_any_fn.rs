use crate::{error::AnyFnError, AnyCell, DynamicFunction, RefMut};
use alloc::boxed::Box;
use core::{any::Any, mem::size_of};

/// A native function dynamically defined.
pub trait IntoDynamicFunction<'a, T, S> {
    /// Converts itself into a dynamic function.
    fn into_dynamic(self) -> DynamicFunction<'a>;
}

macro_rules! impl_function {
    ([$($type:ident),*], [$($ref:ident),*]) => {
        impl<'a, T1: FnMut($($type,)* $(&mut $ref,)*) -> T2 + 'a, T2: Any, $($type: Any + Clone,)* $($ref: Any,)*> IntoDynamicFunction<'a, ($($type,)* $(RefMut<$ref>,)*), T2> for T1 {
            #[allow(non_snake_case)]
            fn into_dynamic(mut self) -> DynamicFunction<'a> {
                #[allow(unused, unused_mut)]
                DynamicFunction::new(
                    (&[$(size_of::<$type>(),)* $(size_of::<$ref>(),)*] as &[usize]).len(),
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
                                .downcast_mut::<$ref>()
                                .ok_or(AnyFnError::Downcast)?,
                            )*
                        )))
                    }),
                )
            }
        }
    };
}

macro_rules! impl_ref_functions {
    ([$($type:ident),*], [$first_ref:ident, $($ref:ident),*]) => {
        impl_function!([$($type),*], [$first_ref, $($ref),*]);
        impl_ref_functions!([$($type),*], [$($ref),*]);
    };
    ([$($type:ident),*], [$ref:ident]) => {
        impl_function!([$($type),*], [$ref]);
        impl_function!([$($type),*], []);
    }
}

macro_rules! impl_functions {
    ([$first_type:ident, $($type:ident),*], [$($ref:ident),*]) => {
        impl_ref_functions!([$first_type, $($type),*], [$($ref),*]);
        impl_functions!([$($type),*], [$($ref),*]);
    };
    ([$type:ident], [$($ref:ident),*]) => {
        impl_ref_functions!([$type], [$($ref),*]);
        impl_ref_functions!([], [$($ref),*]);
    }
}

impl_functions!(
    [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z],
    [い, ろ, は, に, お, へ, と, ち, り, ぬ, る, を]
);
