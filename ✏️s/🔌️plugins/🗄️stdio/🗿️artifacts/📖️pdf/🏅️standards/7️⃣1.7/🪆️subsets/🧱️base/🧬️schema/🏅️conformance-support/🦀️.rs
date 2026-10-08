//! 🏅️ The object-graph edit primitives the SIX PDF 1.7 conformance-class subsets (`🗄️a`, `📐️e`,
//! `⚕️h`, `♿️ua`, `🧾️vt`, `🖨️x`) share. They live here, with the `PdfSnapshot` they edit, because the
//! snapshot is this subset's — a conformance subset re-exports it verbatim and owns only a
//! VOCABULARY over it — and because six subsets sharing one named module is the alternative to six
//! copies of the same twenty lines. Nothing here is conformance-specific: every function is a plain
//! operation on the retained indirect-object graph, and which of them a conformance class composes
//! into a mutation is that subset's own business.
//!
//! @see ../../../🗄️a/🧬️schema/🧬️mutations/🦀️.rs — the first of the six vocabularies built on this.

//#region 🏅️ConformanceSupport
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDictAdded, PdfDictDiff, PdfDictModified, PdfDiff, PdfObjectAdded, PdfObjectsDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};

//#region 🔖️Objects
/// 🆕️ The lowest object number no retained object uses — where a fresh indirect object lands.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn next_object_id(snapshot: &PdfSnapshot) -> ObjRef {
    ObjRef { num: snapshot.objects.iter().map(|object| object.id.num).max().unwrap_or(0) + 1, gen: 0 }
}



// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn object(snapshot: &PdfSnapshot, id: ObjRef) -> Option<&PdfObject> {
    snapshot.objects.iter().find(|object| object.id == id).map(|object| &object.value)
}

/// 🔗️ Resolves one level of indirection, leaving a direct object as it is.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn resolve<'a>(snapshot: &'a PdfSnapshot, value: &'a PdfObject) -> Option<&'a PdfObject> {
    match value {
        PdfObject::Ref(id) => object(snapshot, *id),
        other => Some(other),
    }
}

/// 🔎️ Every retained object whose value satisfies `predicate`, in retained order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_objects(snapshot: &PdfSnapshot, predicate: impl Fn(&PdfObject) -> bool) -> Vec<ObjRef> {
    snapshot.objects.iter().filter(|object| predicate(&object.value)).map(|object| object.id).collect()
}

/// 🏷️ A dictionary entry read as a `/Name`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_name<'a>(value: &'a PdfObject, key: &str) -> Option<&'a str> {
    value.dict_get(key)?.as_name()
}

/// 🔤️ A dictionary entry read as a literal string.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict_text(value: &PdfObject, key: &str) -> Option<String> {
    match value.dict_get(key)? {
        PdfObject::Text(text) => Some(text.clone()),
        _ => None,
    }
}


//#endregion 🔖️Objects

//#region 🔖️Catalog
/// 📕️ The `/Type /Catalog` object — the document root every conformance class hangs its
/// required keys off.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn catalog_id(snapshot: &PdfSnapshot) -> Option<ObjRef> {
    snapshot.objects.iter().find(|object| dict_name(&object.value, "Type") == Some("Catalog")).map(|object| object.id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn catalog_entry<'a>(snapshot: &'a PdfSnapshot, key: &str) -> Option<&'a PdfObject> {
    object(snapshot, catalog_id(snapshot)?)?.dict_get(key)
}



/// ✅️ A boolean read out of a catalog sub-dictionary (`/MarkInfo /Marked`,
/// `/ViewerPreferences /DisplayDocTitle`), resolving the sub-dictionary through a reference if
/// the writer stored it indirectly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn catalog_flag(snapshot: &PdfSnapshot, container: &str, key: &str) -> Option<bool> {
    let value = resolve(snapshot, catalog_entry(snapshot, container)?)?;
    match value.dict_get(key)? {
        PdfObject::Bool(flag) => Some(*flag),
        _ => Some(false),
    }
}

/// 🧱️ A one-entry dictionary — the shape `/MarkInfo`, `/ViewerPreferences` and `/DPM` all take.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn single_entry_dict(key: &str, value: PdfObject) -> PdfObject {
    PdfObject::Dict(vec![PdfDictEntry { key: key.to_string(), value }])
}

/// 🧱️ A dictionary from a list of key/value pairs, in the order given.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dict(entries: Vec<(&str, PdfObject)>) -> PdfObject {
    PdfObject::Dict(entries.into_iter().map(|(key, value)| PdfDictEntry { key: key.to_string(), value }).collect())
}

