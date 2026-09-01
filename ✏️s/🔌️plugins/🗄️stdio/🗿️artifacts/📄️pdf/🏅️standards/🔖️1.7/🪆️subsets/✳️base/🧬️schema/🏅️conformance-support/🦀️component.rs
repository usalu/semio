//! 🏅️ The object-graph edit primitives the SIX PDF 1.7 conformance-class subsets (`✳️a`, `✳️e`,
//! `✳️h`, `✳️ua`, `✳️vt`, `✳️x`) share. They live here, with the `PdfSnapshot` they edit, because the
//! snapshot is this subset's — a conformance subset re-exports it verbatim and owns only a
//! VOCABULARY over it — and because six subsets sharing one named module is the alternative to six
//! copies of the same twenty lines. Nothing here is conformance-specific: every function is a plain
//! operation on the retained indirect-object graph, and which of them a conformance class composes
//! into a mutation is that subset's own business.
//!
//! @see ../../../✳️a/🧬️schema/🧬️mutations/🦀️component.rs — the first of the six vocabularies built on this.

//#region 🏅️ConformanceSupport
use crate::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};

//#region 🔖️Objects
/// 🆕️ The lowest object number no retained object uses — where a fresh indirect object lands.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn next_object_id(snapshot: &PdfSnapshot) -> ObjRef {
    ObjRef { num: snapshot.objects.iter().map(|object| object.id.num).max().unwrap_or(0) + 1, gen: 0 }
}

/// ➕️ Appends a fresh indirect object and returns the reference it landed at.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_object(snapshot: &mut PdfSnapshot, value: PdfObject) -> ObjRef {
    let id = next_object_id(snapshot);
    snapshot.objects.push(PdfIndirectObject { id, value });
    id
}

/// ➖️ Drops the object at `id`, if it is there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_object(snapshot: &mut PdfSnapshot, id: ObjRef) {
    snapshot.objects.retain(|object| object.id != id);
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
        PdfObject::Str(bytes) => Some(String::from_utf8_lossy(bytes).into_owned()),
        _ => None,
    }
}

/// 🔧️ Upserts `key` in object `id`'s own dictionary, preserving entry order for an existing key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_entry(snapshot: &mut PdfSnapshot, id: ObjRef, key: &str, value: PdfObject) {
    let Some(target) = snapshot.objects.iter_mut().find(|object| object.id == id) else { return };
    let entries = match &mut target.value {
        PdfObject::Dict(entries) => entries,
        PdfObject::Stream { dict, .. } => dict,
        _ => return,
    };
    match entries.iter_mut().find(|entry| entry.key == key) {
        Some(entry) => entry.value = value,
        None => entries.push(PdfDictEntry { key: key.to_string(), value }),
    }
}

/// 🔧️ Drops `key` from object `id`'s own dictionary.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_entry(snapshot: &mut PdfSnapshot, id: ObjRef, key: &str) {
    let Some(target) = snapshot.objects.iter_mut().find(|object| object.id == id) else { return };
    let entries = match &mut target.value {
        PdfObject::Dict(entries) => entries,
        PdfObject::Stream { dict, .. } => dict,
        _ => return,
    };
    entries.retain(|entry| entry.key != key);
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_catalog_entry(snapshot: &mut PdfSnapshot, key: &str, value: PdfObject) {
    let Some(id) = catalog_id(snapshot) else { return };
    set_entry(snapshot, id, key, value);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_catalog_entry(snapshot: &mut PdfSnapshot, key: &str) {
    let Some(id) = catalog_id(snapshot) else { return };
    remove_entry(snapshot, id, key);
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
    PdfObject::Str(text.as_bytes().to_vec())
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

/// 🔤️ Every distinct font-program object any descriptor currently references, sorted by object
/// number — the ordinal space a donor program is named in.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn font_programs(snapshot: &PdfSnapshot) -> Vec<ObjRef> {
    let mut programs: Vec<ObjRef> = font_descriptors(snapshot).into_iter().filter_map(|descriptor| font_program(snapshot, descriptor).map(|(_, id)| id)).collect();
    programs.sort();
    programs.dedup();
    programs
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
    fields
        .iter()
        .filter_map(|item| item.as_ref())
        .filter(|id| object(snapshot, *id).map(|value| dict_name(value, "FT") == Some("Sig")).unwrap_or(false))
        .collect()
}

/// ✍️ The signature field titled `title`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn signature_field_named(snapshot: &PdfSnapshot, title: &str) -> Option<ObjRef> {
    signature_fields(snapshot).into_iter().find(|id| object(snapshot, *id).and_then(|value| dict_text(value, "T")).as_deref() == Some(title))
}

/// ✍️ Rewrites `/Root/AcroForm` around `fields`, dropping the key entirely when nothing is left —
/// so inserting the only field and removing it again lands back on a document with no AcroForm.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_acro_form_fields(snapshot: &mut PdfSnapshot, fields: Vec<ObjRef>) {
    if fields.is_empty() {
        remove_catalog_entry(snapshot, "AcroForm");
        return;
    }
    let array = PdfObject::Array(fields.into_iter().map(PdfObject::Ref).collect());
    set_catalog_entry(snapshot, "AcroForm", single_entry_dict("Fields", array));
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

