/// A type that has an associated default [`GeneratorWithPersianRug`]
///
/// This trait is implemented via the
/// [`macro@GeneratableWithPersianRug`] derive macro. It cannot be
/// directly implemented because the library itself provides a blanket
/// implementation from a more complex underlying trait
/// `MiniGeneratableWithPersianRug`, which is not currently
/// documented.
///
/// This restriction may be removed in a future version; much of the
/// complexity in this module stems from lacking generic associated
/// types on stable.
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub trait GeneratableWithPersianRug<C>
where
    C: persian_rug::Context,
{
    /// A default choice of [`GeneratorWithPersianRug`] for this type.
    type Generator: GeneratorWithPersianRug<C, Output = Self>;
    /// Return this object's generator.
    ///
    /// Example
    /// ```rust
    /// use boulder::{GeneratableWithPersianRug, GeneratorWithPersianRug};
    /// use persian_rug::{contextual, persian_rug, Context, Mutator};
    ///
    /// #[contextual(State)]
    /// struct Foo {
    ///    a: i32
    /// }
    ///
    /// #[persian_rug]
    /// struct State (
    ///   #[table] Foo,
    /// );
    ///
    /// struct FooGenerator {
    ///   a: i32
    /// };
    ///
    /// impl GeneratorWithPersianRug<State> for FooGenerator {
    ///   type Output = Foo;
    ///   fn generate<'b, B>(&mut self, context: B) -> (Foo, B)
    ///   where
    ///     B: 'b + Mutator<Context = State>
    ///   {
    ///     self.a += 1;
    ///     (Foo { a: self.a }, context)
    ///   }
    /// }
    ///
    /// impl GeneratableWithPersianRug<State> for Foo {
    ///   type Generator = FooGenerator;
    ///   fn generator() -> Self::Generator {
    ///     FooGenerator { a: 0 }
    ///   }
    /// }
    ///
    /// let mut s = State(Default::default());
    /// let mut g = Foo::generator();
    ///
    /// let (f1, _) = g.generate(&mut s);
    /// assert_eq!(f1.a, 1);
    /// let (f2, _) = g.generate(&mut s);
    /// assert_eq!(f2.a, 2);
    /// ```
    fn generator() -> Self::Generator;
}

