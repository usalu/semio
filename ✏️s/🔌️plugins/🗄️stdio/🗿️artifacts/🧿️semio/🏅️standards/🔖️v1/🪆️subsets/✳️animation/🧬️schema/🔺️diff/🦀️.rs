//! 🔺️ SemioAnimationDiff — sparse per-field diff, handcrafted per `🧬️schema-design.md`. Replaces
//! the W1b `replacement: Option<SemioAnimationSnapshot>` full-replace scaffold wholesale (this file
//! carries NO such slot anywhere). `timelines` -> `channels` -> `keyframes` is a 3-level nested
//! index-keyed collection tree (gltf's own top-level arrays are index-keyed, not name-keyed — glTF
//! `animation.name` is optional and not spec-required to be unique, so timelines/channels/keyframes
//! all key by position, per `🧬️schema-design.md`'s "Key kinds per collection"). The generic
//! index-transport/absorb/inverse algebra below (`between_indexed`/`apply_indexed`/`absorb_indexed`/
//! `inverse_indexed`) is written ONCE and reused at all 3 nesting depths — ported from the gif 89a
//! `GifFramesDiff` precedent (`rank_excluding`/`unrank_excluding`/`transport_forward`), generalized
//! over the item/diff type pair instead of gif's 3 hand-duplicated instantiations. Collection-triple
//! wire codecs (`enc_indexed_triple`/`dec_indexed_triple`) come from the shared
//! `engine::triples` module per the ticket's explicit instruction — not re-derived here.

use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{dec_indexed_triple, enc_indexed_triple, split_top_level, strip_brackets, IndexAdded, IndexModified, IndexedTripleDiff};
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::MutationDiff;
use schema::ArtifactSchema;

