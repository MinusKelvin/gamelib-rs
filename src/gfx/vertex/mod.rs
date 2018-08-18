#[macro_export]
macro_rules! vertex_attributes {
    (pub $name:ident: $type:ty, $($rest:tt)+) => (vertex_attributes!(pub $name: $type); vertex_attributes!($($rest)*););
    ($name:ident: $type:ty, $($rest:tt)+) => (vertex_attributes!($name: $type); vertex_attributes!($($rest)*););
    (pub $name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        pub struct $name;
        impl $crate::gfx::vertex::Attribute for $name {
            type Type = $type;
        }
    );
    ($name:ident: $type:ty) => (
        #[derive(Debug,Copy,Clone)]
        struct $name;
        impl $crate::gfx::vertex::Attribute for $name {
            type Type = $type;
        }
    )
}

pub mod format;
pub use self::format::Format;

pub mod layout;
pub use self::layout::Layout;
pub use self::layout::LayoutBuilder;

pub mod structure;
pub use self::structure::Struct;

use std::fmt::Debug;

use gfx::GlslType;

pub trait Attribute : Debug + Copy {
    type Type: GlslType;
}
