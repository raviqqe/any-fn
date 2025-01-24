use crate::{AnyFn, Ref, RefMut, Value};
use alloc::{boxed::Box, vec};
use core::any::{Any, TypeId};

/// A trait to convert a statically-typed function into a dynamically-typed
/// function.
pub trait IntoAnyFn<'a, T, S> {
    /// Converts itself into a dynamically-typed function.
    fn into_any_fn(self) -> AnyFn<'a>;
}

macro_rules! impl_function {
    ([$($name:ident),*], [$([$($trait:tt),*]),*], [$($parameter:ty),*], [$($argument:item),*], [$($type:ty),*]) => {
        #[allow(unused_parens)]
        impl<'a, T1: FnMut($($parameter),*) -> T2 + 'a, T2: Any, $($name: Any $(+$trait)*),*> IntoAnyFn<'a, ($($type,)*), T2> for T1 {
            #[allow(non_snake_case)]
            fn into_any_fn(mut self) -> AnyFn<'a> {
                #[allow(unused, unused_mut)]
                AnyFn::new(
                    vec![$(TypeId::of::<$name>()),*],
                    TypeId::of::<T2>(),
                    Box::new(move |arguments: &[&Value]| {
                        let mut iter = 0..;
                        $($argument);*
                        Ok(Value::new(self($($name!(arguments, iter)),*)))
                    }),
                )
            }
        }
    };
}

macro_rules! impl_function_combination {
    ([$($x:ident),*]) => {
        impl_function_combination!([$($x),*], [], [], [], [], []);
    };
    (
        [$x:ident$(,)? $($y:ident),*],
        [$($name:ident),* $(,)?],
        [$([$($trait:tt),* $(,)?]),* $(,)?],
        [$($parameter:ty),* $(,)?],
        [$($argument:item),* $(,)?],
        [$($type:ty),* $(,)?]
    ) => {
        impl_function_combination!(
            [$($y),*],
            [$x, $($name),*],
            [[Clone], $([$($trait),*]),*],
            [$x, $($parameter),*],
            [
                macro_rules! $x {
                    ($arguments:ident, $iter:ident) => {
                        $arguments[$iter.next().unwrap_or_default()]
                            .downcast_ref::<$x>()?
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
            [[], $([$($trait),*]),*],
            [&$x, $($parameter),*],
            [
                macro_rules! $x {
                    ($arguments:ident, $iter:ident) => {
                        &*($arguments[$iter.next().unwrap_or_default()]
                            .downcast_ref::<$x>()?)
                    };
                },
                $($argument),*
            ],
            [Ref<$x>, $($type),*]
        );
        impl_function_combination!(
            [$($y),*],
            [$x, $($name),*],
            [[], $([$($trait),*]),*],
            [&mut $x, $($parameter),*],
            [
                macro_rules! $x {
                    ($arguments:ident, $iter:ident) => {
                        &mut *($arguments[$iter.next().unwrap_or_default()]
                            .downcast_mut::<$x>()?)
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
        [$([$($trait:tt),* $(,)?]),* $(,)?],
        [$($parameter:ty),* $(,)?],
        [$($argument:item),* $(,)?],
        [$($type:ty),* $(,)?]
    ) => {
        impl_function!(
            [$($name),*],
            [$([$($trait),*]),*],
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

impl_functions!(A, B, C, D, E, F);
