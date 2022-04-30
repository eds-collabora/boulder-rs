pub trait BuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    type Builder: BuilderWithPersianRug<C, Result = Self>;
    fn builder() -> Self::Builder;
}

pub trait BuilderWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    type Result;
    fn build<'b, B>(self, context: B) -> (Self::Result, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

#[doc(hidden)]
pub mod guts {
    use super::{BuilderWithPersianRug, BuildableWithPersianRug};
    use crate::builder::guts::BuilderBase;
        
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub trait MiniBuildableWithPersianRug<T, C>: Sized
    where
        C: persian_rug::Context
    {
        type Builder: MiniBuilderWithPersianRug<C, Result=Self>;
        fn mini_builder() -> Self::Builder;
    }
    
    pub trait MiniBuilderWithPersianRug<C>: Sized
    where
        C: persian_rug::Context
    {
        type Result;
        fn build<'b, B>(self, context: B) -> (Self::Result, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>;
    }
    
    impl<T, C> BuildableWithPersianRug<C> for T
    where
        T: BuilderBase,
        T: MiniBuildableWithPersianRug<<T as BuilderBase>::Base, C>,
        C: persian_rug::Context
     {
        type Builder = <T as MiniBuildableWithPersianRug<<T as BuilderBase>::Base, C>>::Builder;
        fn builder() -> Self::Builder {
            <T as MiniBuildableWithPersianRug<<T as BuilderBase>::Base, C>>::mini_builder()
        }
    }

    impl<T, C> BuilderWithPersianRug<C> for T
    where
        T: MiniBuilderWithPersianRug<C>,
        C: persian_rug::Context
    {
        type Result =<T as MiniBuilderWithPersianRug<C>>::Result;
        fn build<'b, B>(self, context: B) -> (Self::Result, B)
        where
            B: 'b + persian_rug::Mutator<Context=C>
        {
            <Self as MiniBuilderWithPersianRug<C>>::build(self, context)
        }
    }

    impl<T> BuilderBase for persian_rug::Proxy<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }
}    
