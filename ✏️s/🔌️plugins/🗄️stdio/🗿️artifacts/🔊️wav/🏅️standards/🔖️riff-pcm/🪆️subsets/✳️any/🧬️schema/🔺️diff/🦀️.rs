//! 🔺️ WavDiff — sparse per-field RIFF/WAVE diff, including the complete chunk sequence.

use crate::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavChunkRef, WavData, WavFmt, WavSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️Splices
/// ✂️ One sequential edit of the sample lane: `remove` elements from `index` replaced by `insert`, whose variant must be the lane's own. A diff applies its splices in order,
/// each addressing the lane the previous one left behind, so coalescing two diffs is plain concatenation.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WavSplice {
    pub index: u64,
    pub remove: u64,
    pub insert: WavData,
}

/// 🔢️ How many sample elements the lane holds.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn data_len(data: &WavData) -> usize {
    match data {
        WavData::Pcm16(values) => values.len(),
        WavData::Pcm8(values) | WavData::Raw(values) => values.len(),
        WavData::Float32(values) => values.len(),
    }
}

/// 🧱️ The elements `start..end` of the lane as a lane of the same variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn data_slice(data: &WavData, start: usize, end: usize) -> WavData {
    match data {
        WavData::Pcm16(values) => WavData::Pcm16(values[start..end].to_vec()),
        WavData::Pcm8(values) => WavData::Pcm8(values[start..end].to_vec()),
        WavData::Float32(values) => WavData::Float32(values[start..end].to_vec()),
        WavData::Raw(values) => WavData::Raw(values[start..end].to_vec()),
    }
}

/// ▶️ Replaces the elements `start..end` of `data` by `insert`; `false` when the range leaves the lane or `insert` is another variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn splice_lane(data: &mut WavData, start: usize, end: usize, insert: &WavData) -> bool {
    if start > end || end > data_len(data) {
        return false;
    }
    match (data, insert) {
        (WavData::Pcm16(values), WavData::Pcm16(insert)) => values.splice(start..end, insert.iter().copied()).for_each(drop),
        (WavData::Pcm8(values), WavData::Pcm8(insert)) | (WavData::Raw(values), WavData::Raw(insert)) => values.splice(start..end, insert.iter().copied()).for_each(drop),
        (WavData::Float32(values), WavData::Float32(insert)) => values.splice(start..end, insert.iter().copied()).for_each(drop),
        _ => return false,
    }
    true
}

/// ▶️ Applies the splices in order to `data`, refusing the first one that leaves the lane or changes its variant.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_splices(data: &mut WavData, splices: &[WavSplice]) -> Result<(), String> {
    for splice in splices {
        let (Ok(start), Ok(count)) = (usize::try_from(splice.index), usize::try_from(splice.remove)) else { return Err("WAV splice exceeds this platform".into()) };
        let Some(end) = start.checked_add(count) else { return Err("WAV splice range overflows".into()) };
        if !splice_lane(data, start, end, &splice.insert) {
            return Err("WAV splice is outside the sample lane or carries another sample kind".into());
        }
    }
    Ok(())
}

/// ↩️ The splices that undo `splices` on `data`, last first: each puts back the elements its forward splice removed and takes out the ones it inserted.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_splices(data: &WavData, splices: &[WavSplice]) -> Vec<WavSplice> {
    let mut running = (splices.len() > 1).then(|| data.clone());
    let mut undo = Vec::with_capacity(splices.len());
    for splice in splices {
        let lane = running.as_ref().unwrap_or(data);
        let (Ok(start), Ok(count)) = (usize::try_from(splice.index), usize::try_from(splice.remove)) else { break };
        let Some(end) = start.checked_add(count).filter(|end| *end <= data_len(lane) && start <= *end) else { break };
        undo.push(WavSplice { index: splice.index, remove: data_len(&splice.insert) as u64, insert: data_slice(lane, start, end) });
        if let Some(lane) = running.as_mut() {
            splice_lane(lane, start, end, &splice.insert);
        }
    }
    undo.reverse();
    undo
}

