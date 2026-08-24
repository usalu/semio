//! 🦀️ PDF 1.7 exhaustive mutation case — Rust adapter. Ticket 26/08/23/END-TO-END-TESTING-REFACTOR
//! wave 7.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory
//! first; the committed asset is never written to. `oracle` drives the registered `lopdf` reference
//! implementation (`../../🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_pdf`/`encode_pdf`/`apply_pdf_mutation` over the full 18-kind `PdfMutation` vocabulary.
//! Both results are read back by the SAME independent `project_pdf_1_7` (`lopdf`, augmented with
//! each page's `/CropBox` and `/Rotate` and with the resolved trailer/catalog object graph) before
//! the `semantic-pdf-v1` profile compares them. The subject half is gated behind the generated
//! host's `sut` feature so the oracle-only run never compiles the local implementation.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module and under `semantic-pdf-v1`'s own tolerance, so a scenario cannot pass merely because
//! `lopdf` declined to error: `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>`
//! must land back on the untouched document's projection, and `identity-round-trip` must both
//! preserve the projection and produce bytes that differ from the input. The two carve-outs — one
//! kind exempt from observability, one axis exempt from the inverse law for three kinds — are named
//! by the subset's own oracle module (`UNOBSERVABLE`, `regenerates_page_content`), argued there in
//! full, and repeated in this case's feature description.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_pdf_1_7, regenerates_page_content, without_content_operators, KINDS, UNOBSERVABLE};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores_within, mutation_is_observable_within, reparsed_not_copied, round_trip_preserves_within};

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🈳️ The `no-mutation` spec, which is how the identity round trip asks the reference to parse and
/// re-serialize the real document without changing anything.
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}
//#endregion 🔖️Input

