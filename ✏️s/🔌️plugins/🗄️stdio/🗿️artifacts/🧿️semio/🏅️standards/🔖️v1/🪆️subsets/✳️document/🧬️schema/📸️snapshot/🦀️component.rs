//! 🧬️ SemioDocumentSnapshot — complete-per-spec block tree (Paragraph/Heading/List/Table/Code/
//! Quote/Image/PageBreak) + named styles + id-keyed images, informed by docx's body block tree
//! and md's `MdBlock`/`MdInline`; replaces `PageDoc`/`TextDoc`. Reused by `presentation`'s
//! `SlideShape::TextBox`, which embeds `DocBlock` directly (spec-mandated cross-reuse, see
//! `w1b-type-ownership.md`) — `DocBlock`/`DocRun`/`DocStyle` are this subset's owned types.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{split_top_level, strip_brackets};
use crate::artifacts::semio::standards::v1::subsets::document::schema::diff::{dec_block, dec_image, dec_str, dec_style, enc_block, enc_image, enc_str, enc_style};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentModel
/// 🎨️ Character-level formatting for one `DocRun`. Named struct (never a bare tuple) per the f6
/// §4.3 `DslField`-for-tuples gap this schema style avoids everywhere.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStyle {
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub font: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
}

/// ✍️ One inline run of literal text plus its formatting.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocRun {
    pub text: String,
    #[serde(default)]
    pub style: RunStyle,
}

impl DocRun {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn plain(text: impl Into<String>) -> Self {
        Self { text: text.into(), style: RunStyle::default() }
    }
}

/// 🎨️ One named paragraph/character style (docx `w:style`-shaped: id, display name, optional
/// parent for inheritance chains).
/// 🩹 Derives `Default` (empty id/name, no parent) so `DocStyle` satisfies the shared
/// `engine::triples::NamedTripleDiff<K,D,T>`'s conservative `T: Default` bound (a serde-derive
/// limitation identical to the one docx's OWN local `NamedTripleDiff` copy works around via an
/// explicit `#[serde(bound(...))]` override — the shared `engine::triples` copy lacks that
/// override; per this ticket's "shared infra gaps → report only" rule, fixed here locally rather
/// than editing that shared file).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocStyle {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub based_on: Option<String>,
}

/// 🖼️ One embedded raster/vector image, addressed by id from `DocBlock::Image`. Derives
/// `Default` for the same shared-`engine::triples`-bound reason as `DocStyle` above.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocImage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub mime: String,
    #[serde(default)]
    pub bytes: Vec<u8>,
}

/// 🔲 One list item — recursively holds its own block content (a list item may itself contain
/// paragraphs, nested lists, tables, …), matching CommonMark/WordprocessingML's own model.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocListItem {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

/// 🔲️ One table cell — recursively holds block content.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableCell {
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

/// ➖️ One table row.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocTableRow {
    #[serde(default)]
    pub cells: Vec<DocTableCell>,
}

/// 🧱️ One block-level content item — the recursive tree shape the master plan's snapshot spec
/// names: Paragraph/Heading/List/Table/Code/Quote/Image/PageBreak. `List`/`Table`/`Quote` nest
/// `DocBlock` recursively (list items, table cells, blockquote body), the same recursive-diff
/// shape svg's `SvgNodeDiff` and docx's `DocxBlock::Table` establish.
/// 🩹 Derives `Default` (`#[default]` on the fieldless `PageBreak` variant) for the same shared
/// `engine::triples::IndexedTripleDiff<D,T>` bound reason `DocStyle` documents above — `DocBlock`
/// is used as `T` in `BlocksDiff = IndexedTripleDiff<DocBlockDiff, DocBlock>`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DocBlock {
    Paragraph {
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    Heading {
        level: u8,
        #[serde(default)]
        style_id: Option<String>,
        #[serde(default)]
        runs: Vec<DocRun>,
    },
    List {
        #[serde(default)]
        ordered: bool,
        #[serde(default)]
        items: Vec<DocListItem>,
    },
    Table {
        #[serde(default)]
        rows: Vec<DocTableRow>,
    },
    Code {
        #[serde(default)]
        language: Option<String>,
        #[serde(default)]
        text: String,
    },
    Quote {
        #[serde(default)]
        blocks: Vec<DocBlock>,
    },
    Image {
        image_id: String,
        #[serde(default)]
        alt: String,
        #[serde(default)]
        width: Option<f64>,
        #[serde(default)]
        height: Option<f64>,
    },
    #[default]
    PageBreak,
}

