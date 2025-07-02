macro_rules! impl_for {
    ($($T: ident)*) => {
        impl_for!(@unfold [] [$($T)*]);
    };
    (@impl $($T: ident)*) => {
        impl <D: ?Sized, $($T),*> crate::Modification<D> for ($($T,)*)
        where
            $($T: crate::Modification<D>,)*
        {
            #[allow(unused_variables, non_snake_case)]
            fn modify(self, value: &mut D) {
                let ($($T,)*) = self;
                $(
                    $T.modify(value);
                )*
            }
        }
    };
    (@unfold [$($T: ident)*] []) => {
        impl_for!(@impl $($T)*);
    };
    (@unfold [$($T: ident)*] [$TN: ident $($TRest: ident)*]) => {
        impl_for!(@impl $($T)* );
        impl_for!(@unfold [$($T)* $TN] [$($TRest)*]);
    };
}

impl_for!(T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15);
