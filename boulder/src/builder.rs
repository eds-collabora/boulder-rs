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
/// derive macro. Note that it cannot be directly implemented because
/// the library itself provides a blanket implementation from a
/// more complex underlying trait.
pub trait Buildable: Sized
where
    Self: guts::BoulderBase,
{
    type Builder: Builder<Result = Self>;
    /// Create a new builder for this type.
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
