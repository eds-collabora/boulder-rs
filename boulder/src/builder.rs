use core::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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
pub trait Buildable {
    type Builder: Builder<Result = Self>;
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

impl<T> Buildable for Option<T>
where
    T: Buildable,
{
    type Builder = OptionBuilder<<T as Buildable>::Builder>;
    fn builder() -> Self::Builder {
        OptionBuilder(T::builder())
    }
}

pub struct OptionBuilder<T>(T);

impl<T> Deref for OptionBuilder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Builder for OptionBuilder<T>
where
    T: Builder,
{
    type Result = Option<<T as Builder>::Result>;
    fn build(self) -> Self::Result {
        Some(self.0.build())
    }
}

impl<T> Buildable for Rc<T>
where
    T: Buildable,
{
    type Builder = RcBuilder<<T as Buildable>::Builder>;
    fn builder() -> Self::Builder {
        RcBuilder(T::builder())
    }
}

pub struct RcBuilder<T>(T);

impl<T> Deref for RcBuilder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RcBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Builder for RcBuilder<T>
where
    T: Builder,
{
    type Result = Rc<<T as Builder>::Result>;
    fn build(self) -> Self::Result {
        Rc::new(self.0.build())
    }
}

impl<T> Buildable for Arc<T>
where
    T: Buildable,
{
    type Builder = ArcBuilder<<T as Buildable>::Builder>;
    fn builder() -> Self::Builder {
        ArcBuilder(T::builder())
    }
}

pub struct ArcBuilder<T>(T);

impl<T> Deref for ArcBuilder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ArcBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Builder for ArcBuilder<T>
where
    T: Builder,
{
    type Result = Arc<<T as Builder>::Result>;
    fn build(self) -> Self::Result {
        Arc::new(self.0.build())
    }
}

impl<T> Buildable for Mutex<T>
where
    T: Buildable,
{
    type Builder = MutexBuilder<<T as Buildable>::Builder>;
    fn builder() -> Self::Builder {
        MutexBuilder(T::builder())
    }
}

pub struct MutexBuilder<T>(T);

impl<T> Deref for MutexBuilder<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MutexBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Builder for MutexBuilder<T>
where
    T: Builder,
{
    type Result = Mutex<<T as Builder>::Result>;
    fn build(self) -> Self::Result {
        Mutex::new(self.0.build())
    }
}

pub use boulder_derive::Buildable;