/// Something which can generate a sequence of objects of some
/// [`persian_rug::Contextual`] type.
///
/// The only required function in this trait is
/// ['generate'](GeneratorWithPersianRug::generate) which creates a
/// new object, mutating the generator as a byproduct, using a
/// [`Context`](persian_rug::Context). Most generators will allow
/// customisation of the sequence of produced objects in some way.
///
/// An object implementing this trait will be automatically created
/// for you as part of the [`macro@GeneratableWithPersianRug`] derive
/// macro. That generator will have a method for each field of the
/// result type, to allow you to set a generator for the field. It
/// will produce a default sequence (as configured by the attributes
/// placed on the type) for every field that is not customised.
///
/// Note that the generator produced by this macro changes type as its
/// default sequences are altered; this is mostly transparent. This is
/// required because this trait is not object safe, and therefore
/// there is no overarching type that can represent any generator for
/// a given field.
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub trait GeneratorWithPersianRug<C>
where
    C: persian_rug::Context,
{
    /// The output type.
    type Output;
    /// Make a new object.
    ///
    /// Example
    /// ```rust
    /// use boulder::GeneratorWithPersianRug;
    /// use persian_rug::{contextual, persian_rug, Context, Mutator};
    ///
    /// #[contextual(State)]
    /// struct Foo {
    ///    a: i32
    /// }
    ///
    /// #[persian_rug]
    /// struct State (
    ///   #[table] Foo,
    /// );
    ///
    /// struct FooGenerator {
    ///   next: i32
    /// };
    ///
    /// impl GeneratorWithPersianRug<State> for FooGenerator {
    ///   type Output = Foo;
    ///   fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    ///   where
    ///     B: 'b + Mutator<Context = State>
    ///   {
    ///     let result = self.next;
    ///     self.next = self.next + 1;
    ///     (Foo { a: result }, context)
    ///   }
    /// }
    ///
    /// let mut s = State(Default::default());
    /// let mut g = FooGenerator { next: 6 };
    /// let (f1, _) = g.generate(&mut s);
    /// assert_eq!(f1.a, 6);
    /// let (f2, _) = g.generate(&mut s);
    /// assert_eq!(f2.a, 7);
    /// ```
    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

/// An owning iterator for any generator.
///
/// It makes any generator into an infinite sequence. One reason not
/// to use this type is that it prevents modifying the generator
/// mid-sequence.
///
/// Example:
/// ```rust
/// use boulder::{GeneratableWithPersianRug, GeneratorWithPersianRug, GeneratorWithPersianRugIterator, Inc};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// #[derive(GeneratableWithPersianRug)]
/// #[boulder(persian_rug(context=Rug))]
/// struct Foo {
///    #[boulder(generator=Inc(1))]
///    a: i32
/// }
///
/// #[persian_rug]
/// struct Rug (
///   #[table] Foo,
/// );
///
/// let mut r = Rug(Default::default());
/// let g = Proxy::<Foo>::generator();
/// let mut iter = GeneratorWithPersianRugIterator::new(g, &mut r);
/// let f1 = iter.next().unwrap();
/// let f2 = iter.next().unwrap();
/// let (mut g, _) = iter.into_inner();
/// assert_eq!(r.get(&f1).a, 1);
/// assert_eq!(r.get(&f2).a, 2);
/// let (f3, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f3).a, 3);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
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
    /// Create a new iterator from a generator and a mutator
    pub fn new(generator: T, mutator: B) -> Self {
        Self {
            gen: generator,
            mutator: Some(mutator),
        }
    }

    /// Destroy the iterator, recovering the generator and mutator inside.
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
///
/// Example:
/// ```rust
/// use boulder::{GeneratableWithPersianRug, GeneratorWithPersianRug, GeneratorWithPersianRugMutIterator, Inc};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// #[derive(GeneratableWithPersianRug)]
/// #[boulder(persian_rug(context=Rug))]
/// struct Foo {
///    #[boulder(generator=Inc(1))]
///    a: i32
/// }
///
/// #[persian_rug]
/// struct Rug (
///   #[table] Foo,
/// );
///
/// let mut r = Rug(Default::default());
/// let mut g = Proxy::<Foo>::generator();
/// let mut iter = GeneratorWithPersianRugMutIterator::new(&mut g, &mut r);
/// let f1 = iter.next().unwrap();
/// let f2 = iter.next().unwrap();
/// let _ = iter.into_inner();
/// assert_eq!(r.get(&f1).a, 1);
/// assert_eq!(r.get(&f2).a, 2);
/// let (f3, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f3).a, 3);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
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

/// Collections drawn from an underlying generator.
///
/// This wraps an underlying generator that produces items, which this
/// then gathers into collections. A separate
/// [`GeneratorWithPersianRug`] is used to determine how many elements
/// are present in each successively yielded collection.
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct SequenceGeneratorWithPersianRug<S, T, V> {
    _marker: core::marker::PhantomData<V>,
    seq: S,
    elt: T,
}

impl<S, T, V> SequenceGeneratorWithPersianRug<S, T, V> {
    /// Create a new instance
    ///
    /// - `seq` is a [`GeneratorWithPersianRug`] that produces
    ///   something that can be converted into `usize`, which will
    ///   be the number of elements in each yielded collection.
    /// - `elt` is a [`GeneratorWithPersianRug`] that produces
    ///   something that can be convered into the the container
    ///   element type.
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

/// Convert a [`Generator`](crate::Generator) into a [`GeneratorWithPersianRug`].
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct GeneratorToGeneratorWithPersianRugWrapper<T> {
    gen: Box<dyn crate::Generator<Output = T>>,
}

impl<T> GeneratorToGeneratorWithPersianRugWrapper<T> {
    pub fn new<U: crate::Generator<Output = T>>(value: U) -> Self {
        Self {
            gen: Box::new(value),
        }
    }
}

impl<C, T> GeneratorWithPersianRug<C> for GeneratorToGeneratorWithPersianRugWrapper<T>
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

/// Cycle through the existing [`Proxy<T>`](persian_rug::Proxy) items
/// in the context.
///
/// Each proxy will be yielded in turn, in insertion order, and then
/// the sequence will repeat.  It is safe to insert more values to
/// yield, and they will occur in their natural position when they are
/// reached.
///
/// [`generate`](RepeatFromPersianRug::generate) routine will panic if
/// there are no available items.
///
/// Example:
/// ```rust
/// use boulder::{GeneratorWithPersianRug, RepeatFromPersianRug};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// struct Foo {
///   a: i32
/// }
///
/// #[persian_rug]
/// struct Rug(#[table] Foo);
///
/// let mut r = Rug(Default::default());
/// r.add(Foo { a: 0 });
/// r.add(Foo { a: 1 });
///
/// let mut g = RepeatFromPersianRug::<Foo>::new();
/// let (f1, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f1).a, 0);
/// let (f2, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f2).a, 1);
/// let (f3, _) = g.generate(&mut r);
/// assert_eq!(f1, f3);
///
/// r.add(Foo { a: 2} );
/// let (f4, _) = g.generate(&mut r);
/// assert_eq!(f2, f4);
/// let (f5, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f5).a, 2);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct RepeatFromPersianRug<T> {
    _marker: core::marker::PhantomData<T>,
    index: u64,
}

impl<T> Default for RepeatFromPersianRug<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            index: 0u64,
        }
    }
}

