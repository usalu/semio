//! 🧬️ PdfMutation (1.7) — document mutation dispatch over the real object-graph model. Ticket
//! 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION D2 `📄️pdf` row: beyond
//! the page-level vocabulary (`InsertPage`/`RemovePage`/`SetPageMediaBox`/`AppendPageContent`/
//! `SetInfo`), adds real object-graph mutations (`InsertObject`/`RemoveObject`/`SetObjectValue`/
//! `SetDictEntry`/`RemoveDictEntry`/`SetTrailerEntry`/`RemoveTrailerEntry`) per the brief. Every
//! variant's `diff()` is handcrafted directly against `base` (never apply-and-capture) and every
//! `inverse()` is handcrafted per variant.

use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::{self, PdfDiff, PdfPathSegment};
use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
use protocol::Mutation;
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
}
//#endregion 🔖️Mutations

//#region 🔖️PathNavigationMut
/// 🔍️ Mutable walk of `path` from `root`, mirroring `diff::resolve_value`'s read-only version --
/// kept local to this module (apply is the only place that needs mutable access).
fn resolve_value_mut<'a>(root: &'a mut PdfObject, path: &[PdfPathSegment]) -> Option<&'a mut PdfObject> {
    let mut current = root;
    for seg in path {
        current = match (seg, current) {
            (PdfPathSegment::ArrayIndex { index }, PdfObject::Array(items)) => items.get_mut(*index)?,
            (PdfPathSegment::DictKey { key }, PdfObject::Dict(entries)) => &mut entries.iter_mut().find(|e| &e.key == key)?.value,
            (PdfPathSegment::DictKey { key }, PdfObject::Stream { dict, .. }) => &mut dict.iter_mut().find(|e| &e.key == key)?.value,
            _ => return None,
        };
    }
    Some(current)
}

fn dict_entries_of_mut(value: &mut PdfObject) -> Option<&mut Vec<PdfDictEntry>> {
    match value {
        PdfObject::Dict(d) => Some(d),
        PdfObject::Stream { dict, .. } => Some(dict),
        _ => None,
    }
}

fn upsert_dict_entry(entries: &mut Vec<PdfDictEntry>, key: &str, value: PdfObject) {
    match entries.iter_mut().find(|e| e.key == key) {
        Some(e) => e.value = value,
        None => entries.push(PdfDictEntry { key: key.to_string(), value }),
    }
}