impl DocBlock {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn paragraph(text: impl Into<String>) -> Self {
        Self::Paragraph { style_id: None, runs: vec![DocRun::plain(text)] }
    }
}
//#endregion 🔖️DocumentModel

//#region 🔖️Ids
pub const STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA: &str = "s.stdio.semio.document";
//#endregion 🔖️Ids

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.document")]
pub struct SemioDocumentSnapshot {
    #[state(artifact)]
    pub schema: String,
    /// 🎨️ Named styles, keyed by `DocStyle::id`.
    #[state(artifact)]
    #[serde(default)]
    pub styles: Vec<DocStyle>,
    /// 🖼️ Embedded images, keyed by `DocImage::id`, referenced from `DocBlock::Image::image_id`.
    #[state(artifact)]
    #[serde(default)]
    pub images: Vec<DocImage>,
    /// 🧱️ The top-level block tree.
    #[state(artifact)]
    #[serde(default)]
    pub blocks: Vec<DocBlock>,
}

impl Default for SemioDocumentSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(), styles: Default::default(), images: Default::default(), blocks: Default::default() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️TextCodec
/// 🧪️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real structured
/// text body — `schema=<hex>` / `styles=[<style>,...]` / `images=[<image>,...]` /
/// `blocks=[<block>,...]`, one line per top-level field, matching the flow/model/brep pilots'
/// own `print_*_snapshot_body` shape. Reuses `🔺️diff`'s ALREADY-real, already-tested
/// `enc_str`/`enc_style`/`enc_image`/`enc_block` value codecs (established there pre-wave) rather
/// than duplicating a third independent copy — this subset's own established convention (its
/// `🧬️mutations` facet already does the same). Replaces the old hex-of-`serde_json` passthrough.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_document_snapshot_body(s: &SemioDocumentSnapshot) -> String {
    format!(
        "schema={}\nstyles=[{}]\nimages=[{}]\nblocks=[{}]",
        enc_str(&s.schema),
        s.styles.iter().map(enc_style).collect::<Vec<_>>().join(","),
        s.images.iter().map(enc_image).collect::<Vec<_>>().join(","),
        s.blocks.iter().map(enc_block).collect::<Vec<_>>().join(",")
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_document_snapshot_body(body: &str) -> Result<SemioDocumentSnapshot, String> {
    let mut schema = None;
    let mut styles = Vec::new();
    let mut images = Vec::new();
    let mut blocks = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("schema=") {
            schema = Some(dec_str(rest)?);
        } else if let Some(rest) = line.strip_prefix("styles=") {
            let inner = strip_brackets(rest)?;
            styles = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_style).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("images=") {
            let inner = strip_brackets(rest)?;
            images = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_image).collect::<Result<Vec<_>, String>>()?;
        } else if let Some(rest) = line.strip_prefix("blocks=") {
            let inner = strip_brackets(rest)?;
            blocks = split_top_level(inner, ',').into_iter().filter(|s| !s.is_empty()).map(dec_block).collect::<Result<Vec<_>, String>>()?;
        } else {
            return Err(format!("document snapshot: unknown line {line:?}"));
        }
    }
    let schema = schema.ok_or_else(|| "document snapshot: missing schema line".to_string())?;
    Ok(SemioDocumentSnapshot { schema, styles, images, blocks })
}
//#endregion 🔖️TextCodec

