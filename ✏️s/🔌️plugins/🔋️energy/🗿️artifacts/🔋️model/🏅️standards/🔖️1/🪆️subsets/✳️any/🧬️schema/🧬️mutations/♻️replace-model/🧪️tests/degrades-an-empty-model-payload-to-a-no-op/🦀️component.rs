//! 🧪️ `replace-model` fixture — `degrades-an-empty-model-payload-to-a-no-op`.
//!
//! `replace-model` is the energy artifact's ONLY mutation, and its diff oracle chains two
//! behaviours this case pins together:
//!   1. **Honest degradation.** `new_model_json` is parsed with `unwrap_or_default()`, so a payload
//!      that is not a full `crate::model::Model` becomes `Model::default()` — never a panic, and
//!      never the pre-migration behaviour of storing arbitrary opaque JSON text verbatim.
//!   2. **The no-op guard.** The decoded model is compared against `energy_model(base)`, which
//!      itself fails soft to `Model::default()` when the composed `structure` handle resolves to
//!      nothing in this process's working-scene cache — the documented fresh-process / undo-past-
//!      history state the committed `⬅️before` deliberately represents.
//! The two meet: the oracle short-circuits to a Warning `mutation.no-op` with an empty diff, and
//! `structure`/`zones` are NOT re-minted. That last point is the load-bearing one — a spurious
//! re-mint here would rewrite both content addresses in response to a payload carrying no model.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`); the derived encodings come from `fixtures generate`.

use crate::artifacts::model::diff::EnergyModelDiff;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;
use semio_framework_os_kernel::{FromValue, ToValue};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️component.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️component.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️component.json");
const DIFF: &str = include_str!("🔺️diff/🔣️component.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️component.json");

fn before() -> EnergyModelSnapshot {
    pack::from_json_str(BEFORE).expect("before energy document decodes")
}
fn expected_after() -> EnergyModelSnapshot {
    pack::from_json_str(AFTER).expect("after energy document decodes")
}
fn mutation() -> EnergyModelMutation {
    pack::from_json_str(MUTATION).expect("replace-model mutation decodes")
}
fn built_outcome() -> protocol::MutationOutcome<EnergyModelDiff> {
    <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ The degraded replace changes nothing — and above all leaves BOTH composed child handles at
/// the content addresses the committed before-document carries.
#[semio_framework_async_macros::async_test]
async fn the_degraded_replace_leaves_both_child_handles_alone() {
    let base = before();
    let applied = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("replace-model applies to its committed before-document");
    assert_eq!(applied, expected_after(), "replace-model/degrades-an-empty-model-payload-to-a-no-op: the no-op replace must reproduce the committed after-snapshot");
    assert_eq!(
        (applied.structure.child_id.as_str(), applied.zones.child_id.as_str()),
        (base.structure.child_id.as_str(), base.zones.child_id.as_str()),
        "replace-model/degrades-an-empty-model-payload-to-a-no-op: a payload carrying no model must not re-mint structure or zones"
    );
}

/// ↩️ `replace-model` is its own inverse partner: the undo re-serializes the model read out of
/// BASE. Here that is the fail-soft `Model::default()`, so the undo is a full model JSON — not the
/// `"{}"` the forward payload carried — and replaying it still lands back on the before-document.
#[semio_framework_async_macros::async_test]
async fn the_inverse_reserializes_the_base_model() {
    let base = before();
    let inverse = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::inverse(&mutation(), &base);
    assert_eq!(inverse.len(), 1, "replace-model/degrades-an-empty-model-payload-to-a-no-op: replace is its own inverse partner, so exactly one undo step");
    let EnergyModelMutation::ReplaceModel(undo) = &inverse[0];
    assert_eq!(
        undo.new_model_json,
        pack::to_json_string(&crate::model::Model::default()),
        "replace-model/degrades-an-empty-model-payload-to-a-no-op: the undo must carry BASE's own model re-serialized in full, never the forward payload"
    );
    let mut snapshot = protocol::MutationDiff::apply(built_outcome().diff(), &base).expect("forward replace-model applies");
    for step in &inverse {
        let redo = <EnergyModelMutation as protocol::Mutation<EnergyModelSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::MutationDiff::apply(redo.diff(), &snapshot).expect("the replace-model inverse step applies");
    }
    assert_eq!(snapshot, base, "replace-model/degrades-an-empty-model-payload-to-a-no-op: undoing a no-op must still land back on the before-document");
}

/// 🔣️ Both committed documents and the `replaceModel` payload are canonical — note the absent
/// `referencedModel`, which `EnergyModelSnapshot` omits entirely via `skip_serializing_if`, and the
/// payload's `newModelJson`, which stays a JSON STRING rather than an inlined object.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: EnergyModelSnapshot = pack::from_json_str(text).expect("energy document decodes");
        let reencoded = pack::json_from_dsl_value(&decoded.to_value());
        let original = pack::parse_json(text).expect("energy document reparses");
        assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-model/degrades-an-empty-model-payload-to-a-no-op: committed {label} energy JSON is not canonical ({reencoded:?} vs {original:?})");
    }
    let reencoded = pack::json_from_dsl_value(&mutation().to_value());
    let original = pack::parse_json(MUTATION).expect("replaceModel payload reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-model/degrades-an-empty-model-payload-to-a-no-op: committed replaceModel JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🎯️ `"{}"` is not a decodable `Model` — every one of `Model`'s fields is required, since the
/// struct carries no `#[serde(default)]` — so the oracle's `unwrap_or_default()` is what puts
/// `Model::default()` on both sides of its comparison, and the declared outcome is `applied` with
/// exactly one `mutation.no-op` Warning.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    assert!(pack::from_json_str::<crate::model::Model>("{}").is_err(), "replace-model/degrades-an-empty-model-payload-to-a-no-op: this fixture depends on \"{{}}\" NOT being a decodable Model");
    let declared = pack::parse_json(OUTCOME).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(pack::JsonValue::as_str), Some("applied"), "replace-model/degrades-an-empty-model-payload-to-a-no-op: a degraded no-op is applied, never rejected — the oracle has no rejection branch at all");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), Some(protocol::Severity::Warning), "replace-model/degrades-an-empty-model-payload-to-a-no-op: an unchanged model is a Warning");
    assert_eq!(produced.messages().len(), 1, "replace-model/degrades-an-empty-model-payload-to-a-no-op: exactly one diagnostic is raised");
    assert_eq!(
        produced.messages()[0].code.0.as_str(),
        declared["messages"][0]["code"].as_str().expect("declared message code is a string"),
        "replace-model/degrades-an-empty-model-payload-to-a-no-op: raised diagnostic code differs from the declared one"
    );
}

