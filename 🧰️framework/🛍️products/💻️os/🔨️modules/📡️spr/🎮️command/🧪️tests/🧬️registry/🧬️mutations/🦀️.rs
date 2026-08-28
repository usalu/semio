//! 🧬️ Transparent miniature mutation roster used by kernel integration tests.

//#region 🧬️Leaves
#[path = "📛️rename-mini/🦀️.rs"]
pub mod rename_mini;
pub use rename_mini::RenameMini;
//#endregion 🧬️Leaves

//#region 🧬️Aggregate
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::Mutations)]
#[serde(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = super::MiniDoc, diff = super::MiniDiff, schema = "mini.doc")]
pub enum MiniMutation {
    RenameMini(RenameMini),
}
//#endregion 🧬️Aggregate
