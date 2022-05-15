#![allow(dead_code)]
#![allow(unused_imports)]

use std::any::Any;

mod persian_rug;

use boulder::{Buildable, Builder, Generatable, Generator};

fn foo(a: i16) -> i16 {
    a + 6
}

#[derive(Debug, Buildable)]
pub struct Womble {
    #[boulder(default = "hullo")]
    a: String,
    #[boulder(default=foo(1))]
    b: i32,
}

#[derive(Debug, Buildable)] //
pub struct Badger {
    #[boulder(buildable(a="hallo", b=foo(5)))]
    w: Womble,
    #[boulder(buildable)]
    v: Womble,
}

#[test]
fn test_simple() {
    let w = Womble::builder().a("hello").b(4i16).build();
    assert_eq!(std::any::TypeId::of::<Womble>(), w.type_id());
    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 4i32);

    let w = Womble::builder().build();
    assert_eq!(std::any::TypeId::of::<Womble>(), w.type_id());
    assert_eq!(w.a, "hullo".to_string());
    assert_eq!(w.b, 7i32);

    let b = Badger::builder().build();
    assert_eq!(std::any::TypeId::of::<Badger>(), b.type_id());
    assert_eq!(b.w.a, "hallo".to_string());
    assert_eq!(b.w.b, 11i32);
    assert_eq!(b.v.a, "hullo".to_string());
    assert_eq!(b.v.b, 7i32);
}

#[test]
fn test_option() {
    let w = Option::<Womble>::builder().a("hello").b(4i16).build();
    assert_eq!(std::any::TypeId::of::<Option<Womble>>(), w.type_id());
    assert_eq!(w.as_ref().map(|w| &w.a), Some(&"hello".to_string()));
    assert_eq!(w.as_ref().map(|w| w.b), Some(4i32));
}

#[test]
fn test_rc() {
    let w = std::rc::Rc::<Womble>::builder().a("hello").b(4i16).build();
    assert_eq!(std::any::TypeId::of::<std::rc::Rc<Womble>>(), w.type_id());
    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 4i32);
}

#[test]
fn test_arc() {
    let w = std::sync::Arc::<Womble>::builder()
        .a("hello")
        .b(4i16)
        .build();
    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<Womble>>(),
        w.type_id()
    );
    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 4i32);
}

#[test]
fn test_mutex() {
    let w = std::sync::Mutex::<Womble>::builder()
        .a("hello")
        .b(4i16)
        .build();
    assert_eq!(
        std::any::TypeId::of::<std::sync::Mutex<Womble>>(),
        w.type_id()
    );
    assert_eq!(w.lock().unwrap().a, "hello".to_string());
    assert_eq!(w.lock().unwrap().b, 4i32);
}

#[test]
fn test_ref_cell() {
    let w = std::cell::RefCell::<Womble>::builder()
        .a("hello")
        .b(4i16)
        .build();
    assert_eq!(
        std::any::TypeId::of::<std::cell::RefCell<Womble>>(),
        w.type_id()
    );
    assert_eq!(w.borrow().a, "hello".to_string());
    assert_eq!(w.borrow().b, 4i32);
}

#[test]
fn test_cell() {
    let w = std::cell::Cell::<Womble>::builder()
        .a("hello")
        .b(4i16)
        .build();

    assert_eq!(
        std::any::TypeId::of::<std::cell::Cell<Womble>>(),
        w.type_id()
    );
    let w_contents = w.into_inner();
    assert_eq!(w_contents.a, "hello".to_string());
    assert_eq!(w_contents.b, 4i32);
}

#[test]
fn test_arc_mutex() {
    let w = std::sync::Arc::<std::sync::Mutex<Womble>>::builder()
        .a("hello")
        .b(4i16)
        .build();
    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<std::sync::Mutex<Womble>>>(),
        w.type_id()
    );
    assert_eq!(w.lock().unwrap().a, "hello".to_string());
    assert_eq!(w.lock().unwrap().b, 4i32);
}

#[derive(Debug, Buildable)]
pub struct Bodger<U: Buildable>
where
    U: core::fmt::Debug,
{
    #[boulder(buildable(a="hallo", b=foo(5)))]
    w: Womble,
    #[boulder(buildable)]
    v: U,
}

#[test]
fn test_generic() {
    let w: Bodger<Womble> = Bodger::<Womble>::builder()
        .w(Womble::builder().build())
        .v(Womble::builder().build())
        .build();
    assert_eq!(std::any::TypeId::of::<Bodger<Womble>>(), w.type_id());
    assert_eq!(w.w.a, "hullo".to_string());
    assert_eq!(w.w.b, 7i32);
    assert_eq!(w.v.a, "hullo".to_string());
    assert_eq!(w.v.b, 7i32);
}

#[derive(Debug, Generatable)]
pub struct Wizard {
    #[boulder(default = "hello")]
    a: String,
    #[boulder(generator=boulder::gen::Inc(5))]
    b: i32,
}

