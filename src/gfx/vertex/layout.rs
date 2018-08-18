use std::fmt::Debug;
use std::marker::PhantomData;

use gl::types::*;

use gfx::vertex::Attribute;
use tlprog::{ TLNatural, Zero, Successor };

pub trait Layout : Debug + Copy {}

#[derive(Debug, Copy, Clone)]
pub struct Nil;
impl Layout for Nil {}

#[derive(Debug, Copy, Clone)]
pub struct Cons<CAR: Attribute, CDR: Layout>(GLint, PhantomData<CAR>, CDR);
impl<CAR: Attribute, CDR: Layout> Layout for Cons<CAR, CDR> {}

pub trait LayoutBuilder : Debug + Copy {
    type Layout: Layout;
    fn into<F: Fn(&str) -> GLint>(self, id_getter: F) -> Self::Layout;
}

#[derive(Debug, Copy, Clone)]
pub struct BuilderNil;
impl LayoutBuilder for BuilderNil {
    type Layout = Nil;
    fn into<F: Fn(&str) -> GLint>(self, _id_getter: F) -> Nil {
        Nil
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BuilderCons<'a, CAR: Attribute, CDR: LayoutBuilder>(&'a str, PhantomData<CAR>, CDR);
impl<'a, CAR: Attribute, CDR: LayoutBuilder> LayoutBuilder for BuilderCons<'a, CAR, CDR> {
    type Layout = Cons<CAR, CDR::Layout>;
    fn into<F: Fn(&str) -> GLint>(self, id_getter: F) -> Self::Layout {
        Cons(id_getter(self.0), PhantomData, self.2.into(id_getter))
    }
}

impl<'a, CAR: Attribute, CDR: LayoutBuilder> BuilderCons<'a, CAR, CDR> {
    pub fn new(name: &'a str, cdr: CDR) -> Self {
        BuilderCons(name, PhantomData, cdr)
    }
}

pub trait Pluck<A: Attribute, I: TLNatural> : Layout {
    type Remainder: Layout;

    fn pluck(self) -> (GLint, Self::Remainder);
}

impl<A: Attribute, CDR: Layout> Pluck<A, Zero> for Cons<A, CDR> {
    type Remainder = CDR;

    fn pluck(self) -> (GLint, CDR) {
        (self.0, self.2)
    }
}

impl<CAR, A, I, CDR> Pluck<A, Successor<I>> for Cons<CAR, CDR>
where
    CAR: Attribute,
    A: Attribute,
    I: TLNatural,
    CDR: Pluck<A, I>
{
    type Remainder = Cons<CAR, CDR::Remainder>;

    fn pluck(self) -> (GLint, Self::Remainder) {
        let t = self.2.pluck();
        (t.0, Cons(self.0, PhantomData, t.1))
    }
}