/// 🧭️ The common-prefix/common-suffix splice that carries lane `a` to lane `b` (`None` when equal), or `None` plus the whole lane when the variants differ.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_lanes(a: &WavData, b: &WavData) -> (Option<WavData>, Vec<WavSplice>) {
    fn trim<T: Clone>(a: &[T], b: &[T], same: impl Fn(&T, &T) -> bool) -> (usize, usize, Vec<T>) {
        let prefix = a.iter().zip(b).take_while(|(x, y)| same(x, y)).count();
        let suffix = a[prefix..].iter().rev().zip(b[prefix..].iter().rev()).take_while(|(x, y)| same(x, y)).count();
        (prefix, a.len() - prefix - suffix, b[prefix..b.len() - suffix].to_vec())
    }
    let (index, remove, insert) = match (a, b) {
        _ if a == b => return (None, Vec::new()),
        (WavData::Pcm16(x), WavData::Pcm16(y)) => {
            let (index, remove, insert) = trim(x, y, |p, q| p == q);
            (index, remove, WavData::Pcm16(insert))
        }
        (WavData::Pcm8(x), WavData::Pcm8(y)) => {
            let (index, remove, insert) = trim(x, y, |p, q| p == q);
            (index, remove, WavData::Pcm8(insert))
        }
        (WavData::Raw(x), WavData::Raw(y)) => {
            let (index, remove, insert) = trim(x, y, |p, q| p == q);
            (index, remove, WavData::Raw(insert))
        }
        (WavData::Float32(x), WavData::Float32(y)) => {
            let (index, remove, insert) = trim(x, y, |p, q| p.to_bits() == q.to_bits());
            (index, remove, WavData::Float32(insert))
        }
        _ => return (Some(b.clone()), Vec::new()),
    };
    (None, vec![WavSplice { index: index as u64, remove: remove as u64, insert }])
}
//#endregion 🔖️Splices

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct WavDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fmt: Option<WavFmt>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WavData>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub data_splices: Vec<WavSplice>,
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
    fn apply(&self, base: &WavSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WavSnapshot> {
        let mut next = base.clone();
        if let Some(v) = &self.fmt {
            next.fmt = v.clone();
        }
        if let Some(v) = &self.data {
            next.data = v.clone();
        }
        apply_splices(&mut next.data, &self.data_splices).map_err(|message| protocol::MutationApplyError::new("mutation.apply.invalid-splice", message).at(["data"]))?;
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
            self.data_splices.clear();
        }
        match &mut self.data {
            Some(owned) => {
                let _ = apply_splices(owned, &other.data_splices);
            }
            None => self.data_splices.extend(other.data_splices),
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
        let (data, data_splices) = between_lanes(&base.data, &other.data);
        WavDiff {
            fmt: (base.fmt != other.fmt).then(|| other.fmt.clone()),
            data,
            data_splices,
            fmt_pad_byte: (base.fmt_pad_byte != other.fmt_pad_byte).then_some(other.fmt_pad_byte),
            data_pad_byte: (base.data_pad_byte != other.data_pad_byte).then_some(other.data_pad_byte),
            other_chunks: (base.other_chunks != other.other_chunks).then(|| other.other_chunks.clone()),
            chunk_order: (base.chunk_order != other.chunk_order).then(|| other.chunk_order.clone()),
        }
    }
    /// 🔁️ Concrete diff-level undo: every replaced field takes the value `base` carries, and a spliced lane puts back what its splices removed, last splice first.
    fn inverse(&self, base: &WavSnapshot) -> Self {
        let lane_replaced = self.data.as_ref().is_some_and(|data| *data != base.data);
        WavDiff {
            fmt: self.fmt.as_ref().filter(|fmt| **fmt != base.fmt).map(|_| base.fmt.clone()),
            data: lane_replaced.then(|| base.data.clone()),
            data_splices: if lane_replaced { Vec::new() } else { inverse_splices(&base.data, &self.data_splices) },
            fmt_pad_byte: self.fmt_pad_byte.filter(|byte| *byte != base.fmt_pad_byte).map(|_| base.fmt_pad_byte),
            data_pad_byte: self.data_pad_byte.filter(|byte| *byte != base.data_pad_byte).map(|_| base.data_pad_byte),
            other_chunks: self.other_chunks.as_ref().filter(|chunks| **chunks != base.other_chunks).map(|_| base.other_chunks.clone()),
            chunk_order: self.chunk_order.as_ref().filter(|order| **order != base.chunk_order).map(|_| base.chunk_order.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.fmt.is_none() && self.data.is_none() && self.data_splices.is_empty() && self.fmt_pad_byte.is_none() && self.data_pad_byte.is_none() && self.other_chunks.is_none() && self.chunk_order.is_none()
    }
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
/// 🧩 Builds a set-other-chunks diff: the new chunk list plus the base chunk order reconciled with it.
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
/// ✂️ `diff` with every replaced field that `base` already carries dropped, so a mutation that changes nothing yields the empty diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sparse_against(base: &WavSnapshot, diff: WavDiff) -> WavDiff {
    WavDiff {
        fmt: diff.fmt.filter(|fmt| *fmt != base.fmt),
        data: diff.data.filter(|data| *data != base.data),
        fmt_pad_byte: diff.fmt_pad_byte.filter(|byte| *byte != base.fmt_pad_byte),
        data_pad_byte: diff.data_pad_byte.filter(|byte| *byte != base.data_pad_byte),
        other_chunks: diff.other_chunks.filter(|chunks| *chunks != base.other_chunks),
        chunk_order: diff.chunk_order.filter(|order| *order != base.chunk_order),
        data_splices: diff.data_splices,
    }
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