/// 🏳️ Installs `/Root/OutputIntents` with one intent carrying `subtype` and `identifier`, and —
/// when `dest_profile` — a real ICC destination-profile stream ISO 15930-7 requires alongside it.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_output_intent(snapshot: &mut PdfSnapshot, subtype: &str, identifier: &str, dest_profile: bool) {
    let mut entries = vec![
        ("Type", PdfObject::Name("OutputIntent".to_string())),
        ("S", PdfObject::Name(subtype.to_string())),
        ("OutputConditionIdentifier", literal(identifier)),
        ("Info", literal(identifier)),
    ];
    if dest_profile {
        let stream = PdfObject::Stream {
            dict: vec![PdfDictEntry { key: "N".to_string(), value: PdfObject::Int(3) }],
            data: format!("ICC destination output profile for {identifier}").into_bytes(),
            filters: Vec::new(),
        };
        let program = insert_object(snapshot, stream);
        entries.push(("DestOutputProfile", PdfObject::Ref(program)));
    }
    let intent = insert_object(snapshot, dict(entries));
    set_catalog_entry(snapshot, "OutputIntents", PdfObject::Array(vec![PdfObject::Ref(intent)]));
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
    dict(vec![
        ("Type", PdfObject::Name("Annot".to_string())),
        ("Subtype", PdfObject::Name(subtype.to_string())),
        ("T", literal(title)),
        ("Rect", PdfObject::Array(vec![PdfObject::Int(0), PdfObject::Int(0), PdfObject::Int(144), PdfObject::Int(96)])),
    ])
}

/// 📎️ Adds a `/Type /Filespec` with a real `/EF` attached-file stream and NO `/AFRelationship` —
/// the exact shape ISO 19005-3 requires the relationship key on and ISO 19005-2 forbids outright.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_file_spec(snapshot: &mut PdfSnapshot, file_name: &str) -> ObjRef {
    let payload = insert_object(snapshot, PdfObject::Stream { dict: Vec::new(), data: format!("attached payload for {file_name}").into_bytes(), filters: Vec::new() });
    let spec = dict(vec![
        ("Type", PdfObject::Name("Filespec".to_string())),
        ("F", literal(file_name)),
        ("UF", literal(file_name)),
        ("EF", single_entry_dict("F", PdfObject::Ref(payload))),
    ]);
    insert_object(snapshot, spec)
}

/// 🌲️ An empty but well-formed `/Type /StructTreeRoot` — PDF/UA's structure tree in its minimal
/// legitimate form, which is what `check_ua_conformance`'s presence check reads.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn struct_tree_root_object() -> PdfObject {
    dict(vec![("Type", PdfObject::Name("StructTreeRoot".to_string())), ("K", PdfObject::Array(Vec::new()))])
}

/// ✍️ Adds a `/FT /Sig` field titled `title` to `/Root/AcroForm/Fields`, creating the form when
/// the document has none.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn insert_signature_field(snapshot: &mut PdfSnapshot, title: &str) {
    let field = insert_object(snapshot, dict(vec![("FT", PdfObject::Name("Sig".to_string())), ("T", literal(title))]));
    let mut fields = signature_fields(snapshot);
    fields.push(field);
    set_acro_form_fields(snapshot, fields);
}

/// ✍️ Drops the `/FT /Sig` field titled `title`, and the whole `/AcroForm` with it when it was
/// the last one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn remove_signature_field(snapshot: &mut PdfSnapshot, title: &str) {
    let Some(field) = signature_field_named(snapshot, title) else { return };
    remove_object(snapshot, field);
    let fields = signature_fields(snapshot);
    set_acro_form_fields(snapshot, fields);
}

/// 🗂️ Installs `/Root/DPartRoot` over one `/Type /DPart` node, carrying `/DPM << /Job … >>` when
/// `job` is non-empty — ISO 16612-2's variable-data partitioning in its minimal legitimate form.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_dpart_root(snapshot: &mut PdfSnapshot, job: &str) {
    let mut node = vec![("Type", PdfObject::Name("DPart".to_string()))];
    if !job.is_empty() {
        node.push(("DPM", single_entry_dict("Job", literal(job))));
    }
    let node_id = insert_object(snapshot, dict(node));
    let root = insert_object(snapshot, dict(vec![("Type", PdfObject::Name("DPartRoot".to_string())), ("DPartRootNode", PdfObject::Ref(node_id))]));
    set_catalog_entry(snapshot, "DPartRoot", PdfObject::Ref(root));
}

/// 🗂️ Rewrites the root `/DPart` node's `/DPM`, or drops it when `job` is `None`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn set_dpart_job(snapshot: &mut PdfSnapshot, job: Option<&str>) {
    let Some(node) = dpart_root_node(snapshot) else { return };
    match job {
        Some(value) => set_entry(snapshot, node, "DPM", single_entry_dict("Job", literal(value))),
        None => remove_entry(snapshot, node, "DPM"),
    }
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
//#endregion 🏅️ConformanceSupport