/// 🔺️ The committed diff is `EnergyModelDiff`'s all-null default: the oracle returns before it ever
/// reaches `diff_from_model`, so neither `structure` nor `zones` — the two fields a real replace
/// always writes TOGETHER — appears, and `schema`/`resultsJson` are untouchable by this verb anyway.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = pack::json_from_dsl_value(&built_outcome().diff().to_value());
    let committed = pack::parse_json(DIFF).expect("committed diff decodes");
    assert!(pack::json::value_eq_ignoring_object_order(&produced, &committed), "replace-model/degrades-an-empty-model-payload-to-a-no-op: produced diff differs from the committed 🔺️diff/🔣️component.json ({produced:?} vs {committed:?})");
}

/// 🔣️ The committed diff decodes to `EnergyModelDiff` and re-encodes unchanged — including
/// `referencedModel`, whose OUTER `Option` being null means "the link slot's presence did not
/// change at all", not "the link is now absent".
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded: EnergyModelDiff = pack::from_json_str(DIFF).expect("committed replace-model diff decodes");
    assert_eq!(decoded, EnergyModelDiff::default(), "replace-model/degrades-an-empty-model-payload-to-a-no-op: a no-op's committed diff must be the type's own default");
    assert!(decoded.referenced_model.is_none(), "replace-model/degrades-an-empty-model-payload-to-a-no-op: replacing the model must never disturb the forward model link slot");
    let reencoded = pack::json_from_dsl_value(&decoded.to_value());
    let original = pack::parse_json(DIFF).expect("committed diff reparses");
    assert!(pack::json::value_eq_ignoring_object_order(&reencoded, &original), "replace-model/degrades-an-empty-model-payload-to-a-no-op: committed diff JSON is not canonical ({reencoded:?} vs {original:?})");
}

/// 🩹 The committed diff alone carries the before-document to the after-document — trivially, but
/// it must still be the committed diff that does it.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded: EnergyModelDiff = pack::from_json_str(DIFF).expect("committed replace-model diff decodes");
    let produced = protocol::MutationDiff::apply(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "replace-model/degrades-an-empty-model-payload-to-a-no-op: committed diff did not carry before to after");
}
