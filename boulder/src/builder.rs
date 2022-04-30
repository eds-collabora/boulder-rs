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
pub trait Buildable: Sized
where
    Self: guts::BuilderBase,
//    Self: guts::MiniBuildable<<Self as guts::BuilderBase>::Base>,
{
    type Builder: guts::MiniBuilder<Self>;
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

    pub trait MiniBuildable<T>: Sized {
        type Builder: MiniBuilder<Self>;
        fn mini_builder() -> Self::Builder;
    }
    
    pub trait MiniBuilder<T>: Sized {
        fn build(self) -> T;
    }

    pub trait BuilderBase {
        type Base;
    }
    
    impl<T> Buildable for T
    where
        T: BuilderBase,
        T: MiniBuildable<<T as BuilderBase>::Base>,
     {
        type Builder = <T as MiniBuildable<<T as BuilderBase>::Base>>::Builder;
        fn builder() -> Self::Builder {
            <T as MiniBuildable<<T as BuilderBase>::Base>>::mini_builder()
        }
    }

    impl<T> BuilderBase for Option<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for Arc<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for Rc<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for Box<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for RefCell<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for Cell<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }

    impl<T> BuilderBase for Mutex<T>
    where
        T: BuilderBase
    {
        type Base = <T as BuilderBase>::Base;
    }
}    
