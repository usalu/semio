//! 💡️ Measuring boundary-representation queries of the 3D kernel: bounding volumes, point classification and mass
//! properties, which close over each other. Intersection, tessellation, validation and analysis are their own crates
//! (`semio-framework-3d-brep-{intersect,tessellation,validation,analysis}`).
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-14-wave-e4-3d-architect-bim.md`).

#[path = "🌳bounding-volume/🦀️.rs"]
pub mod bounding_volume;
#[path = "🏷️classification/🦀️.rs"]
pub mod classification;
#[path = "📏mass-properties/🦀️.rs"]
pub mod mass_properties;