/// 🔤️ A literal string object.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn literal(text: &str) -> PdfObject {
    PdfObject::Text(text.to_owned())
}
//#endregion 🔖️Catalog

//#region 🔖️Fonts
/// 🔤️ The three keys ISO 32000-1 §9.9 lets a `/FontDescriptor` carry an embedded font program in.
pub const FONT_PROGRAM_KEYS: [&str; 3] = ["FontFile", "FontFile2", "FontFile3"];

/// 🔤️ Every `/Type /FontDescriptor` object, in retained order — a stable ordinal space no
/// conformance mutation adds to or removes from, which is what lets one be addressed by ordinal.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn font_descriptors(snapshot: &PdfSnapshot) -> Vec<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "Type") == Some("FontDescriptor"))
}

/// 🔤️ Which of the three keys carries descriptor `id`'s embedded program, and the object it
/// points at.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn font_program(snapshot: &PdfSnapshot, id: ObjRef) -> Option<(String, ObjRef)> {
    let value = object(snapshot, id)?;
    FONT_PROGRAM_KEYS.iter().find_map(|key| value.dict_get(key).and_then(|entry| entry.as_ref()).map(|program| ((*key).to_string(), program)))
}
//#endregion 🔖️Fonts

//#region 🔖️FileSpecs
/// 📎️ Every `/Type /Filespec` object, in retained order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_specs(snapshot: &PdfSnapshot) -> Vec<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "Type") == Some("Filespec"))
}

/// 📎️ The `/Type /Filespec` object naming `file_name` in its `/F` (or `/UF`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_spec_named(snapshot: &PdfSnapshot, file_name: &str) -> Option<ObjRef> {
    file_specs(snapshot).into_iter().find(|id| {
        let Some(value) = object(snapshot, *id) else { return false };
        dict_text(value, "F").as_deref() == Some(file_name) || dict_text(value, "UF").as_deref() == Some(file_name)
    })
}
//#endregion 🔖️FileSpecs

//#region 🔖️Actions
/// 📜️ Every action object whose `/S` is `subtype` and whose `payload_key` entry equals `payload`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn action_with(snapshot: &PdfSnapshot, subtype: &str, payload_key: &str, payload: &str) -> Option<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "S") == Some(subtype)).into_iter().find(|id| object(snapshot, *id).and_then(|value| dict_text(value, payload_key)).as_deref() == Some(payload))
}

/// 🎬️ The `/Subtype /Movie` or `/Subtype /Sound` annotation titled `title`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn media_annotation(snapshot: &PdfSnapshot, subtype: &str, title: &str) -> Option<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "Subtype") == Some(subtype)).into_iter().find(|id| object(snapshot, *id).and_then(|value| dict_text(value, "T")).as_deref() == Some(title))
}
//#endregion 🔖️Actions

//#region 🔖️AcroForm
/// ✍️ Every `/AcroForm` field with `/FT /Sig`, resolved through `/Root/AcroForm/Fields`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn signature_fields(snapshot: &PdfSnapshot) -> Vec<ObjRef> {
    let Some(form) = catalog_entry(snapshot, "AcroForm").and_then(|value| resolve(snapshot, value)) else { return Vec::new() };
    let Some(fields) = form.dict_get("Fields").and_then(|value| resolve(snapshot, value)).and_then(|value| value.as_array()) else { return Vec::new() };
    fields.iter().filter_map(|item| item.as_ref()).filter(|id| object(snapshot, *id).is_some_and(|value| dict_name(value, "FT") == Some("Sig"))).collect()
}

/// ✍️ The signature field titled `title`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn signature_field_named(snapshot: &PdfSnapshot, title: &str) -> Option<ObjRef> {
    signature_fields(snapshot).into_iter().find(|id| object(snapshot, *id).and_then(|value| dict_text(value, "T")).as_deref() == Some(title))
}

//#endregion 🔖️AcroForm

//#region 🔖️DocumentParts
/// 🗂️ The `/DPart` node `/Root/DPartRoot/DPartRootNode` points at.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dpart_root_node(snapshot: &PdfSnapshot) -> Option<ObjRef> {
    let root = resolve(snapshot, catalog_entry(snapshot, "DPartRoot")?)?;
    root.dict_get("DPartRootNode")?.as_ref()
}

/// 🗂️ The `/Job` entry of the root `/DPart` node's `/DPM` metadata dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dpart_job(snapshot: &PdfSnapshot) -> Option<String> {
    let node = object(snapshot, dpart_root_node(snapshot)?)?;
    dict_text(resolve(snapshot, node.dict_get("DPM")?)?, "Job")
}
//#endregion 🔖️DocumentParts

