/// Something which can be turned into an object of type `Result`
pub trait Builder {
    type Result;
    /// Create the final object.
    ///
    /// Example
    /// ```rust
    /// use boulder::Builder;
    ///
    /// struct Foo {
    ///    a: i32
    /// }
    ///
    /// struct FooBuilder {
    ///    a: i32
    /// }
    ///
    /// impl Builder for FooBuilder {
    ///    type Result = Foo;
    ///    fn build(self) -> Self::Result {
    ///       Foo { a: self.a }
    ///    }
    /// }
    ///
    /// let b = FooBuilder { a: 3 };
    /// let f = b.build();
    /// assert_eq!(f.a, 3);
    /// ```
    fn build(self) -> Self::Result;
}

/// A type that has an associated default `Builder`.
///
/// The convenient way to implement this trait is via the `Buildable`
/// derive macro.
pub trait Buildable: Sized {
    type Builder: guts::MegaBuilder<Self>;
    /// Create a new builder for this type.
    ///
    /// Example
    /// ```rust
    /// use boulder::{Builder, Buildable};
    ///
    /// struct Foo {
    ///    a: i32
    /// }
    ///
    /// impl Buildable for Foo {
    ///    type Builder = FooBuilder;
    ///    fn builder() -> Self::Builder {
    ///       FooBuilder { a: 0 }
    ///    }
    /// }
    ///
    /// struct FooBuilder {
    ///    a: i32
    /// }
    ///
    /// impl Builder for FooBuilder {
    ///    type Result = Foo;
    ///    fn build(self) -> Self::Result {
    ///       Foo { a: self.a }
    ///    }
    /// }
    ///
    /// let b = Foo::builder();
    /// let f = b.build();
    /// assert_eq!(f.a, 0);
    /// ```
    fn builder() -> Self::Builder;
}

pub use boulder_derive::Buildable;

#[doc(hidden)]
pub mod guts {
    use super::{Builder, Buildable};

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    
    pub trait MegaBuildable<T> {
        type Builder: MegaBuilder<T>;
        fn builder(marker: core::marker::PhantomData<T>) -> Self::Builder;
    }

    pub trait MegaBuilder<T>: Sized {
        fn build(self) -> T;
    }
    
    impl<T> Buildable for T
    where
        T: MegaBuildable<T>,
     {
        type Builder = <T as MegaBuildable<T>>::Builder;
        fn builder() -> Self::Builder {
            <T as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuildable<Option<T>> for Option<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Option<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Option<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Option<T> {
            Some(<U as MegaBuilder<T>>::build(self))
        }
    }

    impl<U, T> MegaBuildable<Rc<T>> for Rc<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Rc<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Rc<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Rc<T> {
            Rc::new(<U as MegaBuilder<T>>::build(self))
        }
    }


    impl<U, T> MegaBuildable<Arc<T>> for Arc<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Arc<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Arc<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Arc<T> {
            Arc::new(<U as MegaBuilder<T>>::build(self))
        }
    }

    impl<U, T> MegaBuildable<Mutex<T>> for Mutex<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Mutex<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Mutex<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Mutex<T> {
            Mutex::new(<U as MegaBuilder<T>>::build(self))
        }
    }


    impl<U, T> MegaBuildable<RefCell<T>> for RefCell<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<RefCell<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<RefCell<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> RefCell<T> {
            RefCell::new(<U as MegaBuilder<T>>::build(self))
        }
    }

    impl<U, T> MegaBuildable<Cell<T>> for Cell<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Cell<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Cell<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Cell<T> {
            Cell::new(<U as MegaBuilder<T>>::build(self))
        }
    }

    impl<U, T> MegaBuildable<Box<T>> for Box<U>
    where
        U: MegaBuildable<T>
    {
        type Builder = <U as MegaBuildable<T>>::Builder;
        fn builder(_marker: core::marker::PhantomData<Box<T>>) -> Self::Builder {
            <U as MegaBuildable<T>>::builder(Default::default())
        }
    }

    impl<U, T> MegaBuilder<Box<T>> for U
    where
        U: MegaBuilder<T>
    {
        fn build(self) -> Box<T> {
            Box::new(<U as MegaBuilder<T>>::build(self))
        }
    }
    
    // impl<T, U> Builder for T
    // where
    //     T: MegaBuilder<U>
    // {
    //     type Result = U;
    //     fn build(self) -> Self::Result {
    //         <T as MegaBuilder<U>>::build(self)
    //     }
    // }

    
// #[doc(hidden)]
// pub trait Converter<From> {
//     type Output;
//     fn convert(self, input: From) -> Self::Output;
// }

// #[doc(hidden)]
// pub struct SelfConverter;

// impl<T> Converter<T> for SelfConverter {
//     type Output = T;
//     fn convert(self, input: T) -> T {
//         input
//     }
// }

