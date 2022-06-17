#![allow(clippy::blacklisted_name)]

mod builder_basic {
    use boulder::{BuildableWithPersianRug, BuilderWithPersianRug};
    use boulder::{GeneratableWithPersianRug, GeneratorWithPersianRug};

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Foo2<C>)))]
    struct Foo2<C> {
        _marker: core::marker::PhantomData<C>,
        a: i32,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Foo2<C> {
        type Context = C;
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Foo2<C>, Bar2<C>)))]
    struct Bar2<C> {
        a: i32,
        #[boulder(buildable_with_persian_rug(a = 5))]
        foo: persian_rug::Proxy<Foo2<C>>,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Bar2<C> {
        type Context = C;
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Foo2<C>, Bar2<C>, Baz2<C>)))]
    struct Baz2<C> {
        a: i32,
        #[boulder(buildable_with_persian_rug)]
        bar: persian_rug::Proxy<Bar2<C>>,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Baz2<C> {
        type Context = C;
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct State2 {
        #[table]
        foos: Foo2<State2>,
        #[table]
        bars: Bar2<State2>,
        #[table]
        bazs: Baz2<State2>,
    }

    #[test]
    fn test_simple() {
        let mut s: State2 = Default::default();

        let (f1, _) = Foo2::<State2>::builder().a(1).build(&mut s);
        let f1 = <State2 as persian_rug::Context>::add(&mut s, f1);

        let (b1, _) = Bar2::<State2>::builder().a(2).build(&mut s);
        let b1 = <State2 as persian_rug::Context>::add(&mut s, b1);

        let (z1, _) = persian_rug::Proxy::<Baz2<State2>>::builder()
            .a(3)
            .build(&mut s);

        println!("Got foo2 {:?}", f1);
        println!("Got bar2 {:?}", b1);
        println!("Got baz2 {:?}", z1);
    }
}

mod builder_wrappers {
    use super::*;

    use boulder::{BuildableWithPersianRug, BuilderWithPersianRug};
    use persian_rug::Proxy;
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[persian_rug::contextual(C)]
    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Foo2<C>)))]
    struct Foo2<C: persian_rug::Context> {
        _marker: core::marker::PhantomData<C>,
        a: i32,
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct State2 {
        #[table]
        foos: Foo2<State2>,
    }

    #[test]
    fn test_option() {
        let mut s: State2 = Default::default();

        let (f1, _) = Option::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Option<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.as_ref().map(|f1| f1.a), Some(5));
    }

    #[test]
    fn test_proxy() {
        let mut s: State2 = Default::default();

        let (f1, _) = Proxy::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Proxy<Foo2<State2>>>(), f1.type_id());
        assert_eq!(<&State2 as persian_rug::Accessor>::get(&&s, &f1).a, 5);
    }

    #[test]
    fn test_option_proxy() {
        let mut s: State2 = Default::default();

        let (f1, _) = Option::<Proxy<Foo2<State2>>>::builder().a(5).build(&mut s);
        assert_eq!(
            std::any::TypeId::of::<Option<Proxy<Foo2<State2>>>>(),
            f1.type_id()
        );
        assert_eq!(
            f1.as_ref()
                .map(|f1| <State2 as persian_rug::Context>::get(&s, &f1).a),
            Some(5)
        );
    }

    #[test]
    fn test_arc() {
        let mut s: State2 = Default::default();

        let (f1, _) = Arc::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Arc<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.a, 5);
    }

    #[test]
    fn test_mutex() {
        let mut s: State2 = Default::default();

        let (f1, _) = Mutex::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Mutex<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.lock().unwrap().a, 5);
    }

    #[test]
    fn test_arc_mutex() {
        let mut s: State2 = Default::default();

        let (f1, _) = Arc::<Mutex<Foo2<State2>>>::builder().a(5).build(&mut s);
        assert_eq!(
            std::any::TypeId::of::<Arc<Mutex<Foo2<State2>>>>(),
            f1.type_id()
        );
        assert_eq!(f1.lock().unwrap().a, 5);
    }

    #[test]
    fn test_rc() {
        let mut s: State2 = Default::default();

        let (f1, _) = Rc::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Rc<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.a, 5);
    }

    #[test]
    fn test_cell() {
        let mut s: State2 = Default::default();

        let (f1, _) = Cell::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(std::any::TypeId::of::<Cell<Foo2<State2>>>(), f1.type_id());
        let f1_contents = f1.into_inner();
        assert_eq!(f1_contents.a, 5);
    }

    #[test]
    fn test_ref_cell() {
        let mut s: State2 = Default::default();

        let (f1, _) = RefCell::<Foo2<State2>>::builder().a(5).build(&mut s);
        assert_eq!(
            std::any::TypeId::of::<RefCell<Foo2<State2>>>(),
            f1.type_id()
        );
        assert_eq!(f1.borrow().a, 5);
    }
}

mod builder_coverage {
    use boulder::{Buildable, BuildableWithPersianRug, Builder, BuilderWithPersianRug};
    use boulder::{Generatable, GeneratableWithPersianRug, Generator, GeneratorWithPersianRug};
    use std::any::Any;

    struct Carrot1 {
        c1: i32,
    }

    struct Carrot1Generator {
        c1: i32,
    }

    #[persian_rug::constraints(context=C)]
    impl<C> GeneratorWithPersianRug<C> for Carrot1Generator {
        type Output = Carrot1;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + ::persian_rug::Mutator<Context = C>,
        {
            let ix = self.c1;
            self.c1 += 1;

            (Carrot1 { c1: ix }, context)
        }
    }

    struct Carrot2 {
        c2: i32,
    }

    struct Carrot2Generator {
        c2: i32,
    }

    impl Generator for Carrot2Generator {
        type Output = Carrot2;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c2;
            self.c2 += 1;
            Carrot2 { c2: ix }
        }
    }

    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Carrot3<C: 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c3: i32,
    }

    struct TestIndexGenerator {
        value: i32,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestIndexGenerator {
        type Output = i32;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    #[derive(Generatable)]
    struct Carrot4 {
        #[boulder(generator=boulder::Inc(0))]
        c4: i32,
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Carrot5<C> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c5: i32,
    }

    #[derive(Buildable)]
    struct Carrot6 {
        #[boulder(default = 0)]
        c6: i32,
    }

    struct Carrot7 {
        c7: i32,
    }

    struct Carrot8 {
        c8: i32,
    }

    #[derive(Default)]
    struct Carrot9 {
        c9: i32,
    }

    #[persian_rug::contextual(C)]
    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Rabbit<C>)))]
    struct Rabbit<C: persian_rug::Context>
    where
        C: 'static,
    {
        // #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 })]
        // v1: Carrot1,
        // #[boulder(generator=Carrot2Generator {c2: 2})]
        // v2: Carrot2,
        // //v3: Carrot3,
        // #[boulder(generatable(c4=boulder::Inc(4)))]
        // v4: Carrot4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Carrot5<C>,
        #[boulder(buildable(c6 = 6))]
        v6: Carrot6,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Rabbit<_>>().count(); (Carrot7 { c7: 7 }, context) })]
        v7: Carrot7,
        #[boulder(default=Carrot8 { c8: 8 })]
        v8: Carrot8,
        v9: Carrot9,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence=1)]
        s1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence=2)]
        s2: Vec<Carrot2>,
        #[boulder(generatable_with_persian_rug(c3=TestIndexGenerator { value: 3 }), sequence=3)]
        s3: Vec<Carrot3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence=4)]
        s4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5)]
        s5: Vec<Carrot5<C>>,
        #[boulder(buildable(c6 = 6), sequence = 6)]
        s6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Rabbit<_>>().count(); (Carrot7 { c7: 7 }, context) }, sequence=7)]
        s7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence=8)]
        s8: Vec<Carrot8>,
        #[boulder(sequence = 9)]
        s9: Vec<Carrot9>,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence_with_persian_rug=|context| { context.get_iter::<Rabbit<_>>().count(); (1, context) } )]
        p1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence_with_persian_rug=|context| { context.get_iter::<Rabbit<_>>().count(); (2, context) })]
        p2: Vec<Carrot2>,
        #[boulder(generatable_with_persian_rug(c3=TestIndexGenerator { value: 3 }), sequence_with_persian_rug=|context| { context.get_iter::<Rabbit<_>>().count();  (3, context)})]
        p3: Vec<Carrot3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (4, context)})]
        p4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5=5), sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (5, context)})]
        p5: Vec<Carrot5<C>>,
        #[boulder(buildable(c6=6), sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (6, context)})]
        p6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (Carrot7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (7, context)})]
        p7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (8, context)})]
        p8: Vec<Carrot8>,
        #[boulder(sequence_with_persian_rug=|context| {context.get_iter::<Rabbit<_>>().count(); (9, context)})]
        p9: Vec<Carrot9>,
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct RabbitState {
        #[table]
        rabbits: Rabbit<RabbitState>,
    }

    #[test]
    fn test_defaults() {
        let mut s: RabbitState = Default::default();

        let (r, _) = Rabbit::builder().build(&mut s);

        assert_eq!(r.v5.c5, 5);
        assert_eq!(r.v6.c6, 6);
        assert_eq!(r.v7.c7, 7);
        assert_eq!(r.v8.c8, 8);
        assert_eq!(r.v9.c9, 0);

        assert_eq!(r.s1.len(), 1);
        assert_eq!(r.s1[0].c1, 1);
        assert_eq!(r.s2.len(), 2);
        assert_eq!(r.s2[0].c2, 2);
        assert_eq!(r.s2[1].c2, 3);
        assert_eq!(r.s3.len(), 3);
        assert_eq!(r.s3[0].c3, 3);
        assert_eq!(r.s3[1].c3, 4);
        assert_eq!(r.s3[2].c3, 5);
        assert_eq!(r.s4.len(), 4);
        assert_eq!(r.s4[0].c4, 4);
        assert_eq!(r.s4[1].c4, 5);
        assert_eq!(r.s4[2].c4, 6);
        assert_eq!(r.s4[3].c4, 7);
        assert_eq!(r.s5.len(), 5);
        assert_eq!(r.s5[0].c5, 5);
        assert_eq!(r.s5[1].c5, 5);
        assert_eq!(r.s5[2].c5, 5);
        assert_eq!(r.s5[3].c5, 5);
        assert_eq!(r.s5[4].c5, 5);
        assert_eq!(r.s6.len(), 6);
        assert_eq!(r.s6[0].c6, 6);
        assert_eq!(r.s6[1].c6, 6);
        assert_eq!(r.s6[2].c6, 6);
        assert_eq!(r.s6[3].c6, 6);
        assert_eq!(r.s6[4].c6, 6);
        assert_eq!(r.s6[5].c6, 6);
        assert_eq!(r.s7.len(), 7);
        assert_eq!(r.s7[0].c7, 7);
        assert_eq!(r.s7[1].c7, 7);
        assert_eq!(r.s7[2].c7, 7);
        assert_eq!(r.s7[3].c7, 7);
        assert_eq!(r.s7[4].c7, 7);
        assert_eq!(r.s7[5].c7, 7);
        assert_eq!(r.s7[6].c7, 7);
        assert_eq!(r.s8.len(), 8);
        assert_eq!(r.s8[0].c8, 8);
        assert_eq!(r.s8[1].c8, 8);
        assert_eq!(r.s8[2].c8, 8);
        assert_eq!(r.s8[3].c8, 8);
        assert_eq!(r.s8[4].c8, 8);
        assert_eq!(r.s8[5].c8, 8);
        assert_eq!(r.s8[6].c8, 8);
        assert_eq!(r.s8[7].c8, 8);
        assert_eq!(r.s9.len(), 9);
        assert_eq!(r.s9[0].c9, 0);
        assert_eq!(r.s9[1].c9, 0);
        assert_eq!(r.s9[2].c9, 0);
        assert_eq!(r.s9[3].c9, 0);
        assert_eq!(r.s9[4].c9, 0);
        assert_eq!(r.s9[5].c9, 0);
        assert_eq!(r.s9[6].c9, 0);
        assert_eq!(r.s9[7].c9, 0);
        assert_eq!(r.s9[8].c9, 0);

        assert_eq!(r.p1.len(), 1);
        assert_eq!(r.p1[0].c1, 1);
        assert_eq!(r.p2.len(), 2);
        assert_eq!(r.p2[0].c2, 2);
        assert_eq!(r.p2[1].c2, 3);
        assert_eq!(r.p3.len(), 3);
        assert_eq!(r.p3[0].c3, 3);
        assert_eq!(r.p3[1].c3, 4);
        assert_eq!(r.p3[2].c3, 5);
        assert_eq!(r.p4.len(), 4);
        assert_eq!(r.p4[0].c4, 4);
        assert_eq!(r.p4[1].c4, 5);
        assert_eq!(r.p4[2].c4, 6);
        assert_eq!(r.p4[3].c4, 7);
        assert_eq!(r.p5.len(), 5);
        assert_eq!(r.p5[0].c5, 5);
        assert_eq!(r.p5[1].c5, 5);
        assert_eq!(r.p5[2].c5, 5);
        assert_eq!(r.p5[3].c5, 5);
        assert_eq!(r.p5[4].c5, 5);
        assert_eq!(r.p6.len(), 6);
        assert_eq!(r.p6[0].c6, 6);
        assert_eq!(r.p6[1].c6, 6);
        assert_eq!(r.p6[2].c6, 6);
        assert_eq!(r.p6[3].c6, 6);
        assert_eq!(r.p6[4].c6, 6);
        assert_eq!(r.p6[5].c6, 6);
        assert_eq!(r.p7.len(), 7);
        assert_eq!(r.p7[0].c7, 7);
        assert_eq!(r.p7[1].c7, 7);
        assert_eq!(r.p7[2].c7, 7);
        assert_eq!(r.p7[3].c7, 7);
        assert_eq!(r.p7[4].c7, 7);
        assert_eq!(r.p7[5].c7, 7);
        assert_eq!(r.p7[6].c7, 7);
        assert_eq!(r.p8.len(), 8);
        assert_eq!(r.p8[0].c8, 8);
        assert_eq!(r.p8[1].c8, 8);
        assert_eq!(r.p8[2].c8, 8);
        assert_eq!(r.p8[3].c8, 8);
        assert_eq!(r.p8[4].c8, 8);
        assert_eq!(r.p8[5].c8, 8);
        assert_eq!(r.p8[6].c8, 8);
        assert_eq!(r.p8[7].c8, 8);
        assert_eq!(r.p9.len(), 9);
        assert_eq!(r.p9[0].c9, 0);
        assert_eq!(r.p9[1].c9, 0);
        assert_eq!(r.p9[2].c9, 0);
        assert_eq!(r.p9[3].c9, 0);
        assert_eq!(r.p9[4].c9, 0);
        assert_eq!(r.p9[5].c9, 0);
        assert_eq!(r.p9[6].c9, 0);
        assert_eq!(r.p9[7].c9, 0);
        assert_eq!(r.p9[8].c9, 0);
    }

    #[test]
    fn test_customise() {
        let mut s: RabbitState = Default::default();

        let (r, _) = Rabbit::builder()
            .v5(Carrot5 {
                c5: 55,
                _marker: Default::default(),
            })
            .v6(Carrot6 { c6: 66 })
            .v7(Carrot7 { c7: 77 })
            .v8(Carrot8 { c8: 88 })
            .v9(Carrot9 { c9: 99 })
            .s1(vec![Carrot1 { c1: 11 }])
            .s2(vec![Carrot2 { c2: 22 }])
            .s3(vec![Carrot3 {
                c3: 33,
                _marker: Default::default(),
            }])
            .s4(vec![Carrot4 { c4: 44 }])
            .s5(vec![Carrot5 {
                c5: 55,
                _marker: Default::default(),
            }])
            .s6(vec![Carrot6 { c6: 66 }])
            .s7(vec![Carrot7 { c7: 77 }])
            .s8(vec![Carrot8 { c8: 88 }])
            .s9(vec![Carrot9 { c9: 99 }])
            .p1(vec![Carrot1 { c1: 11 }])
            .p2(vec![Carrot2 { c2: 22 }])
            .p3(vec![Carrot3 {
                c3: 33,
                _marker: Default::default(),
            }])
            .p4(vec![Carrot4 { c4: 44 }])
            .p5(vec![Carrot5 {
                c5: 55,
                _marker: Default::default(),
            }])
            .p6(vec![Carrot6 { c6: 66 }])
            .p7(vec![Carrot7 { c7: 77 }])
            .p8(vec![Carrot8 { c8: 88 }])
            .p9(vec![Carrot9 { c9: 99 }])
            .build(&mut s);

        assert_eq!(r.v5.c5, 55);
        assert_eq!(r.v6.c6, 66);
        assert_eq!(r.v7.c7, 77);
        assert_eq!(r.v8.c8, 88);
        assert_eq!(r.v9.c9, 99);

        assert_eq!(r.s1.len(), 1);
        assert_eq!(r.s1[0].c1, 11);
        assert_eq!(r.s2.len(), 1);
        assert_eq!(r.s2[0].c2, 22);
        assert_eq!(r.s3.len(), 1);
        assert_eq!(r.s3[0].c3, 33);
        assert_eq!(r.s4.len(), 1);
        assert_eq!(r.s4[0].c4, 44);
        assert_eq!(r.s5.len(), 1);
        assert_eq!(r.s5[0].c5, 55);
        assert_eq!(r.s6.len(), 1);
        assert_eq!(r.s6[0].c6, 66);
        assert_eq!(r.s7.len(), 1);
        assert_eq!(r.s7[0].c7, 77);
        assert_eq!(r.s8.len(), 1);
        assert_eq!(r.s8[0].c8, 88);
        assert_eq!(r.s9.len(), 1);
        assert_eq!(r.s9[0].c9, 99);

        assert_eq!(r.p1.len(), 1);
        assert_eq!(r.p1[0].c1, 11);
        assert_eq!(r.p2.len(), 1);
        assert_eq!(r.p2[0].c2, 22);
        assert_eq!(r.p3.len(), 1);
        assert_eq!(r.p3[0].c3, 33);
        assert_eq!(r.p4.len(), 1);
        assert_eq!(r.p4[0].c4, 44);
        assert_eq!(r.p5.len(), 1);
        assert_eq!(r.p5[0].c5, 55);
        assert_eq!(r.p6.len(), 1);
        assert_eq!(r.p6[0].c6, 66);
        assert_eq!(r.p7.len(), 1);
        assert_eq!(r.p7[0].c7, 77);
        assert_eq!(r.p8.len(), 1);
        assert_eq!(r.p8[0].c8, 88);
        assert_eq!(r.p9.len(), 1);
        assert_eq!(r.p9[0].c9, 99);
    }
}