//#region 🔖️OutputIntents
/// 🏳️ Every intent reachable from `/Root/OutputIntents`, by its `/S` marker.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn output_intent_subtypes(snapshot: &PdfSnapshot) -> Vec<String> {
    let Some(intents) = catalog_entry(snapshot, "OutputIntents").and_then(|value| resolve(snapshot, value)).and_then(|value| value.as_array()) else { return Vec::new() };
    intents.iter().filter_map(|item| resolve(snapshot, item)).filter_map(|value| dict_name(value, "S").map(|name| name.to_string())).collect()
}

/// 🏳️ The output condition identifier of the first intent, for an inverse that has to put back
/// the one the document already carried.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn output_intent_identifier(snapshot: &PdfSnapshot) -> Option<String> {
    let intents = catalog_entry(snapshot, "OutputIntents").and_then(|value| resolve(snapshot, value)).and_then(|value| value.as_array())?;
    dict_text(resolve(snapshot, intents.first()?)?, "OutputConditionIdentifier")
}

//#endregion 🔖️OutputIntents

//#region 🔖️Axes
/// 🏅️ One named operation per CONFORMANCE AXIS, shared by whichever of the six subsets declares
/// that axis. They live here rather than in six vocabularies because the EDIT is one operation —
/// "put an `/S /JavaScript` action in the graph" is the same graph surgery whether PDF/A, PDF/E,
/// PDF/H or PDF/X is the class forbidding it — while WHICH axes a subset declares, and therefore
/// which of these it composes, is that subset's own vocabulary and is not shared at all.
/// 🔒️ A real Standard Security Handler dictionary: the `/Filter /Standard` + `/V` + `/R` + `/O` +
/// `/U` shape every conformance checker in this standard scans for, with the 32-byte owner and
/// user strings ISO 32000-1 §7.6.3.3 fixes the length of.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encryption_dictionary(version: i64, revision: i64) -> PdfObject {
    dict(vec![
        ("Filter", PdfObject::Name("Standard".to_string())),
        ("V", PdfObject::Int(version)),
        ("R", PdfObject::Int(revision)),
        ("O", PdfObject::Str(vec![0x4f; 32])),
        ("U", PdfObject::Str(vec![0x55; 32])),
        ("P", PdfObject::Int(-1)),
        ("Length", PdfObject::Int(128)),
    ])
}

/// 🔒️ The encryption dictionary declaring exactly `/V version /R revision`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encryption_dictionary_with(snapshot: &PdfSnapshot, version: i64, revision: i64) -> Option<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "Filter") == Some("Standard")).into_iter().find(|id| {
        let Some(value) = object(snapshot, *id) else { return false };
        value.dict_get("V").and_then(PdfObject::as_i64) == Some(version) && value.dict_get("R").and_then(PdfObject::as_i64) == Some(revision) && value.dict_get("O").is_some() && value.dict_get("U").is_some()
    })
}

/// 📜️ An action dictionary — `/S /JavaScript` with its `/JS`, or `/S /Launch` with its `/F`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn action_object(subtype: &str, payload_key: &str, payload: &str) -> PdfObject {
    dict(vec![("Type", PdfObject::Name("Action".to_string())), ("S", PdfObject::Name(subtype.to_string())), (payload_key, literal(payload))])
}

/// 🎬️ A `/Subtype /Movie` or `/Subtype /Sound` annotation. `/Subtype /3D` is a different name and
/// is never produced here — ISO 24517-1 forbids the first two and explicitly allows the third.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn media_annotation_object(subtype: &str, title: &str) -> PdfObject {
    dict(vec![("Type", PdfObject::Name("Annot".to_string())), ("Subtype", PdfObject::Name(subtype.to_string())), ("T", literal(title)), ("Rect", PdfObject::Array(vec![PdfObject::Int(0), PdfObject::Int(0), PdfObject::Int(144), PdfObject::Int(96)]))])
}


/// 🌲️ An empty but well-formed `/Type /StructTreeRoot` — PDF/UA's structure tree in its minimal
/// legitimate form, which is what `check_ua_conformance`'s presence check reads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn struct_tree_root_object() -> PdfObject {
    dict(vec![("Type", PdfObject::Name("StructTreeRoot".to_string())), ("K", PdfObject::Array(Vec::new()))])
}





/// 📄️ The `/Type /Page` objects, in retained order — the ordinal space `/TrimBox` is addressed in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_objects(snapshot: &PdfSnapshot) -> Vec<ObjRef> {
    find_objects(snapshot, |value| dict_name(value, "Type") == Some("Page"))
}

