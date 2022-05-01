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
        T: BoulderBase,
        T: MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>,
        C: persian_rug::Context
     {
        type Builder = <T as MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>>::Builder;
        fn builder() -> Self::Builder {
            <T as MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>>::mini_builder()
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

    pub trait BoulderBase {
        type Base;
    }
    
    impl<T> BoulderBase for Option<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Arc<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Rc<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for RefCell<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Cell<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Mutex<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for persian_rug::Proxy<T>
    where
        T: BoulderBase
    {
        type Base = <T as BoulderBase>::Base;
    }
}    
