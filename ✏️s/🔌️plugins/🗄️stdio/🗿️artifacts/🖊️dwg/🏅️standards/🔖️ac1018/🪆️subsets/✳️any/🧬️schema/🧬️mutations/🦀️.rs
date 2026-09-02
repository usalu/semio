//! 🧬️ AC1018 DWG logical mutations.

pub use crate::artifacts::dwg::standards::v_ac1024::subsets::any::schema::mutations::*;

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases for the AC1018 tree, wired from this tree's own
/// mutations root so `🦀️.rs` stays untouched (`#[path]` on a non-inline module resolves
/// against this file's own directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/bumps-the-auxiliary-save-counter/🦀️.rs"]
mod set_snapshot_bumps_the_auxiliary_save_counter;
//#endregion 🧪️FixtureCases