/// 📄️ One page's `/TrimBox` (or `/ArtBox`) as four numbers.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn page_box(snapshot: &PdfSnapshot, page: ObjRef, key: &str) -> Option<[f64; 4]> {
    let items = object(snapshot, page)?.dict_get(key)?.as_array()?;
    if items.len() != 4 {
        return None;
    }
    let values: Vec<f64> = items.iter().map(|item| item.as_f64().unwrap_or(0.0)).collect();
    Some([values[0], values[1], values[2], values[3]])
}

/// 📄️ A four-number box array in PDF's own real-number form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn box_object(values: [f64; 4]) -> PdfObject {
    PdfObject::Array(values.iter().map(|value| PdfObject::Real((*value).into())).collect())
}
//#endregion 🔖️Axes
//#region 🔖️GraphRows
/// 📍️ Where one created object lands: its object number and its position in the retained object list.
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct ObjectPlacement {
    pub id: ObjRef,
    pub index: usize,
}

/// ➕️ Fresh indirect objects added to a document: each takes the placement given for it, or lands at the lowest unused object
/// number and at the end of the retained list, and the rows it collects spell the insertion without touching the document it
/// was measured against.
pub struct Insertion {
    next: u32,
    end: usize,
    placements: Vec<ObjectPlacement>,
    added: Vec<PdfObjectAdded>,
}

impl Insertion {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn after(base: &PdfSnapshot, placements: &[ObjectPlacement]) -> Self {
        Self { next: next_object_id(base).num, end: base.objects.len(), placements: placements.to_vec(), added: Vec::new() }
    }

    /// ➕️ Adds `value` and returns the reference it landed at.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn push(&mut self, value: PdfObject) -> ObjRef {
        let slot = self.added.len();
        let (id, index) = match self.placements.get(slot) {
            Some(placement) => (placement.id, placement.index),
            None => (ObjRef { num: self.next, gen: 0 }, self.end + slot),
        };
        self.next = self.next.max(id.num + 1);
        self.added.push(PdfObjectAdded { index, id, value });
        id
    }

    /// 🧱️ The `objects` rows of everything pushed so far, in final-position order.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn rows(mut self) -> PdfDiff {
        if self.added.is_empty() {
            return PdfDiff::default();
        }
        self.added.sort_by_key(|item| item.index);
        for (position, item) in self.added.iter_mut().enumerate() {
            item.index = item.index.min(self.end + position);
        }
        PdfDiff { objects: Some(PdfObjectsDiff { added: self.added, ..Default::default() }), ..Default::default() }
    }
}

/// 📍️ The placement of every object in `ids`, in the order given; empty when the document lacks any of them, which lets a
/// builder fall back to fresh placements.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn placements_of(base: &PdfSnapshot, ids: &[ObjRef]) -> Vec<ObjectPlacement> {
    ids.iter().map(|id| base.objects.iter().position(|object| object.id == *id).map(|index| ObjectPlacement { id: *id, index })).collect::<Option<Vec<_>>>().unwrap_or_default()
}

/// 📍️ Where `key` sits among object `id`'s own dictionary entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn entry_position(base: &PdfSnapshot, id: ObjRef, key: &str) -> Option<usize> {
    match object(base, id)? {
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().position(|entry| entry.key == key),
        _ => None,
    }
}

/// 📍️ Where `key` sits among the catalog's own entries.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn catalog_entry_position(base: &PdfSnapshot, key: &str) -> Option<usize> {
    entry_position(base, catalog_id(base)?, key)
}

/// ➕️ The rows adding one fresh indirect object, and the reference it lands at.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_object_rows(base: &PdfSnapshot, value: PdfObject, placements: &[ObjectPlacement]) -> (ObjRef, PdfDiff) {
    let mut insertion = Insertion::after(base, placements);
    let id = insertion.push(value);
    (id, insertion.rows())
}

/// ➖️ The rows dropping the object at `id`, if the document holds it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_object_rows(base: &PdfSnapshot, id: ObjRef) -> PdfDiff {
    object(base, id).map_or_else(PdfDiff::default, |_| diff::diff_remove_object(id))
}

/// 🔧️ The rows upserting `key` in object `id`'s own dictionary, preserving entry order for an existing key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_entry_rows(base: &PdfSnapshot, id: ObjRef, key: &str, value: PdfObject, index: Option<usize>) -> PdfDiff {
    diff::diff_set_dict_entry(base, id, &[], key, value, index)
}