mod builder_coverage_no_generics {
    use boulder::{Buildable, BuildableWithPersianRug, Builder, BuilderWithPersianRug};
    use boulder::{Generatable, GeneratableWithPersianRug, Generator, GeneratorWithPersianRug};
    use std::any::Any;

    struct Carrot1 {
        c1: i32,
    }

    struct Carrot1Generator {
        c1: i32,
    }

    #[persian_rug::constraints(context=C)]
    impl<C> GeneratorWithPersianRug<C> for Carrot1Generator {
        type Output = Carrot1;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + ::persian_rug::Mutator<Context = C>,
        {
            let ix = self.c1;
            self.c1 += 1;

            (Carrot1 { c1: ix }, context)
        }
    }

    struct Carrot2 {
        c2: i32,
    }

    struct Carrot2Generator {
        c2: i32,
    }

    impl Generator for Carrot2Generator {
        type Output = Carrot2;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c2;
            self.c2 += 1;
            Carrot2 { c2: ix }
        }
    }

    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Carrot3<C: 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c3: i32,
    }

    struct TestIndexGenerator {
        value: i32,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestIndexGenerator {
        type Output = i32;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    #[derive(Generatable)]
    struct Carrot4 {
        #[boulder(generator=boulder::Inc(0))]
        c4: i32,
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Carrot5<C> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c5: i32,
    }

    #[derive(Buildable)]
    struct Carrot6 {
        #[boulder(default = 0)]
        c6: i32,
    }

    struct Carrot7 {
        c7: i32,
    }

    struct Carrot8 {
        c8: i32,
    }

    #[derive(Default)]
    struct Carrot9 {
        c9: i32,
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=HareState, access(Hare)))]
    #[persian_rug::contextual(HareState)]
    struct Hare {
        // #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 })]
        // v1: Carrot1,
        // #[boulder(generator=Carrot2Generator {c2: 2})]
        // v2: Carrot2,
        // //v3: Carrot3,
        // #[boulder(generatable(c4=boulder::Inc(4)))]
        // v4: Carrot4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Carrot5<HareState>,
        #[boulder(buildable(c6 = 6))]
        v6: Carrot6,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Hare>().count(); (Carrot7 { c7: 7 }, context) })]
        v7: Carrot7,
        #[boulder(default=Carrot8 { c8: 8 })]
        v8: Carrot8,
        v9: Carrot9,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence=1)]
        s1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence=2)]
        s2: Vec<Carrot2>,
        #[boulder(generatable_with_persian_rug(c3=TestIndexGenerator { value: 3 }), sequence=3)]
        s3: Vec<Carrot3<HareState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence=4)]
        s4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5)]
        s5: Vec<Carrot5<HareState>>,
        #[boulder(buildable(c6 = 6), sequence = 6)]
        s6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Hare>().count(); (Carrot7 { c7: 7 }, context) }, sequence=7)]
        s7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence=8)]
        s8: Vec<Carrot8>,
        #[boulder(sequence = 9)]
        s9: Vec<Carrot9>,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (1, context)})]
        p1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (2, context)})]
        p2: Vec<Carrot2>,
        #[boulder(generatable_with_persian_rug(c3=TestIndexGenerator { value: 3 }), sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (3, context)})]
        p3: Vec<Carrot3<HareState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (4, context)})]
        p4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5=5), sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (5, context)})]
        p5: Vec<Carrot5<HareState>>,
        #[boulder(buildable(c6=6), sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (6, context)})]
        p6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (Carrot7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (7, context)})]
        p7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (8, context)})]
        p8: Vec<Carrot8>,
        #[boulder(sequence_with_persian_rug=|context| {context.get_iter::<Hare>().count(); (9, context)})]
        p9: Vec<Carrot9>,
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct HareState {
        #[table]
        rabbits: Hare,
    }

    #[test]
    fn test_defaults() {
        let mut s = Default::default();

        let (r, _) = Hare::builder().build(&mut s);

        assert_eq!(r.v5.c5, 5);
        assert_eq!(r.v6.c6, 6);
        assert_eq!(r.v7.c7, 7);
        assert_eq!(r.v8.c8, 8);
        assert_eq!(r.v9.c9, 0);

        assert_eq!(r.s1.len(), 1);
        assert_eq!(r.s1[0].c1, 1);
        assert_eq!(r.s2.len(), 2);
        assert_eq!(r.s2[0].c2, 2);
        assert_eq!(r.s2[1].c2, 3);
        assert_eq!(r.s3.len(), 3);
        assert_eq!(r.s3[0].c3, 3);
        assert_eq!(r.s3[1].c3, 4);
        assert_eq!(r.s3[2].c3, 5);
        assert_eq!(r.s4.len(), 4);
        assert_eq!(r.s4[0].c4, 4);
        assert_eq!(r.s4[1].c4, 5);
        assert_eq!(r.s4[2].c4, 6);
        assert_eq!(r.s4[3].c4, 7);
        assert_eq!(r.s5.len(), 5);
        assert_eq!(r.s5[0].c5, 5);
        assert_eq!(r.s5[1].c5, 5);
        assert_eq!(r.s5[2].c5, 5);
        assert_eq!(r.s5[3].c5, 5);
        assert_eq!(r.s5[4].c5, 5);
        assert_eq!(r.s6.len(), 6);
        assert_eq!(r.s6[0].c6, 6);
        assert_eq!(r.s6[1].c6, 6);
        assert_eq!(r.s6[2].c6, 6);
        assert_eq!(r.s6[3].c6, 6);
        assert_eq!(r.s6[4].c6, 6);
        assert_eq!(r.s6[5].c6, 6);
        assert_eq!(r.s7.len(), 7);
        assert_eq!(r.s7[0].c7, 7);
        assert_eq!(r.s7[1].c7, 7);
        assert_eq!(r.s7[2].c7, 7);
        assert_eq!(r.s7[3].c7, 7);
        assert_eq!(r.s7[4].c7, 7);
        assert_eq!(r.s7[5].c7, 7);
        assert_eq!(r.s7[6].c7, 7);
        assert_eq!(r.s8.len(), 8);
        assert_eq!(r.s8[0].c8, 8);
        assert_eq!(r.s8[1].c8, 8);
        assert_eq!(r.s8[2].c8, 8);
        assert_eq!(r.s8[3].c8, 8);
        assert_eq!(r.s8[4].c8, 8);
        assert_eq!(r.s8[5].c8, 8);
        assert_eq!(r.s8[6].c8, 8);
        assert_eq!(r.s8[7].c8, 8);
        assert_eq!(r.s9.len(), 9);
        assert_eq!(r.s9[0].c9, 0);
        assert_eq!(r.s9[1].c9, 0);
        assert_eq!(r.s9[2].c9, 0);
        assert_eq!(r.s9[3].c9, 0);
        assert_eq!(r.s9[4].c9, 0);
        assert_eq!(r.s9[5].c9, 0);
        assert_eq!(r.s9[6].c9, 0);
        assert_eq!(r.s9[7].c9, 0);
        assert_eq!(r.s9[8].c9, 0);

        assert_eq!(r.p1.len(), 1);
        assert_eq!(r.p1[0].c1, 1);
        assert_eq!(r.p2.len(), 2);
        assert_eq!(r.p2[0].c2, 2);
        assert_eq!(r.p2[1].c2, 3);
        assert_eq!(r.p3.len(), 3);
        assert_eq!(r.p3[0].c3, 3);
        assert_eq!(r.p3[1].c3, 4);
        assert_eq!(r.p3[2].c3, 5);
        assert_eq!(r.p4.len(), 4);
        assert_eq!(r.p4[0].c4, 4);
        assert_eq!(r.p4[1].c4, 5);
        assert_eq!(r.p4[2].c4, 6);
        assert_eq!(r.p4[3].c4, 7);
        assert_eq!(r.p5.len(), 5);
        assert_eq!(r.p5[0].c5, 5);
        assert_eq!(r.p5[1].c5, 5);
        assert_eq!(r.p5[2].c5, 5);
        assert_eq!(r.p5[3].c5, 5);
        assert_eq!(r.p5[4].c5, 5);
        assert_eq!(r.p6.len(), 6);
        assert_eq!(r.p6[0].c6, 6);
        assert_eq!(r.p6[1].c6, 6);
        assert_eq!(r.p6[2].c6, 6);
        assert_eq!(r.p6[3].c6, 6);
        assert_eq!(r.p6[4].c6, 6);
        assert_eq!(r.p6[5].c6, 6);
        assert_eq!(r.p7.len(), 7);
        assert_eq!(r.p7[0].c7, 7);
        assert_eq!(r.p7[1].c7, 7);
        assert_eq!(r.p7[2].c7, 7);
        assert_eq!(r.p7[3].c7, 7);
        assert_eq!(r.p7[4].c7, 7);
        assert_eq!(r.p7[5].c7, 7);
        assert_eq!(r.p7[6].c7, 7);
        assert_eq!(r.p8.len(), 8);
        assert_eq!(r.p8[0].c8, 8);
        assert_eq!(r.p8[1].c8, 8);
        assert_eq!(r.p8[2].c8, 8);
        assert_eq!(r.p8[3].c8, 8);
        assert_eq!(r.p8[4].c8, 8);
        assert_eq!(r.p8[5].c8, 8);
        assert_eq!(r.p8[6].c8, 8);
        assert_eq!(r.p8[7].c8, 8);
        assert_eq!(r.p9.len(), 9);
        assert_eq!(r.p9[0].c9, 0);
        assert_eq!(r.p9[1].c9, 0);
        assert_eq!(r.p9[2].c9, 0);
        assert_eq!(r.p9[3].c9, 0);
        assert_eq!(r.p9[4].c9, 0);
        assert_eq!(r.p9[5].c9, 0);
        assert_eq!(r.p9[6].c9, 0);
        assert_eq!(r.p9[7].c9, 0);
        assert_eq!(r.p9[8].c9, 0);
    }

    #[test]
    fn test_customise() {
        let mut s = Default::default();

        let (r, _) = Hare::builder()
            .v5(Carrot5 {
                c5: 55,
                _marker: Default::default(),
            })
            .v6(Carrot6 { c6: 66 })
            .v7(Carrot7 { c7: 77 })
            .v8(Carrot8 { c8: 88 })
            .v9(Carrot9 { c9: 99 })
            .s1(vec![Carrot1 { c1: 11 }])
            .s2(vec![Carrot2 { c2: 22 }])
            .s3(vec![Carrot3 {
                c3: 33,
                _marker: Default::default(),
            }])
            .s4(vec![Carrot4 { c4: 44 }])
            .s5(vec![Carrot5 {
                c5: 55,
                _marker: Default::default(),
            }])
            .s6(vec![Carrot6 { c6: 66 }])
            .s7(vec![Carrot7 { c7: 77 }])
            .s8(vec![Carrot8 { c8: 88 }])
            .s9(vec![Carrot9 { c9: 99 }])
            .p1(vec![Carrot1 { c1: 11 }])
            .p2(vec![Carrot2 { c2: 22 }])
            .p3(vec![Carrot3 {
                c3: 33,
                _marker: Default::default(),
            }])
            .p4(vec![Carrot4 { c4: 44 }])
            .p5(vec![Carrot5 {
                c5: 55,
                _marker: Default::default(),
            }])
            .p6(vec![Carrot6 { c6: 66 }])
            .p7(vec![Carrot7 { c7: 77 }])
            .p8(vec![Carrot8 { c8: 88 }])
            .p9(vec![Carrot9 { c9: 99 }])
            .build(&mut s);

        assert_eq!(r.v5.c5, 55);
        assert_eq!(r.v6.c6, 66);
        assert_eq!(r.v7.c7, 77);
        assert_eq!(r.v8.c8, 88);
        assert_eq!(r.v9.c9, 99);

        assert_eq!(r.s1.len(), 1);
        assert_eq!(r.s1[0].c1, 11);
        assert_eq!(r.s2.len(), 1);
        assert_eq!(r.s2[0].c2, 22);
        assert_eq!(r.s3.len(), 1);
        assert_eq!(r.s3[0].c3, 33);
        assert_eq!(r.s4.len(), 1);
        assert_eq!(r.s4[0].c4, 44);
        assert_eq!(r.s5.len(), 1);
        assert_eq!(r.s5[0].c5, 55);
        assert_eq!(r.s6.len(), 1);
        assert_eq!(r.s6[0].c6, 66);
        assert_eq!(r.s7.len(), 1);
        assert_eq!(r.s7[0].c7, 77);
        assert_eq!(r.s8.len(), 1);
        assert_eq!(r.s8[0].c8, 88);
        assert_eq!(r.s9.len(), 1);
        assert_eq!(r.s9[0].c9, 99);

        assert_eq!(r.p1.len(), 1);
        assert_eq!(r.p1[0].c1, 11);
        assert_eq!(r.p2.len(), 1);
        assert_eq!(r.p2[0].c2, 22);
        assert_eq!(r.p3.len(), 1);
        assert_eq!(r.p3[0].c3, 33);
        assert_eq!(r.p4.len(), 1);
        assert_eq!(r.p4[0].c4, 44);
        assert_eq!(r.p5.len(), 1);
        assert_eq!(r.p5[0].c5, 55);
        assert_eq!(r.p6.len(), 1);
        assert_eq!(r.p6[0].c6, 66);
        assert_eq!(r.p7.len(), 1);
        assert_eq!(r.p7[0].c7, 77);
        assert_eq!(r.p8.len(), 1);
        assert_eq!(r.p8[0].c8, 88);
        assert_eq!(r.p9.len(), 1);
        assert_eq!(r.p9[0].c9, 99);
    }
}

