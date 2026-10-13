//! 💡️ Non-mutating boundary-representation queries of the 3D kernel: bounding volumes, point classification,
//! curve and surface intersection, mass properties, tessellation, validation and shape analysis.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-13-wave-e4-3d-architect-bim.md`).

extern crate semio_framework_value_derive as value_derive;
#[path = "🌳bounding-volume/🦀️.rs"]
pub mod bounding_volume;
#[path = "🏷️classification/🦀️.rs"]
pub mod classification;
#[path = "📏mass-properties/🦀️.rs"]
pub mod mass_properties;
#[path = "🧩tessellation/🦀️.rs"]
pub mod tessellation;
#[path = "✅validation/🦀️.rs"]
pub mod validation;
#[path = "🔎️analysis/🦀️.rs"]
pub mod analysis;
#[path = "✂️intersect/🦀️.rs"]
pub mod intersect;

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