//#region 🔖️Profile
/// 📏️ `semantic-pdf-v1`'s own declared freedom list and tolerance (`../../../../🧪️oracle/
/// 🔣️component.json`), mirrored here so an in-handler law check is exactly as strict as the profile
/// the case is measured by — never stricter, which would invent a failure the comparison itself
/// would forgive, and never looser, which would let a real one through.
const PDF_WRITER_FREEDOM: &[&str] = &["objectNumber", "xrefOffset", "producer", "creationDate", "modificationDate", "documentId", "fileSize", "byteLength", "generation", "streamFilter", "streamLength"];
const PDF_TOLERANCE: f64 = 0.0001;
//#endregion 🔖️Profile

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference applies the kind
/// to the real thesis and the result has to differ from the untouched document under the very
/// profile the case is measured by. Returning the projection uncompared is what made these
/// eighteen scenarios pass whenever `lopdf` merely did not error. The one exemption is this
/// subset's own [`UNOBSERVABLE`], which names `insert-object` and says why in full.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_pdf_1_7(&bytes)?;
    mutation_is_observable_within(&spec.str("kind"), &projection, &project_pdf_1_7(&input)?, UNOBSERVABLE, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL document's own projection, read through the same
/// independent reader. `regenerates_page_content`'s three kinds drop `pages.N.contentOperators`
/// from BOTH sides and nothing else — that carve-out lives in the subset's oracle module, next to
/// the reason for it, so this handler and the module's own law test can never exempt different
/// things.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let original = project_pdf_1_7(&input)?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_pdf_1_7(&bytes)?;
    let (expected, restored) = if regenerates_page_content(&kind) { (without_content_operators(&original), without_content_operators(&projection)) } else { (original, projection.clone()) };
    inverse_restores_within(&kind, &restored, &expected, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: `lopdf` fully parses the real
/// document and re-serializes it from its own object graph alone (the same `no-mutation` routing
/// every other kind goes through), the re-serialized bytes must differ from the input — our
/// encoder cannot reproduce another writer's object layout, so bit-identical output would mean the
/// input was smuggled rather than parsed — and the projection must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_pdf_1_7(&bytes)?;
    round_trip_preserves_within(&projection, &project_pdf_1_7(&input)?, PDF_WRITER_FREEDOM, PDF_TOLERANCE)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::{decode_pdf, encode_pdf};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::diff::PdfPathSegment;
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::mutations::{apply_pdf_mutation, PdfMutation};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfDecimal, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::any::project_pdf_1_7;

    //#region 🔖️SpecCodec
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }

    fn str_field(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    fn media_box_field(value: &Json, key: &str) -> Option<[f64; 4]> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 4 => {
                let n: Vec<f64> = items
                    .iter()
                    .map(|item| match item {
                        Json::Number(number) => *number,
                        _ => 0.0,
                    })
                    .collect();
                Some([n[0], n[1], n[2], n[3]])
            }
            _ => None,
        }
    }

    fn object_id_field(value: &Json) -> ObjRef {
        let id = value.get("id").cloned().unwrap_or_else(|| value.clone());
        ObjRef { num: number_field(&id, "num") as u32, gen: number_field(&id, "gen") as u16 }
    }

    fn path_field(items: Vec<Json>) -> Vec<PdfPathSegment> {
        items
            .iter()
            .filter_map(|segment| match segment.str("kind").as_str() {
                "index" => Some(PdfPathSegment::ArrayIndex { index: usize_field(segment, "index") }),
                "key" => Some(PdfPathSegment::DictKey { key: segment.str("key") }),
                _ => None,
            })
            .collect()
    }

    /// 🔎️ The same owned PDF-object JSON grammar the oracle side speaks
    /// (`{"kind":"null"|"bool"|"int"|"real"|"str"|"name"|"array"|"dict"|"ref", ...}`), decoded into
    /// the PRODUCTION `PdfObject` here instead of `lopdf::Object`.
    fn json_to_pdf_object(value: &Json) -> PdfObject {
        match value.str("kind").as_str() {
            "bool" => PdfObject::Bool(matches!(value.get("value"), Some(Json::Bool(true)))),
            "int" => PdfObject::Int(number_field(value, "value") as i64),
            "real" => PdfObject::Real(PdfDecimal::from(number_field(value, "value"))),
            "str" => PdfObject::Str(value.str("value").into_bytes()),
            "name" => PdfObject::Name(value.str("value")),
            "array" => PdfObject::Array(value.array("items").iter().map(json_to_pdf_object).collect()),
            "dict" => PdfObject::Dict(value.array("entries").iter().map(|entry| PdfDictEntry { key: entry.str("key"), value: json_to_pdf_object(entry.get("value").unwrap_or(&Json::Null)) }).collect()),
            "ref" => PdfObject::Ref(object_id_field(value)),
            _ => PdfObject::Null,
        }
    }

    fn json_to_pdf_page(value: &Json) -> PdfPage {
        PdfPage { media_box: media_box_field(value, "mediaBox").unwrap_or([0.0, 0.0, 612.0, 792.0]), crop_box: media_box_field(value, "cropBox"), rotate: number_field(value, "rotate") as i32, text: value.str("text") }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `PdfMutation` this subset
    /// declares for it. `set-snapshot` mirrors the oracle's own extension of "replace metadata" --
    /// only `declaredVersion`/`title` are spec-driven, applied on top of the currently decoded
    /// `base` snapshot so every OTHER field (pages, objects, trailer) survives untouched.
    fn mutation_from_spec(spec: &Json, base: &PdfSnapshot) -> Result<PdfMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(PdfMutation::NoMutation),
            "set-snapshot" => {
                let mut snapshot = base.clone();
                if let Some(version) = str_field(&params, "declaredVersion") {
                    snapshot.declared_version = version;
                }
                if let Some(title) = str_field(&params, "title") {
                    snapshot.info.title = Some(title);
                }
                Ok(PdfMutation::SetSnapshot { snapshot })
            }
            "insert-page" => Ok(PdfMutation::InsertPage { index: usize_field(&params, "index"), page: json_to_pdf_page(&params.get("page").cloned().unwrap_or(Json::Null)) }),
            "remove-page" => Ok(PdfMutation::RemovePage { index: usize_field(&params, "index") }),
            "set-page-media-box" => Ok(PdfMutation::SetPageMediaBox { index: usize_field(&params, "index"), media_box: media_box_field(&params, "mediaBox").unwrap_or([0.0, 0.0, 612.0, 792.0]) }),
            "set-page-crop-box" => Ok(PdfMutation::SetPageCropBox { index: usize_field(&params, "index"), crop_box: media_box_field(&params, "cropBox") }),
            "append-page-content" => Ok(PdfMutation::AppendPageContent { index: usize_field(&params, "index"), text: params.str("text") }),
            "set-info" => Ok(PdfMutation::SetInfo { info: PdfInfo { title: str_field(&params, "title"), author: str_field(&params, "author"), ..Default::default() } }),
            "insert-object" => Ok(PdfMutation::InsertObject { id: object_id_field(&params), value: json_to_pdf_object(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "remove-object" => Ok(PdfMutation::RemoveObject { id: object_id_field(&params) }),
            "set-object-value" => Ok(PdfMutation::SetObjectValue { id: object_id_field(&params), value: json_to_pdf_object(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "set-dict-entry" => Ok(PdfMutation::SetDictEntry { id: object_id_field(&params), path: path_field(params.array("path")), key: params.str("key"), value: json_to_pdf_object(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "remove-dict-entry" => Ok(PdfMutation::RemoveDictEntry { id: object_id_field(&params), path: path_field(params.array("path")), key: params.str("key") }),
            "set-trailer-entry" => Ok(PdfMutation::SetTrailerEntry { key: params.str("key"), value: json_to_pdf_object(&params.get("value").cloned().unwrap_or(Json::Null)) }),
            "remove-trailer-entry" => Ok(PdfMutation::RemoveTrailerEntry { key: params.str("key") }),
            "move-page" => Ok(PdfMutation::MovePage { from: usize_field(&params, "from"), to: usize_field(&params, "to") }),
            "set-page-content" => Ok(PdfMutation::SetPageContent { index: usize_field(&params, "index"), text: params.str("text") }),
            "set-page-rotation" => Ok(PdfMutation::SetPageRotation { index: usize_field(&params, "index"), rotation: number_field(&params, "rotation") as u16 }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    /// 🔍️ Read-only lookup of the CURRENT value at `key` inside object `id`'s value tree at `path` --
    /// verbatim copy of `../../🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s
    /// own private `original_dict_value`, duplicated here (not imported) for the same reason
    /// `mutate-pdf-1-4`'s own `inverse_of` gives: written in closed form so this adapter needs no
    /// extra crate dependency beyond `semio-s-plugin-stdio` itself.
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

    /// ↩️ `PdfMutation::inverse` in closed form (same source as `original_dict_value` above) --
    /// every variant's own `Mutation::inverse` arm, transplanted rather than called through the
    /// trait.
    fn inverse_of(mutation: &PdfMutation, base: &PdfSnapshot) -> PdfMutation {
        match mutation {
            PdfMutation::NoMutation => PdfMutation::NoMutation,
            PdfMutation::SetSnapshot { .. } => PdfMutation::SetSnapshot { snapshot: base.clone() },
            PdfMutation::InsertPage { index, .. } => PdfMutation::RemovePage { index: *index },
            PdfMutation::RemovePage { index } => match base.pages.get(*index) {
                Some(page) => PdfMutation::InsertPage { index: *index, page: page.clone() },
                None => PdfMutation::NoMutation,
            },
            PdfMutation::SetPageMediaBox { index, .. } => PdfMutation::SetPageMediaBox { index: *index, media_box: base.pages.get(*index).map(|page| page.media_box).unwrap_or([0.0, 0.0, 612.0, 792.0]) },
            PdfMutation::SetPageCropBox { index, .. } => PdfMutation::SetPageCropBox { index: *index, crop_box: base.pages.get(*index).and_then(|page| page.crop_box) },
            PdfMutation::AppendPageContent { index, .. } => PdfMutation::SetPageContent { index: *index, text: base.pages.get(*index).map(|page| page.text.clone()).unwrap_or_default() },
            PdfMutation::SetInfo { .. } => PdfMutation::SetInfo { info: base.info.clone() },
            PdfMutation::InsertObject { id, .. } => PdfMutation::RemoveObject { id: *id },
            PdfMutation::RemoveObject { id } => match base.objects.iter().find(|o| o.id == *id) {
                Some(o) => PdfMutation::InsertObject { id: *id, value: o.value.clone() },
                None => PdfMutation::NoMutation,
            },
            PdfMutation::SetObjectValue { id, .. } => match base.objects.iter().find(|o| o.id == *id) {
                Some(o) => PdfMutation::SetObjectValue { id: *id, value: o.value.clone() },
                None => PdfMutation::RemoveObject { id: *id },
            },
            PdfMutation::SetDictEntry { id, path, key, .. } => match original_dict_value(base, *id, path, key) {
                Some(value) => PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value },
                None => PdfMutation::RemoveDictEntry { id: *id, path: path.clone(), key: key.clone() },
            },
            PdfMutation::RemoveDictEntry { id, path, key } => match original_dict_value(base, *id, path, key) {
                Some(value) => PdfMutation::SetDictEntry { id: *id, path: path.clone(), key: key.clone(), value },
                None => PdfMutation::NoMutation,
            },
            PdfMutation::SetTrailerEntry { key, .. } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() },
                None => PdfMutation::RemoveTrailerEntry { key: key.clone() },
            },
            PdfMutation::RemoveTrailerEntry { key } => match base.trailer.iter().find(|e| e.key == *key) {
                Some(e) => PdfMutation::SetTrailerEntry { key: key.clone(), value: e.value.clone() },
                None => PdfMutation::NoMutation,
            },
            PdfMutation::MovePage { from, to } => match base.pages.get(*from) {
                Some(_) => PdfMutation::MovePage { from: (*to).min(base.pages.len().saturating_sub(1)), to: *from },
                None => PdfMutation::NoMutation,
            },
            PdfMutation::SetPageContent { index, .. } => PdfMutation::SetPageContent { index: *index, text: base.pages.get(*index).map(|page| page.text.clone()).unwrap_or_default() },
            PdfMutation::SetPageRotation { index, .. } => PdfMutation::SetPageRotation { index: *index, rotation: base.pages.get(*index).map(|page| page.rotate).unwrap_or(0).rem_euclid(360) as u16 },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_pdf_mutation(&mut snapshot, &mutation);
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        let projection = project_pdf_1_7(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_pdf_mutation(&mut snapshot, &mutation);
        apply_pdf_mutation(&mut snapshot, &undo);
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        let projection = project_pdf_1_7(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_pdf`/`encode_pdf` are this
    /// subset's ONLY channel from input to output (no separate text-DSL layer over the snapshot).
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_pdf(&input).map_err(|error| format!("decode_pdf failed: {error:?}"))?;
        let output = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {error:?}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_pdf_1_7(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 18-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 18 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
