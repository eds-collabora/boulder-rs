use core::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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

impl<T> Generatable for Option<T>
where
    T: Generatable,
{
    type Generator = OptionGenerator<<T as Generatable>::Generator>;
    fn generator() -> Self::Generator {
        OptionGenerator(T::generator())
    }
}

pub struct OptionGenerator<T>(T);

impl<T> Deref for OptionGenerator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptionGenerator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Generator for OptionGenerator<T>
where
    T: Generator,
{
    type Output = Option<<T as Generator>::Output>;
    fn generate(&mut self) -> Self::Output {
        Some(self.0.generate())
    }
}

impl<T> IntoIterator for OptionGenerator<T>
where
    T: Generator,
{
    type Item = Option<<T as Generator>::Output>;
    type IntoIter = GeneratorIterator<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut OptionGenerator<T>
where
    T: Generator,
{
    type Item = Option<<T as Generator>::Output>;
    type IntoIter = GeneratorMutIterator<'a, OptionGenerator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> Generatable for Rc<T>
where
    T: Generatable,
{
    type Generator = RcGenerator<<T as Generatable>::Generator>;
    fn generator() -> Self::Generator {
        RcGenerator(T::generator())
    }
}

pub struct RcGenerator<T>(T);

impl<T> Deref for RcGenerator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RcGenerator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Generator for RcGenerator<T>
where
    T: Generator,
{
    type Output = Rc<<T as Generator>::Output>;
    fn generate(&mut self) -> Self::Output {
        Rc::new(self.0.generate())
    }
}

impl<T> IntoIterator for RcGenerator<T>
where
    T: Generator,
{
    type Item = Rc<<T as Generator>::Output>;
    type IntoIter = GeneratorIterator<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut RcGenerator<T>
where
    T: Generator,
{
    type Item = Rc<<T as Generator>::Output>;
    type IntoIter = GeneratorMutIterator<'a, RcGenerator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> Generatable for Arc<T>
where
    T: Generatable,
{
    type Generator = ArcGenerator<<T as Generatable>::Generator>;
    fn generator() -> Self::Generator {
        ArcGenerator(T::generator())
    }
}

pub struct ArcGenerator<T>(T);

impl<T> Deref for ArcGenerator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ArcGenerator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Generator for ArcGenerator<T>
where
    T: Generator,
{
    type Output = Arc<<T as Generator>::Output>;
    fn generate(&mut self) -> Self::Output {
        Arc::new(self.0.generate())
    }
}

impl<T> IntoIterator for ArcGenerator<T>
where
    T: Generator,
{
    type Item = Arc<<T as Generator>::Output>;
    type IntoIter = GeneratorIterator<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut ArcGenerator<T>
where
    T: Generator,
{
    type Item = Arc<<T as Generator>::Output>;
    type IntoIter = GeneratorMutIterator<'a, ArcGenerator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<T> Generatable for Mutex<T>
where
    T: Generatable,
{
    type Generator = MutexGenerator<<T as Generatable>::Generator>;
    fn generator() -> Self::Generator {
        MutexGenerator(T::generator())
    }
}

pub struct MutexGenerator<T>(T);

impl<T> Deref for MutexGenerator<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MutexGenerator<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Generator for MutexGenerator<T>
where
    T: Generator,
{
    type Output = Mutex<<T as Generator>::Output>;
    fn generate(&mut self) -> Self::Output {
        Mutex::new(self.0.generate())
    }
}

impl<T> IntoIterator for MutexGenerator<T>
where
    T: Generator,
{
    type Item = Mutex<<T as Generator>::Output>;
    type IntoIter = GeneratorIterator<Self>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a, T> IntoIterator for &'a mut MutexGenerator<T>
where
    T: Generator,
{
    type Item = Mutex<<T as Generator>::Output>;
    type IntoIter = GeneratorMutIterator<'a, MutexGenerator<T>>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub use boulder_derive::Generatable;

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
