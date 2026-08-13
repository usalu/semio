//! 🧬️ PptxMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, index-aware.

use crate::artifacts::pptx::schema::diff::{
    diff_insert_shape, diff_insert_slide, diff_move_slide, diff_remove_shape, diff_remove_slide,
    diff_set_shape_position, diff_set_shape_text, diff_set_snapshot, PptxDiff,
};
use crate::artifacts::pptx::schema::diff::{
    dec_ct_entry, dec_list, dec_owner_rels, dec_paragraph, dec_part, dec_shape, dec_slide, dec_str, dec_transform,
    enc_ct_entry, enc_list, enc_owner_rels, enc_paragraph, enc_part, enc_shape, enc_slide, enc_str, enc_transform,
    split_top_level, strip_brackets,
};
use crate::artifacts::pptx::schema::diff::{
    dec_paragraph_bin, dec_part_bin, dec_rel_bin, dec_shape_bin, dec_slide_bin, dec_transform_bin, enc_paragraph_bin, enc_part_bin,
    enc_rel_bin, enc_shape_bin, enc_slide_bin, enc_transform_bin, read_str_lp, write_str_lp,
};
use crate::artifacts::pptx::schema::snapshot::{PptxParagraph, PptxPresentation, PptxShape, PptxSlide, PptxTransform};
use crate::artifacts::pptx::PptxSnapshot;
use crate::artifacts::zip::opc::OpcPackage;
use protocol::{Mutation, OpText};
#[cfg(test)]
use protocol::OpBinary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pptx`. Addresses `presentation.slides` by index
/// (slide order matters -- see `MoveSlide`) and, within a slide, `shapes` by
/// `(slide_index, shape_index)` -- a flat two-level address is sufficient since PresentationML
/// slides don't nest arbitrarily (grouped shapes fall back to `PptxShape::Other`, see the
/// snapshot module's doc comment).
/// 🧪️ F6 CONFIRMED: `#[derive(dsl::DslOps)]` on this enum fails to compile — real `cargo check`
/// error: `the trait bound PptxSnapshot: DslField is not satisfied` at `SetSnapshot{snapshot}`
/// (`PptxSnapshot` embeds `PptxShape`, a data-carrying enum, and the generic `IndexedTripleDiff`/
/// `NamedTripleDiff` collection engine `PptxPresentation`/`OpcPackage` route through — see the diff
/// file's `HandcraftedDiffCodec` doc comment for the full three-reason citation), plus a SECOND,
/// independent hit at `InsertShape{shape: PptxShape}`/`InsertSlide{slide: PptxSlide}` (`PptxShape`/
/// `PptxSlide` carry the same enum-shaped payload DIRECTLY as a variant field, mirroring
/// `SvgMutation::InsertElement`'s `node: XmlNode` blocker). `OpText`/`OpBinary` hand-rolled below,
/// reusing `PptxDiff`'s `pub(crate)` grammar primitives.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PptxMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PptxSnapshot,
    },
    /// ➕️ Inserts `slide` at `index` (FINAL state).
    InsertSlide {
        index: usize,
        slide: PptxSlide,
    },
    /// ➖️ Removes the slide at `index` (BASE-state index).
    RemoveSlide {
        index: usize,
    },
    /// 🔀️ Moves the slide at BASE-state index `from` to FINAL-state index `to`.
    MoveSlide {
        from: usize,
        to: usize,
    },
    /// ➕️ Inserts `shape` at `shape_index` (FINAL state) on the slide at `slide_index`.
    InsertShape {
        slide_index: usize,
        shape_index: usize,
        shape: PptxShape,
    },
    /// ➖️ Removes the shape at `shape_index` (BASE-state index) on the slide at `slide_index`.
    RemoveShape {
        slide_index: usize,
        shape_index: usize,
    },
    /// ✍️ Replaces a `TextBox`/`Placeholder` shape's `text_frame` (no-op on `Picture`/`Other`).
    SetShapeText {
        slide_index: usize,
        shape_index: usize,
        text_frame: Vec<PptxParagraph>,
    },
    /// 📐️ Sets a shape's `position` (no-op on `Other`, which has none).
    SetShapePosition {
        slide_index: usize,
        shape_index: usize,
        position: PptxTransform,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub fn apply_pptx_mutation(snapshot: &mut PptxSnapshot, mutation: &PptxMutation) -> PptxDiff {
    let diff = Mutation::diff(mutation, snapshot);
    *snapshot = protocol::MutationDiff::apply(&diff, snapshot);
    diff
}
//#endregion 🔖️Apply

//#region 🔖️Helpers
fn slide_at<'a>(base: &'a PptxSnapshot, index: usize) -> Option<&'a PptxSlide> {
    base.presentation.slides.get(index)
}

fn shape_at<'a>(base: &'a PptxSnapshot, slide_index: usize, shape_index: usize) -> Option<&'a PptxShape> {
    base.presentation.slides.get(slide_index)?.shapes.get(shape_index)
}
//#endregion 🔖️Helpers

//#region 🔖️MutationTrait
impl Mutation<PptxSnapshot> for PptxMutation {
    type Diff = PptxDiff;

    fn diff(&self, base: &PptxSnapshot) -> Self::Diff {
        match self {
            PptxMutation::NoMutation => PptxDiff::default(),
            PptxMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            PptxMutation::InsertSlide { index, slide } => diff_insert_slide(*index, slide.clone()),
            PptxMutation::RemoveSlide { index } => diff_remove_slide(*index),
            PptxMutation::MoveSlide { from, to } => diff_move_slide(&base.presentation, *from, *to),
            PptxMutation::InsertShape { slide_index, shape_index, shape } => diff_insert_shape(*slide_index, *shape_index, shape.clone()),
            PptxMutation::RemoveShape { slide_index, shape_index } => diff_remove_shape(*slide_index, *shape_index),
            PptxMutation::SetShapeText { slide_index, shape_index, text_frame } => {
                diff_set_shape_text(&base.presentation, *slide_index, *shape_index, text_frame.clone())
            }
            PptxMutation::SetShapePosition { slide_index, shape_index, position } => {
                diff_set_shape_position(&base.presentation, *slide_index, *shape_index, *position)
            }
        }
    }

