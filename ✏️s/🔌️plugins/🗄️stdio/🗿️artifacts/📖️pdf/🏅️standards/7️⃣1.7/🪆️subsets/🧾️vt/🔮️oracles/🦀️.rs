//! 🔮️ Mutation oracle for this subset — every CONFORMANCE-CLASS mutation the `pdf` `1.7`/`🧾️vt`
//! subset declares, performed by the registered `lopdf` 0.44 reference implementation over the real
//! object graph of a real document, so the subject's own mutation has an independent result to be
//! compared against instead of being checked against its own reading.
//!
//! **What this subset's vocabulary is, and why it is not a copy of `🧱️base`.** The `🧱️base` subset owns
//! the DOCUMENT vocabulary — pages, media boxes, page content, `/Info` as authoring metadata, and the
//! raw object/dict/trailer edit primitives. This subset owns ISO 16612-2 (PDF/VT-1), a CONFORMANCE CLASS,
//! which is a property of the object graph as a whole and of no page at all. Its vocabulary is
//! derived one kind per axis from this subset's OWN `check_vt_conformance`
//! (`../🧬️schema/🦀️.rs`), which reads every axis `check_x_conformance` reads, plus two of its own: `/Root/DPartRoot` (hard) and a `/DPM` metadata dictionary on every `/DPart` node reachable from it (soft).
//!
//! This is the one place in this artifact where a vocabulary is a strict SUPERSET of a sibling's, and it is so by the subset's own code rather than by copying: `check_vt_conformance`'s first statement is literally `let mut out = check_x_conformance(snapshot);`, because ISO 16612-2 is defined ON TOP of ISO 15930 — a PDF/VT file is a PDF/X file with a document-part hierarchy. The fourteen inherited kinds are therefore not duplicated prose but a stated inheritance, and the four that are this subset's own — the `/DPartRoot` pair and the `/DPM` pair — are the variable-data partitioning mechanism no other conformance class in this standard has any concept of. The implementation is shared through the named `document::pdf_conformance` engine, never copied: what differs between `🖨️x` and `🧾️vt` is the declared axis list and the declared vocabulary, which is what a subset is.
//!
//! No `🧱️base` mutation moves any of those axes, and no mutation here touches page content: the two
//! vocabularies are disjoint by construction, which is exactly why this subset needs its own.
//!
//! The implementation lives in the shared `document::pdf_conformance` family module because all six
//! PDF 1.7 conformance subsets genuinely share the MECHANISM — every axis is a fact of the COS object
//! graph — see that module's own doc comment. What is NOT shared is which axes this subset polices,
//! which marker its OutputIntent demands and which kinds it declares: those are [`PROFILE`] and
//! [`KINDS`] below, and a kind this subset does not declare is refused here even when the engine
//! could perform it.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.
//! @see ../🧬️schema/🦀️.rs — `check_vt_conformance`, the one axis list everything here derives from.

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `PdfVtMutation` declares, in
/// declaration order. The catalog `pdf-1-7-vt` is measured against this exact list, and the
/// subject-side `KINDS` carries the test that proves enum, constant and manifest never drift apart.
pub const KINDS: &[&str] = &["insert-encryption-dictionary", "remove-encryption-dictionary", "set-output-intent", "remove-output-intent", "set-trim-box", "remove-trim-box", "embed-font-file", "remove-font-file", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-media-annotation", "remove-media-annotation", "set-dpart-root", "remove-dpart-root", "set-dpart-metadata", "remove-dpart-metadata"];
//#endregion 🔖️Vocabulary

//#region 🔖️Profile
/// 🏅️ This subset's conformance coordinates: the axes `check_vt_conformance` reads, the OutputIntent
/// marker it demands, and whether that intent must carry a `/DestOutputProfile`. The projection is
/// scoped to exactly these axes, so this subset is never judged on an axis its own checker ignores.
#[cfg(feature = "oracles")]
pub const PROFILE: crate::document::pdf_conformance::PdfConformanceProfile = crate::document::pdf_conformance::PdfConformanceProfile {
    subset: "vt",
    output_intent_subtype: "GTS_PDFX",
    output_intent_dest_profile: true,
    conformant_title: "A PDF/VT-1 conformant document",
    axes: &["encryptionDictionaries", "outputIntents", "pageBoxes", "fontPrograms", "javaScriptActions", "launchActions", "mediaAnnotations", "dpartRoot"],
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
        return Err(format!("mutation kind {kind:?} is not declared by the pdf-1-7-vt catalog"));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
