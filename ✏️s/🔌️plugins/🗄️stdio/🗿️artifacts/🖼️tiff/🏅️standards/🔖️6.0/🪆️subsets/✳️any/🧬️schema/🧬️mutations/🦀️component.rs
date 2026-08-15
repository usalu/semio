//! 🧬️ TiffMutation — document mutation dispatch. Every variant's `diff()` is handcrafted
//! (constructs the sparse `TiffDiff` directly via the `schema::diff` builders — apply-and-
//! capture is banned); `inverse()` is handcrafted per variant, index/tag-aware, reading the
//! pre-state it needs from `base`. `apply_tiff_mutation` follows csv/png's proven
//! single-source-of-truth shape: `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`.
//!
//! 🧪️ F6 CONFIRMED (real `cargo check`, ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION): adding
//! `#[derive(dsl::DslOps)]` to `TiffMutation` fails — `SetTag.values: TiffValues` carries the
//! data-carrying `TiffValues` enum DIRECTLY as a variant field, and `SetSnapshot.snapshot:
//! TiffSnapshot`/`InsertIfd.ifd: TiffIfd` both recursively reach the same `TiffValues` through
//! `ifds`/`entries` (`error[E0277]: the trait bound …::TiffValues: DslField is not satisfied`,
//! same root cause as `TiffDiff`'s `DiffCodec` blocker — see that file's doc comment). `OpText`/
//! `OpBinary` hand-rolled below, reusing `TiffDiff`'s `pub(crate)` grammar primitives
//! (`hex_encode`/`enc_values`/`split_top_level`/…) rather than duplicating them a second time.

