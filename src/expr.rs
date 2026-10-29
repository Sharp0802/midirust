use std::fmt::{self, Display, Formatter};

use crate::{Context, Error};

#[derive(Clone, Debug)]
pub enum Value {}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Expr;

impl TryFrom<syn::Expr> for Expr {
    type Error = Error;

    fn try_from(value: syn::Expr) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Expr {
    pub fn eval(&self, cx: &mut Context) -> Result<Value, Error> {
        todo!()
    }
}
