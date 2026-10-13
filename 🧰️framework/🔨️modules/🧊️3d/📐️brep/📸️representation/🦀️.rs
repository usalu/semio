//! 📸️ Boundary-representation vocabulary of the 3D kernel: generational arena ids, analytic and NURBS curves and
//! surfaces, f64 vectors and frames, the `Body` topology with its seed, history, reachability and compaction, tolerances,
//! kernel errors and the neutral kernel contract (`🔖️contract`). Every other brep crate builds on these types.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-13-wave-e4-3d-architect-bim.md`).

extern crate semio_framework_value_derive as value_derive;
/// ♻️ Implements `RetireOwned` for a geometry enum by retiring every field of its matching variant.
#[macro_export]
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
#[path = "🏟️arena/🦀️.rs"]
pub mod arena;
#[path = "🔖️contract/🦀️.rs"]
pub mod contract;
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

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
