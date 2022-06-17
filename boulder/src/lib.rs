#![cfg_attr(docsrs, feature(doc_cfg))]

//! This crate is based around two traits, [`Buildable`] and
//! [`Generatable`], and their associated derive macros, which provide
//! ways to construct objects succinctly.
//!
//! # Builder
//!
//! A [`Builder`] is a way to create a single object, specifying only the
//! fields you wish to give non-default values. The default values are
//! specified using attributes on your type for ease of reading. Each
//! field gains a method of the same type in the builder which can be
//! be used to customise it.
//!
//! Example
//! ```rust
//! use boulder::{Builder, Buildable};
//! #[derive(Buildable)]
//! struct Foo {
//!    #[boulder(default=5)]
//!    pub a: i32,
//!    #[boulder(default="hello")]
//!    pub b: String,
//! }
//!
//! let f = Foo::builder()
//!          .a(27)
//!          .build();
//! assert_eq!(f.a, 27);
//! assert_eq!(f.b, "hello".to_string());
//! ```
//! # Generator
//!
//! A [`Generator`] is a way to create an infinite sequence of
//! objects, again specifying only the functions for generating the
//! fields you wish to have non-default sequences. The default
//! sequences are specified using attributes on your type for ease of
//! reading. Each field gains a method of the same name in the
//! generator, which can be be used to customise the sequence of
//! values produced for that field as objects are made.
//!
//! Example
//! ```rust
//! use boulder::{Generator, Generatable, Inc, Pattern};
//! #[derive(Generatable)]
//! struct Bar {
//!    #[boulder(generator=Inc(1i32))]
//!    pub a: i32,
//!    #[boulder(generator=Pattern!("hello-{}-foo", Inc(5i32)))]
//!    pub b: String,
//! }
//!
//! let mut n = 2;
//! let mut gen = Bar::generator()
//!      .a(move || {
//!          n = n + 2;
//!          n
//! });
//!
//! let bar1 = gen.generate();
//! assert_eq!(bar1.a, 4);
//! assert_eq!(bar1.b, "hello-5-foo".to_string());
//!
//! let bar2 = gen.generate();
//! assert_eq!(bar2.a, 6);
//! assert_eq!(bar2.b, "hello-6-foo".to_string());
//! ```
//!

#![cfg_attr(
    feature = "persian-rug",
    doc = r##"

# persian-rug

When the `"persian-rug"` feature is enabled, a parallel set of types,
traits and derive macros are enabled, i.e. [`BuildableWithPersianRug`]
and [`GeneratableWithPersianRug`]. These provide similar facilities,
but for types based on the [persian-rug](::persian_rug) crate.

Briefly, [persian-rug](::persian_rug) moves ownership of all values
into a single container (the "rug" or
[`Context`](::persian_rug::Context)). Client code then deals with handles
(in the form [`Proxy<T>`](::persian_rug::Proxy)) to these values. The
builders and generators take a mutator for the type's context in their
[`build`](BuilderWithPersianRug::build) and
[`generate`](GeneratorWithPersianRug::generate) methods, which means
at the point of construction, the set of other constructed instances
(of all types in that context) is available.

Since the derived generators pass through
[`Proxy<T>`](::persian_rug::Proxy), and since to get the most benefit from
this system you should work with proxies where possible, the most common
usage is to use these solely through the pass throughs.

Example:
```rust
use boulder::{BuilderWithPersianRug, BuildableWithPersianRug};
use persian_rug::{contextual, persian_rug, Context, Proxy};

#[persian_rug]
struct Rug(
  #[table] Foo,
  #[table] Bar,
);

#[contextual(Rug)]
#[derive(BuildableWithPersianRug)]
#[boulder(persian_rug(context=Rug))]
struct Foo {
   #[boulder(default=5)]
   pub a: i32,
   #[boulder(buildable_with_persian_rug(a=5))]
   pub b: Proxy<Bar>,
}

#[contextual(Rug)]
#[derive(BuildableWithPersianRug)]
#[boulder(persian_rug(context=Rug))]
struct Bar {
  #[boulder(default=2)]
  pub a: i32
}

let mut r = Rug(Default::default(), Default::default());
let (f, _) = Proxy::<Foo>::builder()
         .a(27)
         .build(&mut r);
assert_eq!(r.get(&f).a, 27);
assert_eq!(r.get(&r.get(&f).b).a, 5);
```

"##
)]

mod builder;
mod generator;

pub use self::builder::{Buildable, Builder};
pub use self::generator::generators::{
    Const, Cycle, Inc, Pattern, Repeat, Sample, Some, Subsets, Time,
};
pub use self::generator::{Generatable, Generator};
pub use self::generator::{GeneratorIterator, GeneratorMutIterator};

#[cfg(feature = "persian-rug")]
mod persian_rug;
#[cfg(feature = "persian-rug")]
#[cfg_attr(docsrs, doc(cfg(feature = "persian-rug")))]
pub use self::persian_rug::{
    BuildableWithPersianRug, BuilderWithPersianRug, GeneratableWithPersianRug,
    GeneratorToGeneratorWithPersianRugWrapper, GeneratorWithPersianRug,
    GeneratorWithPersianRugIterator, GeneratorWithPersianRugMutIterator, RepeatFromPersianRug,
    SampleFromPersianRug, SequenceGeneratorWithPersianRug, SubsetsFromPersianRug,
    TryRepeatFromPersianRug,
};

#[doc(hidden)]
pub mod guts {
    pub use crate::builder::guts as builder;
    pub use crate::generator::guts as generator;

    #[cfg(feature = "persian-rug")]
    pub mod persian_rug {
        pub use crate::persian_rug::builder::guts as builder;
        pub use crate::persian_rug::generator::guts as generator;
    }
}
