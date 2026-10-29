use crate::Error;

#[derive(Clone, Debug, Default)]
pub struct Context;

impl Context {
    pub fn push(&mut self, item: syn::Item) -> Result<(), Error> {
        todo!()
    }

    pub fn merge(&mut self, cx: Self) -> Result<(), Error> {
        todo!()
    }
}