mod generators_basic {
    use boulder::{GeneratableWithPersianRug, GeneratorWithPersianRug};
    use persian_rug::Context;
    use std::any::Any;

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct StateWizards {
        #[table]
        wizards: Wizard<StateWizards>,
    }

    struct TestPersianRugString;

    impl<C> GeneratorWithPersianRug<C> for TestPersianRugString
    where
        C: persian_rug::Context + persian_rug::Owner<Wizard<C>> + 'static,
    {
        type Output = String;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            if let Some(wizard) = context.get_iter().next() {
                return (format!("{}!", wizard.a), context);
            }
            ("hello".to_string(), context)
        }
    }

    #[derive(Debug, GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Wizard<C>)))]
    pub struct Wizard<C: 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = "hullo")]
        a: String,
        #[boulder(generator=boulder::Inc(5))]
        b: i32,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Wizard<C> {
        type Context = C;
    }

    #[test]
    fn test_generator() {
        let mut s = Default::default();
        let mut g = Wizard::<StateWizards>::generator().a(TestPersianRugString);

        let (w, _) = g.generate(&mut s);
        let w = s.add(w);

        let (w2, _) = g.generate(&mut s);

        assert_eq!(
            std::any::TypeId::of::<persian_rug::Proxy<Wizard<StateWizards>>>(),
            w.type_id()
        );
        assert_eq!(std::any::TypeId::of::<Wizard<StateWizards>>(), w2.type_id());

        assert_eq!(s.get(&w).a, "hello".to_string());
        assert_eq!(s.get(&w).b, 5);
        assert_eq!(w2.a, "hello!".to_string());
        assert_eq!(w2.b, 6);
    }
}

mod generator_coverage {
    use boulder::{Buildable, BuildableWithPersianRug, Builder, BuilderWithPersianRug};
    use boulder::{Generatable, GeneratableWithPersianRug, Generator, GeneratorWithPersianRug};

    struct Strawberry1 {
        c1: i32,
    }

    struct Strawberry1Generator {
        c1: i32,
    }

    #[persian_rug::constraints(context=C)]
    impl<C> GeneratorWithPersianRug<C> for Strawberry1Generator {
        type Output = Strawberry1;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + ::persian_rug::Mutator<Context = C>,
        {
            let ix = self.c1;
            self.c1 += 1;

            (Strawberry1 { c1: ix }, context)
        }
    }

    struct Strawberry2 {
        c2: i32,
    }

    struct Strawberry2Generator {
        c2: i32,
    }