use crate::artifacts::tiff::schema::diff::{
    self, dec_byte_order, dec_field_type, dec_ifd, dec_ifd_bin, dec_list, dec_str, dec_values, dec_values_bin, enc_byte_order, enc_field_type, enc_ifd, enc_ifd_bin, enc_list, enc_str, enc_values, enc_values_bin, hex_decode, hex_encode, parse_num,
    read_bytes_lp, read_str_lp, split_top_level, strip_brackets, write_bytes_lp, write_str_lp, TiffDiff,
};
use crate::artifacts::tiff::schema::snapshot::{TiffByteOrder, TiffFieldType, TiffIfd, TiffTag, TiffValues};
use crate::artifacts::tiff::TiffSnapshot;
#[cfg(test)]
use protocol::OpBinary;
use protocol::{Mutation, MutationDiff, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.tiff`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum TiffMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: TiffSnapshot,
    },
    /// 🧭️ Replaces the II/MM byte-order mark.
    SetByteOrder {
        byte_order: TiffByteOrder,
    },
    /// ➕️ Inserts a whole IFD at `index` (final position, clamped to `len`).
    InsertIfd {
        index: usize,
        ifd: TiffIfd,
    },
    /// ➖️ Removes the IFD at `index` (no-op if out of range).
    RemoveIfd {
        index: usize,
    },
    /// ✏️ Creates-or-updates one tag entry in `ifds[ifd_index]` (no-op if `ifd_index` is out
    /// of range).
    SetTag {
        ifd_index: usize,
        tag: u16,
        kind: TiffFieldType,
        values: TiffValues,
    },
    /// ➖️ Removes one tag entry from `ifds[ifd_index]` (no-op if out of range or absent).
    RemoveTag {
        ifd_index: usize,
        tag: u16,
    },
    /// 🖼️ Replaces the decoded canonical RGBA8 raster wholesale.
    SetPixels {
        pixels: Vec<u8>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` — the diff is the single semantics source (csv/png precedent).
pub fn apply_tiff_mutation(snapshot: &mut TiffSnapshot, mutation: &TiffMutation) -> TiffDiff {
    let d = <TiffMutation as Mutation<TiffSnapshot>>::diff(mutation, snapshot);
    *snapshot = <TiffDiff as MutationDiff<TiffSnapshot>>::apply(&d, snapshot);
    d
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<TiffSnapshot> for TiffMutation {
    type Diff = TiffDiff;

    fn diff(&self, base: &TiffSnapshot) -> Self::Diff {
        match self {
            TiffMutation::NoMutation => TiffDiff::default(),
            TiffMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            TiffMutation::SetByteOrder { byte_order } => diff::diff_set_byte_order(base, *byte_order),
            TiffMutation::InsertIfd { index, ifd } => diff::diff_insert_ifd(base, *index, ifd.clone()),
            TiffMutation::RemoveIfd { index } => diff::diff_remove_ifd(base, *index),
            TiffMutation::SetTag { ifd_index, tag, kind, values } => diff::diff_set_tag(base, *ifd_index, *tag, *kind, values.clone()),
            TiffMutation::RemoveTag { ifd_index, tag } => diff::diff_remove_tag(base, *ifd_index, *tag),
            TiffMutation::SetPixels { pixels } => diff::diff_set_pixels(base, pixels.clone()),
        }
    }

    /// ↩️ Handcrafted, index/tag-aware mutation-level inverses. Out-of-range targets invert to
    /// `NoMutation` (nothing to undo).
    fn inverse(&self, base: &TiffSnapshot) -> Vec<Self> {
        match self {
            TiffMutation::NoMutation => vec![TiffMutation::NoMutation],
            TiffMutation::SetSnapshot { .. } => vec![TiffMutation::SetSnapshot { snapshot: base.clone() }],
            TiffMutation::SetByteOrder { .. } => vec![TiffMutation::SetByteOrder { byte_order: base.byte_order }],
            TiffMutation::InsertIfd { index, .. } => vec![TiffMutation::RemoveIfd { index: (*index).min(base.ifds.len()) }],
            TiffMutation::RemoveIfd { index } => match base.ifds.get(*index) {
                Some(ifd) => vec![TiffMutation::InsertIfd { index: *index, ifd: ifd.clone() }],
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::SetTag { ifd_index, tag, .. } => match base.ifds.get(*ifd_index) {
                Some(ifd) => match ifd.entries.iter().find(|t| t.tag == *tag) {
                    Some(existing) => vec![TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() }],
                    None => vec![TiffMutation::RemoveTag { ifd_index: *ifd_index, tag: *tag }],
                },
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::RemoveTag { ifd_index, tag } => match base.ifds.get(*ifd_index).and_then(|ifd| ifd.entries.iter().find(|t| t.tag == *tag)) {
                Some(existing) => vec![TiffMutation::SetTag { ifd_index: *ifd_index, tag: *tag, kind: existing.kind, values: existing.values.clone() }],
                None => vec![TiffMutation::NoMutation],
            },
            TiffMutation::SetPixels { .. } => vec![TiffMutation::SetPixels { pixels: base.pixels.clone() }],
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `TiffMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `TiffDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_values`/`split_top_level`/`enc_list`/…) rather than duplicating them a
/// second time in this file. Grammar: `keyword arg=value ...` (space-separated, same shape the
/// derive's own handcrafted-wrapper convention uses), one match arm per variant (no `DslVariants`
/// scaffolding available since nothing here derives it).
fn enc_snapshot(s: &TiffSnapshot) -> String {
    format!("[{},{},{},{}]", enc_str(&s.schema), enc_byte_order(s.byte_order), enc_list(&s.ifds, enc_ifd), hex_encode(&s.pixels))
}
fn dec_snapshot(s: &str) -> Result<TiffSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, byte_order, ifds, pixels] = parts.as_slice() else { return Err(format!("tiff snapshot: expected 4 fields, got {}", parts.len())) };
    Ok(TiffSnapshot { schema: dec_str(schema)?, byte_order: dec_byte_order(byte_order)?, ifds: dec_list(ifds, dec_ifd)?, pixels: hex_decode(pixels)? })
}

fn print_tiff_mutation(m: &TiffMutation) -> String {
    match m {
        TiffMutation::NoMutation => "no-mutation".to_string(),
        TiffMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        TiffMutation::SetByteOrder { byte_order } => format!("set-byte-order byte-order={}", enc_byte_order(*byte_order)),
        TiffMutation::InsertIfd { index, ifd } => format!("insert-ifd index={index} ifd={}", enc_ifd(ifd)),
        TiffMutation::RemoveIfd { index } => format!("remove-ifd index={index}"),
        TiffMutation::SetTag { ifd_index, tag, kind, values } => {
            format!("set-tag ifd-index={ifd_index} tag={tag} kind={} values={}", enc_field_type(*kind), enc_values(values))
        }
        TiffMutation::RemoveTag { ifd_index, tag } => format!("remove-tag ifd-index={ifd_index} tag={tag}"),
        TiffMutation::SetPixels { pixels } => format!("set-pixels pixels={}", hex_encode(pixels)),
    }
}
fn parse_tiff_mutation(line: &str) -> Result<TiffMutation, String> {
    if line == "no-mutation" {
        return Ok(TiffMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("tiff mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("tiff mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { parse_num::<usize>(arg(k)?) };
    let u16_arg = |k: &str| -> Result<u16, String> { parse_num::<u16>(arg(k)?) };
    match keyword {
        "set-snapshot" => Ok(TiffMutation::SetSnapshot { snapshot: dec_snapshot(arg("snapshot")?)? }),
        "set-byte-order" => Ok(TiffMutation::SetByteOrder { byte_order: dec_byte_order(arg("byte-order")?)? }),
        "insert-ifd" => Ok(TiffMutation::InsertIfd { index: usize_arg("index")?, ifd: dec_ifd(arg("ifd")?)? }),
        "remove-ifd" => Ok(TiffMutation::RemoveIfd { index: usize_arg("index")? }),
        "set-tag" => Ok(TiffMutation::SetTag { ifd_index: usize_arg("ifd-index")?, tag: u16_arg("tag")?, kind: dec_field_type(arg("kind")?)?, values: dec_values(arg("values")?)? }),
        "remove-tag" => Ok(TiffMutation::RemoveTag { ifd_index: usize_arg("ifd-index")?, tag: u16_arg("tag")? }),
        "set-pixels" => Ok(TiffMutation::SetPixels { pixels: hex_decode(arg("pixels")?)? }),
        other => Err(format!("tiff mutation: unknown keyword {other:?}")),
    }
}

impl OpText for TiffMutation {
    fn print_op(&self) -> String {
        print_tiff_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_tiff_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG2: real recursive binary twin of [`enc_snapshot`]/[`dec_snapshot`] — reuses
/// `TiffDiff`'s `pub(crate)` [`write_str_lp`]/[`read_str_lp`]/[`write_bytes_lp`]/[`read_bytes_lp`]/
/// [`enc_ifd_bin`]/[`dec_ifd_bin`] (`../🔺️diff/🦀️component.rs`), same intra-artifact reuse
/// convention this file's own text codec already uses off `TiffDiff`'s grammar primitives.
fn enc_snapshot_bin(s: &TiffSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    out.push(match s.byte_order {
        TiffByteOrder::LittleEndian => 0,
        TiffByteOrder::BigEndian => 1,
    });
    store::pack_rt::write_varint_u64(out, s.ifds.len() as u64);
    s.ifds.iter().for_each(|ifd| enc_ifd_bin(ifd, out));
    write_bytes_lp(out, &s.pixels);
}
fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<TiffSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let byte_order = if reader.read_u8().map_err(|e| e.to_string())? == 0 { TiffByteOrder::LittleEndian } else { TiffByteOrder::BigEndian };
    let n = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut ifds = Vec::with_capacity(n as usize);
    for _ in 0..n {
        ifds.push(dec_ifd_bin(reader)?);
    }
    let pixels = read_bytes_lp(reader)?;
    Ok(TiffSnapshot { schema, byte_order, ifds, pixels })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG2: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape —
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `TiffMutation` variant ordinal, in the same 0-7 order `print_tiff_mutation`'s own keyword
/// match uses.
impl protocol::OpBinary for TiffMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            TiffMutation::NoMutation => 0,
            TiffMutation::SetSnapshot { .. } => 1,
            TiffMutation::SetByteOrder { .. } => 2,
            TiffMutation::InsertIfd { .. } => 3,
            TiffMutation::RemoveIfd { .. } => 4,
            TiffMutation::SetTag { .. } => 5,
            TiffMutation::RemoveTag { .. } => 6,
            TiffMutation::SetPixels { .. } => 7,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            TiffMutation::NoMutation => {}
            TiffMutation::SetSnapshot { snapshot } => enc_snapshot_bin(snapshot, &mut out),
            TiffMutation::SetByteOrder { byte_order } => out.push(match byte_order {
                TiffByteOrder::LittleEndian => 0,
                TiffByteOrder::BigEndian => 1,
            }),
            TiffMutation::InsertIfd { index, ifd } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_ifd_bin(ifd, &mut out);
            }
            TiffMutation::RemoveIfd { index } => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            TiffMutation::SetTag { ifd_index, tag, kind, values } => {
                store::pack_rt::write_varint_u64(&mut out, *ifd_index as u64);
                out.extend_from_slice(&tag.to_le_bytes());
                out.push(kind.to_u16() as u8);
                enc_values_bin(values, &mut out);
            }
            TiffMutation::RemoveTag { ifd_index, tag } => {
                store::pack_rt::write_varint_u64(&mut out, *ifd_index as u64);
                out.extend_from_slice(&tag.to_le_bytes());
            }
            TiffMutation::SetPixels { pixels } => write_bytes_lp(&mut out, pixels),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(TiffMutation::NoMutation),
            1 => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(TiffMutation::SetSnapshot { snapshot })
            }
            2 => {
                let v = reader.read_u8().map_err(|e| malformed("op byte_order", reader.position(), e.to_string()))?;
                Ok(TiffMutation::SetByteOrder { byte_order: if v == 0 { TiffByteOrder::LittleEndian } else { TiffByteOrder::BigEndian } })
            }
            3 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let ifd = dec_ifd_bin(&mut reader).map_err(|e| malformed("op ifd", reader.position(), e))?;
                Ok(TiffMutation::InsertIfd { index, ifd })
            }
            4 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(TiffMutation::RemoveIfd { index })
            }
            5 => {
                let ifd_index = reader.read_varint_u64().map_err(|e| malformed("op ifd_index", reader.position(), e.to_string()))? as usize;
                let tag = reader.read_u16_le().map_err(|e| malformed("op tag", reader.position(), e.to_string()))?;
                let kind = TiffFieldType::from_u16(reader.read_u8().map_err(|e| malformed("op kind", reader.position(), e.to_string()))? as u16).map_err(|e| malformed("op kind", reader.position(), e))?;
                let values = dec_values_bin(&mut reader).map_err(|e| malformed("op values", reader.position(), e))?;
                Ok(TiffMutation::SetTag { ifd_index, tag, kind, values })
            }
            6 => {
                let ifd_index = reader.read_varint_u64().map_err(|e| malformed("op ifd_index", reader.position(), e.to_string()))? as usize;
                let tag = reader.read_u16_le().map_err(|e| malformed("op tag", reader.position(), e.to_string()))?;
                Ok(TiffMutation::RemoveTag { ifd_index, tag })
            }
            7 => {
                let pixels = read_bytes_lp(&mut reader).map_err(|e| malformed("op pixels", reader.position(), e))?;
                Ok(TiffMutation::SetPixels { pixels })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG2: representative `TiffMutation` values (every variant, incl. every `TiffValues`
/// field-type family the recursive `SetTag` payload can carry) — the single source of truth
/// reused by `ops_grammar_conformance_law`/`protocol_walk_law` below (`⚙️engine/🦀️component.rs`).
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<TiffMutation> {
    vec![
        TiffMutation::NoMutation,
        TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian },
        TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![TiffTag { tag: 270, kind: TiffFieldType::Short, values: TiffValues::Short(vec![1]) }] } },
        TiffMutation::RemoveIfd { index: 0 },
        TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) },
        TiffMutation::SetTag { ifd_index: 0, tag: 282, kind: TiffFieldType::Rational, values: TiffValues::Rational(vec![(72, 1)]) },
        TiffMutation::SetTag { ifd_index: 0, tag: 700, kind: TiffFieldType::Undefined, values: TiffValues::Undefined(vec![0xde, 0xad]) },
        TiffMutation::SetTag { ifd_index: 0, tag: 33421, kind: TiffFieldType::SRational, values: TiffValues::SRational(vec![(-3, 10)]) },
        TiffMutation::SetTag { ifd_index: 0, tag: 65001, kind: TiffFieldType::Float, values: TiffValues::Float(vec![1.5, -2.25]) },
        TiffMutation::SetTag { ifd_index: 0, tag: 65002, kind: TiffFieldType::Double, values: TiffValues::Double(vec![3.14159265358979]) },
        TiffMutation::RemoveTag { ifd_index: 0, tag: 296 },
        TiffMutation::SetPixels { pixels: vec![9u8; 16] },
    ]
}
//#endregion 🔖️DemoCases

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;

    //#region 🔖️Fixtures
    fn short_tag(tag: u16, v: u16) -> TiffTag {
        TiffTag { tag, kind: TiffFieldType::Short, values: TiffValues::Short(vec![v]) }
    }

    fn base_snapshot() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![TiffIfd {
                entries: vec![
                    TiffTag { tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageWidth
                    TiffTag { tag: 257, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) }, // ImageLength
                    short_tag(296, 2),                                                                  // ResolutionUnit
                ],
            }],
            pixels: vec![0u8; 4 * 4 * 4],
        }
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️FieldSweepFixtures
    /// 🧬️ `sweep_a`/`sweep_b` differ in EVERY mutable field. `ifds` (index-keyed) and, within
    /// the surviving `ifds[0]`, `entries` (tag-id-keyed) are both deliberately DIFFERENT
    /// length/membership — the recipe's own documented workaround for the structural
    /// "same-length `between()` can show removed XOR added, never both from one call" trap
    /// (see F1's `f1-closer-report.md` §4.4): the IFD-level triple needs the split-across-
    /// directions workaround (positional pairwise matching), while the TAG-level triple is
    /// id-keyed via a `BTreeMap` union, so it genuinely shows removed+modified+added from a
    /// SINGLE `between()` call — no split needed there.
    fn sweep_a() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::LittleEndian,
            ifds: vec![
                TiffIfd { entries: vec![short_tag(300, 1), short_tag(301, 9)] },                                                       // tag 300 survives+changes, 301 removed
                TiffIfd { entries: vec![TiffTag { tag: 302, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("gone".into()) }] }, // whole IFD removed in b
            ],
            pixels: vec![0u8, 0, 0, 255, 1, 1, 1, 255],
        }
    }

    fn sweep_b() -> TiffSnapshot {
        TiffSnapshot {
            schema: "stdio.tiff".into(),
            byte_order: TiffByteOrder::BigEndian,
            ifds: vec![TiffIfd { entries: vec![short_tag(300, 2), TiffTag { tag: 303, kind: TiffFieldType::Long, values: TiffValues::Long(vec![42]) }] }], // 300 changed, 303 added
            pixels: vec![9u8, 9, 9, 255],
        }
    }
    //#endregion 🔖️FieldSweepFixtures

    //#region 🔖️mutation_diff_law
    fn assert_mutation_diff_law(base: &TiffSnapshot, mutation: TiffMutation) {
        let expected_diff = mutation.diff(base);
        let mut applied_snapshot = base.clone();
        let returned_diff = apply_tiff_mutation(&mut applied_snapshot, &mutation);
        assert_eq!(returned_diff, expected_diff, "apply_tiff_mutation must return mutation.diff(base) for {mutation:?}");
        assert_eq!(expected_diff.apply(base), applied_snapshot, "diff.apply(base) must equal the imperative mutation result for {mutation:?}");
    }

    fn all_variants(base: &TiffSnapshot) -> Vec<TiffMutation> {
        vec![
            TiffMutation::NoMutation,
            TiffMutation::SetSnapshot {
                snapshot: {
                    let mut s = base.clone();
                    s.byte_order = TiffByteOrder::BigEndian;
                    s
                },
            },
            TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian },
            TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(270, 1)] } },
            TiffMutation::RemoveIfd { index: 0 },
            TiffMutation::SetTag { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![3]) }, // modify existing
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) }, // create new
            TiffMutation::RemoveTag { ifd_index: 0, tag: 296 },
            TiffMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
            // Out-of-range targets: graceful no-ops, still law-compliant.
            TiffMutation::RemoveIfd { index: 99 },
            TiffMutation::RemoveTag { ifd_index: 99, tag: 1 },
            TiffMutation::RemoveTag { ifd_index: 0, tag: 9999 },
            TiffMutation::SetTag { ifd_index: 99, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) },
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
            apply_tiff_mutation(&mut snap, &m);
            for inv in m.inverse(&base) {
                apply_tiff_mutation(&mut snap, &inv);
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
    fn assert_absorb_law(base: &TiffSnapshot, m1: TiffMutation, m2: TiffMutation) {
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

        // IFD-level (index-keyed), Insert+Remove-before: insert a new IFD at 1 -> [ifd0,new],
        // then remove index 0 -> [new] lands at final index 0 (the recipe's own canonical
        // shift case).
        assert_absorb_law(&base, TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(1, 1)] } }, TiffMutation::RemoveIfd { index: 0 });

        // IFD-level, Insert+Insert-same-index: both survive.
        assert_absorb_law(&base, TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(2, 2)] } }, TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(3, 3)] } });

        // Tag-level (id-keyed), Add+SetField: the second mutation patches directly into the
        // still-pending added tag.
        assert_absorb_law(
            &base,
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("orig".into()) },
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("patched".into()) },
        );

        // Tag-level, Modify+Remove: a pending field patch on a since-removed base tag vanishes.
        assert_absorb_law(&base, TiffMutation::SetTag { ifd_index: 0, tag: 296, kind: TiffFieldType::Short, values: TiffValues::Short(vec![7]) }, TiffMutation::RemoveTag { ifd_index: 0, tag: 296 });

        // Tag-level, Add then annihilate the very same add.
        assert_absorb_law(&base, TiffMutation::SetTag { ifd_index: 0, tag: 317, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) }, TiffMutation::RemoveTag { ifd_index: 0, tag: 317 });

        // Two unrelated scalar sets absorb via LWW.
        assert_absorb_law(&base, TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian }, TiffMutation::SetByteOrder { byte_order: TiffByteOrder::LittleEndian });
    }

    #[test]
    fn absorb_law_associativity() {
        let base = base_snapshot();
        let d1 = TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a".into()) }.diff(&base);
        let s1 = d1.apply(&base);
        let d2 = TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("a2".into()) }.diff(&s1);
        let s2 = d2.apply(&s1);
        let d3 = TiffMutation::RemoveTag { ifd_index: 0, tag: 296 }.diff(&s2);
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
        b.byte_order = TiffByteOrder::BigEndian;
        b.ifds[0].entries.push(TiffTag { tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("Extra".into()) });
        b.pixels = vec![5u8; a.pixels.len()];

        let d = TiffDiff::between(&a, &b);
        assert_eq!(d.apply(&a), b, "between(a,b).apply(a) must equal b");
        let d_rev = TiffDiff::between(&b, &a);
        assert_eq!(d_rev.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(TiffDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️codec_retention_law
    #[test]
    fn codec_retention_law() {
        let bytes = crate::artifacts::tiff::engine::encode_tiff(&base_snapshot()).expect("encode synthetic fixture");
        let decoded = crate::artifacts::tiff::engine::decode_tiff(&bytes).expect("decode fixture");
        let reencoded = crate::artifacts::tiff::engine::encode_tiff(&decoded).expect("re-encode fixture");
        let redecoded = crate::artifacts::tiff::engine::decode_tiff(&reencoded).expect("re-decode fixture");
        // Engine's EncodeScopeNote: encode always canonicalizes to a single IFD/single strip —
        // pixel CONTENT + carried non-core tags are the retained invariant.
        assert_eq!(decoded.width(), redecoded.width());
        assert_eq!(decoded.height(), redecoded.height());
        assert_eq!(decoded.pixels, redecoded.pixels);
        assert_eq!(decoded.tag(296), redecoded.tag(296), "carried non-core tag must survive a second round trip");
    }
    //#endregion 🔖️codec_retention_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_covers_every_mutable_field() {
        let a = sweep_a();
        let b = sweep_b();

        let forward = TiffDiff::between(&a, &b);
        assert_eq!(forward.apply(&a), b, "between(a,b).apply(a) must equal b");
        let backward = TiffDiff::between(&b, &a);
        assert_eq!(backward.apply(&b), a, "between(b,a).apply(b) must equal a");
        assert!(TiffDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

        assert_eq!(forward.byte_order, Some(TiffByteOrder::BigEndian));
        assert_eq!(backward.byte_order, Some(TiffByteOrder::LittleEndian));
        assert!(forward.pixels.is_some(), "pixels must be diffed");
        assert!(backward.pixels.is_some());

        // ifds (index-keyed): forward shows removed(IFD1)+modified(IFD0); backward shows
        // added(IFD1)+modified(IFD0) — the split-across-both-directions workaround.
        let fwd_ifds = forward.ifds.as_ref().expect("ifds diff present (forward)");
        assert_eq!(fwd_ifds.removed, vec![1]);
        assert_eq!(fwd_ifds.modified.len(), 1);
        assert!(fwd_ifds.added.is_empty());
        let bwd_ifds = backward.ifds.as_ref().expect("ifds diff present (backward)");
        assert!(bwd_ifds.removed.is_empty());
        assert_eq!(bwd_ifds.modified.len(), 1);
        assert_eq!(bwd_ifds.added.len(), 1);

        // entries within ifds[0] (tag-id-keyed): a SINGLE between() call genuinely shows
        // removed+modified+added together (id-keyed union, no positional-pairing trap).
        let fwd_entries = &fwd_ifds.modified[0].diff;
        assert_eq!(fwd_entries.removed, vec![301]);
        assert_eq!(fwd_entries.modified.len(), 1);
        assert_eq!(fwd_entries.modified[0].tag, 300);
        assert_eq!(fwd_entries.added.len(), 1);
        assert_eq!(fwd_entries.added[0].tag, 303);

        let bwd_entries = &bwd_ifds.modified[0].diff;
        assert_eq!(bwd_entries.removed, vec![303]);
        assert_eq!(bwd_entries.modified.len(), 1);
        assert_eq!(bwd_entries.modified[0].tag, 300);
        assert_eq!(bwd_entries.added.len(), 1);
        assert_eq!(bwd_entries.added[0].tag, 301);
    }
    //#endregion 🔖️field_sweep

    #[test]
    fn out_of_range_mutation_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveIfd { index: 42 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag { ifd_index: 42, tag: 1 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::RemoveTag { ifd_index: 0, tag: 9999 });
        assert_eq!(snap, base);
        apply_tiff_mutation(&mut snap, &TiffMutation::SetTag { ifd_index: 42, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1]) });
        assert_eq!(snap, base);
    }

    //#region 🔖️op_text_binary_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `TiffMutation` grammar —
    /// exercises every variant incl. `SetTag`/`SetSnapshot`'s bare `TiffValues` payload across
    /// every one of the 12 field-type variants (`Rational`/`SRational` pair lists, `Ascii`/`Byte`/
    /// `Undefined` hex, signed and unsigned numeric lists, `Float`/`Double`).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            TiffMutation::NoMutation,
            TiffMutation::SetSnapshot { snapshot: base.clone() },
            TiffMutation::SetByteOrder { byte_order: TiffByteOrder::BigEndian },
            TiffMutation::InsertIfd { index: 1, ifd: TiffIfd { entries: vec![short_tag(270, 1)] } },
            TiffMutation::RemoveIfd { index: 0 },
            TiffMutation::SetTag { ifd_index: 0, tag: 256, kind: TiffFieldType::Long, values: TiffValues::Long(vec![4]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 258, kind: TiffFieldType::Short, values: TiffValues::Short(vec![8, 8, 8]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 315, kind: TiffFieldType::Ascii, values: TiffValues::Ascii("An Author".into()) },
            TiffMutation::SetTag { ifd_index: 0, tag: 282, kind: TiffFieldType::Rational, values: TiffValues::Rational(vec![(72, 1), (0, 1)]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 700, kind: TiffFieldType::Undefined, values: TiffValues::Undefined(vec![0xde, 0xad, 0xbe, 0xef]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 1, kind: TiffFieldType::Byte, values: TiffValues::Byte(vec![1, 2, 3]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 2, kind: TiffFieldType::SByte, values: TiffValues::SByte(vec![-1, -2, 3]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 3, kind: TiffFieldType::SShort, values: TiffValues::SShort(vec![-100, 200]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 4, kind: TiffFieldType::SLong, values: TiffValues::SLong(vec![-100000]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 5, kind: TiffFieldType::SRational, values: TiffValues::SRational(vec![(-3, 10)]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 6, kind: TiffFieldType::Float, values: TiffValues::Float(vec![1.5, -2.25]) },
            TiffMutation::SetTag { ifd_index: 0, tag: 7, kind: TiffFieldType::Double, values: TiffValues::Double(vec![3.14159265358979]) },
            TiffMutation::RemoveTag { ifd_index: 0, tag: 296 },
            TiffMutation::SetPixels { pixels: vec![9u8; base.pixels.len()] },
            // Out-of-range targets: still valid grammar, no special-casing needed.
            TiffMutation::RemoveIfd { index: 99 },
            TiffMutation::RemoveTag { ifd_index: 99, tag: 1 },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = TiffMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = TiffMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️op_text_binary_roundtrip_law
}
//#endregion Tests
