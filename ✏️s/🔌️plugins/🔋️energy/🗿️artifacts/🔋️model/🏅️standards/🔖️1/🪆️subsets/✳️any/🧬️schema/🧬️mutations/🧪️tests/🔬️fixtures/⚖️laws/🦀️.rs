//! ⚖️ Laws of the energy mutation fixtures: the committed document readers, the fixture writer and the eight laws every vector is held to, with the codec round-trip test. See [`super`].

use super::case::{fixture_root, Case};
use crate::diff::EnergyModelDiff;
use crate::mutations::EnergyModelMutation;
use crate::snapshot::EnergyModelSnapshot;
use protocol::{ColdMutation, Mutation, SemanticMutation};
use semio_framework_value::ToValue;

fn committed_document(case: &Case, label: &str, text: &str) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}/{}: committed {label} is not valid JSON: {error}", case.kind, case.directory))
}

fn decode(case: &Case, label: &str, text: &str) -> EnergyModelSnapshot {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{}/{}: committed {label} does not decode: {error}", case.kind, case.directory))
}

fn built(case: &Case) -> protocol::MutationOutcome<EnergyModelDiff> {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::diff(&mutation, &base)
}

fn level_name(level: semio_framework_diagnostic::Severity) -> &'static str {
    match level {
        semio_framework_diagnostic::Severity::Info => "info",
        semio_framework_diagnostic::Severity::Warning => "warning",
        semio_framework_diagnostic::Severity::Error => "error",
        semio_framework_diagnostic::Severity::Fatal => "fatal",
    }
}

/// 🎯️ The declared outcome document a produced `MutationOutcome` corresponds to, in the outcome-class vocabulary
/// `applied | no-op | rejected`: a refusal names one fault code and the offending address, an application or a no-op
/// (the `mutation.no-op` warning) names its ordered message list.
fn outcome_document(outcome: &protocol::MutationOutcome<EnergyModelDiff>) -> semio_framework_pack_json::Value {
    let rejected = outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error);
    if rejected {
        let first = outcome.messages().iter().find(|message| message.level >= semio_framework_diagnostic::Severity::Error).expect("a refusal carries its own message");
        return semio_framework_pack_json::object([
            ("status".to_string(), semio_framework_pack_json::Value::String("rejected".to_string())),
            ("code".to_string(), semio_framework_pack_json::Value::String(first.code.0.clone())),
            ("path".to_string(), semio_framework_pack_json::Value::Array(first.target.iter().map(|entry| semio_framework_pack_json::Value::String(entry.clone())).collect())),
        ]);
    }
    let no_op = outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op");
    semio_framework_pack_json::object([
        ("status".to_string(), semio_framework_pack_json::Value::String(if no_op { "no-op" } else { "applied" }.to_string())),
        (
            "messages".to_string(),
            semio_framework_pack_json::Value::Array(
                outcome.messages().iter().map(|message| semio_framework_pack_json::object([("level".to_string(), semio_framework_pack_json::Value::String(level_name(message.level).to_string())), ("code".to_string(), semio_framework_pack_json::Value::String(message.code.0.clone()))])).collect(),
            ),
        ),
    ])
}

fn write_file(path: std::path::PathBuf, text: &str) {
    std::fs::create_dir_all(path.parent().expect("fixture file has a parent")).expect("fixture directory is writable");
    std::fs::write(path, format!("{text}\n")).expect("fixture file is writable");
}