impl<T> RepeatFromPersianRug<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> GeneratorWithPersianRug<C> for RepeatFromPersianRug<T> {
    type Output = persian_rug::Proxy<T>;

    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        let proxies = context.get_proxy_iter().collect::<Vec<_>>();
        if proxies.is_empty() {
            panic!(
                "Failing to generate from rug {} for type {}: no available objects",
                std::any::type_name::<C>(),
                std::any::type_name::<T>()
            );
        }
        let res = proxies[self.index as usize];
        self.index = (self.index + 1) % (proxies.len() as u64);
        (*res, context)
    }
}

/// Cycle through the existing [`Proxy<T>`](persian_rug::Proxy) items
/// in the context.
///
/// Each proxy will be yielded in turn, in insertion order, and then
/// the sequence will repeat.  It is safe to insert more values to
/// yield, and they will occur in their natural position when they are
/// reached.
///
/// If there are no available items,
/// [`generate`](TryRepeatFromPersianRug::generate) returns `None`.
///
/// Example:
/// ```rust
/// use boulder::{GeneratorWithPersianRug, TryRepeatFromPersianRug};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// struct Foo {
///   a: i32
/// }
///
/// #[persian_rug]
/// struct Rug(#[table] Foo);
///
/// let mut r = Rug(Default::default());
///
/// let mut g = TryRepeatFromPersianRug::<Foo>::new();
///
/// let (f0, _) = g.generate(&mut r);
/// assert!(f0.is_none());
///
/// r.add(Foo { a: 0 });
/// r.add(Foo { a: 1 });
///
/// let (f1, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f1.unwrap()).a, 0);
/// let (f2, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f2.unwrap()).a, 1);
/// let (f3, _) = g.generate(&mut r);
/// assert_eq!(f1, f3);
///
/// r.add(Foo { a: 2} );
/// let (f4, _) = g.generate(&mut r);
/// assert_eq!(f2, f4);
/// let (f5, _) = g.generate(&mut r);
/// assert_eq!(r.get(&f5.unwrap()).a, 2);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct TryRepeatFromPersianRug<T> {
    _marker: core::marker::PhantomData<T>,
    index: u64,
}

impl<T> Default for TryRepeatFromPersianRug<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            index: 0u64,
        }
    }
}

impl<T> TryRepeatFromPersianRug<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> GeneratorWithPersianRug<C> for TryRepeatFromPersianRug<T> {
    type Output = Option<persian_rug::Proxy<T>>;

    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        let proxies = context.get_proxy_iter().collect::<Vec<_>>();
        if proxies.is_empty() {
            (None, context)
        } else {
            let res = proxies[self.index as usize];
            self.index = (self.index + 1) % (proxies.len() as u64);
            (Some(*res), context)
        }
    }
}

