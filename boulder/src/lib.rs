//! This crate is based around two derive macros, `Buildable` and
//! `Generatable` which provide a method to construct a `Builder` and
//! `Generator` respectively for your type, wrapping up a lot of
//! boilerplate within them.
//!
//! # Builder
//!
//! A `Builder` is a way to create a single object, specifying only the
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
//! A `Generator` is a way to create an infinite sequence of objects,
//! again specifying only the function for generating the fields you
//! wish to have non-default sequences. The default sequences are
//! specified using attributes on your type for ease of reading. Each
//! field gains a method of the same type in the generator which can
//! be be used to customise the sequence of values it takes between
//! objects.
//!
//! Example
//! ```rust
//! use boulder::{gen, Generator, Generatable};
//! #[derive(Generatable)]
//! struct Bar {
//!    #[boulder(generator=gen::Inc(1i32))]
//!    pub a: i32,
//!    #[boulder(generator=gen::Pattern!("hello-{}-foo", gen::Inc(5i32)))]
//!    pub b: String,
//! }
//!
//! let mut n = 2;
//! let mut gen = Bar::generator();
//! gen.a(move || {
//!      n = n + 2;
//!      n
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

mod builder;
mod generator;

pub use self::builder::{Buildable, Builder};
pub use self::generator::{generators as gen, Generatable, Generator};
pub use self::generator::{GeneratorIterator, GeneratorMutIterator};