/// 🏗️ Materializes the five committed JSON files from the typed scenario. Off unless
/// `SEMIO_ENERGY_WRITE_FIXTURES=1`, so an ordinary test run never touches the source tree.
/// The root is the SAME `🧫️fixtures/🧬️mutations/<kind>/<case>` tree every leaf test
/// `include_str!`s from (`../../../../../🧫️fixtures/…` relative to the leaf) — the writer once
/// pointed at `🧬️schema/🧬️mutations/…`, which left every committed quintet a `{}` stub.
pub fn write_when_requested(case: &Case) {
    if std::env::var("SEMIO_ENERGY_WRITE_FIXTURES").ok().as_deref() != Some("1") {
        return;
    }
    let (before, mutation) = (case.scenario)();
    let outcome = <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::diff(&mutation, &before);
    let after = protocol::apply_diff_cold(outcome.diff(), &before).expect("the scenario's forward diff applies");
    let root = fixture_root().join(case.directory);
    let json = |value: semio_framework_pack_json::Value| semio_framework_pack_json::to_string_pretty(&value);
    write_file(root.join("📸️snapshot/⬅️before/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&before.to_value())));
    write_file(root.join("📸️snapshot/➡️after/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&after.to_value())));
    write_file(root.join("🦠️mutation/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&mutation.to_value())));
    write_file(root.join("🔺️diff/🔣️.json"), &json(semio_framework_pack_json::from_dsl_value(&outcome.diff().to_value())));
    let mut document = outcome_document(&outcome);
    if let (Some(invariant), Some(object)) = (committed_outcome(case).1, document.as_object_mut()) {
        object.insert("invariant", semio_framework_pack_json::Value::String(invariant));
    }
    write_file(root.join("🎯️outcome/🔣️.json"), &json(document));
}

/// 📖️ The committed before-snapshot and mutation of one vector, decoded — the inputs a leaf hands the framework's inverse-sum law.
pub fn committed(case: &Case) -> (EnergyModelSnapshot, EnergyModelMutation) {
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    (decode(case, "before-snapshot", case.before.as_str()), mutation)
}

/// ▶️ Applying the committed mutation to the committed before-snapshot reproduces the committed
/// after-snapshot exactly, composed child handles included.
pub fn assert_forward(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let applied = protocol::apply_diff_cold(built(case).diff(), &base).expect("committed mutation applies to its committed before-snapshot");
    assert_eq!(applied, decode(case, "after-snapshot", case.after.as_str()), "{}/{}: the applied document is not the committed after-snapshot", case.kind, case.directory);
    assert_eq!((applied.structure.child_id.as_str(), applied.zones.child_id.as_str()), (base.structure.child_id.as_str(), base.zones.child_id.as_str()), "{}/{}: structure and zones must stay one scene", case.kind, case.directory);
}

/// ↩️ Applying the committed mutation and then its OWN computed inverse restores the committed
/// before-snapshot; an inverse step that is itself refused fails here rather than passing quietly.
/// The steps are replayed in REVERSE order — exactly as `ArtifactStore::replay_mutations`
/// (`back.reverse()`) and the framework's `assert_mutation_inverse_law` do — so a multi-step inverse
/// that only works forwards (parent re-created first) is caught here, not in the shell's undo.
pub fn assert_inverse(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let mut snapshot = protocol::apply_diff_cold(built(case).diff(), &base).expect("committed mutation applies");
    for step in <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").into_iter().rev() {
        let outcome = <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::diff(&step, &snapshot);
        assert!(!outcome.worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error), "{}/{}: an inverse step was itself refused", case.kind, case.directory);
        snapshot = protocol::apply_diff_cold(outcome.diff(), &snapshot).expect("the inverse step applies");
    }
    assert_eq!(snapshot, base, "{}/{}: undoing did not land back on the committed before-snapshot", case.kind, case.directory);
}

/// 🔣️ Both committed documents and the committed payload re-encode to themselves.
pub fn assert_canonical(case: &Case) {
    for (label, text) in [("before", case.before.as_str()), ("after", case.after.as_str())] {
        let decoded = decode(case, label, text);
        let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
        assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed_document(case, label, text)), "{}/{}: committed {label} snapshot is not canonical", case.kind, case.directory);
    }
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&mutation.to_value());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed_document(case, "mutation", case.mutation.as_str())), "{}/{}: committed mutation payload is not canonical", case.kind, case.directory);
}

/// 🧷️ The payload-intrinsic invariant ids the committed mutation's own leaf payload schema declares in `x-semio-invariant`.
fn declared_invariants(case: &Case) -> Vec<String> {
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let schema = <EnergyModelMutation as Mutation<EnergyModelSnapshot>>::input_schema(&mutation).expect("every energy leaf publishes its payload schema");
    let document = semio_framework_pack_json::parse(schema, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("leaf payload schema is valid JSON");
    document.get("x-semio-invariant").and_then(semio_framework_pack_json::Value::as_array).map(|rows| rows.iter().filter_map(|row| row.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string)).collect()).unwrap_or_default()
}

/// 🧷️ The committed outcome split into the part the implementation produces and the `invariant` id a payload-intrinsic
/// refusal names — a rule the produced message cannot carry, which the leaf schema declares in `x-semio-invariant`.
fn committed_outcome(case: &Case) -> (semio_framework_pack_json::Value, Option<String>) {
    let mut declared = committed_document(case, "outcome", case.outcome.as_str());
    let invariant = declared.as_object_mut().and_then(|object| object.remove("invariant")).map(|value| value.as_str().expect("an outcome's invariant is an id").to_string());
    (declared, invariant)
}

/// 🎯️ The diagnostics the implementation raises are the ones the committed outcome declares; a named invariant is a
/// `mutation.invariant` refusal whose id the leaf's payload schema declares.
pub fn assert_outcome(case: &Case) {
    let produced = outcome_document(&built(case));
    let (declared, invariant) = committed_outcome(case);
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced outcome {produced:?} differs from the committed one {declared:?}", case.kind, case.directory);
    if let Some(invariant) = invariant {
        assert_eq!(produced["code"].as_str(), Some("mutation.invariant"), "{}/{}: only a mutation.invariant refusal names an invariant", case.kind, case.directory);
        assert!(declared_invariants(case).contains(&invariant), "{}/{}: the leaf payload schema declares no x-semio-invariant {invariant:?}", case.kind, case.directory);
    }
}

