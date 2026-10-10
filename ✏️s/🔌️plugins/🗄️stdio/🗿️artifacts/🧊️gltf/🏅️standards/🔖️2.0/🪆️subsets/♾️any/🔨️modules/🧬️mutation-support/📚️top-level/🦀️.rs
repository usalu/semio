//! 🔒 Pure mechanics private to executable document-level glTF leaves.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfTopLevelMutationRejection {
    pub code: String,
    pub path: String,
    pub detail: String,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn reject(code: impl Into<String>, path: impl Into<String>, detail: impl Into<String>) -> GltfTopLevelMutationRejection {
    GltfTopLevelMutationRejection { code: code.into(), path: path.into(), detail: detail.into() }
}

/// ⚖️ The frozen mutation outcome code a glTF rejection reports as: an address the document lacks is `target-missing`, an
/// identity it already holds `duplicate-id`, an edit the document's current shape contradicts `target-mismatch`, a payload
/// malformed on its own `invariant` (`📡️replication/🎮️mutation/🧫️fixtures/🧫️outcome-code`).
pub(crate) fn rejection_outcome_code(code: &str) -> protocol::OutcomeCode {
    match code.strip_prefix("gltf.mutation.").unwrap_or(code) {
        "no-observable-change" => protocol::OutcomeCode::NoOp,
        "gltf.reference.in-use" => protocol::OutcomeCode::TargetReferenced,
        "gltf.reference.invalid-default-scene" => protocol::OutcomeCode::TargetMismatch,
        "duplicate-id" | "duplicate-extension" | "duplicate-scene-root" => protocol::OutcomeCode::DuplicateId,
        "index-out-of-range" | "insert-out-of-range" | "position-out-of-range" | "reference-out-of-range" | "relation-absent" | "extension-absent" | "missing" | "missing-mesh" | "not-found" | "invalid-reference" => protocol::OutcomeCode::TargetMissing,
        "stale-diff" | "stale-inverse" | "node-cycle" | "invalid-permutation" | "invalid-child-link" | "invalid-index-accessor" | "morph-target-arity" | "morph-weight-arity" | "collection-overflow" | "reference-overflow" | "buffer-alignment" | "extension-required" | "required-extension-not-used" => protocol::OutcomeCode::TargetMismatch,
        _ => protocol::OutcomeCode::Invariant,
    }
}

/// 📨️ Converts a glTF mutation rejection into its protocol outcome: the vocabulary code at its level and the rejection path
/// as the target; a mismatch or invariant keeps the glTF code in front of the detail, since its generic code alone does not
/// say which law refused.
pub(crate) fn rejection_outcome(code: &str, path: &str, detail: String) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
    let target = path.split('/').filter(|part| !part.is_empty()).map(str::to_string).collect::<Vec<_>>();
    match rejection_outcome_code(code) {
        protocol::OutcomeCode::NoOp => protocol::MutationOutcome::new(Default::default()).warning("mutation.no-op", detail),
        outcome_code @ (protocol::OutcomeCode::TargetMismatch | protocol::OutcomeCode::Invariant) => protocol::MutationOutcome::refuse(outcome_code, format!("{code}: {detail}"), target),
        outcome_code => protocol::MutationOutcome::refuse(outcome_code, detail, target),
    }
}
