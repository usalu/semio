//! 📝️ Text representation codec surface for `stdio.semio.presentation` (diff).

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

#[allow(unused_imports)]
mod diff_codec {
use super::*;
use crate::standards::v1::subsets::presentation::io::binary::diff::{encode_option, decode_option};
use crate::standards::v1::subsets::presentation::schema::diff::*;
use protocol::{DiffText,DiffBinary};
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff, NamedModified, NamedTripleDiff};
use crate::standards::v1::subsets::base::io::text::snapshot::{dec_named_added, enc_named_added};
use crate::standards::v1::subsets::audio::io::text::diff::{strip_brackets};
use crate::standards::v1::subsets::audio::io::text::diff::{split_top_level};
/// 🧱️ REUSE, don't reinvent — `document::DocBlock`'s own real, already-tested text codec
/// (`ws-codec-document-report.md`), re-exported here so both this file's own leaf encoders AND
/// the sibling `🧬️mutations`/`📸️snapshot` facets can import `{enc_block, dec_block}` from THIS
/// module (matching the pre-existing convention where this file is the one place that owns every
/// value codec presentation's other facets import from).
use crate::standards::v1::subsets::document::io::text::diff::{dec_block};
use crate::standards::v1::subsets::document::io::text::diff::{enc_block};
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot;
use crate::standards::v1::subsets::presentation::schema::snapshot::{PlaceholderKind, Slide, SlideFrame, SlideLayout, SlideMaster, SlidePictureImage, SlideShape, SlideTableCell, SlideTableRow};
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn print_presentation_diff(d: &SemioPresentationDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.masters {
        tokens.push(format!("masters={}", enc_masters_diff(v)));
    }
    if let Some(v) = &d.layouts {
        tokens.push(format!("layouts={}", enc_layouts_diff(v)));
    }
    if let Some(v) = &d.slides {
        tokens.push(format!("slides={}", enc_slides_diff(v)));
    }
    tokens.join(" ")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_presentation_diff(line: &str) -> Result<SemioPresentationDiff, String> {
    let mut d = SemioPresentationDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("masters=") {
            d.masters = Some(dec_masters_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("layouts=") {
            d.layouts = Some(dec_layouts_diff(rest)?);
        } else if let Some(rest) = token.strip_prefix("slides=") {
            d.slides = Some(dec_slides_diff(rest)?);
        } else {
            return Err(format!("presentation diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

impl protocol::DiffText for SemioPresentationDiff {
fn print_diff(&self) -> String {
    print_presentation_diff(self)
}
fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
    parse_presentation_diff(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
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
pub(crate) fn enc_f64(v: f64) -> String {
    v.to_bits().to_string()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_f64(s: &str) -> Result<f64, String> {
    s.parse::<u64>().map(f64::from_bits).map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(enc).collect::<Vec<_>>().join(","))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_semio_point2(p: &SemioPoint2) -> String {
    format!("[{},{}]", enc_f64(p.x), enc_f64(p.y))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_semio_point2(s: &str) -> Result<SemioPoint2, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [x, y] = parts.as_slice() else { return Err(format!("point2: expected 2 fields, got {}", parts.len())) };
    Ok(SemioPoint2 { x: dec_f64(x)?, y: dec_f64(y)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame(f: &SlideFrame) -> String {
    format!("[{},{},{}]", enc_semio_point2(&f.origin), enc_f64(f.width), enc_f64(f.height))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame(s: &str) -> Result<SlideFrame, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [origin, width, height] = parts.as_slice() else { return Err(format!("frame: expected 3 fields, got {}", parts.len())) };
    Ok(SlideFrame { origin: dec_semio_point2(origin)?, width: dec_f64(width)?, height: dec_f64(height)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image(i: &SlidePictureImage) -> String {
    format!("[{},{},{}]", enc_str(&i.asset_id), enc_str(&i.mime), hex_encode(&i.bytes))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image(s: &str) -> Result<SlidePictureImage, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [asset_id, mime, bytes] = parts.as_slice() else { return Err(format!("image: expected 3 fields, got {}", parts.len())) };
    Ok(SlidePictureImage { asset_id: dec_str(asset_id)?, mime: dec_str(mime)?, bytes: hex_decode(bytes)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_placeholder_kind(k: &PlaceholderKind) -> String {
    match k {
        PlaceholderKind::Title => "T".to_string(),
        PlaceholderKind::Subtitle => "S".to_string(),
        PlaceholderKind::Body => "B".to_string(),
        PlaceholderKind::Footer => "F".to_string(),
        PlaceholderKind::SlideNumber => "N".to_string(),
        PlaceholderKind::DateTime => "D".to_string(),
        PlaceholderKind::Other { value } => format!("O[{}]", enc_str(value)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_placeholder_kind(s: &str) -> Result<PlaceholderKind, String> {
    match s {
        "T" => Ok(PlaceholderKind::Title),
        "S" => Ok(PlaceholderKind::Subtitle),
        "B" => Ok(PlaceholderKind::Body),
        "F" => Ok(PlaceholderKind::Footer),
        "N" => Ok(PlaceholderKind::SlideNumber),
        "D" => Ok(PlaceholderKind::DateTime),
        other if other.starts_with('O') => Ok(PlaceholderKind::Other { value: dec_str(strip_brackets(&other[1..])?)? }),
        other => Err(format!("placeholder kind: unknown tag {other:?}")),
    }
}

/// 🧱️ `DocRun`/`RunStyle`/`DocListItem`/`DocTableCell`/`DocTableRow`/`DocBlock` are all OWNED by
/// `document` — no local codec for any of them lives here anymore. `enc_block`/`dec_block`
/// (re-exported above from `document::schema::diff`, the same real, already-tested codec
/// `ws-codec-document-report.md` landed) already handles every one of these leaf types internally
/// (Paragraph/Heading's `runs: Vec<DocRun>`, List's `items: Vec<DocListItem>`, Table's
/// `rows: Vec<DocTableRow>` -> `cells: Vec<DocTableCell>`, Quote's recursive `Vec<DocBlock>`) — a
/// prior draft of this file duplicated all of these, a real policy violation this wave fixes.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_cell(c: &SlideTableCell) -> String {
    enc_list(&c.blocks, enc_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_cell(s: &str) -> Result<SlideTableCell, String> {
    Ok(SlideTableCell { blocks: dec_list(s, dec_block)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_row(r: &SlideTableRow) -> String {
    enc_list(&r.cells, enc_table_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_row(s: &str) -> Result<SlideTableRow, String> {
    Ok(SlideTableRow { cells: dec_list(s, dec_table_cell)? })
}

/// 🌳️ `X[frame,blocks]` TextBox / `P[frame,image]` Picture / `T[frame,rows]` Table /
/// `H[frame,kind]` placeHolder.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shape(shape: &SlideShape) -> String {
    match shape {
        SlideShape::TextBox { frame, blocks } => format!("X[{},{}]", enc_frame(frame), enc_list(blocks, enc_block)),
        SlideShape::Picture { frame, image } => format!("P[{},{}]", enc_frame(frame), enc_image(image)),
        SlideShape::Table { frame, rows } => format!("T[{},{}]", enc_frame(frame), enc_list(rows, enc_table_row)),
        SlideShape::Placeholder { frame, kind } => format!("H[{},{}]", enc_frame(frame), enc_placeholder_kind(kind)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shape(s: &str) -> Result<SlideShape, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "X" => {
            let [frame, blocks] = parts.as_slice() else { return Err(format!("textbox: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::TextBox { frame: dec_frame(frame)?, blocks: dec_list(blocks, dec_block)? })
        }
        "P" => {
            let [frame, image] = parts.as_slice() else { return Err(format!("picture: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Picture { frame: dec_frame(frame)?, image: dec_image(image)? })
        }
        "T" => {
            let [frame, rows] = parts.as_slice() else { return Err(format!("table: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Table { frame: dec_frame(frame)?, rows: dec_list(rows, dec_table_row)? })
        }
        "H" => {
            let [frame, kind] = parts.as_slice() else { return Err(format!("placeholder: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShape::Placeholder { frame: dec_frame(frame)?, kind: dec_placeholder_kind(kind)? })
        }
        other => Err(format!("shape: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_master(m: &SlideMaster) -> String {
    format!("[{},{}]", enc_str(&m.id), enc_list(&m.shapes, enc_shape))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_master(s: &str) -> Result<SlideMaster, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, shapes] = parts.as_slice() else { return Err(format!("master: expected 2 fields, got {}", parts.len())) };
    Ok(SlideMaster { id: dec_str(id)?, shapes: dec_list(shapes, dec_shape)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layout(l: &SlideLayout) -> String {
    format!("[{},{},{}]", enc_str(&l.id), enc_str(&l.master_id), enc_list(&l.shapes, enc_shape))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layout(s: &str) -> Result<SlideLayout, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, master_id, shapes] = parts.as_slice() else { return Err(format!("layout: expected 3 fields, got {}", parts.len())) };
    Ok(SlideLayout { id: dec_str(id)?, master_id: dec_str(master_id)?, shapes: dec_list(shapes, dec_shape)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_slide(sl: &Slide) -> String {
    format!("[{},{},{},{}]", enc_str(&sl.id), encode_option(&sl.layout_id, |v| enc_str(v)), enc_list(&sl.shapes, enc_shape), enc_list(&sl.notes, enc_block))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_slide(s: &str) -> Result<Slide, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, layout_id, shapes, notes] = parts.as_slice() else { return Err(format!("slide: expected 4 fields, got {}", parts.len())) };
    Ok(Slide { id: dec_str(id)?, layout_id: decode_option(layout_id, dec_str)?, shapes: dec_list(shapes, dec_shape)?, notes: dec_list(notes, dec_block)? })
}

/// 🌳️ `[removed];[modified];[added]` — generic over `IndexedTripleDiff<D,T>`'s own `D`/`T`, local
/// copy (see the file's `GenericCollectionTriples` doc comment for why not the shared one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_indexed_triple<D, T>(diff: &IndexedTripleDiff<D, T>, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", m.index, enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(|a| format!("{}:{}", a.index, enc_t(&a.item))).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_indexed_triple<D, T>(body: &str, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<IndexedTripleDiff<D, T>, String> {
    let three = split_top_level(body, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("indexed triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(parse_usize).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed modified: bad entry {entry:?}"))?;
            Ok(IndexModified { index: parse_usize(idx)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (idx, rest) = entry.split_once(':').ok_or_else(|| format!("indexed added: bad entry {entry:?}"))?;
            Ok(IndexAdded { index: parse_usize(idx)?, item: dec_t(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(IndexedTripleDiff { removed, modified, added })
}

/// 🏷️ `[removed];[modified];[added]` — generic over `NamedTripleDiff<K,D,T>`'s own `K`/`D`/`T`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_named_triple<K, D, T>(diff: &NamedTripleDiff<K, D, T>, enc_k: impl Fn(&K) -> String, enc_d: impl Fn(&D) -> String, enc_t: impl Fn(&T) -> String) -> String {
    let removed = diff.removed.iter().map(&enc_k).collect::<Vec<_>>().join(",");
    let modified = diff.modified.iter().map(|m| format!("{}:{}", enc_k(&m.key), enc_d(&m.diff))).collect::<Vec<_>>().join(",");
    let added = diff.added.iter().map(enc_t).collect::<Vec<_>>().join(",");
    format!("[{removed}];[{modified}];[{added}]")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_named_triple<K, D, T>(s: &str, dec_k: impl Fn(&str) -> Result<K, String>, dec_d: impl Fn(&str) -> Result<D, String>, dec_t: impl Fn(&str) -> Result<T, String>) -> Result<NamedTripleDiff<K, D, T>, String> {
    let three = split_top_level(s, ';');
    let [removed_s, modified_s, added_s] = three.as_slice() else { return Err(format!("named triple: expected 3 sections, got {}", three.len())) };
    let removed = split_top_level(strip_brackets(removed_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(&dec_k).collect::<Result<Vec<_>, String>>()?;
    let modified = split_top_level(strip_brackets(modified_s)?, ',')
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|entry| {
            let (k, rest) = entry.split_once(':').ok_or_else(|| format!("named triple modified: bad entry {entry:?}"))?;
            Ok(NamedModified { key: dec_k(k)?, diff: dec_d(rest)? })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let added = split_top_level(strip_brackets(added_s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_t).collect::<Result<Vec<_>, String>>()?;
    Ok(NamedTripleDiff { removed, modified, added })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_frame_diff(d: &SlideFrameDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.origin, enc_semio_point2), encode_option(&d.width, |v| enc_f64(*v)), encode_option(&d.height, |v| enc_f64(*v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_frame_diff(s: &str) -> Result<SlideFrameDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [origin, width, height] = parts.as_slice() else { return Err(format!("frame diff: expected 3 fields, got {}", parts.len())) };
    Ok(SlideFrameDiff { origin: decode_option(origin, dec_semio_point2)?, width: decode_option(width, dec_f64)?, height: decode_option(height, dec_f64)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_image_diff(d: &SlidePictureImageDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.asset_id, |v| enc_str(v)), encode_option(&d.mime, |v| enc_str(v)), encode_option(&d.bytes, |v: &Vec<u8>| hex_encode(v)))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_image_diff(s: &str) -> Result<SlidePictureImageDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [asset_id, mime, bytes] = parts.as_slice() else { return Err(format!("image diff: expected 3 fields, got {}", parts.len())) };
    Ok(SlidePictureImageDiff { asset_id: decode_option(asset_id, dec_str)?, mime: decode_option(mime, dec_str)?, bytes: decode_option(bytes, hex_decode)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_doc_blocks_diff(d: &DocBlocksDiff) -> String {
    enc_indexed_triple(d, enc_block, enc_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_doc_blocks_diff(s: &str) -> Result<DocBlocksDiff, String> {
    dec_indexed_triple(s, dec_block, dec_block)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_cell_diff(d: &SlideTableCellDiff) -> String {
    format!("[{}]", encode_option(&d.blocks, enc_doc_blocks_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_cell_diff(s: &str) -> Result<SlideTableCellDiff, String> {
    Ok(SlideTableCellDiff { blocks: decode_option(strip_brackets(s)?, dec_doc_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_cells_diff(d: &SlideTableCellsDiff) -> String {
    enc_indexed_triple(d, enc_table_cell_diff, enc_table_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_cells_diff(s: &str) -> Result<SlideTableCellsDiff, String> {
    dec_indexed_triple(s, dec_table_cell_diff, dec_table_cell)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_row_diff(d: &SlideTableRowDiff) -> String {
    format!("[{}]", encode_option(&d.cells, enc_table_cells_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_row_diff(s: &str) -> Result<SlideTableRowDiff, String> {
    Ok(SlideTableRowDiff { cells: decode_option(strip_brackets(s)?, dec_table_cells_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_table_rows_diff(d: &SlideTableRowsDiff) -> String {
    enc_indexed_triple(d, enc_table_row_diff, enc_table_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_table_rows_diff(s: &str) -> Result<SlideTableRowsDiff, String> {
    dec_indexed_triple(s, dec_table_row_diff, dec_table_row)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shapes_diff(d: &SlideShapesDiff) -> String {
    enc_indexed_triple(d, enc_shape_diff, enc_shape)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shapes_diff(s: &str) -> Result<SlideShapesDiff, String> {
    dec_indexed_triple(s, dec_shape_diff, dec_shape)
}

/// 🌳️ `X[frame,blocks]`/`P[frame,image]`/`T[frame,rows]`/`H[frame,kind]`/`R[shape]` (wholesale
/// replace, shape KIND changed).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_shape_diff(d: &SlideShapeDiff) -> String {
    match d {
        SlideShapeDiff::TextBox { frame, blocks } => format!("X[{},{}]", encode_option(frame, enc_frame_diff), encode_option(blocks, enc_doc_blocks_diff)),
        SlideShapeDiff::Picture { frame, image } => format!("P[{},{}]", encode_option(frame, enc_frame_diff), encode_option(image, enc_image_diff)),
        SlideShapeDiff::Table { frame, rows } => format!("T[{},{}]", encode_option(frame, enc_frame_diff), encode_option(rows, enc_table_rows_diff)),
        SlideShapeDiff::Placeholder { frame, kind } => format!("H[{},{}]", encode_option(frame, enc_frame_diff), encode_option(kind, enc_placeholder_kind)),
        SlideShapeDiff::Replace { shape } => format!("R[{}]", enc_shape(shape)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_shape_diff(s: &str) -> Result<SlideShapeDiff, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "X" => {
            let parts = split_top_level(inner, ',');
            let [frame, blocks] = parts.as_slice() else { return Err(format!("textbox diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::TextBox { frame: decode_option(frame, dec_frame_diff)?, blocks: decode_option(blocks, dec_doc_blocks_diff)? })
        }
        "P" => {
            let parts = split_top_level(inner, ',');
            let [frame, image] = parts.as_slice() else { return Err(format!("picture diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Picture { frame: decode_option(frame, dec_frame_diff)?, image: decode_option(image, dec_image_diff)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [frame, rows] = parts.as_slice() else { return Err(format!("table diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Table { frame: decode_option(frame, dec_frame_diff)?, rows: decode_option(rows, dec_table_rows_diff)? })
        }
        "H" => {
            let parts = split_top_level(inner, ',');
            let [frame, kind] = parts.as_slice() else { return Err(format!("placeholder diff: expected 2 fields, got {}", parts.len())) };
            Ok(SlideShapeDiff::Placeholder { frame: decode_option(frame, dec_frame_diff)?, kind: decode_option(kind, dec_placeholder_kind)? })
        }
        "R" => Ok(SlideShapeDiff::Replace { shape: dec_shape(inner)? }),
        other => Err(format!("shape diff: unknown tag {other:?}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_master_diff(d: &SlideMasterDiff) -> String {
    format!("[{}]", encode_option(&d.shapes, enc_shapes_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_master_diff(s: &str) -> Result<SlideMasterDiff, String> {
    Ok(SlideMasterDiff { shapes: decode_option(strip_brackets(s)?, dec_shapes_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layout_diff(d: &SlideLayoutDiff) -> String {
    format!("[{},{}]", encode_option(&d.master_id, |v| enc_str(v)), encode_option(&d.shapes, enc_shapes_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layout_diff(s: &str) -> Result<SlideLayoutDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [master_id, shapes] = parts.as_slice() else { return Err(format!("layout diff: expected 2 fields, got {}", parts.len())) };
    Ok(SlideLayoutDiff { master_id: decode_option(master_id, dec_str)?, shapes: decode_option(shapes, dec_shapes_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_slide_diff(d: &SlideDiff) -> String {
    format!("[{},{},{},{}]", encode_option(&d.id, |v| enc_str(v)), encode_option(&d.layout_id, |inner: &Option<String>| encode_option(inner, |v| enc_str(v))), encode_option(&d.shapes, enc_shapes_diff), encode_option(&d.notes, enc_doc_blocks_diff))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_slide_diff(s: &str) -> Result<SlideDiff, String> {
    let inner = strip_brackets(s)?;
    let parts = split_top_level(inner, ',');
    let [id, layout_id, shapes, notes] = parts.as_slice() else { return Err(format!("slide diff: expected 4 fields, got {}", parts.len())) };
    Ok(SlideDiff { id: decode_option(id, dec_str)?, layout_id: decode_option(layout_id, |s| decode_option(s, dec_str))?, shapes: decode_option(shapes, dec_shapes_diff)?, notes: decode_option(notes, dec_doc_blocks_diff)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_masters_diff(d: &SlideMastersDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_master_diff, |a| enc_named_added(a, enc_master))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_masters_diff(s: &str) -> Result<SlideMastersDiff, String> {
    dec_named_triple(s, dec_str, dec_master_diff, |t| dec_named_added(t, dec_master))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_layouts_diff(d: &SlideLayoutsDiff) -> String {
    enc_named_triple(d, |k| enc_str(k), enc_layout_diff, |a| enc_named_added(a, enc_layout))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_layouts_diff(s: &str) -> Result<SlideLayoutsDiff, String> {
    dec_named_triple(s, dec_str, dec_layout_diff, |t| dec_named_added(t, dec_layout))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_slides_diff(d: &SlidesDiff) -> String {
    enc_indexed_triple(d, enc_slide_diff, enc_slide)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_slides_diff(s: &str) -> Result<SlidesDiff, String> {
    dec_indexed_triple(s, dec_slide_diff, dec_slide)
}
}
pub use diff_codec::*;
