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

    #[test]
    fn test_option() {
        let mut s: State2 = Default::default();

        let (f1, _) = Option::<Foo2<State2>>::builder().a(5).build(&mut s);

        let f1 = <State2 as persian_rug::Context>::add(&mut s, f1.unwrap());
        println!("Got foo2 {:?}", f1);

        let (b1, _) = Option::<persian_rug::Proxy<Bar2<State2>>>::builder()
            .a(5)
            .build(&mut s);
        println!("Got bar2 {:?}", b1);
    }
}

mod builder_coverage {
    use boulder::{Buildable, BuildableWithPersianRug, Builder, BuilderWithPersianRug};
    use boulder::{Generatable, GeneratableWithPersianRug, Generator, GeneratorWithPersianRug};

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
        #[boulder(generator=boulder::gen::Inc(0))]
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
    #[boulder(persian_rug(context=C, access(Rabbit<C>)))]
    struct Rabbit<C: persian_rug::Context + 'static> {
        _marker: core::marker::PhantomData<C>,
        // #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 })]
        // v1: Carrot1,
        // #[boulder(generator=Carrot2Generator {c2: 2})]
        // v2: Carrot2,
        // //v3: Carrot3,
        // #[boulder(generatable(c4=boulder::gen::Inc(4)))]
        // v4: Carrot4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Carrot5<C>,
        #[boulder(buildable(c6 = 6))]
        v6: Carrot6,
        #[boulder(default_with_persian_rug=|context| { (Carrot7 { c7: 7 }, context) })]
        v7: Carrot7,
        #[boulder(default=Carrot8 { c8: 8 })]
        v8: Carrot8,
        v9: Carrot9,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence=1)]
        s1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence=2)]
        s2: Vec<Carrot2>,
        #[boulder(generatable_with_persian_rug(c3=TestIndexGenerator { value: 3 }))]
        s3: Vec<Carrot3<C>>,
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence=4)]
        s4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5)]
        s5: Vec<Carrot5<C>>,
        #[boulder(buildable(c6 = 6), sequence = 6)]
        s6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| { (Carrot7 { c7: 7 }, context) }, sequence=7)]
        s7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence=8)]
        s8: Vec<Carrot8>,
        #[boulder(sequence = 9)]
        s9: Vec<Carrot9>,

        #[boulder(generator_with_persian_rug=Carrot1Generator { c1: 1 }, sequence_with_persian_rug=|context| (1, context))]
        p1: Vec<Carrot1>,
        #[boulder(generator=Carrot2Generator {c2: 2}, sequence_with_persian_rug=|context| (2, context))]
        p2: Vec<Carrot2>,
        // //s3: Carrot3,
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence_with_persian_rug=|context| (4, context))]
        p4: Vec<Carrot4>,
        #[boulder(buildable_with_persian_rug(c5=5), sequence_with_persian_rug=|context| (5, context))]
        p5: Vec<Carrot5<C>>,
        #[boulder(buildable(c6=6), sequence_with_persian_rug=|context| (6, context))]
        p6: Vec<Carrot6>,
        #[boulder(default_with_persian_rug=|context| { (Carrot7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| (7, context))]
        p7: Vec<Carrot7>,
        #[boulder(default=Carrot8 { c8: 8 }, sequence_with_persian_rug=|context| (8, context))]
        p8: Vec<Carrot8>,
        #[boulder(sequence_with_persian_rug=|context| (9, context))]
        p9: Vec<Carrot9>,
    }

    impl<C: persian_rug::Context> persian_rug::Contextual for Rabbit<C> {
        type Context = C;
    }

    #[derive(Default)]
    #[persian_rug::persian_rug]
    struct RabbitState {
        #[table]
        rabbits: Rabbit<RabbitState>,
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
            for wizard in context.get_iter() {
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
        #[boulder(generator=boulder::gen::Inc(5))]
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
        #[boulder(generator=boulder::gen::Inc(0))]
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
    struct Donkey<C: persian_rug::Context + 'static> {
        _marker: core::marker::PhantomData<C>,
        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator)]
        v1: Strawberry1,
        #[boulder(generator=Strawberry2Generator {c2: 2})]
        v2: Strawberry2,
        #[boulder(generatable_with_persian_rug)]
        v3: Strawberry3<C>,
        #[boulder(generatable(c4=boulder::gen::Inc(4)))]
        v4: Strawberry4,
        #[boulder(buildable_with_persian_rug(c5 = 5))]
        v5: Strawberry5<C>,
        #[boulder(buildable(c6 = 6))]
        v6: Strawberry6,
        #[boulder(default_with_persian_rug=|context| { (Strawberry7 { c7: 7 }, context) })]
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
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence=4usize)]
        s4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5usize)]
        s5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence = 6usize)]
        s6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { (Strawberry7 { c7: 7 }, context) }, sequence=7usize)]
        s7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence=8usize)]
        s8: Vec<Strawberry8>,
        #[boulder(sequence = 9usize)]
        s9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_with_persian_rug=|context| (1usize, context))]
        p1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_with_persian_rug=|context| (2usize, context))]
        p2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_with_persian_rug=|context| (3usize, context))]
        p3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence_with_persian_rug=|context| (4usize, context))]
        p4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_with_persian_rug=|context| (5usize, context))]
        p5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence_with_persian_rug=|context| (6usize, context))]
        p6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { (Strawberry7 { c7: 7 }, context) }, sequence_with_persian_rug=|context| (7usize, context))]
        p7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_with_persian_rug=|context| (8usize, context))]
        p8: Vec<Strawberry8>,
        #[boulder(sequence_with_persian_rug=|context| (9usize, context))]
        p9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator=boulder::gen::Inc(1usize))]
        t1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator=boulder::gen::Inc(2usize))]
        t2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator=boulder::gen::Inc(3usize))]
        t3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence_generator=boulder::gen::Inc(4usize))]
        t4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence = 5usize)]
        t5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence = 6usize)]
        t6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { (Strawberry7 { c7: 7 }, context) }, sequence_generator=boulder::gen::Inc(7usize))]
        t7: Vec<Strawberry7>,
        #[boulder(default=Strawberry8 { c8: 8 }, sequence_generator=boulder::gen::Inc(8usize))]
        t8: Vec<Strawberry8>,
        #[boulder(sequence = 9usize)]
        t9: Vec<Strawberry9>,

        #[boulder(generator_with_persian_rug=Strawberry1Generator { c1: 1 }: Strawberry1Generator, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 1usize }: TestUsizeGenerator)]
        q1: Vec<Strawberry1>,
        #[boulder(generator=Strawberry2Generator {c2: 2}, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 2usize }: TestUsizeGenerator)]
        q2: Vec<Strawberry2>,
        #[boulder(generatable_with_persian_rug, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 3usize }: TestUsizeGenerator)]
        q3: Vec<Strawberry3<C>>,
        #[boulder(generatable(c4=boulder::gen::Inc(4)), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 4usize }: TestUsizeGenerator)]
        q4: Vec<Strawberry4>,
        #[boulder(buildable_with_persian_rug(c5 = 5), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 5usize }: TestUsizeGenerator)]
        q5: Vec<Strawberry5<C>>,
        #[boulder(buildable(c6 = 6), sequence_generator_with_persian_rug=TestUsizeGenerator { value: 6usize }: TestUsizeGenerator)]
        q6: Vec<Strawberry6>,
        #[boulder(default_with_persian_rug=|context| { (Strawberry7 { c7: 7 }, context) }, sequence_generator_with_persian_rug=TestUsizeGenerator { value: 7usize }: TestUsizeGenerator)]
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
}