    impl Generator for Strawberry2Generator {
        type Output = Strawberry2;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c2;
            self.c2 += 1;
            Strawberry2 { c2: ix }
        }
    }

    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Strawberry3<C: 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(generator=boulder::Inc(3))]
        c3: i32,
    }

    struct TestIndexGenerator {
        value: i32,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestIndexGenerator {
        type Output = i32;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    struct TestUsizeGenerator {
        value: usize,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestUsizeGenerator {
        type Output = usize;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    #[derive(Generatable)]
    struct Strawberry4 {
        #[boulder(generator=boulder::Inc(0))]
        c4: i32,
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Strawberry5<C> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c5: i32,
    }

    #[derive(Buildable)]
    struct Strawberry6 {
        #[boulder(default = 0)]
        c6: i32,
    }

    struct Strawberry7 {
        c7: i32,
    }

    struct Strawberry8 {
        c8: i32,
    }

    #[derive(Default)]
    struct Strawberry9 {
        c9: i32,
    }

    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Donkey<C>)))]
    struct Donkey<C: persian_rug::Context>
    where
        C: 'static,
    {
        _marker: core::marker::PhantomData<C>,
        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator)]
        v1: Strawberry1,
        #[boulder(generator=Strawberry2Generator {c2: 2})]
        v2: Strawberry2,
        #[boulder(generatable_with_persian_rug)]
        v3: Strawberry3<C>,
        #[boulder(generatable(c4=boulder::Inc(4)))]
        v4: Strawberry4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Strawberry5<C>,
        #[boulder(buildable(c6 = 6))]
        v6: Strawberry6,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (Strawberry7 { c7: 7 }, context) })]
        v7: Strawberry7,
        #[boulder(default=Strawberry8 { c8: 8 })]
        v8: Strawberry8,
        v9: Strawberry9,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence=1usize)]
        s1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence=2usize)]
        s2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence = 3usize)]
        s3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence=4usize)]
        s4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5usize)]
        s5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence = 6usize)]
        s6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (Strawberry7 { c7: 7 }, context) }, sequence=7usize)]
        s7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence=8usize)]
        s8: Vec<Strawberry8>,
        #[boulder(sequence = 9usize)]
        s9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (1usize, context)})]
        p1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (2usize, context)})]
        p2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (3usize, context)})]
        p3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (4usize, context)})]
        p4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (5usize, context)})]
        p5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (6usize, context)})]
        p6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (7usize, context)})]
        p7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (8usize, context)})]
        p8: Vec<Strawberry8>,
        #[boulder(sequence_with_persian_rug=|context| {context.get_iter::<Donkey<_>>().count(); (9usize, context)})]
        p9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator=boulder::Inc(1usize))]
        t1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator=boulder::Inc(2usize))]
        t2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator=boulder::Inc(3usize))]
        t3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_generator=boulder::Inc(4usize))]
        t4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_generator = boulder::Inc(5usize))]
        t5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence_generator = boulder::Inc(6usize))]
        t6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Donkey<_>>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_generator=boulder::Inc(7usize))]
        t7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_generator=boulder::Inc(8usize))]
        t8: Vec<Strawberry8>,
        #[boulder(sequence_generator = boulder::Inc(9usize))]
        t9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 1usize }: TestUsizeGenerator)]
        q1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 2usize }: TestUsizeGenerator)]
        q2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 3usize }: TestUsizeGenerator)]
        q3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 4usize }: TestUsizeGenerator)]
        q4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 5usize }: TestUsizeGenerator)]
        q5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 6usize }: TestUsizeGenerator)]
        q6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { context.get_iter::<Donkey<_>>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 7usize }: TestUsizeGenerator)]
        q7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 8usize }: TestUsizeGenerator)]
        q8: Vec<Strawberry8>,
        #[boulder(sequence_generator_with_persian_rug=TestUsizeGenerator { value: 9usize }: TestUsizeGenerator)]
        q9: Vec<Strawberry9>,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Donkey<C> {
        type Context = C;
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct DonkeyState {
        #[table]
        donkeys: Donkey<DonkeyState>,
    }

    #[test]
    fn test_defaults() {
        let mut s: DonkeyState = Default::default();

        let mut g = Donkey::generator();

        let (d1, _) = g.generate(&mut s);
        let (d2, _) = g.generate(&mut s);

        assert_eq!(d1.v1.c1, 1);
        assert_eq!(d1.v2.c2, 2);
        assert_eq!(d1.v3.c3, 3);
        assert_eq!(d1.v4.c4, 4);
        assert_eq!(d1.v5.c5, 5);
        assert_eq!(d1.v6.c6, 6);
        assert_eq!(d1.v7.c7, 7);
        assert_eq!(d1.v8.c8, 8);
        assert_eq!(d1.v9.c9, 0);
        assert_eq!(d2.v1.c1, 2);
        assert_eq!(d2.v2.c2, 3);
        assert_eq!(d2.v3.c3, 4);
        assert_eq!(d2.v4.c4, 5);
        assert_eq!(d2.v5.c5, 5);
        assert_eq!(d2.v6.c6, 6);
        assert_eq!(d2.v7.c7, 7);
        assert_eq!(d2.v8.c8, 8);
        assert_eq!(d2.v9.c9, 0);

        assert_eq!(d1.s1.len(), 1);
        assert_eq!(d1.s1[0].c1, 1);
        assert_eq!(d1.s2.len(), 2);
        assert_eq!(d1.s2[0].c2, 2);
        assert_eq!(d1.s2[1].c2, 3);
        assert_eq!(d1.s3.len(), 3);
        assert_eq!(d1.s3[0].c3, 3);
        assert_eq!(d1.s3[1].c3, 4);
        assert_eq!(d1.s3[2].c3, 5);
        assert_eq!(d1.s4.len(), 4);
        assert_eq!(d1.s4[0].c4, 4);
        assert_eq!(d1.s4[1].c4, 5);
        assert_eq!(d1.s4[2].c4, 6);
        assert_eq!(d1.s4[3].c4, 7);
        assert_eq!(d1.s5.len(), 5);
        assert_eq!(d1.s5[0].c5, 5);
        assert_eq!(d1.s5[1].c5, 5);
        assert_eq!(d1.s5[2].c5, 5);
        assert_eq!(d1.s5[3].c5, 5);
        assert_eq!(d1.s5[4].c5, 5);
        assert_eq!(d1.s6.len(), 6);
        assert_eq!(d1.s6[0].c6, 6);
        assert_eq!(d1.s6[1].c6, 6);
        assert_eq!(d1.s6[2].c6, 6);
        assert_eq!(d1.s6[3].c6, 6);
        assert_eq!(d1.s6[4].c6, 6);
        assert_eq!(d1.s6[5].c6, 6);
        assert_eq!(d1.s7.len(), 7);
        assert_eq!(d1.s7[0].c7, 7);
        assert_eq!(d1.s7[1].c7, 7);
        assert_eq!(d1.s7[2].c7, 7);
        assert_eq!(d1.s7[3].c7, 7);
        assert_eq!(d1.s7[4].c7, 7);
        assert_eq!(d1.s7[5].c7, 7);
        assert_eq!(d1.s7[6].c7, 7);
        assert_eq!(d1.s8.len(), 8);
        assert_eq!(d1.s8[0].c8, 8);
        assert_eq!(d1.s8[1].c8, 8);
        assert_eq!(d1.s8[2].c8, 8);
        assert_eq!(d1.s8[3].c8, 8);
        assert_eq!(d1.s8[4].c8, 8);
        assert_eq!(d1.s8[5].c8, 8);
        assert_eq!(d1.s8[6].c8, 8);
        assert_eq!(d1.s8[7].c8, 8);
        assert_eq!(d1.s9.len(), 9);
        assert_eq!(d1.s9[0].c9, 0);
        assert_eq!(d1.s9[1].c9, 0);
        assert_eq!(d1.s9[2].c9, 0);
        assert_eq!(d1.s9[3].c9, 0);
        assert_eq!(d1.s9[4].c9, 0);
        assert_eq!(d1.s9[5].c9, 0);
        assert_eq!(d1.s9[6].c9, 0);
        assert_eq!(d1.s9[7].c9, 0);
        assert_eq!(d1.s9[8].c9, 0);
        assert_eq!(d2.s1.len(), 1);
        assert_eq!(d2.s1[0].c1, 2);
        assert_eq!(d2.s2.len(), 2);
        assert_eq!(d2.s2[0].c2, 4);
        assert_eq!(d2.s2[1].c2, 5);
        assert_eq!(d2.s3.len(), 3);
        assert_eq!(d2.s3[0].c3, 6);
        assert_eq!(d2.s3[1].c3, 7);
        assert_eq!(d2.s3[2].c3, 8);
        assert_eq!(d2.s4.len(), 4);
        assert_eq!(d2.s4[0].c4, 8);
        assert_eq!(d2.s4[1].c4, 9);
        assert_eq!(d2.s4[2].c4, 10);
        assert_eq!(d2.s4[3].c4, 11);
        assert_eq!(d2.s5.len(), 5);
        assert_eq!(d2.s5[0].c5, 5);
        assert_eq!(d2.s5[1].c5, 5);
        assert_eq!(d2.s5[2].c5, 5);
        assert_eq!(d2.s5[3].c5, 5);
        assert_eq!(d2.s5[4].c5, 5);
        assert_eq!(d2.s6.len(), 6);
        assert_eq!(d2.s6[0].c6, 6);
        assert_eq!(d2.s6[1].c6, 6);
        assert_eq!(d2.s6[2].c6, 6);
        assert_eq!(d2.s6[3].c6, 6);
        assert_eq!(d2.s6[4].c6, 6);
        assert_eq!(d2.s6[5].c6, 6);
        assert_eq!(d2.s7.len(), 7);
        assert_eq!(d2.s7[0].c7, 7);
        assert_eq!(d2.s7[1].c7, 7);
        assert_eq!(d2.s7[2].c7, 7);
        assert_eq!(d2.s7[3].c7, 7);
        assert_eq!(d2.s7[4].c7, 7);
        assert_eq!(d2.s7[5].c7, 7);
        assert_eq!(d2.s7[6].c7, 7);
        assert_eq!(d2.s8.len(), 8);
        assert_eq!(d2.s8[0].c8, 8);
        assert_eq!(d2.s8[1].c8, 8);
        assert_eq!(d2.s8[2].c8, 8);
        assert_eq!(d2.s8[3].c8, 8);
        assert_eq!(d2.s8[4].c8, 8);
        assert_eq!(d2.s8[5].c8, 8);
        assert_eq!(d2.s8[6].c8, 8);
        assert_eq!(d2.s8[7].c8, 8);
        assert_eq!(d2.s9.len(), 9);
        assert_eq!(d2.s9[0].c9, 0);
        assert_eq!(d2.s9[1].c9, 0);
        assert_eq!(d2.s9[2].c9, 0);
        assert_eq!(d2.s9[3].c9, 0);
        assert_eq!(d2.s9[4].c9, 0);
        assert_eq!(d2.s9[5].c9, 0);
        assert_eq!(d2.s9[6].c9, 0);
        assert_eq!(d2.s9[7].c9, 0);
        assert_eq!(d2.s9[8].c9, 0);

        assert_eq!(d1.p1.len(), 1);
        assert_eq!(d1.p1[0].c1, 1);
        assert_eq!(d1.p2.len(), 2);
        assert_eq!(d1.p2[0].c2, 2);
        assert_eq!(d1.p2[1].c2, 3);
        assert_eq!(d1.p3.len(), 3);
        assert_eq!(d1.p3[0].c3, 3);
        assert_eq!(d1.p3[1].c3, 4);
        assert_eq!(d1.p3[2].c3, 5);
        assert_eq!(d1.p4.len(), 4);
        assert_eq!(d1.p4[0].c4, 4);
        assert_eq!(d1.p4[1].c4, 5);
        assert_eq!(d1.p4[2].c4, 6);
        assert_eq!(d1.p4[3].c4, 7);
        assert_eq!(d1.p5.len(), 5);
        assert_eq!(d1.p5[0].c5, 5);
        assert_eq!(d1.p5[1].c5, 5);
        assert_eq!(d1.p5[2].c5, 5);
        assert_eq!(d1.p5[3].c5, 5);
        assert_eq!(d1.p5[4].c5, 5);
        assert_eq!(d1.p6.len(), 6);
        assert_eq!(d1.p6[0].c6, 6);
        assert_eq!(d1.p6[1].c6, 6);
        assert_eq!(d1.p6[2].c6, 6);
        assert_eq!(d1.p6[3].c6, 6);
        assert_eq!(d1.p6[4].c6, 6);
        assert_eq!(d1.p6[5].c6, 6);
        assert_eq!(d1.p7.len(), 7);
        assert_eq!(d1.p7[0].c7, 7);
        assert_eq!(d1.p7[1].c7, 7);
        assert_eq!(d1.p7[2].c7, 7);
        assert_eq!(d1.p7[3].c7, 7);
        assert_eq!(d1.p7[4].c7, 7);
        assert_eq!(d1.p7[5].c7, 7);
        assert_eq!(d1.p7[6].c7, 7);
        assert_eq!(d1.p8.len(), 8);
        assert_eq!(d1.p8[0].c8, 8);
        assert_eq!(d1.p8[1].c8, 8);
        assert_eq!(d1.p8[2].c8, 8);
        assert_eq!(d1.p8[3].c8, 8);
        assert_eq!(d1.p8[4].c8, 8);
        assert_eq!(d1.p8[5].c8, 8);
        assert_eq!(d1.p8[6].c8, 8);
        assert_eq!(d1.p8[7].c8, 8);
        assert_eq!(d1.p9.len(), 9);
        assert_eq!(d1.p9[0].c9, 0);
        assert_eq!(d1.p9[1].c9, 0);
        assert_eq!(d1.p9[2].c9, 0);
        assert_eq!(d1.p9[3].c9, 0);
        assert_eq!(d1.p9[4].c9, 0);
        assert_eq!(d1.p9[5].c9, 0);
        assert_eq!(d1.p9[6].c9, 0);
        assert_eq!(d1.p9[7].c9, 0);
        assert_eq!(d1.p9[8].c9, 0);
        assert_eq!(d2.p1.len(), 1);
        assert_eq!(d2.p1[0].c1, 2);
        assert_eq!(d2.p2.len(), 2);
        assert_eq!(d2.p2[0].c2, 4);
        assert_eq!(d2.p2[1].c2, 5);
        assert_eq!(d2.p3.len(), 3);
        assert_eq!(d2.p3[0].c3, 6);
        assert_eq!(d2.p3[1].c3, 7);
        assert_eq!(d2.p3[2].c3, 8);
        assert_eq!(d2.p4.len(), 4);
        assert_eq!(d2.p4[0].c4, 8);
        assert_eq!(d2.p4[1].c4, 9);
        assert_eq!(d2.p4[2].c4, 10);
        assert_eq!(d2.p4[3].c4, 11);
        assert_eq!(d2.p5.len(), 5);
        assert_eq!(d2.p5[0].c5, 5);
        assert_eq!(d2.p5[1].c5, 5);
        assert_eq!(d2.p5[2].c5, 5);
        assert_eq!(d2.p5[3].c5, 5);
        assert_eq!(d2.p5[4].c5, 5);
        assert_eq!(d2.p6.len(), 6);
        assert_eq!(d2.p6[0].c6, 6);
        assert_eq!(d2.p6[1].c6, 6);
        assert_eq!(d2.p6[2].c6, 6);
        assert_eq!(d2.p6[3].c6, 6);
        assert_eq!(d2.p6[4].c6, 6);
        assert_eq!(d2.p6[5].c6, 6);
        assert_eq!(d2.p7.len(), 7);
        assert_eq!(d2.p7[0].c7, 7);
        assert_eq!(d2.p7[1].c7, 7);
        assert_eq!(d2.p7[2].c7, 7);
        assert_eq!(d2.p7[3].c7, 7);
        assert_eq!(d2.p7[4].c7, 7);
        assert_eq!(d2.p7[5].c7, 7);
        assert_eq!(d2.p7[6].c7, 7);
        assert_eq!(d2.p8.len(), 8);
        assert_eq!(d2.p8[0].c8, 8);
        assert_eq!(d2.p8[1].c8, 8);
        assert_eq!(d2.p8[2].c8, 8);
        assert_eq!(d2.p8[3].c8, 8);
        assert_eq!(d2.p8[4].c8, 8);
        assert_eq!(d2.p8[5].c8, 8);
        assert_eq!(d2.p8[6].c8, 8);
        assert_eq!(d2.p8[7].c8, 8);
        assert_eq!(d2.p9.len(), 9);
        assert_eq!(d2.p9[0].c9, 0);
        assert_eq!(d2.p9[1].c9, 0);
        assert_eq!(d2.p9[2].c9, 0);
        assert_eq!(d2.p9[3].c9, 0);
        assert_eq!(d2.p9[4].c9, 0);
        assert_eq!(d2.p9[5].c9, 0);
        assert_eq!(d2.p9[6].c9, 0);
        assert_eq!(d2.p9[7].c9, 0);
        assert_eq!(d2.p9[8].c9, 0);

        assert_eq!(d1.t1.len(), 1);
        assert_eq!(d1.t1[0].c1, 1);
        assert_eq!(d1.t2.len(), 2);
        assert_eq!(d1.t2[0].c2, 2);
        assert_eq!(d1.t2[1].c2, 3);
        assert_eq!(d1.t3.len(), 3);
        assert_eq!(d1.t3[0].c3, 3);
        assert_eq!(d1.t3[1].c3, 4);
        assert_eq!(d1.t3[2].c3, 5);
        assert_eq!(d1.t4.len(), 4);
        assert_eq!(d1.t4[0].c4, 4);
        assert_eq!(d1.t4[1].c4, 5);
        assert_eq!(d1.t4[2].c4, 6);
        assert_eq!(d1.t4[3].c4, 7);
        assert_eq!(d1.t5.len(), 5);
        assert_eq!(d1.t5[0].c5, 5);
        assert_eq!(d1.t5[1].c5, 5);
        assert_eq!(d1.t5[2].c5, 5);
        assert_eq!(d1.t5[3].c5, 5);
        assert_eq!(d1.t5[4].c5, 5);
        assert_eq!(d1.t6.len(), 6);
        assert_eq!(d1.t6[0].c6, 6);
        assert_eq!(d1.t6[1].c6, 6);
        assert_eq!(d1.t6[2].c6, 6);
        assert_eq!(d1.t6[3].c6, 6);
        assert_eq!(d1.t6[4].c6, 6);
        assert_eq!(d1.t6[5].c6, 6);
        assert_eq!(d1.t7.len(), 7);
        assert_eq!(d1.t7[0].c7, 7);
        assert_eq!(d1.t7[1].c7, 7);
        assert_eq!(d1.t7[2].c7, 7);
        assert_eq!(d1.t7[3].c7, 7);
        assert_eq!(d1.t7[4].c7, 7);
        assert_eq!(d1.t7[5].c7, 7);
        assert_eq!(d1.t7[6].c7, 7);
        assert_eq!(d1.t8.len(), 8);
        assert_eq!(d1.t8[0].c8, 8);
        assert_eq!(d1.t8[1].c8, 8);
        assert_eq!(d1.t8[2].c8, 8);
        assert_eq!(d1.t8[3].c8, 8);
        assert_eq!(d1.t8[4].c8, 8);
        assert_eq!(d1.t8[5].c8, 8);
        assert_eq!(d1.t8[6].c8, 8);
        assert_eq!(d1.t8[7].c8, 8);
        assert_eq!(d1.t9.len(), 9);
        assert_eq!(d1.t9[0].c9, 0);
        assert_eq!(d1.t9[1].c9, 0);
        assert_eq!(d1.t9[2].c9, 0);
        assert_eq!(d1.t9[3].c9, 0);
        assert_eq!(d1.t9[4].c9, 0);
        assert_eq!(d1.t9[5].c9, 0);
        assert_eq!(d1.t9[6].c9, 0);
        assert_eq!(d1.t9[7].c9, 0);
        assert_eq!(d1.t9[8].c9, 0);
        assert_eq!(d2.t1.len(), 2);
        assert_eq!(d2.t1[0].c1, 2);
        assert_eq!(d2.t1[1].c1, 3);
        assert_eq!(d2.t2.len(), 3);
        assert_eq!(d2.t2[0].c2, 4);
        assert_eq!(d2.t2[1].c2, 5);
        assert_eq!(d2.t2[2].c2, 6);
        assert_eq!(d2.t3.len(), 4);
        assert_eq!(d2.t3[0].c3, 6);
        assert_eq!(d2.t3[1].c3, 7);
        assert_eq!(d2.t3[2].c3, 8);
        assert_eq!(d2.t3[3].c3, 9);
        assert_eq!(d2.t4.len(), 5);
        assert_eq!(d2.t4[0].c4, 8);
        assert_eq!(d2.t4[1].c4, 9);
        assert_eq!(d2.t4[2].c4, 10);
        assert_eq!(d2.t4[3].c4, 11);
        assert_eq!(d2.t4[4].c4, 12);
        assert_eq!(d2.t5.len(), 6);
        assert_eq!(d2.t5[0].c5, 5);
        assert_eq!(d2.t5[1].c5, 5);
        assert_eq!(d2.t5[2].c5, 5);
        assert_eq!(d2.t5[3].c5, 5);
        assert_eq!(d2.t5[4].c5, 5);
        assert_eq!(d2.t5[5].c5, 5);
        assert_eq!(d2.t6.len(), 7);
        assert_eq!(d2.t6[0].c6, 6);
        assert_eq!(d2.t6[1].c6, 6);
        assert_eq!(d2.t6[2].c6, 6);
        assert_eq!(d2.t6[3].c6, 6);
        assert_eq!(d2.t6[4].c6, 6);
        assert_eq!(d2.t6[5].c6, 6);
        assert_eq!(d2.t6[6].c6, 6);
        assert_eq!(d2.t7.len(), 8);
        assert_eq!(d2.t7[0].c7, 7);
        assert_eq!(d2.t7[1].c7, 7);
        assert_eq!(d2.t7[2].c7, 7);
        assert_eq!(d2.t7[3].c7, 7);
        assert_eq!(d2.t7[4].c7, 7);
        assert_eq!(d2.t7[5].c7, 7);
        assert_eq!(d2.t7[6].c7, 7);
        assert_eq!(d2.t7[7].c7, 7);
        assert_eq!(d2.t8.len(), 9);
        assert_eq!(d2.t8[0].c8, 8);
        assert_eq!(d2.t8[1].c8, 8);
        assert_eq!(d2.t8[2].c8, 8);
        assert_eq!(d2.t8[3].c8, 8);
        assert_eq!(d2.t8[4].c8, 8);
        assert_eq!(d2.t8[5].c8, 8);
        assert_eq!(d2.t8[6].c8, 8);
        assert_eq!(d2.t8[7].c8, 8);
        assert_eq!(d2.t8[8].c8, 8);
        assert_eq!(d2.t9.len(), 10);
        assert_eq!(d2.t9[0].c9, 0);
        assert_eq!(d2.t9[1].c9, 0);
        assert_eq!(d2.t9[2].c9, 0);
        assert_eq!(d2.t9[3].c9, 0);
        assert_eq!(d2.t9[4].c9, 0);
        assert_eq!(d2.t9[5].c9, 0);
        assert_eq!(d2.t9[6].c9, 0);
        assert_eq!(d2.t9[7].c9, 0);
        assert_eq!(d2.t9[8].c9, 0);
        assert_eq!(d2.t9[9].c9, 0);

        assert_eq!(d1.q1.len(), 1);
        assert_eq!(d1.q1[0].c1, 1);
        assert_eq!(d1.q2.len(), 2);
        assert_eq!(d1.q2[0].c2, 2);
        assert_eq!(d1.q2[1].c2, 3);
        assert_eq!(d1.q3.len(), 3);
        assert_eq!(d1.q3[0].c3, 3);
        assert_eq!(d1.q3[1].c3, 4);
        assert_eq!(d1.q3[2].c3, 5);
        assert_eq!(d1.q4.len(), 4);
        assert_eq!(d1.q4[0].c4, 4);
        assert_eq!(d1.q4[1].c4, 5);
        assert_eq!(d1.q4[2].c4, 6);
        assert_eq!(d1.q4[3].c4, 7);
        assert_eq!(d1.q5.len(), 5);
        assert_eq!(d1.q5[0].c5, 5);
        assert_eq!(d1.q5[1].c5, 5);
        assert_eq!(d1.q5[2].c5, 5);
        assert_eq!(d1.q5[3].c5, 5);
        assert_eq!(d1.q5[4].c5, 5);
        assert_eq!(d1.q6.len(), 6);
        assert_eq!(d1.q6[0].c6, 6);
        assert_eq!(d1.q6[1].c6, 6);
        assert_eq!(d1.q6[2].c6, 6);
        assert_eq!(d1.q6[3].c6, 6);
        assert_eq!(d1.q6[4].c6, 6);
        assert_eq!(d1.q6[5].c6, 6);
        assert_eq!(d1.q7.len(), 7);
        assert_eq!(d1.q7[0].c7, 7);
        assert_eq!(d1.q7[1].c7, 7);
        assert_eq!(d1.q7[2].c7, 7);
        assert_eq!(d1.q7[3].c7, 7);
        assert_eq!(d1.q7[4].c7, 7);
        assert_eq!(d1.q7[5].c7, 7);
        assert_eq!(d1.q7[6].c7, 7);
        assert_eq!(d1.q8.len(), 8);
        assert_eq!(d1.q8[0].c8, 8);
        assert_eq!(d1.q8[1].c8, 8);
        assert_eq!(d1.q8[2].c8, 8);
        assert_eq!(d1.q8[3].c8, 8);
        assert_eq!(d1.q8[4].c8, 8);
        assert_eq!(d1.q8[5].c8, 8);
        assert_eq!(d1.q8[6].c8, 8);
        assert_eq!(d1.q8[7].c8, 8);
        assert_eq!(d1.q9.len(), 9);
        assert_eq!(d1.q9[0].c9, 0);
        assert_eq!(d1.q9[1].c9, 0);
        assert_eq!(d1.q9[2].c9, 0);
        assert_eq!(d1.q9[3].c9, 0);
        assert_eq!(d1.q9[4].c9, 0);
        assert_eq!(d1.q9[5].c9, 0);
        assert_eq!(d1.q9[6].c9, 0);
        assert_eq!(d1.q9[7].c9, 0);
        assert_eq!(d1.q9[8].c9, 0);
        assert_eq!(d2.q1.len(), 2);
        assert_eq!(d2.q1[0].c1, 2);
        assert_eq!(d2.q1[1].c1, 3);
        assert_eq!(d2.q2.len(), 3);
        assert_eq!(d2.q2[0].c2, 4);
        assert_eq!(d2.q2[1].c2, 5);
        assert_eq!(d2.q2[2].c2, 6);
        assert_eq!(d2.q3.len(), 4);
        assert_eq!(d2.q3[0].c3, 6);
        assert_eq!(d2.q3[1].c3, 7);
        assert_eq!(d2.q3[2].c3, 8);
        assert_eq!(d2.q3[3].c3, 9);
        assert_eq!(d2.q4.len(), 5);
        assert_eq!(d2.q4[0].c4, 8);
        assert_eq!(d2.q4[1].c4, 9);
        assert_eq!(d2.q4[2].c4, 10);
        assert_eq!(d2.q4[3].c4, 11);
        assert_eq!(d2.q4[4].c4, 12);
        assert_eq!(d2.q5.len(), 6);
        assert_eq!(d2.q5[0].c5, 5);
        assert_eq!(d2.q5[1].c5, 5);
        assert_eq!(d2.q5[2].c5, 5);
        assert_eq!(d2.q5[3].c5, 5);
        assert_eq!(d2.q5[4].c5, 5);
        assert_eq!(d2.q5[5].c5, 5);
        assert_eq!(d2.q6.len(), 7);
        assert_eq!(d2.q6[0].c6, 6);
        assert_eq!(d2.q6[1].c6, 6);
        assert_eq!(d2.q6[2].c6, 6);
        assert_eq!(d2.q6[3].c6, 6);
        assert_eq!(d2.q6[4].c6, 6);
        assert_eq!(d2.q6[5].c6, 6);
        assert_eq!(d2.q6[6].c6, 6);
        assert_eq!(d2.q7.len(), 8);
        assert_eq!(d2.q7[0].c7, 7);
        assert_eq!(d2.q7[1].c7, 7);
        assert_eq!(d2.q7[2].c7, 7);
        assert_eq!(d2.q7[3].c7, 7);
        assert_eq!(d2.q7[4].c7, 7);
        assert_eq!(d2.q7[5].c7, 7);
        assert_eq!(d2.q7[6].c7, 7);
        assert_eq!(d2.q7[7].c7, 7);
        assert_eq!(d2.q8.len(), 9);
        assert_eq!(d2.q8[0].c8, 8);
        assert_eq!(d2.q8[1].c8, 8);
        assert_eq!(d2.q8[2].c8, 8);
        assert_eq!(d2.q8[3].c8, 8);
        assert_eq!(d2.q8[4].c8, 8);
        assert_eq!(d2.q8[5].c8, 8);
        assert_eq!(d2.q8[6].c8, 8);
        assert_eq!(d2.q8[7].c8, 8);
        assert_eq!(d2.q8[8].c8, 8);
        assert_eq!(d2.q9.len(), 10);
        assert_eq!(d2.q9[0].c9, 0);
        assert_eq!(d2.q9[1].c9, 0);
        assert_eq!(d2.q9[2].c9, 0);
        assert_eq!(d2.q9[3].c9, 0);
        assert_eq!(d2.q9[4].c9, 0);
        assert_eq!(d2.q9[5].c9, 0);
        assert_eq!(d2.q9[6].c9, 0);
        assert_eq!(d2.q9[7].c9, 0);
        assert_eq!(d2.q9[8].c9, 0);
        assert_eq!(d2.q9[9].c9, 0);
    }

    #[test]
    fn test_customise() {
        use boulder::GeneratorToGeneratorWithPersianRugWrapper as GeneratorWrapper;
        let mut s: DonkeyState = Default::default();

        let mut g = Donkey::generator()
            .v1(GeneratorWrapper::new(|| Strawberry1 { c1: 11 }))
            .v2(GeneratorWrapper::new(|| Strawberry2 { c2: 22 }))
            .v3(GeneratorWrapper::new(|| Strawberry3 {
                c3: 33,
                _marker: Default::default(),
            }))
            .v4(GeneratorWrapper::new(|| Strawberry4 { c4: 44 }))
            .v5(GeneratorWrapper::new(|| Strawberry5 {
                c5: 55,
                _marker: Default::default(),
            }))
            .v6(GeneratorWrapper::new(|| Strawberry6 { c6: 66 }))
            .v7(GeneratorWrapper::new(|| Strawberry7 { c7: 77 }))
            .v8(GeneratorWrapper::new(|| Strawberry8 { c8: 88 }))
            .v9(GeneratorWrapper::new(|| Strawberry9 { c9: 99 }))
            .s1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .s2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .s3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .s4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .s5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .s6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .s7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .s8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .s9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .p1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .p2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .p3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .p4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .p5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .p6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .p7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .p8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .p9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .t1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .t2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .t3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .t4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .t5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .t6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .t7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .t8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .t9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .q1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .q2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .q3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .q4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .q5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .q6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .q7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .q8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .q9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]));

        let (d, _) = g.generate(&mut s);

        assert_eq!(d.v1.c1, 11);
        assert_eq!(d.v2.c2, 22);
        assert_eq!(d.v3.c3, 33);
        assert_eq!(d.v4.c4, 44);
        assert_eq!(d.v5.c5, 55);
        assert_eq!(d.v6.c6, 66);
        assert_eq!(d.v7.c7, 77);
        assert_eq!(d.v8.c8, 88);
        assert_eq!(d.v9.c9, 99);

        assert_eq!(d.s1.len(), 1);
        assert_eq!(d.s1[0].c1, 11);
        assert_eq!(d.s2.len(), 1);
        assert_eq!(d.s2[0].c2, 22);
        assert_eq!(d.s3.len(), 1);
        assert_eq!(d.s3[0].c3, 33);
        assert_eq!(d.s4.len(), 1);
        assert_eq!(d.s4[0].c4, 44);
        assert_eq!(d.s5.len(), 1);
        assert_eq!(d.s5[0].c5, 55);
        assert_eq!(d.s6.len(), 1);
        assert_eq!(d.s6[0].c6, 66);
        assert_eq!(d.s7.len(), 1);
        assert_eq!(d.s7[0].c7, 77);
        assert_eq!(d.s8.len(), 1);
        assert_eq!(d.s8[0].c8, 88);
        assert_eq!(d.s9.len(), 1);
        assert_eq!(d.s9[0].c9, 99);

        assert_eq!(d.p1.len(), 1);
        assert_eq!(d.p1[0].c1, 11);
        assert_eq!(d.p2.len(), 1);
        assert_eq!(d.p2[0].c2, 22);
        assert_eq!(d.p3.len(), 1);
        assert_eq!(d.p3[0].c3, 33);
        assert_eq!(d.p4.len(), 1);
        assert_eq!(d.p4[0].c4, 44);
        assert_eq!(d.p5.len(), 1);
        assert_eq!(d.p5[0].c5, 55);
        assert_eq!(d.p6.len(), 1);
        assert_eq!(d.p6[0].c6, 66);
        assert_eq!(d.p7.len(), 1);
        assert_eq!(d.p7[0].c7, 77);
        assert_eq!(d.p8.len(), 1);
        assert_eq!(d.p8[0].c8, 88);
        assert_eq!(d.p9.len(), 1);
        assert_eq!(d.p9[0].c9, 99);

        assert_eq!(d.t1.len(), 1);
        assert_eq!(d.t1[0].c1, 11);
        assert_eq!(d.t2.len(), 1);
        assert_eq!(d.t2[0].c2, 22);
        assert_eq!(d.t3.len(), 1);
        assert_eq!(d.t3[0].c3, 33);
        assert_eq!(d.t4.len(), 1);
        assert_eq!(d.t4[0].c4, 44);
        assert_eq!(d.t5.len(), 1);
        assert_eq!(d.t5[0].c5, 55);
        assert_eq!(d.t6.len(), 1);
        assert_eq!(d.t6[0].c6, 66);
        assert_eq!(d.t7.len(), 1);
        assert_eq!(d.t7[0].c7, 77);
        assert_eq!(d.t8.len(), 1);
        assert_eq!(d.t8[0].c8, 88);
        assert_eq!(d.t9.len(), 1);
        assert_eq!(d.t9[0].c9, 99);

        assert_eq!(d.q1.len(), 1);
        assert_eq!(d.q1[0].c1, 11);
        assert_eq!(d.q2.len(), 1);
        assert_eq!(d.q2[0].c2, 22);
        assert_eq!(d.q3.len(), 1);
        assert_eq!(d.q3[0].c3, 33);
        assert_eq!(d.q4.len(), 1);
        assert_eq!(d.q4[0].c4, 44);
        assert_eq!(d.q5.len(), 1);
        assert_eq!(d.q5[0].c5, 55);
        assert_eq!(d.q6.len(), 1);
        assert_eq!(d.q6[0].c6, 66);
        assert_eq!(d.q7.len(), 1);
        assert_eq!(d.q7[0].c7, 77);
        assert_eq!(d.q8.len(), 1);
        assert_eq!(d.q8[0].c8, 88);
        assert_eq!(d.q9.len(), 1);
        assert_eq!(d.q9[0].c9, 99);
    }
}

