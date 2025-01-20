use crate::{error::AnyFnError, AnyCell, AnyFn, RefMut};
use alloc::boxed::Box;
use core::{any::Any, mem::size_of};

/// A trait to convert a statically-typed function into a dynamically-typed function.
pub trait IntoAnyFn<'a, T, S> {
    /// Converts itself into a dynamically-typed function.
    fn into_any_fn(self) -> AnyFn<'a>;
}

macro_rules! impl_function {
    ([$($name:ident),*], [$($parameter:ty),*], [$($argument:item),*], [$($type:ty),*]) => {
        #[allow(unused_parens)]
        impl<'a, T1: FnMut($($parameter),*) -> T2 + 'a, T2: Any, $($name: Any + Clone),*> IntoAnyFn<'a, ($($type,)*), T2> for T1 {
            #[allow(non_snake_case)]
            fn into_any_fn(mut self) -> AnyFn<'a> {
                #[allow(unused, unused_mut)]
                AnyFn::new(
                    (&[$(size_of::<$name>()),*] as &[usize]).len(),
                    Box::new(move |arguments: &[AnyCell]| {
                        let mut iter = 0..;
                        $($argument);*
                        Ok(Box::new(self($($name!(arguments, iter)),*)))
                    }),
                )
            }
        }
    };
}

macro_rules! impl_function_combination {
    ([$($x:ident),*]) => {
        impl_function_combination!([$($x),*], [], [], [], []);
    };
    (
        [$x:ident$(,)? $($y:ident),*],
        [$($name:ident),* $(,)?],
        [$($parameter:ty),* $(,)?],
        [$($argument:item),* $(,)?],
        [$($type:ty),* $(,)?]
    ) => {
        impl_function_combination!(
            [$($y),*],
            [$x, $($name),*],
            [$x, $($parameter),*],
            [
                macro_rules! $x {
                    ($arguments:ident, $iter:ident) => {
                        $arguments[$iter.next().unwrap_or_default()]
                            .borrow()
                            .downcast_ref::<$x>()
                            .ok_or(AnyFnError::Downcast)?
                            .clone()
                    };
                },
                $($argument),*
            ],
            [$x, $($type),*]
        );
        impl_function_combination!(
            [$($y),*],
            [$x, $($name),*],
            [&mut $x, $($parameter),*],
            [
                macro_rules! $x {
                    ($arguments:ident, $iter:ident) => {
                        $arguments[$iter.next().unwrap_or_default()]
                            .borrow_mut()
                            .downcast_mut::<$x>()
                            .ok_or(AnyFnError::Downcast)?
                    };
                },
                $($argument),*
            ],
            [RefMut<$x>, $($type),*]
        );
    };
    (
        [],
        [$($name:ident),* $(,)?],
        [$($parameter:ty),* $(,)?],
        [$($argument:item),* $(,)?],
        [$($type:ty),* $(,)?]
    ) => {
        impl_function!(
            [$($name),*],
            [$($parameter),*],
            [$($argument),*],
            [$($type),*]
        );
    }
}

macro_rules! impl_functions {
    ($first_type:ident, $($type:ident),*) => {
        impl_function_combination!([$first_type, $($type),*]);
        impl_functions!($($type),*);
    };
    ($type:ident) => {
        impl_function_combination!([$type]);
        impl_function_combination!([]);
    }
}

impl_functions!(A, B, C, D, E, F, G, H);
