mod attributes;
mod builder;
mod generator;
mod repeat;
mod string;

use proc_macro::{self, TokenStream};

/// Derive the `Buildable` trait for a type, creating a suitable
/// `Builder`.
///
/// This is only implemented for structs with named fields; there is
/// no implementation for either enums or structs with unnamed fields.
/// All fields will be default constructed in the absence of other
/// instructions. You can customise the construction process for your
/// type by using the `boulder` attribute on its fields, as follows:
///
/// - `#[boulder(default=Foo)]` The default value for this field is
///   `Foo`, where an arbitrary well-formed Rust expression can be
///   used in place of `Foo`.
///
/// - `#[boulder(buildable)]` The type for this field implements
///   `Buildable` itself, so new values should be constructed using
///   `T::builder().build()`.
///
/// - `#[boulder(buildable(a=5, b=10))]` The type for this field implements
///   `Buildable`, and new instances should be customised from the
///    default by setting `a=5` and `b=10` where `a` and `b` are
///    member names, and `5` and `10` can be replaced by arbitrary
///    well-formed Rust expressions.
///
/// - `#[boulder(sequence=3)]` This field is assumed to be a
///   collection type (a type which can be the target of
///   `collect()`). Generate 3 items and initialize a new collection
///   from them, where `3` can be replaced by an arbitrary well-formed
///   Rust expression. The generation mechanism will be taken from any
///   generator specification (`generator` or `generatable`) if one is
///   given; otherwise it will be from the builder specification
///   (`default` or `buildable`, as described above) if one is given;
///   otherwise the items will be default initialized.
///
/// Example:
/// ```rust
/// use boulder::{Buildable, Generatable, Builder};
///
/// #[derive(Buildable)]
/// struct Foo {
///   // This field will be default-initialized (a=0)
///   a: i32,
///   // This field will be initialized from the expression (b=5)
///   #[boulder(default=6-1)]
///   b: i32,
/// }
///
/// #[derive(Buildable)]
/// struct Bar {
///   // This field will be initialized as Foo::builder().build()
///   #[boulder(buildable)]
///   f1: Foo,
///   // This field will be initialized as Foo::builder().a(1).build()
///   #[boulder(buildable(a=1))]
///   f2: Foo,
///   // This field will be initialized with two copies of Foo::builder().build()
///   #[boulder(buildable, sequence=2)]
///   f3: Vec<Foo>,
///   // This field will be initialized with one zero
///   #[boulder(sequence=1)]
///   ary: Vec<i32>,
/// }
///
/// let foo = Foo::builder().build();
/// assert_eq!(foo.a, 0);
/// assert_eq!(foo.b, 5);
/// let bar = Bar::builder().build();
/// assert_eq!(bar.f1.a, 0);
/// assert_eq!(bar.f1.b, 5);
/// assert_eq!(bar.f2.a, 1);
/// assert_eq!(bar.f2.b, 5);
/// assert_eq!(bar.f3.len(), 2);
/// assert_eq!(bar.f3[0].a, 0);
/// assert_eq!(bar.f3[0].b, 5);
/// assert_eq!(bar.f3[1].a, 0);
/// assert_eq!(bar.f3[1].b, 5);
/// assert_eq!(bar.ary.len(), 1);
/// assert_eq!(bar.ary[0], 0);
/// ```
#[proc_macro_derive(Buildable, attributes(boulder))]
pub fn builder(input: TokenStream) -> TokenStream {
    builder::derive_buildable(syn::parse_macro_input!(input)).into()
}

