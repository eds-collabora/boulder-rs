pub trait BuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    type Builder: BuilderWithPersianRug<C, Result = Self>;
    fn builder() -> Self::Builder;
}

pub trait BuilderWithPersianRug<C>: Sized
where
    C: persian_rug::Context,
{
    type Result;
    fn build<'b, B>(self, context: B) -> (Self::Result, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

#[doc(hidden)]
pub trait ConverterWithPersianRug<C, From>
where
    C: persian_rug::Context
{
    type Output;
    fn convert<'b, B>(self, input: From, context: B) -> (Self::Output, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>;
}

#[doc(hidden)]
pub struct SelfConverterWithPersianRug;

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> ConverterWithPersianRug<C, T> for SelfConverterWithPersianRug
{
    type Output = T;
    fn convert<'b, B>(self, input: T, context: B) -> (T, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>
    {
        (input, context)
    }
}

pub trait ProxyBuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context
{
    type Builder: BuilderWithPersianRug<C, Result = persian_rug::Proxy<Self>>;
    fn proxy_builder() -> Self::Builder;
}

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> BuildableWithPersianRug<C> for persian_rug::Proxy<T>
where
    T: ProxyBuildableWithPersianRug<C>
{
    type Builder = <T as ProxyBuildableWithPersianRug<C>>::Builder;
    fn builder() -> Self::Builder {
        T::proxy_builder()
    }
}

#[doc(hidden)]
pub struct ProxyConverterWithPersianRug;

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> ConverterWithPersianRug<C, T> for ProxyConverterWithPersianRug
{
    type Output = persian_rug::Proxy<T>;
    fn convert<'b, B>(self, input: T, mut context: B) -> (persian_rug::Proxy<T>, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>
    {
        (context.add(input), context)
    }
}

pub trait OptionBuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context
{
    type Builder: BuilderWithPersianRug<C, Result = Option<Self>>;
    fn option_builder() -> Self::Builder;
}

impl<C, T> BuildableWithPersianRug<C> for Option<T>
where
    C: persian_rug::Context,
    T: OptionBuildableWithPersianRug<C>
{
    type Builder = <T as OptionBuildableWithPersianRug<C>>::Builder;
    fn builder() -> Self::Builder {
        T::option_builder()
    }
}

#[doc(hidden)]
pub struct OptionConverterWithPersianRug;

impl<C, T> ConverterWithPersianRug<C, T> for OptionConverterWithPersianRug
where
    C: persian_rug::Context
{
    type Output = Option<T>;
    fn convert<'b, B>(self, input: T, context: B) -> (Option<T>, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>
    {
        (Some(input), context)
    }
}

// So far nesting defeats me; but I only currently care about Option<Proxy<T>>
pub trait OptionProxyBuildableWithPersianRug<C>: Sized
where
    C: persian_rug::Context
{
    type Builder: BuilderWithPersianRug<C, Result = Option<persian_rug::Proxy<Self>>>;
    fn option_proxy_builder() -> Self::Builder;
}


#[persian_rug::constraints(context=C, access(T))]
impl<C, T> OptionBuildableWithPersianRug<C> for persian_rug::Proxy<T>
where
    T: OptionProxyBuildableWithPersianRug<C>,
    C: persian_rug::Context
{
    type Builder = <T as OptionProxyBuildableWithPersianRug<C>>::Builder;
    fn option_builder() -> Self::Builder {
        T::option_proxy_builder()
    }
}

#[doc(hidden)]
pub struct OptionProxyConverterWithPersianRug;

#[persian_rug::constraints(context=C, access(T))]
impl<C, T> ConverterWithPersianRug<C, T> for OptionProxyConverterWithPersianRug
{
    type Output = Option<persian_rug::Proxy<T>>;
    fn convert<'b, B>(self, input: T, mut context: B) -> (Option<persian_rug::Proxy<T>>, B)
    where
        B: 'b + persian_rug::Mutator<Context = C>
    {
        (Some(context.add(input)), context)
    }
}
