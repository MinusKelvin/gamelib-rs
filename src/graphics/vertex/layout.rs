use std::fmt::Debug;
use std::marker::PhantomData;

use graphics::vertex::Attribute;
use tlprog::{ TLNatural, Zero, Successor };

pub trait Layout : Debug + Copy {}

#[derive(Debug, Copy, Clone)]
pub struct Nil;
impl Layout for Nil {}

#[derive(Debug, Copy, Clone)]
pub struct Cons<CAR: Attribute, CDR: Layout>(u8, PhantomData<CAR>, CDR);
impl<CAR: Attribute, CDR: Layout> Layout for Cons<CAR, CDR> {}

impl<CAR: Attribute, CDR: Layout> Cons<CAR, CDR> {
    pub fn new(car: u8, cdr: CDR) -> Self {
        Cons(car, PhantomData, cdr)
    }
}

pub trait Pluck<A: Attribute, I: TLNatural> : Layout {
    type Remainder: Layout;

    fn pluck(self) -> (u8, Self::Remainder);
}

impl<A: Attribute, CDR: Layout> Pluck<A, Zero> for Cons<A, CDR> {
    type Remainder = CDR;

    fn pluck(self) -> (u8, CDR) {
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

    fn pluck(self) -> (u8, Self::Remainder) {
        let t = self.2.pluck();
        (t.0, Cons::new(self.0, t.1))
    }
}

#[macro_export]
macro_rules! layout_indexes {
    ($v:expr, $($rest:tt)+) => ($crate::graphics::vertex::layout::Cons::new($v, layout_indexes!($($rest)*)));
    ($v:expr) => ($crate::graphics::vertex::layout::Cons::new($v, $crate::graphics::vertex::layout::Nil));
    () => ($crate::graphics::vertex::layout::Nil);
}
