//! 🧬️ AC1018 DWG logical mutations.

pub use crate::standards::v_ac1024::subsets::any::schema::mutations::*;

//#region 🧪️FixtureCases
/// 🧪️ The AC1018 fixture cases: this tree owns no leaves (its vocabulary is AC1024's), so its own drawings
/// exercise AC1024's leaves through this module path.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod tests_fixture;
//#endregion 🧪️FixtureCases