/// Produce subsets of the existing [`Proxy<T>`](persian_rug::Proxy)
/// items in the context.
///
/// The yield pattern is the same as the regular
/// [`Subsets`](crate::Subsets) generator. It is safe to add more
/// items, and they will occur in their natural place in future items
/// yielded by the generator.
///
/// Example:
/// ```rust
/// use boulder::{GeneratorWithPersianRug, SubsetsFromPersianRug};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// struct Foo {
///   a: i32
/// }
///
/// #[persian_rug]
/// struct Rug(#[table] Foo);
///
/// let mut r = Rug(Default::default());
/// let f1 = r.add(Foo { a: 0 });
/// let f2 = r.add(Foo { a: 1 });
///
/// let mut g = SubsetsFromPersianRug::<Foo>::new();
/// let (s1, _) = g.generate(&mut r);
/// assert_eq!(s1, Vec::new());
/// let (s2, _) = g.generate(&mut r);
/// assert_eq!(s2, vec![ f1 ]);
/// let (s3, _) = g.generate(&mut r);
/// assert_eq!(s3, vec![ f2 ]);
/// let (s4, _) = g.generate(&mut r);
/// assert_eq!(s4, vec![ f1, f2 ]);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct SubsetsFromPersianRug<T> {
    _marker: core::marker::PhantomData<T>,
    index: u64,
}

impl<T> Default for SubsetsFromPersianRug<T> {
    fn default() -> Self {
        Self {
            _marker: Default::default(),
            index: 0u64,
        }
    }
}

impl<T> SubsetsFromPersianRug<T> {
    pub fn new() -> Self {
        Default::default()
    }
}

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> GeneratorWithPersianRug<C> for SubsetsFromPersianRug<T> {
    type Output = Vec<persian_rug::Proxy<T>>;

    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        let mut res = Vec::new();
        let proxies = context.get_proxy_iter().collect::<Vec<_>>();
        for i in 0..std::cmp::min(64, proxies.len()) {
            if (self.index & (1u64 << i)) != 0 {
                res.push(*proxies[i as usize]);
            }
        }
        self.index += 1;
        (res, context)
    }
}

/// Produce collections of the existing
/// [`Proxy<T>`](persian_rug::Proxy) items in the context.
///
/// It is safe to add more items, and they will occur in their natural
/// place in future items yielded by the generator. The generator
/// argument controls the maximum number of items in each yielded
/// collection. [`SampleFromPersianRug`] will always return this
/// maximum if it is possible to do so without duplicating elements.
///
/// Example:
/// ```rust
/// use boulder::{GeneratorWithPersianRug, Inc, SampleFromPersianRug};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// struct Foo {
///   a: i32
/// }
///
/// #[persian_rug]
/// struct Rug(#[table] Foo);
///
/// let mut r = Rug(Default::default());
/// let f1 = r.add(Foo { a: 0 });
/// let f2 = r.add(Foo { a: 1 });
///
/// let mut g = SampleFromPersianRug::<Foo, _, Vec<_>>::new(Inc(0));
/// let (s1, _) = g.generate(&mut r);
/// assert_eq!(s1, Vec::new());
/// let (s2, _) = g.generate(&mut r);
/// assert_eq!(s2, vec![ f1 ]);
/// let (s3, _) = g.generate(&mut r);
/// assert_eq!(s3, vec![ f2, f1 ]);
/// let (s4, _) = g.generate(&mut r);
/// assert_eq!(s4, vec![ f2, f1 ]);
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub struct SampleFromPersianRug<T, U, V> {
    index: u64,
    count: U,
    _type_marker: core::marker::PhantomData<T>,
    _result_marker: core::marker::PhantomData<V>,
}

#[persian_rug::constraints(context = C, access(T))]
impl<C, T, U, V> SampleFromPersianRug<T, U, V>
where
    U: GeneratorWithPersianRug<C, Output = usize>,
    V: FromIterator<persian_rug::Proxy<T>> + 'static,
{
    pub fn new(count: U) -> Self {
        Self {
            index: 0u64,
            count,
            _type_marker: Default::default(),
            _result_marker: Default::default(),
        }
    }
}

