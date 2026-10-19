//! 🛠️ The per-mutation diff builders and the page box vocabulary.

use super::*;

//#region 🔖️MutationDiffBuilders
/// 🔁️ The whole-list replacement rows: every held row is removed and every new row is added at its final index; nothing when both agree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_indexed<T: Clone + PartialEq>(held: &[T], target: &[T]) -> Option<PdfIndexedDiff<T>> {
    (held != target).then(|| PdfIndexedDiff { removed: (0..held.len()).collect(), modified: Vec::new(), added: target.iter().cloned().enumerate().map(|(index, value)| PdfIndexedItem { index, value }).collect() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn page_patch(index: usize, diff: PdfPageDiff) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { modified: vec![PdfPageModified { index, diff }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_page(index: usize, page: PdfPage) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { added: vec![PdfPageAdded { index, page }], ..Default::default() }), ..Default::default() }
}
/// 📄️ Replaces page `index` with `page`: the page is removed and re-added at the same index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_page(base: &PdfSnapshot, index: usize, page: &PdfPage) -> PdfDiff {
    let Some(current) = base.pages.get(index) else { return PdfDiff::default() };
    if current == page {
        return PdfDiff::default();
    }
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![index], added: vec![PdfPageAdded { index, page: page.clone() }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_page(index: usize) -> PdfDiff {
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![index], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_media_box(index: usize, media_box: PdfRect) -> PdfDiff {
    page_patch(index, PdfPageDiff { media_box: Some(media_box), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_crop_box(index: usize, crop_box: Option<PdfRect>) -> PdfDiff {
    page_patch(index, PdfPageDiff { crop_box: Some(PdfSet::from_option(&crop_box)), ..Default::default() })
}
/// 📐 Sets one of the optional page boxes (`bleed`, `trim`, `art`) or clears it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_box(index: usize, kind: PdfPageBox, rect: Option<PdfRect>) -> PdfDiff {
    let set = Some(PdfSet::from_option(&rect));
    page_patch(
        index,
        match kind {
            PdfPageBox::Crop => PdfPageDiff { crop_box: set, ..Default::default() },
            PdfPageBox::Bleed => PdfPageDiff { bleed_box: set, ..Default::default() },
            PdfPageBox::Trim => PdfPageDiff { trim_box: set, ..Default::default() },
            PdfPageBox::Art => PdfPageDiff { art_box: set, ..Default::default() },
        },
    )
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_rotation(index: usize, rotation: i32) -> PdfDiff {
    page_patch(index, PdfPageDiff { rotate: Some(rotation), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_user_unit(index: usize, user_unit: Option<f64>) -> PdfDiff {
    page_patch(index, PdfPageDiff { user_unit: Some(PdfSet::from_option(&user_unit)), ..Default::default() })
}
/// ✏️️ Replaces page `index`'s whole content: every held operator is removed and the new ones are added in order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let current: &[PdfOp] = base.pages.get(index).map(|page| page.content.as_slice()).unwrap_or(&[]);
    match replace_indexed(current, content) {
        Some(content) => page_patch(index, PdfPageDiff { content: Some(content), ..Default::default() }),
        None => PdfDiff::default(),
    }
}
/// ➕️ Appends operators to page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_append_page_content(base: &PdfSnapshot, index: usize, content: &[PdfOp]) -> PdfDiff {
    let start = base.pages.get(index).map(|page| page.content.len()).unwrap_or(0);
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: start + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// ➕️ Inserts operators at position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_content(index: usize, at: usize, content: &[PdfOp]) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { added: content.iter().enumerate().map(|(offset, op)| PdfIndexedItem { index: at + offset, value: op.clone() }).collect(), ..Default::default() }), ..Default::default() })
}
/// 🗑️ Removes `count` operators from position `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_content(index: usize, at: usize, count: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { removed: (at..at + count).collect(), ..Default::default() }), ..Default::default() })
}
/// ✏️️ Replaces the operator at `at` of page `index`'s content.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_replace_content(index: usize, at: usize, op: PdfOp) -> PdfDiff {
    page_patch(index, PdfPageDiff { content: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: op }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { added: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_annotation(index: usize, at: usize) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_annotation(index: usize, at: usize, annotation: PdfAnnotation) -> PdfDiff {
    page_patch(index, PdfPageDiff { annotations: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: annotation }], ..Default::default() }), ..Default::default() })
}
/// 🔀️ Moves the page at BASE-state index `from` to FINAL-state index `to` -- `removed`/`added`
/// compose the move (no dedicated "moved" slot on `PdfPagesDiff`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_move_page(base: &PdfSnapshot, from: usize, to: usize) -> PdfDiff {
    let Some(page) = base.pages.get(from) else { return PdfDiff::default() };
    let final_to = to.min(base.pages.len().saturating_sub(1));
    if from == final_to {
        return PdfDiff::default();
    }
    PdfDiff { pages: Some(PdfPagesDiff { removed: vec![from], added: vec![PdfPageAdded { index: final_to, page: page.clone() }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_info(base: &PdfSnapshot, info: &PdfInfo) -> PdfDiff {
    let held = &base.info;
    let diff = PdfInfoDiff {
        title: tri(&held.title, &info.title),
        author: tri(&held.author, &info.author),
        subject: tri(&held.subject, &info.subject),
        keywords: tri(&held.keywords, &info.keywords),
        creator: tri(&held.creator, &info.creator),
        producer: tri(&held.producer, &info.producer),
        creation_date: tri(&held.creation_date, &info.creation_date),
        modification_date: tri(&held.modification_date, &info.modification_date),
        trapped: tri(&held.trapped, &info.trapped),
        extra: (held.extra != info.extra).then(|| info.extra.clone()),
    };
    PdfDiff { info: (!diff.is_empty()).then_some(diff), ..Default::default() }
}

/// 🆔 Upsert of one keyed collection item: `modified` when the key exists in `base`, `added` at `index` (clamped; absent appends)
/// otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_upsert<T: Clone + PartialEq + Keyed>(base: &[T], value: T, index: Option<usize>) -> Option<PdfKeyedDiff<T>> {
    match base.iter().find(|item| item.key() == value.key()) {
        Some(existing) if *existing == value => None,
        Some(_) => Some(PdfKeyedDiff { modified: vec![PdfKeyedItem { key: value.key().to_string(), value }], ..Default::default() }),
        None => Some(PdfKeyedDiff { added: vec![PdfIndexedItem { index: index.map_or(base.len(), |at| at.min(base.len())), value }], ..Default::default() }),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn keyed_remove<T: Clone + PartialEq + Keyed>(base: &[T], key: &str) -> Option<PdfKeyedDiff<T>> {
    base.iter().any(|item| item.key() == key).then(|| PdfKeyedDiff { removed: vec![key.to_string()], ..Default::default() })
}
macro_rules! keyed_builders {
    ($($set:ident / $remove:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            pub fn $set(base: &PdfSnapshot, value: $ty, index: Option<usize>) -> PdfDiff {
                PdfDiff { $field: keyed_upsert(&base.$field, value, index), ..Default::default() }
            }
            pub fn $remove(base: &PdfSnapshot, key: &str) -> PdfDiff {
                PdfDiff { $field: keyed_remove(&base.$field, key), ..Default::default() }
            }
        )*
    };
}
keyed_builders!(
    diff_set_font / diff_remove_font => fonts: PdfFont,
    diff_set_image / diff_remove_image => images: PdfImage,
    diff_set_form / diff_remove_form => forms: PdfFormXObject,
    diff_set_ext_g_state / diff_remove_ext_g_state => ext_g_states: PdfExtGState,
    diff_set_shading / diff_remove_shading => shadings: PdfShading,
    diff_set_pattern / diff_remove_pattern => patterns: PdfPattern,
    diff_set_color_space / diff_remove_color_space => color_spaces: PdfNamedColorSpace,
    diff_set_properties / diff_remove_properties => properties: PdfNamedProperties,
    diff_set_embedded_file / diff_remove_embedded_file => embedded_files: PdfEmbeddedFile,
);
/// 📑 Replaces the outline tree.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_outlines(base: &PdfSnapshot, outlines: &[PdfOutlineItem]) -> PdfDiff {
    PdfDiff { outlines: replace_indexed(&base.outlines, outlines), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_named_destination(base: &PdfSnapshot, destination: PdfNamedDestination, index: Option<usize>) -> PdfDiff {
    let d = match base.named_destinations.iter().position(|item| item.name == destination.name) {
        Some(at) if base.named_destinations[at] == destination => return PdfDiff::default(),
        Some(at) => PdfIndexedDiff { modified: vec![PdfIndexedItem { index: at, value: destination }], ..Default::default() },
        None => PdfIndexedDiff { added: vec![PdfIndexedItem { index: index.map_or(base.named_destinations.len(), |at| at.min(base.named_destinations.len())), value: destination }], ..Default::default() },
    };
    PdfDiff { named_destinations: Some(d), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_named_destination(base: &PdfSnapshot, name: &str) -> PdfDiff {
    match base.named_destinations.iter().position(|item| item.name == name) {
        Some(index) => PdfDiff { named_destinations: Some(PdfIndexedDiff { removed: vec![index], ..Default::default() }), ..Default::default() },
        None => PdfDiff::default(),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_page_labels(base: &PdfSnapshot, labels: &[PdfPageLabelRange]) -> PdfDiff {
    PdfDiff { page_labels: replace_indexed(&base.page_labels, labels), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_output_intents(base: &PdfSnapshot, intents: &[PdfOutputIntent]) -> PdfDiff {
    PdfDiff { output_intents: replace_indexed(&base.output_intents, intents), ..Default::default() }
}
macro_rules! tri_builders {
    ($($name:ident => $field:ident : $ty:ty),* $(,)?) => {
        $(
            pub fn $name(base: &PdfSnapshot, value: Option<$ty>) -> PdfDiff {
                PdfDiff { $field: tri(&base.$field, &value), ..Default::default() }
            }
        )*
    };
}
tri_builders!(
    diff_set_acro_form => acro_form: PdfAcroForm,
    diff_set_optional_content => optional_content: PdfOptionalContent,
    diff_set_page_layout => page_layout: PdfPageLayout,
    diff_set_page_mode => page_mode: PdfPageMode,
    diff_set_viewer_preferences => viewer_preferences: PdfViewerPreferences,
    diff_set_open_action => open_action: PdfOpenAction,
    diff_set_language => language: String,
    diff_set_mark_info => mark_info: PdfMarkInfo,
    diff_set_metadata => metadata: String,
    diff_set_document_id => document_id: [Vec<u8>; 2],
    diff_set_encryption => encryption: PdfEncryption,
);
/// 🔁️ The sparse row replacing `existing` by `value`; nothing when they already agree. The row names the new value whole -- no
/// recursive comparison -- and the concrete inverse reads the replaced value from the base.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn replace_row(existing: &PdfObject, value: &PdfObject) -> Option<PdfValueDiff> {
    (existing != value).then(|| PdfValueDiff::Replace { value: value.clone() })
}
/// 🔧️ Upserts `key` in the catalog's retained entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_catalog_entry(base: &PdfSnapshot, key: &str, value: PdfObject, index: Option<usize>) -> PdfDiff {
    let leaf = match base.catalog_extra.iter().position(|e| e.key == key) {
        Some(pos) => match replace_row(&base.catalog_extra[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: index.map_or(base.catalog_extra.len(), |at| at.min(base.catalog_extra.len())), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { catalog_extra: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_catalog_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.catalog_extra.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { catalog_extra: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_object(id: ObjRef, index: usize, value: PdfObject) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { added: vec![PdfObjectAdded { index, id, value }], ..Default::default() }), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_object(id: ObjRef) -> PdfDiff {
    PdfDiff { objects: Some(PdfObjectsDiff { removed: vec![id], ..Default::default() }), ..Default::default() }
}
/// 🔧️ Upserts object `id`'s value: `modified` against BASE if present, `added` (at the final Vec
/// position) otherwise.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_object_value(base: &PdfSnapshot, id: ObjRef, value: PdfObject, index: Option<usize>) -> PdfDiff {
    match base.objects.iter().find(|o| o.id == id) {
        Some(existing) => match replace_row(&existing.value, &value) {
            None => PdfDiff::default(),
            Some(d) => PdfDiff { objects: Some(PdfObjectsDiff { modified: vec![PdfObjectModified { id, diff: d }], ..Default::default() }), ..Default::default() },
        },
        None => diff_insert_object(id, index.map_or(base.objects.len(), |at| at.min(base.objects.len())), value),
    }
}
/// 🔧️ Upserts `key` at `path` inside object `id`'s value tree (`modified` if `key` already
/// exists at that container, `added` otherwise). Graceful empty diff if `id`/`path` don't
/// resolve to a real `Dict`/`Stream` container in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str, value: PdfObject, index: Option<usize>) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = match entries.iter().position(|e| e.key == key) {
        Some(pos) => match replace_row(&entries[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: index.map_or(entries.len(), |at| at.min(entries.len())), key: key.to_string(), item: value }], ..Default::default() },
    };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Removes `key` at `path` inside object `id`'s value tree. Graceful empty diff if the key
/// isn't actually present in `base`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_dict_entry(base: &PdfSnapshot, id: ObjRef, path: &[PdfPathSegment], key: &str) -> PdfDiff {
    let Some(obj) = base.objects.iter().find(|o| o.id == id) else { return PdfDiff::default() };
    let Some(container) = resolve_value(&obj.value, path) else { return PdfDiff::default() };
    let Some(entries) = dict_entries_of(container) else { return PdfDiff::default() };
    if !entries.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    let is_root_stream = path.is_empty() && matches!(obj.value, PdfObject::Stream { .. });
    let leaf = PdfDictDiff { removed: vec![key.to_string()], ..Default::default() };
    diff_at_object_path(id, path, is_root_stream, leaf)
}
/// 🔧️ Upserts `key` in the top-level trailer dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_trailer_entry(base: &PdfSnapshot, key: &str, value: PdfObject, index: Option<usize>) -> PdfDiff {
    let leaf = match base.trailer.iter().position(|e| e.key == key) {
        Some(pos) => match replace_row(&base.trailer[pos].value, &value) {
            None => return PdfDiff::default(),
            Some(d) => PdfDictDiff { modified: vec![PdfDictModified { key: key.to_string(), diff: d }], ..Default::default() },
        },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: index.map_or(base.trailer.len(), |at| at.min(base.trailer.len())), key: key.to_string(), item: value }], ..Default::default() },
    };
    PdfDiff { trailer: Some(leaf), ..Default::default() }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_trailer_entry(base: &PdfSnapshot, key: &str) -> PdfDiff {
    if !base.trailer.iter().any(|e| e.key == key) {
        return PdfDiff::default();
    }
    PdfDiff { trailer: Some(PdfDictDiff { removed: vec![key.to_string()], ..Default::default() }), ..Default::default() }
}

/// 🪢 Marks `rows` — a diff of the retained COS graph (`objects`, `trailer`) — as a graph edit: applying it carries every typed
/// lane the edited graph reads differently (@see `schema::graph_projection::carry_graph_edit`). A diff without graph rows stays unmarked.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn graph_edit(rows: PdfDiff) -> PdfDiff {
    let graph = rows.objects.is_some() || rows.trailer.is_some();
    PdfDiff { graph_edit: graph, ..rows }
}

/// 🧷️ Includes explicit semantic stream payloads in the same typed graph edit.
pub fn graph_edit_with_roles(mut rows: PdfDiff, roles: Option<&PdfIndexedDiff<PdfAdmittedStreamRole>>) -> PdfDiff {
    rows.admitted_stream_roles = roles.cloned();
    graph_edit(rows)
}

/// ➕️ `first` then `second` as one diff — the rows of two edits that touch different entities, coalesced by [`MutationDiff::absorb`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn sequence(first: PdfDiff, second: PdfDiff) -> PdfDiff {
    let mut diff = first;
    <PdfDiff as MutationDiff<PdfSnapshot>>::absorb(&mut diff, second);
    diff
}

/// 📐 Which optional page box a mutation addresses.
#[derive(semio_framework_value::RetireOwned, Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum PdfPageBox {
    Crop,
    Bleed,
    Trim,
    Art,
}
//#endregion 🔖️MutationDiffBuilders
