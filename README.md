# boulder - like a builder but heavier

This provides two main traits and associated derive macros:
- [`Buildable`](https://docs.rs/boulder/latest/boulder/trait.Buildable.html),
  which lets you set complex default values, and then customise only
  the fields of interest for a given instance.
- [`Generatable`](https://docs.rs/boulder/latest/boulder/trait.Generatable.html)
  which lets you set default sequences for each field, and then
  override only the particular sequences of interest.

These traits pass-through
[`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html),
[`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
[`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and
[`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) in
the sense deriving
[`Generatable`](https://docs.rs/boulder/latest/boulder/trait.Generatable.html)
or
[`Buildable`](https://docs.rs/boulder/latest/boulder/trait.Buildable.html)
for `T` automatically gives you default generators and builders for
all of these simple wrappers.

## License

This crate is made available under either an
[Apache-2.0](https://opensource.org/licenses/Apache-2.0) or an [MIT
license](https://opensource.org/licenses/MIT).