/// 🔧️ The rows dropping `key` from object `id`'s own dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_entry_rows(base: &PdfSnapshot, id: ObjRef, key: &str) -> PdfDiff {
    diff::diff_remove_dict_entry(base, id, &[], key)
}

/// 🔧️ The rows upserting `key` in the catalog; nothing when the document has no catalog.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_catalog_entry_rows(base: &PdfSnapshot, key: &str, value: PdfObject, index: Option<usize>) -> PdfDiff {
    catalog_id(base).map_or_else(PdfDiff::default, |id| set_entry_rows(base, id, key, value, index))
}

/// 🔧️ The rows dropping `key` from the catalog.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_catalog_entry_rows(base: &PdfSnapshot, key: &str) -> PdfDiff {
    catalog_id(base).map_or_else(PdfDiff::default, |id| remove_entry_rows(base, id, key))
}

/// 🔗️ How many times `id` is referenced inside `value`, however deep.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn count_references(value: &PdfObject, id: ObjRef) -> usize {
    match value {
        PdfObject::Ref(target) => usize::from(*target == id),
        PdfObject::Array(items) => items.iter().map(|item| count_references(item, id)).sum(),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().map(|entry| count_references(&entry.value, id)).sum(),
        _ => 0,
    }
}

/// 🔗️ Every reference `value` holds, however deep.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn references_in(value: &PdfObject) -> Vec<ObjRef> {
    match value {
        PdfObject::Ref(target) => vec![*target],
        PdfObject::Array(items) => items.iter().flat_map(references_in).collect(),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().flat_map(|entry| references_in(&entry.value)).collect(),
        _ => Vec::new(),
    }
}

/// 🧺️ The objects `entry` exclusively owns: reachable through its references, and referenced by nothing outside `entry` and
/// the objects already owned — what dropping `entry` leaves unreachable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn owned_objects(base: &PdfSnapshot, entry: &PdfObject) -> Vec<ObjRef> {
    let mut owned: Vec<ObjRef> = Vec::new();
    let mut frontier = references_in(entry);
    while let Some(id) = frontier.pop() {
        let Some(value) = object(base, id) else { continue };
        if owned.contains(&id) {
            continue;
        }
        let total: usize = base.objects.iter().map(|item| count_references(&item.value, id)).sum::<usize>() + base.trailer.iter().map(|item| count_references(&item.value, id)).sum::<usize>();
        let held: usize = count_references(entry, id) + owned.iter().filter_map(|member| object(base, *member)).map(|member| count_references(member, id)).sum::<usize>();
        if total != held {
            continue;
        }
        owned.push(id);
        frontier.extend(references_in(value));
    }
    owned.sort_by_key(|id| (id.num, id.gen));
    owned
}

/// ➖️ The rows dropping `key` from the catalog together with every object that entry exclusively owned, so the entry and what
/// it installed leave the document as one edit.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_catalog_entry_owned_rows(base: &PdfSnapshot, key: &str) -> PdfDiff {
    let Some(entry) = catalog_entry(base, key) else { return PdfDiff::default() };
    owned_objects(base, entry).into_iter().fold(remove_catalog_entry_rows(base, key), |rows, id| diff::sequence(rows, diff::diff_remove_object(id)))
}
//#endregion 🔖️GraphRows

//#region 🔖️CompositeRows
/// ✍️ The rows rewriting `/Root/AcroForm` around `fields`, dropping the key entirely when nothing is left — so inserting the
/// only field and removing it again lands back on a document with no AcroForm.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn acro_form_rows(base: &PdfSnapshot, fields: Vec<ObjRef>, entry_index: Option<usize>) -> PdfDiff {
    if fields.is_empty() {
        return remove_catalog_entry_rows(base, "AcroForm");
    }
    let array = PdfObject::Array(fields.into_iter().map(PdfObject::Ref).collect());
    set_catalog_entry_rows(base, "AcroForm", single_entry_dict("Fields", array), entry_index)
}

/// ✍️ The rows adding a `/FT /Sig` field titled `title` to `/Root/AcroForm/Fields`, creating the form when the document has none.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_signature_field_rows(base: &PdfSnapshot, title: &str, placements: &[ObjectPlacement], field_index: Option<usize>, entry_index: Option<usize>) -> PdfDiff {
    let mut insertion = Insertion::after(base, placements);
    let field = insertion.push(dict(vec![("FT", PdfObject::Name("Sig".to_string())), ("T", literal(title))]));
    let mut fields = signature_fields(base);
    fields.insert(field_index.map_or(fields.len(), |at| at.min(fields.len())), field);
    diff::sequence(insertion.rows(), acro_form_rows(base, fields, entry_index))
}