    fn inverse(&self, base: &PptxSnapshot) -> Vec<Self> {
        match self {
            PptxMutation::NoMutation => vec![PptxMutation::NoMutation],
            PptxMutation::SetSnapshot { .. } => vec![PptxMutation::SetSnapshot { snapshot: base.clone() }],
            PptxMutation::InsertSlide { index, .. } => vec![PptxMutation::RemoveSlide { index: *index }],
            PptxMutation::RemoveSlide { index } => match slide_at(base, *index) {
                Some(slide) => vec![PptxMutation::InsertSlide { index: *index, slide: slide.clone() }],
                None => vec![PptxMutation::NoMutation],
            },
            PptxMutation::MoveSlide { from, to } => {
                // 🧭️ After moving `from -> to`, the slide ends up at `min(to, len-1)` (per
                // `apply_indexed`'s own remove-then-insert semantics: one item shorter after the
                // removal, then inserted at `min(to, that_shorter_len)`) -- moving it back to
                // `from` restores the original order exactly.
                let len = base.presentation.slides.len();
                let final_pos = (*to).min(len.saturating_sub(1));
                vec![PptxMutation::MoveSlide { from: final_pos, to: *from }]
            }
            PptxMutation::InsertShape { slide_index, shape_index, .. } => vec![PptxMutation::RemoveShape { slide_index: *slide_index, shape_index: *shape_index }],
            PptxMutation::RemoveShape { slide_index, shape_index } => match shape_at(base, *slide_index, *shape_index) {
                Some(shape) => vec![PptxMutation::InsertShape { slide_index: *slide_index, shape_index: *shape_index, shape: shape.clone() }],
                None => vec![PptxMutation::NoMutation],
            },
            PptxMutation::SetShapeText { slide_index, shape_index, .. } => {
                let old = shape_at(base, *slide_index, *shape_index).and_then(|s| match s {
                    PptxShape::TextBox { text_frame, .. } | PptxShape::Placeholder { text_frame, .. } => Some(text_frame.clone()),
                    _ => None,
                });
                match old {
                    Some(text_frame) => vec![PptxMutation::SetShapeText { slide_index: *slide_index, shape_index: *shape_index, text_frame }],
                    None => vec![PptxMutation::NoMutation],
                }
            }
            PptxMutation::SetShapePosition { slide_index, shape_index, .. } => {
                let old = shape_at(base, *slide_index, *shape_index).and_then(|s| match s {
                    PptxShape::TextBox { position, .. } | PptxShape::Picture { position, .. } | PptxShape::Placeholder { position, .. } => Some(*position),
                    PptxShape::Other { .. } => None,
                });
                match old {
                    Some(position) => vec![PptxMutation::SetShapePosition { slide_index: *slide_index, shape_index: *shape_index, position }],
                    None => vec![PptxMutation::NoMutation],
                }
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `PptxMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `PptxDiff`'s `pub(crate)` grammar primitives
/// (`enc_shape`/`enc_slide`/`enc_transform`/`split_top_level`/`encode_option`/...) rather than
/// duplicating them a second time in this file (same intra-artifact reuse `SvgMutation` established
/// for `SvgDiff`'s primitives). Grammar: `keyword arg=value ...` (space-separated, same shape the
/// derive's own handcrafted-wrapper convention uses), one match arm per variant.
//#region 🔖️SnapshotCodec
/// 🌳 Full `OpcPackage` (used only by `SetSnapshot`'s `snapshot` payload, never by `PptxDiff` --
/// diffs only ever carry the sparse `PptxOpcDiff`): `[parts,defaults,overrides,relationships]`.
/// `relationships` (a `HashMap`) is sorted by owner key first for deterministic `encode_op` output
/// (`OpBinary`'s own LAW: "encoding is deterministic -- byte-identical output for equal operations").
fn enc_opc_package(p: &OpcPackage) -> String {
    let mut owners: Vec<&String> = p.relationships.keys().collect();
    owners.sort();
    let rels = owners.iter().map(|owner| enc_owner_rels(&(owner.to_string(), p.relationships[*owner].clone()))).collect::<Vec<_>>();
    format!(
        "[{},{},{},{}]",
        enc_list(&p.parts, enc_part),
        enc_list(&p.content_types.defaults, enc_ct_entry),
        enc_list(&p.content_types.overrides, enc_ct_entry),
        format!("[{}]", rels.join(",")),
    )
}
fn dec_opc_package(s: &str) -> Result<OpcPackage, String> {
    let parts_top = split_top_level(strip_brackets(s)?, ',');
    let [parts, defaults, overrides, rels] = parts_top.as_slice() else { return Err(format!("opc package: expected 4 fields, got {}", parts_top.len())) };
    let mut package = OpcPackage::empty();
    package.parts = dec_list(parts, dec_part)?;
    package.content_types.defaults = dec_list(defaults, dec_ct_entry)?;
    package.content_types.overrides = dec_list(overrides, dec_ct_entry)?;
    for (owner, list) in dec_list(rels, dec_owner_rels)? {
        package.relationships.insert(owner, list);
    }
    Ok(package)
}
/// 🌳 Full `PptxSnapshot`: `[schema,opc,slides]` -- `presentation` collapses to its own single
/// field (`slides: Vec<PptxSlide>`), same convention `enc_slide`/`enc_paragraph` use.
fn enc_snapshot(s: &PptxSnapshot) -> String {
    format!("[{},{},{}]", enc_str(&s.schema), enc_opc_package(&s.opc), enc_list(&s.presentation.slides, enc_slide))
}
fn dec_snapshot(s: &str) -> Result<PptxSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, opc, slides] = parts.as_slice() else { return Err(format!("snapshot: expected 3 fields, got {}", parts.len())) };
    Ok(PptxSnapshot {
        schema: dec_str(schema)?,
        opc: dec_opc_package(opc)?,
        presentation: PptxPresentation { slides: dec_list(slides, dec_slide)? },
    })
}
//#endregion 🔖️SnapshotCodec

fn print_pptx_mutation(m: &PptxMutation) -> String {
    match m {
        PptxMutation::NoMutation => "no-mutation".to_string(),
        PptxMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_snapshot(snapshot)),
        PptxMutation::InsertSlide { index, slide } => format!("insert-slide index={index} slide={}", enc_slide(slide)),
        PptxMutation::RemoveSlide { index } => format!("remove-slide index={index}"),
        PptxMutation::MoveSlide { from, to } => format!("move-slide from={from} to={to}"),
        PptxMutation::InsertShape { slide_index, shape_index, shape } => {
            format!("insert-shape slide-index={slide_index} shape-index={shape_index} shape={}", enc_shape(shape))
        }
        PptxMutation::RemoveShape { slide_index, shape_index } => format!("remove-shape slide-index={slide_index} shape-index={shape_index}"),
        PptxMutation::SetShapeText { slide_index, shape_index, text_frame } => {
            format!("set-shape-text slide-index={slide_index} shape-index={shape_index} text-frame={}", enc_list(text_frame, enc_paragraph))
        }
        PptxMutation::SetShapePosition { slide_index, shape_index, position } => {
            format!("set-shape-position slide-index={slide_index} shape-index={shape_index} position={}", enc_transform(position))
        }
    }
}
fn parse_pptx_mutation(line: &str) -> Result<PptxMutation, String> {
    if line == "no-mutation" {
        return Ok(PptxMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|tok| tok.split_once('=').ok_or_else(|| format!("pptx mutation: bad arg token {tok:?}")))
        .collect::<Result<Vec<_>, String>>()?
        .into_iter()
        .collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("pptx mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(PptxMutation::SetSnapshot { snapshot: dec_snapshot(arg("snapshot")?)? }),
        "insert-slide" => Ok(PptxMutation::InsertSlide { index: usize_arg("index")?, slide: dec_slide(arg("slide")?)? }),
        "remove-slide" => Ok(PptxMutation::RemoveSlide { index: usize_arg("index")? }),
        "move-slide" => Ok(PptxMutation::MoveSlide { from: usize_arg("from")?, to: usize_arg("to")? }),
        "insert-shape" => Ok(PptxMutation::InsertShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, shape: dec_shape(arg("shape")?)? }),
        "remove-shape" => Ok(PptxMutation::RemoveShape { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")? }),
        "set-shape-text" => Ok(PptxMutation::SetShapeText {
            slide_index: usize_arg("slide-index")?,
            shape_index: usize_arg("shape-index")?,
            text_frame: dec_list(arg("text-frame")?, dec_paragraph)?,
        }),
        "set-shape-position" => Ok(PptxMutation::SetShapePosition { slide_index: usize_arg("slide-index")?, shape_index: usize_arg("shape-index")?, position: dec_transform(arg("position")?)? }),
        other => Err(format!("pptx mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for PptxMutation {
    fn print_op(&self) -> String {
        print_pptx_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_pptx_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ FG-wave: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📜️docx/…/🧬️mutations/🦀️component.rs`'s own `enc_docx_snapshot_bin`/`enc_opc_package_bin`
/// shape, reusing `store::pack_rt::write_varint_u64`/`store::ByteReader` plus `PptxDiff`'s own
/// `write_str_lp`/`read_str_lp`/`enc_shape_bin`/`dec_shape_bin`/`enc_slide_bin`/`dec_slide_bin`/
/// `enc_transform_bin`/`dec_transform_bin`/`enc_part_bin`/`dec_part_bin`/`enc_rel_bin`/`dec_rel_bin`
/// (`../🔺️diff/🦀️component.rs`, `pub(crate)` to this artifact).
///
/// 🌱 Full (non-diff) `OpcPackage`/`PptxSnapshot` binary codecs -- only `SetSnapshot`'s
/// whole-payload encoding needs these, mirroring this file's own `enc_opc_package`/`enc_snapshot`
/// text forms above. Relationship owners sorted for a deterministic encoding, same `HashMap`
/// -iteration-order caveat those text forms document.
fn enc_opc_package_bin(pkg: &OpcPackage, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, pkg.parts.len() as u64);
    for p in &pkg.parts {
        enc_part_bin(p, out);
    }
    store::pack_rt::write_varint_u64(out, pkg.content_types.defaults.len() as u64);
    for e in &pkg.content_types.defaults {
        write_str_lp(out, &e.0);
        write_str_lp(out, &e.1);
    }
    store::pack_rt::write_varint_u64(out, pkg.content_types.overrides.len() as u64);
    for e in &pkg.content_types.overrides {
        write_str_lp(out, &e.0);
        write_str_lp(out, &e.1);
    }
    let mut owners: Vec<&String> = pkg.relationships.keys().collect();
    owners.sort();
    store::pack_rt::write_varint_u64(out, owners.len() as u64);
    for owner in owners {
        write_str_lp(out, owner);
        let list = &pkg.relationships[owner];
        store::pack_rt::write_varint_u64(out, list.len() as u64);
        for r in list {
            enc_rel_bin(r, out);
        }
    }
}
fn dec_opc_package_bin(reader: &mut store::ByteReader<'_>) -> Result<OpcPackage, String> {
    let part_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut parts = Vec::with_capacity(part_count as usize);
    for _ in 0..part_count {
        parts.push(dec_part_bin(reader)?);
    }
    let mut package = OpcPackage::empty();
    package.parts = parts;
    let default_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    for _ in 0..default_count {
        let k = read_str_lp(reader)?;
        let v = read_str_lp(reader)?;
        package.content_types.defaults.push((k, v));
    }
    let override_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    for _ in 0..override_count {
        let k = read_str_lp(reader)?;
        let v = read_str_lp(reader)?;
        package.content_types.overrides.push((k, v));
    }
    let owner_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut relationships = HashMap::with_capacity(owner_count as usize);
    for _ in 0..owner_count {
        let owner = read_str_lp(reader)?;
        let rel_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
        let mut list = Vec::with_capacity(rel_count as usize);
        for _ in 0..rel_count {
            list.push(dec_rel_bin(reader)?);
        }
        relationships.insert(owner, list);
    }
    package.relationships = relationships;
    Ok(package)
}
/// 🌳 Full `PptxSnapshot`: `[schema,opc,slides]`, mirroring `enc_snapshot`'s text form above.
fn enc_snapshot_bin(s: &PptxSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    enc_opc_package_bin(&s.opc, out);
    store::pack_rt::write_varint_u64(out, s.presentation.slides.len() as u64);
    for slide in &s.presentation.slides {
        enc_slide_bin(slide, out);
    }
}
fn dec_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<PptxSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let opc = dec_opc_package_bin(reader)?;
    let slide_count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut slides = Vec::with_capacity(slide_count as usize);
    for _ in 0..slide_count {
        slides.push(dec_slide_bin(reader)?);
    }
    Ok(PptxSnapshot { schema, opc, presentation: PptxPresentation { slides } })
}
fn enc_text_frame_bin(ps: &[PptxParagraph], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, ps.len() as u64);
    for p in ps {
        enc_paragraph_bin(p, out);
    }
}
fn dec_text_frame_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<PptxParagraph>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(dec_paragraph_bin(reader)?);
    }
    Ok(out)
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ FG-wave: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F1's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `PptxMutation` variant ordinal, in the same 0-8 order `print_pptx_mutation`'s own keyword
/// match uses.
impl protocol::OpBinary for PptxMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            PptxMutation::NoMutation => 0,
            PptxMutation::SetSnapshot { .. } => 1,
            PptxMutation::InsertSlide { .. } => 2,
            PptxMutation::RemoveSlide { .. } => 3,
            PptxMutation::MoveSlide { .. } => 4,
            PptxMutation::InsertShape { .. } => 5,
            PptxMutation::RemoveShape { .. } => 6,
            PptxMutation::SetShapeText { .. } => 7,
            PptxMutation::SetShapePosition { .. } => 8,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            PptxMutation::NoMutation => {}
            PptxMutation::SetSnapshot { snapshot } => enc_snapshot_bin(snapshot, &mut out),
            PptxMutation::InsertSlide { index, slide } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_slide_bin(slide, &mut out);
            }
            PptxMutation::RemoveSlide { index } => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            PptxMutation::MoveSlide { from, to } => {
                store::pack_rt::write_varint_u64(&mut out, *from as u64);
                store::pack_rt::write_varint_u64(&mut out, *to as u64);
            }
            PptxMutation::InsertShape { slide_index, shape_index, shape } => {
                store::pack_rt::write_varint_u64(&mut out, *slide_index as u64);
                store::pack_rt::write_varint_u64(&mut out, *shape_index as u64);
                enc_shape_bin(shape, &mut out);
            }
            PptxMutation::RemoveShape { slide_index, shape_index } => {
                store::pack_rt::write_varint_u64(&mut out, *slide_index as u64);
                store::pack_rt::write_varint_u64(&mut out, *shape_index as u64);
            }
            PptxMutation::SetShapeText { slide_index, shape_index, text_frame } => {
                store::pack_rt::write_varint_u64(&mut out, *slide_index as u64);
                store::pack_rt::write_varint_u64(&mut out, *shape_index as u64);
                enc_text_frame_bin(text_frame, &mut out);
            }
            PptxMutation::SetShapePosition { slide_index, shape_index, position } => {
                store::pack_rt::write_varint_u64(&mut out, *slide_index as u64);
                store::pack_rt::write_varint_u64(&mut out, *shape_index as u64);
                enc_transform_bin(position, &mut out);
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
            0 => Ok(PptxMutation::NoMutation),
            1 => {
                let snapshot = dec_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))?;
                Ok(PptxMutation::SetSnapshot { snapshot })
            }
            2 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let slide = dec_slide_bin(&mut reader).map_err(|e| malformed("op slide", reader.position(), e))?;
                Ok(PptxMutation::InsertSlide { index, slide })
            }
            3 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                Ok(PptxMutation::RemoveSlide { index })
            }
            4 => {
                let from = reader.read_varint_u64().map_err(|e| malformed("op from", reader.position(), e.to_string()))? as usize;
                let to = reader.read_varint_u64().map_err(|e| malformed("op to", reader.position(), e.to_string()))? as usize;
                Ok(PptxMutation::MoveSlide { from, to })
            }
            5 => {
                let slide_index = reader.read_varint_u64().map_err(|e| malformed("op slide_index", reader.position(), e.to_string()))? as usize;
                let shape_index = reader.read_varint_u64().map_err(|e| malformed("op shape_index", reader.position(), e.to_string()))? as usize;
                let shape = dec_shape_bin(&mut reader).map_err(|e| malformed("op shape", reader.position(), e))?;
                Ok(PptxMutation::InsertShape { slide_index, shape_index, shape })
            }
            6 => {
                let slide_index = reader.read_varint_u64().map_err(|e| malformed("op slide_index", reader.position(), e.to_string()))? as usize;
                let shape_index = reader.read_varint_u64().map_err(|e| malformed("op shape_index", reader.position(), e.to_string()))? as usize;
                Ok(PptxMutation::RemoveShape { slide_index, shape_index })
            }
            7 => {
                let slide_index = reader.read_varint_u64().map_err(|e| malformed("op slide_index", reader.position(), e.to_string()))? as usize;
                let shape_index = reader.read_varint_u64().map_err(|e| malformed("op shape_index", reader.position(), e.to_string()))? as usize;
                let text_frame = dec_text_frame_bin(&mut reader).map_err(|e| malformed("op text_frame", reader.position(), e))?;
                Ok(PptxMutation::SetShapeText { slide_index, shape_index, text_frame })
            }
            8 => {
                let slide_index = reader.read_varint_u64().map_err(|e| malformed("op slide_index", reader.position(), e.to_string()))? as usize;
                let shape_index = reader.read_varint_u64().map_err(|e| malformed("op shape_index", reader.position(), e.to_string()))? as usize;
                let position = dec_transform_bin(&mut reader).map_err(|e| malformed("op position", reader.position(), e))?;
                Ok(PptxMutation::SetShapePosition { slide_index, shape_index, position })
            }
            other => Err(malformed("op tag", 1, format!("unknown PptxMutation tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ FG-wave: representative `PptxMutation` values -- one per variant -- the single source of
/// truth reused by this file's own `mutation_diff_law`/`inverse_law`/`op_text_binary_roundtrip_law`
/// tests AND by `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests, same shape `📜️docx/…/🧬️mutations/🦀️component.rs`'s own
/// `demo_mutation_cases()` establishes.
pub(crate) fn demo_fixture() -> PptxSnapshot {
    crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(PptxPresentation {
        slides: vec![
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
            PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
        ],
    })
}

pub(crate) fn demo_mutation_cases() -> Vec<PptxMutation> {
    vec![
        PptxMutation::NoMutation,
        PptxMutation::SetSnapshot { snapshot: demo_fixture() },
        PptxMutation::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform::default() }] } },
        PptxMutation::RemoveSlide { index: 0 },
        PptxMutation::MoveSlide { from: 0, to: 1 },
        PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } } },
        PptxMutation::RemoveShape { slide_index: 0, shape_index: 0 },
        PptxMutation::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("z")] },
        PptxMutation::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::pptx::schema::diff::{PptxOpcPartDiff, PptxShapeDiff};
    use crate::artifacts::pptx::schema::snapshot::{PptxPresentation, PptxRun};
    use crate::artifacts::zip::opc::{OpcPackage, OpcRelationship, OpcTargetMode, REL_TYPE_OFFICE_DOCUMENT, RELS_CONTENT_TYPE};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    fn fixture() -> PptxSnapshot {
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(PptxPresentation {
            slides: vec![
                PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("first")], position: PptxTransform { x: 0, y: 0, cx: 100, cy: 100 } }] },
                PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("second")], position: PptxTransform::default() }] },
            ],
        })
    }

    #[test]
    fn insert_then_remove_slide_apply_and_inverse() {
        let base = fixture();
        let insert = PptxMutation::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("inserted")], position: PptxTransform::default() }] } };
        let mut after = base.clone();
        apply_pptx_mutation(&mut after, &insert);
        assert_eq!(after.presentation.slides.len(), 3);

        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_pptx_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn remove_slide_inverse_restores_removed_slide() {
        let base = fixture();
        let remove = PptxMutation::RemoveSlide { index: 0 };
        let mut after = base.clone();
        apply_pptx_mutation(&mut after, &remove);
        assert_eq!(after.presentation.slides.len(), 1);
        for inv in Mutation::inverse(&remove, &base) {
            apply_pptx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn move_slide_apply_and_inverse() {
        let base = fixture();
        let mv = PptxMutation::MoveSlide { from: 0, to: 1 };
        let mut after = base.clone();
        apply_pptx_mutation(&mut after, &mv);
        assert_eq!(after.presentation.slides[1], base.presentation.slides[0]);
        assert_eq!(after.presentation.slides[0], base.presentation.slides[1]);
        for inv in Mutation::inverse(&mv, &base) {
            apply_pptx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);
    }

    #[test]
    fn insert_then_remove_shape_apply_and_inverse() {
        let base = fixture();
        let shape = PptxShape::Picture { blip_rel_id: "rId9".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } };
        let insert = PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: shape.clone() };
        let mut after = base.clone();
        apply_pptx_mutation(&mut after, &insert);
        assert_eq!(after.presentation.slides[0].shapes.len(), 2);
        assert_eq!(after.presentation.slides[0].shapes[1], shape);
        for inv in Mutation::inverse(&insert, &base) {
            apply_pptx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let remove = PptxMutation::RemoveShape { slide_index: 0, shape_index: 0 };
        let mut after2 = base.clone();
        apply_pptx_mutation(&mut after2, &remove);
        assert!(after2.presentation.slides[0].shapes.is_empty());
        for inv in Mutation::inverse(&remove, &base) {
            apply_pptx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[test]
    fn set_shape_text_and_position_apply_and_inverse() {
        let base = fixture();
        let mutation = PptxMutation::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("changed")] };
        let mut after = base.clone();
        apply_pptx_mutation(&mut after, &mutation);
        let PptxShape::TextBox { text_frame, .. } = &after.presentation.slides[0].shapes[0] else { panic!("text box") };
        assert_eq!(text_frame[0].runs[0].text, "changed");
        for inv in Mutation::inverse(&mutation, &base) {
            apply_pptx_mutation(&mut after, &inv);
        }
        assert_eq!(after, base);

        let pos = PptxTransform { x: 10, y: 20, cx: 30, cy: 40 };
        let set_pos = PptxMutation::SetShapePosition { slide_index: 0, shape_index: 0, position: pos };
        let mut after2 = base.clone();
        apply_pptx_mutation(&mut after2, &set_pos);
        let PptxShape::TextBox { position, .. } = &after2.presentation.slides[0].shapes[0] else { panic!("text box") };
        assert_eq!(*position, pos);
        for inv in Mutation::inverse(&set_pos, &base) {
            apply_pptx_mutation(&mut after2, &inv);
        }
        assert_eq!(after2, base);
    }

    #[test]
    fn set_shape_text_on_picture_is_a_no_op() {
        let mut base = fixture();
        base.presentation.slides[0].shapes.push(PptxShape::Picture { blip_rel_id: "rId5".into(), position: PptxTransform::default() });
        let mutation = PptxMutation::SetShapeText { slide_index: 0, shape_index: 1, text_frame: vec![PptxParagraph::text("nope")] };
        let diff = Mutation::diff(&mutation, &base);
        assert!(<PptxDiff as DiffAlgebra<PptxSnapshot>>::is_empty(&diff));
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field, both `presentation` and `opc`.
    ///
    /// The recipe's "known structural trap" applies RECURSIVELY: within ANY single `between(a,b)`
    /// call, no nesting level can show both `removed` AND `added` at once (a naive positional
    /// diff has only one tail flavor per direction). So EVERY level here (top-level `slides`,
    /// AND `slide0`'s own nested `shapes`) uses different-length lists and splits its
    /// removed/added coverage across the two `between()` directions, same technique repeated one
    /// level deeper:
    /// - `slide0`: `sweep_a` has `[TextBox_old, Picture_toDrop]` (len 2), `sweep_b` has
    ///   `[TextBox_new]` (len 1) -- `a -> b` shows `shapes.removed` (Picture) +
    ///   `shapes.modified` (TextBox, every field incl. the `font_size` tri-state and a nested
    ///   `text_frame` paragraph ADDED); `b -> a` shows `shapes.added` (the same Picture, whole).
    /// - `slide1`: `[Placeholder_old]` -> `[Placeholder_new]`, same length both sides -- pure
    ///   `shapes.modified`, exercising `kind`/`text_frame`/`position` all three in one diff.
    /// - top level: `sweep_a` has 3 slides, `sweep_b` has 2 -- `a -> b` shows `slides.removed`
    ///   (the dropped `Other`-shaped slide2) + `slides.modified` (both slide0 and slide1); `b ->
    ///   a` shows `slides.added` (slide2, whole, carrying its `Other` shape verbatim).
    ///
    /// `opc` content_types/parts/relationships each get one removed, one modified, one added,
    /// same convention as docx's own sweep fixtures (name-keyed collections have no such trap).
    fn sweep_a() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.content_types.set_default("toRemove", "application/octet-stream");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>".to_vec());
        opc.set_part("ppt/toModify.xml", "application/xml", b"old".to_vec());
        opc.set_part("ppt/toRemove.xml", "application/xml", b"gone".to_vec());
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        opc.add_relationship("", "rId9", "http://example/toRemove", "ppt/toRemove.xml");
        opc.relationships.insert(
            "ppt/presentation.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "slides/old.xml".into(), target_mode: OpcTargetMode::Internal }],
        );
        opc.relationships.insert(
            "ppt/toRemove.xml".into(),
            vec![OpcRelationship { id: "rId8".into(), rel_type: "http://example/ownerToRemove".into(), target: "media/gone.png".into(), target_mode: OpcTargetMode::Internal }],
        );

        PptxSnapshot::from_parts(
            opc,
            PptxPresentation {
                slides: vec![
                    PptxSlide {
                        shapes: vec![
                            PptxShape::TextBox {
                                text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "old".into(), bold: false, italic: false, font_size: Some(10) }] }],
                                position: PptxTransform { x: 1, y: 1, cx: 1, cy: 1 },
                            },
                            PptxShape::Picture { blip_rel_id: "rIdToDrop".into(), position: PptxTransform::default() },
                        ],
                    },
                    PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("stay old")], position: PptxTransform::default() }] },
                    PptxSlide { shapes: vec![PptxShape::Other { xml: "<p:graphicFrame/>".into() }] },
                ],
            },
        )
    }

    fn sweep_b() -> PptxSnapshot {
        let mut opc = OpcPackage::empty();
        opc.content_types.set_default("rels", RELS_CONTENT_TYPE);
        opc.content_types.set_default("xml", "application/xml");
        opc.content_types.set_default("added", "application/octet-stream");
        opc.set_part("ppt/presentation.xml", "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml", b"<p:presentation/>changed".to_vec());
        opc.set_part("ppt/toModify.xml", "application/xml", b"new".to_vec());
        opc.set_part("ppt/added.xml", "application/xml", b"fresh".to_vec());
        opc.content_types.set_override("ppt/toModify.xml", "application/xml-modified");
        opc.add_relationship("", "rId1", REL_TYPE_OFFICE_DOCUMENT, "ppt/presentation.xml");
        opc.relationships.insert(
            "ppt/presentation.xml".into(),
            vec![OpcRelationship { id: "rId2".into(), rel_type: "http://example/toModify".into(), target: "slides/new.xml".into(), target_mode: OpcTargetMode::Internal }],
        );
        opc.relationships.insert("ppt/added.xml".into(), vec![OpcRelationship { id: "rId3".into(), rel_type: "http://example/added".into(), target: "media/added.png".into(), target_mode: OpcTargetMode::Internal }]);

        PptxSnapshot::from_parts(
            opc,
            PptxPresentation {
                slides: vec![
                    PptxSlide {
                        shapes: vec![PptxShape::TextBox {
                            text_frame: vec![
                                PptxParagraph { runs: vec![PptxRun { text: "new".into(), bold: true, italic: true, font_size: None }] },
                                PptxParagraph::text("second para"),
                            ],
                            position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 },
                        }],
                    },
                    PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "subTitle".into(), text_frame: vec![PptxParagraph::text("stay new")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] },
                ],
            },
        )
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    /// 🎯️ A `SetSnapshot` target for `sample_mutations()`, built through `build_minimal_pptx`
    /// with the SAME slide COUNT as `fixture()` (2), only DIFFERENT content -- unlike `sweep_b()`
    /// (a deliberately minimal, unrelated hand-built OPC used only by `field_sweep`/
    /// `between_roundtrip_law`, which document/tolerate the name-keyed collection's known
    /// order-on-append caveat), this shares `fixture()`'s EXACT OPC key set (same six parts:
    /// slideMaster/slideLayout/theme/slide1/slide2/presentation.xml), so `between()` produces
    /// ONLY `modified` entries (zero removed/added) at the OPC level -- a clean bijection that
    /// round-trips losslessly through TWO independent `between()` calls (base->next->base),
    /// which `inverse_law` requires. Mixing OPC key SETS (as `sweep_b()` does) is fine for a
    /// SINGLE `between(a,b).apply(a)==b` check (that's what `field_sweep` proves) but is NOT
    /// guaranteed to survive a round trip THROUGH an intermediate state whose key set differs
    /// again on the way back -- a structural property of the append-new-at-end convention this
    /// engine shares with docx's, not a regression.
    fn mutated_fixture() -> PptxSnapshot {
        crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(PptxPresentation {
            slides: vec![
                PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("changed first")], position: PptxTransform { x: 9, y: 9, cx: 9, cy: 9 } }] },
                PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("changed second")], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } }] },
            ],
        })
    }

    fn sample_mutations() -> Vec<PptxMutation> {
        vec![
            PptxMutation::NoMutation,
            PptxMutation::SetSnapshot { snapshot: mutated_fixture() },
            PptxMutation::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform::default() }] } },
            PptxMutation::RemoveSlide { index: 0 },
            PptxMutation::MoveSlide { from: 0, to: 1 },
            PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform::default() } },
            PptxMutation::RemoveShape { slide_index: 0, shape_index: 0 },
            PptxMutation::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("z")] },
            PptxMutation::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } },
        ]
    }

    #[test]
    fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(&diff_direct, &base);

            let mut via_apply = base.clone();
            let diff_from_apply = apply_pptx_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[test]
    fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_pptx_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <PptxMutation as Mutation<PptxSnapshot>>::inverse(&mutation, &base) {
                apply_pptx_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(&diff, &base);
            let inverse_diff = DiffAlgebra::inverse(&diff, &base);
            let restored = MutationDiff::apply(&inverse_diff, &next);
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    fn assert_absorb_matches_sequential(base: &PptxSnapshot, d1: &PptxDiff, d2: &PptxDiff) -> PptxDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base));
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    fn slides_diff(diff: &PptxDiff) -> &crate::artifacts::pptx::schema::diff::PptxSlidesDiff {
        diff.presentation.as_ref().expect("presentation diff present").slides.as_ref().expect("slides diff present")
    }

    #[test]
    fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = fixture();
            let slide = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
            let d1 = Mutation::diff(&PptxMutation::InsertSlide { index: 2, slide: slide.clone() }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&PptxMutation::RemoveSlide { index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = slides_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            assert_eq!(triple.added[0].item, slide);
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = fixture();
            let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
            let g = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("g")], position: PptxTransform::default() }] };
            let d1 = Mutation::diff(&PptxMutation::InsertSlide { index: 2, slide: f.clone() }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&PptxMutation::InsertSlide { index: 2, slide: g.clone() }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = slides_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            assert!(triple.added.iter().any(|a| a.item == f));
            assert!(triple.added.iter().any(|a| a.item == g));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = fixture();
            let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
            let d1 = Mutation::diff(&PptxMutation::InsertSlide { index: 1, slide: f }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&PptxMutation::SetShapeText { slide_index: 1, shape_index: 0, text_frame: vec![PptxParagraph::text("patched")] }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = slides_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            let PptxShape::TextBox { text_frame, .. } = &triple.added[0].item.shapes[0] else { panic!("text box") };
            assert_eq!(text_frame[0].runs[0].text, "patched");
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = fixture();
            let d1 = Mutation::diff(&PptxMutation::SetShapeText { slide_index: 1, shape_index: 0, text_frame: vec![PptxParagraph::text("patched")] }, &base);
            let mid = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&PptxMutation::RemoveSlide { index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, &d1, &d2);
            let triple = slides_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = fixture();
            let f = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("f")], position: PptxTransform::default() }] };
            let g = PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("g")], position: PptxTransform::default() }] };
            let d1 = Mutation::diff(&PptxMutation::InsertSlide { index: 2, slide: f }, &base);
            let mid1 = MutationDiff::apply(&d1, &base);
            let d2 = Mutation::diff(&PptxMutation::InsertSlide { index: 2, slide: g }, &mid1);
            let mid2 = MutationDiff::apply(&d2, &mid1);
            let d3 = Mutation::diff(&PptxMutation::RemoveSlide { index: 0 }, &mid2);
            let sequential = MutationDiff::apply(&d3, &mid2);

            let mut left = d1.clone();
            MutationDiff::absorb(&mut left, d2.clone());
            MutationDiff::absorb(&mut left, d3.clone());

            let mut d2_then_d3 = d2.clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.clone());
            let mut right = d1.clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[test]
    fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &b), &a), b);
        assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&b, &a), &b), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&sample, &sample), &sample), sample);

        // "Real" fixture leg: a realistic multi-slide presentation diffed against a mutated variant.
        let real = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(PptxPresentation {
            slides: vec![
                PptxSlide { shapes: vec![PptxShape::Placeholder { kind: "title".into(), text_frame: vec![PptxParagraph::text("Chapter One")], position: PptxTransform::default() }] },
                PptxSlide { shapes: vec![PptxShape::TextBox { text_frame: vec![PptxParagraph::text("Body text goes here.")], position: PptxTransform::default() }] },
            ],
        });
        let mut mutated = real.clone();
        apply_pptx_mutation(&mut mutated, &PptxMutation::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph::text("Chapter Two")] });
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&real, &mutated), &real), mutated);
        assert_eq!(MutationDiff::apply(&<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&mutated, &real), &mutated), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[test]
    fn codec_retention_law() {
        let snap = crate::artifacts::pptx::standards::v_ecma_376::subsets::any::io::export::serializers::build_minimal_pptx(PptxPresentation {
            slides: vec![PptxSlide {
                shapes: vec![
                    PptxShape::Placeholder { kind: "ctrTitle".into(), text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "Hello".into(), bold: true, italic: true, font_size: Some(44) }] }], position: PptxTransform { x: 100, y: 200, cx: 300, cy: 400 } },
                    PptxShape::Picture { blip_rel_id: "rId2".into(), position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } },
                    PptxShape::Other { xml: "<p:graphicFrame><a:graphic/></p:graphicFrame>".into() },
                ],
            }],
        });
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <PptxSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field across BOTH
    /// `opc` and `presentation`, at every nesting level (`slides` AND, within a modified slide,
    /// `shapes` AND, within a modified `TextBox`/`Placeholder`, `text_frame`/`runs`) -- see the
    /// fixtures' doc comment for exactly how each collection flavor (removed/modified/added) is
    /// exercised and split across the two `between()` directions per this ticket's "known
    /// structural trap" note, which this test found applies recursively at every level, not just
    /// the top one.
    #[test]
    fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a), b);
        let diff_ba = <PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b), a);
        assert!(<PptxDiff as DiffAlgebra<PptxSnapshot>>::between(&a, &a).is_empty());

        // opc: content_types (both defaults+overrides), parts, relationships all populated.
        let opc_diff = diff_ab.opc.as_ref().expect("opc diff present");
        let ct = opc_diff.content_types.as_ref().expect("content_types diff present");
        let defaults = ct.defaults.as_ref().expect("defaults diff present");
        assert!(!defaults.added.is_empty(), "content_types.defaults: added not exercised");
        let overrides = ct.overrides.as_ref().expect("overrides diff present");
        assert!(!overrides.modified.is_empty(), "content_types.overrides: modified not exercised");
        let parts = opc_diff.parts.as_ref().expect("parts diff present");
        assert!(!parts.removed.is_empty(), "opc.parts: removed not exercised");
        assert!(!parts.modified.is_empty(), "opc.parts: modified not exercised");
        assert!(!parts.added.is_empty(), "opc.parts: added not exercised");
        let part_mod = &parts.modified[0];
        assert!(matches!(&part_mod.diff, PptxOpcPartDiff { bytes: Some(_), .. }));
        let rels = opc_diff.relationships.as_ref().expect("relationships diff present");
        assert!(!rels.removed.is_empty(), "opc.relationships: removed (owner) not exercised");
        assert!(!rels.modified.is_empty(), "opc.relationships: modified (owner) not exercised");
        assert!(!rels.added.is_empty(), "opc.relationships: added (owner) not exercised");

        // presentation.slides, `a -> b` direction: top-level `removed` (the dropped `Other`
        // slide, index 2) + `modified` (BOTH slide0 and slide1 differ). The recipe's "known
        // structural trap" applies RECURSIVELY -- within this ONE `between(a,b)` call, no level
        // (top OR nested) can show both `removed` and `added` at once, so slide0's own nested
        // `shapes` diff (below) shows removed+modified here, and `added` only in the `b -> a` leg.
        let pres_diff = diff_ab.presentation.as_ref().expect("presentation diff present");
        let slides = pres_diff.slides.as_ref().expect("slides diff present");
        assert_eq!(slides.removed, vec![2], "slides: removed (top) not exercised");
        assert_eq!(slides.modified.len(), 2, "both slide0 and slide1 must be modified");

        // slide0 (modified.index == 0): shapes.removed (Picture dropped) + shapes.modified
        // (TextBox, every field incl. the font_size tri-state Some(None) + a nested text_frame
        // paragraph ADDED).
        let slide0_diff = &slides.modified.iter().find(|m| m.index == 0).expect("slide0 modified").diff;
        let shapes_diff = slide0_diff.shapes.as_ref().expect("slide0: shapes not exercised");
        assert!(!shapes_diff.removed.is_empty(), "shapes: removed (Picture dropped) not exercised");
        assert!(!shapes_diff.modified.is_empty(), "shapes: modified (TextBox) not exercised");
        let PptxShapeDiff::TextBox(tb_diff) = &shapes_diff.modified[0].diff else { panic!("expected TextBox diff") };
        assert!(tb_diff.position.is_some(), "modified TextBox: position not exercised");
        let tf_diff = tb_diff.text_frame.as_ref().expect("modified TextBox: text_frame not exercised");
        assert!(!tf_diff.modified.is_empty(), "text_frame: modified not exercised");
        assert!(!tf_diff.added.is_empty(), "text_frame: added (second paragraph) not exercised");
        let run_diff = &tf_diff.modified[0].diff.runs.as_ref().expect("paragraph: runs not exercised").modified[0].diff;
        assert!(run_diff.text.is_some() && run_diff.bold.is_some() && run_diff.italic.is_some(), "modified run: text/bold/italic not exercised");
        assert_eq!(run_diff.font_size, Some(None), "run font_size tri-state Some(None) not exercised");

        // slide1 (modified.index == 1): Placeholder modified in ALL THREE of its own fields
        // (kind/text_frame/position) in one diff -- same length both sides, pure `modified`.
        let slide1_diff = &slides.modified.iter().find(|m| m.index == 1).expect("slide1 modified").diff;
        let shapes_diff_1 = slide1_diff.shapes.as_ref().expect("slide1: shapes not exercised");
        let PptxShapeDiff::Placeholder(ph_diff) = &shapes_diff_1.modified[0].diff else { panic!("expected Placeholder diff") };
        assert_eq!(ph_diff.kind.as_deref(), Some("subTitle"), "placeholder kind not exercised");
        assert!(ph_diff.text_frame.is_some(), "placeholder text_frame not exercised");
        assert!(ph_diff.position.is_some(), "placeholder position not exercised");

        // `b -> a` exercises: top-level `added` (the dropped `Other` slide, carried whole) +
        // slide0's `shapes.added` (the Picture, whole) + the font_size tri-state's OTHER state
        // (`Some(Some(10))`, restoring the value `a` had).
        let slides_ba = diff_ba.presentation.as_ref().unwrap().slides.as_ref().expect("slides diff (b->a) present");
        assert!(!slides_ba.added.is_empty(), "slides (b->a): added (top) not exercised");
        let PptxShape::Other { xml } = &slides_ba.added[0].item.shapes[0] else { panic!("expected added Other shape") };
        assert!(!xml.is_empty());

        let slide0_diff_ba = &slides_ba.modified.iter().find(|m| m.index == 0).expect("slide0 modified (b->a)").diff;
        let shapes_diff_ba = slide0_diff_ba.shapes.as_ref().expect("shapes diff (b->a) present");
        assert!(!shapes_diff_ba.added.is_empty(), "shapes (b->a): added (Picture) not exercised");
        let PptxShape::Picture { blip_rel_id, .. } = &shapes_diff_ba.added[0].item else { panic!("expected added Picture") };
        assert_eq!(blip_rel_id, "rIdToDrop");
        let PptxShapeDiff::TextBox(tb_ba) = &shapes_diff_ba.modified[0].diff else { panic!("expected TextBox diff (b->a)") };
        let tf_ba = tb_ba.text_frame.as_ref().expect("text_frame diff (b->a) present");
        let run_diff_ba = &tf_ba.modified[0].diff.runs.as_ref().expect("runs diff (b->a) present").modified[0].diff;
        assert_eq!(run_diff_ba.font_size, Some(Some(10)), "run font_size tri-state Some(Some(_)) not exercised");
    }
    //#endregion 🔖️FieldSweep

    //#region 🔖️OpTextBinaryRoundtripLaw
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PptxMutation` grammar --
    /// exercises every variant incl. `SetSnapshot`'s full `PptxSnapshot` (OPC package + typed
    /// slides), `InsertShape`'s bare `PptxShape` enum payload (every `TextBox`/`Picture`/
    /// `Placeholder`/`Other` variant), and `SetShapeText`'s `Vec<PptxParagraph>`.
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = fixture();
        let mutations = vec![
            PptxMutation::NoMutation,
            PptxMutation::SetSnapshot { snapshot: base.clone() },
            PptxMutation::InsertSlide { index: 1, slide: PptxSlide { shapes: vec![PptxShape::Other { xml: "<p:graphicFrame/>".into() }] } },
            PptxMutation::RemoveSlide { index: 0 },
            PptxMutation::MoveSlide { from: 0, to: 1 },
            PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::TextBox { text_frame: vec![PptxParagraph::text("x")], position: PptxTransform { x: 1, y: 2, cx: 3, cy: 4 } } },
            PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Picture { blip_rel_id: "rId7".into(), position: PptxTransform::default() } },
            PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Placeholder { kind: "body".into(), text_frame: vec![PptxParagraph::text("ph")], position: PptxTransform::default() } },
            PptxMutation::InsertShape { slide_index: 0, shape_index: 1, shape: PptxShape::Other { xml: "<p:cxnSp/>".into() } },
            PptxMutation::RemoveShape { slide_index: 0, shape_index: 0 },
            PptxMutation::SetShapeText { slide_index: 0, shape_index: 0, text_frame: vec![PptxParagraph { runs: vec![PptxRun { text: "bold".into(), bold: true, italic: false, font_size: Some(24) }] }, PptxParagraph::text("second")] },
            PptxMutation::SetShapePosition { slide_index: 0, shape_index: 0, position: PptxTransform { x: 5, y: 6, cx: 7, cy: 8 } },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = PptxMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = PptxMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️OpTextBinaryRoundtripLaw
}
//#endregion 🧪️Tests
