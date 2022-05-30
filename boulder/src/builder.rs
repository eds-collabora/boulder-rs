/// Something which can create a default object of some type.
///
/// The only required function in this trait is
/// [`build`](Builder::build) which creates an object, consuming the
/// builder. Most builders will allow customisation of the produced
/// object in some way.
///
/// An object implementing this trait will be automatically created
/// for you as part of the [`macro@Buildable`] derive macro. That
/// builder will have a method for each field of the result type, to
/// customise its value, and will produce a default value for every
/// field which is not customised.
pub trait Builder {
    /// The output type.
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

/// A type that has an associated default [`Builder`].
///
/// This trait is implemented via the [`macro@Buildable`] derive
/// macro. It cannot be directly implemented because the library
/// itself provides a blanket implementation from a more complex
/// underlying trait `MiniBuildable`, which is not currently
/// documented.
///
/// This restriction may be removed in a future version; much of the
/// complexity in this module stems from lacking generic associated
/// types on stable.
pub trait Buildable: Sized
where
    Self: guts::BoulderBase,
{
    /// A default choice of [`Builder`] for this type.
    type Builder: Builder<Result = Self>;
    /// Create a new default builder.
    ///
    /// Example
    /// ```rust
    /// use boulder::{Builder, Buildable};
    ///
    /// #[derive(Buildable)]
    /// struct Foo {
    ///    a: i32
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
    use super::Buildable;

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub use super::Builder as MiniBuilder;

    pub trait MiniBuildable<T>: Sized {
        type Builder: MiniBuilder<Result = Self>;
        fn mini_builder() -> Self::Builder;
    }

    impl<T> Buildable for T
    where
        T: BoulderBase,
        T: MiniBuildable<<T as BoulderBase>::Base>,
    {
        type Builder = <T as MiniBuildable<<T as BoulderBase>::Base>>::Builder;
        fn builder() -> Self::Builder {
            <T as MiniBuildable<<T as BoulderBase>::Base>>::mini_builder()
        }
    }

    pub trait BoulderBase {
        type Base;
    }

    impl<T> BoulderBase for Option<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Arc<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Rc<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for RefCell<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Cell<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }

    impl<T> BoulderBase for Mutex<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }
}