/// 🔺️ The produced delta is the committed delta — which pins WHICH fields the kind may touch,
/// not merely where the document ended up.
pub fn assert_diff(case: &Case) {
    let produced = semio_framework_pack_json::from_dsl_value(&built(case).diff().to_value());
    let declared = committed_document(case, "diff", case.diff.as_str());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&produced, &declared), "{}/{}: produced diff {produced:?} differs from the committed 🔺️diff/🔣️.json {declared:?}", case.kind, case.directory);
}

/// 🔣️ The committed delta decodes to the real `EnergyModelDiff` and re-encodes unchanged.
pub fn assert_diff_canonical(case: &Case) {
    let decoded: EnergyModelDiff = semio_framework_pack_json::from_json_str(case.diff.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let reencoded = semio_framework_pack_json::from_dsl_value(&decoded.to_value());
    assert!(semio_framework_pack_json::value_eq_ignoring_object_order(&reencoded, &committed_document(case, "diff", case.diff.as_str())), "{}/{}: committed diff is not canonical", case.kind, case.directory);
}

/// 🩹 The committed delta ALONE carries the before-document to the after-document.
pub fn assert_diff_applies(case: &Case) {
    let decoded: EnergyModelDiff = semio_framework_pack_json::from_json_str(case.diff.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    let produced = protocol::apply_diff_cold(&decoded, &decode(case, "before-snapshot", case.before.as_str())).expect("committed diff applies to the before-document");
    assert_eq!(produced, decode(case, "after-snapshot", case.after.as_str()), "{}/{}: the committed diff did not carry before to after", case.kind, case.directory);
}

/// 🧭️ The kind's semantic descriptor uses an approved verb and its inverse closes the loop.
pub async fn assert_semantics(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let descriptor = SemanticMutation::semantics(&mutation);
    assert!(protocol::is_approved_verb(descriptor.verb), "{}: {:?} is not an approved verb", case.kind, descriptor.verb);
    assert_eq!(descriptor.kind, case.kind);
    assert_eq!(<EnergyModelMutation as SemanticMutation<EnergyModelSnapshot>>::kinds().len(), super::KINDS.len());
    assert_inverse(case);
    let _ = base;
}

/// 🎯️ Whether the committed outcome declares this vector a refusal (`{"status":"rejected",…}`).
fn committed_is_rejected(case: &Case) -> bool {
    matches!(committed_document(case, "outcome", case.outcome.as_str()).get("status"), Some(semio_framework_pack_json::Value::String(status)) if status == "rejected")
}

/// ⚖️ The framework's own laws, run in role against this vector: the sequential inverse law, the L3 inverse-sum law (the concrete
/// inverse steps' diffs, absorbed, equal the negative of the forward diff) and the diff-absorb law over the forward diff followed
/// by the diff of the first inverse step applied. A vector the committed outcome declares REJECTED is exempt from the inverse
/// laws by construction — the framework laws panic on any Error/Fatal forward outcome, and a refusal IS that outcome; what a
/// refusal owes instead (the exact fault code + path, an empty delta, before == after) is pinned by
/// `assert_outcome`/`assert_diff`/`assert_forward`. The absorb law still holds for it.
pub async fn assert_laws(case: &Case) {
    let base = decode(case, "before-snapshot", case.before.as_str());
    let mutation: EnergyModelMutation = semio_framework_pack_json::from_json_str(case.mutation.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed mutation payload decodes");
    let first = built(case).diff().clone();
    if committed_is_rejected(case) {
        assert!(built(case).worst_level().is_some_and(|level| level >= semio_framework_diagnostic::Severity::Error), "{}/{}: the committed outcome is a refusal but the implementation applied the mutation", case.kind, case.directory);
        assert_eq!(decode(case, "after-snapshot", case.after.as_str()), base, "{}/{}: a refused vector must leave the document untouched", case.kind, case.directory);
        protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first.clone(), first).await;
        return;
    }
    protocol::os_spr::protocol_laws::assert_mutation_inverse_law(&base, &mutation).await;
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    let after = protocol::apply_diff_cold(&first, &base).expect("the committed forward diff applies");
    let undo = <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::inverse(&mutation, &base).expect("valid retained mutation inverse fixture").pop();
    let second = undo.map_or_else(EnergyModelDiff::default, |step| <EnergyModelMutation as ColdMutation<EnergyModelSnapshot>>::diff(&step, &after).diff().clone());
    protocol::os_spr::protocol_laws::assert_mutation_diff_absorb_law(&base, first, second).await;
}

/// 📝️ Every declared kind survives the text and binary op codecs the `dsl::DslEnum` derive
/// generates — one loop, no per-kind round-trip test to forget.
#[semio_framework_async_macros::async_test]
async fn every_kind_round_trips_through_text_and_binary() {
    for operation in super::wire_probes() {
        store::os_store::test_support::assert_op_line_round_trip(&operation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