//#region 🔖️BinaryCodec
/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`, same helpers flow/model/brep's own upgraded `ArtifactPack`s use)
/// backing `encode_document_snapshot_binary`/`decode_document_snapshot_binary` below.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 1 } else { 0 });
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_bool(reader: &mut store::ByteReader<'_>) -> Result<bool, String> {
    Ok(reader.read_u8().map_err(|e| e.to_string())? != 0)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_opt_str(out: &mut Vec<u8>, v: &Option<String>) {
    match v {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            write_str_lp(out, s);
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_opt_str(reader: &mut store::ByteReader<'_>) -> Result<Option<String>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(read_str_lp(reader)?)),
        other => Err(format!("opt str: bad tag {other}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_opt_f64(out: &mut Vec<u8>, v: Option<f64>) {
    match v {
        None => out.push(0),
        Some(f) => {
            out.push(1);
            out.extend_from_slice(&f.to_le_bytes());
        }
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_opt_f64(reader: &mut store::ByteReader<'_>) -> Result<Option<f64>, String> {
    match reader.read_u8().map_err(|e| e.to_string())? {
        0 => Ok(None),
        1 => Ok(Some(reader.read_f64_le().map_err(|e| e.to_string())?)),
        other => Err(format!("opt f64: bad tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_run_style(out: &mut Vec<u8>, s: &RunStyle) {
    write_bool(out, s.bold);
    write_bool(out, s.italic);
    write_bool(out, s.underline);
    write_opt_f64(out, s.size);
    write_opt_str(out, &s.font);
    write_opt_str(out, &s.color);
    write_opt_str(out, &s.link);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_run_style(reader: &mut store::ByteReader<'_>) -> Result<RunStyle, String> {
    Ok(RunStyle { bold: read_bool(reader)?, italic: read_bool(reader)?, underline: read_bool(reader)?, size: read_opt_f64(reader)?, font: read_opt_str(reader)?, color: read_opt_str(reader)?, link: read_opt_str(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_run(out: &mut Vec<u8>, r: &DocRun) {
    write_str_lp(out, &r.text);
    write_run_style(out, &r.style);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_run(reader: &mut store::ByteReader<'_>) -> Result<DocRun, String> {
    Ok(DocRun { text: read_str_lp(reader)?, style: read_run_style(reader)? })
}

/// 🌳️ Real per-variant tag byte (0=Paragraph 1=Heading 2=List 3=Table 4=Code 5=Quote 6=Image
/// 7=PageBreak) + fields, genuinely recursive for `List`/`Table`/`Quote`'s nested `Vec<DocBlock>`
/// (`protocol-prim-ref-recursion`/`protocol-array-of-records` — the Rust side stays fully
/// structured; only the `.protocol.semio` DESCRIPTION stops at an opaque tail, per the recipe).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_block(out: &mut Vec<u8>, b: &DocBlock) {
    match b {
        DocBlock::Paragraph { style_id, runs } => {
            out.push(0);
            write_opt_str(out, style_id);
            store::pack_rt::write_varint_u64(out, runs.len() as u64);
            for r in runs {
                write_run(out, r);
            }
        }
        DocBlock::Heading { level, style_id, runs } => {
            out.push(1);
            out.push(*level);
            write_opt_str(out, style_id);
            store::pack_rt::write_varint_u64(out, runs.len() as u64);
            for r in runs {
                write_run(out, r);
            }
        }
        DocBlock::List { ordered, items } => {
            out.push(2);
            write_bool(out, *ordered);
            store::pack_rt::write_varint_u64(out, items.len() as u64);
            for item in items {
                store::pack_rt::write_varint_u64(out, item.blocks.len() as u64);
                for b in &item.blocks {
                    write_block(out, b);
                }
            }
        }
        DocBlock::Table { rows } => {
            out.push(3);
            store::pack_rt::write_varint_u64(out, rows.len() as u64);
            for row in rows {
                store::pack_rt::write_varint_u64(out, row.cells.len() as u64);
                for cell in &row.cells {
                    store::pack_rt::write_varint_u64(out, cell.blocks.len() as u64);
                    for b in &cell.blocks {
                        write_block(out, b);
                    }
                }
            }
        }
        DocBlock::Code { language, text } => {
            out.push(4);
            write_opt_str(out, language);
            write_str_lp(out, text);
        }
        DocBlock::Quote { blocks } => {
            out.push(5);
            store::pack_rt::write_varint_u64(out, blocks.len() as u64);
            for b in blocks {
                write_block(out, b);
            }
        }
        DocBlock::Image { image_id, alt, width, height } => {
            out.push(6);
            write_str_lp(out, image_id);
            write_str_lp(out, alt);
            write_opt_f64(out, *width);
            write_opt_f64(out, *height);
        }
        DocBlock::PageBreak => out.push(7),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_block(reader: &mut store::ByteReader<'_>) -> Result<DocBlock, String> {
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let style_id = read_opt_str(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut runs = Vec::with_capacity(n as usize);
            for _ in 0..n {
                runs.push(read_run(reader)?);
            }
            Ok(DocBlock::Paragraph { style_id, runs })
        }
        1 => {
            let level = reader.read_u8().map_err(|e| e.to_string())?;
            let style_id = read_opt_str(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut runs = Vec::with_capacity(n as usize);
            for _ in 0..n {
                runs.push(read_run(reader)?);
            }
            Ok(DocBlock::Heading { level, style_id, runs })
        }
        2 => {
            let ordered = read_bool(reader)?;
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut items = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let bn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                let mut blocks = Vec::with_capacity(bn as usize);
                for _ in 0..bn {
                    blocks.push(read_block(reader)?);
                }
                items.push(DocListItem { blocks });
            }
            Ok(DocBlock::List { ordered, items })
        }
        3 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut rows = Vec::with_capacity(n as usize);
            for _ in 0..n {
                let cn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                let mut cells = Vec::with_capacity(cn as usize);
                for _ in 0..cn {
                    let bn = reader.read_varint_u64().map_err(|e| e.to_string())?;
                    let mut blocks = Vec::with_capacity(bn as usize);
                    for _ in 0..bn {
                        blocks.push(read_block(reader)?);
                    }
                    cells.push(DocTableCell { blocks });
                }
                rows.push(DocTableRow { cells });
            }
            Ok(DocBlock::Table { rows })
        }
        4 => {
            let language = read_opt_str(reader)?;
            let text = read_str_lp(reader)?;
            Ok(DocBlock::Code { language, text })
        }
        5 => {
            let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
            let mut blocks = Vec::with_capacity(n as usize);
            for _ in 0..n {
                blocks.push(read_block(reader)?);
            }
            Ok(DocBlock::Quote { blocks })
        }
        6 => {
            let image_id = read_str_lp(reader)?;
            let alt = read_str_lp(reader)?;
            let width = read_opt_f64(reader)?;
            let height = read_opt_f64(reader)?;
            Ok(DocBlock::Image { image_id, alt, width, height })
        }
        7 => Ok(DocBlock::PageBreak),
        other => Err(format!("block: unknown tag {other}")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_style(out: &mut Vec<u8>, s: &DocStyle) {
    write_str_lp(out, &s.id);
    write_str_lp(out, &s.name);
    write_opt_str(out, &s.based_on);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_style(reader: &mut store::ByteReader<'_>) -> Result<DocStyle, String> {
    Ok(DocStyle { id: read_str_lp(reader)?, name: read_str_lp(reader)?, based_on: read_opt_str(reader)? })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn write_image(out: &mut Vec<u8>, i: &DocImage) {
    write_str_lp(out, &i.id);
    write_str_lp(out, &i.mime);
    write_bytes_lp(out, &i.bytes);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_image(reader: &mut store::ByteReader<'_>) -> Result<DocImage, String> {
    Ok(DocImage { id: read_str_lp(reader)?, mime: read_str_lp(reader)?, bytes: read_bytes_lp(reader)? })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn encode_document_snapshot_binary(s: &SemioDocumentSnapshot) -> Vec<u8> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut out = Vec::new();
    out.push(PACK_BINARY_FORMAT);
    write_str_lp(&mut out, &s.schema);
    store::pack_rt::write_varint_u64(&mut out, s.styles.len() as u64);
    for st in &s.styles {
        write_style(&mut out, st);
    }
    store::pack_rt::write_varint_u64(&mut out, s.images.len() as u64);
    for im in &s.images {
        write_image(&mut out, im);
    }
    store::pack_rt::write_varint_u64(&mut out, s.blocks.len() as u64);
    for b in &s.blocks {
        write_block(&mut out, b);
    }
    out
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn decode_document_snapshot_binary(bytes: &[u8]) -> Result<SemioDocumentSnapshot, String> {
    const PACK_BINARY_FORMAT: u8 = 1;
    let mut reader = store::ByteReader::new(bytes);
    let format = reader.read_u8().map_err(|e| e.to_string())?;
    if format != PACK_BINARY_FORMAT {
        return Err(format!("unsupported pack format {format}"));
    }
    let schema = read_str_lp(&mut reader)?;
    let style_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut styles = Vec::with_capacity(style_count as usize);
    for _ in 0..style_count {
        styles.push(read_style(&mut reader)?);
    }
    let image_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut images = Vec::with_capacity(image_count as usize);
    for _ in 0..image_count {
        images.push(read_image(&mut reader)?);
    }
    let block_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut blocks = Vec::with_capacity(block_count as usize);
    for _ in 0..block_count {
        blocks.push(read_block(&mut reader)?);
    }
    Ok(SemioDocumentSnapshot { schema, styles, images, blocks })
}
//#endregion 🔖️BinaryCodec

//#region 🔖️HandcraftedArtifactCodecs
/// 🧩️ ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION document wave: real structured
/// text/binary codecs, replacing the old hex-of-`serde_json` passthrough (both `parse_dsl`'s and
/// `encode_pack_with`'s old bodies called straight into `serde_json::{to_vec,from_slice}`). The
/// derive path (`#[derive(dsl::DslArtifact)]`) was tried first per the ticket's brief and hits the
/// same wall every hand-rolled-tagged-enum semio subset (model/brep) already hit: `DocBlock` is a
/// `#[serde(tag = "kind")]` data-carrying enum with heterogeneous per-variant field sets — no
/// `dsl::DslField`/`DslEnum` impl exists for it, and `IndexedTripleDiff<DocBlockDiff, DocBlock>`
/// compounds the gap. Hand-rolled instead, matching `🔺️diff`'s already-hand-rolled convention.
impl store::ArtifactDsl for SemioDocumentSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str {
        STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_document_snapshot_body(body).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let body = print_document_snapshot_body(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioDocumentSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = encode_document_snapshot_binary(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let _ = options;
        decode_document_snapshot_binary(&inner).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📥️ Parses this subset's own committed `.dsl.semio` text into a real [`SemioDocumentSnapshot`] — a thin
/// wrapper over `store::ArtifactDsl::parse_dsl` so external Rust callers that cannot name this
/// crate's private `store` extern-crate item (the `mutate-semio-document` test adapter, which reads the
/// REAL committed example artifact rather than a hand-transcribed Rust literal of it) can still
/// drive the same codec production does. Same rationale as `✳️kit`'s `decode_kit_snapshot_json`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_semio_document_dsl(text: &str) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactDsl::print_dsl` inverse of [`parse_semio_document_dsl`] — same rationale.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn print_semio_document_dsl(snapshot: &SemioDocumentSnapshot) -> String {
    <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioDocumentSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_pack(bytes: &[u8]) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}

/// 📤️ The `store::ArtifactPack::encode_pack` inverse of [`decode_semio_document_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_document_pack(snapshot: &SemioDocumentSnapshot) -> Vec<u8> {
    <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(snapshot)
}

/// 📤️ This subset's own `#[serde(rename_all = "camelCase")]` structural JSON projection of
/// `s.stdio.semio.document` — the shape the `mutate-semio-document` case compares under `ordered-json-v1`. A thin
/// `serde_json` wrapper (already a direct dependency of this crate, used behind this interface per
/// CLAUDE.md's "external libraries behind an interface" rule, never a new one), so a projection is
/// derived from the snapshot type itself rather than hand-written a second time in the adapter,
/// where it could drift.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_document_snapshot_json(snapshot: &SemioDocumentSnapshot) -> String {
    serde_json::to_string(snapshot).expect("SemioDocumentSnapshot serialization is infallible")
}

