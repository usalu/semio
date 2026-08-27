//! 🔮️ Mutation oracle for this subset — every CONFORMANCE-CLASS mutation the `docx` `✳️strict`
//! subset declares, performed by the registered `quick-xml` 0.42 reference implementation over the
//! `zip` 6 container codec, so the subject's own mutation has an independent result to be compared
//! against instead of being checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it is not a copy of `✳️any`.** The `✳️any` subset
//! owns the DOCUMENT vocabulary — sheets, cells, blocks, runs, slides, shapes. This subset owns the
//! ISO/IEC 29500-1 Strict CONFORMANCE CLASS, which is a property of the OPC package and of no
//! document object at all. `check_strict_conformance` reads six axes on an already-decoded `DocxSnapshot`: the main document part's Strict WordprocessingML namespace, the presence of the Transitional namespace anywhere in the package, the presence of the VML namespace anywhere in the package, the `officeDocument` relationship base of every relationship, the main part's root `conformance="strict"`, and `mc:AlternateContent` compatibility markup. One kind per axis.
//!
//! No `✳️any` mutation moves any of those axes, and no mutation here touches document content: the
//! two vocabularies are disjoint by construction, which is exactly why this subset needs its own.
//!
//! The implementation lives in the shared `document::ooxml` family module because all six
//! `✳️strict`/`✳️transitional` OOXML subsets genuinely share it — see that module's own doc comment.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `DocxStrictMutation` declares, in
/// declaration order. The catalog `docx-ecma-376-strict` is measured against this exact list, and the
/// subject-side `KINDS` carries the test that proves enum, constant and manifest never drift apart.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute", "insert-vml-part", "remove-vml-part", "insert-alternate-content", "remove-alternate-content"];
//#endregion 🔖️Vocabulary

//#region 🔖️Profile
/// 🏅️ This artifact's conformance-class coordinates, `[transitional, strict]` per pair.
#[cfg(feature = "oracles")]
const PROFILE: crate::document::ooxml::OoxmlProfile = crate::document::ooxml::OoxmlProfile {
    format: "docx",
    main_namespaces: ["http://schemas.openxmlformats.org/wordprocessingml/2006/main", "http://purl.oclc.org/ooxml/wordprocessingml/main"],
    drawing_namespaces: None,
    relationship_namespaces: ["http://schemas.openxmlformats.org/officeDocument/2006/relationships", "http://purl.oclc.org/ooxml/officeDocument/relationships"],
    relationship_bases: ["http://schemas.openxmlformats.org/officeDocument/2006/relationships", "http://purl.oclc.org/ooxml/officeDocument/relationships"],
    vml_content_type: "application/vnd.openxmlformats-officedocument.vmlDrawing",
};
//#endregion 🔖️Profile

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real package and returns the re-serialized bytes. A
/// kind this subset does not declare is an error even when the shared engine could perform it — the
/// gate is the subset's own vocabulary, not the engine's capability.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if !KINDS.contains(&kind.as_str()) {
        return Err(format!("mutation kind {kind:?} is not declared by the docx-ecma-376-strict catalog"));
    }
    crate::document::ooxml::apply_conformance_mutation(input, spec, &PROFILE)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 🔁️ The reference implementation's own decode/re-encode of the container.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    crate::document::ooxml::round_trip(input)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// 👁️ Projects package bytes onto the conformance-class shape this case's oracle and subject are
/// both compared through. Every field is read back out of the BYTES by the independent
/// implementation; nothing is carried by the caller.
#[cfg(feature = "oracles")]
pub fn project_package(input: &[u8]) -> Result<Json, String> {
    crate::document::ooxml::project(input, "docx")
}