#[persian_rug::constraints(context = C, access(T))]
impl<C, T, U, V> GeneratorWithPersianRug<C> for SampleFromPersianRug<T, U, V>
where
    U: GeneratorWithPersianRug<C, Output = usize>,
    V: FromIterator<persian_rug::Proxy<T>> + 'static,
{
    type Output = V;
    fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>,
    {
        let (count, context) = self.count.generate(context);

        let mut res = Vec::new();
        let proxies = context.get_proxy_iter().collect::<Vec<_>>();
        for _ in 0..std::cmp::min(proxies.len(), count) {
            res.push(*proxies[self.index as usize]);
            self.index = (self.index + 1) % (proxies.len() as u64);
        }
        (res.into_iter().collect(), context)
    }
}

pub use boulder_derive::GeneratableWithPersianRug;

#[doc(hidden)]
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

mod gen {
    use super::GeneratorWithPersianRug;
    use crate::{Const, Cycle, Inc, Repeat, Sample, Some, Subsets, Time};
    use num::One;

    impl<C, T> GeneratorWithPersianRug<C> for Const<T>
    where
        T: Clone + 'static,
        C: persian_rug::Context,
    {
        type Output = T;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            (self.0.clone(), context)
        }
    }

    impl<C, T> GeneratorWithPersianRug<C> for Inc<T>
    where
        T: core::ops::AddAssign<T> + One + Clone + 'static,
        C: persian_rug::Context,
    {
        type Output = T;
        fn generate<'b, B>(&mut self, context: B) -> (T, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let res = self.0.clone();
            self.0 += T::one();
            (res, context)
        }
    }

    impl<C, S, T> GeneratorWithPersianRug<C> for Cycle<S>
    where
        S: Iterator<Item = T> + Clone + 'static,
        C: persian_rug::Context,
    {
        type Output = T;
        fn generate<'b, B>(&mut self, context: B) -> (T, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            (self.0.next().unwrap(), context)
        }
    }

    impl<C, T> GeneratorWithPersianRug<C> for Some<T>
    where
        T: GeneratorWithPersianRug<C>,
        C: persian_rug::Context,
    {
        type Output = Option<<T as GeneratorWithPersianRug<C>>::Output>;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let (value, context) = self.0.generate(context);
            (Option::Some(value), context)
        }
    }

    impl<C, F, T> GeneratorWithPersianRug<C> for F
    where
        F: FnMut() -> T + 'static,
        C: persian_rug::Context,
    {
        type Output = T;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            (self(), context)
        }
    }

    impl<C, T, U, V, X> GeneratorWithPersianRug<C> for Sample<T, U, V>
    where
        T: GeneratorWithPersianRug<C, Output = X>,
        U: GeneratorWithPersianRug<C, Output = usize>,
        V: FromIterator<X> + 'static,
        C: persian_rug::Context,
    {
        type Output = V;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let (count, context) = self.count.generate(context);
            let mut iter = super::GeneratorWithPersianRugMutIterator::new(&mut self.value, context);
            let mut res = Vec::new();
            for _ in 0..count {
                res.push(iter.next().unwrap());
            }
            (res.into_iter().collect(), iter.into_inner())
        }
    }

    impl<C, T> GeneratorWithPersianRug<C> for Time<T>
    where
        T: chrono::TimeZone + 'static,
        C: persian_rug::Context,
    {
        type Output = chrono::DateTime<T>;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let res = self.instant.clone();
            self.instant = self.instant.clone() + self.step;
            (res, context)
        }
    }

    impl<C, T> GeneratorWithPersianRug<C> for Subsets<T>
    where
        T: Clone + 'static,
        C: persian_rug::Context,
    {
        type Output = Vec<T>;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let mut v = Vec::new();
            for i in 0..std::cmp::min(std::mem::size_of::<usize>() * 8, self.base.len()) {
                if self.index & (1usize << i) != 0 {
                    v.push(self.base[i].clone());
                }
            }
            self.index += 1;
            (v, context)
        }
    }

    impl<C, T> GeneratorWithPersianRug<C> for Repeat<T>
    where
        T: Clone + 'static,
        C: persian_rug::Context,
    {
        type Output = T;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let res = self.base[self.index % self.base.len()].clone();
            self.index = (self.index + 1usize) % self.base.len();
            (res, context)
        }
    }
}