/// 📥️ The `serde_json` inverse of [`encode_semio_document_snapshot_json`] — decodes the
/// `before`/`after` halves of `mutate-semio-document`'s committed specification vectors
/// (`../../../../../🧪️tests/mutate-semio-document/🧫️fixtures/🦠️<kind>.json`) into real [`SemioDocumentSnapshot`]
/// values, so the adapter never hand-transcribes a fixture into a Rust literal that could silently
/// drift away from the JSON it claims to mirror.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_snapshot_json(text: &str) -> Result<SemioDocumentSnapshot, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge


//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.document` snapshot — one style, one image, and one block of every
/// kind (Heading/Paragraph/List/Table/Code/Quote/Image/PageBreak), exercising every leaf shape at
/// least once. Single source of truth for `📚️examples/📄️memo/🖼️assets/🗣️example.dsl.semio`/
/// `🎒️example.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️component.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_semio_document_snapshot() -> SemioDocumentSnapshot {
    SemioDocumentSnapshot {
        schema: STDIO_SEMIODOCUMENT_DOCUMENT_SCHEMA.into(),
        styles: vec![DocStyle { id: "heading1".into(), name: "Heading 1".into(), based_on: Some("normal".into()) }],
        images: vec![DocImage { id: "img1".into(), mime: "image/png".into(), bytes: vec![1, 2, 3] }],
        blocks: vec![
            DocBlock::Heading { level: 1, style_id: Some("heading1".into()), runs: vec![DocRun { text: "Title".into(), style: RunStyle { bold: true, ..Default::default() } }] },
            DocBlock::Paragraph { style_id: None, runs: vec![DocRun::plain("Body")] },
            DocBlock::List { ordered: true, items: vec![DocListItem { blocks: vec![DocBlock::paragraph("item one")] }] },
            DocBlock::Table { rows: vec![DocTableRow { cells: vec![DocTableCell { blocks: vec![DocBlock::paragraph("cell")] }] }] },
            DocBlock::Code { language: Some("rust".into()), text: "fn main() {}".into() },
            DocBlock::Quote { blocks: vec![DocBlock::paragraph("quoted")] },
            DocBlock::Image { image_id: "img1".into(), alt: "alt text".into(), width: Some(100.0), height: Some(50.0) },
            DocBlock::PageBreak,
        ],
    }
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn json_pack_round_trips() {
        let snap = demo_semio_document_snapshot();
        let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn dsl_text_round_trips() {
        let snap = demo_semio_document_snapshot();
        let text = <SemioDocumentSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back = <SemioDocumentSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back);
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_snapshot_binary_round_trips() {
        let snap = SemioDocumentSnapshot::default();
        let bytes = <SemioDocumentSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioDocumentSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