#[test]
fn test_generator() {
    let mut g = Wizard::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(std::any::TypeId::of::<Wizard>(), w.type_id());
    assert_eq!(std::any::TypeId::of::<Wizard>(), w2.type_id());

    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 5);
    assert_eq!(w2.a, "hello".to_string());
    assert_eq!(w2.b, 6);
}

#[test]
fn test_option_generator() {
    let mut g = Option::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(std::any::TypeId::of::<Option<Wizard>>(), w.type_id());
    assert_eq!(std::any::TypeId::of::<Option<Wizard>>(), w2.type_id());

    assert_eq!(w.as_ref().map(|w| &w.a), Some(&"hello".to_string()));
    assert_eq!(w.as_ref().map(|w| &w.b), Some(&5));
    assert_eq!(w2.as_ref().map(|w| &w.a), Some(&"hello".to_string()));
    assert_eq!(w2.as_ref().map(|w| &w.b), Some(&6));

    let mut g = Option::<Option<Wizard>>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<Option<Option<Wizard>>>(),
        w.type_id()
    );
    assert_eq!(
        std::any::TypeId::of::<Option<Option<Wizard>>>(),
        w2.type_id()
    );

    assert_eq!(
        w.as_ref().map(|w| w.as_ref().map(|w| &w.a)),
        Some(Some(&"hello".to_string()))
    );
    assert_eq!(w.as_ref().map(|w| w.as_ref().map(|w| &w.b)), Some(Some(&5)));
    assert_eq!(
        w2.as_ref().map(|w| w.as_ref().map(|w| &w.a)),
        Some(Some(&"hello".to_string()))
    );
    assert_eq!(
        w2.as_ref().map(|w| w.as_ref().map(|w| &w.b)),
        Some(Some(&6))
    );
}

#[test]
fn test_rc_generator() {
    let mut g = std::rc::Rc::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(std::any::TypeId::of::<std::rc::Rc<Wizard>>(), w.type_id());
    assert_eq!(std::any::TypeId::of::<std::rc::Rc<Wizard>>(), w2.type_id());

    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 5);
    assert_eq!(w2.a, "hello".to_string());
    assert_eq!(w2.b, 6);
}

#[test]
fn test_arc_generator() {
    let mut g = std::sync::Arc::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<Wizard>>(),
        w.type_id()
    );
    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<Wizard>>(),
        w2.type_id()
    );

    assert_eq!(w.a, "hello".to_string());
    assert_eq!(w.b, 5);
    assert_eq!(w2.a, "hello".to_string());
    assert_eq!(w2.b, 6);
}

#[test]
fn test_mutex_generator() {
    let mut g = std::sync::Mutex::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<std::sync::Mutex<Wizard>>(),
        w.type_id()
    );
    assert_eq!(
        std::any::TypeId::of::<std::sync::Mutex<Wizard>>(),
        w2.type_id()
    );

    assert_eq!(w.lock().unwrap().a, "hello".to_string());
    assert_eq!(w.lock().unwrap().b, 5);
    assert_eq!(w2.lock().unwrap().a, "hello".to_string());
    assert_eq!(w2.lock().unwrap().b, 6);
}

#[test]
fn test_arc_mutex_generator() {
    let mut g = std::sync::Arc::<std::sync::Mutex<Wizard>>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<std::sync::Mutex<Wizard>>>(),
        w.type_id()
    );
    assert_eq!(
        std::any::TypeId::of::<std::sync::Arc<std::sync::Mutex<Wizard>>>(),
        w2.type_id()
    );

    assert_eq!(w.lock().unwrap().a, "hello".to_string());
    assert_eq!(w.lock().unwrap().b, 5);
    assert_eq!(w2.lock().unwrap().a, "hello".to_string());
    assert_eq!(w2.lock().unwrap().b, 6);
}

#[test]
fn test_cell_generator() {
    let mut g = std::cell::Cell::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<std::cell::Cell<Wizard>>(),
        w.type_id()
    );

    assert_eq!(
        std::any::TypeId::of::<std::cell::Cell<Wizard>>(),
        w2.type_id()
    );

    let w_contents = w.into_inner();
    assert_eq!(w_contents.a, "hello".to_string());
    assert_eq!(w_contents.b, 5);
    let w2_contents = w2.into_inner();
    assert_eq!(w2_contents.a, "hello".to_string());
    assert_eq!(w2_contents.b, 6);
}

#[test]
fn test_ref_cell_generator() {
    let mut g = std::cell::RefCell::<Wizard>::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(
        std::any::TypeId::of::<std::cell::RefCell<Wizard>>(),
        w.type_id()
    );

    assert_eq!(
        std::any::TypeId::of::<std::cell::RefCell<Wizard>>(),
        w2.type_id()
    );

    assert_eq!(w.borrow().a, "hello".to_string());
    assert_eq!(w.borrow().b, 5);
    assert_eq!(w2.borrow().a, "hello".to_string());
    assert_eq!(w2.borrow().b, 6);
}