//#region 🔖️IndexedCollectionAlgebra
/// 📐️ Shared rank/unrank arithmetic for index-keyed collection diffs — see `🧬️schema-design.md`
/// §Absorb for the derivation. `excluded_sorted` must be sorted ascending.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_le(sorted: &[usize], x: usize) -> usize {
    sorted.partition_point(|&v| v <= x)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rank_excluding(pos: usize, excluded_sorted: &[usize]) -> usize {
    pos - count_le(excluded_sorted, pos)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn unrank_excluding(rank: usize, excluded_sorted: &[usize]) -> usize {
    let mut candidate = rank;
    loop {
        let next = rank + count_le(excluded_sorted, candidate);
        if next == candidate {
            return candidate;
        }
        candidate = next;
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transport_forward(index: usize, removed_sorted: &[usize], added_index_sorted: &[usize]) -> usize {
    unrank_excluding(rank_excluding(index, removed_sorted), added_index_sorted)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn indexed_is_empty<D, T>(d: &IndexedTripleDiff<D, T>) -> bool {
    d.removed.is_empty() && d.modified.is_empty() && d.added.is_empty()
}

/// 🧭️ Position-pairwise state delta: `0..min(len)` compare as `modified`, base's tail is `removed`,
/// other's tail is `added` — per `🧬️schema-design.md`'s `between` matching rule for index keys.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_indexed<T: Clone, D>(base: &[T], other: &[T], between_item: impl Fn(&T, &T) -> D, item_is_empty: impl Fn(&D) -> bool) -> IndexedTripleDiff<D, T> {
    let min = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min {
        let d = between_item(&base[i], &other[i]);
        if !item_is_empty(&d) {
            modified.push(IndexModified { index: i, diff: d });
        }
    }
    let removed: Vec<usize> = (min..base.len()).collect();
    let added: Vec<IndexAdded<T>> = (min..other.len()).map(|i| IndexAdded { index: i, item: other[i].clone() }).collect();
    IndexedTripleDiff { removed, modified, added }
}

/// ▶️ Apply semantics (normative, `🧬️schema-design.md`): modify against BASE indices, remove
/// descending, then insert `added` ascending at `min(index, len)` against the FINAL positions.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_indexed<T: Clone, D>(diff: &IndexedTripleDiff<D, T>, base: &[T], apply_item: impl Fn(&D, &T) -> T) -> Vec<T> {
    let mut next: Vec<Option<T>> = base.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(slot) = next.get_mut(m.index) {
            if let Some(item) = slot {
                *item = apply_item(&m.diff, item);
            }
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable();
    removed_sorted.reverse();
    for &r in &removed_sorted {
        if r < next.len() {
            next.remove(r);
        }
    }
    let mut out: Vec<T> = next.into_iter().flatten().collect();
    let mut added_sorted = diff.added.clone();
    added_sorted.sort_by_key(|a| a.index);
    for a in added_sorted {
        let at = a.index.min(out.len());
        out.insert(at, a.item);
    }
    out
}

/// 🧮️ Sequential-coalesce absorb (base->mid composed with mid->after). Canonical correctness cases
/// (Insert+Remove-before, Insert+Insert-same-index, Insert+SetField-into-added) are proven in this
/// module's tests against the innermost `keyframes` level.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_indexed<T: Clone, D: Clone>(mine: &mut IndexedTripleDiff<D, T>, other: IndexedTripleDiff<D, T>, mut absorb_diff: impl FnMut(&mut D, D), apply_diff_to_item: impl Fn(&D, &T) -> T) {
    let removed1_sorted = {
        let mut v = mine.removed.clone();
        v.sort_unstable();
        v
    };
    let added1_index_sorted = {
        let mut v: Vec<usize> = mine.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };
    let removed2_sorted = {
        let mut v = other.removed.clone();
        v.sort_unstable();
        v
    };
    let added2_index_sorted = {
        let mut v: Vec<usize> = other.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };

    let mut merged_added: Vec<IndexAdded<T>> = std::mem::take(&mut mine.added);
    let mut annihilated: std::collections::HashSet<usize> = Default::default();

    //#region Removed
    let mut merged_removed_base: Vec<usize> = removed1_sorted.clone();
    for &r2 in &removed2_sorted {
        if added1_index_sorted.binary_search(&r2).is_ok() {
            annihilated.insert(r2);
            merged_added.retain(|a| a.index != r2);
        } else {
            let post_remove_rank = rank_excluding(r2, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            merged_removed_base.push(base_index);
        }
    }
    merged_removed_base.sort_unstable();
    merged_removed_base.dedup();
    //#endregion Removed

    //#region Modified
    let mut modified_map: std::collections::BTreeMap<usize, D> = std::mem::take(&mut mine.modified).into_iter().map(|m| (m.index, m.diff)).collect();
    for base_index in &merged_removed_base {
        modified_map.remove(base_index);
    }
    for m2 in other.modified {
        let mp = m2.index;
        if annihilated.contains(&mp) {
            continue;
        }
        if added1_index_sorted.binary_search(&mp).is_ok() {
            if let Some(entry) = merged_added.iter_mut().find(|a| a.index == mp) {
                entry.item = apply_diff_to_item(&m2.diff, &entry.item);
            }
        } else {
            let post_remove_rank = rank_excluding(mp, &added1_index_sorted);
            let base_index = unrank_excluding(post_remove_rank, &removed1_sorted);
            if merged_removed_base.binary_search(&base_index).is_ok() {
                continue;
            }
            modified_map.entry(base_index).and_modify(|d| absorb_diff(d, m2.diff.clone())).or_insert(m2.diff);
        }
    }
    //#endregion Modified

    //#region Added
    let mut merged_added_final: Vec<IndexAdded<T>> = merged_added
        .into_iter()
        .map(|a| {
            let after_pos = if removed2_sorted.binary_search(&a.index).is_ok() {
                a.index
            } else {
                let post_remove_rank = rank_excluding(a.index, &removed2_sorted);
                unrank_excluding(post_remove_rank, &added2_index_sorted)
            };
            IndexAdded { index: after_pos, item: a.item }
        })
        .collect();
    merged_added_final.extend(other.added);
    merged_added_final.sort_by_key(|a| a.index);
    //#endregion Added

    mine.removed = merged_removed_base;
    mine.modified = modified_map.into_iter().map(|(index, diff)| IndexModified { index, diff }).collect();
    mine.added = merged_added_final;
}

/// ↩️ Diff-level inverse for an index-keyed triple, given the ORIGINAL base items.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_indexed<T: Clone, D>(diff: &IndexedTripleDiff<D, T>, base_items: &[T], diff_inverse: impl Fn(&D, &T) -> D) -> IndexedTripleDiff<D, T> {
    let removed_sorted = {
        let mut v = diff.removed.clone();
        v.sort_unstable();
        v
    };
    let added_index_sorted = {
        let mut v: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
        v.sort_unstable();
        v
    };

    let mut inv_removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut inv_modified: Vec<IndexModified<D>> = Vec::new();
    for m in &diff.modified {
        if let Some(orig) = base_items.get(m.index) {
            let after_index = transport_forward(m.index, &removed_sorted, &added_index_sorted);
            inv_modified.push(IndexModified { index: after_index, diff: diff_inverse(&m.diff, orig) });
        }
    }
    let mut inv_added: Vec<IndexAdded<T>> = Vec::new();
    for &r in &diff.removed {
        if let Some(orig) = base_items.get(r) {
            inv_added.push(IndexAdded { index: r, item: orig.clone() });
        }
    }
    inv_removed.sort_unstable();
    inv_added.sort_by_key(|a| a.index);
    IndexedTripleDiff { removed: inv_removed, modified: inv_modified, added: inv_added }
}
//#endregion 🔖️IndexedCollectionAlgebra

//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|i| enc(i)).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
/// 🧭️ Full-item ("added"-slot) encoders/decoders for every snapshot-owned value type — reused both
/// by the collection-triple `added` entries and by the sparse per-field diff codecs below.
/// 🎯️ `t`/`r`/`s`/`w` for the unit variants, `c:<hex>` for `Custom{name}` (W2c closer fix: the
/// trailing `:` separator, not `c<hex>` glued directly, is REQUIRED — the shared lexer's
/// `is_ident_continue` includes alphanumerics, so a bare `c` immediately followed by hex digits
/// lexes as ONE fused identifier token, not two; the grammar's `"c" ":" hex` production could
/// never match a glued token otherwise. Matches `📸️snapshot/🦀️.rs`'s own duplicated
/// `enc_property`/`dec_property` field-for-field.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_property(p: &AnimTargetProperty) -> String {
    match p {
        AnimTargetProperty::Translation => "t".to_string(),
        AnimTargetProperty::Rotation => "r".to_string(),
        AnimTargetProperty::Scale => "s".to_string(),
        AnimTargetProperty::Weights => "w".to_string(),
        AnimTargetProperty::Custom { name } => format!("c:{}", enc_str(name)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_property(s: &str) -> Result<AnimTargetProperty, String> {
    match s {
        "t" => Ok(AnimTargetProperty::Translation),
        "r" => Ok(AnimTargetProperty::Rotation),
        "s" => Ok(AnimTargetProperty::Scale),
        "w" => Ok(AnimTargetProperty::Weights),
        other => {
            let rest = other.strip_prefix("c:").ok_or_else(|| format!("bad property {other:?}"))?;
            Ok(AnimTargetProperty::Custom { name: dec_str(rest)? })
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_target(t: &AnimTarget) -> String {
    format!("[{},{}]", enc_str(&t.node), enc_property(&t.property))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_target(s: &str) -> Result<AnimTarget, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [node, prop] = parts.as_slice() else { return Err(format!("target: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTarget { node: dec_str(node)?, property: dec_property(prop)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_interpolation(i: AnimInterpolation) -> char {
    match i {
        AnimInterpolation::Linear => 'l',
        AnimInterpolation::Step => 's',
        AnimInterpolation::CubicSpline => 'c',
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_interpolation(s: &str) -> Result<AnimInterpolation, String> {
    match s {
        "l" => Ok(AnimInterpolation::Linear),
        "s" => Ok(AnimInterpolation::Step),
        "c" => Ok(AnimInterpolation::CubicSpline),
        other => Err(format!("bad interpolation {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_value(v: &AnimValue) -> String {
    match v {
        AnimValue::Scalar { value } => format!("S:{value}"),
        AnimValue::Vec3 { value } => format!("V:[{},{},{}]", value.x, value.y, value.z),
        AnimValue::Quat { value } => format!("Q:[{},{},{},{}]", value.x, value.y, value.z, value.w),
        AnimValue::Weights { values } => format!("W:{}", enc_list(values, |v| v.to_string())),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_value(s: &str) -> Result<AnimValue, String> {
    let (tag, rest) = s.split_once(':').ok_or_else(|| format!("value: bad shape {s:?}"))?;
    match tag {
        "S" => Ok(AnimValue::Scalar { value: parse_f64(rest)? }),
        "V" => {
            let parts = split_top_level(strip_brackets(rest)?, ',');
            let [x, y, z] = parts.as_slice() else { return Err(format!("vec3: expected 3 fields, got {}", parts.len())) };
            Ok(AnimValue::Vec3 { value: SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? } })
        }
        "Q" => {
            let parts = split_top_level(strip_brackets(rest)?, ',');
            let [x, y, z, w] = parts.as_slice() else { return Err(format!("quat: expected 4 fields, got {}", parts.len())) };
            Ok(AnimValue::Quat { value: SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? } })
        }
        "W" => Ok(AnimValue::Weights { values: dec_list(rest, parse_f64)? }),
        other => Err(format!("value: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_keyframe(k: &AnimKeyframe) -> String {
    format!("[{},{}]", k.t, enc_value(&k.value))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_keyframe(s: &str) -> Result<AnimKeyframe, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [t, value] = parts.as_slice() else { return Err(format!("keyframe: expected 2 fields, got {}", parts.len())) };
    Ok(AnimKeyframe { t: parse_f64(t)?, value: dec_value(value)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_channel(c: &AnimChannel) -> String {
    format!("[{},{},{}]", enc_target(&c.target), enc_interpolation(c.interpolation), enc_list(&c.keyframes, enc_keyframe))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_channel(s: &str) -> Result<AnimChannel, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [target, interp, kfs] = parts.as_slice() else { return Err(format!("channel: expected 3 fields, got {}", parts.len())) };
    Ok(AnimChannel { target: dec_target(target)?, interpolation: dec_interpolation(interp)?, keyframes: dec_list(kfs, dec_keyframe)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_timeline(t: &AnimTimeline) -> String {
    format!("[{},{}]", encode_option(&t.name, |n| enc_str(n)), enc_list(&t.channels, enc_channel))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_timeline(s: &str) -> Result<AnimTimeline, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [name, channels] = parts.as_slice() else { return Err(format!("timeline: expected 2 fields, got {}", parts.len())) };
    Ok(AnimTimeline { name: decode_option(name, dec_str)?, channels: dec_list(channels, dec_channel)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️KeyframeDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AnimKeyframeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<AnimValue>,
}

impl AnimKeyframeDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.t.is_none() && self.value.is_none()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &AnimKeyframe, other: &AnimKeyframe) -> Self {
        Self { t: (base.t != other.t).then_some(other.t), value: (base.value != other.value).then_some(other.value.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &AnimKeyframe) -> AnimKeyframe {
        let mut next = base.clone();
        if let Some(v) = self.t {
            next.t = v;
        }
        if let Some(v) = &self.value {
            next.value = v.clone();
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base: &AnimKeyframe) -> Self {
        Self { t: self.t.map(|_| base.t), value: self.value.as_ref().map(|_| base.value.clone()) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.t.is_some() {
            self.t = other.t;
        }
        if other.value.is_some() {
            self.value = other.value;
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_keyframe_diff(d: &AnimKeyframeDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = d.t {
        parts.push(format!("T:{v}"));
    }
    if let Some(v) = &d.value {
        parts.push(format!("Y:{}", enc_value(v)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_keyframe_diff(s: &str) -> Result<AnimKeyframeDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimKeyframeDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("keyframe diff: bad entry {entry:?}"))?;
        match tag {
            "T" => d.t = Some(parse_f64(val)?),
            "Y" => d.value = Some(dec_value(val)?),
            other => return Err(format!("keyframe diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
//#endregion 🔖️KeyframeDiff

//#region 🔖️ChannelDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AnimChannelDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<AnimTarget>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub interpolation: Option<AnimInterpolation>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub keyframes: Option<IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe>>,
}

impl AnimChannelDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.target.is_none() && self.interpolation.is_none() && self.keyframes.as_ref().map(indexed_is_empty).unwrap_or(true)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &AnimChannel, other: &AnimChannel) -> Self {
        let kf = between_indexed(&base.keyframes, &other.keyframes, AnimKeyframeDiff::between, AnimKeyframeDiff::is_empty);
        Self { target: (base.target != other.target).then_some(other.target.clone()), interpolation: (base.interpolation != other.interpolation).then_some(other.interpolation), keyframes: (!indexed_is_empty(&kf)).then_some(kf) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &AnimChannel) -> AnimChannel {
        let mut next = base.clone();
        if let Some(v) = &self.target {
            next.target = v.clone();
        }
        if let Some(v) = self.interpolation {
            next.interpolation = v;
        }
        if let Some(d) = &self.keyframes {
            next.keyframes = apply_indexed(d, &next.keyframes, |d, item| d.apply(item));
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base: &AnimChannel) -> Self {
        Self { target: self.target.as_ref().map(|_| base.target.clone()), interpolation: self.interpolation.map(|_| base.interpolation), keyframes: self.keyframes.as_ref().map(|d| inverse_indexed(d, &base.keyframes, |d, item| d.inverse(item))) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.target.is_some() {
            self.target = other.target;
        }
        if other.interpolation.is_some() {
            self.interpolation = other.interpolation;
        }
        match (&mut self.keyframes, other.keyframes) {
            (Some(mine), Some(theirs)) => absorb_indexed(mine, theirs, |d, o| d.absorb(o), |d, item| d.apply(item)),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_channel_diff(d: &AnimChannelDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.target {
        parts.push(format!("G:{}", enc_target(v)));
    }
    if let Some(v) = d.interpolation {
        parts.push(format!("I:{}", enc_interpolation(v)));
    }
    if let Some(v) = &d.keyframes {
        parts.push(format!("K:[{}]", enc_indexed_triple(v, enc_keyframe_diff, enc_keyframe)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_channel_diff(s: &str) -> Result<AnimChannelDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimChannelDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("channel diff: bad entry {entry:?}"))?;
        match tag {
            "G" => d.target = Some(dec_target(val)?),
            "I" => d.interpolation = Some(dec_interpolation(val)?),
            "K" => d.keyframes = Some(dec_indexed_triple(strip_brackets(val)?, dec_keyframe_diff, dec_keyframe)?),
            other => return Err(format!("channel diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
//#endregion 🔖️ChannelDiff

//#region 🔖️TimelineDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct AnimTimelineDiff {
    /// 🏷️ Tri-state: `None` = unchanged, `Some(None)` = name cleared, `Some(Some(v))` = renamed.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<IndexedTripleDiff<AnimChannelDiff, AnimChannel>>,
}

impl AnimTimelineDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_empty(&self) -> bool {
        self.name.is_none() && self.channels.as_ref().map(indexed_is_empty).unwrap_or(true)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn between(base: &AnimTimeline, other: &AnimTimeline) -> Self {
        let ch = between_indexed(&base.channels, &other.channels, AnimChannelDiff::between, AnimChannelDiff::is_empty);
        Self { name: (base.name != other.name).then_some(other.name.clone()), channels: (!indexed_is_empty(&ch)).then_some(ch) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn apply(&self, base: &AnimTimeline) -> AnimTimeline {
        let mut next = base.clone();
        if let Some(v) = &self.name {
            next.name = v.clone();
        }
        if let Some(d) = &self.channels {
            next.channels = apply_indexed(d, &next.channels, |d, item| d.apply(item));
        }
        next
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn inverse(&self, base: &AnimTimeline) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), channels: self.channels.as_ref().map(|d| inverse_indexed(d, &base.channels, |d, item| d.inverse(item))) }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn absorb(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        match (&mut self.channels, other.channels) {
            (Some(mine), Some(theirs)) => absorb_indexed(mine, theirs, |d, o| d.absorb(o), |d, item| d.apply(item)),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_timeline_diff(d: &AnimTimelineDiff) -> String {
    let mut parts = Vec::new();
    if let Some(v) = &d.name {
        parts.push(format!("N:{}", encode_option(v, |n| enc_str(n))));
    }
    if let Some(v) = &d.channels {
        parts.push(format!("C:[{}]", enc_indexed_triple(v, enc_channel_diff, enc_channel)));
    }
    format!("[{}]", parts.join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_timeline_diff(s: &str) -> Result<AnimTimelineDiff, String> {
    let inner = strip_brackets(s)?;
    let mut d = AnimTimelineDiff::default();
    for entry in split_top_level(inner, ',') {
        if entry.is_empty() {
            continue;
        }
        let (tag, val) = entry.split_once(':').ok_or_else(|| format!("timeline diff: bad entry {entry:?}"))?;
        match tag {
            "N" => d.name = Some(decode_option(val, dec_str)?),
            "C" => d.channels = Some(dec_indexed_triple(strip_brackets(val)?, dec_channel_diff, dec_channel)?),
            other => return Err(format!("timeline diff: unknown tag {other:?}")),
        }
    }
    Ok(d)
}
//#endregion 🔖️TimelineDiff

//#region 🔖️Diff
/// 🔺️ Diff for `s.stdio.semio.animation`. No `replacement: Option<SemioAnimationSnapshot>`
/// full-replace slot anywhere — a single sparse `timelines` collection-triple field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation.diff")]
pub struct SemioAnimationDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub timelines: Option<IndexedTripleDiff<AnimTimelineDiff, AnimTimeline>>,
}

impl SemioAnimationDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.timelines.as_ref().map(indexed_is_empty).unwrap_or(true)
    }
}

impl MutationDiff<SemioAnimationSnapshot> for SemioAnimationDiff {
    fn apply(&self, base: &SemioAnimationSnapshot) -> protocol::MutationApplyResult<SemioAnimationSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.timelines {
            crate::artifacts::semio::standards::v1::subsets::base::schema::triples::validate_indexed_triple(d, next.timelines.len(), ["timelines"])?;
            next.timelines = apply_indexed(d, &next.timelines, |d, item| d.apply(item));
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        match (&mut self.timelines, other.timelines) {
            (Some(mine), Some(theirs)) => absorb_indexed(mine, theirs, |d, o| d.absorb(o), |d, item| d.apply(item)),
            (slot @ None, Some(theirs)) => *slot = Some(theirs),
            _ => {}
        }
    }
}

impl DiffAlgebra<SemioAnimationSnapshot> for SemioAnimationDiff {
    fn inverse(&self, base: &SemioAnimationSnapshot) -> Self {
        Self { timelines: self.timelines.as_ref().map(|d| inverse_indexed(d, &base.timelines, |d, item| d.inverse(item))) }
    }

    fn between(base: &SemioAnimationSnapshot, other: &SemioAnimationSnapshot) -> Self {
        let d = between_indexed(&base.timelines, &other.timelines, AnimTimelineDiff::between, AnimTimelineDiff::is_empty);
        Self { timelines: (!indexed_is_empty(&d)).then_some(d) }
    }

    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}

/// 🧩 Builds a set-snapshot diff — sparse field-by-field, never a full-replace slot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioAnimationSnapshot, snapshot: &SemioAnimationSnapshot) -> SemioAnimationDiff {
    <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Handcrafted `DiffCodec` grammar: one `timelines{[removed];[modified];[added]}` section
/// (empty diff prints as `""`). Sparse per-field entries inside `modified` use single-letter tags
/// (`N`/`C` for timelines, `G`/`I`/`K` for channels, `T`/`Y` for keyframes); `Option<T>` uses the
/// `[0]`=None / `[1,<T>]`=Some(T) tag shared with the full-item value codecs above. Bytes/strings
/// are lowercase hex — no escaping needed, matching this artifact's own `ArtifactDsl` and the
/// gif 89a precedent. Binary = the text bytes verbatim (same simplification `WriterDiff`'s
/// hand-rolled `DiffCodec` and gif 89a's own `GifDiff` use).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_animation_diff(d: &SemioAnimationDiff) -> String {
    match &d.timelines {
        Some(v) => format!("timelines{{{}}}", enc_indexed_triple(v, enc_timeline_diff, enc_timeline)),
        None => String::new(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_animation_diff(line: &str) -> Result<SemioAnimationDiff, String> {
    if line.is_empty() {
        return Ok(SemioAnimationDiff::default());
    }
    let body = line.strip_prefix("timelines{").and_then(|s| s.strip_suffix('}')).ok_or_else(|| format!("diff: bad shape {line:?}"))?;
    let d = dec_indexed_triple(body, dec_timeline_diff, dec_timeline)?;
    Ok(SemioAnimationDiff { timelines: (!indexed_is_empty(&d)).then_some(d) })
}

/// 🔢️ Real binary diff frame (animation wave — off the old `print_diff().into_bytes()` F6
/// text-as-binary shortcut). `format u8` + `presence u8` (bit0=`timelines`) as two real fixed
/// header fields; when present, the SAME `enc_indexed_triple`-produced text this facet's own
/// `print_diff` already emits follows as one opaque trailing byte chain (last field in the frame,
/// so no length prefix is needed — matches the recipe's §2.5 "opaque payload LAST" rule). Only one
/// collection exists here (unlike brep's 6/flow's 2), so `presence` only ever uses bit0.
const DIFF_BINARY_FORMAT: u8 = 1;

impl DiffCodec for SemioAnimationDiff {
    fn print_diff(&self) -> String {
        print_semio_animation_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_semio_animation_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![DIFF_BINARY_FORMAT];
        match &self.timelines {
            Some(v) => {
                out.push(1u8);
                out.extend_from_slice(enc_indexed_triple(v, enc_timeline_diff, enc_timeline).as_bytes());
            }
            None => out.push(0u8),
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let malformed = |what: &'static str, detail: String| protocol::ProtocolError::Malformed { what, offset: 0, detail };
        let [format, presence, rest @ ..] = bytes else { return Err(malformed("diff header", format!("expected at least 2 bytes, got {}", bytes.len()))) };
        if *format != DIFF_BINARY_FORMAT {
            return Err(malformed("diff format", format!("unsupported diff format {format}")));
        }
        let timelines = match presence {
            0 => None,
            1 => {
                let text = std::str::from_utf8(rest).map_err(|e| malformed("diff payload utf8", e.to_string()))?;
                Some(dec_indexed_triple(text, dec_timeline_diff, dec_timeline).map_err(|e| malformed("diff payload", e))?)
            }
            other => return Err(malformed("diff presence", format!("unknown presence byte {other}"))),
        };
        Ok(SemioAnimationDiff { timelines })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

/// 🧱️ Small value-builder helpers, module-scope (not `mod tests`-local) so the `🎹️composer`
/// conformance-law tests can reuse them via `demo_diff_cases()` below — same promotion pattern
/// every prior semio wave's report documents (a private item of a child `mod tests` isn't visible
/// to a sibling module).
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn kf(t: f64, value: AnimValue) -> AnimKeyframe {
    AnimKeyframe { t, value }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn channel(node: &str, property: AnimTargetProperty, interpolation: AnimInterpolation, keyframes: Vec<AnimKeyframe>) -> AnimChannel {
    AnimChannel { target: AnimTarget { node: node.into(), property }, interpolation, keyframes }
}
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn timeline(name: Option<&str>, channels: Vec<AnimChannel>) -> AnimTimeline {
    AnimTimeline { name: name.map(String::from), channels }
}

/// 🌱 Representative `SemioAnimationDiff` cases (empty + both directions of a rich `between`) —
/// single source of truth for the composer's `diff_grammar_conformance_law`/`protocol_walk_law`,
/// reused by this module's own `diff_codec_text_binary_roundtrip_law` test below.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioAnimationDiff> {
    let a = SemioAnimationSnapshot {
        timelines: vec![timeline(Some("gone"), vec![]), timeline(Some("kept"), vec![channel("n", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Scalar { value: 1.0 })])])],
        ..SemioAnimationSnapshot::default()
    };
    let b = SemioAnimationSnapshot {
        timelines: vec![
            timeline(None, vec![channel("n", AnimTargetProperty::Rotation, AnimInterpolation::CubicSpline, vec![kf(0.0, AnimValue::Quat { value: SemioQuaternion::default() }), kf(1.0, AnimValue::Weights { values: vec![0.1, 0.9] })])]),
            timeline(Some("added"), vec![channel("m", AnimTargetProperty::Custom { name: "x".into() }, AnimInterpolation::Step, vec![])]),
        ],
        ..SemioAnimationSnapshot::default()
    };
    vec![SemioAnimationDiff::default(), <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&a, &b), <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&b, &a)]
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region AbsorbCanonical
    /// 🧪️ Canonical absorb case 1 (at the innermost `keyframes` level): `Insert(2,f)` then
    /// `Remove(0)` -> `{removed:[0], added:[(1,f)]}`.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_remove_before_shifts_index() {
        let f = kf(9.0, AnimValue::Scalar { value: 9.0 });
        let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { removed: vec![0], ..Default::default() };
        absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
        assert_eq!(d1.removed, vec![0]);
        assert_eq!(d1.added, vec![IndexAdded { index: 1, item: f }]);
        assert!(d1.modified.is_empty());
    }

    /// 🧪️ Canonical absorb case 2: `Insert(2,f)` then `Insert(2,g)` -> BOTH survive.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_insert_same_index_both_survive() {
        let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
        let g = kf(2.0, AnimValue::Scalar { value: 2.0 });
        let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: f.clone() }], ..Default::default() };
        let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 2, item: g.clone() }], ..Default::default() };
        absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
        assert_eq!(d1.added, vec![IndexAdded { index: 2, item: g }, IndexAdded { index: 3, item: f }]);
    }

    /// 🧪️ Canonical absorb case 3: `Insert(1,f)` then `SetField(1,v)` patches INTO the added
    /// payload — no separate `modified` entry survives.
    #[semio_framework_async_macros::async_test]
    async fn absorb_insert_then_set_field_patches_into_added() {
        let f = kf(1.0, AnimValue::Scalar { value: 1.0 });
        let mut d1: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { added: vec![IndexAdded { index: 1, item: f.clone() }], ..Default::default() };
        let d2: IndexedTripleDiff<AnimKeyframeDiff, AnimKeyframe> = IndexedTripleDiff { modified: vec![IndexModified { index: 1, diff: AnimKeyframeDiff { t: Some(42.0), value: None } }], ..Default::default() };
        absorb_indexed(&mut d1, d2, |d, o| d.absorb(o), |d, item| d.apply(item));
        assert!(d1.modified.is_empty());
        assert_eq!(d1.added.len(), 1);
        assert_eq!(d1.added[0].item.t, 42.0);
        assert_eq!(d1.added[0].index, 1);
    }
    //#endregion AbsorbCanonical

    /// 🧪️ absorb_law: full 3-level snapshot chain, base -> mid -> after.
    #[semio_framework_async_macros::async_test]
    async fn absorb_law_holds_over_curated_ops() {
        let base = SemioAnimationSnapshot {
            timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } })])]), timeline(Some("blink"), vec![])],
            ..SemioAnimationSnapshot::default()
        };
        let mid = {
            let mut s = base.clone();
            s.timelines[0].channels[0].keyframes.push(kf(1.0, AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }));
            s.timelines.remove(1);
            s.timelines.push(timeline(Some("wave"), vec![]));
            s
        };
        let after = {
            let mut s = mid.clone();
            s.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
            s.timelines[1].channels.push(channel("hand", AnimTargetProperty::Rotation, AnimInterpolation::Step, vec![kf(0.0, AnimValue::Quat { value: SemioQuaternion::default() })]));
            s
        };
        let mut d1 = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&base, &mid);
        let d2 = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
    }

    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a =
            SemioAnimationSnapshot { timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Scalar { value: 1.0 })])])], ..SemioAnimationSnapshot::default() };
        let b = SemioAnimationSnapshot {
            timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Step, vec![kf(0.0, AnimValue::Scalar { value: 1.0 }), kf(1.0, AnimValue::Scalar { value: 2.0 })])]), timeline(None, vec![])],
            ..SemioAnimationSnapshot::default()
        };
        let ab = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&a, &b);
        assert_eq!(ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        let ba = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&b, &a);
        assert_eq!(ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&a, &a).is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        let base =
            SemioAnimationSnapshot { timelines: vec![timeline(Some("walk"), vec![channel("hip", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(0.0, AnimValue::Scalar { value: 1.0 })])])], ..SemioAnimationSnapshot::default() };
        let next = {
            let mut s = base.clone();
            s.timelines[0].name = None;
            s.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
            s.timelines[0].channels[0].keyframes[0].value = AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } };
            s.timelines.push(timeline(Some("wave"), vec![]));
            s
        };
        let d = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&base, &next);
        let mutated = d.apply(&base).expect("apply must succeed for a well-formed fixture");
        let inv = d.inverse(&base);
        assert_eq!(inv.apply(&mutated).expect("apply must succeed for a well-formed fixture"), base);
    }

    /// 🧪️ field_sweep — the acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable
    /// field at every nesting depth (timeline/channel/keyframe).await, including the `name` tri-state
    /// and an `AnimValue` variant change, with a removed+modified collection member exercised at
    /// SOME direction/level and an added+modified member exercised at the opposite
    /// direction/level — `timelines`/`channels`/`keyframes` are ALL `IndexedTripleDiff`
    /// (position-keyed, per `between_indexed`'s own truncating-tail semantics: an index only ever
    /// falls in `removed` when `base` is strictly longer than `other` at that tail, or in `added`
    /// when `other` is strictly longer — never both from the SAME `between()` call, at ANY
    /// nesting level, since one collection can't simultaneously be longer AND shorter than the
    /// other; W2b closer fix: the fixture previously kept every level's `sweep_a`/`sweep_b` pair
    /// at EQUAL length, so neither `removed` nor `added` could ever be non-empty anywhere — only
    /// `modified` — even though the doc comment already correctly named this exact structural
    /// trap. Every level below now carries a genuine length asymmetry, deliberately alternating
    /// which direction shows `removed` vs `added` per level (timelines: removed-forward/
    /// added-reverse; channels: removed-forward/added-reverse; keyframes: added-forward, the
    /// mirror case) — same split `presentation`'s own field_sweep test already uses for its
    /// index-keyed `slides`, applied consistently at all 3 nesting depths here.
    #[semio_framework_async_macros::async_test]
    async fn field_sweep_covers_every_mutable_field() {
        let sweep_a = SemioAnimationSnapshot {
            timelines: vec![
                timeline(
                    Some("kept"),
                    vec![channel("kept-node", AnimTargetProperty::Translation, AnimInterpolation::Linear, vec![kf(9.0, AnimValue::Scalar { value: 9.0 })]), channel("gone-node", AnimTargetProperty::Weights, AnimInterpolation::Linear, vec![])],
                ),
                timeline(Some("filler"), vec![]),
                timeline(Some("gone"), vec![]),
            ],
            ..SemioAnimationSnapshot::default()
        };
        let sweep_b = SemioAnimationSnapshot {
            timelines: vec![
                timeline(None, vec![channel("kept-node", AnimTargetProperty::Rotation, AnimInterpolation::CubicSpline, vec![kf(1.0, AnimValue::Quat { value: SemioQuaternion::default() }), kf(2.0, AnimValue::Weights { values: vec![0.5, 0.5] })])]),
                timeline(Some("filler2"), vec![]),
            ],
            ..SemioAnimationSnapshot::default()
        };

        let ab = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_a, &sweep_b);
        assert_eq!(ab.apply(&sweep_a).expect("apply must succeed for a well-formed fixture"), sweep_b);
        let timelines_ab = ab.timelines.as_ref().expect("timelines must differ");
        assert!(!timelines_ab.removed.is_empty(), "sweep must exercise a removed timeline");
        assert!(!timelines_ab.modified.is_empty(), "sweep must exercise a modified timeline");
        let timeline_diff = &timelines_ab.modified[0].diff;
        assert_eq!(timeline_diff.name, Some(None), "name Some->None must be tri-state Some(None)");
        let channels_ab = timeline_diff.channels.as_ref().expect("channels must differ");
        assert!(!channels_ab.removed.is_empty(), "sweep must exercise a removed channel");
        assert!(!channels_ab.modified.is_empty(), "sweep must exercise a modified channel");
        let channel_diff = &channels_ab.modified[0].diff;
        assert!(channel_diff.target.is_some());
        assert!(channel_diff.interpolation.is_some());
        let keyframes_ab = channel_diff.keyframes.as_ref().expect("keyframes must differ");
        assert!(!keyframes_ab.modified.is_empty(), "sweep must exercise a modified keyframe");
        assert!(!keyframes_ab.added.is_empty(), "sweep must exercise an added keyframe");
        let keyframe_diff = &keyframes_ab.modified[0].diff;
        assert!(keyframe_diff.t.is_some());
        assert!(keyframe_diff.value.is_some(), "AnimValue variant change must be captured");

        let ba = <SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_b, &sweep_a);
        assert_eq!(ba.apply(&sweep_b).expect("apply must succeed for a well-formed fixture"), sweep_a);
        let timelines_ba = ba.timelines.as_ref().expect("timelines must differ");
        assert!(!timelines_ba.added.is_empty(), "reverse direction must exercise an added timeline");
        assert!(!timelines_ba.modified.is_empty(), "reverse direction must exercise a modified timeline");
        let timeline_diff_ba = &timelines_ba.modified[0].diff;
        let channels_ba = timeline_diff_ba.channels.as_ref().expect("channels must differ in reverse too");
        assert!(!channels_ba.added.is_empty(), "reverse direction must exercise an added channel");
        assert!(!channels_ba.modified.is_empty(), "reverse direction must exercise a modified channel");

        assert!(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(&sweep_a, &sweep_a).is_empty());
    }

    /// 🧪️ diff_codec_text_binary_roundtrip_law: hand-rolled `DiffCodec` text/binary grammar —
    /// exercises the empty diff, the tri-state `name`, an `AnimValue` variant change, and all
    /// three collection triples (removed/modified/added) at every nesting depth.
    #[semio_framework_async_macros::async_test]
    async fn diff_codec_text_binary_roundtrip_law() {
        for d in demo_diff_cases() {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioAnimationDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioAnimationDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
}
//#endregion 🔖️Tests
