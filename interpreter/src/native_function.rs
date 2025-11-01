use crate::value::InterpreterValue;
use effy_base::error::EffyResult;

pub struct NativeFunction {
    name: String,
    function: Box<dyn NativeFunctionTrait>,
}

impl NativeFunction {
    pub fn new(name: impl Into<String>, function: impl NativeFunctionTrait) -> Self {
        Self {
            name: name.into(),
            function: Box::new(function),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn invoke(&self, context: &mut NativeFunctionContext) -> EffyResult<InterpreterValue> {
        self.function.invoke(context)
    }
}

pub struct NativeFunctionContext {}
pub trait NativeFunctionTrait: Send + Sync + 'static {
    fn invoke(&self, context: &mut NativeFunctionContext) -> EffyResult<InterpreterValue>;
}

impl<F> NativeFunctionTrait for F
where
    F: for<'a> Fn(&'a mut NativeFunctionContext) -> EffyResult<InterpreterValue>
        + Send
        + Sync
        + 'static,
{
    fn invoke(&self, context: &mut NativeFunctionContext) -> EffyResult<InterpreterValue> {
        (self)(context)
    }
}
/*
impl<F> NativeFunctionTrait for F
where
    F: 'static + for<'a> FnMut(&'a mut NativeFunctionContext) -> EffyResult<InterpreterValue>,
{
    fn call<'a>(&mut self, context: &'a mut NativeFunctionContext) -> EffyResult<InterpreterValue> {
        self(context)
    }
}
*/

/*
impl <F> NativeFunctionTrait for F where F: for<'a> FnMut(&'a mut NativeFunctionContext) -> EffyResult<InterpreterValue> + 'static {
    fn call(&mut self, context: &mut NativeFunctionContext) -> EffyResult<InterpreterValue> {
        (self)(context)
    }
}

 */
