use boulder::{BuildableWithPersianRug, BuilderWithPersianRug};
use boulder_derive::BuildableWithPersianRug;

#[derive(BuildableWithPersianRug)]
#[boulder(persian_rug(context=C, access(Foo2<C>)))]
struct Foo2<C> {
    _marker: core::marker::PhantomData<C>,
    a: i32,
}

impl<C: persian_rug::Context> persian_rug::StatefulCtx for Foo2<C> {
    type Context = C;
}

#[derive(BuildableWithPersianRug)]
#[boulder(persian_rug(context=C, access(Foo2<C>, Bar2<C>)))]
struct Bar2<C> {
    a: i32,
    #[boulder(buildable_with_context(a=5))]
    foo: persian_rug::Proxy<Foo2<C>>
}

impl<C: persian_rug::Context> persian_rug::StatefulCtx for Bar2<C> {
    type Context = C;
}

#[derive(BuildableWithPersianRug)]
#[boulder(persian_rug(context=C, access(Foo2<C>, Bar2<C>, Baz2<C>)))]
struct Baz2<C> {
    a: i32,
    #[boulder(buildable_with_context)]
    bar: persian_rug::Proxy<Bar2<C>>
}

impl<C: persian_rug::Context> persian_rug::StatefulCtx for Baz2<C> {
    type Context = C;
}

#[derive(Default)]
#[persian_rug::state]
struct State2 {
    #[table]
    foos: Foo2<State2>,
    #[table]
    bars: Bar2<State2>,
    #[table]
    bazs: Baz2<State2>
}

#[test]
fn test_simple() {
    let mut s: State2 = Default::default();

    let (f1, _) = Foo2::<State2>::builder()
        .a(1)
        .build(&mut s);
    let f1 = <State2 as persian_rug::Context>::add(&mut s, f1);
    
    let (b1, _) = Bar2::<State2>::builder()
        .a(2)
        .build(&mut s);
    let b1 = <State2 as persian_rug::Context>::add(&mut s, b1);

    let (z1, _) = persian_rug::Proxy::<Baz2<State2>>::builder()
        .a(3)
        .build(&mut s);

    println!("Got foo2 {:?}", f1);
    println!("Got bar2 {:?}", b1);
    println!("Got baz2 {:?}", z1);
}

#[test]
fn test_option() {
    let mut s: State2 = Default::default();

    let (f1, _) = Option::<Foo2<State2>>::builder()
        .a(5)
        .build(&mut s);
    
    let f1 = <State2 as persian_rug::Context>::add(&mut s, f1.unwrap());
    println!("Got foo2 {:?}", f1);
    
    let (b1, _) = Option::<persian_rug::Proxy<Bar2<State2>>>::builder()
        .a(5)
        .build(&mut s);
    println!("Got bar2 {:?}", b1);
}