/// Derive the `Generatable` trait for a type, creating a suitable
/// `Generator`.
///
/// This is only implemented for structs with named fields; there is
/// no implementation for either enums or structs with unnamed fields.
/// All fields will be default constructed (i.e. `Default::default()`)
/// in the absence of other instructions. You can customise the
/// construction process for your type by using the `boulder`
/// attribute as follows:
///
/// - `#[boulder(generator=Inc(0))]` Each successive instance produced
///   by this generator should, by default, have a value one higher
///   for this field than the previous one. The first generated value for
///   this field should be 0. `Inc(0)` can be replaced by an arbitrary
///   expression which evaluates to a `Generator`.
///
/// - `#[boulder(generatable)]` The type for this field implements
///   `Generatable`, so new instances of the containing type should
///   have values for this field taken from the default sequence for
///   the field type.
///
/// - `#[boulder(generatable(a=Inc(3i32)))]` The type for this field
///   implements `Generatable`, and the generator for values for this
///   field should be customised, such that the nested field `a` uses
///   the generator `Inc(3i32)`. `Inc(3i32)` can be replaced by an
///   arbitrary expression which evaluates to a `Generator` instance.
///
/// - `#[boulder(sequence_generator=Repeat(2usize, 3usize))]` This
///   field is assumed to be a collection type (a type which can be
///   the target of `collect()`). Successive instances should have
///   alternately 2 and 3 items. The items will be default
///   initialized. This tag stacks with `generatable`, `buildable` and
///   `default` to provide more control of the container contents.
///   `Repeat(2usize, 3usize)` can be replaced by an arbitrary
///   expression which evaluates to a `Generator`.
///
/// The generator will additionally use all tags defined for
/// `Buildable` if those specific to `Generatable` are not present. In
/// this case, all instances in the sequence the generator produces
/// will receive the same value for the given field. This includes the
/// `sequence` tag.
///
/// Example:
/// ```rust
/// use boulder::{Generatable, Generator, Inc};
///
/// #[derive(Generatable)]
/// struct Foo {
///   // This field will be default initialized in every instance (a=0)
///   a: i32,
///   // This field will increase by 1 from 5 in successive instances.
///   #[boulder(generator=Inc(5))]
///   b: i32,
/// }
///
/// #[derive(Generatable)]
/// struct Bar {
///   // This field will take values from Foo::generator()
///   #[boulder(generatable)]
///   f1: Foo,
///   // This field will take values from Foo::generator().a(Inc(1)).build()
///   #[boulder(generatable(a=Inc(1)))]
///   f2: Foo,
///   // This field will be initialized with an increasing number of
///   // instances from Foo::generator().generate()
///   #[boulder(generatable, sequence_generator=Inc(0usize))]
///   f3: Vec<Foo>,
///   // This field will be initialized with one zero in every instance
///   #[boulder(sequence=1)]
///   ary: Vec<i32>,
/// }
///
/// let mut gen = Bar::generator();
/// let bar = gen.generate();
/// assert_eq!(bar.f1.a, 0);
/// assert_eq!(bar.f1.b, 5);
/// assert_eq!(bar.f2.a, 1);
/// assert_eq!(bar.f2.b, 5);
/// assert_eq!(bar.f3.len(), 0);
/// assert_eq!(bar.ary.len(), 1);
/// assert_eq!(bar.ary[0], 0);
/// let bar = gen.generate();
/// assert_eq!(bar.f1.a, 0);
/// assert_eq!(bar.f1.b, 6);
/// assert_eq!(bar.f2.a, 2);
/// assert_eq!(bar.f2.b, 6);
/// assert_eq!(bar.f3.len(), 1);
/// assert_eq!(bar.f3[0].a, 0);
/// assert_eq!(bar.f3[0].b, 5);
/// assert_eq!(bar.ary.len(), 1);
/// assert_eq!(bar.ary[0], 0);
/// ```
#[proc_macro_derive(Generatable, attributes(boulder))]
pub fn generatable(input: TokenStream) -> TokenStream {
    generator::derive_generatable(syn::parse_macro_input!(input)).into()
}

/// Make a `Generator` for formatted Strings.
///
/// The arguments to this macro broadly match the `format!` macro. The
/// first argument is a format string, and then subsequent arguments
/// must resolve to instances of `Generator`, whose output will be
/// formatted and inserted into the string as usual.
///
/// This macro is particularly useful when deriving `Generatable`,
/// since it's quite awkward to construct the necessary lambdas inside
/// the arguments to the `boulder` attribute.
///
/// Example:
/// ```rust
/// use boulder::{Const, Generator, Inc, Pattern};
///
/// let mut g = Pattern!("hello-{}-{}", Inc(11i32), Const(4i32));
/// assert_eq!(g.generate(), "hello-11-4");
/// assert_eq!(g.generate(), "hello-12-4");
/// assert_eq!(g.generate(), "hello-13-4");
/// ```
#[proc_macro]
pub fn string_pattern(input: TokenStream) -> TokenStream {
    string::pattern_macro(syn::parse_macro_input!(input)).into()
}

