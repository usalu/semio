//! 📝️ Text representation codec surface for `stdio.semio.image` (mutations) — grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

/// 🧾️ Each record kind's text-grammar keyword, the head `decode_op` re-prefixes onto the argument tail before `parse_op`.
pub(crate) const TEXT_KEYWORDS: [(&str, &str); 11] = [
    ("set-dimensions", "setDimensions"),
    ("set-colorspace", "setColorspace"),
    ("set-bit-depth", "setBitDepth"),
    ("set-icc", "setIcc"),
    ("insert-frame", "insertFrame"),
    ("remove-frame", "removeFrame"),
    ("move-frame", "moveFrame"),
    ("set-frame-delay", "setFrameDelay"),
    ("set-frame-pixels", "setFramePixels"),
    ("set-metadata-entry", "setMetadataEntry"),
    ("remove-metadata-entry", "removeMetadataEntry"),
];

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::image::schema::mutations::*;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, NamedModified};
use crate::standards::v1::subsets::base::io::text::snapshot::{split_top_level, strip_brackets};
use crate::standards::v1::subsets::image::schema::diff::{SemioImageDiff, SemioImageFrameDiff, SemioImageFramesDiff, SemioImageMetadataDiff};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_metadata_entry};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_metadata_entry};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_frame};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_frame};
use crate::standards::v1::subsets::image::io::text::snapshot::{dec_colorspace};
use crate::standards::v1::subsets::image::io::text::snapshot::{enc_colorspace};
use crate::standards::v1::subsets::image::io::text::snapshot::{decode_option};
use crate::standards::v1::subsets::image::io::text::snapshot::{encode_option};
use crate::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot};
use protocol::Mutation;
/// 🔧️ Unconditional — `impl protocol::OpBinary for SemioImageMutation` below calls
/// `self.print_op()`/`Self::parse_op(...)` via method syntax, which needs `OpText` in scope in
/// production code (was missing entirely, even test-gated) (W2b closer fix).
use protocol::OpText;

/// 🎙️ Hand-rolled `OpText`/`OpBinary` — same reasoning as `SemioImageDiff`'s hand-rolled
/// `DiffCodec` (see that module's doc comment): `SetIcc`'s `Option<Vec<u8>>` payload is the same
/// bare-`Option` shape the `dsl` derive machinery cannot bind, and per this ticket's own
/// instruction ("hand-roll all diff/op codecs — do not fight the derive"), every variant is
/// handcrafted rather than mixed derive/hand-roll. One space-free token per op: `tag` then `:`
/// then comma-separated positional fields (bracket-depth-aware, reusing the shared
/// `engine::triples` split/strip helpers so a nested `[...]` payload — e.g. `SetSnapshot`'s whole
/// snapshot — never confuses the top-level split).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_snapshot(s: &SemioImageSnapshot) -> String {
    let frames = s.frames.iter().map(enc_frame).collect::<Vec<_>>().join(",");
    let metadata = s.metadata.iter().map(enc_metadata_entry).collect::<Vec<_>>().join(",");
    format!("[{},{},{},{},{},[{}],[{}]]", s.width, s.height, enc_colorspace(s.colorspace), s.bit_depth, encode_option(&s.icc, |b| b.iter().map(|x| format!("{x:02x}")).collect::<String>()), frames, metadata,)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_snapshot(s: &str) -> Result<SemioImageSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [width, height, colorspace, bit_depth, icc, frames, metadata] = parts.as_slice() else {
        return Err(format!("snapshot: expected 7 fields, got {}", parts.len()));
    };
    let frames = split_top_level(strip_brackets(frames)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_frame).collect::<Result<Vec<_>, String>>()?;
    let metadata = split_top_level(strip_brackets(metadata)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_metadata_entry).collect::<Result<Vec<_>, String>>()?;
    Ok(SemioImageSnapshot {
        schema: crate::standards::v1::subsets::image::schema::snapshot::STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),
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
pub(crate) fn enc_bytes(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_bytes(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_str(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(dec_bytes(s)?).map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_image_mutation(m: &SemioImageMutation) -> String {
    match m {
        SemioImageMutation::SetDimensions(set_dimensions::SetDimensions { width, height }) => format!("setDimensions:{width},{height}"),
        SemioImageMutation::SetColorspace(set_colorspace::SetColorspace { colorspace }) => format!("setColorspace:{}", enc_colorspace(*colorspace)),
        SemioImageMutation::SetBitDepth(set_bit_depth::SetBitDepth { bit_depth }) => format!("setBitDepth:{bit_depth}"),
        SemioImageMutation::SetIcc(set_icc::SetIcc { icc }) => format!("setIcc:{}", encode_option(icc, |b| enc_bytes(b))),
        SemioImageMutation::InsertFrame(insert_frame::InsertFrame { index, frame }) => format!("insertFrame:{index},{}", enc_frame(frame)),
        SemioImageMutation::RemoveFrame(remove_frame::RemoveFrame { index }) => format!("removeFrame:{index}"),
        SemioImageMutation::MoveFrame(move_frame::MoveFrame { from, to }) => format!("moveFrame:{from},{to}"),
        SemioImageMutation::SetFrameDelay(set_frame_delay::SetFrameDelay { index, delay_ms }) => format!("setFrameDelay:{index},{delay_ms}"),
        SemioImageMutation::SetFramePixels(set_frame_pixels::SetFramePixels { index, rgba8 }) => format!("setFramePixels:{index},{}", enc_bytes(rgba8)),
        SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key, value, at }) => format!("setMetadataEntry:{},{}{}", enc_str(key), enc_str(value), at.map(|at| format!(",{at}")).unwrap_or_default()),
        SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key }) => format!("removeMetadataEntry:{}", enc_str(key)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_image_mutation(line: &str) -> Result<SemioImageMutation, String> {
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("mutation: missing tag separator in {line:?}"))?;
    match tag {
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
            let (key, value, at) = match parts.as_slice() {
                [key, value] => (key, value, None),
                [key, value, at] => (key, value, Some(at.parse::<usize>().map_err(|error| error.to_string())?)),
                _ => return Err(format!("setMetadataEntry: expected 2 or 3 fields, got {}", parts.len())),
            };
            Ok(SemioImageMutation::SetMetadataEntry(set_metadata_entry::SetMetadataEntry { key: dec_str(key)?, value: dec_str(value)?, at }))
        }
        "removeMetadataEntry" => Ok(SemioImageMutation::RemoveMetadataEntry(remove_metadata_entry::RemoveMetadataEntry { key: dec_str(rest)? })),
        other => Err(format!("mutation: unknown tag {other:?}")),
    }
}

impl OpText for SemioImageMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_image_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_image_mutation(self)
    }
}
}
pub use mutations_codec::*;