#[cfg(not(feature = "oracles"))]
pub fn project_package(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Bridge
/// 🎬️ Prepares the input a removal kind needs its target to be present in — see the shared engine's
/// [`crate::document::ooxml::conformance_arrange`]. Every other kind reads the committed bytes.
#[cfg(feature = "oracles")]
pub fn oracle_arrange(input: &[u8], forward: &Json) -> Result<Vec<u8>, String> {
    crate::document::ooxml::conformance_arrange(input, forward, &PROFILE)
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_arrange(_input: &[u8], _forward: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

/// ↩️ The undo of `forward`, read out of `base` by the independent implementation alone.
#[cfg(feature = "oracles")]
pub fn oracle_inverse_spec(base: &[u8], forward: &Json) -> Result<Json, String> {
    crate::document::ooxml::conformance_inverse_spec(base, forward, &PROFILE)
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

    /// 🧫️ The real committed package this subset's case runs on, read where the artifact keeps it.
    const FIXTURE: &[u8] = include_bytes!("../../../../../🧫️fixtures/📜️example-readme.docx");

    fn json_object(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    /// 🧾️ One representative parameter set per declared kind — the same rows the case's feature file
    /// carries, so a failure here and a failure there have the same cause.
    fn params_for(kind: &str) -> Json {
        match kind {
            "no-mutation" => json_object(vec![]),
            "set-snapshot" => json_object(vec![("conformanceClass", Json::String("strict".to_string()))]),
            "set-main-namespace" => json_object(vec![("namespace", Json::String("http://purl.oclc.org/ooxml/wordprocessingml/main".to_string()))]),
            "set-relationship-base" => json_object(vec![("base", Json::String("http://purl.oclc.org/ooxml/officeDocument/relationships".to_string()))]),
            "set-conformance-attribute" => json_object(vec![("value", Json::String("strict".to_string()))]),
            "remove-conformance-attribute" => json_object(vec![]),
            "insert-vml-part" => json_object(vec![("path", Json::String("word/vmlDrawing1.vml".to_string()))]),
            "remove-vml-part" => json_object(vec![("path", Json::String("word/vmlDrawing1.vml".to_string()))]),
            "insert-alternate-content" => json_object(vec![("path", Json::String("word/document.xml".to_string()))]),
            "remove-alternate-content" => json_object(vec![("path", Json::String("word/document.xml".to_string()))]),
            other => panic!("no test parameters for kind {other:?}"),
        }
    }

    fn spec(kind: &str) -> Json {
        json_object(vec![("kind", Json::String(kind.to_string())), ("params", params_for(kind))])
    }

    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_package() {
        let original = FIXTURE.to_vec();
        for kind in KINDS {
            let forward = spec(kind);
            let base = oracle_arrange(&original, &forward).unwrap_or_else(|error| panic!("{kind}: arrange failed: {error}"));
            let base_projection = project_package(&base).unwrap_or_else(|error| panic!("{kind}: projecting the base failed: {error}"));
            let mutated = oracle_apply_mutation(&base, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let mutated_projection = project_package(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if *kind != "no-mutation" {
                assert_ne!(mutated_projection, base_projection, "{kind} must be observable in the conformance-class projection");
            }
            let undo = oracle_inverse_spec(&base, &forward).unwrap_or_else(|error| panic!("{kind}: inverse spec: {error}"));
            let restored = oracle_apply_mutation(&mutated, &undo).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            let restored_projection = project_package(&restored).unwrap_or_else(|error| panic!("{kind}: projecting the restored package failed: {error}"));
            assert_eq!(restored_projection, base_projection, "{kind}: undoing the mutation must restore the package's conformance-class projection");
        }
    }

    #[test]
    fn no_mutation_is_a_true_byte_identity() {
        assert_eq!(oracle_apply_mutation(FIXTURE, &spec("no-mutation")).unwrap(), FIXTURE.to_vec());
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        assert!(oracle_apply_mutation(FIXTURE, &json_object(vec![("kind", Json::String("not-a-real-kind".to_string())), ("params", json_object(vec![]))])).is_err());
    }

    #[test]
    fn a_kind_this_subset_does_not_declare_is_refused_even_when_the_engine_could_perform_it() {
        let undeclared = ["set-snapshot", "set-main-namespace", "set-drawing-namespace", "set-relationships-namespace", "set-relationship-base", "insert-vml-part", "insert-alternate-content", "set-worksheet-content-type"].into_iter().find(|kind| !KINDS.contains(kind));
        let Some(kind) = undeclared else { return };
        let spec = json_object(vec![("kind", Json::String(kind.to_string())), ("params", json_object(vec![]))]);
        assert!(oracle_apply_mutation(FIXTURE, &spec).is_err(), "{kind} is not in this subset's vocabulary and must be refused");
    }

    #[test]
    fn the_container_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let rebuilt = oracle_round_trip(FIXTURE).unwrap();
        assert_ne!(rebuilt, FIXTURE.to_vec(), "the reference rebuilds the container; identical bytes would mean the input was smuggled");
        assert_eq!(project_package(&rebuilt).unwrap(), project_package(FIXTURE).unwrap());
    }
}
//#endregion 🧪️Tests
