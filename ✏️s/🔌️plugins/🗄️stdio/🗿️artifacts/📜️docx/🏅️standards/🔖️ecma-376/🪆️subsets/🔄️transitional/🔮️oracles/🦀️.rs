//! 🔮️ Mutation oracle for this subset — every CONFORMANCE-CLASS mutation the `docx` `🔄️transitional`
//! subset declares, performed by the registered `quick-xml` 0.42 reference implementation over the
//! `zip` 6 container codec, so the subject's own mutation has an independent result to be compared
//! against instead of being checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it is not a copy of `✳️any`.** The `✳️any` subset
//! owns the DOCUMENT vocabulary — sheets, cells, blocks, runs, slides, shapes. This subset owns the
//! ISO/IEC 29500-4 Transitional CONFORMANCE CLASS, which is a property of the OPC package and of no
//! document object at all. `check_transitional_conformance` reads three axes: the main document part's Transitional WordprocessingML namespace, the presence of ANY strict-family (`purl.oclc.org/ooxml`) namespace in a part or a relationship type, and a root `conformance="strict"` that would contradict the stamp. VML and `mc:AlternateContent` are legal Transitional markup and are not policed — which is why this vocabulary declares four kinds fewer than its 📏️strict sibling.
//!
//! No `✳️any` mutation moves any of those axes, and no mutation here touches document content: the
//! two vocabularies are disjoint by construction, which is exactly why this subset needs its own.
//!
//! The implementation lives in the shared `document::ooxml` family module because all six
//! `📏️strict`/`🔄️transitional` OOXML subsets genuinely share it — see that module's own doc comment.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `DocxTransitionalMutation` declares, in
/// declaration order. The catalog `docx-ecma-376-transitional` is measured against this exact list, and the
/// subject-side `KINDS` carries the test that proves enum, constant and manifest never drift apart.
pub const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-main-namespace", "set-relationship-base", "set-conformance-attribute", "remove-conformance-attribute"];
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
        return Err(format!("mutation kind {kind:?} is not declared by the docx-ecma-376-transitional catalog"));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
