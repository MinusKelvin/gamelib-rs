#[macro_export]
macro_rules! vertex_attributes {
    (pub $name:ident: $type:ty, $($rest:tt)+) => (vertex_attributes!(pub $name: $type); vertex_attributes!($($rest)*););
    ($name:ident: $type:ty, $($rest:tt)+) => (vertex_attributes!($name: $type); vertex_attributes!($($rest)*););
    (pub $name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        pub struct $name;
        impl $crate::graphics::vertex::Attribute for $name {
            type Type = $type;
        }
    );
    ($name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        struct $name;
        impl $crate::graphics::vertex::Attribute for $name {
            type Type = $type;
        }
    )
}

pub mod format;
pub use self::format::Format;

pub mod layout;
pub use self::layout::Layout;

pub mod structure;
pub use self::structure::Struct;

use std::fmt::Debug;

use graphics::GlslType;

pub trait Attribute : Debug + Copy {
    type Type: GlslType;
}
