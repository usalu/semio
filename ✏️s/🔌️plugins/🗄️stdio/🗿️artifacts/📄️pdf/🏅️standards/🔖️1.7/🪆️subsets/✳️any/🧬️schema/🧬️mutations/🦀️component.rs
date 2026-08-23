//! 🧬️ PdfMutation (1.7) — document mutation dispatch over the real object-graph model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row: beyond
//! the page-level vocabulary (`InsertPage`/`RemovePage`/`SetPageMediaBox`/`AppendPageContent`/
//! `SetInfo`), adds real object-graph mutations (`InsertObject`/`RemoveObject`/`SetObjectValue`/
//! `SetDictEntry`/`RemoveDictEntry`/`SetTrailerEntry`/`RemoveTrailerEntry`) per the brief. Every
//! variant's `diff()` is handcrafted directly against `base` (never apply-and-capture) and every
//! `inverse()` is handcrafted per variant.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{
    self, dec_box, dec_objref, dec_pdf_info, dec_pdf_object, dec_pdf_page, dec_str, decode_option, enc_box, enc_objref, enc_pdf_info, enc_pdf_object, enc_pdf_page, enc_str, encode_option, hex_decode, hex_encode, split_top_level, strip_brackets,
    PdfDiff, PdfPathSegment,
};
/// 🧪️ P2-FG3: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// reuses the diff facet's own `pub(crate)` binary codecs (`../🔺️diff/🦀️component.rs`) rather
/// than duplicating them a second time in this file, same intra-artifact reuse pattern this
/// module's `OpText` already uses for the text-form primitives.
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{
    dec_box_bin, dec_objref_bin, dec_path_bin, dec_pdf_info_bin, dec_pdf_object_bin, dec_pdf_page_bin, dec_pdf_snapshot_bin, enc_box_bin, enc_objref_bin, enc_path_bin, enc_pdf_info_bin, enc_pdf_object_bin, enc_pdf_page_bin, enc_pdf_snapshot_bin,
    read_str_lp, write_str_lp,
};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
#[cfg(test)]
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{PdfIndirectObject, PdfStreamFilter};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.pdf.1.7`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum PdfMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: PdfSnapshot,
    },
    InsertPage {
        index: usize,
        page: PdfPage,
    },
    RemovePage {
        index: usize,
    },
    SetPageMediaBox {
        index: usize,
        media_box: [f64; 4],
    },
    SetPageCropBox {
        index: usize,
        crop_box: Option<[f64; 4]>,
    },
    /// ➕ Appends `text` to the page's authoring text (newline-separated from whatever was
    /// already there). No natural minimal inverse exists for "append" within this vocabulary
    /// (there's no `RemovePageContent` counterpart) -- its `inverse` below uses the
    /// full-snapshot-restore escape hatch instead, same as `SetSnapshot`'s own inverse does.
    AppendPageContent {
        index: usize,
        text: String,
    },
    SetInfo {
        info: PdfInfo,
    },
    /// ➕️ Inserts a NEW indirect object at `id` -- a no-op if `id` already exists (use
    /// `SetObjectValue` to overwrite; keeps `InsertObject`/`RemoveObject` a clean inverse pair).
    InsertObject {
        id: ObjRef,
        value: PdfObject,
    },
    RemoveObject {
        id: ObjRef,
    },
    /// 🔧️ Upserts object `id`'s value (modifies if present, inserts if absent).
    SetObjectValue {
        id: ObjRef,
        value: PdfObject,
    },
    /// 🔧️ Upserts `key` at `path` inside object `id`'s value tree (`path` addresses nesting via
    /// `PdfPathSegment::{ArrayIndex,DictKey}` steps, same `NodePath`-style addressing xml/svg
    /// use -- see `diff::diff_at_object_path`'s doc comment).
    SetDictEntry {
        id: ObjRef,
        path: Vec<PdfPathSegment>,
        key: String,
        value: PdfObject,
    },
    RemoveDictEntry {
        id: ObjRef,
        path: Vec<PdfPathSegment>,
        key: String,
    },
    SetTrailerEntry {
        key: String,
        value: PdfObject,
    },
    RemoveTrailerEntry {
        key: String,
    },
    /// 🔀️ Moves the page at BASE-state index `from` to FINAL-state index `to` -- same shape as
    /// `PptxMutation::MoveSlide`, PDF's own precedent for a reorder mutation.
    MovePage {
        from: usize,
        to: usize,
    },
    /// ✏️️ REPLACES page `index`'s authoring text outright. Distinct from `AppendPageContent`,
    /// which only ever grows the text and (for lack of a natural counterpart) falls back to
    /// `SetSnapshot` for its own inverse -- this variant reads the prior text from `base` and
    /// carries a proper, minimal `SetPageContent` inverse instead.
    SetPageContent {
        index: usize,
        text: String,
    },
    /// 🔄️ Sets page `index`'s resolved `/Rotate` value (degrees, one of 0/90/180/270 per ISO
    /// 32000-1 §7.7.3.4 -- `PdfPage::rotate`/`PdfPageDiff::rotate` already carry the wider `i32`
    /// the codec round-trips; `u16` is this mutation's own, narrower authoring surface).
    SetPageRotation {
        index: usize,
        rotation: u16,
    },
}

/// 📇️ Kebab-case spelling of every `PdfMutation` variant, in declaration order -- the exhaustive
/// mutation catalog `../🧪️oracle/🔣️component.json`'s `kinds` array is required to match verbatim
/// (`kinds_const_matches_enum_variants_in_declaration_order` below is what keeps that honest; the
/// framework never parses Rust to check it itself).
pub const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-page",
    "remove-page",
    "set-page-media-box",
    "set-page-crop-box",
    "append-page-content",
    "set-info",
    "insert-object",
    "remove-object",
    "set-object-value",
    "set-dict-entry",
    "remove-dict-entry",
    "set-trailer-entry",
    "remove-trailer-entry",
    "move-page",
    "set-page-content",
    "set-page-rotation",
];
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range indices / missing ids / unresolvable paths
/// are no-ops rather than panics -- a stale reference (e.g. from a concurrent edit) should
/// degrade gracefully, not crash the engine.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> protocol::MutationOutcome<PdfDiff> {
    let outcome = <PdfMutation as Mutation<PdfSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            PdfMutation::NoMutation => PdfDiff::default(),
            PdfMutation::SetSnapshot { snapshot } => diff::diff_set_snapshot(base, snapshot),
            PdfMutation::InsertPage { index, page } => diff::diff_insert_page(*index, page.clone()),
            PdfMutation::RemovePage { index } => diff::diff_remove_page(*index),
            PdfMutation::SetPageMediaBox { index, media_box } => diff::diff_set_page_media_box(*index, *media_box),
            PdfMutation::SetPageCropBox { index, crop_box } => diff::diff_set_page_crop_box(*index, *crop_box),
            PdfMutation::AppendPageContent { index, text } => diff::diff_append_page_content(base, *index, text),
            PdfMutation::SetInfo { info } => diff::diff_set_info(info.clone()),
            PdfMutation::InsertObject { id, value } => diff::diff_insert_object(*id, base.objects.len(), value.clone()),
            PdfMutation::RemoveObject { id } => diff::diff_remove_object(*id),
            PdfMutation::SetObjectValue { id, value } => diff::diff_set_object_value(base, *id, value.clone()),
            PdfMutation::SetDictEntry { id, path, key, value } => diff::diff_set_dict_entry(base, *id, path, key, value.clone()),
            PdfMutation::RemoveDictEntry { id, path, key } => diff::diff_remove_dict_entry(base, *id, path, key),
            PdfMutation::SetTrailerEntry { key, value } => diff::diff_set_trailer_entry(base, key, value.clone()),
            PdfMutation::RemoveTrailerEntry { key } => diff::diff_remove_trailer_entry(base, key),
            PdfMutation::MovePage { from, to } => diff::diff_move_page(base, *from, *to),
            PdfMutation::SetPageContent { index, text } => diff::diff_set_page_content(*index, text),
            PdfMutation::SetPageRotation { index, rotation } => diff::diff_set_page_rotation(*index, *rotation as i32),
        })
    }

    /// ↩️ Real, round-trippable inverses: `apply(inverse(m, base), apply(m, base)) == base` for
    /// every variant, proven by `mutation_apply_inverse_round_trips_every_variant` below.
    fn inverse(&self, base: &PdfSnapshot) -> Vec<Self> {
        match self {
            PdfMutation::NoMutation => vec![PdfMutation::NoMutation],
            PdfMutation::SetSnapshot { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
            PdfMutation::InsertPage { index, .. } => vec![PdfMutation::RemovePage { index: *index }],
            PdfMutation::RemovePage { index } => match base.pages.get(*index) {
                Some(page) => vec![PdfMutation::InsertPage { index: *index, page: page.clone() }],
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::SetPageMediaBox { index, .. } => {
                let prior = base.pages.get(*index).map(|p| p.media_box).unwrap_or([0.0, 0.0, 612.0, 792.0]);
                vec![PdfMutation::SetPageMediaBox { index: *index, media_box: prior }]
            }
            PdfMutation::SetPageCropBox { index, .. } => {
                let prior = base.pages.get(*index).map(|p| p.crop_box).unwrap_or(None);
                vec![PdfMutation::SetPageCropBox { index: *index, crop_box: prior }]
            }
            PdfMutation::AppendPageContent { .. } => vec![PdfMutation::SetSnapshot { snapshot: base.clone() }],
            PdfMutation::SetInfo { .. } => vec![PdfMutation::SetInfo { info: base.info.clone() }],
            PdfMutation::InsertObject { id, .. } => vec![PdfMutation::RemoveObject { id: *id }],
            PdfMutation::RemoveObject { id } => match base.objects.iter().find(|o| o.id == *id) {
                Some(o) => vec![PdfMutation::InsertObject { id: *id, value: o.value.clone() }],
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::SetObjectValue { id, .. } => match base.objects.iter().find(|o| o.id == *id) {
                Some(o) => vec![PdfMutation::SetObjectValue { id: *id, value: o.value.clone() }],
                None => vec![PdfMutation::RemoveObject { id: *id }],
            },
            PdfMutation::SetDictEntry { id, path, key, .. } => match original_dict_value(base, *id, path, key) {
                Some(orig) => vec![PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value: orig }],
                None => vec![PdfMutation::RemoveDictEntry { id: *id, path: path.clone(), key: key.clone() }],
            },
            PdfMutation::RemoveDictEntry { id, path, key } => match original_dict_value(base, *id, path, key) {
                Some(orig) => vec![PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value: orig }],
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::SetTrailerEntry { key, .. } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => vec![PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() }],
                None => vec![PdfMutation::RemoveTrailerEntry { key: key.clone() }],
            },
            PdfMutation::RemoveTrailerEntry { key } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => vec![PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() }],
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::MovePage { from, to } => match base.pages.get(*from) {
                // 🧭️ Mirrors `diff_move_page`'s own clamp: after the removal the page lands at
                // `min(to, len-1)`, so moving IT back to `from` restores the original order exactly
                // (same reasoning `PptxMutation::MoveSlide`'s inverse documents).
                Some(_) => {
                    let len = base.pages.len();
                    let final_pos = (*to).min(len.saturating_sub(1));
                    vec![PdfMutation::MovePage { from: final_pos, to: *from }]
                }
                None => vec![PdfMutation::NoMutation],
            },
            PdfMutation::SetPageContent { index, .. } => {
                let prior = base.pages.get(*index).map(|p| p.text.clone()).unwrap_or_default();
                vec![PdfMutation::SetPageContent { index: *index, text: prior }]
            }
            PdfMutation::SetPageRotation { index, .. } => {
                let prior = base.pages.get(*index).map(|p| p.rotate).unwrap_or(0);
                vec![PdfMutation::SetPageRotation { index: *index, rotation: prior.rem_euclid(360) as u16 }]
            }
        }
    }
}

/// 🔍️ Read-only lookup of the CURRENT value at `key` inside object `id`'s value tree at `path`,
/// for building `SetDictEntry`/`RemoveDictEntry` inverses.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn original_dict_value(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str) -> Option<PdfObject> {
    let obj = base.objects.iter().find(|o| o.id == id)?;
    let mut current = &obj.value;
    for seg in path {
        current = match (seg, current) {
            (PdfPathSegment::ArrayIndex { index }, PdfObject::Array(items)) => items.get(*index)?,
            (PdfPathSegment::DictKey { key }, PdfObject::Dict(entries)) => &entries.iter().find(|e| &e.key == key)?.value,
            (PdfPathSegment::DictKey { key }, PdfObject::Stream { dict, .. }) => &dict.iter().find(|e| &e.key == key)?.value,
            _ => return None,
        };
    }
    let entries: &[PdfDictEntry] = match current {
        PdfObject::Dict(d) => d.as_slice(),
        PdfObject::Stream { dict, .. } => dict.as_slice(),
        _ => return None,
    };
    entries.iter().find(|e| e.key == key).map(|e| e.value.clone())
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: `#[derive(dsl::DslOps)]` on `PdfMutation` was tried for real and confirmed rejected —
/// `cargo check -p semio-s-plugin-stdio --lib` fails with `the trait bound
/// v1_7::...::PdfObject: DslField is not satisfied` (blocker 3a — every variant carrying a raw
/// object-graph value, incl. `SetSnapshot`'s whole `PdfSnapshot`, reaches `PdfObject` and `ObjRef`
/// directly) and `v1_7::...::PdfPathSegment: DslField is not satisfied` (`SetDictEntry`/
/// `RemoveDictEntry`'s own `path` arg is itself a data-carrying enum). `OpText`/`OpBinary` are
/// hand-rolled below, reusing `PdfDiff`'s `pub(crate)` grammar primitives (`hex_encode`/
/// `enc_pdf_object`/`split_top_level`/`encode_option`/...) rather than duplicating them a second
/// time in this file — same intra-artifact reuse `SvgMutation` uses over `SvgDiff`'s primitives.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path_segment(seg: &PdfPathSegment) -> String {
    match seg {
        PdfPathSegment::ArrayIndex { index } => format!("I[{index}]"),
        PdfPathSegment::DictKey { key } => format!("K[{}]", enc_str(key)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path_segment(s: &str) -> Result<PdfPathSegment, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    match tag {
        "I" => Ok(PdfPathSegment::ArrayIndex { index: inner.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        "K" => Ok(PdfPathSegment::DictKey { key: dec_str(inner)? }),
        other => Err(format!("path segment: unknown tag {other:?}")),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_path(path: &[PdfPathSegment]) -> String {
    format!("[{}]", path.iter().map(enc_path_segment).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_path(s: &str) -> Result<Vec<PdfPathSegment>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_path_segment).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_pdf_snapshot(s: &PdfSnapshot) -> String {
    let mut bytes = Vec::new();
    enc_pdf_snapshot_bin(s, &mut bytes);
    hex_encode(&bytes)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_pdf_snapshot(s: &str) -> Result<PdfSnapshot, String> {
    let bytes = hex_decode(s)?;
    let mut reader = store::ByteReader::new(&bytes);
    let snapshot = dec_pdf_snapshot_bin(&mut reader)?;
    if reader.remaining() != 0 {
        return Err(format!("snapshot: {} trailing bytes", reader.remaining()));
    }
    Ok(snapshot)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_pdf_mutation(m: &PdfMutation) -> String {
    match m {
        PdfMutation::NoMutation => "no-mutation".to_string(),
        PdfMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_pdf_snapshot(snapshot)),
        PdfMutation::InsertPage { index, page } => format!("insert-page index={index} page={}", enc_pdf_page(page)),
        PdfMutation::RemovePage { index } => format!("remove-page index={index}"),
        PdfMutation::SetPageMediaBox { index, media_box } => format!("set-page-media-box index={index} media-box={}", enc_box(media_box)),
        PdfMutation::SetPageCropBox { index, crop_box } => format!("set-page-crop-box index={index} crop-box={}", encode_option(crop_box, enc_box)),
        PdfMutation::AppendPageContent { index, text } => format!("append-page-content index={index} text={}", enc_str(text)),
        PdfMutation::SetInfo { info } => format!("set-info info={}", enc_pdf_info(info)),
        PdfMutation::InsertObject { id, value } => format!("insert-object id={} value={}", enc_objref(id), enc_pdf_object(value)),
        PdfMutation::RemoveObject { id } => format!("remove-object id={}", enc_objref(id)),
        PdfMutation::SetObjectValue { id, value } => format!("set-object-value id={} value={}", enc_objref(id), enc_pdf_object(value)),
        PdfMutation::SetDictEntry { id, path, key, value } => format!("set-dict-entry id={} path={} key={} value={}", enc_objref(id), enc_path(path), enc_str(key), enc_pdf_object(value)),
        PdfMutation::RemoveDictEntry { id, path, key } => format!("remove-dict-entry id={} path={} key={}", enc_objref(id), enc_path(path), enc_str(key)),
        PdfMutation::SetTrailerEntry { key, value } => format!("set-trailer-entry key={} value={}", enc_str(key), enc_pdf_object(value)),
        PdfMutation::RemoveTrailerEntry { key } => format!("remove-trailer-entry key={}", enc_str(key)),
        PdfMutation::MovePage { from, to } => format!("move-page from={from} to={to}"),
        PdfMutation::SetPageContent { index, text } => format!("set-page-content index={index} text={}", enc_str(text)),
        PdfMutation::SetPageRotation { index, rotation } => format!("set-page-rotation index={index} rotation={rotation}"),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_pdf_mutation(line: &str) -> Result<PdfMutation, String> {
    if line == "no-mutation" {
        return Ok(PdfMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("pdf mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("pdf mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(PdfMutation::SetSnapshot { snapshot: dec_pdf_snapshot(arg("snapshot")?)? }),
        "insert-page" => Ok(PdfMutation::InsertPage { index: usize_arg("index")?, page: dec_pdf_page(arg("page")?)? }),
        "remove-page" => Ok(PdfMutation::RemovePage { index: usize_arg("index")? }),
        "set-page-media-box" => Ok(PdfMutation::SetPageMediaBox { index: usize_arg("index")?, media_box: dec_box(arg("media-box")?)? }),
        "set-page-crop-box" => Ok(PdfMutation::SetPageCropBox { index: usize_arg("index")?, crop_box: decode_option(arg("crop-box")?, dec_box)? }),
        "append-page-content" => Ok(PdfMutation::AppendPageContent { index: usize_arg("index")?, text: dec_str(arg("text")?)? }),
        "set-info" => Ok(PdfMutation::SetInfo { info: dec_pdf_info(arg("info")?)? }),
        "insert-object" => Ok(PdfMutation::InsertObject { id: dec_objref(arg("id")?)?, value: dec_pdf_object(arg("value")?)? }),
        "remove-object" => Ok(PdfMutation::RemoveObject { id: dec_objref(arg("id")?)? }),
        "set-object-value" => Ok(PdfMutation::SetObjectValue { id: dec_objref(arg("id")?)?, value: dec_pdf_object(arg("value")?)? }),
        "set-dict-entry" => Ok(PdfMutation::SetDictEntry { id: dec_objref(arg("id")?)?, path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)?, value: dec_pdf_object(arg("value")?)? }),
        "remove-dict-entry" => Ok(PdfMutation::RemoveDictEntry { id: dec_objref(arg("id")?)?, path: dec_path(arg("path")?)?, key: dec_str(arg("key")?)? }),
        "set-trailer-entry" => Ok(PdfMutation::SetTrailerEntry { key: dec_str(arg("key")?)?, value: dec_pdf_object(arg("value")?)? }),
        "remove-trailer-entry" => Ok(PdfMutation::RemoveTrailerEntry { key: dec_str(arg("key")?)? }),
        "move-page" => Ok(PdfMutation::MovePage { from: usize_arg("from")?, to: usize_arg("to")? }),
        "set-page-content" => Ok(PdfMutation::SetPageContent { index: usize_arg("index")?, text: dec_str(arg("text")?)? }),
        "set-page-rotation" => Ok(PdfMutation::SetPageRotation { index: usize_arg("index")?, rotation: arg("rotation")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())? }),
        other => Err(format!("pdf mutation: unknown keyword {other:?}")),
    }
}

impl OpText for PdfMutation {
    fn print_op(&self) -> String {
        print_pdf_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_pdf_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

/// 🧪️ P2-FG3: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut, per the mandatory
/// FG1/FG2 binary-frame lesson. `tag` is the `PdfMutation` variant ordinal, in the SAME 0-14
/// order `print_pdf_mutation`'s own keyword match uses. The payload past `format`/`tag` is
/// genuine LEB128-varint/length-prefixed recursive binary (reusing the diff facet's own
/// `pub(crate)` primitives), never the text form's bytes.
impl OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            PdfMutation::NoMutation => 0,
            PdfMutation::SetSnapshot { .. } => 1,
            PdfMutation::InsertPage { .. } => 2,
            PdfMutation::RemovePage { .. } => 3,
            PdfMutation::SetPageMediaBox { .. } => 4,
            PdfMutation::SetPageCropBox { .. } => 5,
            PdfMutation::AppendPageContent { .. } => 6,
            PdfMutation::SetInfo { .. } => 7,
            PdfMutation::InsertObject { .. } => 8,
            PdfMutation::RemoveObject { .. } => 9,
            PdfMutation::SetObjectValue { .. } => 10,
            PdfMutation::SetDictEntry { .. } => 11,
            PdfMutation::RemoveDictEntry { .. } => 12,
            PdfMutation::SetTrailerEntry { .. } => 13,
            PdfMutation::RemoveTrailerEntry { .. } => 14,
            PdfMutation::MovePage { .. } => 15,
            PdfMutation::SetPageContent { .. } => 16,
            PdfMutation::SetPageRotation { .. } => 17,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            PdfMutation::NoMutation => {}
            PdfMutation::SetSnapshot { snapshot } => enc_pdf_snapshot_bin(snapshot, &mut out),
            PdfMutation::InsertPage { index, page } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_pdf_page_bin(page, &mut out);
            }
            PdfMutation::RemovePage { index } => store::pack_rt::write_varint_u64(&mut out, *index as u64),
            PdfMutation::SetPageMediaBox { index, media_box } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_box_bin(media_box, &mut out);
            }
            PdfMutation::SetPageCropBox { index, crop_box } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                out.push(if crop_box.is_some() { 1 } else { 0 });
                if let Some(b) = crop_box {
                    enc_box_bin(b, &mut out);
                }
            }
            PdfMutation::AppendPageContent { index, text } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, text);
            }
            PdfMutation::SetInfo { info } => enc_pdf_info_bin(info, &mut out),
            PdfMutation::InsertObject { id, value } => {
                enc_objref_bin(id, &mut out);
                enc_pdf_object_bin(value, &mut out);
            }
            PdfMutation::RemoveObject { id } => enc_objref_bin(id, &mut out),
            PdfMutation::SetObjectValue { id, value } => {
                enc_objref_bin(id, &mut out);
                enc_pdf_object_bin(value, &mut out);
            }
            PdfMutation::SetDictEntry { id, path, key, value } => {
                enc_objref_bin(id, &mut out);
                enc_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
                enc_pdf_object_bin(value, &mut out);
            }
            PdfMutation::RemoveDictEntry { id, path, key } => {
                enc_objref_bin(id, &mut out);
                enc_path_bin(path, &mut out);
                write_str_lp(&mut out, key);
            }
            PdfMutation::SetTrailerEntry { key, value } => {
                write_str_lp(&mut out, key);
                enc_pdf_object_bin(value, &mut out);
            }
            PdfMutation::RemoveTrailerEntry { key } => write_str_lp(&mut out, key),
            PdfMutation::MovePage { from, to } => {
                store::pack_rt::write_varint_u64(&mut out, *from as u64);
                store::pack_rt::write_varint_u64(&mut out, *to as u64);
            }
            PdfMutation::SetPageContent { index, text } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                write_str_lp(&mut out, text);
            }
            PdfMutation::SetPageRotation { index, rotation } => {
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                store::pack_rt::write_varint_u64(&mut out, *rotation as u64);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT {
            return Err(malformed("op format", 0, format!("expected {}, got {format}", store::pack_rt::OP_BINARY_FORMAT)));
        }
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        let mutation = match tag {
            0 => Ok(PdfMutation::NoMutation),
            1 => Ok(PdfMutation::SetSnapshot { snapshot: dec_pdf_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", reader.position(), e))? }),
            2 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let page = dec_pdf_page_bin(&mut reader).map_err(|e| malformed("op page", reader.position(), e))?;
                Ok(PdfMutation::InsertPage { index, page })
            }
            3 => Ok(PdfMutation::RemovePage { index: reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize }),
            4 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let media_box = dec_box_bin(&mut reader).map_err(|e| malformed("op media_box", reader.position(), e))?;
                Ok(PdfMutation::SetPageMediaBox { index, media_box })
            }
            5 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let has = reader.read_u8().map_err(|e| malformed("op crop_box presence", reader.position(), e.to_string()))?;
                if has > 1 {
                    return Err(malformed("op crop_box presence", reader.position() - 1, format!("expected 0 or 1, got {has}")));
                }
                let crop_box = if has != 0 { Some(dec_box_bin(&mut reader).map_err(|e| malformed("op crop_box", reader.position(), e))?) } else { None };
                Ok(PdfMutation::SetPageCropBox { index, crop_box })
            }
            6 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                Ok(PdfMutation::AppendPageContent { index, text })
            }
            7 => Ok(PdfMutation::SetInfo { info: dec_pdf_info_bin(&mut reader).map_err(|e| malformed("op info", reader.position(), e))? }),
            8 => {
                let id = dec_objref_bin(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let value = dec_pdf_object_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(PdfMutation::InsertObject { id, value })
            }
            9 => Ok(PdfMutation::RemoveObject { id: dec_objref_bin(&mut reader).map_err(|e| malformed("op id", reader.position(), e))? }),
            10 => {
                let id = dec_objref_bin(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let value = dec_pdf_object_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(PdfMutation::SetObjectValue { id, value })
            }
            11 => {
                let id = dec_objref_bin(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_pdf_object_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(PdfMutation::SetDictEntry { id, path, key, value })
            }
            12 => {
                let id = dec_objref_bin(&mut reader).map_err(|e| malformed("op id", reader.position(), e))?;
                let path = dec_path_bin(&mut reader).map_err(|e| malformed("op path", reader.position(), e))?;
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                Ok(PdfMutation::RemoveDictEntry { id, path, key })
            }
            13 => {
                let key = read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))?;
                let value = dec_pdf_object_bin(&mut reader).map_err(|e| malformed("op value", reader.position(), e))?;
                Ok(PdfMutation::SetTrailerEntry { key, value })
            }
            14 => Ok(PdfMutation::RemoveTrailerEntry { key: read_str_lp(&mut reader).map_err(|e| malformed("op key", reader.position(), e))? }),
            15 => {
                let from = reader.read_varint_u64().map_err(|e| malformed("op from", reader.position(), e.to_string()))? as usize;
                let to = reader.read_varint_u64().map_err(|e| malformed("op to", reader.position(), e.to_string()))? as usize;
                Ok(PdfMutation::MovePage { from, to })
            }
            16 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", reader.position(), e))?;
                Ok(PdfMutation::SetPageContent { index, text })
            }
            17 => {
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", reader.position(), e.to_string()))? as usize;
                let rotation = reader.read_varint_u64().map_err(|e| malformed("op rotation", reader.position(), e.to_string()))? as u16;
                Ok(PdfMutation::SetPageRotation { index, rotation })
            }
            other => Err(malformed("op tag", 1, format!("unknown PdfMutation tag {other}"))),
        }?;
        if reader.remaining() != 0 {
            return Err(malformed("op trailing bytes", reader.position(), format!("{} trailing bytes", reader.remaining())));
        }
        Ok(mutation)
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sample_page(seed: u8) -> PdfPage {
        PdfPage { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: format!("page-{seed}") }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn oref(num: u32, gen: u16) -> ObjRef {
        ObjRef { num, gen }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: "stdio.pdf.1.7".into(),
            declared_version: "1.7".into(),
            pages: vec![sample_page(1), sample_page(2), sample_page(3)],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }]) },
                PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![PdfDictEntry { key: "Length".into(), value: PdfObject::Int(3) }], data: vec![1, 2, 3], filters: vec![] } },
            ],
            trailer: vec![PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(oref(1, 0)) }, PdfDictEntry { key: "Size".into(), value: PdfObject::Int(3) }],
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn round_trips(base: &PdfSnapshot, mutation: PdfMutation) {
        let diff = mutation.diff(base);
        let mutated = diff.diff().apply(base).unwrap();
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = inv_diff.diff().apply(&restored).unwrap();
        }
        assert_eq!(&restored, base, "apply(inverse(m), apply(m, base)) must recover base for {mutation:?}");
    }

    //#region mutation_diff_law
    #[test]
    fn mutation_diff_law_matches_apply_pdf_mutation() {
        let base = base_snapshot();
        let cases = vec![
            PdfMutation::NoMutation,
            PdfMutation::InsertPage { index: 1, page: sample_page(9) },
            PdfMutation::RemovePage { index: 1 },
            PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 200.0, 300.0] },
            PdfMutation::SetPageCropBox { index: 0, crop_box: Some([1.0, 1.0, 100.0, 100.0]) },
            PdfMutation::AppendPageContent { index: 0, text: "more".into() },
            PdfMutation::SetInfo { info: PdfInfo { author: Some("Ueli".into()), ..Default::default() } },
            PdfMutation::InsertObject { id: oref(3, 0), value: PdfObject::Int(42) },
            PdfMutation::RemoveObject { id: oref(2, 0) },
            PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Name("Pages".into()) },
            PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![], key: "Count".into(), value: PdfObject::Int(5) },
            PdfMutation::RemoveDictEntry { id: oref(1, 0), path: vec![], key: "Type".into() },
            PdfMutation::SetTrailerEntry { key: "Prev".into(), value: PdfObject::Int(100) },
            PdfMutation::RemoveTrailerEntry { key: "Size".into() },
            PdfMutation::MovePage { from: 0, to: 2 },
            PdfMutation::SetPageContent { index: 0, text: "replaced".into() },
            PdfMutation::SetPageRotation { index: 0, rotation: 90 },
        ];
        for m in cases {
            let mut snap = base.clone();
            let returned_diff = apply_pdf_mutation(&mut snap, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
            assert_eq!(snap, expected_diff.diff().apply(&base).unwrap(), "apply_pdf_mutation's snapshot mutation must equal diff.diff().apply(base) for {m:?}");
        }
    }
    //#endregion mutation_diff_law

    //#region inverse_law
    #[test]
    fn mutation_apply_inverse_round_trips_every_variant() {
        let base = base_snapshot();
        round_trips(&base, PdfMutation::NoMutation);
        round_trips(&base, PdfMutation::SetSnapshot { snapshot: PdfSnapshot { info: PdfInfo { title: Some("X".into()), ..Default::default() }, ..base.clone() } });
        round_trips(&base, PdfMutation::InsertPage { index: 1, page: sample_page(9) });
        round_trips(&base, PdfMutation::RemovePage { index: 1 });
        round_trips(&base, PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 200.0, 300.0] });
        round_trips(&base, PdfMutation::SetPageCropBox { index: 0, crop_box: Some([1.0, 1.0, 100.0, 100.0]) });
        round_trips(&base, PdfMutation::AppendPageContent { index: 0, text: "more text".into() });
        round_trips(&base, PdfMutation::SetInfo { info: PdfInfo { author: Some("Ueli".into()), ..Default::default() } });
        round_trips(&base, PdfMutation::InsertObject { id: oref(3, 0), value: PdfObject::Int(42) });
        round_trips(&base, PdfMutation::RemoveObject { id: oref(2, 0) });
        round_trips(&base, PdfMutation::RemoveObject { id: oref(99, 0) }); // absent id: no-op
        round_trips(&base, PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Name("Pages".into()) });
        round_trips(&base, PdfMutation::SetObjectValue { id: oref(50, 0), value: PdfObject::Int(7) }); // upsert-as-insert
        round_trips(&base, PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![], key: "Count".into(), value: PdfObject::Int(5) });
        round_trips(&base, PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![], key: "New".into(), value: PdfObject::Bool(true) });
        round_trips(&base, PdfMutation::RemoveDictEntry { id: oref(1, 0), path: vec![], key: "Type".into() });
        round_trips(&base, PdfMutation::SetDictEntry { id: oref(2, 0), path: vec![], key: "Length".into(), value: PdfObject::Int(9) });
        round_trips(&base, PdfMutation::SetTrailerEntry { key: "Prev".into(), value: PdfObject::Int(100) });
        round_trips(&base, PdfMutation::SetTrailerEntry { key: "Size".into(), value: PdfObject::Int(4) });
        round_trips(&base, PdfMutation::RemoveTrailerEntry { key: "Size".into() });
        round_trips(&base, PdfMutation::MovePage { from: 0, to: 2 });
        round_trips(&base, PdfMutation::MovePage { from: 2, to: 0 });
        round_trips(&base, PdfMutation::MovePage { from: 1, to: 1 }); // no-op move
        round_trips(&base, PdfMutation::MovePage { from: 0, to: 99 }); // clamped to len-1
        round_trips(&base, PdfMutation::MovePage { from: 99, to: 0 }); // out-of-range from: no-op
        round_trips(&base, PdfMutation::SetPageContent { index: 0, text: "replaced text".into() });
        round_trips(&base, PdfMutation::SetPageContent { index: 99, text: "out of range".into() });
        round_trips(&base, PdfMutation::SetPageRotation { index: 0, rotation: 90 });
        round_trips(&base, PdfMutation::SetPageRotation { index: 99, rotation: 180 }); // out of range: no-op
    }

    #[test]
    fn set_dict_entry_nested_path_round_trips() {
        let mut base = base_snapshot();
        base.objects.push(PdfIndirectObject { id: oref(4, 0), value: PdfObject::Dict(vec![PdfDictEntry { key: "Kids".into(), value: PdfObject::Array(vec![PdfObject::Dict(vec![PdfDictEntry { key: "Rotate".into(), value: PdfObject::Int(0) }])]) }]) });
        let path = vec![PdfPathSegment::DictKey { key: "Kids".into() }, PdfPathSegment::ArrayIndex { index: 0 }];
        round_trips(&base, PdfMutation::SetDictEntry { id: oref(4, 0), path: path.clone(), key: "Rotate".into(), value: PdfObject::Int(90) });
        round_trips(&base, PdfMutation::RemoveDictEntry { id: oref(4, 0), path, key: "Rotate".into() });
    }

    #[test]
    fn remove_page_out_of_range_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        apply_pdf_mutation(&mut snap, &PdfMutation::RemovePage { index: 99 });
        assert_eq!(snap, base);
    }

    #[test]
    fn set_dict_entry_unresolvable_path_is_noop_not_panic() {
        let base = base_snapshot();
        let mut snap = base.clone();
        let d = apply_pdf_mutation(&mut snap, &PdfMutation::SetDictEntry { id: oref(999, 0), path: vec![], key: "X".into(), value: PdfObject::Int(1) });
        assert_eq!(snap, base);
        assert!(d.diff().is_empty());
    }
    //#endregion inverse_law

    //#region field_sweep (see 🔺️diff module's own field_sweep tests for the full snapshot-level sweep)
    #[test]
    fn field_sweep_mutation_vocabulary_covers_every_snapshot_field() {
        // 📏 One mutation exists (or composes via SetSnapshot) per top-level PdfSnapshot field:
        // declaredVersion (via SetSnapshot), info (SetInfo), pages (Insert/Remove/SetMediaBox/
        // SetCropBox/AppendPageContent), objects (Insert/Remove/SetObjectValue/SetDictEntry/
        // RemoveDictEntry), trailer (SetTrailerEntry/RemoveTrailerEntry).
        let base = base_snapshot();
        let mut snap = base.clone();
        let d1 = apply_pdf_mutation(&mut snap, &PdfMutation::SetInfo { info: PdfInfo { author: Some("A".into()), ..Default::default() } });
        assert!(d1.diff().info.is_some());
        let d2 = apply_pdf_mutation(&mut snap, &PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 100.0, 100.0] });
        assert!(d2.diff().pages.is_some());
        let d3 = apply_pdf_mutation(&mut snap, &PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Name("Changed".into()) });
        assert!(d3.diff().objects.is_some());
        let d4 = apply_pdf_mutation(&mut snap, &PdfMutation::SetTrailerEntry { key: "Prev".into(), value: PdfObject::Int(1) });
        assert!(d4.diff().trailer.is_some());
        let next = PdfSnapshot { declared_version: "1.4".into(), ..snap.clone() };
        let d5 = apply_pdf_mutation(&mut snap, &PdfMutation::SetSnapshot { snapshot: next });
        assert!(d5.diff().declared_version.is_some());
    }
    //#endregion field_sweep

    //#region op_codec_roundtrip_law
    /// 🧪️ F6: `OpText`/`OpBinary` round-trip laws for the hand-rolled `PdfMutation` grammar —
    /// exercises every variant, incl. `SetSnapshot`'s full object-graph payload (`PdfObject::
    /// Array`/`Dict`/`Stream`/`Ref` recursion), `SetPageCropBox`'s tri-state-like `Option<[f64;4]>`
    /// arg, and `SetDictEntry`/`RemoveDictEntry`'s `path: Vec<PdfPathSegment>` (both segment kinds).
    #[test]
    fn op_text_binary_roundtrip_law() {
        let base = base_snapshot();
        let mutations = vec![
            PdfMutation::NoMutation,
            PdfMutation::SetSnapshot { snapshot: base.clone() },
            PdfMutation::InsertPage { index: 1, page: sample_page(9) },
            PdfMutation::RemovePage { index: 1 },
            PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 200.0, 300.0] },
            PdfMutation::SetPageCropBox { index: 0, crop_box: Some([1.0, 1.0, 100.0, 100.0]) },
            PdfMutation::SetPageCropBox { index: 0, crop_box: None },
            PdfMutation::AppendPageContent { index: 0, text: "more text\nsecond line".into() },
            PdfMutation::SetInfo { info: PdfInfo { author: Some("Ueli".into()), producer: Some("semio".into()), ..Default::default() } },
            PdfMutation::InsertObject { id: oref(3, 0), value: PdfObject::Int(42) },
            PdfMutation::InsertObject { id: oref(4, 0), value: PdfObject::Array(vec![PdfObject::Real(1.5.into()), PdfObject::Str(vec![0, 255, 128]), PdfObject::Ref(oref(1, 0))]) },
            PdfMutation::RemoveObject { id: oref(2, 0) },
            PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Name("Pages".into()) },
            PdfMutation::SetObjectValue { id: oref(2, 0), value: PdfObject::Stream { dict: vec![PdfDictEntry { key: "Length".into(), value: PdfObject::Int(2) }], data: vec![1, 2], filters: vec![PdfStreamFilter::Flate { predictor: None }] } },
            PdfMutation::SetObjectValue { id: oref(2, 0), value: PdfObject::Null },
            PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![], key: "Count".into(), value: PdfObject::Int(5) },
            PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![PdfPathSegment::DictKey { key: "Kids".into() }, PdfPathSegment::ArrayIndex { index: 2 }], key: "Rotate".into(), value: PdfObject::Int(90) },
            PdfMutation::RemoveDictEntry { id: oref(1, 0), path: vec![], key: "Type".into() },
            PdfMutation::SetTrailerEntry { key: "Prev".into(), value: PdfObject::Int(100) },
            PdfMutation::RemoveTrailerEntry { key: "Size".into() },
            PdfMutation::MovePage { from: 0, to: 2 },
            PdfMutation::SetPageContent { index: 0, text: "replaced text\nsecond line".into() },
            PdfMutation::SetPageRotation { index: 0, rotation: 270 },
        ];
        for mutation in mutations {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = PdfMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = PdfMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
    //#endregion op_codec_roundtrip_law

    //#region kinds_law
    /// 🧪️ Keeps `KINDS` honest against the enum it claims to spell: every variant's
    /// `print_pdf_mutation` keyword, in the SAME declaration order `OpBinary`'s own tag match
    /// uses, must equal `KINDS` entry-for-entry -- the framework never parses Rust to check this
    /// itself (see `KINDS`'s own doc comment), so this test is the one thing that does.
    #[test]
    fn kinds_const_matches_enum_variants_in_declaration_order() {
        let base = base_snapshot();
        let one_per_variant = vec![
            PdfMutation::NoMutation,
            PdfMutation::SetSnapshot { snapshot: base.clone() },
            PdfMutation::InsertPage { index: 0, page: sample_page(9) },
            PdfMutation::RemovePage { index: 0 },
            PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 1.0, 1.0] },
            PdfMutation::SetPageCropBox { index: 0, crop_box: None },
            PdfMutation::AppendPageContent { index: 0, text: "x".into() },
            PdfMutation::SetInfo { info: PdfInfo::default() },
            PdfMutation::InsertObject { id: oref(3, 0), value: PdfObject::Null },
            PdfMutation::RemoveObject { id: oref(1, 0) },
            PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Null },
            PdfMutation::SetDictEntry { id: oref(1, 0), path: vec![], key: "K".into(), value: PdfObject::Null },
            PdfMutation::RemoveDictEntry { id: oref(1, 0), path: vec![], key: "K".into() },
            PdfMutation::SetTrailerEntry { key: "K".into(), value: PdfObject::Null },
            PdfMutation::RemoveTrailerEntry { key: "K".into() },
            PdfMutation::MovePage { from: 0, to: 1 },
            PdfMutation::SetPageContent { index: 0, text: "x".into() },
            PdfMutation::SetPageRotation { index: 0, rotation: 0 },
        ];
        assert_eq!(one_per_variant.len(), KINDS.len(), "one_per_variant must cover every KINDS entry exactly once");
        for (mutation, kind) in one_per_variant.iter().zip(KINDS.iter()) {
            let printed = mutation.print_op();
            let keyword = printed.split(' ').next().unwrap_or(&printed);
            assert_eq!(keyword, *kind, "KINDS order must match the enum's own OpText keyword order for {mutation:?}");
        }
    }
    //#endregion kinds_law
}
//#endregion Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/rotates-the-plan-page-and-titles-the-document/🦀️component.rs"]
    mod tests_set_snapshot_rotates_the_plan_page_and_titles_the_document;
}
//#endregion 🧪️FixtureTests
