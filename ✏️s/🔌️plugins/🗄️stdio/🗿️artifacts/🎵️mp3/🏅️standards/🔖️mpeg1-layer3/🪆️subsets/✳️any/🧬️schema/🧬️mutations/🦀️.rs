//! 🧬️ Mp3Mutation — the real per-field mutation vocabulary over `Mp3Snapshot`'s three
//! top-level fields (`id3v2`/`frames`/`id3v1`).

//#region 🔖️Leaves
#[path = "🎼️set-frames/🦀️.rs"]
pub mod set_frames;
#[path = "🔖️set-id3v1/🦀️.rs"]
pub mod set_id3v1;
#[path = "🏷️set-id3v2/🦀️.rs"]
pub mod set_id3v2;
//#endregion 🔖️Leaves

use crate::standards::mpeg1_layer3::subsets::any::schema::diff::{diff_set_frames, diff_set_id3v1, diff_set_id3v2, Mp3Diff};
use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3Snapshot};
use protocol::Mutation;


//#region 🔖️Mutation
/// 📐️ Typed mutation for this subset. `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires
/// every variant to wrap exactly one leaf payload and a unit variant wraps none.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = Mp3Snapshot, diff = Mp3Diff, schema = "Mp3Mutation")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum Mp3Mutation {
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v2 tag wholesale.
    SetId3v2(set_id3v2::SetId3v2),
    /// 🎼️ Replaces the MPEG frame sequence wholesale.
    SetFrames(set_frames::SetFrames),
    /// 🏷️ Sets (`Some`) or clears (`None`) the ID3v1 trailer wholesale.
    SetId3v1(set_id3v1::SetId3v1),
}



//#endregion 🔖️Mutation


//#region 🔖️Kinds
impl Mp3Mutation {
    /// 🏷️ Kebab-case kind spelling — the exact vocabulary `../../🔣️oracle.json`'s
    /// `mutationCatalogs[].kinds` declares and `🎛️mutate-mp3-mpeg1-layer3`'s Scenario Outline row ids
    /// equal. Hand-matched rather than derived, so [`KINDS`] is checked against something with its
    /// own reason to be right; and exhaustive, so a variant added to the enum is a COMPILE error
    /// here rather than a silently uncatalogued kind.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn kind(&self) -> &'static str {
        match self {
            Mp3Mutation::SetId3v2(_) => "set-id3v2",
            Mp3Mutation::SetFrames(_) => "set-frames",
            Mp3Mutation::SetId3v1(_) => "set-id3v1",
        }
    }
}

/// 🏷️ Every declared kind, kebab-case, in the enum's own declaration order — mirrors the catalog's
/// `mutationCatalogs[].kinds` exactly.
pub const KINDS: &[&str] = &["set-id3v2", "set-frames", "set-id3v1"];
//#endregion 🔖️Kinds

//#region OpCodecs





//#endregion OpCodecs

//#endregion 🔖️MutationTrait

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests


#[cfg(test)]
use protocol::{OpBinary,OpText};
