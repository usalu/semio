//! 🔺️ Mp3Diff — sparse per-field MPEG1/ID3 container diff. `id3v2`/`id3v1` are independently
//! nullable in `Mp3Snapshot` (a tag may be added or removed entirely), so both are the
//! `DeflateDiff::dict_id`-style tri-state `Option<Option<T>>` (`None` = unchanged, `Some(None)` =
//! tag cleared, `Some(Some(tag))` = tag set/changed); `frames` is a plain `Option<Vec<_>>`
//! "changed or not" slot, same shape as `DeflateDiff::payload`.

use crate::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader, Mp3Snapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Mp3Diff {
    /// 🪆️ Tri-state: `None` = unchanged, `Some(None)` = id3v2 tag cleared, `Some(Some(tag))` =
    /// tag set/changed to `tag`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id3v2: Option<Option<Id3v2Tag>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<Mp3Frame>>,
    /// 🪆️ Tri-state, same shape as `id3v2`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub id3v1: Option<Option<Id3v1Tag>>,
}

impl MutationDiff<Mp3Snapshot> for Mp3Diff {
    fn apply(&self, base: &Mp3Snapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Mp3Snapshot> {
        let mut next = base.clone();
        if let Some(v) = &self.id3v2 {
            next.id3v2 = v.clone();
        }
        if let Some(v) = &self.frames {
            next.frames = v.clone();
        }
        if let Some(v) = &self.id3v1 {
            next.id3v1 = v.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.id3v2.is_some() {
            self.id3v2 = other.id3v2;
        }
        if other.frames.is_some() {
            self.frames = other.frames;
        }
        if other.id3v1.is_some() {
            self.id3v1 = other.id3v1;
        }
    }
}

impl DiffAlgebra<Mp3Snapshot> for Mp3Diff {
    fn between(base: &Mp3Snapshot, other: &Mp3Snapshot) -> Self {
        Mp3Diff { id3v2: (base.id3v2 != other.id3v2).then(|| other.id3v2.clone()), frames: (base.frames != other.frames).then(|| other.frames.clone()), id3v1: (base.id3v1 != other.id3v1).then(|| other.id3v1.clone()) }
    }
    fn inverse(&self, base: &Mp3Snapshot) -> Self {
        Mp3Diff { id3v2: self.id3v2.as_ref().map(|_| base.id3v2.clone()), frames: self.frames.as_ref().map(|_| base.frames.clone()), id3v1: self.id3v1.as_ref().map(|_| base.id3v1.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.id3v2.is_none() && self.frames.is_none() && self.id3v1.is_none()
    }
}

/// 🧩 Builds a set-id3v2 diff (`None` clears the tag).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_id3v2(id3v2: Option<Id3v2Tag>) -> Mp3Diff {
    Mp3Diff { id3v2: Some(id3v2), ..Default::default() }
}
/// 🧩 Builds a set-frames diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_frames(frames: Vec<Mp3Frame>) -> Mp3Diff {
    Mp3Diff { frames: Some(frames), ..Default::default() }
}
/// 🧩 Builds a set-id3v1 diff (`None` clears the tag).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_id3v1(id3v1: Option<Id3v1Tag>) -> Mp3Diff {
    Mp3Diff { id3v1: Some(id3v1), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` (per ticket `26/08/11/…-RETIREMENT`'s mandate: no
/// `#[derive(dsl::DslDiff)]` — `Mp3Frame`/`Id3v2Tag` embed nested collections of named structs,
/// the same generic-collection-diff shape `f6-final-summary.md` §4.4 documents as needing a
/// hand-rolled bridge; hand-rolled following `DeflateDiff`'s own tri-state grammar template,
/// `f6-recon-report.md` §5's primitive set copied verbatim). Grammar: one space-separated
/// `name=value` token per changed top-level field; `id3v2`/`id3v1` use the uniform
/// `[0]`=unchanged-inner-None / `[1,<T>]`=inner-Some(T) tri-state tag via `encode_option`/
/// `decode_option`; `frames` is a `[frame1;frame2;…]` list.
//#region 🔖️Primitives










//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs

















//#endregion 🔖️ValueCodecs

//#region 🔖️TopLevel




//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
