//! 📐️ General boundary representation representation.
macro_rules! retire_geometry_variants {
    ($type:ty {$($variant:ident {$($field:ident),+}),+ $(,)?}) => {
        impl semio_framework_value::retirement::RetireOwned for $type {
            fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {
                match self {$ (Self::$variant {$($field),+}=>semio_framework_value::retirement::sequence(vec![$(semio_framework_value::retirement::deferred($field)),+])),+}
            }
            fn retirement_birth_bytes(&self)->Option<usize> {
                match self {$ (Self::$variant {$($field),+}=>semio_framework_value::retirement::sequence_birth_bytes(&[$(semio_framework_value::retirement::deferred_birth_bytes_for($field)),+])),+}
            }
            fn controlled_retirement_supported()->bool {true}
        }
    };
}
pub(crate) use retire_geometry_variants;
#[path = "🏟️arena/🦀️.rs"]
pub mod arena;
#[path = "➰️curve/🦀️.rs"]
pub mod curve;
#[path = "🚨️error/🦀️.rs"]
pub mod error;
#[path = "〰️polynomial/🦀️.rs"]
pub mod polynomial;
#[path = "🏄️surface/🦀️.rs"]
pub mod surface;
#[path = "📏️tolerance/🦀️.rs"]
pub mod tolerance;
#[path = "🕸️topology/🦀️.rs"]
pub mod topology;
#[path = "➡️vector/🦀️.rs"]
pub mod vector;