/// ✍️ The rows dropping the `/FT /Sig` field titled `title`, and the whole `/AcroForm` with it when it was the last one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_signature_field_rows(base: &PdfSnapshot, title: &str) -> PdfDiff {
    let Some(field) = signature_field_named(base, title) else { return PdfDiff::default() };
    let remaining: Vec<ObjRef> = signature_fields(base).into_iter().filter(|candidate| *candidate != field).collect();
    diff::sequence(diff::diff_remove_object(field), acro_form_rows(base, remaining, None))
}

/// 🏳️ The rows installing `/Root/OutputIntents` with one intent carrying `subtype` and `identifier`, and — when `dest_profile` —
/// a real ICC destination-profile stream ISO 15930-7 requires alongside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn output_intent_rows(base: &PdfSnapshot, subtype: &str, identifier: &str, dest_profile: bool, placements: &[ObjectPlacement], entry_index: Option<usize>) -> PdfDiff {
    let mut insertion = Insertion::after(base, placements);
    let mut entries = vec![("Type", PdfObject::Name("OutputIntent".to_string())), ("S", PdfObject::Name(subtype.to_string())), ("OutputConditionIdentifier", literal(identifier)), ("Info", literal(identifier))];
    if dest_profile {
        let stream = PdfObject::Stream { dict: vec![PdfDictEntry { key: "N".to_string(), value: PdfObject::Int(3) }], data: format!("ICC destination output profile for {identifier}").into_bytes(), filters: Vec::new() };
        entries.push(("DestOutputProfile", PdfObject::Ref(insertion.push(stream))));
    }
    let intent = insertion.push(dict(entries));
    diff::sequence(insertion.rows(), set_catalog_entry_rows(base, "OutputIntents", PdfObject::Array(vec![PdfObject::Ref(intent)]), entry_index))
}

/// 📎️ The rows adding a `/Type /Filespec` with a real `/EF` attached-file stream and NO `/AFRelationship` — the exact shape
/// ISO 19005-3 requires the relationship key on and ISO 19005-2 forbids outright.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_file_spec_rows(base: &PdfSnapshot, file_name: &str, placements: &[ObjectPlacement]) -> PdfDiff {
    let mut insertion = Insertion::after(base, placements);
    let payload = insertion.push(PdfObject::Stream { dict: Vec::new(), data: format!("attached payload for {file_name}").into_bytes(), filters: Vec::new() });
    insertion.push(dict(vec![("Type", PdfObject::Name("Filespec".to_string())), ("F", literal(file_name)), ("UF", literal(file_name)), ("EF", single_entry_dict("F", PdfObject::Ref(payload)))]));
    insertion.rows()
}

/// 📎️ The rows dropping the `/Type /Filespec` object `id` together with the attached-file streams only it referenced.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_file_spec_rows(base: &PdfSnapshot, id: ObjRef) -> PdfDiff {
    let Some(spec) = object(base, id) else { return PdfDiff::default() };
    let spec_children: Vec<ObjRef> = owned_objects(base, spec).into_iter().filter(|child| *child != id).collect();
    spec_children.into_iter().fold(diff::diff_remove_object(id), |rows, child| diff::sequence(rows, diff::diff_remove_object(child)))
}

/// 🗂️ The rows installing `/Root/DPartRoot` over one `/Type /DPart` node, carrying `/DPM << /Job … >>` when `job` is non-empty —
/// ISO 16612-2's variable-data partitioning in its minimal legitimate form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dpart_root_rows(base: &PdfSnapshot, job: &str, placements: &[ObjectPlacement], entry_index: Option<usize>) -> PdfDiff {
    let mut insertion = Insertion::after(base, placements);
    let mut node = vec![("Type", PdfObject::Name("DPart".to_string()))];
    if !job.is_empty() {
        node.push(("DPM", single_entry_dict("Job", literal(job))));
    }
    let node_id = insertion.push(dict(node));
    let root = insertion.push(dict(vec![("Type", PdfObject::Name("DPartRoot".to_string())), ("DPartRootNode", PdfObject::Ref(node_id))]));
    diff::sequence(insertion.rows(), set_catalog_entry_rows(base, "DPartRoot", PdfObject::Ref(root), entry_index))
}

