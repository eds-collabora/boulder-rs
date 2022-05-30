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
[`Cell<T>`](https://doc.rust-lang.org/std/cell/struct.Cell.html),
[`RefCell<T>`](https://doc.rust-lang.org/std/cell/struct.RefCell.html),
[`Rc<T>`](https://doc.rust-lang.org/std/rc/struct.Rc.html),
[`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html) and
[`Mutex<T>`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) in
the sense deriving
[`Generatable`](https://docs.rs/boulder/latest/boulder/trait.Generatable.html)
or
[`Buildable`](https://docs.rs/boulder/latest/boulder/trait.Buildable.html)
for `T` automatically gives you default generators and builders for
all of these simple wrappers.

If you enable the `persian-rug` feature, you get two new traits:
- [`BuildableWithPersianRug`](https://docs.rs/boulder/latest/boulder/trait.BuildableWithPersianRug.html),
  which lets you build instances which belong to a
  [`persian_rug::Context`](https://docs.rs/boulder/latest/persian-rug/trait.Context.html).
- [`GeneratableWithPersianRug`](https://docs.rs/boulder/latest/boulder/trait.GeneratableWithPersianRug.html),
  which lets you generate instances which belong to a
  [`persian_rug::Context`](https://docs.rs/boulder/latest/persian-rug/trait.Context.html).

These traits pass-through all the same all the same wrappers as the
base traits, but additionally
[`Proxy<T>`](https://docs.rs/boulder/latest/persian-rug/struct.Proxy.html).

## License

This crate is made available under either an
[Apache-2.0](https://opensource.org/licenses/Apache-2.0) or an [MIT
license](https://opensource.org/licenses/MIT).
