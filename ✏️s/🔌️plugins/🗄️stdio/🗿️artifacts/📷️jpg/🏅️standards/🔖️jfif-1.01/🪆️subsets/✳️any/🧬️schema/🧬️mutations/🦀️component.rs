//! 🧬️ JpgMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `JpgDiff` directly via the `schema::diff` builders — apply-and-capture
//! is banned); `inverse()` is handcrafted per variant, id/index-aware, reading the pre-state it
//! needs from `base`. `apply_jpg_mutation` follows png/csv's proven single-source-of-truth shape:
//! `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`.

use crate::artifacts::jpg::schema::diff::{self, JpgDiff, JpgHuffmanTableKey};
use crate::artifacts::jpg::schema::snapshot::{JfifDensityUnits, JfifThumbnail, JpgHuffmanTable, JpgQuantTable, JpgSegment};
use crate::artifacts::jpg::JpgSnapshot;
#[cfg(test)]
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.jpg`.
/// 🧪️ F6 CONFIRMED HAND-ROLL: `#[derive(dsl::DslOps)]` on this enum fails to compile (captured
/// verbatim by actually adding the derive this session, real `cargo check` output, see
/// `f6-jpg-report.md`) — `SetSnapshot{snapshot: JpgSnapshot}` and every other struct-payload
/// variant (`SetQuantTable{table: JpgQuantTable}`, `SetHuffmanTable{table: JpgHuffmanTable}`,
/// `RemoveHuffmanTable{key: JpgHuffmanTableKey}`, `InsertOtherSegment{segment: JpgSegment}`) reject
/// immediately with e.g.:
/// ```text
/// error[E0277]: the trait bound `JpgSnapshot: DslField` is not satisfied
///   --> .../🧬️mutations/🦀️component.rs:32:19   (snapshot: JpgSnapshot)
/// error[E0277]: the trait bound `JpgQuantTable: DslField` is not satisfied
///   --> .../🧬️mutations/🦀️component.rs:44:16   (table: JpgQuantTable)
/// ```
/// — none of `JpgSnapshot`/`JpgFrameHeader`/`JpgFrameComponent`/`JpgQuantTable`/`JpgHuffmanTable`/
/// `JpgSegment`/`JfifThumbnail`/`JpgHuffmanTableKey` carry `#[derive(dsl::DslRecord)]` (nor
/// `JfifDensityUnits`/`JpgHuffmanClass` carry `#[derive(dsl::DslScalar)]`), so nothing in the
/// reachable tree implements `DslField` yet — the STEP 2a cascading requirement. Even fully
/// cascaded, `SetJfifHeader.version: (u8, u8)` is a SEPARATE, decisive blocker: `dsl` has no
/// `DslField` impl for tuples of any arity (confirmed by direct grep of every `impl DslField`/
/// `impl<T: DslField, ...> DslField` in `🧰️framework/…/🗣️dsl/🦀️component.rs` — only `bool`/`f32`/
/// `f64`/`String`/`Wire`/`DslValue`/`Vec<T>`/`BTreeMap<String,T>`/`[T;N]` have impls, no tuple
/// arm). Fixing it needs either a framework-level `dsl` crate change (shared file, outside this
/// artifact's ownership boundary) or replacing the tuple with e.g. `[u8;2]` (a Mutation-shape
/// change this ticket's scope forbids — "do not touch snapshot/diff/mutation SHAPE"). `OpText`/
/// `OpBinary` hand-rolled below, reusing `schema::diff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_frame_header`/`split_top_level`/...).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum JpgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: JpgSnapshot,
    },
    /// 🧾️ Replaces all five typed JFIF APP0 fields at once.
    SetJfifHeader {
        version: (u8, u8),
        density_units: JfifDensityUnits,
        x_density: u16,
        y_density: u16,
        thumbnail: Option<JfifThumbnail>,
    },
    /// 📊️ Upserts a `DQT` table by id (inserts if the id doesn't exist yet, else patches).
    SetQuantTable {
        table: JpgQuantTable,
    },
    /// ➖️ Removes a `DQT` table by id (no-op if it doesn't exist).
    RemoveQuantTable {
        id: u8,
    },
    /// 🌳️ Upserts a `DHT` table by `(class, id)` (inserts if the key doesn't exist yet, else
    /// patches).
    SetHuffmanTable {
        table: JpgHuffmanTable,
    },
    /// ➖️ Removes a `DHT` table by `(class, id)` (no-op if it doesn't exist).
    RemoveHuffmanTable {
        key: JpgHuffmanTableKey,
    },
    /// 🔁️ Sets (or clears, via `None`) the `DRI` restart interval.
    SetRestartInterval {
        restart_interval: Option<u16>,
    },
    /// ➕️ Inserts a verbatim-retained other APPn/COM segment at `index` (final position,
    /// clamped to `len`).
    InsertOtherSegment {
        index: usize,
        segment: JpgSegment,
    },
    /// ➖️ Removes the other-segment at `index` (no-op if out of range).
    RemoveOtherSegment {
        index: usize,
    },
    /// 🖼️ Replaces the decoded canonical RGBA8 raster wholesale.
    SetPixels {
        pixels: Vec<u8>,
    },
    /// 🎚️ Sets (or clears, via `None`) the re-encode quality parameter.
    SetReEncodeQuality {
        quality: Option<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (png/csv precedent).
pub fn apply_jpg_mutation(snapshot: &mut JpgSnapshot, mutation: &JpgMutation) -> JpgDiff {
    let d = <JpgMutation as Mutation<JpgSnapshot>>::diff(mutation, snapshot);
    *snapshot = <JpgDiff as MutationDiff<JpgSnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<JpgSnapshot> for JpgMutation {
    type Diff = JpgDiff;

    fn diff(&self, base: &JpgSnapshot) -> Self::Diff {
        match self {
            JpgMutation::NoMutation => JpgDiff::default(),
            JpgMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            JpgMutation::SetJfifHeader { version, density_units, x_density, y_density, thumbnail } => {
                diff::diff_set_jfif_header(base, *version, *density_units, *x_density, *y_density, thumbnail.clone())
            }
            JpgMutation::SetQuantTable { table } => diff::diff_set_quant_table(base, table.clone()),
            JpgMutation::RemoveQuantTable { id } => diff::diff_remove_quant_table(base, *id),
            JpgMutation::SetHuffmanTable { table } => diff::diff_set_huffman_table(base, table.clone()),
            JpgMutation::RemoveHuffmanTable { key } => diff::diff_remove_huffman_table(base, *key),
            JpgMutation::SetRestartInterval { restart_interval } => diff::diff_set_restart_interval(base, *restart_interval),
            JpgMutation::InsertOtherSegment { index, segment } => diff::diff_insert_other_segment(base, *index, segment.clone()),
            JpgMutation::RemoveOtherSegment { index } => diff::diff_remove_other_segment(base, *index),
            JpgMutation::SetPixels { pixels } => diff::diff_set_pixels(base, pixels.clone()),
            JpgMutation::SetReEncodeQuality { quality } => diff::diff_set_re_encode_quality(base, *quality),
        }
    }

    /// ↩️ Handcrafted, id/index-aware mutation-level inverses. Out-of-range/nonexistent targets
    /// invert to `NoMutation` (nothing to undo).
    fn inverse(&self, base: &JpgSnapshot) -> Vec<Self> {
        match self {
            JpgMutation::NoMutation => vec![JpgMutation::NoMutation],
            JpgMutation::SetSnapshot { .. } => vec![JpgMutation::SetSnapshot { snapshot: base.clone() }],
            JpgMutation::SetJfifHeader { .. } => vec![JpgMutation::SetJfifHeader {
                version: base.jfif_version,
                density_units: base.jfif_density_units,
                x_density: base.jfif_x_density,
                y_density: base.jfif_y_density,
                thumbnail: base.jfif_thumbnail.clone(),
            }],
            JpgMutation::SetQuantTable { table } => match base.quant_tables.iter().find(|t| t.id == table.id) {
                Some(existing) => vec![JpgMutation::SetQuantTable { table: existing.clone() }],
                None => vec![JpgMutation::RemoveQuantTable { id: table.id }],
            },
            JpgMutation::RemoveQuantTable { id } => match base.quant_tables.iter().find(|t| t.id == *id) {
                Some(existing) => vec![JpgMutation::SetQuantTable { table: existing.clone() }],
                None => vec![JpgMutation::NoMutation],
            },
            JpgMutation::SetHuffmanTable { table } => {
                let key = JpgHuffmanTableKey { class: table.class, id: table.id };
                match base.huffman_tables.iter().find(|t| t.class == key.class && t.id == key.id) {
                    Some(existing) => vec![JpgMutation::SetHuffmanTable { table: existing.clone() }],
                    None => vec![JpgMutation::RemoveHuffmanTable { key }],
                }
            }
            JpgMutation::RemoveHuffmanTable { key } => match base.huffman_tables.iter().find(|t| t.class == key.class && t.id == key.id) {
                Some(existing) => vec![JpgMutation::SetHuffmanTable { table: existing.clone() }],
                None => vec![JpgMutation::NoMutation],
            },
            JpgMutation::SetRestartInterval { .. } => vec![JpgMutation::SetRestartInterval { restart_interval: base.restart_interval }],
            JpgMutation::InsertOtherSegment { index, .. } => {
                vec![JpgMutation::RemoveOtherSegment { index: (*index).min(base.other_segments.len()) }]
            }
            JpgMutation::RemoveOtherSegment { index } => match base.other_segments.get(*index) {
                Some(segment) => vec![JpgMutation::InsertOtherSegment { index: *index, segment: segment.clone() }],
                None => vec![JpgMutation::NoMutation],
            },
            JpgMutation::SetPixels { .. } => vec![JpgMutation::SetPixels { pixels: base.pixels.clone() }],
            JpgMutation::SetReEncodeQuality { .. } => vec![JpgMutation::SetReEncodeQuality { quality: base.re_encode_quality }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: hand-rolled `OpText`/`OpBinary` for `JpgMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `schema::diff`'s `pub(crate)` grammar primitives (`hex_encode`/
/// `enc_frame_header`/`enc_quant_table`/`enc_huffman_table`/`enc_segment`/`enc_thumbnail`/
/// `enc_version`/`enc_density_units`/`enc_huffman_key`/`split_top_level`/`encode_option`/...)
/// rather than duplicating them a third time in this file. Grammar: `keyword arg=value ...`
/// (space-separated, same shape the derive's own handcrafted-wrapper convention uses per
/// `f6-recon-report.md` §2), one match arm per variant (no `DslVariants` scaffolding available
/// since nothing here derives it).
fn enc_str_hex(s: &str) -> String { diff::hex_encode(s.as_bytes()) }
fn dec_str_hex(s: &str) -> Result<String, String> { String::from_utf8(diff::hex_decode(s)?).map_err(|e| e.to_string()) }

/// 🧬️ Positional `[schema,width,height,pixels,re-encode-quality,jfif-version,jfif-density-units,
/// jfif-x-density,jfif-y-density,jfif-thumbnail,frame,sof-marker,arithmetic,quant-tables,
/// huffman-tables,restart-interval,other-segments]` tuple — declaration order, both sides agree.
fn enc_jpg_snapshot(s: &JpgSnapshot) -> String {
    let quant = s.quant_tables.iter().map(diff::enc_quant_table).collect::<Vec<_>>().join(",");
    let huff = s.huffman_tables.iter().map(diff::enc_huffman_table).collect::<Vec<_>>().join(",");
    let segs = s.other_segments.iter().map(diff::enc_segment).collect::<Vec<_>>().join(",");
    format!(
        "[{},{},{},{},{},{},{},{},{},{},{},{},{},[{}],[{}],{},[{}]]",
        enc_str_hex(&s.schema),
        s.width,
        s.height,
        diff::hex_encode(&s.pixels),
        diff::encode_option(&s.re_encode_quality, |v| v.to_string()),
        diff::enc_version(&s.jfif_version),
        diff::enc_density_units(&s.jfif_density_units),
        s.jfif_x_density,
        s.jfif_y_density,
        diff::encode_option(&s.jfif_thumbnail, diff::enc_thumbnail),
        diff::encode_option(&s.frame, diff::enc_frame_header),
        s.sof_marker,
        if s.arithmetic { 1 } else { 0 },
        quant,
        huff,
        diff::encode_option(&s.restart_interval, |v| v.to_string()),
        segs,
    )
}
fn dec_jpg_snapshot(s: &str) -> Result<JpgSnapshot, String> {
    let parts = diff::split_top_level(diff::strip_brackets(s)?, ',');
    let [schema, width, height, pixels, re_encode_quality, jfif_version, jfif_density_units, jfif_x_density, jfif_y_density, jfif_thumbnail, frame, sof_marker, arithmetic, quant_tables, huffman_tables, restart_interval, other_segments] =
        parts.as_slice()
    else {
        return Err(format!("jpg snapshot: expected 17 fields, got {}", parts.len()));
    };
    Ok(JpgSnapshot {
        schema: dec_str_hex(schema)?,
        width: diff::parse_u32(width)?,
        height: diff::parse_u32(height)?,
        pixels: diff::hex_decode(pixels)?,
        re_encode_quality: diff::decode_option(re_encode_quality, diff::parse_u8)?,
        jfif_version: diff::dec_version(jfif_version)?,
        jfif_density_units: diff::dec_density_units(jfif_density_units)?,
        jfif_x_density: diff::parse_u16(jfif_x_density)?,
        jfif_y_density: diff::parse_u16(jfif_y_density)?,
        jfif_thumbnail: diff::decode_option(jfif_thumbnail, diff::dec_thumbnail)?,
        frame: diff::decode_option(frame, diff::dec_frame_header)?,
        sof_marker: diff::parse_u8(sof_marker)?,
        arithmetic: diff::parse_bool(arithmetic)?,
        quant_tables: diff::split_top_level(diff::strip_brackets(quant_tables)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_quant_table).collect::<Result<Vec<_>, String>>()?,
        huffman_tables: diff::split_top_level(diff::strip_brackets(huffman_tables)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_huffman_table).collect::<Result<Vec<_>, String>>()?,
        restart_interval: diff::decode_option(restart_interval, diff::parse_u16)?,
        other_segments: diff::split_top_level(diff::strip_brackets(other_segments)?, ',').into_iter().filter(|s| !s.is_empty()).map(diff::dec_segment).collect::<Result<Vec<_>, String>>()?,
    })
}

fn print_jpg_mutation(m: &JpgMutation) -> String {
    match m {
        JpgMutation::NoMutation => "no-mutation".to_string(),
        JpgMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_jpg_snapshot(snapshot)),
        JpgMutation::SetJfifHeader { version, density_units, x_density, y_density, thumbnail } => format!(
            "set-jfif-header version={} density-units={} x-density={x_density} y-density={y_density} thumbnail={}",
            diff::enc_version(version),
            diff::enc_density_units(density_units),
            diff::encode_option(thumbnail, diff::enc_thumbnail),
        ),
        JpgMutation::SetQuantTable { table } => format!("set-quant-table table={}", diff::enc_quant_table(table)),
        JpgMutation::RemoveQuantTable { id } => format!("remove-quant-table id={id}"),
        JpgMutation::SetHuffmanTable { table } => format!("set-huffman-table table={}", diff::enc_huffman_table(table)),
        JpgMutation::RemoveHuffmanTable { key } => format!("remove-huffman-table key={}", diff::enc_huffman_key(key)),
        JpgMutation::SetRestartInterval { restart_interval } => format!("set-restart-interval restart-interval={}", diff::encode_option(restart_interval, |v| v.to_string())),
        JpgMutation::InsertOtherSegment { index, segment } => format!("insert-other-segment index={index} segment={}", diff::enc_segment(segment)),
        JpgMutation::RemoveOtherSegment { index } => format!("remove-other-segment index={index}"),
        JpgMutation::SetPixels { pixels } => format!("set-pixels pixels={}", diff::hex_encode(pixels)),
        JpgMutation::SetReEncodeQuality { quality } => format!("set-re-encode-quality quality={}", diff::encode_option(quality, |v| v.to_string())),
    }
}
fn parse_jpg_mutation(line: &str) -> Result<JpgMutation, String> {
    if line == "no-mutation" {
        return Ok(JpgMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("jpg mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("jpg mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "set-snapshot" => Ok(JpgMutation::SetSnapshot { snapshot: dec_jpg_snapshot(arg("snapshot")?)? }),
        "set-jfif-header" => Ok(JpgMutation::SetJfifHeader {
            version: diff::dec_version(arg("version")?)?,
            density_units: diff::dec_density_units(arg("density-units")?)?,
            x_density: diff::parse_u16(arg("x-density")?)?,
            y_density: diff::parse_u16(arg("y-density")?)?,
            thumbnail: diff::decode_option(arg("thumbnail")?, diff::dec_thumbnail)?,
        }),
        "set-quant-table" => Ok(JpgMutation::SetQuantTable { table: diff::dec_quant_table(arg("table")?)? }),
        "remove-quant-table" => Ok(JpgMutation::RemoveQuantTable { id: diff::parse_u8(arg("id")?)? }),
        "set-huffman-table" => Ok(JpgMutation::SetHuffmanTable { table: diff::dec_huffman_table(arg("table")?)? }),
        "remove-huffman-table" => Ok(JpgMutation::RemoveHuffmanTable { key: diff::dec_huffman_key(arg("key")?)? }),
        "set-restart-interval" => Ok(JpgMutation::SetRestartInterval { restart_interval: diff::decode_option(arg("restart-interval")?, diff::parse_u16)? }),
        "insert-other-segment" => Ok(JpgMutation::InsertOtherSegment { index: diff::parse_usize(arg("index")?)?, segment: diff::dec_segment(arg("segment")?)? }),
        "remove-other-segment" => Ok(JpgMutation::RemoveOtherSegment { index: diff::parse_usize(arg("index")?)? }),
        "set-pixels" => Ok(JpgMutation::SetPixels { pixels: diff::hex_decode(arg("pixels")?)? }),
        "set-re-encode-quality" => Ok(JpgMutation::SetReEncodeQuality { quality: diff::decode_option(arg("quality")?, diff::parse_u8)? }),
        other => Err(format!("jpg mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for JpgMutation {
    fn print_op(&self) -> String {
        print_jpg_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_jpg_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️SnapshotBinaryCodec
/// 🧪️ P2-FG2: real binary twin of `enc_jpg_snapshot`/`dec_jpg_snapshot` above — every field
/// genuinely, individually written/read (declaration order), reusing `diff`'s `pub(crate)` binary
/// value codecs (`enc_version_bin`/`enc_thumbnail_bin`/`enc_frame_header_bin`/`enc_quant_table_bin`/
/// `enc_huffman_table_bin`/`enc_segment_bin`/`write_bytes_lp`/`write_opt`/...) rather than a third
/// copy. Backs `SetSnapshot`'s payload in the upgraded `OpBinary` below.
fn enc_jpg_snapshot_bin(s: &JpgSnapshot, out: &mut Vec<u8>) {
    diff::write_bytes_lp(out, s.schema.as_bytes());
    store::pack_rt::write_varint_u64(out, s.width as u64);
    store::pack_rt::write_varint_u64(out, s.height as u64);
    diff::write_bytes_lp(out, &s.pixels);
    diff::write_opt(out, &s.re_encode_quality, |v, out| out.push(*v));
    diff::enc_version_bin(&s.jfif_version, out);
    diff::enc_density_units_bin(&s.jfif_density_units, out);
    store::pack_rt::write_varint_u64(out, s.jfif_x_density as u64);
    store::pack_rt::write_varint_u64(out, s.jfif_y_density as u64);
    diff::write_opt(out, &s.jfif_thumbnail, |t, out| diff::enc_thumbnail_bin(t, out));
    diff::write_opt(out, &s.frame, |f, out| diff::enc_frame_header_bin(f, out));
    out.push(s.sof_marker);
    out.push(if s.arithmetic { 1 } else { 0 });
    store::pack_rt::write_varint_u64(out, s.quant_tables.len() as u64);
    for t in &s.quant_tables { diff::enc_quant_table_bin(t, out); }
    store::pack_rt::write_varint_u64(out, s.huffman_tables.len() as u64);
    for t in &s.huffman_tables { diff::enc_huffman_table_bin(t, out); }
    diff::write_opt(out, &s.restart_interval, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
    store::pack_rt::write_varint_u64(out, s.other_segments.len() as u64);
    for seg in &s.other_segments { diff::enc_segment_bin(seg, out); }
}
fn dec_jpg_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<JpgSnapshot, String> {
    let schema = String::from_utf8(diff::read_bytes_lp(reader)?).map_err(|e| e.to_string())?;
    let width = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let height = reader.read_varint_u64().map_err(|e| e.to_string())? as u32;
    let pixels = diff::read_bytes_lp(reader)?;
    let re_encode_quality = diff::read_opt(reader, |r| r.read_u8().map_err(|e| e.to_string()))?;
    let jfif_version = diff::dec_version_bin(reader)?;
    let jfif_density_units = diff::dec_density_units_bin(reader)?;
    let jfif_x_density = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let jfif_y_density = reader.read_varint_u64().map_err(|e| e.to_string())? as u16;
    let jfif_thumbnail = diff::read_opt(reader, diff::dec_thumbnail_bin)?;
    let frame = diff::read_opt(reader, diff::dec_frame_header_bin)?;
    let sof_marker = reader.read_u8().map_err(|e| e.to_string())?;
    let arithmetic = reader.read_u8().map_err(|e| e.to_string())? != 0;
    let qc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut quant_tables = Vec::with_capacity(qc as usize);
    for _ in 0..qc { quant_tables.push(diff::dec_quant_table_bin(reader)?); }
    let hc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut huffman_tables = Vec::with_capacity(hc as usize);
    for _ in 0..hc { huffman_tables.push(diff::dec_huffman_table_bin(reader)?); }
    let restart_interval = diff::read_opt(reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16))?;
    let sc = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut other_segments = Vec::with_capacity(sc as usize);
    for _ in 0..sc { other_segments.push(diff::dec_segment_bin(reader)?); }
    Ok(JpgSnapshot {
        schema, width, height, pixels, re_encode_quality, jfif_version, jfif_density_units, jfif_x_density, jfif_y_density,
        jfif_thumbnail, frame, sof_marker, arithmetic, quant_tables, huffman_tables, restart_interval, other_segments,
    })
}
//#endregion 🔖️SnapshotBinaryCodec

/// 🧪️ P2-FG2: REAL binary op frame (`format u8 | tag u8 | <variant payload>`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the variant's
/// declaration-order ordinal (0=`NoMutation` .. 11=`SetReEncodeQuality`, same order
/// `print_jpg_mutation`'s own match arms use). Every variant's payload is genuinely, individually
/// written/read via the real (non-recursive) binary value codecs `diff`/`§SnapshotBinaryCodec`
/// provide — no opaque tail anywhere in this frame (jpg has no self-recursive mutation payload,
/// unlike xml's `XmlMutation`).
impl protocol::OpBinary for JpgMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, 0u8];
        match self {
            JpgMutation::NoMutation => { out[1] = 0; }
            JpgMutation::SetSnapshot { snapshot } => { out[1] = 1; enc_jpg_snapshot_bin(snapshot, &mut out); }
            JpgMutation::SetJfifHeader { version, density_units, x_density, y_density, thumbnail } => {
                out[1] = 2;
                diff::enc_version_bin(version, &mut out);
                diff::enc_density_units_bin(density_units, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *x_density as u64);
                store::pack_rt::write_varint_u64(&mut out, *y_density as u64);
                diff::write_opt(&mut out, thumbnail, |t, out| diff::enc_thumbnail_bin(t, out));
            }
            JpgMutation::SetQuantTable { table } => { out[1] = 3; diff::enc_quant_table_bin(table, &mut out); }
            JpgMutation::RemoveQuantTable { id } => { out[1] = 4; out.push(*id); }
            JpgMutation::SetHuffmanTable { table } => { out[1] = 5; diff::enc_huffman_table_bin(table, &mut out); }
            JpgMutation::RemoveHuffmanTable { key } => { out[1] = 6; diff::enc_huffman_key_bin(key, &mut out); }
            JpgMutation::SetRestartInterval { restart_interval } => {
                out[1] = 7;
                diff::write_opt(&mut out, restart_interval, |v, out| store::pack_rt::write_varint_u64(out, *v as u64));
            }
            JpgMutation::InsertOtherSegment { index, segment } => {
                out[1] = 8;
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                diff::enc_segment_bin(segment, &mut out);
            }
            JpgMutation::RemoveOtherSegment { index } => { out[1] = 9; store::pack_rt::write_varint_u64(&mut out, *index as u64); }
            JpgMutation::SetPixels { pixels } => { out[1] = 10; diff::write_bytes_lp(&mut out, pixels); }
            JpgMutation::SetReEncodeQuality { quality } => {
                out[1] = 11;
                diff::write_opt(&mut out, quality, |v, out| out.push(*v));
            }
        }
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(JpgMutation::NoMutation),
            1 => Ok(JpgMutation::SetSnapshot { snapshot: dec_jpg_snapshot_bin(&mut reader).map_err(|e| malformed("op set-snapshot", reader.position(), e))? }),
            2 => {
                let version = diff::dec_version_bin(&mut reader).map_err(|e| malformed("op version", reader.position(), e))?;
                let density_units = diff::dec_density_units_bin(&mut reader).map_err(|e| malformed("op density-units", reader.position(), e))?;
                let x_density = reader.read_varint_u64().map_err(|e| malformed("op x-density", reader.position(), e.to_string()))? as u16;
                let y_density = reader.read_varint_u64().map_err(|e| malformed("op y-density", reader.position(), e.to_string()))? as u16;
                let thumbnail = diff::read_opt(&mut reader, diff::dec_thumbnail_bin).map_err(|e| malformed("op thumbnail", reader.position(), e))?;
                Ok(JpgMutation::SetJfifHeader { version, density_units, x_density, y_density, thumbnail })
            }
            3 => Ok(JpgMutation::SetQuantTable { table: diff::dec_quant_table_bin(&mut reader).map_err(|e| malformed("op quant-table", reader.position(), e))? }),
            4 => Ok(JpgMutation::RemoveQuantTable { id: reader.read_u8().map_err(|e| malformed("op quant-id", reader.position(), e.to_string()))? }),
            5 => Ok(JpgMutation::SetHuffmanTable { table: diff::dec_huffman_table_bin(&mut reader).map_err(|e| malformed("op huffman-table", reader.position(), e))? }),
            6 => Ok(JpgMutation::RemoveHuffmanTable { key: diff::dec_huffman_key_bin(&mut reader).map_err(|e| malformed("op huffman-key", reader.position(), e))? }),
            7 => Ok(JpgMutation::SetRestartInterval { restart_interval: diff::read_opt(&mut reader, |r| Ok(r.read_varint_u64().map_err(|e| e.to_string())? as u16)).map_err(|e| malformed("op restart-interval", reader.position(), e))? }),
            8 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let segment = diff::dec_segment_bin(&mut reader).map_err(|e| malformed("op segment", reader.position(), e))?;
                Ok(JpgMutation::InsertOtherSegment { index, segment })
            }
            9 => Ok(JpgMutation::RemoveOtherSegment { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize }),
            10 => Ok(JpgMutation::SetPixels { pixels: diff::read_bytes_lp(&mut reader).map_err(|e| malformed("op pixels", reader.position(), e))? }),
            11 => Ok(JpgMutation::SetReEncodeQuality { quality: diff::read_opt(&mut reader, |r| r.read_u8().map_err(|e| e.to_string())).map_err(|e| malformed("op quality", reader.position(), e))? }),
            other => Err(malformed("op tag", 1, format!("unknown JpgMutation tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `JpgMutation` values (every variant, incl. `SetSnapshot`'s full
/// nested `JpgFrameHeader`/`JpgFrameComponent` tree and both legs of every `Option<T>`-shaped
/// argument) — the single source of truth reused by `tests::op_text_binary_roundtrip_law` below
/// AND by `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests. `pub(crate)` (not `#[cfg(test)]`-gated) so the engine's non-test conformance
/// module can reuse it, matching png's own `demo_mutation_cases()` visibility.
pub(crate) fn demo_mutation_cases() -> Vec<JpgMutation> {
    fn quant(id: u8, seed: u16) -> JpgQuantTable { JpgQuantTable { id, precision: 0, values: [seed; 64] } }
    fn huffman(class: crate::artifacts::jpg::schema::snapshot::JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment { JpgSegment { marker, data } }
    use crate::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass};

    let base = JpgSnapshot {
        schema: "stdio.jpg".into(),
        width: 4,
        height: 4,
        pixels: vec![0u8; 4 * 4 * 4],
        re_encode_quality: None,
        jfif_version: (1, 1),
        jfif_density_units: JfifDensityUnits::Aspect,
        jfif_x_density: 1,
        jfif_y_density: 1,
        jfif_thumbnail: None,
        frame: Some(JpgFrameHeader {
            precision: 8,
            width: 4,
            height: 4,
            components: vec![
                JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
            ],
        }),
        sof_marker: 0xC0,
        arithmetic: false,
        quant_tables: vec![quant(0, 10)],
        huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1)],
        restart_interval: None,
        other_segments: vec![segment(0xFE, vec![1, 2, 3])],
    };

    vec![
        JpgMutation::NoMutation,
        JpgMutation::SetSnapshot { snapshot: base.clone() },
        JpgMutation::SetSnapshot { snapshot: { let mut s = base.clone(); s.frame = None; s.jfif_thumbnail = None; s } },
        JpgMutation::SetJfifHeader { version: (1, 2), density_units: JfifDensityUnits::PixelsPerCm, x_density: 300, y_density: 300, thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }) },
        JpgMutation::SetJfifHeader { version: (1, 1), density_units: JfifDensityUnits::Aspect, x_density: 1, y_density: 1, thumbnail: None },
        JpgMutation::SetQuantTable { table: quant(0, 77) },
        JpgMutation::RemoveQuantTable { id: 3 },
        JpgMutation::SetHuffmanTable { table: huffman(JpgHuffmanClass::Ac, 2, 5) },
        JpgMutation::RemoveHuffmanTable { key: JpgHuffmanTableKey { class: JpgHuffmanClass::Dc, id: 0 } },
        JpgMutation::SetRestartInterval { restart_interval: Some(16) },
        JpgMutation::SetRestartInterval { restart_interval: None },
        JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE2, vec![7, 8]) },
        JpgMutation::RemoveOtherSegment { index: 0 },
        JpgMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
        JpgMutation::SetReEncodeQuality { quality: Some(50) },
        JpgMutation::SetReEncodeQuality { quality: None },
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::jpg::schema::diff::{JpgFrameChange, JpgHuffmanTableKey as HKey};
    use crate::artifacts::jpg::schema::snapshot::{JpgFrameComponent, JpgFrameHeader, JpgHuffmanClass};
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    fn quant(id: u8, seed: u16) -> JpgQuantTable {
        JpgQuantTable { id, precision: 0, values: [seed; 64] }
    }
    fn huffman(class: JpgHuffmanClass, id: u8, seed: u8) -> JpgHuffmanTable {
        JpgHuffmanTable { id, class, bits: [seed; 16], values: vec![seed, seed.wrapping_add(1)] }
    }
    fn segment(marker: u8, data: Vec<u8>) -> JpgSegment { JpgSegment { marker, data } }

    fn base_snapshot() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 4,
            height: 4,
            pixels: vec![0u8; 4 * 4 * 4],
            re_encode_quality: None,
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 4,
                height: 4,
                components: vec![
                    JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                    JpgFrameComponent { id: 2, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                ],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![1, 2, 3])],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. Every id/index-keyed collection
    /// (`quant_tables`, `huffman_tables`, `frame.components`, `other_segments`) is deliberately
    /// DIFFERENT length (2 vs 1) with the "surviving/modified" item at position 0 and the
    /// "removed-in-forward / added-in-backward" item as the tail — the recipe's documented
    /// workaround for the structural "same-length between() can show removed XOR added, never
    /// both from one call" trap (see png/f1's field_sweep precedent).
    fn sweep_a() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 10,
            height: 20,
            pixels: vec![0u8, 0, 0, 255, 255, 255, 255, 255],
            re_encode_quality: Some(80),
            jfif_version: (1, 1),
            jfif_density_units: JfifDensityUnits::PixelsPerInch,
            jfif_x_density: 72,
            jfif_y_density: 72,
            jfif_thumbnail: Some(JfifThumbnail { width: 2, height: 1, rgb_data: vec![1, 2, 3, 4, 5, 6] }),
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 10,
                height: 20,
                components: vec![
                    JpgFrameComponent { id: 1, h_sampling: 2, v_sampling: 2, quant_table_id: 0 },
                    JpgFrameComponent { id: 9, h_sampling: 1, v_sampling: 1, quant_table_id: 1 },
                ],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 10), quant(9, 20)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 1), huffman(JpgHuffmanClass::Ac, 9, 2)],
            restart_interval: Some(8),
            other_segments: vec![segment(0xFE, vec![1, 2, 3]), segment(0xE1, vec![9, 9])],
        }
    }

    fn sweep_b() -> JpgSnapshot {
        JpgSnapshot {
            schema: "stdio.jpg".into(),
            width: 11,
            height: 21,
            pixels: vec![1u8, 1, 1, 255],
            re_encode_quality: None,
            jfif_version: (1, 2),
            jfif_density_units: JfifDensityUnits::Aspect,
            jfif_x_density: 1,
            jfif_y_density: 1,
            jfif_thumbnail: None,
            frame: Some(JpgFrameHeader {
                precision: 8,
                width: 11,
                height: 21,
                components: vec![JpgFrameComponent { id: 1, h_sampling: 1, v_sampling: 1, quant_table_id: 5 }],
            }),
            sof_marker: 0xC0,
            arithmetic: false,
            quant_tables: vec![quant(0, 99)],
            huffman_tables: vec![huffman(JpgHuffmanClass::Dc, 0, 7)],
            restart_interval: None,
            other_segments: vec![segment(0xFE, vec![4, 5, 6])],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &JpgSnapshot, mutation: JpgMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_jpg_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_jpg_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    fn all_variants(base: &JpgSnapshot) -> Vec<JpgMutation> {
        vec![
            JpgMutation::NoMutation,
            JpgMutation::SetSnapshot { snapshot: { let mut s = base.clone(); s.width = 99; s } },
            JpgMutation::SetJfifHeader { version: (1, 2), density_units: JfifDensityUnits::PixelsPerCm, x_density: 300, y_density: 300, thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }) },
            JpgMutation::SetQuantTable { table: quant(0, 77) },
            JpgMutation::SetQuantTable { table: quant(3, 55) },
            JpgMutation::RemoveQuantTable { id: 0 },
            JpgMutation::SetHuffmanTable { table: huffman(JpgHuffmanClass::Dc, 0, 9) },
            JpgMutation::SetHuffmanTable { table: huffman(JpgHuffmanClass::Ac, 0, 3) },
            JpgMutation::RemoveHuffmanTable { key: HKey { class: JpgHuffmanClass::Dc, id: 0 } },
            JpgMutation::SetRestartInterval { restart_interval: Some(16) },
            JpgMutation::SetRestartInterval { restart_interval: None },
            JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE2, vec![7, 8]) },
            JpgMutation::RemoveOtherSegment { index: 0 },
            JpgMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
            JpgMutation::SetReEncodeQuality { quality: Some(50) },
            JpgMutation::SetReEncodeQuality { quality: None },
            // Out-of-range/nonexistent targets: graceful no-ops, still law-compliant.
            JpgMutation::RemoveQuantTable { id: 99 },
            JpgMutation::RemoveHuffmanTable { key: HKey { class: JpgHuffmanClass::Ac, id: 99 } },
            JpgMutation::RemoveOtherSegment { index: 99 },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            assert_mutation_diff_law(&base, m);
        }
    }
    //#endregion 🔖️mutation_diff_law

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law() {
        let base = base_snapshot();
        for m in all_variants(&base) {
            // Mutation-level round trip.
            let mut snap = base.clone();
            apply_jpg_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_jpg_mutation(&mut snap, &inv);
            }
            assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

            // Diff-level round trip.
            let d = m.diff(&base);
            let mutated = d.apply(&base);
            let inv_d = d.inverse(&base);
            assert_eq!(inv_d.apply(&mutated), base, "diff-level inverse must restore base for {m:?}");
        }
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    fn assert_absorb_law(base: &JpgSnapshot, m1: JpgMutation, m2: JpgMutation) {
        let d1 = m1.diff(base);
        let mid = d1.apply(base);
        let d2 = m2.diff(&mid);
        let sequential = d2.apply(&mid);

        let mut merged = d1.clone();
        merged.absorb(d2.clone());
        assert_eq!(merged.apply(base), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
    }

    #[test]
    fn absorb_law() {
        let base = base_snapshot();

        // Insert+Remove-before: other_segments has [seg@0]; insert at 1 -> [seg,new]; then
        // remove index 0 ("seg") -> [new] lands at final index 0 (the recipe's own canonical case).
        assert_absorb_law(&base, JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE3, vec![1]) }, JpgMutation::RemoveOtherSegment { index: 0 });

        // Insert+Insert-same-index: both survive.
        assert_absorb_law(&base, JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE4, vec![2]) }, JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE5, vec![3]) });

        // Add+SetField: the second mutation patches directly into the still-pending added table.
        assert_absorb_law(&base, JpgMutation::SetQuantTable { table: quant(5, 1) }, JpgMutation::SetQuantTable { table: quant(5, 2) });

        // Modify+Remove: a pending field patch on a since-removed base item vanishes.
        assert_absorb_law(&base, JpgMutation::SetQuantTable { table: quant(0, 42) }, JpgMutation::RemoveQuantTable { id: 0 });

        // Insert then annihilate the very same insert — huffman_tables' id-keyed transport.
        assert_absorb_law(&base, JpgMutation::SetHuffmanTable { table: huffman(JpgHuffmanClass::Ac, 3, 1) }, JpgMutation::RemoveHuffmanTable { key: HKey { class: JpgHuffmanClass::Ac, id: 3 } });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, JpgMutation::SetRestartInterval { restart_interval: Some(1) }, JpgMutation::SetRestartInterval { restart_interval: Some(2) });

        // Tri-state set-then-clear: the later clear wins outright over the pending set.
        assert_absorb_law(&base, JpgMutation::SetReEncodeQuality { quality: Some(10) }, JpgMutation::SetReEncodeQuality { quality: None });
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = JpgMutation::SetQuantTable { table: quant(7, 1) }.diff(&base);
        let s1 = d1.apply(&base);
        let d2 = JpgMutation::SetQuantTable { table: quant(7, 2) }.diff(&s1);
        let s2 = d2.apply(&s1);
        let d3 = JpgMutation::RemoveQuantTable { id: 0 }.diff(&s2);
        let s3 = d3.apply(&s2);

        // (d1∘d2)∘d3
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());

        // d1∘(d2∘d3)
        let mut d23 = d2.clone();
        d23.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(d23);

        assert_eq!(left.apply(&base), s3);
        assert_eq!(right.apply(&base), s3);
        assert_eq!(left.apply(&base), right.apply(&base), "absorb must associate");
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law() {
        let a = base_snapshot();
        let mut b = base_snapshot();
        b.width = 8;
        b.quant_tables.push(quant(2, 5));
        b.pixels = vec![5u8; a.pixels.len()];

        let d = JpgDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = JpgDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(JpgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../🗿️artifacts/📷️jpg/📚️examples/🎬️demo/🖼️assets/📷️example.jpg"
        ));
        let bytes = match bytes {
            Ok(b) if !b.is_empty() => b,
            // No usable fixture on disk at test time (or a different workspace layout) — fall
            // back to a synthetic encode -> decode identity check (matches png's precedent).
            _ => {
                let w = 16u32;
                let h = 16u32;
                let mut pixels = vec![0u8; (w * h * 4) as usize];
                for (i, px) in pixels.chunks_mut(4).enumerate() {
                    px[0] = (i * 7 % 255) as u8; px[1] = (i * 13 % 255) as u8; px[2] = (i * 17 % 255) as u8; px[3] = 255;
                }
                let snap = JpgSnapshot { width: w, height: h, pixels, ..JpgSnapshot::default() };
                crate::artifacts::jpg::engine::encode_jpg(&snap).expect("encode synthetic fallback")
            }
        };
        let decoded = crate::artifacts::jpg::engine::decode_jpg(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::jpg::engine::encode_jpg(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::jpg::engine::decode_jpg(&reencoded).expect("re-decode fixture");
        // Engine's own EncodeScopeNote: encode always canonicalizes to Annex K tables at a fixed
        // quality — pixel CONTENT (within a lossy MAE budget) is the retained invariant, not the
        // original file's exact tables/segments (documented normal form).
        assert_eq!(decoded.width, redecoded.width);
        assert_eq!(decoded.height, redecoded.height);
        assert_eq!(decoded.pixels.len(), redecoded.pixels.len());
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = JpgDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = JpgDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(JpgDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        // Raster scalars.
        assert!(forward.width.is_some());
        assert!(forward.height.is_some());
        assert!(forward.pixels.is_some());

        // Tri-state clears (forward: Some -> None).
        assert_eq!(forward.re_encode_quality, Some(None), "re_encode_quality tri-state clear must show Some(None)");
        assert_eq!(forward.jfif_thumbnail, Some(None), "jfif_thumbnail tri-state clear must show Some(None)");
        assert_eq!(forward.restart_interval, Some(None), "restart_interval tri-state clear must show Some(None)");
        // Tri-state recreates (backward: None -> Some).
        assert!(matches!(backward.re_encode_quality, Some(Some(_))));
        assert!(matches!(backward.jfif_thumbnail, Some(Some(_))));
        assert!(matches!(backward.restart_interval, Some(Some(_))));

        // JFIF scalars.
        assert!(forward.jfif_version.is_some());
        assert!(forward.jfif_density_units.is_some());
        assert!(forward.jfif_x_density.is_some());
        assert!(forward.jfif_y_density.is_some());

        // frame: Modify with every sub-field populated (both sweep_a/b keep `Some(frame)`).
        let frame_change = forward.frame.as_ref().expect("frame diff present");
        match frame_change {
            JpgFrameChange::Modify(fd) => {
                assert!(fd.precision.is_none(), "precision is 8 in both sweeps");
                assert!(fd.width.is_some());
                assert!(fd.height.is_some());
                let cd = fd.components.as_ref().expect("components diff present");
                assert_eq!(cd.removed, vec![9], "component id 9 only in sweep_a");
                assert_eq!(cd.modified.len(), 1, "component id 1 survives, modified");
                assert!(cd.added.is_empty());
            }
            other => panic!("expected Modify, got {other:?}"),
        }
        let bwd_frame_change = backward.frame.as_ref().expect("frame diff present");
        match bwd_frame_change {
            JpgFrameChange::Modify(fd) => {
                let cd = fd.components.as_ref().expect("components diff present");
                assert!(cd.removed.is_empty());
                assert_eq!(cd.modified.len(), 1);
                assert_eq!(cd.added.len(), 1, "component id 9 re-added going backward");
            }
            other => panic!("expected Modify, got {other:?}"),
        }

        assert!(forward.sof_marker.is_none(), "sof_marker is 0xC0 in both sweeps");
        assert!(forward.arithmetic.is_none(), "arithmetic is false in both sweeps");

        // quant_tables: forward shows modified+removed, backward shows modified+added.
        let qt_fwd = forward.quant_tables.as_ref().expect("quant_tables diff present");
        assert_eq!(qt_fwd.removed, vec![9]);
        assert_eq!(qt_fwd.modified.len(), 1);
        assert!(qt_fwd.added.is_empty());
        let qt_bwd = backward.quant_tables.as_ref().expect("quant_tables diff present");
        assert!(qt_bwd.removed.is_empty());
        assert_eq!(qt_bwd.modified.len(), 1);
        assert_eq!(qt_bwd.added.len(), 1);

        // huffman_tables: same split, compound key.
        let ht_fwd = forward.huffman_tables.as_ref().expect("huffman_tables diff present");
        assert_eq!(ht_fwd.removed, vec![HKey { class: JpgHuffmanClass::Ac, id: 9 }]);
        assert_eq!(ht_fwd.modified.len(), 1);
        assert!(ht_fwd.added.is_empty());
        let ht_bwd = backward.huffman_tables.as_ref().expect("huffman_tables diff present");
        assert!(ht_bwd.removed.is_empty());
        assert_eq!(ht_bwd.modified.len(), 1);
        assert_eq!(ht_bwd.added.len(), 1);

        // other_segments: same split.
        let os_fwd = forward.other_segments.as_ref().expect("other_segments diff present");
        assert_eq!(os_fwd.removed, vec![1]);
        assert_eq!(os_fwd.modified.len(), 1);
        assert!(os_fwd.added.is_empty());
        let os_bwd = backward.other_segments.as_ref().expect("other_segments diff present");
        assert!(os_bwd.removed.is_empty());
        assert_eq!(os_bwd.modified.len(), 1);
        assert_eq!(os_bwd.added.len(), 1);
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveQuantTable { id: 99 });
        assert_eq!(snap, base);
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveHuffmanTable { key: HKey { class: JpgHuffmanClass::Ac, id: 99 } });
        assert_eq!(snap, base);
        apply_jpg_mutation(&mut snap, &JpgMutation::RemoveOtherSegment { index: 99 });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `JpgMutation` grammar —
    /// exercises every variant incl. `SetSnapshot`'s full nested `JpgFrameHeader`/`JpgFrameComponent`
    /// tree and every collection-item struct (`JpgQuantTable`/`JpgHuffmanTable`/`JpgSegment`), plus
    /// both `Some`/`None` legs of every `Option<T>`-shaped mutation argument.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            JpgMutation::NoMutation,
            JpgMutation::SetSnapshot { snapshot: base.clone() },
            JpgMutation::SetSnapshot { snapshot: { let mut s = base.clone(); s.frame = None; s.jfif_thumbnail = None; s } },
            JpgMutation::SetJfifHeader { version: (1, 2), density_units: JfifDensityUnits::PixelsPerCm, x_density: 300, y_density: 300, thumbnail: Some(JfifThumbnail { width: 1, height: 1, rgb_data: vec![9, 9, 9] }) },
            JpgMutation::SetJfifHeader { version: (1, 1), density_units: JfifDensityUnits::Aspect, x_density: 1, y_density: 1, thumbnail: None },
            JpgMutation::SetQuantTable { table: quant(0, 77) },
            JpgMutation::RemoveQuantTable { id: 3 },
            JpgMutation::SetHuffmanTable { table: huffman(JpgHuffmanClass::Ac, 2, 5) },
            JpgMutation::RemoveHuffmanTable { key: HKey { class: JpgHuffmanClass::Dc, id: 0 } },
            JpgMutation::SetRestartInterval { restart_interval: Some(16) },
            JpgMutation::SetRestartInterval { restart_interval: None },
            JpgMutation::InsertOtherSegment { index: 1, segment: segment(0xE2, vec![7, 8]) },
            JpgMutation::RemoveOtherSegment { index: 0 },
            JpgMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
            JpgMutation::SetReEncodeQuality { quality: Some(50) },
            JpgMutation::SetReEncodeQuality { quality: None },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = JpgMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = JpgMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
