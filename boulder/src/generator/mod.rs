pub mod generators;

/// Something which can generate a sequence of objects of some type.
///
/// The only required function in this trait is
/// ['generate'](Generator::generate) which creates a new object,
/// mutating the generator as a byproduct. Most generators will allow
/// customisation of the sequence of produced objects in some way.
///
/// An object implementing this trait will be automatically created
/// for you as part of the [`macro@Generatable`] derive macro. That
/// generator will have a method for each field of the result type, to
/// allow you to set a generator for the field. It will produce a
/// default sequence (as configured by the attributes placed on the
/// type) for every field that is not customised.
pub trait Generator
where
    Self: 'static,
{
    /// The output type.
    type Output;
    /// Make a new object.
    ///
    /// Example
    /// ```rust
    /// use boulder::Generator;
    /// struct MyGenerator {
    ///   next: i32
    /// };
    ///
    /// impl Generator for MyGenerator {
    ///   type Output = i32;
    ///   fn generate(&mut self) -> Self::Output {
    ///     let result = self.next;
    ///     self.next = self.next + 1;
    ///     result
    ///   }
    /// }
    ///
    /// let mut g = MyGenerator { next: 6 };
    /// assert_eq!(g.generate(), 6);
    /// assert_eq!(g.generate(), 7);
    /// ```
    fn generate(&mut self) -> Self::Output;
}

/// A type that has an associated default [`Generator`]
///
/// This trait is implemented via the [`macro@Generatable`] derive
/// macro. It cannot be directly implemented because the library
/// itself provides a blanket implementation from a more complex
/// underlying trait `MiniGeneratable`, which is not currently
/// documented.
///
/// This restriction may be removed in a future version; much of the
/// complexity in this module stems from lacking generic associated
/// types on stable.
pub trait Generatable {
    /// A default choice of [`Generator`] for this type.
    type Generator: Generator<Output = Self>;
    /// Return this object's generator.
    ///
    /// Example
    /// ```rust
    /// use boulder::{Generatable, Generator};
    ///
    /// struct FooGenerator {
    ///   a: i32
    /// };
    ///
    /// impl Generator for FooGenerator {
    ///   type Output = Foo;
    ///   fn generate(&mut self) -> Foo {
    ///     self.a += 1;
    ///     Foo { a: self.a }
    ///   }
    /// }
    ///
    /// struct Foo {
    ///   a: i32
    /// };
    ///
    /// impl Generatable for Foo {
    ///   type Generator = FooGenerator;
    ///   fn generator() -> Self::Generator {
    ///     FooGenerator { a: 0 }
    ///   }
    /// }
    ///
    /// let mut g = Foo::generator();
    /// assert_eq!(g.generate().a, 1);
    /// assert_eq!(g.generate().a, 2);
    /// ```
    fn generator() -> Self::Generator;
}

/// An owning iterator for any generator.
///
/// This type converts any generator into an iterator over an infinite
/// sequence.
pub struct GeneratorIterator<T> {
    gen: T,
}

impl<T> GeneratorIterator<T> {
    /// Create a new iterator from the given generator.
    pub fn new(generator: T) -> Self {
        Self { gen: generator }
    }

    /// Retrieve the original generator, destroying the iterator.
    pub fn into_inner(self) -> T {
        self.gen
    }
}

impl<T, U> Iterator for GeneratorIterator<T>
where
    T: Generator<Output = U> + 'static,
{
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gen.generate())
    }
}

/// A non-owning iterator for any generator.
///
/// This type is an iterator over the infinite sequence of values
/// produced by a generator. This type holds a mutable reference to
/// the generator, but does not take ownership.
pub struct GeneratorMutIterator<'a, T> {
    gen: &'a mut T,
}

impl<'a, T> GeneratorMutIterator<'a, T> {
    pub fn new(generator: &'a mut T) -> Self {
        Self { gen: generator }
    }
}

impl<'a, T, U> Iterator for GeneratorMutIterator<'a, T>
where
    T: Generator<Output = U>,
{
    type Item = U;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.gen.generate())
    }
}

pub use boulder_derive::Generatable;

#[doc(hidden)]
pub mod guts {
    use super::Generatable;
    pub use super::Generator as MiniGenerator;

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub trait MiniGeneratable<T>: Sized {
        type Generator: MiniGenerator<Output = Self>;
        fn mini_generator() -> Self::Generator;
    }

    impl<T> Generatable for T
    where
        T: BoulderBase,
        T: MiniGeneratable<<T as BoulderBase>::Base>,
    {
        type Generator = <T as MiniGeneratable<<T as BoulderBase>::Base>>::Generator;
        fn generator() -> Self::Generator {
            <T as MiniGeneratable<<T as BoulderBase>::Base>>::mini_generator()
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
