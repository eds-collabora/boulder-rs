pub mod generators;

/// Something which can generate a sequence of objects of type
/// `Output`.
pub trait Generator
where
    Self: 'static,
{
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

/// A type that has an associated default `Generator`
///
/// The convenient way to implement this trait is via the
/// `Generatable` derive macro.
pub trait Generatable {
    type Generator: Generator<Output = Self>;
    /// Return this object's generator.
    ///
    /// Example
    /// ```rust
    /// use boulder::{Generatable, Generator, gen};
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
/// It makes any generator into an infinite sequence. One reason not
/// to use this type is that it prevents modifying the generator
/// mid-sequence.
pub struct GeneratorIterator<T> {
    gen: T,
}

impl<T> GeneratorIterator<T> {
    pub fn new(generator: T) -> Self {
        Self { gen: generator }
    }

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
/// It makes any generator into an infinite sequence. Using this type
/// prevents modifying the generator mid-sequence, but permits
/// re-using it once the desired values have been extracted.
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
