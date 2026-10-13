//! 🛠️ Topology-mutating boundary-representation operations of the 3D kernel: the checked euler editors, primitives,
//! staged drivers, booleans, blends, offsets, sweeps, sewing and transforms.
//!
//! Crate cut of `semio-framework-3d` (wave E4, `📓️2026-10-13-wave-e4-3d-architect-bim.md`).
#[path = "🎨️blend/🦀️.rs"]
pub mod blend;
#[path = "🔀️boolean/🦀️.rs"]
pub mod boolean;
#[path = "🔺️euler/🦀️.rs"]
pub mod euler;
#[path = "↔️offset/🦀️.rs"]
pub mod offset;
#[path = "🧱️primitives/🦀️.rs"]
pub mod primitives;
#[path = "⏱️staged/🦀️.rs"]
pub mod staged;
#[path = "🧵️sew/🦀️.rs"]
pub mod sew;
#[path = "➡️sweep/🦀️.rs"]
pub mod sweep;
#[path = "🔁️transform/🦀️.rs"]
pub mod transform;

//#region 🧪️TestSupport
#[cfg(test)]
#[path = "../../🧪️tests/🧰️support/🦀️.rs"]
pub(crate) mod test_support;
//#endregion 🧪️TestSupport