#[test]
fn test_string_pattern() {
    let mut g = boulder::gen::Pattern!("example-{}", boulder::gen::Inc(2));
    for i in 0..5 {
        assert_eq!(g.generate(), format!("example-{}", i + 2));
    }
}

#[derive(Debug, Generatable)]
pub struct Sorceress {
    #[boulder(generator=boulder::gen::Pattern!("an-example-{}", boulder::gen::Inc(1)))]
    a: String,
    #[boulder(generator=boulder::gen::Inc(5))]
    b: i32,
}

#[test]
fn test_generator2() {
    let mut g = Sorceress::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(w.a, "an-example-1".to_string());
    assert_eq!(w.b, 5);
    assert_eq!(w2.a, "an-example-2".to_string());
    assert_eq!(w2.b, 6);
}

#[derive(Debug, Generatable)]
pub struct Sorceress2 {
    #[boulder(generator=boulder::gen::Pattern!("{}-an-example-{}", boulder::gen::Inc(1), boulder::gen::Inc(5)))]
    a: String,
    #[boulder(generator=boulder::gen::Inc(5))]
    b: i32,
}

#[test]
fn test_generator3() {
    let mut g = Sorceress2::generator();

    let w = g.generate();
    let w2 = g.generate();

    assert_eq!(w.a, "1-an-example-5".to_string());
    assert_eq!(w.b, 5);
    assert_eq!(w2.a, "2-an-example-6".to_string());
    assert_eq!(w2.b, 6);
}

#[derive(Debug, Generatable)]
pub struct Elephant<T>
where
    T: Generatable + 'static,
{
    #[boulder(generatable)]
    foo: T,
    #[boulder(default = 5)]
    ival: i64,
}

#[test]
fn test_iterator() {
    let g = Elephant::<Sorceress2>::generator();

    for (count, elt) in g.into_iter().take(5).enumerate() {
        assert_eq!(elt.foo.a, format!("{}-an-example-{}", count + 1, count + 5));
        assert_eq!(elt.foo.b, (count + 5) as i32);
        assert_eq!(elt.ival, 5i64);
    }
}

#[derive(Debug, Generatable)]
pub struct Giraffe {
    a: i32,
    b: String,
}

#[test]
fn test_closure() {
    let mut z = 5;
    let mut s = String::new();
    let mut gen = Giraffe::generator()
        .a(move || {
            z += 1;
            z
        })
        .b(move || {
            s.push('+');
            s.clone()
        });
    let g1 = gen.generate();
    let g2 = gen.generate();
    assert_eq!(g1.a, 6);
    assert_eq!(g1.b, "+".to_string());
    assert_eq!(g2.a, 7);
    assert_eq!(g2.b, "++".to_string());
}

#[derive(Debug, Buildable)]
pub struct Zebra1 {
    a: i32,
    #[boulder(sequence = 2)]
    b: Vec<String>,
}

#[derive(Debug, Buildable)]
pub struct Zebra2 {
    a: i32,
    #[boulder(sequence = 3, default = "hello")]
    b: Vec<String>,
}

#[derive(Debug, Buildable)]
pub struct Zebra3 {
    a: i32,
    #[boulder(sequence=4, generator=boulder::gen::Pattern!("a-{}", boulder::gen::Inc(0)))]
    b: Vec<String>,
}

#[derive(Clone, Debug, Buildable, Generatable)]
struct Nested {
    a: i32,
    b: String,
}

#[derive(Debug, Buildable)]
struct Zebra4 {
    a: i32,
    #[boulder(sequence=5, generatable(a=boulder::gen::Inc(5),
                                      b=boulder::gen::Pattern!("x{}", boulder::gen::Inc(2))))]
    b: Vec<Nested>,
}

#[derive(Debug, Buildable)]
struct Zebra5 {
    a: i32,
    #[boulder(sequence = 6, buildable(a = 10, b = "hello"))]
    b: Vec<Nested>,
}

