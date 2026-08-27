//! 🔮️ Mutation oracle for this subset — every CONFORMANCE-CLASS mutation the `pdf` `1.7`/`✳️a`
//! subset declares, performed by the registered `lopdf` 0.44 reference implementation over the real
//! object graph of a real document, so the subject's own mutation has an independent result to be
//! compared against instead of being checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it is not a copy of `✳️any`.** The `✳️any` subset owns
//! the DOCUMENT vocabulary — pages, media boxes, page content, `/Info` as authoring metadata, and the
//! raw object/dict/trailer edit primitives. This subset owns ISO 19005-2 / ISO 19005-3 (PDF/A-2, PDF/A-3), a CONFORMANCE CLASS,
//! which is a property of the object graph as a whole and of no page at all. Its vocabulary is
//! derived one kind per axis from this subset's OWN `check_pdf_a_conformance`
//! (`../🧬️schema/🦀️component.rs`), which reads six axes: any Standard Security Handler `/Encrypt` dictionary object, any `/S /JavaScript` action or bare `/JS` key, any `/S /Launch` action, any `/Type /Filespec` carrying `/EF` without an `/AFRelationship`, an `/S /GTS_PDFA1` OutputIntent reachable from `/Root/OutputIntents`, and a `/FontFile`, `/FontFile2` or `/FontFile3` embedded program on every font's `/FontDescriptor`.
//!
//! What no other PDF subset here shares is the EMBEDDED-FILE pair. `/AFRelationship` is the single signal `detect_pdfa_level` uses to tell ISO 19005-3 (Part 3) from ISO 19005-2 (Part 2), and the checker's own hard rule is `/EF` present WITHOUT it; `set-af-relationship`/`remove-af-relationship` move exactly that bit, and no other subset in this standard declares them because no other subset's checker reads a Filespec at all.
//!
//! No `✳️any` mutation moves any of those axes, and no mutation here touches page content: the two
//! vocabularies are disjoint by construction, which is exactly why this subset needs its own.
//!
//! The implementation lives in the shared `document::pdf_conformance` family module because all six
//! PDF 1.7 conformance subsets genuinely share the MECHANISM — every axis is a fact of the COS object
//! graph — see that module's own doc comment. What is NOT shared is which axes this subset polices,
//! which marker its OutputIntent demands and which kinds it declares: those are [`PROFILE`] and
//! [`KINDS`] below, and a kind this subset does not declare is refused here even when the engine
//! could perform it.
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/🦀️component.rs — `check_pdf_a_conformance`, the one axis list everything here derives from.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfAMutation` declares, in
/// declaration order. The catalog `pdf-1-7-a` is measured against this exact list, and the
/// subject-side derived catalog test proves aggregate, direct descriptors, and manifest never drift apart.
pub const KINDS: &[&str] = &["insert-encryption-dictionary", "remove-encryption-dictionary", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-embedded-file", "remove-embedded-file", "set-af-relationship", "remove-af-relationship", "set-output-intent", "remove-output-intent", "embed-font-file", "remove-font-file"];
//#endregion 🔖️Vocabulary

//#region 🔖️Profile
/// 🏅️ This subset's conformance coordinates: the axes `check_pdf_a_conformance` reads, the OutputIntent
/// marker it demands, and whether that intent must carry a `/DestOutputProfile`. The projection is
/// scoped to exactly these axes, so this subset is never judged on an axis its own checker ignores.
#[cfg(feature = "oracles")]
pub const PROFILE: crate::document::pdf_conformance::PdfConformanceProfile = crate::document::pdf_conformance::PdfConformanceProfile {
    subset: "a",
    output_intent_subtype: "GTS_PDFA1",
    output_intent_dest_profile: true,
    conformant_title: "A PDF/A conformant document",
    axes: &["encryptionDictionaries", "javaScriptActions", "launchActions", "embeddedFiles", "outputIntents", "fontPrograms"],
};
//#endregion 🔖️Profile

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real document and returns the re-serialized bytes. A
/// kind this subset does not declare is an error even when the shared engine could perform it — the
/// gate is the subset's own vocabulary, not the engine's capability.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("mutation kind {kind:?} is not declared by the pdf-1-7-a catalog"));
    }
    crate::document::pdf_conformance::apply_conformance_mutation(input, spec, &PROFILE)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The reference implementation's own decode/re-encode of the whole object graph.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    crate::document::pdf_conformance::round_trip(input)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 👁️ Projects document bytes onto this subset's own conformance-class shape. Every field is read
/// back out of the BYTES by the independent implementation; nothing is carried by the caller.
#[cfg(feature = "oracles")]
pub fn project_conformance(input: &[u8]) -> Result<Json, String> {
    crate::document::pdf_conformance::project(input, &PROFILE)
}

#[cfg(not(feature = "oracles"))]
pub fn project_conformance(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Bridge
/// 🎬️ Prepares the input a kind needs its target to exist in — see the shared engine's
/// [`crate::document::pdf_conformance::conformance_arrange`]. Every other kind reads the committed
/// bytes untouched.
#[cfg(feature = "oracles")]
pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    crate::document::pdf_conformance::conformance_arrange(input, forward, &PROFILE)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_arrange(_input: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of `forward`, read out of `base` by the independent implementation alone.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    crate::document::pdf_conformance::conformance_inverse_spec(base, forward, &PROFILE)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_inverse_spec(_base: &[u8], _forward: &Json) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Bridge

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;

    /// 🧫️ The real committed document this subset's case runs on, read where the artifact already
    /// keeps it — a 6.3 MB, 65-page LaTeX bachelor thesis with 3,189 indirect objects and 23
    /// `/FontDescriptor` objects, every one of them carrying an embedded font program.
    const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    /// 🧾️ One representative parameter set per declared kind — the same rows the case's feature file
    /// carries, so a failure here and a failure there have the same cause.
    fn params_for(kind: &str) -> Json {
        match kind {
            "insert-encryption-dictionary" => json_object(vec![("version", Json::Number(2.0)), ("revision", Json::Number(3.0))]),
            "remove-encryption-dictionary" => json_object(vec![("version", Json::Number(2.0)), ("revision", Json::Number(3.0))]),
            "insert-javascript-action" => json_object(vec![("script", Json::String("app.alert('this document phones home');".to_string()))]),
            "remove-javascript-action" => json_object(vec![("script", Json::String("app.alert('this document phones home');".to_string()))]),
            "insert-launch-action" => json_object(vec![("target", Json::String("render-plots.bat".to_string()))]),
            "remove-launch-action" => json_object(vec![("target", Json::String("render-plots.bat".to_string()))]),
            "insert-embedded-file" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
            "remove-embedded-file" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
            "set-af-relationship" => json_object(vec![("fileName", Json::String("measurements.csv".to_string())), ("relationship", Json::String("Data".to_string()))]),
            "remove-af-relationship" => json_object(vec![("fileName", Json::String("measurements.csv".to_string()))]),
            "set-output-intent" => json_object(vec![("identifier", Json::String("sRGB IEC61966-2.1".to_string()))]),
            "remove-output-intent" => json_object(vec![]),
            "embed-font-file" => json_object(vec![("descriptorOrdinal", Json::Number(4.0)), ("key", Json::String("FontFile2".to_string())), ("programOrdinal", Json::Number(0.0))]),
            "remove-font-file" => json_object(vec![("descriptorOrdinal", Json::Number(4.0))]),
            other => panic!("no test parameters for kind {other:?}"),
        }
    }

    fn spec(kind: &str) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
    }

    fn fixture() -> Vec<u8> {
        std::fs::read(FIXTURE).expect("the committed bachelor-thesis document")
    }

    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let original = fixture();
        for kind in KINDS {
            let forward = spec(kind);
            let base = oracle_arrange(&original, &forward).unwrap_or_else(|error| panic!("{kind}: arrange failed: {error}"));
            let base_projection = project_conformance(&base).unwrap_or_else(|error| panic!("{kind}: projecting the base failed: {error}"));
            let mutated = oracle_apply_mutation(&base, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let mutated_projection = project_conformance(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");
            let undo = oracle_inverse_spec(&base, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
            let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            let restored_projection = project_conformance(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored document failed: {error}"));
            assert_eq!(restored_projection, base_projection, "{kind}: undoing the mutation must restore the conformance-class projection");
        }
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let spec = json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err());
    }

    #[test]
    fn a_kind_this_subset_does_not_declare_is_refused_even_when_the_engine_could_perform_it() {
        let undeclared = ["insert-media-annotation", "insert-signature-field", "remove-display-doc-title", "remove-dpart-metadata", "remove-dpart-root", "remove-lang", "remove-mark-info", "remove-media-annotation", "remove-signature-field", "remove-struct-tree-root", "remove-trim-box", "set-display-doc-title", "set-dpart-metadata", "set-dpart-root", "set-info-author", "set-info-title", "set-lang", "set-mark-info", "set-struct-tree-root", "set-trim-box"];
        let sample = undeclared.into_iter().find(|kind| !KINDS.contains(kind)).expect("the sibling subsets declare at least one kind this one does not");
        let spec = json_object(vec![("kind", Json::String(sample.to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(&fixture(), &spec).is_err(), "{sample} is not in this subset's vocabulary and must be refused");
    }

    #[test]
    fn the_object_graph_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let original = fixture();
        let rebuilt = oracle_round_trip(&original).expect("the reference implementation re-serializes the document");
        assert_ne!(rebuilt, original, "the reference rebuilds the file from its own object graph; identical bytes would mean the input was smuggled");
        assert_eq!(project_conformance(&rebuilt).unwrap(), project_conformance(&original).unwrap());
    }
}
//#endregion 🧪️Tests
