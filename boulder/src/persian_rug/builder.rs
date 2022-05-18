/// A type that has an associated default [`BuilderWithPersianRug`]
///
/// [persian_rug] is a crate that provides arena based management for
/// objects of different types. The natural reason to derive this
/// trait over [`Buildable`] is when your type belongs to a
/// [`persian_rug::Context`].
///
/// This trait is implemented via the
/// [`macro@BuildableWithPersianRug`] derive macro.  It cannot be
/// directly implemented because the library itself provides a blanket
/// implementation from a more complex underlying trait
/// `MiniBuildable`, which is not currently documented.
///
/// This restriction may be removed in a future version; much of the
/// complexity in this module stems from lacking generic associated
/// types on stable.
pub trait BuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    type Builder: BuilderWithPersianRug<C, Result = Self>;
    fn builder() -> Self::Builder;
}

/// Something which can create a default object of some type.
///
/// [persian_rug] is a crate that provides arena based management for
/// objects of different types. The natural reason to derive this
/// trait over [`Builder`] is when your target type belongs to a
/// [`persian_rug::Context`].
///
/// The only required function in this trait is
/// [`build`](BuilderWithPersianRug::build) which creates an object,
/// using a [`Context`](persian_rug::Context), consuming the
/// builder. Most builders will allow customisation of the produced
/// object in some way.
///
/// An object implementing this trait will be automatically created
/// for you as part of the [`macro@BuildableWithPersianRug`] derive
/// macro. That builder will have a method for each field of the
/// result type, to customise its value, and will produce a default
/// value for every field which is not customised.
pub trait BuilderWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    /// The output type.
    type Result;
    /// Create the final object. The `context` argument is a
    /// [`persian_rug::Mutator`]. It is not a mutable reference to a
    /// mutator, so it cannot be reborrowed, instead it must be
    /// returned for re-use, without being consumed.
    ///
    /// Example
    /// ```rust
    /// use boulder::BuilderWithPersianRug;
    /// use persian_rug::{contextual, persian_rug, Mutator};
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
    /// struct FooBuilder {
    ///    a: i32
    /// }
    ///
    /// impl BuilderWithPersianRug<State> for FooBuilder {
    ///    type Result = Foo;
    ///    fn build<'b, B>(self, mutator: B) -> (Self::Result, B)
    ///    where
    ///       B: 'b + Mutator<Context = State>
    ///    {
    ///       (Foo { a: self.a }, mutator)
    ///    }
    /// }
    ///
    /// let mut s = State(Default::default());
    /// let b = FooBuilder { a: 3 };
    /// let (f, _) = b.build(&mut s);
    /// assert_eq!(f.a, 3);
    /// ```
    fn build<'b, B>(self, context: B) -> (Self::Result, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

pub use boulder_derive::BuildableWithPersianRug;

#[doc(hidden)]
pub mod guts {
    use super::BuildableWithPersianRug;

    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    pub use super::BuilderWithPersianRug as MiniBuilderWithPersianRug;

    pub trait MiniBuildableWithPersianRug<T, C>: Sized
    where
        C: persian_rug::Context,
    {
        type Builder: MiniBuilderWithPersianRug<C, Result = Self>;
        fn mini_builder() -> Self::Builder;
    }

    impl<T, C> BuildableWithPersianRug<C> for T
    where
        T: BoulderBase,
        T: MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>,
        C: persian_rug::Context,
    {
        type Builder = <T as MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>>::Builder;
        fn builder() -> Self::Builder {
            <T as MiniBuildableWithPersianRug<<T as BoulderBase>::Base, C>>::mini_builder()
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