mod generator_coverage_no_generics {
    use boulder::{Buildable, BuildableWithPersianRug, Builder, BuilderWithPersianRug};
    use boulder::{Generatable, GeneratableWithPersianRug, Generator, GeneratorWithPersianRug};

    struct Strawberry1 {
        c1: i32,
    }

    struct Strawberry1Generator {
        c1: i32,
    }

    #[persian_rug::constraints(context=C)]
    impl<C> GeneratorWithPersianRug<C> for Strawberry1Generator {
        type Output = Strawberry1;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + ::persian_rug::Mutator<Context = C>,
        {
            let ix = self.c1;
            self.c1 += 1;

            (Strawberry1 { c1: ix }, context)
        }
    }

    struct Strawberry2 {
        c2: i32,
    }

    struct Strawberry2Generator {
        c2: i32,
    }

    impl Generator for Strawberry2Generator {
        type Output = Strawberry2;
        fn generate(&mut self) -> Self::Output {
            let ix = self.c2;
            self.c2 += 1;
            Strawberry2 { c2: ix }
        }
    }

    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Strawberry3<C: 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(generator=boulder::Inc(3))]
        c3: i32,
    }

    struct TestIndexGenerator {
        value: i32,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestIndexGenerator {
        type Output = i32;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    struct TestUsizeGenerator {
        value: usize,
    }

    impl<C: persian_rug::Context + 'static> GeneratorWithPersianRug<C> for TestUsizeGenerator {
        type Output = usize;
        fn generate<'b, B>(&mut self, context: B) -> (Self::Output, B)
        where
            B: 'b + persian_rug::Mutator<Context = C>,
        {
            let ix = self.value;
            self.value += 1;
            (ix, context)
        }
    }

    #[derive(Generatable)]
    struct Strawberry4 {
        #[boulder(generator=boulder::Inc(0))]
        c4: i32,
    }

    #[derive(BuildableWithPersianRug)]
    #[boulder(persian_rug(context=C))]
    struct Strawberry5<C> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(default = 0)]
        c5: i32,
    }

    #[derive(Buildable)]
    struct Strawberry6 {
        #[boulder(default = 0)]
        c6: i32,
    }

    struct Strawberry7 {
        c7: i32,
    }

    struct Strawberry8 {
        c8: i32,
    }

    #[derive(Default)]
    struct Strawberry9 {
        c9: i32,
    }

    #[persian_rug::contextual(MuleState)]
    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=MuleState, access(Mule)))]
    struct Mule {
        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator)]
        v1: Strawberry1,
        #[boulder(generator=Strawberry2Generator {c2: 2})]
        v2: Strawberry2,
        #[boulder(generatable_with_persian_rug)]
        v3: Strawberry3<MuleState>,
        #[boulder(generatable(c4=boulder::Inc(4)))]
        v4: Strawberry4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Strawberry5<MuleState>,
        #[boulder(buildable(c6 = 6))]
        v6: Strawberry6,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (Strawberry7 { c7: 7 }, context) })]
        v7: Strawberry7,
        #[boulder(default=Strawberry8 { c8: 8 })]
        v8: Strawberry8,
        v9: Strawberry9,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence=1usize)]
        s1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence=2usize)]
        s2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence = 3usize)]
        s3: Vec<Strawberry3<MuleState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence=4usize)]
        s4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5usize)]
        s5: Vec<Strawberry5<MuleState>>,
        #[boulder(buildable(c6 = 6), sequence = 6usize)]
        s6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (Strawberry7 { c7: 7 }, context) }, sequence=7usize)]
        s7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence=8usize)]
        s8: Vec<Strawberry8>,
        #[boulder(sequence = 9usize)]
        s9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (1usize, context)})]
        p1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (2usize, context)})]
        p2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (3usize, context)})]
        p3: Vec<Strawberry3<MuleState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (4usize, context)})]
        p4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (5usize, context)})]
        p5: Vec<Strawberry5<MuleState>>,
        #[boulder(buildable(c6 = 6), sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (6usize, context)})]
        p6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (7usize, context)})]
        p7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (8usize, context)})]
        p8: Vec<Strawberry8>,
        #[boulder(sequence_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (9usize, context)})]
        p9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator=boulder::Inc(1usize))]
        t1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator=boulder::Inc(2usize))]
        t2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator=boulder::Inc(3usize))]
        t3: Vec<Strawberry3<MuleState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_generator=boulder::Inc(4usize))]
        t4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_generator = boulder::Inc(5usize))]
        t5: Vec<Strawberry5<MuleState>>,
        #[boulder(buildable(c6 = 6), sequence_generator = boulder::Inc(6usize))]
        t6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_generator=boulder::Inc(7usize))]
        t7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_generator=boulder::Inc(8usize))]
        t8: Vec<Strawberry8>,
        #[boulder(sequence_generator = boulder::Inc(9usize))]
        t9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 1usize }: TestUsizeGenerator)]
        q1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 2usize }: TestUsizeGenerator)]
        q2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 3usize }: TestUsizeGenerator)]
        q3: Vec<Strawberry3<MuleState>>,
        #[boulder(generatable(c4=boulder::Inc(4)), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 4usize }: TestUsizeGenerator)]
        q4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 5usize }: TestUsizeGenerator)]
        q5: Vec<Strawberry5<MuleState>>,
        #[boulder(buildable(c6 = 6), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 6usize }: TestUsizeGenerator)]
        q6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| {context.get_iter::<Mule>().count(); (Strawberry7 { c7: 7 }, context) }, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 7usize }: TestUsizeGenerator)]
        q7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 8usize }: TestUsizeGenerator)]
        q8: Vec<Strawberry8>,
        #[boulder(sequence_generator_with_persian_rug=TestUsizeGenerator { value: 9usize }: TestUsizeGenerator)]
        q9: Vec<Strawberry9>,
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct MuleState {
        #[table]
        mules: Mule,
    }

    #[test]
    fn test_defaults() {
        let mut s = Default::default();

        let mut g = Mule::generator();

        let (d1, _) = g.generate(&mut s);
        let (d2, _) = g.generate(&mut s);

        assert_eq!(d1.v1.c1, 1);
        assert_eq!(d1.v2.c2, 2);
        assert_eq!(d1.v3.c3, 3);
        assert_eq!(d1.v4.c4, 4);
        assert_eq!(d1.v5.c5, 5);
        assert_eq!(d1.v6.c6, 6);
        assert_eq!(d1.v7.c7, 7);
        assert_eq!(d1.v8.c8, 8);
        assert_eq!(d1.v9.c9, 0);
        assert_eq!(d2.v1.c1, 2);
        assert_eq!(d2.v2.c2, 3);
        assert_eq!(d2.v3.c3, 4);
        assert_eq!(d2.v4.c4, 5);
        assert_eq!(d2.v5.c5, 5);
        assert_eq!(d2.v6.c6, 6);
        assert_eq!(d2.v7.c7, 7);
        assert_eq!(d2.v8.c8, 8);
        assert_eq!(d2.v9.c9, 0);

        assert_eq!(d1.s1.len(), 1);
        assert_eq!(d1.s1[0].c1, 1);
        assert_eq!(d1.s2.len(), 2);
        assert_eq!(d1.s2[0].c2, 2);
        assert_eq!(d1.s2[1].c2, 3);
        assert_eq!(d1.s3.len(), 3);
        assert_eq!(d1.s3[0].c3, 3);
        assert_eq!(d1.s3[1].c3, 4);
        assert_eq!(d1.s3[2].c3, 5);
        assert_eq!(d1.s4.len(), 4);
        assert_eq!(d1.s4[0].c4, 4);
        assert_eq!(d1.s4[1].c4, 5);
        assert_eq!(d1.s4[2].c4, 6);
        assert_eq!(d1.s4[3].c4, 7);
        assert_eq!(d1.s5.len(), 5);
        assert_eq!(d1.s5[0].c5, 5);
        assert_eq!(d1.s5[1].c5, 5);
        assert_eq!(d1.s5[2].c5, 5);
        assert_eq!(d1.s5[3].c5, 5);
        assert_eq!(d1.s5[4].c5, 5);
        assert_eq!(d1.s6.len(), 6);
        assert_eq!(d1.s6[0].c6, 6);
        assert_eq!(d1.s6[1].c6, 6);
        assert_eq!(d1.s6[2].c6, 6);
        assert_eq!(d1.s6[3].c6, 6);
        assert_eq!(d1.s6[4].c6, 6);
        assert_eq!(d1.s6[5].c6, 6);
        assert_eq!(d1.s7.len(), 7);
        assert_eq!(d1.s7[0].c7, 7);
        assert_eq!(d1.s7[1].c7, 7);
        assert_eq!(d1.s7[2].c7, 7);
        assert_eq!(d1.s7[3].c7, 7);
        assert_eq!(d1.s7[4].c7, 7);
        assert_eq!(d1.s7[5].c7, 7);
        assert_eq!(d1.s7[6].c7, 7);
        assert_eq!(d1.s8.len(), 8);
        assert_eq!(d1.s8[0].c8, 8);
        assert_eq!(d1.s8[1].c8, 8);
        assert_eq!(d1.s8[2].c8, 8);
        assert_eq!(d1.s8[3].c8, 8);
        assert_eq!(d1.s8[4].c8, 8);
        assert_eq!(d1.s8[5].c8, 8);
        assert_eq!(d1.s8[6].c8, 8);
        assert_eq!(d1.s8[7].c8, 8);
        assert_eq!(d1.s9.len(), 9);
        assert_eq!(d1.s9[0].c9, 0);
        assert_eq!(d1.s9[1].c9, 0);
        assert_eq!(d1.s9[2].c9, 0);
        assert_eq!(d1.s9[3].c9, 0);
        assert_eq!(d1.s9[4].c9, 0);
        assert_eq!(d1.s9[5].c9, 0);
        assert_eq!(d1.s9[6].c9, 0);
        assert_eq!(d1.s9[7].c9, 0);
        assert_eq!(d1.s9[8].c9, 0);
        assert_eq!(d2.s1.len(), 1);
        assert_eq!(d2.s1[0].c1, 2);
        assert_eq!(d2.s2.len(), 2);
        assert_eq!(d2.s2[0].c2, 4);
        assert_eq!(d2.s2[1].c2, 5);
        assert_eq!(d2.s3.len(), 3);
        assert_eq!(d2.s3[0].c3, 6);
        assert_eq!(d2.s3[1].c3, 7);
        assert_eq!(d2.s3[2].c3, 8);
        assert_eq!(d2.s4.len(), 4);
        assert_eq!(d2.s4[0].c4, 8);
        assert_eq!(d2.s4[1].c4, 9);
        assert_eq!(d2.s4[2].c4, 10);
        assert_eq!(d2.s4[3].c4, 11);
        assert_eq!(d2.s5.len(), 5);
        assert_eq!(d2.s5[0].c5, 5);
        assert_eq!(d2.s5[1].c5, 5);
        assert_eq!(d2.s5[2].c5, 5);
        assert_eq!(d2.s5[3].c5, 5);
        assert_eq!(d2.s5[4].c5, 5);
        assert_eq!(d2.s6.len(), 6);
        assert_eq!(d2.s6[0].c6, 6);
        assert_eq!(d2.s6[1].c6, 6);
        assert_eq!(d2.s6[2].c6, 6);
        assert_eq!(d2.s6[3].c6, 6);
        assert_eq!(d2.s6[4].c6, 6);
        assert_eq!(d2.s6[5].c6, 6);
        assert_eq!(d2.s7.len(), 7);
        assert_eq!(d2.s7[0].c7, 7);
        assert_eq!(d2.s7[1].c7, 7);
        assert_eq!(d2.s7[2].c7, 7);
        assert_eq!(d2.s7[3].c7, 7);
        assert_eq!(d2.s7[4].c7, 7);
        assert_eq!(d2.s7[5].c7, 7);
        assert_eq!(d2.s7[6].c7, 7);
        assert_eq!(d2.s8.len(), 8);
        assert_eq!(d2.s8[0].c8, 8);
        assert_eq!(d2.s8[1].c8, 8);
        assert_eq!(d2.s8[2].c8, 8);
        assert_eq!(d2.s8[3].c8, 8);
        assert_eq!(d2.s8[4].c8, 8);
        assert_eq!(d2.s8[5].c8, 8);
        assert_eq!(d2.s8[6].c8, 8);
        assert_eq!(d2.s8[7].c8, 8);
        assert_eq!(d2.s9.len(), 9);
        assert_eq!(d2.s9[0].c9, 0);
        assert_eq!(d2.s9[1].c9, 0);
        assert_eq!(d2.s9[2].c9, 0);
        assert_eq!(d2.s9[3].c9, 0);
        assert_eq!(d2.s9[4].c9, 0);
        assert_eq!(d2.s9[5].c9, 0);
        assert_eq!(d2.s9[6].c9, 0);
        assert_eq!(d2.s9[7].c9, 0);
        assert_eq!(d2.s9[8].c9, 0);

        assert_eq!(d1.p1.len(), 1);
        assert_eq!(d1.p1[0].c1, 1);
        assert_eq!(d1.p2.len(), 2);
        assert_eq!(d1.p2[0].c2, 2);
        assert_eq!(d1.p2[1].c2, 3);
        assert_eq!(d1.p3.len(), 3);
        assert_eq!(d1.p3[0].c3, 3);
        assert_eq!(d1.p3[1].c3, 4);
        assert_eq!(d1.p3[2].c3, 5);
        assert_eq!(d1.p4.len(), 4);
        assert_eq!(d1.p4[0].c4, 4);
        assert_eq!(d1.p4[1].c4, 5);
        assert_eq!(d1.p4[2].c4, 6);
        assert_eq!(d1.p4[3].c4, 7);
        assert_eq!(d1.p5.len(), 5);
        assert_eq!(d1.p5[0].c5, 5);
        assert_eq!(d1.p5[1].c5, 5);
        assert_eq!(d1.p5[2].c5, 5);
        assert_eq!(d1.p5[3].c5, 5);
        assert_eq!(d1.p5[4].c5, 5);
        assert_eq!(d1.p6.len(), 6);
        assert_eq!(d1.p6[0].c6, 6);
        assert_eq!(d1.p6[1].c6, 6);
        assert_eq!(d1.p6[2].c6, 6);
        assert_eq!(d1.p6[3].c6, 6);
        assert_eq!(d1.p6[4].c6, 6);
        assert_eq!(d1.p6[5].c6, 6);
        assert_eq!(d1.p7.len(), 7);
        assert_eq!(d1.p7[0].c7, 7);
        assert_eq!(d1.p7[1].c7, 7);
        assert_eq!(d1.p7[2].c7, 7);
        assert_eq!(d1.p7[3].c7, 7);
        assert_eq!(d1.p7[4].c7, 7);
        assert_eq!(d1.p7[5].c7, 7);
        assert_eq!(d1.p7[6].c7, 7);
        assert_eq!(d1.p8.len(), 8);
        assert_eq!(d1.p8[0].c8, 8);
        assert_eq!(d1.p8[1].c8, 8);
        assert_eq!(d1.p8[2].c8, 8);
        assert_eq!(d1.p8[3].c8, 8);
        assert_eq!(d1.p8[4].c8, 8);
        assert_eq!(d1.p8[5].c8, 8);
        assert_eq!(d1.p8[6].c8, 8);
        assert_eq!(d1.p8[7].c8, 8);
        assert_eq!(d1.p9.len(), 9);
        assert_eq!(d1.p9[0].c9, 0);
        assert_eq!(d1.p9[1].c9, 0);
        assert_eq!(d1.p9[2].c9, 0);
        assert_eq!(d1.p9[3].c9, 0);
        assert_eq!(d1.p9[4].c9, 0);
        assert_eq!(d1.p9[5].c9, 0);
        assert_eq!(d1.p9[6].c9, 0);
        assert_eq!(d1.p9[7].c9, 0);
        assert_eq!(d1.p9[8].c9, 0);
        assert_eq!(d2.p1.len(), 1);
        assert_eq!(d2.p1[0].c1, 2);
        assert_eq!(d2.p2.len(), 2);
        assert_eq!(d2.p2[0].c2, 4);
        assert_eq!(d2.p2[1].c2, 5);
        assert_eq!(d2.p3.len(), 3);
        assert_eq!(d2.p3[0].c3, 6);
        assert_eq!(d2.p3[1].c3, 7);
        assert_eq!(d2.p3[2].c3, 8);
        assert_eq!(d2.p4.len(), 4);
        assert_eq!(d2.p4[0].c4, 8);
        assert_eq!(d2.p4[1].c4, 9);
        assert_eq!(d2.p4[2].c4, 10);
        assert_eq!(d2.p4[3].c4, 11);
        assert_eq!(d2.p5.len(), 5);
        assert_eq!(d2.p5[0].c5, 5);
        assert_eq!(d2.p5[1].c5, 5);
        assert_eq!(d2.p5[2].c5, 5);
        assert_eq!(d2.p5[3].c5, 5);
        assert_eq!(d2.p5[4].c5, 5);
        assert_eq!(d2.p6.len(), 6);
        assert_eq!(d2.p6[0].c6, 6);
        assert_eq!(d2.p6[1].c6, 6);
        assert_eq!(d2.p6[2].c6, 6);
        assert_eq!(d2.p6[3].c6, 6);
        assert_eq!(d2.p6[4].c6, 6);
        assert_eq!(d2.p6[5].c6, 6);
        assert_eq!(d2.p7.len(), 7);
        assert_eq!(d2.p7[0].c7, 7);
        assert_eq!(d2.p7[1].c7, 7);
        assert_eq!(d2.p7[2].c7, 7);
        assert_eq!(d2.p7[3].c7, 7);
        assert_eq!(d2.p7[4].c7, 7);
        assert_eq!(d2.p7[5].c7, 7);
        assert_eq!(d2.p7[6].c7, 7);
        assert_eq!(d2.p8.len(), 8);
        assert_eq!(d2.p8[0].c8, 8);
        assert_eq!(d2.p8[1].c8, 8);
        assert_eq!(d2.p8[2].c8, 8);
        assert_eq!(d2.p8[3].c8, 8);
        assert_eq!(d2.p8[4].c8, 8);
        assert_eq!(d2.p8[5].c8, 8);
        assert_eq!(d2.p8[6].c8, 8);
        assert_eq!(d2.p8[7].c8, 8);
        assert_eq!(d2.p9.len(), 9);
        assert_eq!(d2.p9[0].c9, 0);
        assert_eq!(d2.p9[1].c9, 0);
        assert_eq!(d2.p9[2].c9, 0);
        assert_eq!(d2.p9[3].c9, 0);
        assert_eq!(d2.p9[4].c9, 0);
        assert_eq!(d2.p9[5].c9, 0);
        assert_eq!(d2.p9[6].c9, 0);
        assert_eq!(d2.p9[7].c9, 0);
        assert_eq!(d2.p9[8].c9, 0);

        assert_eq!(d1.t1.len(), 1);
        assert_eq!(d1.t1[0].c1, 1);
        assert_eq!(d1.t2.len(), 2);
        assert_eq!(d1.t2[0].c2, 2);
        assert_eq!(d1.t2[1].c2, 3);
        assert_eq!(d1.t3.len(), 3);
        assert_eq!(d1.t3[0].c3, 3);
        assert_eq!(d1.t3[1].c3, 4);
        assert_eq!(d1.t3[2].c3, 5);
        assert_eq!(d1.t4.len(), 4);
        assert_eq!(d1.t4[0].c4, 4);
        assert_eq!(d1.t4[1].c4, 5);
        assert_eq!(d1.t4[2].c4, 6);
        assert_eq!(d1.t4[3].c4, 7);
        assert_eq!(d1.t5.len(), 5);
        assert_eq!(d1.t5[0].c5, 5);
        assert_eq!(d1.t5[1].c5, 5);
        assert_eq!(d1.t5[2].c5, 5);
        assert_eq!(d1.t5[3].c5, 5);
        assert_eq!(d1.t5[4].c5, 5);
        assert_eq!(d1.t6.len(), 6);
        assert_eq!(d1.t6[0].c6, 6);
        assert_eq!(d1.t6[1].c6, 6);
        assert_eq!(d1.t6[2].c6, 6);
        assert_eq!(d1.t6[3].c6, 6);
        assert_eq!(d1.t6[4].c6, 6);
        assert_eq!(d1.t6[5].c6, 6);
        assert_eq!(d1.t7.len(), 7);
        assert_eq!(d1.t7[0].c7, 7);
        assert_eq!(d1.t7[1].c7, 7);
        assert_eq!(d1.t7[2].c7, 7);
        assert_eq!(d1.t7[3].c7, 7);
        assert_eq!(d1.t7[4].c7, 7);
        assert_eq!(d1.t7[5].c7, 7);
        assert_eq!(d1.t7[6].c7, 7);
        assert_eq!(d1.t8.len(), 8);
        assert_eq!(d1.t8[0].c8, 8);
        assert_eq!(d1.t8[1].c8, 8);
        assert_eq!(d1.t8[2].c8, 8);
        assert_eq!(d1.t8[3].c8, 8);
        assert_eq!(d1.t8[4].c8, 8);
        assert_eq!(d1.t8[5].c8, 8);
        assert_eq!(d1.t8[6].c8, 8);
        assert_eq!(d1.t8[7].c8, 8);
        assert_eq!(d1.t9.len(), 9);
        assert_eq!(d1.t9[0].c9, 0);
        assert_eq!(d1.t9[1].c9, 0);
        assert_eq!(d1.t9[2].c9, 0);
        assert_eq!(d1.t9[3].c9, 0);
        assert_eq!(d1.t9[4].c9, 0);
        assert_eq!(d1.t9[5].c9, 0);
        assert_eq!(d1.t9[6].c9, 0);
        assert_eq!(d1.t9[7].c9, 0);
        assert_eq!(d1.t9[8].c9, 0);
        assert_eq!(d2.t1.len(), 2);
        assert_eq!(d2.t1[0].c1, 2);
        assert_eq!(d2.t1[1].c1, 3);
        assert_eq!(d2.t2.len(), 3);
        assert_eq!(d2.t2[0].c2, 4);
        assert_eq!(d2.t2[1].c2, 5);
        assert_eq!(d2.t2[2].c2, 6);
        assert_eq!(d2.t3.len(), 4);
        assert_eq!(d2.t3[0].c3, 6);
        assert_eq!(d2.t3[1].c3, 7);
        assert_eq!(d2.t3[2].c3, 8);
        assert_eq!(d2.t3[3].c3, 9);
        assert_eq!(d2.t4.len(), 5);
        assert_eq!(d2.t4[0].c4, 8);
        assert_eq!(d2.t4[1].c4, 9);
        assert_eq!(d2.t4[2].c4, 10);
        assert_eq!(d2.t4[3].c4, 11);
        assert_eq!(d2.t4[4].c4, 12);
        assert_eq!(d2.t5.len(), 6);
        assert_eq!(d2.t5[0].c5, 5);
        assert_eq!(d2.t5[1].c5, 5);
        assert_eq!(d2.t5[2].c5, 5);
        assert_eq!(d2.t5[3].c5, 5);
        assert_eq!(d2.t5[4].c5, 5);
        assert_eq!(d2.t5[5].c5, 5);
        assert_eq!(d2.t6.len(), 7);
        assert_eq!(d2.t6[0].c6, 6);
        assert_eq!(d2.t6[1].c6, 6);
        assert_eq!(d2.t6[2].c6, 6);
        assert_eq!(d2.t6[3].c6, 6);
        assert_eq!(d2.t6[4].c6, 6);
        assert_eq!(d2.t6[5].c6, 6);
        assert_eq!(d2.t6[6].c6, 6);
        assert_eq!(d2.t7.len(), 8);
        assert_eq!(d2.t7[0].c7, 7);
        assert_eq!(d2.t7[1].c7, 7);
        assert_eq!(d2.t7[2].c7, 7);
        assert_eq!(d2.t7[3].c7, 7);
        assert_eq!(d2.t7[4].c7, 7);
        assert_eq!(d2.t7[5].c7, 7);
        assert_eq!(d2.t7[6].c7, 7);
        assert_eq!(d2.t7[7].c7, 7);
        assert_eq!(d2.t8.len(), 9);
        assert_eq!(d2.t8[0].c8, 8);
        assert_eq!(d2.t8[1].c8, 8);
        assert_eq!(d2.t8[2].c8, 8);
        assert_eq!(d2.t8[3].c8, 8);
        assert_eq!(d2.t8[4].c8, 8);
        assert_eq!(d2.t8[5].c8, 8);
        assert_eq!(d2.t8[6].c8, 8);
        assert_eq!(d2.t8[7].c8, 8);
        assert_eq!(d2.t8[8].c8, 8);
        assert_eq!(d2.t9.len(), 10);
        assert_eq!(d2.t9[0].c9, 0);
        assert_eq!(d2.t9[1].c9, 0);
        assert_eq!(d2.t9[2].c9, 0);
        assert_eq!(d2.t9[3].c9, 0);
        assert_eq!(d2.t9[4].c9, 0);
        assert_eq!(d2.t9[5].c9, 0);
        assert_eq!(d2.t9[6].c9, 0);
        assert_eq!(d2.t9[7].c9, 0);
        assert_eq!(d2.t9[8].c9, 0);
        assert_eq!(d2.t9[9].c9, 0);

        assert_eq!(d1.q1.len(), 1);
        assert_eq!(d1.q1[0].c1, 1);
        assert_eq!(d1.q2.len(), 2);
        assert_eq!(d1.q2[0].c2, 2);
        assert_eq!(d1.q2[1].c2, 3);
        assert_eq!(d1.q3.len(), 3);
        assert_eq!(d1.q3[0].c3, 3);
        assert_eq!(d1.q3[1].c3, 4);
        assert_eq!(d1.q3[2].c3, 5);
        assert_eq!(d1.q4.len(), 4);
        assert_eq!(d1.q4[0].c4, 4);
        assert_eq!(d1.q4[1].c4, 5);
        assert_eq!(d1.q4[2].c4, 6);
        assert_eq!(d1.q4[3].c4, 7);
        assert_eq!(d1.q5.len(), 5);
        assert_eq!(d1.q5[0].c5, 5);
        assert_eq!(d1.q5[1].c5, 5);
        assert_eq!(d1.q5[2].c5, 5);
        assert_eq!(d1.q5[3].c5, 5);
        assert_eq!(d1.q5[4].c5, 5);
        assert_eq!(d1.q6.len(), 6);
        assert_eq!(d1.q6[0].c6, 6);
        assert_eq!(d1.q6[1].c6, 6);
        assert_eq!(d1.q6[2].c6, 6);
        assert_eq!(d1.q6[3].c6, 6);
        assert_eq!(d1.q6[4].c6, 6);
        assert_eq!(d1.q6[5].c6, 6);
        assert_eq!(d1.q7.len(), 7);
        assert_eq!(d1.q7[0].c7, 7);
        assert_eq!(d1.q7[1].c7, 7);
        assert_eq!(d1.q7[2].c7, 7);
        assert_eq!(d1.q7[3].c7, 7);
        assert_eq!(d1.q7[4].c7, 7);
        assert_eq!(d1.q7[5].c7, 7);
        assert_eq!(d1.q7[6].c7, 7);
        assert_eq!(d1.q8.len(), 8);
        assert_eq!(d1.q8[0].c8, 8);
        assert_eq!(d1.q8[1].c8, 8);
        assert_eq!(d1.q8[2].c8, 8);
        assert_eq!(d1.q8[3].c8, 8);
        assert_eq!(d1.q8[4].c8, 8);
        assert_eq!(d1.q8[5].c8, 8);
        assert_eq!(d1.q8[6].c8, 8);
        assert_eq!(d1.q8[7].c8, 8);
        assert_eq!(d1.q9.len(), 9);
        assert_eq!(d1.q9[0].c9, 0);
        assert_eq!(d1.q9[1].c9, 0);
        assert_eq!(d1.q9[2].c9, 0);
        assert_eq!(d1.q9[3].c9, 0);
        assert_eq!(d1.q9[4].c9, 0);
        assert_eq!(d1.q9[5].c9, 0);
        assert_eq!(d1.q9[6].c9, 0);
        assert_eq!(d1.q9[7].c9, 0);
        assert_eq!(d1.q9[8].c9, 0);
        assert_eq!(d2.q1.len(), 2);
        assert_eq!(d2.q1[0].c1, 2);
        assert_eq!(d2.q1[1].c1, 3);
        assert_eq!(d2.q2.len(), 3);
        assert_eq!(d2.q2[0].c2, 4);
        assert_eq!(d2.q2[1].c2, 5);
        assert_eq!(d2.q2[2].c2, 6);
        assert_eq!(d2.q3.len(), 4);
        assert_eq!(d2.q3[0].c3, 6);
        assert_eq!(d2.q3[1].c3, 7);
        assert_eq!(d2.q3[2].c3, 8);
        assert_eq!(d2.q3[3].c3, 9);
        assert_eq!(d2.q4.len(), 5);
        assert_eq!(d2.q4[0].c4, 8);
        assert_eq!(d2.q4[1].c4, 9);
        assert_eq!(d2.q4[2].c4, 10);
        assert_eq!(d2.q4[3].c4, 11);
        assert_eq!(d2.q4[4].c4, 12);
        assert_eq!(d2.q5.len(), 6);
        assert_eq!(d2.q5[0].c5, 5);
        assert_eq!(d2.q5[1].c5, 5);
        assert_eq!(d2.q5[2].c5, 5);
        assert_eq!(d2.q5[3].c5, 5);
        assert_eq!(d2.q5[4].c5, 5);
        assert_eq!(d2.q5[5].c5, 5);
        assert_eq!(d2.q6.len(), 7);
        assert_eq!(d2.q6[0].c6, 6);
        assert_eq!(d2.q6[1].c6, 6);
        assert_eq!(d2.q6[2].c6, 6);
        assert_eq!(d2.q6[3].c6, 6);
        assert_eq!(d2.q6[4].c6, 6);
        assert_eq!(d2.q6[5].c6, 6);
        assert_eq!(d2.q6[6].c6, 6);
        assert_eq!(d2.q7.len(), 8);
        assert_eq!(d2.q7[0].c7, 7);
        assert_eq!(d2.q7[1].c7, 7);
        assert_eq!(d2.q7[2].c7, 7);
        assert_eq!(d2.q7[3].c7, 7);
        assert_eq!(d2.q7[4].c7, 7);
        assert_eq!(d2.q7[5].c7, 7);
        assert_eq!(d2.q7[6].c7, 7);
        assert_eq!(d2.q7[7].c7, 7);
        assert_eq!(d2.q8.len(), 9);
        assert_eq!(d2.q8[0].c8, 8);
        assert_eq!(d2.q8[1].c8, 8);
        assert_eq!(d2.q8[2].c8, 8);
        assert_eq!(d2.q8[3].c8, 8);
        assert_eq!(d2.q8[4].c8, 8);
        assert_eq!(d2.q8[5].c8, 8);
        assert_eq!(d2.q8[6].c8, 8);
        assert_eq!(d2.q8[7].c8, 8);
        assert_eq!(d2.q8[8].c8, 8);
        assert_eq!(d2.q9.len(), 10);
        assert_eq!(d2.q9[0].c9, 0);
        assert_eq!(d2.q9[1].c9, 0);
        assert_eq!(d2.q9[2].c9, 0);
        assert_eq!(d2.q9[3].c9, 0);
        assert_eq!(d2.q9[4].c9, 0);
        assert_eq!(d2.q9[5].c9, 0);
        assert_eq!(d2.q9[6].c9, 0);
        assert_eq!(d2.q9[7].c9, 0);
        assert_eq!(d2.q9[8].c9, 0);
        assert_eq!(d2.q9[9].c9, 0);
    }

    #[test]
    fn test_customise() {
        use boulder::GeneratorToGeneratorWithPersianRugWrapper as GeneratorWrapper;

        let mut s: MuleState = Default::default();

        let mut g = Mule::generator()
            .v1(GeneratorWrapper::new(|| Strawberry1 { c1: 11 }))
            .v2(GeneratorWrapper::new(|| Strawberry2 { c2: 22 }))
            .v3(GeneratorWrapper::new(|| Strawberry3 {
                c3: 33,
                _marker: Default::default(),
            }))
            .v4(GeneratorWrapper::new(|| Strawberry4 { c4: 44 }))
            .v5(GeneratorWrapper::new(|| Strawberry5 {
                c5: 55,
                _marker: Default::default(),
            }))
            .v6(GeneratorWrapper::new(|| Strawberry6 { c6: 66 }))
            .v7(GeneratorWrapper::new(|| Strawberry7 { c7: 77 }))
            .v8(GeneratorWrapper::new(|| Strawberry8 { c8: 88 }))
            .v9(GeneratorWrapper::new(|| Strawberry9 { c9: 99 }))
            .s1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .s2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .s3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .s4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .s5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .s6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .s7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .s8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .s9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .p1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .p2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .p3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .p4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .p5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .p6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .p7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .p8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .p9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .t1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .t2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .t3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .t4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .t5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .t6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .t7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .t8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .t9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]))
            .q1(GeneratorWrapper::new(|| vec![Strawberry1 { c1: 11 }]))
            .q2(GeneratorWrapper::new(|| vec![Strawberry2 { c2: 22 }]))
            .q3(GeneratorWrapper::new(|| {
                vec![Strawberry3 {
                    c3: 33,
                    _marker: Default::default(),
                }]
            }))
            .q4(GeneratorWrapper::new(|| vec![Strawberry4 { c4: 44 }]))
            .q5(GeneratorWrapper::new(|| {
                vec![Strawberry5 {
                    c5: 55,
                    _marker: Default::default(),
                }]
            }))
            .q6(GeneratorWrapper::new(|| vec![Strawberry6 { c6: 66 }]))
            .q7(GeneratorWrapper::new(|| vec![Strawberry7 { c7: 77 }]))
            .q8(GeneratorWrapper::new(|| vec![Strawberry8 { c8: 88 }]))
            .q9(GeneratorWrapper::new(|| vec![Strawberry9 { c9: 99 }]));

        let (d, _) = g.generate(&mut s);

        assert_eq!(d.v1.c1, 11);
        assert_eq!(d.v2.c2, 22);
        assert_eq!(d.v3.c3, 33);
        assert_eq!(d.v4.c4, 44);
        assert_eq!(d.v5.c5, 55);
        assert_eq!(d.v6.c6, 66);
        assert_eq!(d.v7.c7, 77);
        assert_eq!(d.v8.c8, 88);
        assert_eq!(d.v9.c9, 99);

        assert_eq!(d.s1.len(), 1);
        assert_eq!(d.s1[0].c1, 11);
        assert_eq!(d.s2.len(), 1);
        assert_eq!(d.s2[0].c2, 22);
        assert_eq!(d.s3.len(), 1);
        assert_eq!(d.s3[0].c3, 33);
        assert_eq!(d.s4.len(), 1);
        assert_eq!(d.s4[0].c4, 44);
        assert_eq!(d.s5.len(), 1);
        assert_eq!(d.s5[0].c5, 55);
        assert_eq!(d.s6.len(), 1);
        assert_eq!(d.s6[0].c6, 66);
        assert_eq!(d.s7.len(), 1);
        assert_eq!(d.s7[0].c7, 77);
        assert_eq!(d.s8.len(), 1);
        assert_eq!(d.s8[0].c8, 88);
        assert_eq!(d.s9.len(), 1);
        assert_eq!(d.s9[0].c9, 99);

        assert_eq!(d.p1.len(), 1);
        assert_eq!(d.p1[0].c1, 11);
        assert_eq!(d.p2.len(), 1);
        assert_eq!(d.p2[0].c2, 22);
        assert_eq!(d.p3.len(), 1);
        assert_eq!(d.p3[0].c3, 33);
        assert_eq!(d.p4.len(), 1);
        assert_eq!(d.p4[0].c4, 44);
        assert_eq!(d.p5.len(), 1);
        assert_eq!(d.p5[0].c5, 55);
        assert_eq!(d.p6.len(), 1);
        assert_eq!(d.p6[0].c6, 66);
        assert_eq!(d.p7.len(), 1);
        assert_eq!(d.p7[0].c7, 77);
        assert_eq!(d.p8.len(), 1);
        assert_eq!(d.p8[0].c8, 88);
        assert_eq!(d.p9.len(), 1);
        assert_eq!(d.p9[0].c9, 99);

        assert_eq!(d.t1.len(), 1);
        assert_eq!(d.t1[0].c1, 11);
        assert_eq!(d.t2.len(), 1);
        assert_eq!(d.t2[0].c2, 22);
        assert_eq!(d.t3.len(), 1);
        assert_eq!(d.t3[0].c3, 33);
        assert_eq!(d.t4.len(), 1);
        assert_eq!(d.t4[0].c4, 44);
        assert_eq!(d.t5.len(), 1);
        assert_eq!(d.t5[0].c5, 55);
        assert_eq!(d.t6.len(), 1);
        assert_eq!(d.t6[0].c6, 66);
        assert_eq!(d.t7.len(), 1);
        assert_eq!(d.t7[0].c7, 77);
        assert_eq!(d.t8.len(), 1);
        assert_eq!(d.t8[0].c8, 88);
        assert_eq!(d.t9.len(), 1);
        assert_eq!(d.t9[0].c9, 99);

        assert_eq!(d.q1.len(), 1);
        assert_eq!(d.q1[0].c1, 11);
        assert_eq!(d.q2.len(), 1);
        assert_eq!(d.q2[0].c2, 22);
        assert_eq!(d.q3.len(), 1);
        assert_eq!(d.q3[0].c3, 33);
        assert_eq!(d.q4.len(), 1);
        assert_eq!(d.q4[0].c4, 44);
        assert_eq!(d.q5.len(), 1);
        assert_eq!(d.q5[0].c5, 55);
        assert_eq!(d.q6.len(), 1);
        assert_eq!(d.q6[0].c6, 66);
        assert_eq!(d.q7.len(), 1);
        assert_eq!(d.q7[0].c7, 77);
        assert_eq!(d.q8.len(), 1);
        assert_eq!(d.q8[0].c8, 88);
        assert_eq!(d.q9.len(), 1);
        assert_eq!(d.q9[0].c9, 99);
    }
}