#[test]
fn test_build_vector() {
    let z1: Zebra1 = Zebra1::builder().build();
    assert_eq!(z1.a, 0);
    assert_eq!(z1.b.len(), 2);
    assert_eq!(z1.b[0], String::new());
    assert_eq!(z1.b[1], String::new());

    let z2: Zebra2 = Zebra2::builder().build();
    assert_eq!(z2.a, 0);
    assert_eq!(z2.b.len(), 3);
    assert_eq!(z2.b[0], "hello".to_string());
    assert_eq!(z2.b[1], "hello".to_string());
    assert_eq!(z2.b[2], "hello".to_string());

    let z3: Zebra3 = Zebra3::builder().build();
    assert_eq!(z3.a, 0);
    assert_eq!(z3.b.len(), 4);
    assert_eq!(z3.b[0], "a-0".to_string());
    assert_eq!(z3.b[1], "a-1".to_string());
    assert_eq!(z3.b[2], "a-2".to_string());
    assert_eq!(z3.b[3], "a-3".to_string());

    let z4: Zebra4 = Zebra4::builder().build();
    assert_eq!(z4.a, 0);
    assert_eq!(z4.b.len(), 5);
    assert_eq!(z4.b[0].a, 5);
    assert_eq!(z4.b[0].b, "x2".to_string());
    assert_eq!(z4.b[1].a, 6);
    assert_eq!(z4.b[1].b, "x3".to_string());
    assert_eq!(z4.b[2].a, 7);
    assert_eq!(z4.b[2].b, "x4".to_string());
    assert_eq!(z4.b[3].a, 8);
    assert_eq!(z4.b[3].b, "x5".to_string());
    assert_eq!(z4.b[4].a, 9);
    assert_eq!(z4.b[4].b, "x6".to_string());

    let z5: Zebra5 = Zebra5::builder().build();
    assert_eq!(z5.a, 0);
    assert_eq!(z5.b.len(), 6);
    assert_eq!(z5.b[0].a, 10);
    assert_eq!(z5.b[0].b, "hello".to_string());
    assert_eq!(z5.b[1].a, 10);
    assert_eq!(z5.b[1].b, "hello".to_string());
    assert_eq!(z5.b[2].a, 10);
    assert_eq!(z5.b[2].b, "hello".to_string());
    assert_eq!(z5.b[3].a, 10);
    assert_eq!(z5.b[3].b, "hello".to_string());
    assert_eq!(z5.b[4].a, 10);
    assert_eq!(z5.b[4].b, "hello".to_string());
    assert_eq!(z5.b[5].a, 10);
    assert_eq!(z5.b[5].b, "hello".to_string());
}

#[derive(Debug, Generatable)]
pub struct Kangaroo1 {
    a: i32,
    #[boulder(sequence_generator = boulder::gen::Inc(2usize))]
    b: Vec<String>,
}

#[derive(Debug, Generatable)]
pub struct Kangaroo2 {
    a: i32,
    #[boulder(sequence_generator = boulder::gen::Inc(3usize), default = "hello")]
    b: Vec<String>,
}

#[derive(Debug, Generatable)]
pub struct Kangaroo3 {
    a: i32,
    #[boulder(sequence_generator= boulder::gen::Inc(4usize),
              generator=boulder::gen::Pattern!("a-{}", boulder::gen::Inc(0)))]
    b: Vec<String>,
}

#[derive(Debug, Generatable)]
struct Kangaroo4 {
    a: i32,
    #[boulder(sequence_generator= boulder::gen::Inc(5usize),
              generatable(a=boulder::gen::Inc(5),
                          b=boulder::gen::Pattern!("x{}", boulder::gen::Inc(2))))]
    b: Vec<Nested>,
}

#[derive(Debug, Generatable)]
struct Kangaroo5 {
    a: i32,
    #[boulder(sequence_generator = boulder::gen::Inc(6usize),
              buildable(a = 10, b = "hello"))]
    b: Vec<Nested>,
}

#[derive(Debug, Generatable)]
struct Kangaroo6 {
    a: i32,
    #[boulder(sequence = 3,
              generatable(a=boulder::gen::Inc(5),
                          b=boulder::gen::Pattern!("x{}", boulder::gen::Inc(2))))]
    b: Vec<Nested>,
}

