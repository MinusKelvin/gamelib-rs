use std::marker::PhantomData;
use std::mem::size_of;
use std::fmt;

use graphics::vertex::{ Attribute, Format, Layout };
use graphics::vertex::layout;
use tlprog::{ TLNatural, Zero, Successor };

#[repr(C)]
#[repr(align(4))]
#[derive(Copy, Clone, Debug)]
struct Align<T: Copy>(T);

pub trait Struct : fmt::Debug + Copy {
    type Layout: Layout;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Atom<CAR1, CAR2>(PhantomData<CAR1>, Align<CAR2::Concrete>)
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>;

impl<CAR1, CAR2> Struct for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>
{
    type Layout = layout::Cons<CAR1, layout::Nil>;
}

impl<CAR1: Attribute, CAR2: Format<Type=CAR1::Type>> Atom<CAR1, CAR2> {
    pub fn new(_car1: CAR1, car2: CAR2::Concrete) -> Self {
        Atom(PhantomData, Align(car2))
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Cons<CAR1, CAR2, CDR>(PhantomData<CAR1>, Align<CAR2::Concrete>, CDR)
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Struct;

impl<CAR1, CAR2, CDR> Struct for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Struct
{
    type Layout = layout::Cons<CAR1, CDR::Layout>;
}

impl<CAR1: Attribute, CAR2: Format<Type=CAR1::Type>, CDR: Struct> Cons<CAR1, CAR2, CDR> {
    pub fn new(_car1: CAR1, car2: CAR2::Concrete, cdr: CDR) -> Self {
        Cons(PhantomData, Align(car2), cdr)
    }
}

pub trait GetAttribute<A: Attribute, I: TLNatural> : Struct {
    type Value;
    fn get(&self, _name: A) -> &Self::Value;
    fn get_mut(&mut self, _name: A) -> &mut Self::Value;
}

impl<CAR1, CAR2> GetAttribute<CAR1, Zero> for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>
{
    type Value = CAR2::Concrete;

    fn get(&self, _name: CAR1) -> &CAR2::Concrete {
        &(self.1).0
    }

    fn get_mut(&mut self, _name: CAR1) -> &mut CAR2::Concrete {
        &mut (self.1).0
    }
}

impl<CAR1, CAR2, CDR> GetAttribute<CAR1, Zero> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute,
    CAR2: Format<Type=CAR1::Type>,
    CDR: Struct
{
    type Value = CAR2::Concrete;

    fn get(&self, _name: CAR1) -> &Self::Value {
        &(self.1).0
    }

    fn get_mut(&mut self, _name: CAR1) -> &mut Self::Value {
        &mut (self.1).0
    }
}

impl<CAR1, CAR2, CDR, A, I> GetAttribute<A, Successor<I>> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: GetAttribute<A, I>,
    A: Attribute,
    I: TLNatural
{
    type Value = CDR::Value;

    fn get(&self, name: A) -> &Self::Value {
        self.2.get(name)
    }

    fn get_mut(&mut self, name: A) -> &mut Self::Value {
        self.2.get_mut(name)
    }
}

pub trait Offset<A: Attribute, F: Format<Type=A::Type>, I: TLNatural> : Struct {
    const OFFSET: usize;
}

impl<CAR1, CAR2> Offset<CAR1, CAR2, Zero> for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>
{
    const OFFSET: usize = 0;
}

impl<CAR1, CAR2, CDR> Offset<CAR1, CAR2, Zero> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Struct
{
    const OFFSET: usize = 0;
}

impl<A, F, CAR1, CAR2, CDR, I> Offset<A, F, Successor<I>> for Cons<CAR1, CAR2, CDR>
where
    A: Attribute, F: Format<Type=A::Type>,
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Offset<A, F, I>,
    I: TLNatural
{
    const OFFSET: usize = size_of::<Align<CAR2::Concrete>>() + CDR::OFFSET;
}

pub trait TList {}
pub struct TListAtom<CAR>(PhantomData<CAR>);
impl<CAR> TList for TListAtom<CAR> {}
pub struct TListCons<CAR, CDR: TList>(PhantomData<CAR>, PhantomData<CDR>);
impl<CAR, CDR: TList> TList for TListCons<CAR, CDR> {}

pub trait Swap<CAR1, CAR2, TCAR1, TCAR2> : Struct + Sized
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    TCAR1: Attribute, TCAR2: Format<Type=TCAR1::Type>
{
    type Output: Struct;
    fn swap(self, other: Cons<TCAR1, TCAR2, Self>) -> Cons<CAR1, CAR2, Self::Output>;
}

impl<CAR1, CAR2, TCAR1, TCAR2> Swap<CAR1, CAR2, TCAR1, TCAR2> for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    TCAR1: Attribute, TCAR2: Format<Type=TCAR1::Type>
{
    type Output = Atom<TCAR1, TCAR2>;
    fn swap(self, other: Cons<TCAR1, TCAR2, Self>) -> Cons<CAR1, CAR2, Self::Output> {
        Cons(self.0, self.1, Atom(other.0, other.1))
    }
}

impl<CAR1, CAR2, CDR, TCAR1, TCAR2> Swap<CAR1, CAR2, TCAR1, TCAR2> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Struct,
    TCAR1: Attribute, TCAR2: Format<Type=TCAR1::Type>
{
    type Output = Cons<TCAR1, TCAR2, CDR>;
    fn swap(self, other: Cons<TCAR1, TCAR2, Self>) -> Cons<CAR1, CAR2, Self::Output> {
        Cons(self.0, self.1, Cons(other.0, other.1, self.2))
    }
}

pub trait BringToFront<CAR1, CAR2, I> : Struct
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    I: TLNatural
{
    type Output: Struct;
    fn bring_to_front(self) -> Self::Output;
}

impl<CAR1, CAR2> BringToFront<CAR1, CAR2, Zero> for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>
{
    type Output = Self;
    fn bring_to_front(self) -> Self {
        self
    }
}

impl<CAR1, CAR2, CDR> BringToFront<CAR1, CAR2, Zero> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: Struct
{
    type Output = Self;
    fn bring_to_front(self) -> Self {
        self
    }
}

impl<CAR1, CAR2, CDR, TCAR1, TCAR2, I> BringToFront<TCAR1, TCAR2, Successor<I>> for Cons<CAR1, CAR2, CDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    CDR: BringToFront<TCAR1, TCAR2, I>,
    TCAR1: Attribute, TCAR2: Format<Type=TCAR1::Type>,
    I: TLNatural,
    <CDR as BringToFront<TCAR1, TCAR2, I>>::Output: Swap<TCAR1, TCAR2, CAR1, CAR2>
{
    type Output = Cons<TCAR1, TCAR2, <<CDR as BringToFront<TCAR1, TCAR2, I>>::Output as Swap<TCAR1, TCAR2, CAR1, CAR2>>::Output>;
    fn bring_to_front(self) -> Self::Output {
        let t = Cons(self.0, self.1, self.2.bring_to_front());
        t.2.swap(t)
    }
}

pub trait Reshape<TI: TList> : Struct {
    type Output: Struct;
    fn reshape(self) -> Self::Output;
}

impl<CAR1, CAR2> Reshape<TListAtom<(CAR1, CAR2, Zero)>> for Atom<CAR1, CAR2>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>
{
    type Output = Self;
    fn reshape(self) -> Self {
        self
    }
}

impl<CAR1, CAR2, QCDR, TCAR1, TCAR2, OCDR, I, TI> Reshape<TListCons<(TCAR1, TCAR2, I), TI>> for Cons<CAR1, CAR2, QCDR>
where
    CAR1: Attribute, CAR2: Format<Type=CAR1::Type>,
    QCDR: Struct,
    TCAR1: Attribute, TCAR2: Format<Type=TCAR1::Type>,
    OCDR: Reshape<TI>,
    I: TLNatural,
    TI: TList,
    Self: BringToFront<TCAR1, TCAR2, I, Output=Cons<TCAR1, TCAR2, OCDR>>
{
    type Output = Cons<TCAR1, TCAR2, OCDR::Output>;
    fn reshape(self) -> Self::Output {
        let t = self.bring_to_front();
        Cons(t.0, t.1, t.2.reshape())
    }
}

#[macro_export]
macro_rules! vertex_struct_type {
    ($attr:ty: $format:ty) => ($crate::graphics::vertex::structure::Atom<$attr, $format>);
    ($attr:ty: $format:ty, $($rest:tt)*) => ($crate::graphics::vertex::structure::Cons<$attr, $format, vertex_struct_type!($($rest)*)>);
}

#[macro_export]
macro_rules! vertex_struct {
    ($attr:path: $value:expr) => ($crate::graphics::vertex::structure::Atom::new($attr, $value));
    ($attr:path: $value:expr, $($rest:tt)*) => ($crate::graphics::vertex::structure::Cons::new($attr, $value, vertex_struct!($($rest)*)));
}
