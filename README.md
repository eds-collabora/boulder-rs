# boulder - like a builder but heavier

This provides two main traits and associated derive macros:
- `Buildable` which lets you set complex default values, and then customise
  only the fields of interest for a given instance.
- `Generatable` which lets you set default sequences for each field, and
  then override only the particular sequences of interest.

These traits pass-through `Option<T>`, `Rc<T>`, `Arc<T>` and
`Mutex<T>` in the sense deriving `Generatable` or `Buildable` for `T`
automatically gives you default generators and builders for all of
these simple wrappers.

## License

This crate is made available under either an
[Apache-2.0](https://opensource.org/licenses/Apache-2.0) or an [MIT
license](https://opensource.org/licenses/MIT).