/// 🗂️ The rows rewriting the root `/DPart` node's `/DPM`, or dropping it when `job` is `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dpart_job_rows(base: &PdfSnapshot, job: Option<&str>, entry_index: Option<usize>) -> PdfDiff {
    let Some(node) = dpart_root_node(base) else { return PdfDiff::default() };
    match job {
        Some(value) => set_entry_rows(base, node, "DPM", single_entry_dict("Job", literal(value)), entry_index),
        None => remove_entry_rows(base, node, "DPM"),
    }
}

/// 🔤️ The rows pointing font descriptor `descriptor` at `program` under `key`, dropping any other embedded-program key so the
/// descriptor carries exactly one program.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn embed_font_file_rows(base: &PdfSnapshot, descriptor: ObjRef, key: &str, program: ObjRef, entry_index: Option<usize>) -> PdfDiff {
    let Some(value) = object(base, descriptor) else { return PdfDiff::default() };
    let Some(entries) = value.as_dict() else { return PdfDiff::default() };
    let stale: Vec<String> = FONT_PROGRAM_KEYS.iter().filter(|candidate| **candidate != key && entries.iter().any(|entry| entry.key == **candidate)).map(|candidate| (*candidate).to_string()).collect();
    let reference = PdfObject::Ref(program);
    let leaf = match entries.iter().find(|entry| entry.key == key) {
        Some(existing) => PdfDictDiff { removed: stale, modified: diff::value_diff_between(&existing.value, &reference).map(|change| PdfDictModified { key: key.to_string(), diff: change }).into_iter().collect(), added: Vec::new() },
        None => PdfDictDiff { added: vec![PdfDictAdded { index: entry_index.map_or(entries.len() - stale.len(), |at| at.min(entries.len() - stale.len())), key: key.to_string(), item: reference }], removed: stale, modified: Vec::new() },
    };
    if leaf == PdfDictDiff::default() {
        return PdfDiff::default();
    }
    diff::diff_at_object_path(descriptor, &[], matches!(value, PdfObject::Stream { .. }), leaf)
}

/// 🌲️ The rows installing `/Root/StructTreeRoot` over one empty structure tree root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn struct_tree_root_rows(base: &PdfSnapshot, placements: &[ObjectPlacement], entry_index: Option<usize>) -> PdfDiff {
    let mut insertion = Insertion::after(base, placements);
    let root = insertion.push(struct_tree_root_object());
    diff::sequence(insertion.rows(), set_catalog_entry_rows(base, "StructTreeRoot", PdfObject::Ref(root), entry_index))
}

/// 📎️ The objects `insert_file_spec_rows` creates for the file spec `spec`, in creation order: its attached-file stream, then
/// the spec itself; empty when the spec does not have exactly the one stream that shape owns.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn file_spec_creation_ids(base: &PdfSnapshot, spec: ObjRef) -> Vec<ObjRef> {
    let Some(value) = object(base, spec) else { return Vec::new() };
    match owned_objects(base, value).into_iter().filter(|child| *child != spec).collect::<Vec<_>>().as_slice() {
        [payload] => vec![*payload, spec],
        _ => Vec::new(),
    }
}

/// 🏳️ The objects `output_intent_rows` creates for the document's first output intent, in creation order: the destination
/// profile stream when the intent owns one, then the intent itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn output_intent_creation_ids(base: &PdfSnapshot) -> Vec<ObjRef> {
    let Some(intent) = catalog_entry(base, "OutputIntents").and_then(|value| resolve(base, value)).and_then(|value| value.as_array()).and_then(|items| items.first()).and_then(PdfObject::as_ref) else { return Vec::new() };
    let profile = object(base, intent).and_then(|value| value.dict_get("DestOutputProfile")).and_then(PdfObject::as_ref);
    profile.into_iter().chain([intent]).collect()
}

/// 🗂️ The objects `dpart_root_rows` creates for the document's `/DPartRoot`, in creation order: the root node, then the root.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dpart_root_creation_ids(base: &PdfSnapshot) -> Vec<ObjRef> {
    let Some(root) = catalog_entry(base, "DPartRoot").and_then(PdfObject::as_ref) else { return Vec::new() };
    dpart_root_node(base).into_iter().chain([root]).collect()
}
//#endregion 🔖️CompositeRows
//#region 🧪️Fixtures
/// 🧪️ A document holding exactly `values` as the indirect objects `1 0 R`, `2 0 R`, … in order, with no typed lane and no trailer.
#[cfg(test)]
pub fn document_of(values: Vec<PdfObject>) -> PdfSnapshot {
    PdfSnapshot { objects: values.into_iter().enumerate().map(|(index, value)| PdfIndirectObject { id: ObjRef { num: index as u32 + 1, gen: 0 }, value }).collect(), ..PdfSnapshot::default() }
}