// #[doc(hidden)]
// pub trait OptionBuildable: Sized {
//     type Builder: Builder<Result = Option<Self>>;
//     fn option_builder() -> Self::Builder;
// }

// impl<T> Buildable for Option<T>
// where
//     T: OptionBuildable,
// {
//     type Builder = <T as OptionBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::option_builder()
//     }
// }

// #[doc(hidden)]
// pub struct OptionConverter;

// impl<T> Converter<T> for OptionConverter {
//     type Output = Option<T>;
//     fn convert(self, input: T) -> Option<T> {
//         Some(input)
//     }
// }

// #[doc(hidden)]
// pub trait RcBuildable: Sized {
//     type Builder: Builder<Result = Rc<Self>>;
//     fn rc_builder() -> Self::Builder;
// }

// impl<T> Buildable for Rc<T>
// where
//     T: RcBuildable,
// {
//     type Builder = <T as RcBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::rc_builder()
//     }
// }

// #[doc(hidden)]
// pub struct RcConverter;

// impl<T> Converter<T> for RcConverter {
//     type Output = Rc<T>;
//     fn convert(self, input: T) -> Rc<T> {
//         Rc::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait ArcBuildable: Sized {
//     type Builder: Builder<Result = Arc<Self>>;
//     fn arc_builder() -> Self::Builder;
// }

// impl<T> Buildable for Arc<T>
// where
//     T: ArcBuildable,
// {
//     type Builder = <T as ArcBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::arc_builder()
//     }
// }

// #[doc(hidden)]
// pub struct ArcConverter;

// impl<T> Converter<T> for ArcConverter {
//     type Output = Arc<T>;
//     fn convert(self, input: T) -> Arc<T> {
//         Arc::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait MutexBuildable: Sized {
//     type Builder: Builder<Result = Mutex<Self>>;
//     fn mutex_builder() -> Self::Builder;
// }

// impl<T> Buildable for Mutex<T>
// where
//     T: MutexBuildable,
// {
//     type Builder = <T as MutexBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::mutex_builder()
//     }
// }

// #[doc(hidden)]
// pub struct MutexConverter;

// impl<T> Converter<T> for MutexConverter {
//     type Output = Mutex<T>;
//     fn convert(self, input: T) -> Mutex<T> {
//         Mutex::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait RefCellBuildable: Sized {
//     type Builder: Builder<Result = RefCell<Self>>;
//     fn ref_cell_builder() -> Self::Builder;
// }

// impl<T> Buildable for RefCell<T>
// where
//     T: RefCellBuildable,
// {
//     type Builder = <T as RefCellBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::ref_cell_builder()
//     }
// }

// #[doc(hidden)]
// pub struct RefCellConverter;

// impl<T> Converter<T> for RefCellConverter {
//     type Output = RefCell<T>;
//     fn convert(self, input: T) -> RefCell<T> {
//         RefCell::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait CellBuildable: Sized {
//     type Builder: Builder<Result = Cell<Self>>;
//     fn cell_builder() -> Self::Builder;
// }

// impl<T> Buildable for Cell<T>
// where
//     T: CellBuildable,
// {
//     type Builder = <T as CellBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::cell_builder()
//     }
// }

// #[doc(hidden)]
// pub struct CellConverter;

// impl<T> Converter<T> for CellConverter {
//     type Output = Cell<T>;
//     fn convert(self, input: T) -> Cell<T> {
//         Cell::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait BoxBuildable: Sized {
//     type Builder: Builder<Result = Box<Self>>;
//     fn box_builder() -> Self::Builder;}


// impl<T> Buildable for Box<T>
// where
//     T: BoxBuildable,
// {
//     type Builder = <T as BoxBuildable>::Builder;
//     fn builder() -> Self::Builder {
//         T::box_builder()
//     }
// }

// #[doc(hidden)]
// pub struct BoxConverter;

// impl<T> Converter<T> for BoxConverter {
//     type Output = Box<T>;
//     fn convert(self, input: T) -> Box<T> {
//         Box::new(input)
//     }
// }

// #[doc(hidden)]
// pub trait ArcMutexBuildable: Sized {
//     type Builder: Builder<Result = Arc<Mutex<Self>>>;
//     fn arc_mutex_builder() -> Self::Builder;
// }

// impl<T> ArcBuildable for Mutex<T>
// where
//     T: ArcMutexBuildable,
// {
//     type Builder = <T as ArcMutexBuildable>::Builder;
//     fn arc_builder() -> Self::Builder {
//         T::arc_mutex_builder()
//     }
// }

// #[doc(hidden)]
// pub struct ArcMutexConverter;

// impl<T> Converter<T> for ArcMutexConverter {
//     type Output = Arc<Mutex<T>>;
//     fn convert(self, input: T) -> Arc<Mutex<T>> {
//         Arc::new(Mutex::new(input))
//     }
// }
}
