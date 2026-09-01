//! 🧬️ SemioImageMutation — named-variant enum covering every mutable field of
//! `SemioImageSnapshot`. Every `diff()`/`inverse()` arm is HAND-WRITTEN (never apply-and-capture
//! — `🧬️schema-design.md`'s svg infinite-recursion warning): each variant builds its own sparse
//! `SemioImageDiff` directly and computes its own base-aware inverse mutation.
//!
//! `NoMutation` was dropped: `#[derive(dsl::Mutations)]` requires every variant to wrap exactly one
//! leaf payload, and its sentinel verb `no` is not in `APPROVED_VERBS` — see
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`. Every variant is now a
//! tuple variant wrapping its own mutation leaf (`./*/🦀️.rs`), and this file's `agg_diff`/
//! `agg_inverse` carry the handcrafted semantics every leaf's `MutationKind` impl delegates back to.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets, IndexAdded, IndexModified, NamedModified};
use crate::artifacts::semio::standards::v1::subsets::image::schema::diff::{
    dec_colorspace, dec_frame, dec_metadata_entry, decode_option, diff_set_snapshot, enc_colorspace, enc_frame, enc_metadata_entry, encode_option, SemioImageDiff, SemioImageFrameDiff, SemioImageFramesDiff, SemioImageMetadataDiff,
};
use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioImageMutation` below calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope in
/// production code (was missing entirely, even test-gated) (W2b closer fix).
use protocol::OpText;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
//#region 🔖️Leaves
#[path = "📸️set-snapshot/🦀️.rs"]
pub mod set_snapshot;
#[path = "📐️set-dimensions/🦀️.rs"]
pub mod set_dimensions;
#[path = "🌈️set-colorspace/🦀️.rs"]
pub mod set_colorspace;
#[path = "🔢️set-bit-depth/🦀️.rs"]
pub mod set_bit_depth;
#[path = "🎨️set-icc/🦀️.rs"]
pub mod set_icc;
#[path = "➕️insert-frame/🦀️.rs"]
pub mod insert_frame;
#[path = "📄remove-frame/🦀️.rs"]
pub mod remove_frame;
#[path = "🔀️move-frame/🦀️.rs"]
pub mod move_frame;
#[path = "⏱️set-frame-delay/🦀️.rs"]
pub mod set_frame_delay;
#[path = "🟪️set-frame-pixels/🦀️.rs"]
pub mod set_frame_pixels;
#[path = "🏷️set-metadata-entry/🦀️.rs"]
pub mod set_metadata_entry;
#[path = "🗑️remove-metadata-entry/🦀️.rs"]
pub mod remove_metadata_entry;
//#endregion 🔖️Leaves

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations)]
#[mutations(snapshot = SemioImageSnapshot, diff = SemioImageDiff, schema = "SemioImageMutation")]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SemioImageMutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    SetDimensions(set_dimensions::SetDimensions),
    SetColorspace(set_colorspace::SetColorspace),
    SetBitDepth(set_bit_depth::SetBitDepth),
    /// 🎨️ `icc: None` clears the profile — the mutation payload is the FINAL value (unlike the
    /// diff's own tri-state, a mutation never needs to distinguish "no-op" from "clear").
    SetIcc(set_icc::SetIcc),
    InsertFrame(insert_frame::InsertFrame),
    RemoveFrame(remove_frame::RemoveFrame),
    MoveFrame(move_frame::MoveFrame),
    SetFrameDelay(set_frame_delay::SetFrameDelay),
    SetFramePixels(set_frame_pixels::SetFramePixels),
    SetMetadataEntry(set_metadata_entry::SetMetadataEntry),
    RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry),
}

/// 🏷️ Kebab-case spelling of every `SemioImageMutation` variant, in declaration order — the
/// vocabulary the `semio-v1-image` mutation catalog (`../../🧪️oracle/🔣️.json`) declares and
/// `mutate-semio-image`'s exhaustive test case measures itself against.
pub const KINDS: &[&str] = &["set-snapshot", "set-dimensions", "set-colorspace", "set-bit-depth", "set-icc", "insert-frame", "remove-frame", "move-frame", "set-frame-delay", "set-frame-pixels", "set-metadata-entry", "remove-metadata-entry"];

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and every triad leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_image_mutation(snapshot: &mut SemioImageSnapshot, mutation: &SemioImageMutation) -> protocol::MutationOutcome<SemioImageDiff> {
    let outcome = <SemioImageMutation as Mutation<SemioImageSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `mutate-semio-image`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_image_mutation(mutation: &SemioImageMutation, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(mutation, base)
}
//#endregion 🔖️Mutation

//#region 🔖️MutationTrait
/// ↩️ An index/key that no longer exists in `base` has nothing to restore, so those arms return
/// the empty inverse rather than a sentinel no-op mutation — the convention this migration adopted
/// once `NoMutation` stopped being an available payload.
// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_diff(this: &SemioImageMutation, base: &SemioImageSnapshot) -> protocol::MutationOutcome<SemioImageDiff> {
    protocol::MutationOutcome::new(match this {
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => diff_set_snapshot(base, snapshot),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }) => SemioImageDiff { width: (base.width != *width).then_some(*width), height: (base.height != *height).then_some(*height), ..Default::default() },
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }) => SemioImageDiff { colorspace: (base.colorspace != *colorspace).then_some(*colorspace), ..Default::default() },
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth }) => SemioImageDiff { bit_depth: (base.bit_depth != *bit_depth).then_some(*bit_depth), ..Default::default() },
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc }) => SemioImageDiff { icc: (base.icc != *icc).then_some(icc.clone()), ..Default::default() },
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) => SemioImageDiff { frames: Some(SemioImageFramesDiff { added: vec![IndexAdded { index: *index, item: frame.clone() }], ..Default::default() }), ..Default::default() },
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => SemioImageDiff { frames: Some(SemioImageFramesDiff { removed: vec![*index], ..Default::default() }), ..Default::default() },
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => {
            let frames = base.frames.get(*from).map(|item| SemioImageFramesDiff { removed: vec![*from], added: vec![IndexAdded { index: *to, item: item.clone() }], ..Default::default() });
            SemioImageDiff { frames, ..Default::default() }
        }
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms }) => {
            SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: Some(*delay_ms), rgba8: None } }], ..Default::default() }), ..Default::default() }
        }
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }) => {
            SemioImageDiff { frames: Some(SemioImageFramesDiff { modified: vec![IndexModified { index: *index, diff: SemioImageFrameDiff { delay_ms: None, rgba8: Some(rgba8.clone()) } }], ..Default::default() }), ..Default::default() }
        }
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, value }) => {
            let metadata = if base.metadata.iter().any(|e| &e.key == key) {
                SemioImageMetadataDiff { modified: vec![NamedModified { key: key.clone(), diff: value.clone() }], ..Default::default() }
            } else {
                SemioImageMetadataDiff { added: vec![SemioImageMetadataEntry { key: key.clone(), value: value.clone() }], ..Default::default() }
            };
            SemioImageDiff { metadata: Some(metadata), ..Default::default() }
        }
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => SemioImageDiff { metadata: Some(SemioImageMetadataDiff { removed: vec![key.clone()], ..Default::default() }), ..Default::default() },
    })
}

// 🚫️async: E1 pure codec/computation helper — lifted verbatim from the former `impl Mutation`.
pub(crate) fn agg_inverse(this: &SemioImageMutation, base: &SemioImageSnapshot) -> Vec<SemioImageMutation> {
    vec![match this {
        SemioImageMutation::SetSnapshot(_) => SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
        SemioImageMutation::SetDimensions(_) => SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: base.width, height: base.height }),
        SemioImageMutation::SetColorspace(_) => SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: base.colorspace }),
        SemioImageMutation::SetBitDepth(_) => SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: base.bit_depth }),
        SemioImageMutation::SetIcc(_) => SemioImageMutation::SetIcc(set_icc::SetIcc { icc: base.icc.clone() }),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, .. }) => SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: *index }),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: *index, frame: frame.clone() }),
            None => return Vec::new(),
        },
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: *to, to: *from }),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, .. }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: *index, delay_ms: frame.delay_ms }),
            None => return Vec::new(),
        },
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, .. }) => match base.frames.get(*index) {
            Some(frame) => SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: *index, rgba8: frame.rgba8.clone() }),
            None => return Vec::new(),
        },
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, .. }) => match base.metadata.iter().find(|e| &e.key == key) {
            Some(entry) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone() }),
            None => SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: key.clone() }),
        },
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => match base.metadata.iter().find(|e| &e.key == key) {
            Some(entry) => SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: key.clone(), value: entry.value.clone() }),
            None => return Vec::new(),
        },
    }]
}
//#endregion 🔖️MutationTrait

//#region 🔖️OpCodecs
/// 🎙️ Hand-rolled `OpText`/`OpBinary` — same reasoning as `SemioImageDiff`'s hand-rolled
/// `DiffCodec` (see that module's doc comment): `SetIcc`'s `Option<Vec<u8>>` payload is the same
/// bare-`Option` shape the `dsl` derive machinery cannot bind, and per this ticket's own
/// instruction ("hand-roll all diff/op codecs — do not fight the derive"), every variant is
/// handcrafted rather than mixed derive/hand-roll. One space-free token per op: `tag` then `:`
/// then comma-separated positional fields (bracket-depth-aware, reusing the shared
/// `engine::triples` split/strip helpers so a nested `[...]` payload — e.g. `SetSnapshot`'s whole
/// snapshot — never confuses the top-level split).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_snapshot(s: &SemioImageSnapshot) -> String {
    let frames = s.frames.iter().map(enc_frame).collect::<Vec<_>>().join(",");
    let metadata = s.metadata.iter().map(enc_metadata_entry).collect::<Vec<_>>().join(",");
    format!("[{},{},{},{},{},[{}],[{}]]", s.width, s.height, enc_colorspace(s.colorspace), s.bit_depth, encode_option(&s.icc, |b| b.iter().map(|x| format!("{x:02x}")).collect::<String>()), frames, metadata,)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_snapshot(s: &str) -> Result<SemioImageSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, colorspace, bit_depth, icc, frames, metadata] = parts.as_slice() else {
        return Err(format!("snapshot: expected 7 fields, got {}", parts.len()));
    };
    let frames = split_top_level(strip_brackets(frames)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_frame).collect::<Result<Vec<_>, String>>()?;
    let metadata = split_top_level(strip_brackets(metadata)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_metadata_entry).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioImageSnapshot {
        schema: crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
        width: width.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        height: height.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        colorspace: dec_colorspace(colorspace)?,
        bit_depth: bit_depth.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        icc: decode_option(icc, |h| (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).map_err(|e| e.to_string())).collect())?,
        frames,
        metadata,
    })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_bytes(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(dec_bytes(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_image_mutation(m: &SemioImageMutation) -> String {
    match m {
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }) => format!("setSnapshot:{}", enc_snapshot(snapshot)),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }) => format!("setDimensions:{width},{height}"),
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }) => format!("setColorspace:{}", enc_colorspace(*colorspace)),
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth }) => format!("setBitDepth:{bit_depth}"),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc }) => format!("setIcc:{}", encode_option(icc, |b| enc_bytes(b))),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) => format!("insertFrame:{index},{}", enc_frame(frame)),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => format!("removeFrame:{index}"),
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => format!("moveFrame:{from},{to}"),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms }) => format!("setFrameDelay:{index},{delay_ms}"),
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }) => format!("setFramePixels:{index},{}", enc_bytes(rgba8)),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, value }) => format!("setMetadataEntry:{},{}", enc_str(key), enc_str(value)),
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => format!("removeMetadataEntry:{}", enc_str(key)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_image_mutation(line: &str) -> Result<SemioImageMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("mutation: missing tag separator in {line:?}"))?;
    match tag {
        "setSnapshot" => Ok(SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: dec_snapshot(rest)? })),
        "setDimensions" => {
            let parts = split_top_level(rest, ',');
            let [w, h] = parts.as_slice() else { return Err(format!("setDimensions: expected 2 fields, got {}", parts.len())) };
            Ok(SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: w.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, height: h.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }))
        }
        "setColorspace" => Ok(SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: dec_colorspace(rest)? })),
        "setBitDepth" => Ok(SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: rest.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "setIcc" => Ok(SemioImageMutation::SetIcc(set_icc::SetIcc { icc: decode_option(rest, dec_bytes)? })),
        "insertFrame" => {
            let (idx, frame) = rest.split_once(',').ok_or_else(|| "insertFrame: missing comma".to_string())?;
            Ok(SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, frame: dec_frame(frame)? }))
        }
        "removeFrame" => Ok(SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: rest.parse().map_err(|e: std::num::ParseIntError| e.to_string())? })),
        "moveFrame" => {
            let parts = split_top_level(rest, ',');
            let [from, to] = parts.as_slice() else { return Err(format!("moveFrame: expected 2 fields, got {}", parts.len())) };
            Ok(SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: from.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, to: to.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }))
        }
        "setFrameDelay" => {
            let parts = split_top_level(rest, ',');
            let [idx, delay] = parts.as_slice() else { return Err(format!("setFrameDelay: expected 2 fields, got {}", parts.len())) };
            Ok(SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, delay_ms: delay.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }))
        }
        "setFramePixels" => {
            let (idx, rgba) = rest.split_once(',').ok_or_else(|| "setFramePixels: missing comma".to_string())?;
            Ok(SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: idx.parse().map_err(|e: std::num::ParseIntError| e.to_string())?, rgba8: dec_bytes(rgba)? }))
        }
        "setMetadataEntry" => {
            let parts = split_top_level(rest, ',');
            let [key, value] = parts.as_slice() else { return Err(format!("setMetadataEntry: expected 2 fields, got {}", parts.len())) };
            Ok(SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: dec_str(key)?, value: dec_str(value)? }))
        }
        "removeMetadataEntry" => Ok(SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: dec_str(rest)? })),
        other => Err(format!("mutation: unknown tag {other:?}")),
    }
}

impl OpText for SemioImageMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_image_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_image_mutation(self)
    }
}

/// 🧾️ Keyword table + variant ordinal, 0-indexed in enum declaration order — the binary frame's
/// `tag` byte, `📖️grammar/component.grammar.semio`'s `op` alternatives, and this array must all
/// agree (see `committed_facet_files_parse`/`ops_grammar_conformance_law` in
/// `🎹️composer/🦀️component.rs`).
const OP_KEYWORDS: [&str; 12] = ["setSnapshot", "setDimensions", "setColorspace", "setBitDepth", "setIcc", "insertFrame", "removeFrame", "moveFrame", "setFrameDelay", "setFramePixels", "setMetadataEntry", "removeMetadataEntry"];
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn variant_ordinal(m: &SemioImageMutation) -> u8 {
    match m {
        SemioImageMutation::SetSnapshot(_) => 0,
        SemioImageMutation::SetDimensions(_) => 1,
        SemioImageMutation::SetColorspace(_) => 2,
        SemioImageMutation::SetBitDepth(_) => 3,
        SemioImageMutation::SetIcc(_) => 4,
        SemioImageMutation::InsertFrame(_) => 5,
        SemioImageMutation::RemoveFrame(_) => 6,
        SemioImageMutation::MoveFrame(_) => 7,
        SemioImageMutation::SetFrameDelay(_) => 8,
        SemioImageMutation::SetFramePixels(_) => 9,
        SemioImageMutation::SetMetadataEntry(_) => 10,
        SemioImageMutation::RemoveMetadataEntry(_) => 11,
    }
}
/// ✂️ Just the argument tail of `print_image_mutation` — the binary frame's `tag` byte already
/// carries the keyword, so the text keyword itself (and its `:` separator) is redundant in the
/// binary payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_image_mutation_args(m: &SemioImageMutation) -> String {
    match print_image_mutation(m).split_once(':') {
        Some((_, rest)) => rest.to_string(),
        None => String::new(),
    }
}

/// ⚡️ Real binary op frame, replacing the old `print_op().into_bytes()` text-as-binary shortcut.
/// `format u8` (`OP_BINARY_FORMAT` convention) + `tag u8` (the variant ordinal, see
/// [`OP_KEYWORDS`]) are two REAL fixed fields; the variant's own argument payload follows as one
/// opaque trailing `bytes` chain — reuses the already-real, already-tested `print_image_mutation`/
/// `parse_image_mutation` text codec rather than re-deriving a second independent encoding.
impl protocol::OpBinary for SemioImageMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, variant_ordinal(self)];
        out.extend_from_slice(print_image_mutation_args(self).as_bytes());
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated (need format+tag)".to_string() });
        }
        if bytes[0] != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {}", bytes[0]) });
        }
        let tag = bytes[1];
        let keyword = OP_KEYWORDS.get(tag as usize).ok_or_else(|| protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("tag {tag} out of range for {} declared variants", OP_KEYWORDS.len()) })?;
        let args = std::str::from_utf8(&bytes[2..]).map_err(|e| protocol::ProtocolError::Malformed { what: "op utf8", offset: 2, detail: e.to_string() })?;
        let line = if args.is_empty() { keyword.to_string() } else { format!("{keyword}:{args}") };
        Self::parse_op(&line).map_err(|e| protocol::ProtocolError::Malformed { what: "op text", offset: 2, detail: e.to_string() })
    }
}
//#endregion 🔖️OpCodecs

//#region 🔖️Demo
/// 🌱 Representative `SemioImageMutation` cases, one per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs` and this
/// file's own `op_text_binary_roundtrip_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioImageMutation> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 4,
            height: 4,
            colorspace: SemioColorspace::Rgba,
            bit_depth: 8,
            frames: vec![frame(1, 16), frame(2, 16)],
            icc: Some(vec![9, 9]),
            metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "old".into() }],
            ..SemioImageSnapshot::default()
        }
    }
    vec![
        SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: fixture() }),
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: 8, height: 8 }),
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: SemioColorspace::Grayscale }),
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: 16 }),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc: None }),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc: Some(vec![1, 2, 3]) }),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: 1, frame: frame(5, 16) }),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: 0 }),
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: 0, to: 1 }),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_ms: 250 }),
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 1, rgba8: vec![7; 16] }),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Title".into(), value: "new".into() }),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "someone".into() }),
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: "Title".into() }),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    /// 🔧️ `DiffAlgebra` lives at `protocol::command::DiffAlgebra`, not `protocol::DiffAlgebra`
    /// (W2b closer fix — was an unresolved-import compile error).
    use protocol::command::DiffAlgebra;
    use protocol::{MutationDiff, OpBinary, OpText};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn frame(seed: u8, len: usize) -> SemioImageFrame {
        SemioImageFrame { delay_ms: 100, rgba8: vec![seed; len] }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture() -> SemioImageSnapshot {
        SemioImageSnapshot {
            width: 4,
            height: 4,
            colorspace: SemioColorspace::Rgba,
            bit_depth: 8,
            frames: vec![frame(1, 16), frame(2, 16)],
            icc: Some(vec![9, 9]),
            metadata: vec![SemioImageMetadataEntry { key: "Title".into(), value: "old".into() }],
            ..SemioImageSnapshot::default()
        }
    }

    /// 🌱 Reuses `demo_mutation_cases()` (single source of truth, also feeds
    /// `ops_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`) rather
    /// than an independent copy.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_mutations() -> Vec<SemioImageMutation> {
        demo_mutation_cases()
    }

    //#region 🔖️MutationDiffLaw
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).expect("apply must succeed for a well-formed fixture");

            let mut via_apply = base.clone();
            let diff_from_apply = apply_semio_image_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_semio_image_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&mutation, &base) {
                apply_semio_image_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level).await failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(diff.diff(), &base).expect("apply must succeed for a well-formed fixture");
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = MutationDiff::apply(&inverse_diff, &next).expect("apply must succeed for a well-formed fixture");
            assert_eq!(restored, base, "inverse_law (diff-level).await failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️CodecRetentionLaw
    /// 🧪️ codec_retention_law: `ArtifactPack` decode(encode(snapshot)) on a real (mutation-built,
    /// not just default) snapshot.
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let mut snap = fixture();
        apply_semio_image_mutation(&mut snap, &SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "x".into() }));
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️OpTextBinaryRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for m in sample_mutations() {
            let printed = m.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioImageMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?} (printed {printed:?})");

            let encoded = m.encode_op().unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioImageMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw

    //#region 🔖️CatalogLaw
    /// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a new variant has
    /// no arm here, so the crate stops building until both this match and `KINDS` name it.
    fn kind_of(mutation: &SemioImageMutation) -> &'static str {
        match mutation {
            SemioImageMutation::SetSnapshot(_) => "set-snapshot",
            SemioImageMutation::SetDimensions(_) => "set-dimensions",
            SemioImageMutation::SetColorspace(_) => "set-colorspace",
            SemioImageMutation::SetBitDepth(_) => "set-bit-depth",
            SemioImageMutation::SetIcc(_) => "set-icc",
            SemioImageMutation::InsertFrame(_) => "insert-frame",
            SemioImageMutation::RemoveFrame(_) => "remove-frame",
            SemioImageMutation::MoveFrame(_) => "move-frame",
            SemioImageMutation::SetFrameDelay(_) => "set-frame-delay",
            SemioImageMutation::SetFramePixels(_) => "set-frame-pixels",
            SemioImageMutation::SetMetadataEntry(_) => "set-metadata-entry",
            SemioImageMutation::RemoveMetadataEntry(_) => "remove-metadata-entry",
        }
    }

    /// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
    /// the committed `semio-v1-image` catalog carries — the framework never parses Rust, so this is
    /// the only thing that keeps the catalog honest against the enum.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let one_per_variant = [
            SemioImageMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: SemioImageSnapshot::default() }),
            SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width: 4, height: 2 }),
            SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace: SemioColorspace::Rgba }),
            SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth: 16 }),
            SemioImageMutation::SetIcc(set_icc::SetIcc { icc: None }),
            SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index: 0, frame: SemioImageFrame::default() }),
            SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index: 0 }),
            SemioImageMutation::MoveFrame(move_frame::MoveFrame { from: 1, to: 0 }),
            SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index: 0, delay_ms: 40 }),
            SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index: 0, rgba8: Vec::new() }),
            SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: "Author".into(), value: "semio".into() }),
            SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: "Author".into() }),
        ];
        assert_eq!(KINDS.len(), one_per_variant.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, mutation) in KINDS.iter().zip(one_per_variant.iter()) {
            assert_eq!(*kind, kind_of(mutation), "KINDS must follow the enum's own declaration order and kebab-case spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🔖️CatalogLaw
}
//#endregion 🔖️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`)
/// — one case per triad leaf, self-wired here rather than in `📦️glue.rs` so this subset owns its
/// own test surface. `#[path = "."]` re-roots the nested `#[path]`s at THIS file's directory (the
/// `🧬️mutations` root) instead of the implicit `🦀️component/` child directory. Each case file
/// additionally mounts its OWN leaf `🔺️diff` module, because the enum arms above carry no guard
/// branches — the leaves own every diagnostic.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "➕️insert-frame/🧪️tests/appends-a-second-frame-at-the-end/🦀️component.rs"]
    mod tests_insert_frame_appends_a_second_frame_at_the_end;
    #[path = "🔀️move-frame/🧪️tests/moves-the-last-frame-to-the-front/🦀️component.rs"]
    mod tests_move_frame_moves_the_last_frame_to_the_front;
    #[path = "📄remove-frame/🧪️tests/removes-the-leading-frame/🦀️component.rs"]
    mod tests_remove_frame_removes_the_leading_frame;
    #[path = "🗑️remove-metadata-entry/🧪️tests/removes-the-comment-entry-and-keeps-the-author-entry/🦀️component.rs"]
    mod tests_remove_metadata_entry_removes_the_comment_entry_and_keeps_the_author_entry;
    #[path = "🔢️set-bit-depth/🧪️tests/raises-the-source-bit-depth-to-sixteen/🦀️component.rs"]
    mod tests_set_bit_depth_raises_the_source_bit_depth_to_sixteen;
    #[path = "🌈️set-colorspace/🧪️tests/records-the-source-colorspace-as-rgba/🦀️component.rs"]
    mod tests_set_colorspace_records_the_source_colorspace_as_rgba;
    #[path = "📐️set-dimensions/🧪️tests/widens-the-frameless-canvas-to-four-by-two/🦀️component.rs"]
    mod tests_set_dimensions_widens_the_frameless_canvas_to_four_by_two;
    #[path = "⏱️set-frame-delay/🧪️tests/slows-the-second-frame-down/🦀️component.rs"]
    mod tests_set_frame_delay_slows_the_second_frame_down;
    #[path = "🟪️set-frame-pixels/🧪️tests/repaints-the-only-frame-black/🦀️component.rs"]
    mod tests_set_frame_pixels_repaints_the_only_frame_black;
    #[path = "🎨️set-icc/🧪️tests/attaches-an-icc-profile-where-there-was-none/🦀️component.rs"]
    mod tests_set_icc_attaches_an_icc_profile_where_there_was_none;
    #[path = "🏷️set-metadata-entry/🧪️tests/rewrites-the-existing-author-entry/🦀️component.rs"]
    mod tests_set_metadata_entry_rewrites_the_existing_author_entry;
    #[path = "📸️set-snapshot/🧪️tests/retargets-the-document-onto-a-grayscale-sixteen-bit-variant/🦀️component.rs"]
    mod tests_set_snapshot_retargets_the_document_onto_a_grayscale_sixteen_bit_variant;
}
//#endregion 🧪️FixtureTests