#[test]
fn test_generate_vector() {
    let mut g = Kangaroo1::generator();
    let k11 = g.generate();
    let k12 = g.generate();
    assert_eq!(k11.a, 0);
    assert_eq!(k11.b.len(), 2);
    assert_eq!(k11.b[0], String::new());
    assert_eq!(k11.b[1], String::new());
    assert_eq!(k12.a, 0);
    assert_eq!(k12.b.len(), 3);
    assert_eq!(k12.b[0], String::new());
    assert_eq!(k12.b[1], String::new());
    assert_eq!(k12.b[2], String::new());

    let mut g = Kangaroo2::generator();
    let k21 = g.generate();
    let k22 = g.generate();
    assert_eq!(k21.a, 0);
    assert_eq!(k21.b.len(), 3);
    assert_eq!(k21.b[0], "hello".to_string());
    assert_eq!(k21.b[1], "hello".to_string());
    assert_eq!(k21.b[2], "hello".to_string());
    assert_eq!(k22.a, 0);
    assert_eq!(k22.b.len(), 4);
    assert_eq!(k22.b[0], "hello".to_string());
    assert_eq!(k22.b[1], "hello".to_string());
    assert_eq!(k22.b[2], "hello".to_string());
    assert_eq!(k22.b[3], "hello".to_string());

    let mut g = Kangaroo3::generator();
    let k31 = g.generate();
    let k32 = g.generate();
    assert_eq!(k31.a, 0);
    assert_eq!(k31.b.len(), 4);
    assert_eq!(k31.b[0], "a-0".to_string());
    assert_eq!(k31.b[1], "a-1".to_string());
    assert_eq!(k31.b[2], "a-2".to_string());
    assert_eq!(k31.b[3], "a-3".to_string());
    assert_eq!(k32.a, 0);
    assert_eq!(k32.b.len(), 5);
    assert_eq!(k32.b[0], "a-4".to_string());
    assert_eq!(k32.b[1], "a-5".to_string());
    assert_eq!(k32.b[2], "a-6".to_string());
    assert_eq!(k32.b[3], "a-7".to_string());
    assert_eq!(k32.b[4], "a-8".to_string());

    let mut g = Kangaroo4::generator();
    let k41 = g.generate();
    let k42 = g.generate();
    assert_eq!(k41.a, 0);
    assert_eq!(k41.b.len(), 5);
    assert_eq!(k41.b[0].a, 5);
    assert_eq!(k41.b[0].b, "x2".to_string());
    assert_eq!(k41.b[1].a, 6);
    assert_eq!(k41.b[1].b, "x3".to_string());
    assert_eq!(k41.b[2].a, 7);
    assert_eq!(k41.b[2].b, "x4".to_string());
    assert_eq!(k41.b[3].a, 8);
    assert_eq!(k41.b[3].b, "x5".to_string());
    assert_eq!(k41.b[4].a, 9);
    assert_eq!(k41.b[4].b, "x6".to_string());
    assert_eq!(k42.a, 0);
    assert_eq!(k42.b.len(), 6);
    assert_eq!(k42.b[0].a, 10);
    assert_eq!(k42.b[0].b, "x7".to_string());
    assert_eq!(k42.b[1].a, 11);
    assert_eq!(k42.b[1].b, "x8".to_string());
    assert_eq!(k42.b[2].a, 12);
    assert_eq!(k42.b[2].b, "x9".to_string());
    assert_eq!(k42.b[3].a, 13);
    assert_eq!(k42.b[3].b, "x10".to_string());
    assert_eq!(k42.b[4].a, 14);
    assert_eq!(k42.b[4].b, "x11".to_string());
    assert_eq!(k42.b[5].a, 15);
    assert_eq!(k42.b[5].b, "x12".to_string());

    let mut g = Kangaroo5::generator();
    let k51 = g.generate();
    let k52 = g.generate();
    assert_eq!(k51.a, 0);
    assert_eq!(k51.b.len(), 6);
    assert_eq!(k51.b[0].a, 10);
    assert_eq!(k51.b[0].b, "hello".to_string());
    assert_eq!(k51.b[1].a, 10);
    assert_eq!(k51.b[1].b, "hello".to_string());
    assert_eq!(k51.b[2].a, 10);
    assert_eq!(k51.b[2].b, "hello".to_string());
    assert_eq!(k51.b[3].a, 10);
    assert_eq!(k51.b[3].b, "hello".to_string());
    assert_eq!(k51.b[4].a, 10);
    assert_eq!(k51.b[4].b, "hello".to_string());
    assert_eq!(k51.b[5].a, 10);
    assert_eq!(k51.b[5].b, "hello".to_string());
    assert_eq!(k52.a, 0);
    assert_eq!(k52.b.len(), 7);
    assert_eq!(k52.b[0].a, 10);
    assert_eq!(k52.b[0].b, "hello".to_string());
    assert_eq!(k52.b[1].a, 10);
    assert_eq!(k52.b[1].b, "hello".to_string());
    assert_eq!(k52.b[2].a, 10);
    assert_eq!(k52.b[2].b, "hello".to_string());
    assert_eq!(k52.b[3].a, 10);
    assert_eq!(k52.b[3].b, "hello".to_string());
    assert_eq!(k52.b[4].a, 10);
    assert_eq!(k52.b[4].b, "hello".to_string());
    assert_eq!(k52.b[5].a, 10);
    assert_eq!(k52.b[5].b, "hello".to_string());
    assert_eq!(k52.b[6].a, 10);
    assert_eq!(k52.b[6].b, "hello".to_string());

    let mut g = Kangaroo6::generator();
    let k61 = g.generate();
    let k62 = g.generate();
    assert_eq!(k61.a, 0);
    assert_eq!(k61.b.len(), 3);
    assert_eq!(k61.b[0].a, 5);
    assert_eq!(k61.b[0].b, "x2".to_string());
    assert_eq!(k61.b[1].a, 6);
    assert_eq!(k61.b[1].b, "x3".to_string());
    assert_eq!(k61.b[2].a, 7);
    assert_eq!(k61.b[2].b, "x4".to_string());
    assert_eq!(k62.a, 0);
    assert_eq!(k62.b.len(), 3);
    assert_eq!(k62.b[0].a, 8);
    assert_eq!(k62.b[0].b, "x5".to_string());
    assert_eq!(k62.b[1].a, 9);
    assert_eq!(k62.b[1].b, "x6".to_string());
    assert_eq!(k62.b[2].a, 10);
    assert_eq!(k62.b[2].b, "x7".to_string());
}

