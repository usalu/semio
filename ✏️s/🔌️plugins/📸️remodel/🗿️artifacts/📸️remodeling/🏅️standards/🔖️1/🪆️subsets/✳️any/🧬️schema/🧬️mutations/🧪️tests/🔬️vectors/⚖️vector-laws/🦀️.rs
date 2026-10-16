//! ⚖️ The law set a committed vector is held to: applied, no-op and rejected outcomes, the inverse and the canonical form.

use super::committed_vector::{json_of, refusal_level, Vector};
use semio_framework_os_kernel as protocol;
use semio_framework_pack_json::{from_json_str, parse, JsonMemberPolicy, Value};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_mutations::standards::v1::subsets::any::schema::mutations::{apply_remodeling_mutation, inverse_remodeling_mutation};

//#region ⚖️Laws
/// ▶️ An applied vector reaches its committed after-document, produces exactly the committed diff (which is canonical and
/// alone carries before to after) and emits exactly the declared diagnostics.
fn applied_laws(vector: &Vector) {
    let label = &vector.label;
    let applied = apply_remodeling_mutation(&vector.before(), &vector.mutation()).unwrap_or_else(|error| panic!("{label}: applies to its committed before-snapshot: {error:?}"));
    assert_eq!(applied, vector.expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_ne!(applied, vector.before(), "{label}: an applied vector must move the document");
    let outcome = vector.produced();
    let committed = vector.committed_diff();
    assert_eq!(json_of(outcome.diff()), committed, "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
    let decoded: RemodelingDiff = from_json_str(vector.diff.as_deref().expect("committed diff"), JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{label}: committed diff decodes into the diff type: {error:?}"));
    assert_eq!(json_of(&decoded), committed, "{label}: committed diff JSON is not canonical");
    let carried = protocol::apply_diff(&decoded, &vector.before()).unwrap_or_else(|error| panic!("{label}: committed diff applies to the before-snapshot: {error:?}"));
    assert_eq!(carried, vector.expected_after(), "{label}: committed diff did not carry before to after");
    let declared_codes: Vec<String> = match vector.declared().get("messages") {
        Some(Value::Array(entries)) => entries.iter().filter_map(|entry| entry.get("code").and_then(|code| code.as_str()).map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    let emitted: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    assert_eq!(emitted, declared_codes, "{label}: emitted diagnostics differ from the declared ones");
}

/// 🔁️ A warned no-op leaves the document untouched, carries an empty diff that is still committed as a real all-null
/// delta, and emits exactly one `mutation.no-op` Warning.
fn no_op_laws(vector: &Vector) {
    let label = &vector.label;
    let base = vector.before();
    let applied = apply_remodeling_mutation(&base, &vector.mutation()).unwrap_or_else(|error| panic!("{label}: an empty diff still applies cleanly: {error:?}"));
    assert_eq!(applied, vector.expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "{label}: a no-op must not move the document");
    let produced = vector.produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "{label}: a no-op leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label}: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "{label}: an identical resubmission is a no-op");
    assert_eq!(messages[0].level, semio_framework_diagnostic::Severity::Warning, "{label}: a no-op is a Warning, never an Error");
    assert_eq!(json_of(produced.diff()), vector.committed_diff(), "{label}: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🚫️ A refusal leaves the document untouched, carries an empty diff, emits exactly the declared code at its level and
/// path, and ships the empty `🚫️.absent` marker instead of a diff.
fn rejected_laws(vector: &Vector) {
    let label = &vector.label;
    let base = vector.before();
    let applied = apply_remodeling_mutation(&base, &vector.mutation()).unwrap_or_else(|error| panic!("{label}: an empty diff still applies cleanly: {error:?}"));
    assert_eq!(applied, vector.expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(applied, base, "{label}: a refused mutation must not move the document");
    let declared = vector.declared();
    let produced = vector.produced();
    assert_eq!(produced.diff(), &RemodelingDiff::default(), "{label}: a refusing leaf must carry an empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label}: exactly one diagnostic is expected, got {messages:?}");
    assert_eq!(declared.get("code").and_then(|code| code.as_str()), Some(messages[0].code.0.as_str()), "{label}: the committed outcome must name the emitted code");
    assert_eq!(messages[0].level, refusal_level(&messages[0].code.0), "{label}: the declared level must be the emitted one");
    let declared_path: Vec<String> = match declared.get("path") {
        Some(Value::Array(entries)) => entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect(),
        _ => Vec::new(),
    };
    assert_eq!(declared_path, messages[0].target, "{label}: the declared path must be the emitted target");
    assert!(vector.diff.is_none(), "{label}: a refusal ships no 🔺️diff/🔣️.json");
    assert_eq!(vector.absent_marker.as_deref(), Some(""), "{label}: the absent-diff marker must exist and stay empty");
}

/// ↩️ Applying the verb and then every step of its own computed inverse restores the committed before-document,
/// member positions included.
fn inverse_law(vector: &Vector) {
    let label = &vector.label;
    let base = vector.before();
    let mutation = vector.mutation();
    let mut snapshot = apply_remodeling_mutation(&base, &mutation).unwrap_or_else(|error| panic!("{label}: forward applies: {error:?}"));
    for step in &inverse_remodeling_mutation(&base, &mutation).unwrap_or_else(|error| panic!("{label}: valid retained mutation inverse fixture: {error:?}")) {
        snapshot = apply_remodeling_mutation(&snapshot, step).unwrap_or_else(|error| panic!("{label}: inverse step applies: {error:?}"));
    }
    assert_eq!(snapshot, base, "{label}: inverse did not restore the before-snapshot");
}

/// 🔣️ The committed snapshots and mutation are canonical: decode→encode is a fixed point over `pack::json`.
fn canonical_law(vector: &Vector) {
    let label = &vector.label;
    for (side, text) in [("before", &vector.before), ("after", &vector.after)] {
        let decoded: RemodelingSnapshot = from_json_str(text, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{label}: {side} snapshot decodes: {error:?}"));
        let original = parse(text, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{label}: {side} snapshot reparses: {error:?}"));
        assert_eq!(json_of(&decoded), original, "{label}: committed {side} JSON is not canonical");
    }
    let original = parse(&vector.mutation, JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{label}: mutation reparses: {error:?}"));
    assert_eq!(json_of(&vector.mutation()), original, "{label}: committed mutation JSON is not canonical");
}

/// ⚖️ The vector `<leaf>/<case>` holds the law set its declared status selects, plus the inverse and canonical laws.
pub fn assert_case(leaf: &str, case: &str) {
    let vector = Vector::load(leaf, case);
    match vector.status().as_str() {
        "applied" => applied_laws(&vector),
        "no-op" => no_op_laws(&vector),
        "rejected" => rejected_laws(&vector),
        other => panic!("{}: undeclared status {other:?}", vector.label),
    }
    inverse_law(&vector);
    canonical_law(&vector);
}

/// ➕️ The applied vector `<leaf>/<case>`'s inverse rows sum to the negative of its diff.
pub async fn assert_inverse_sum(leaf: &str, case: &str) {
    let vector = Vector::load(leaf, case);
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&vector.mutation(), &vector.before()).await;
}
//#endregion ⚖️Laws