fn remove_dict_entry(entries: &mut Vec<PdfDictEntry>, key: &str) {
    if let Some(pos) = entries.iter().position(|e| e.key == key) { entries.remove(pos); }
}
//#endregion 🔖️PathNavigationMut

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`. Out-of-range indices / missing ids / unresolvable paths
/// are no-ops rather than panics -- a stale reference (e.g. from a concurrent edit) should
/// degrade gracefully, not crash the engine.
pub fn apply_pdf_mutation(snapshot: &mut PdfSnapshot, mutation: &PdfMutation) -> PdfDiff {
    let __diff = <PdfMutation as protocol::Mutation<PdfSnapshot>>::diff(mutation, snapshot);
    match mutation {
        PdfMutation::NoMutation => {}
        PdfMutation::SetSnapshot { snapshot: next } => *snapshot = next.clone(),
        PdfMutation::InsertPage { index, page } => {
            let at = (*index).min(snapshot.pages.len());
            snapshot.pages.insert(at, page.clone());
        }
        PdfMutation::RemovePage { index } => {
            if *index < snapshot.pages.len() { snapshot.pages.remove(*index); }
        }
        PdfMutation::SetPageMediaBox { index, media_box } => {
            if let Some(page) = snapshot.pages.get_mut(*index) { page.media_box = *media_box; }
        }
        PdfMutation::SetPageCropBox { index, crop_box } => {
            if let Some(page) = snapshot.pages.get_mut(*index) { page.crop_box = *crop_box; }
        }
        PdfMutation::AppendPageContent { index, text } => {
            if let Some(page) = snapshot.pages.get_mut(*index) {
                if !page.text.is_empty() { page.text.push('\n'); }
                page.text.push_str(text);
            }
        }
        PdfMutation::SetInfo { info } => snapshot.info = info.clone(),
        PdfMutation::InsertObject { id, value } => {
            if !snapshot.objects.iter().any(|o| o.id == *id) {
                snapshot.objects.push(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfIndirectObject { id: *id, value: value.clone() });
            }
        }
        PdfMutation::RemoveObject { id } => {
            snapshot.objects.retain(|o| o.id != *id);
        }
        PdfMutation::SetObjectValue { id, value } => {
            match snapshot.objects.iter_mut().find(|o| o.id == *id) {
                Some(o) => o.value = value.clone(),
                None => snapshot.objects.push(crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfIndirectObject { id: *id, value: value.clone() }),
            }
        }
        PdfMutation::SetDictEntry { id, path, key, value } => {
            if let Some(obj) = snapshot.objects.iter_mut().find(|o| o.id == *id) {
                if let Some(container) = resolve_value_mut(&mut obj.value, path) {
                    if let Some(entries) = dict_entries_of_mut(container) {
                        upsert_dict_entry(entries, key, value.clone());
                    }
                }
            }
        }
        PdfMutation::RemoveDictEntry { id, path, key } => {
            if let Some(obj) = snapshot.objects.iter_mut().find(|o| o.id == *id) {
                if let Some(container) = resolve_value_mut(&mut obj.value, path) {
                    if let Some(entries) = dict_entries_of_mut(container) {
                        remove_dict_entry(entries, key);
                    }
                }
            }
        }
        PdfMutation::SetTrailerEntry { key, value } => upsert_dict_entry(&mut snapshot.trailer, key, value.clone()),
        PdfMutation::RemoveTrailerEntry { key } => remove_dict_entry(&mut snapshot.trailer, key),
    }

    __diff
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<PdfSnapshot> for PdfMutation {
    type Diff = PdfDiff;

    fn diff(&self, base: &PdfSnapshot) -> Self::Diff {
        match self {
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
        }
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
            PdfMutation::SetDictEntry { id, path, key, .. } => {
                match original_dict_value(base, *id, path, key) {
                    Some(orig) => vec![PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value: orig }],
                    None => vec![PdfMutation::RemoveDictEntry { id: *id, path: path.clone(), key: key.clone() }],
                }
            }
            PdfMutation::RemoveDictEntry { id, path, key } => {
                match original_dict_value(base, *id, path, key) {
                    Some(orig) => vec![PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value: orig }],
                    None => vec![PdfMutation::NoMutation],
                }
            }
            PdfMutation::SetTrailerEntry { key, .. } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => vec![PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() }],
                None => vec![PdfMutation::RemoveTrailerEntry { key: key.clone() }],
            },
            PdfMutation::RemoveTrailerEntry { key } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => vec![PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() }],
                None => vec![PdfMutation::NoMutation],
            },
        }
    }
}

/// 🔍️ Read-only lookup of the CURRENT value at `key` inside object `id`'s value tree at `path`,
/// for building `SetDictEntry`/`RemoveDictEntry` inverses.
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
impl protocol::OpText for PdfMutation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".into())
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line.trim()).map_err(|e| {
            store::TextError::new(format!("op parse: {e}"), dsl::TextSpan::at(1, 1))
        })
    }
}

impl protocol::OpBinary for PdfMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op encode",
            offset: 0,
            detail: e.to_string(),
        })
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|e| protocol::ProtocolError::Malformed {
            what: "op decode",
            offset: 0,
            detail: e.to_string(),
        })
    }
}
//#endregion OpCodecs

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;
    use protocol::command::DiffAlgebra;
    use crate::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::PdfIndirectObject;

    fn sample_page(seed: u8) -> PdfPage {
        PdfPage { media_box: [0.0, 0.0, 612.0, 792.0], crop_box: None, rotate: 0, text: format!("page-{seed}") }
    }

    fn oref(num: u32, gen: u16) -> ObjRef { ObjRef { num, gen } }

    fn base_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: "stdio.pdf.1.7".into(),
            declared_version: "1.7".into(),
            pages: vec![sample_page(1), sample_page(2), sample_page(3)],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) }]) },
                PdfIndirectObject { id: oref(2, 0), value: PdfObject::Stream { dict: vec![PdfDictEntry { key: "Length".into(), value: PdfObject::Int(3) }], data: vec![1, 2, 3], raw_filter: None } },
            ],
            trailer: vec![PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(oref(1, 0)) }, PdfDictEntry { key: "Size".into(), value: PdfObject::Int(3) }],
        }
    }

    fn round_trips(base: &PdfSnapshot, mutation: PdfMutation) {
        let diff = mutation.diff(base);
        let mutated = diff.apply(base);
        let inverses = mutation.inverse(base);
        let mut restored = mutated.clone();
        for inv in &inverses {
            let inv_diff = inv.diff(&restored);
            restored = inv_diff.apply(&restored);
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
        ];
        for m in cases {
            let mut snap = base.clone();
            let returned_diff = apply_pdf_mutation(&mut snap, &m);
            let expected_diff = m.diff(&base);
            assert_eq!(returned_diff, expected_diff, "returned diff must equal m.diff(base) for {m:?}");
            assert_eq!(snap, expected_diff.apply(&base), "apply_pdf_mutation's snapshot mutation must equal diff.apply(base) for {m:?}");
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
    }

    #[test]
    fn set_dict_entry_nested_path_round_trips() {
        let mut base = base_snapshot();
        base.objects.push(PdfIndirectObject {
            id: oref(4, 0),
            value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Kids".into(), value: PdfObject::Array(vec![PdfObject::Dict(vec![PdfDictEntry { key: "Rotate".into(), value: PdfObject::Int(0) }])]) },
            ]),
        });
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
        assert!(d.is_empty());
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
        assert!(d1.info.is_some());
        let d2 = apply_pdf_mutation(&mut snap, &PdfMutation::SetPageMediaBox { index: 0, media_box: [0.0, 0.0, 100.0, 100.0] });
        assert!(d2.pages.is_some());
        let d3 = apply_pdf_mutation(&mut snap, &PdfMutation::SetObjectValue { id: oref(1, 0), value: PdfObject::Name("Changed".into()) });
        assert!(d3.objects.is_some());
        let d4 = apply_pdf_mutation(&mut snap, &PdfMutation::SetTrailerEntry { key: "Prev".into(), value: PdfObject::Int(1) });
        assert!(d4.trailer.is_some());
        let next = PdfSnapshot { declared_version: "1.4".into(), ..snap.clone() };
        let d5 = apply_pdf_mutation(&mut snap, &PdfMutation::SetSnapshot { snapshot: next });
        assert!(d5.declared_version.is_some());
    }
    //#endregion field_sweep
}
//#endregion Tests