mod builder_coverage {
    use boulder::{Buildable, Builder, Generatable, Generator};
    
    struct Parsnip1 {
        c1: i32,
    }

    struct Parsnip1Generator {
        c1: i32,
    }

    impl Generator for Parsnip1Generator {
        type Output = Parsnip1;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c1;
            self.c1 += 1;
            Parsnip1 { c1: ix }
        }
    }

    #[derive(Generatable)]
    struct Parsnip2 {
        #[boulder(generator=boulder::gen::Inc(0))]
        c2: i32,
    }

    #[derive(Buildable)]
    struct Parsnip3 {
        #[boulder(default = 0)]
        c3: i32,
    }

    struct Parsnip4 {
        c4: i32,
    }

    #[derive(Default)]
    struct Parsnip5 {
        c5: i32,
    }

    #[derive(Buildable)]
    struct Elephant {
        // #[boulder(generator=Parsnip1Generator {c1: 1})]
        // v1: Parsnip2,
        // #[boulder(generatable(c2=boulder::gen::Inc(2)))]
        // v2: Parsnip2,
        #[boulder(buildable(c3 = 3))]
        v3: Parsnip3,
        #[boulder(default=Parsnip4 { c4: 4 })]
        v4: Parsnip4,
        v5: Parsnip5,

        #[boulder(generator=Parsnip1Generator { c1: 1}, sequence=1)]
        s1: Vec<Parsnip1>,
        #[boulder(generatable(c2=boulder::gen::Inc(2)), sequence=2)]
        s2: Vec<Parsnip2>,
        #[boulder(buildable(c3 = 3), sequence = 3)]
        s3: Vec<Parsnip3>,
        #[boulder(default=Parsnip4 { c4: 4 }, sequence=4)]
        s4: Vec<Parsnip4>,
        #[boulder(sequence = 5)]
        s5: Vec<Parsnip5>,
    }

    #[test]
    fn test_defaults() {
        let e = Elephant::builder().build();

        assert_eq!(e.v3.c3, 3);
        assert_eq!(e.v4.c4, 4);
        assert_eq!(e.v5.c5, 0);

        assert_eq!(e.s1.len(), 1);
        assert_eq!(e.s1[0].c1, 1);
        assert_eq!(e.s2.len(), 2);
        assert_eq!(e.s2[0].c2, 2);
        assert_eq!(e.s2[1].c2, 3);
        assert_eq!(e.s3.len(), 3);
        assert_eq!(e.s3[0].c3, 3);
        assert_eq!(e.s3[1].c3, 3);
        assert_eq!(e.s3[2].c3, 3);
        assert_eq!(e.s4.len(), 4);
        assert_eq!(e.s4[0].c4, 4);
        assert_eq!(e.s4[1].c4, 4);
        assert_eq!(e.s4[2].c4, 4);
        assert_eq!(e.s4[3].c4, 4);
        assert_eq!(e.s5.len(), 5);
        assert_eq!(e.s5[0].c5, 0);
        assert_eq!(e.s5[1].c5, 0);
        assert_eq!(e.s5[2].c5, 0);
        assert_eq!(e.s5[3].c5, 0);
    }

    #[test]
    fn test_customise() {
        let e = Elephant::builder()
            .v3( Parsnip3 { c3: 33 } )
            .v4( Parsnip4 { c4: 44 } )
            .v5( Parsnip5 { c5: 55 } )
            .s1( vec![ Parsnip1 { c1: 11 } ] )
            .s2( vec![ Parsnip2 { c2: 22 } ] )
            .s3( vec![ Parsnip3 { c3: 33 } ] )
            .s4( vec![ Parsnip4 { c4: 44 } ] )
            .s5( vec![ Parsnip5 { c5: 55 } ] )
            .build();

        assert_eq!(e.v3.c3, 33);
        assert_eq!(e.v4.c4, 44);
        assert_eq!(e.v5.c5, 55);

        assert_eq!(e.s1.len(), 1);
        assert_eq!(e.s1[0].c1, 11);
        assert_eq!(e.s2.len(), 1);
        assert_eq!(e.s2[0].c2, 22);
        assert_eq!(e.s3.len(), 1);
        assert_eq!(e.s3[0].c3, 33);
        assert_eq!(e.s4.len(), 1);
        assert_eq!(e.s4[0].c4, 44);
        assert_eq!(e.s5.len(), 1);
        assert_eq!(e.s5[0].c5, 55);
    }
}

mod generator_coverage {
    use boulder::{Buildable, Builder, Generatable, Generator};
    
    struct Fig1 {
        c1: i32,
    }

    struct Fig1Generator {
        c1: i32,
    }

