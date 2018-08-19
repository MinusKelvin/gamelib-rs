use std::fmt::Debug;
use std::marker::PhantomData;

use gl::types::*;

use gfx::shader::GlslType;
use tlprog::{ TLNatural, Zero, Successor };

pub trait Uniform : Debug + Copy {
    type Type: GlslType;
}

#[macro_export]
macro_rules! uniforms {
    (pub $name:ident: $type:ty, $($rest:tt)+) => (uniforms!(pub $name: $type); uniforms!($($rest)*););
    ($name:ident: $type:ty, $($rest:tt)+) => (uniforms!($name: $type); uniforms!($($rest)*););
    (pub $name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        pub struct $name;
        impl $crate::gfx::shader::Uniform for $name {
            type Type = $type;
        }
    );
    ($name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        struct $name;
        impl $crate::gfx::shader::Uniform for $name {
            type Type = $type;
        }
    )
}

pub trait UniformList : Debug + Copy {}

#[derive(Debug, Copy, Clone)]
pub struct Nil; impl UniformList for Nil {}

#[derive(Debug, Copy, Clone)]
pub struct Cons<U: Uniform, CDR: UniformList>(i32, PhantomData<U>, CDR);
impl<U: Uniform, CDR: UniformList> UniformList for Cons<U, CDR> {}

pub trait UniformListBuilder : Debug + Copy {
    type UniformList: UniformList;
    fn into<F: Fn(&str) -> GLint>(self, id_getter: F) -> Self::UniformList;
}

#[derive(Debug, Copy, Clone)]
pub struct BuilderNil;
impl UniformListBuilder for BuilderNil {
    type UniformList = Nil;
    fn into<F: Fn(&str) -> GLint>(self, _id_getter: F) -> Nil {
        Nil
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BuilderCons<'a, U: Uniform, CDR: UniformListBuilder>(&'a str, PhantomData<U>, CDR);
impl<'a, U: Uniform, CDR: UniformListBuilder> UniformListBuilder for BuilderCons<'a, U, CDR> {
    type UniformList = Cons<U, CDR::UniformList>;
    fn into<F: Fn(&str) -> GLint>(self, id_getter: F) -> Self::UniformList {
        Cons(id_getter(self.0), PhantomData, self.2.into(id_getter))
    }
}

impl<'a, U: Uniform, CDR: UniformListBuilder> BuilderCons<'a, U, CDR> {
    pub fn new(name: &'a str, cdr: CDR) -> Self {
        BuilderCons(name, PhantomData, cdr)
    }
}

pub trait Pluck<U: Uniform, I: TLNatural> : UniformList {
    type Remainder: UniformList;

    fn pluck(self) -> (i32, Self::Remainder);
}

impl<U: Uniform, CDR: UniformList> Pluck<U, Zero> for Cons<U, CDR> {
    type Remainder = CDR;

    fn pluck(self) -> (i32, CDR) {
        (self.0, self.2)
    }
}

impl<T: Uniform, U: Uniform, I: TLNatural, CDR: Pluck<T, I>> Pluck<T, Successor<I>> for Cons<U, CDR> {
    type Remainder = Cons<U, CDR::Remainder>;

    fn pluck(self) -> (i32, Self::Remainder) {
        let (idx, r) = self.2.pluck();
        (idx, Cons(self.0, self.1, r))
    }
}
