pub trait GeneratableWithPersianRug<C>
where
    C: persian_rug::Context,
{
    type Generator: GeneratorWithPersianRug<C, Output = Self>;
    fn generator() -> Self::Generator;
}

pub trait GeneratorWithPersianRug<C>
where
    C: persian_rug::Context,
{
    type Output;
    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

/// An owning iterator for any generator.
///
/// It makes any generator into an infinite sequence. One reason not
/// to use this type is that it prevents modifying the generator
/// mid-sequence.
pub struct GeneratorWithPersianRugIterator<T, B>
where
    B: persian_rug::Mutator,
{
    gen: T,
    mutator: Option<B>,
}

impl<T, B> GeneratorWithPersianRugIterator<T, B>
where
    B: persian_rug::Mutator,
{
    pub fn new(generator: T, mutator: B) -> Self {
        Self {
            gen: generator,
            mutator: Some(mutator),
        }
    }

    pub fn into_inner(self) -> (T, B) {
        (self.gen, self.mutator.unwrap())
    }
}

impl<T, B, C> Iterator for GeneratorWithPersianRugIterator<T, B>
where
    C: persian_rug::Context,
    B: persian_rug::Mutator<Context = C>,
    T: GeneratorWithPersianRug<C>,
{
    type Item = <T as GeneratorWithPersianRug<C>>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        let (result, context) = self.gen.generate(self.mutator.take().unwrap());
        self.mutator = Some(context);
        Some(result)
    }
}

/// A non-owning iterator for any generator.
///
/// It makes any generator into an infinite sequence. Using this type
/// prevents modifying the generator mid-sequence, but permits
/// re-using it once the desired values have been extracted.
pub struct GeneratorWithPersianRugMutIterator<'a, B, T>
where
    B: persian_rug::Mutator,
{
    gen: &'a mut T,
    mutator: Option<B>,
}

impl<'a, B, T> GeneratorWithPersianRugMutIterator<'a, B, T>
where
    B: persian_rug::Mutator,
{
    pub fn new(generator: &'a mut T, mutator: B) -> Self {
        Self {
            gen: generator,
            mutator: Some(mutator),
        }
    }

    pub fn into_inner(self) -> B {
        self.mutator.unwrap()
    }
}

impl<'a, B, T, C> Iterator for GeneratorWithPersianRugMutIterator<'a, B, T>
where
    C: persian_rug::Context,
    T: GeneratorWithPersianRug<C>,
    B: persian_rug::Mutator<Context = C>,
{
    type Item = <T as GeneratorWithPersianRug<C>>::Output;
    fn next(&mut self) -> Option<Self::Item> {
        let (result, context) = self.gen.generate(self.mutator.take().unwrap());
        self.mutator = Some(context);
        Some(result)
    }
}

pub struct SequenceGeneratorWithPersianRug<S, T, V> {
    _marker: core::marker::PhantomData<V>,
    seq: S,
    elt: T,
}

impl<S, T, V> SequenceGeneratorWithPersianRug<S, T, V> {
    pub fn new(seq: S, elt: T) -> Self {
        Self {
            _marker: Default::default(),
            seq,
            elt,
        }
    }
}

impl<S, T, V, C, W> GeneratorWithPersianRug<C> for SequenceGeneratorWithPersianRug<S, T, V>
where
    C: persian_rug::Context,
    S: GeneratorWithPersianRug<C>,
    T: GeneratorWithPersianRug<C>,
    V: IntoIterator<Item = W> + FromIterator<W>,
    <T as GeneratorWithPersianRug<C>>::Output: Into<W>,
    <S as GeneratorWithPersianRug<C>>::Output: Into<usize>,
{
    type Output = V;

    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        let (count, mut context) = self.seq.generate(context);
        let mut storage = Vec::new();
        for _ in 0usize..count.into() {
            let (value, c) = self.elt.generate(context);
            context = c;
            storage.push(value.into());
        }
        (storage.into_iter().collect(), context)
    }
}

pub struct GeneratorWrapper<T> {
    gen: Box<dyn crate::Generator<Output = T>>,
}

impl<T> GeneratorWrapper<T> {
    pub fn new<U: crate::Generator<Output = T>>(value: U) -> Self {
        Self {
            gen: Box::new(value),
        }
    }
}

impl<C, T> GeneratorWithPersianRug<C> for GeneratorWrapper<T>
where
    C: persian_rug::Context,
    T: 'static,
{
    type Output = T;
    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        (self.gen.generate(), context)
    }
}

pub use boulder_derive::GeneratableWithPersianRug;

pub mod guts {
    use super::GeneratableWithPersianRug;

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub use super::GeneratorWithPersianRug as MiniGeneratorWithPersianRug;

    pub trait MiniGeneratableWithPersianRug<T, C>: Sized
    where
        C: persian_rug::Context,
    {
        type Generator: MiniGeneratorWithPersianRug<C, Output = Self>;
        fn mini_generator() -> Self::Generator;
    }

    impl<T, C> GeneratableWithPersianRug<C> for T
    where
        T: BoulderBase,
        T: MiniGeneratableWithPersianRug<<T as BoulderBase>::Base, C>,
        C: persian_rug::Context,
    {
        type Generator =
            <T as MiniGeneratableWithPersianRug<<T as BoulderBase>::Base, C>>::Generator;
        fn generator() -> Self::Generator {
            <T as MiniGeneratableWithPersianRug<<T as BoulderBase>::Base, C>>::mini_generator()
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

    impl<T> BoulderBase for persian_rug::Proxy<T>
    where
        T: BoulderBase,
    {
        type Base = <T as BoulderBase>::Base;
    }
}