/// Make a `Generator` that recycles a collection of values.
///
/// The main advantage of this macro, over using `Repeat` directly, is
/// that it will coerce the types of the arguments. This can make
/// declarations much (much) more succinct when strings are involved.
///
/// Example:
/// ```rust
/// use boulder::{Generatable, Generator, Repeat};
///
/// #[derive(Generatable)]
/// struct Foo {
///    #[boulder(generator=Repeat!("hi", "bye"))]
///    a: String
/// }
///
/// let mut g = Foo::generator();
/// assert_eq!(g.generate().a, "hi");
/// assert_eq!(g.generate().a, "bye");
/// ```
#[proc_macro]
pub fn repeat(input: TokenStream) -> TokenStream {
    repeat::repeat_macro(syn::parse_macro_input!(input)).into()
}

#[cfg(feature = "persian-rug")]
mod persian_rug;

#[cfg(feature = "persian-rug")]
/// Derive the `BuildableWithPersianRug` trait for a type, creating a
/// suitable `BuilderWithPersianRug`.
///
/// This is only implemented for structs with named fields; there is
/// no implementation for either enums or structs with unnamed fields.
/// All fields will be default constructed in the absence of other
/// instructions. You can customise the construction process for your
/// type by using the `boulder` attribute on its fields, as follows:
///
/// - `#[boulder(default=Foo)]` The default value for this field is
///   `Foo`, where an arbitrary well-formed Rust expression can be
///   used in place of `Foo`.
///
/// - `#[boulder(default_with_persian_rug=|context| {(Foo,
///   context)})]` The default value for this field is derived from
///   the context alone, via a lambda or function which receives the
///   context, and must return it.
///
/// - `#[boulder(buildable)]` The type for this field implements
///   `Buildable` itself, so new values should be constructed using
///   `T::builder().build()`.
///
/// - `#[boulder(buildable_with_persian_rug)]` The type for this field implements
///   `BuildableWithPersianRug` itself, so new values should be constructed using
///   `T::builder().build(context)`.
///
/// - `#[boulder(buildable(a=5, b=10))]` The type for this field implements
///   `Buildable`, and new instances should be customised from the
///    default by setting `a=5` and `b=10` where `a` and `b` are
///    member names, and `5` and `10` can be replaced by arbitrary
///    well-formed Rust expressions.
///
/// - `#[boulder(buildable_with_persian_rug(a=5, b=10))]` The type for
///   this field implements `BuildableWithPersianRug`, and new
///   instances should be customised from the default by setting `a=5`
///   and `b=10` where `a` and `b` are member names, and `5` and `10`
///   can be replaced by arbitrary well-formed Rust expressions.
///
/// - `#[boulder(sequence=3)]` This field is assumed to be a
///   collection type (a type which can be the target of
///   `collect()`). Generate 3 items and initialize a new collection
///   from them, where `3` can be replaced by an arbitrary well-formed
///   Rust expression. The generation mechanism will be taken from any
///   generator specification (`generator` or `generatable`) if one is
///   given; otherwise it will be from the builder specification
///   (`default` or `buildable`, as described above) if one is given;
///   otherwise the items will be default initialized.
///
/// - `#[boulder(sequence_with_persian_rug=|context| {(f(context),
///   context)}]` This field is assumed to be a collection type (a
///   type which can be the target of `collect()`). Generate a number
///   of items dependent on the context, and initialize a new
///   collection from them, where `f(context)` can be replaced by an
///   arbitrary well-formed Rust expression. The generation mechanism
///   will be taken from any generator specification (`generator`,
///   `generatable`, `generator_with_persian_rug` or
///   `generatable_with_persian_rug`) if one is given; otherwise it
///   will be from the builder specification (`default`, `buildable`,
///   `default_with_persian_rug` or `buildable_with_persian_rug` as
///   described above) if one is given; otherwise the items will be
///   default initialized.
///
/// Example:
/// ```rust
/// use boulder::{BuildableWithPersianRug, GeneratableWithPersianRug, BuilderWithPersianRug};
/// use persian_rug::{contextual, persian_rug, Context, Proxy};
///
/// #[contextual(Rug)]
/// #[derive(BuildableWithPersianRug)]
/// #[boulder(persian_rug(context=Rug))]
/// struct Foo {
///   // This field will be default-initialized (a=0)
///   a: i32,
///   // This field will be initialized from the number of Bars currently in the context
///   #[boulder(default_with_persian_rug=|context| (context.get_iter::<Bar>().count(), context))]
///   b: i32,
/// }
///
/// #[contextual(Rug)]
/// #[derive(BuildableWithPersianRug)]
/// #[boulder(persian_rug(context=Rug))]
/// struct Bar {
///   // This field will be initialized as Foo::builder().build()
///   #[boulder(buildable_with_persian_rug)]
///   f1: Proxy<Foo>,
///   // This field will be initialized as Foo::builder().a(1).build()
///   #[boulder(buildable_with_persian_rug(a=1))]
///   f2: Proxy<Foo>,
///   // This field will be initialized with as many items as there are
///   // pre-existing Foos of Foo::builder().build()
///   #[boulder(buildable_with_persian_rug, sequence_with_persian_rug=|context| (context.get_proxy_iter::<Foo>().count(), context))]
///   f3: Vec<Proxy<Foo>>,
/// }
///
/// #[persian_rug]
/// struct Rug(#[table] Foo, #[table] Bar);
///
/// let mut r = Rug(Default::default(), Default::default());
/// let (foo, _) = Proxy::<Foo>::builder().build(&mut r);
/// assert_eq!(r.get(&foo).a, 0);
/// assert_eq!(r.get(&foo).b, 0);
///
/// let (bar, _) = Proxy::<Bar>::builder().build(&mut r);
/// assert_eq!(r.get(&r.get(&bar).f1).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f1).b, 0);
/// assert_eq!(r.get(&r.get(&bar).f2).a, 1);
/// assert_eq!(r.get(&r.get(&bar).f2).b, 0);
/// assert_eq!(r.get(&bar).f3.len(), 3);
/// assert_eq!(r.get(&r.get(&bar).f3[0]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[0]).b, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[1]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[1]).b, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[2]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[2]).b, 0);
///
/// let (foo, _) = Proxy::<Foo>::builder().build(&mut r);
/// assert_eq!(r.get(&foo).a, 0);
/// assert_eq!(r.get(&foo).b, 1);
///
/// let (bar, _) = Proxy::<Bar>::builder().build(&mut r);
/// assert_eq!(r.get(&r.get(&bar).f1).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f1).b, 1);
/// assert_eq!(r.get(&r.get(&bar).f2).a, 1);
/// assert_eq!(r.get(&r.get(&bar).f2).b, 1);
/// assert_eq!(r.get(&bar).f3.len(), 9);
/// assert_eq!(r.get(&r.get(&bar).f3[0]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[0]).b, 1);
/// assert_eq!(r.get(&r.get(&bar).f3[1]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[1]).b, 1);
/// assert_eq!(r.get(&r.get(&bar).f3[2]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[2]).b, 1);
/// assert_eq!(r.get(&r.get(&bar).f3[8]).a, 0);
/// assert_eq!(r.get(&r.get(&bar).f3[8]).b, 1);
/// ```
#[proc_macro_derive(BuildableWithPersianRug, attributes(boulder))]
pub fn buildable_with_persian_rug(input: TokenStream) -> TokenStream {
    persian_rug::builder::derive_buildable_with_persian_rug(syn::parse_macro_input!(input)).into()
}

#[cfg(feature = "persian-rug")]
#[proc_macro_derive(GeneratableWithPersianRug, attributes(boulder))]
pub fn generatable_with_persian_rug(input: TokenStream) -> TokenStream {
    persian_rug::generator::derive_generatable_with_persian_rug(syn::parse_macro_input!(input))
        .into()
}