    impl Generator for Fig1Generator {
        type Output = Fig1;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c1;
            self.c1 += 1;
            Fig1 { c1: ix }
        }
    }

    #[derive(Generatable)]
    struct Fig2 {
        #[boulder(generator=boulder::gen::Inc(0))]
        c2: i32,
    }

    #[derive(Buildable)]
    struct Fig3 {
        #[boulder(default = 0)]
        c3: i32,
    }

    struct Fig4 {
        c4: i32,
    }

    #[derive(Default)]
    struct Fig5 {
        c5: i32,
    }

    #[derive(Generatable)]
    struct Monkey {
        #[boulder(generator=Fig1Generator {c1: 1})]
        v1: Fig1,
        #[boulder(generatable(c2=boulder::gen::Inc(2)))]
        v2: Fig2,
        #[boulder(buildable(c3 = 3))]
        v3: Fig3,
        #[boulder(default=Fig4 { c4: 4 })]
        v4: Fig4,
        v5: Fig5,

        #[boulder(generator=Fig1Generator { c1: 1 }, sequence=1)]
        s1: Vec<Fig1>,
        #[boulder(generatable(c2=boulder::gen::Inc(2)), sequence=2)]
        s2: Vec<Fig2>,
        #[boulder(buildable(c3 = 3), sequence = 3)]
        s3: Vec<Fig3>,
        #[boulder(default=Fig4 { c4: 4 }, sequence=4)]
        s4: Vec<Fig4>,
        #[boulder(sequence = 5)]
        s5: Vec<Fig5>,

        #[boulder(generator=Fig1Generator { c1: 1 }, sequence_generator=boulder::gen::Inc(1usize))]
        p1: Vec<Fig1>,
        #[boulder(generatable(c2=boulder::gen::Inc(2)), sequence_generator=boulder::gen::Inc(2usize))]
        p2: Vec<Fig2>,
        #[boulder(buildable(c3 = 3), sequence_generator=boulder::gen::Inc(3usize))]
        p3: Vec<Fig3>,
        #[boulder(default=Fig4 { c4: 4 }, sequence_generator=boulder::gen::Inc(4usize))]
        p4: Vec<Fig4>,
        #[boulder(sequence_generator = boulder::gen::Inc(5usize))]
        p5: Vec<Fig5>,
    }

    #[test]
    fn test_defaults() {
        let mut g = Monkey::generator();
        let m1 = g.generate();
        let m2 = g.generate();

        assert_eq!(m1.v1.c1, 1);
        assert_eq!(m1.v2.c2, 2);
        assert_eq!(m1.v3.c3, 3);
        assert_eq!(m1.v4.c4, 4);
        assert_eq!(m1.v5.c5, 0);
        assert_eq!(m2.v1.c1, 2);
        assert_eq!(m2.v2.c2, 3);
        assert_eq!(m2.v3.c3, 3);
        assert_eq!(m2.v4.c4, 4);
        assert_eq!(m2.v5.c5, 0);

        assert_eq!(m1.s1.len(), 1);
        assert_eq!(m1.s1[0].c1, 1);
        assert_eq!(m1.s2.len(), 2);
        assert_eq!(m1.s2[0].c2, 2);
        assert_eq!(m1.s2[1].c2, 3);
        assert_eq!(m1.s3.len(), 3);
        assert_eq!(m1.s3[0].c3, 3);
        assert_eq!(m1.s3[1].c3, 3);
        assert_eq!(m1.s3[2].c3, 3);
        assert_eq!(m1.s4.len(), 4);
        assert_eq!(m1.s4[0].c4, 4);
        assert_eq!(m1.s4[1].c4, 4);
        assert_eq!(m1.s4[2].c4, 4);
        assert_eq!(m1.s4[3].c4, 4);
        assert_eq!(m1.s5.len(), 5);
        assert_eq!(m1.s5[0].c5, 0);
        assert_eq!(m1.s5[1].c5, 0);
        assert_eq!(m1.s5[2].c5, 0);
        assert_eq!(m1.s5[3].c5, 0);
        assert_eq!(m2.s1.len(), 1);
        assert_eq!(m2.s1[0].c1, 2);
        assert_eq!(m2.s2.len(), 2);
        assert_eq!(m2.s2[0].c2, 4);
        assert_eq!(m2.s2[1].c2, 5);
        assert_eq!(m2.s3.len(), 3);
        assert_eq!(m2.s3[0].c3, 3);
        assert_eq!(m2.s3[1].c3, 3);
        assert_eq!(m2.s3[2].c3, 3);
        assert_eq!(m2.s4.len(), 4);
        assert_eq!(m2.s4[0].c4, 4);
        assert_eq!(m2.s4[1].c4, 4);
        assert_eq!(m2.s4[2].c4, 4);
        assert_eq!(m2.s4[3].c4, 4);
        assert_eq!(m2.s5.len(), 5);
        assert_eq!(m2.s5[0].c5, 0);
        assert_eq!(m2.s5[1].c5, 0);
        assert_eq!(m2.s5[2].c5, 0);
        assert_eq!(m2.s5[3].c5, 0);
        assert_eq!(m2.s5[4].c5, 0);

        assert_eq!(m1.p1.len(), 1);
        assert_eq!(m1.p1[0].c1, 1);
        assert_eq!(m1.p2.len(), 2);
        assert_eq!(m1.p2[0].c2, 2);
        assert_eq!(m1.p2[1].c2, 3);
        assert_eq!(m1.p3.len(), 3);
        assert_eq!(m1.p3[0].c3, 3);
        assert_eq!(m1.p3[1].c3, 3);
        assert_eq!(m1.p3[2].c3, 3);
        assert_eq!(m1.p4.len(), 4);
        assert_eq!(m1.p4[0].c4, 4);
        assert_eq!(m1.p4[1].c4, 4);
        assert_eq!(m1.p4[2].c4, 4);
        assert_eq!(m1.p4[3].c4, 4);
        assert_eq!(m1.p5.len(), 5);
        assert_eq!(m1.p5[0].c5, 0);
        assert_eq!(m1.p5[1].c5, 0);
        assert_eq!(m1.p5[2].c5, 0);
        assert_eq!(m1.p5[3].c5, 0);
        assert_eq!(m2.p1.len(), 2);
        assert_eq!(m2.p1[0].c1, 2);
        assert_eq!(m2.p1[1].c1, 3);
        assert_eq!(m2.p2.len(), 3);
        assert_eq!(m2.p2[0].c2, 4);
        assert_eq!(m2.p2[1].c2, 5);
        assert_eq!(m2.p2[2].c2, 6);
        assert_eq!(m2.p3.len(), 4);
        assert_eq!(m2.p3[0].c3, 3);
        assert_eq!(m2.p3[1].c3, 3);
        assert_eq!(m2.p3[2].c3, 3);
        assert_eq!(m2.p3[3].c3, 3);
        assert_eq!(m2.p4.len(), 5);
        assert_eq!(m2.p4[0].c4, 4);
        assert_eq!(m2.p4[1].c4, 4);
        assert_eq!(m2.p4[2].c4, 4);
        assert_eq!(m2.p4[3].c4, 4);
        assert_eq!(m2.p4[4].c4, 4);
        assert_eq!(m2.p5.len(), 6);
        assert_eq!(m2.p5[0].c5, 0);
        assert_eq!(m2.p5[1].c5, 0);
        assert_eq!(m2.p5[2].c5, 0);
        assert_eq!(m2.p5[3].c5, 0);
        assert_eq!(m2.p5[4].c5, 0);
        assert_eq!(m2.p5[5].c5, 0);
    }

    #[test]
    fn test_customise() {
        let mut g = Monkey::generator()
            .v1( || Fig1 { c1: 11 } )
            .v2( || Fig2 { c2: 22 } )
            .v3( || Fig3 { c3: 33 } )
            .v4( || Fig4 { c4: 44 } )
            .v5( || Fig5 { c5: 55 } )
            .s1( || vec![ Fig1 { c1: 11 } ] )
            .s2( || vec![ Fig2 { c2: 22 } ] )
            .s3( || vec![ Fig3 { c3: 33 } ] )
            .s4( || vec![ Fig4 { c4: 44 } ] )
            .s5( || vec![ Fig5 { c5: 55 } ] )
            .p1( || vec![ Fig1 { c1: 11 } ] )
            .p2( || vec![ Fig2 { c2: 22 } ] )
            .p3( || vec![ Fig3 { c3: 33 } ] )
            .p4( || vec![ Fig4 { c4: 44 } ] )
            .p5( || vec![ Fig5 { c5: 55 } ] );

        let m1 = g.generate();

        assert_eq!(m1.v1.c1, 11);
        assert_eq!(m1.v2.c2, 22);
        assert_eq!(m1.v3.c3, 33);
        assert_eq!(m1.v4.c4, 44);
        assert_eq!(m1.v5.c5, 55);

        assert_eq!(m1.s1.len(), 1);
        assert_eq!(m1.s1[0].c1, 11);
        assert_eq!(m1.s2.len(), 1);
        assert_eq!(m1.s2[0].c2, 22);
        assert_eq!(m1.s3.len(), 1);
        assert_eq!(m1.s3[0].c3, 33);
        assert_eq!(m1.s4.len(), 1);
        assert_eq!(m1.s4[0].c4, 44);
        assert_eq!(m1.s5.len(), 1);
        assert_eq!(m1.s5[0].c5, 55);

        assert_eq!(m1.p1.len(), 1);
        assert_eq!(m1.p1[0].c1, 11);
        assert_eq!(m1.p2.len(), 1);
        assert_eq!(m1.p2[0].c2, 22);
        assert_eq!(m1.p3.len(), 1);
        assert_eq!(m1.p3[0].c3, 33);
        assert_eq!(m1.p4.len(), 1);
        assert_eq!(m1.p4[0].c4, 44);
        assert_eq!(m1.p5.len(), 1);
        assert_eq!(m1.p5[0].c5, 55);
    }
}