mod generator_wrappers {
    use super::*;

    use boulder::{
        GeneratableWithPersianRug, GeneratorToGeneratorWithPersianRugWrapper as GeneratorWrapper,
        GeneratorWithPersianRug,
    };
    use persian_rug::Proxy;
    use std::any::Any;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[persian_rug::contextual(C)]
    #[derive(GeneratableWithPersianRug)]
    #[boulder(persian_rug(context=C, access(Foo2<C>)))]
    struct Foo2<C: persian_rug::Context + 'static> {
        _marker: core::marker::PhantomData<C>,
        a: i32,
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct State2 {
        #[table]
        foos: Foo2<State2>,
    }

    #[test]
    fn test_option() {
        let mut s: State2 = Default::default();

        let mut g = Option::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Option<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.as_ref().map(|f1| f1.a), Some(5));
    }

    #[test]
    fn test_proxy() {
        let mut s: State2 = Default::default();

        let mut g = Proxy::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Proxy<Foo2<State2>>>(), f1.type_id());
        assert_eq!(<&State2 as persian_rug::Accessor>::get(&&s, &f1).a, 5);
    }

    #[test]
    fn test_option_proxy() {
        let mut s: State2 = Default::default();

        let mut g = Option::<Proxy<Foo2<State2>>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(
            std::any::TypeId::of::<Option<Proxy<Foo2<State2>>>>(),
            f1.type_id()
        );
        assert_eq!(
            f1.as_ref()
                .map(|f1| <State2 as persian_rug::Context>::get(&s, &f1).a),
            Some(5)
        );
    }

    #[test]
    fn test_arc() {
        let mut s: State2 = Default::default();

        let mut g = Arc::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Arc<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.a, 5);
    }

    #[test]
    fn test_mutex() {
        let mut s: State2 = Default::default();

        let mut g = Mutex::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Mutex<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.lock().unwrap().a, 5);
    }

    #[test]
    fn test_arc_mutex() {
        let mut s: State2 = Default::default();

        let mut g = Arc::<Mutex<Foo2<State2>>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(
            std::any::TypeId::of::<Arc<Mutex<Foo2<State2>>>>(),
            f1.type_id()
        );
        assert_eq!(f1.lock().unwrap().a, 5);
    }

    #[test]
    fn test_rc() {
        let mut s: State2 = Default::default();

        let mut g = Rc::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Rc<Foo2<State2>>>(), f1.type_id());
        assert_eq!(f1.a, 5);
    }

    #[test]
    fn test_cell() {
        let mut s: State2 = Default::default();

        let mut g = Cell::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(std::any::TypeId::of::<Cell<Foo2<State2>>>(), f1.type_id());
        let f1_contents = f1.into_inner();
        assert_eq!(f1_contents.a, 5);
    }

    #[test]
    fn test_ref_cell() {
        let mut s: State2 = Default::default();

        let mut g = RefCell::<Foo2<State2>>::generator().a(GeneratorWrapper::new(|| 5));
        let (f1, _) = g.generate(&mut s);
        assert_eq!(
            std::any::TypeId::of::<RefCell<Foo2<State2>>>(),
            f1.type_id()
        );
        assert_eq!(f1.borrow().a, 5);
    }
}
