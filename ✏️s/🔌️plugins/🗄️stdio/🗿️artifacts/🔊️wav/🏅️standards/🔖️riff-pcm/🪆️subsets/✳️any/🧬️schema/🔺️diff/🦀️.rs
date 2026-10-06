//! 🔺️ WavDiff — sparse per-field RIFF/WAVE diff, including the complete chunk sequence.

use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WavDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fmt: Option<WavFmt>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WavData>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fmt_pad_byte: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data_pad_byte: Option<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub other_chunks: Option<Vec<RiffChunk>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub chunk_order: Option<Vec<WavChunkRef>>,
}

impl MutationDiff<WavSnapshot> for WavDiff {
    fn apply(&self, base: &WavSnapshot) -> protocol::MutationApplyResult<WavSnapshot> {
        let mut next = base.clone();
        if let Some(v) = &self.fmt {
            next.fmt = v.clone();
        }
        if let Some(v) = &self.data {
            next.data = v.clone();
        }
        if let Some(v) = self.fmt_pad_byte {
            next.fmt_pad_byte = v;
        }
        if let Some(v) = self.data_pad_byte {
            next.data_pad_byte = v;
        }
        if let Some(v) = &self.other_chunks {
            next.other_chunks = v.clone();
        }
        if let Some(v) = &self.chunk_order {
            next.chunk_order = v.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.fmt.is_some() {
            self.fmt = other.fmt;
        }
        if other.data.is_some() {
            self.data = other.data;
        }
        if other.fmt_pad_byte.is_some() {
            self.fmt_pad_byte = other.fmt_pad_byte;
        }
        if other.data_pad_byte.is_some() {
            self.data_pad_byte = other.data_pad_byte;
        }
        if other.other_chunks.is_some() {
            self.other_chunks = other.other_chunks;
        }
        if other.chunk_order.is_some() {
            self.chunk_order = other.chunk_order;
        }
    }
}

impl DiffAlgebra<WavSnapshot> for WavDiff {
    fn between(base: &WavSnapshot, other: &WavSnapshot) -> Self {
        WavDiff { fmt: (base.fmt != other.fmt).then(|| other.fmt.clone()), data: (base.data != other.data).then(|| other.data.clone()), fmt_pad_byte: (base.fmt_pad_byte != other.fmt_pad_byte).then_some(other.fmt_pad_byte), data_pad_byte: (base.data_pad_byte != other.data_pad_byte).then_some(other.data_pad_byte), other_chunks: (base.other_chunks != other.other_chunks).then(|| other.other_chunks.clone()), chunk_order: (base.chunk_order != other.chunk_order).then(|| other.chunk_order.clone()) }
    }
    fn inverse(&self, base: &WavSnapshot) -> Self {
        WavDiff { fmt: self.fmt.as_ref().map(|_| base.fmt.clone()), data: self.data.as_ref().map(|_| base.data.clone()), fmt_pad_byte: self.fmt_pad_byte.map(|_| base.fmt_pad_byte), data_pad_byte: self.data_pad_byte.map(|_| base.data_pad_byte), other_chunks: self.other_chunks.as_ref().map(|_| base.other_chunks.clone()), chunk_order: self.chunk_order.as_ref().map(|_| base.chunk_order.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.fmt.is_none() && self.data.is_none() && self.fmt_pad_byte.is_none() && self.data_pad_byte.is_none() && self.other_chunks.is_none() && self.chunk_order.is_none()
    }
}

/// 🧩 Builds a set-snapshot diff: the sparse field-by-field delta, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &WavSnapshot, snapshot: &WavSnapshot) -> WavDiff {
    WavDiff::between(base, snapshot)
}
/// 🧩 Builds a set-fmt diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_fmt(fmt: WavFmt) -> WavDiff {
    WavDiff { fmt: Some(fmt), ..Default::default() }
}
/// 🧩 Builds a set-data diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_data(data: WavData) -> WavDiff {
    WavDiff { data: Some(data), ..Default::default() }
}
/// 🧩 Builds a set-other-chunks diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_other_chunks(base: &WavSnapshot, chunks: Vec<RiffChunk>) -> WavDiff {
    let mut referenced = vec![false; chunks.len()];
    let mut chunk_order = Vec::with_capacity(base.chunk_order.len().max(chunks.len() + 2));
    for reference in &base.chunk_order {
        match reference {
            WavChunkRef::Other(index) => {
                let Ok(index) = usize::try_from(*index) else { continue };
                if index >= chunks.len() { continue; }
                referenced[index] = true;
                chunk_order.push(reference.clone());
            }
            _ => chunk_order.push(reference.clone()),
        }
    }
    for (index, was_referenced) in referenced.into_iter().enumerate() {
        if !was_referenced { chunk_order.push(WavChunkRef::Other(index as u64)); }
    }
    WavDiff { other_chunks: Some(chunks), chunk_order: (chunk_order != base.chunk_order).then_some(chunk_order), ..Default::default() }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🧪️ Hand-rolled `protocol::DiffCodec` (per ticket `26/08/11/…-RETIREMENT`'s mandate: no
/// `#[derive(dsl::DslDiff)]` here — `WavData` is a data-carrying enum, the same shape `f6-final-
/// summary.md` §4.4 documents as structurally unbindable by the derive machinery today; hand-
/// rolled following `DeflateDiff`'s own grammar template, `f6-recon-report.md` §5's primitive
/// set copied verbatim). Grammar: one space-separated `name=value` token per changed top-level
/// field (a field absent from the line = unchanged); `WavFmt`/`RiffChunk` values are their own
/// bracketed `[a,b,c,…]` tuple; `WavData` values are `tag:hex` (`p16`/`p8`/`f32`/`raw`); a chunk
/// list is `[chunk1;chunk2;…]`. Worked example:
/// `fmt=[1,1,8000,16000,2,16,[0]] data=p16:0100feff`.
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