/// 🧪️ A real one-page document read back from its own encoding, so its typed lanes and its retained graph agree.
#[cfg(test)]
pub fn document() -> PdfSnapshot {
    use crate::standards::v1_7::subsets::base::io::{decode_pdf, encode_pdf};
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfPage;
    let typed = PdfSnapshot { pages: vec![PdfPage::new(612.0, 792.0)], ..PdfSnapshot::default() };
    decode_pdf(&encode_pdf(&typed).expect("the fixture document encodes")).expect("the fixture document decodes")
}

/// 🧪️ A committed `lopdf` pair read through this subset's reader: `(before, after)`.
#[cfg(test)]
pub fn snapshots(before: &[u8], after: &[u8]) -> (PdfSnapshot, PdfSnapshot) {
    use crate::standards::v1_7::subsets::base::io::decode_pdf;
    (decode_pdf(before).expect("the committed before-document decodes"), decode_pdf(after).expect("the committed after-document decodes"))
}

/// 🧪️ The item `after` holds and `before` lacks.
#[cfg(test)]
pub fn added<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    after.iter().find(|item| !before.contains(item)).cloned().expect("the committed edit adds exactly this item")
}

/// 🧪️ The item `before` holds and `after` lacks.
#[cfg(test)]
pub fn removed<T: Clone + PartialEq>(before: &[T], after: &[T]) -> T {
    before.iter().find(|item| !after.contains(item)).cloned().expect("the committed edit removes exactly this item")
}

/// 🧪️ The first page's operator edit as `(at, removed, inserted)`: the common prefix, the operators dropped after it, and the
/// operators written in their place.
#[cfg(test)]
pub fn content_edit(before: &PdfSnapshot, after: &PdfSnapshot) -> (usize, usize, Vec<crate::standards::v1_7::subsets::base::schema::snapshot::PdfOp>) {
    let (old, new) = (&before.pages[0].content, &after.pages[0].content);
    let at = old.iter().zip(new).take_while(|(left, right)| left == right).count();
    let tail = old[at..].iter().rev().zip(new[at..].iter().rev()).take_while(|(left, right)| left == right).count();
    (at, old.len() - at - tail, new[at..new.len() - tail].to_vec())
}

/// 🧪️ The smallest document a catalog entry can hang off: one `/Type /Catalog` root.
#[cfg(test)]
pub fn catalog_object() -> PdfObject {
    dict(vec![("Type", PdfObject::Name("Catalog".to_string()))])
}

/// 🧪️ `base` after the `rows` of one edit, applied through the central applier.
#[cfg(test)]
pub fn after_rows(base: &PdfSnapshot, rows: PdfDiff) -> PdfSnapshot {
    protocol::apply_diff(&diff::graph_edit(rows), base).expect("the fixture rows apply")
}

/// 🧪️ `base` with one `/Type /Tail` object appended and, when the document has a catalog, one `/Tail` catalog entry appended —
/// the rows that make everything created before them a MIDDLE row.
#[cfg(test)]
pub fn with_tail(base: &PdfSnapshot) -> PdfSnapshot {
    let (_, rows) = insert_object_rows(base, dict(vec![("Type", PdfObject::Name("Tail".to_string()))]), &[]);
    let base = after_rows(base, rows);
    match catalog_id(&base) {
        Some(_) => after_rows(&base, set_catalog_entry_rows(&base, "Tail", PdfObject::Int(1), None)),
        None => base,
    }
}

/// 🧪️ `base` with a `/Tail` entry appended to object `id`'s own dictionary, so every entry before it becomes a MIDDLE entry.
#[cfg(test)]
pub fn with_trailing_entry(base: &PdfSnapshot, id: ObjRef) -> PdfSnapshot {
    after_rows(base, set_entry_rows(base, id, "Tail", PdfObject::Int(1), None))
}

/// 🧪️ The placement of object number `num` at list position `index`.
#[cfg(test)]
pub fn placed(num: u32, index: usize) -> ObjectPlacement {
    ObjectPlacement { id: ObjRef { num, gen: 0 }, index }
}

/// 🧪️ `base` after `mutation`, applied through the central applier.
#[cfg(test)]
pub fn applied<M: protocol::Mutation<PdfSnapshot, Diff = PdfDiff>>(base: &PdfSnapshot, mutation: &M) -> PdfSnapshot {
    protocol::apply_diff(mutation.diff(base).diff(), base).expect("the fixture mutation applies")
}
//#endregion 🧪️Fixtures
//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#endregion 🏅️ConformanceSupport
