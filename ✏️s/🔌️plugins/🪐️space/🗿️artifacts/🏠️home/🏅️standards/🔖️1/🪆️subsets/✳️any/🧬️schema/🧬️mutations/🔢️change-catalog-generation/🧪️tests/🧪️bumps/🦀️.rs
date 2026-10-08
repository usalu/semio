//! 🧪️ `change-catalog-generation` fixture — `📇️bumps-the-catalog-generation-to-7`.
//!
//! Source of truth is the committed JSON quintet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate`, not here.

use crate::standards::v1::subsets::any::schema::diff::SHomeDiff;
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔢️change-catalog-generation/🧪️bumps/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔢️change-catalog-generation/🧪️bumps/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔢️change-catalog-generation/🧪️bumps/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔢️change-catalog-generation/🧪️bumps/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🔢️change-catalog-generation/🧪️bumps/🎯️outcome/🔣️.json");

fn decode_value<T: semio_framework_value::FromValue>(text: &str) -> T {
    let json = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture JSON decodes");
    semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&json)).expect("fixture value decodes")
}
fn encode_value<T: semio_framework_value::ToValue>(value: &T) -> semio_framework_pack_json::Value {
    semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(value))
}
fn before() -> SHomeSnapshot {
    decode_value(BEFORE)
}
fn expected_after() -> SHomeSnapshot {
    decode_value(AFTER)
}
fn mutation() -> SHomeMutation {
    decode_value(MUTATION)
}
fn built_outcome() -> protocol::MutationOutcome<SHomeDiff> {
    <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation(), &before())
}

/// ▶️ Pinning the counter to `7` moves `catalogGeneration` from `3` and leaves the launcher's
/// `schema` field alone — this is a setter, not an increment.
#[semio_framework_async_macros::async_test]
async fn pins_the_counter_of_the_committed_after() {
    let applied = protocol::apply_diff(built_outcome().diff(), &before()).expect("change-catalog-generation applies to its committed before-document");
    assert_eq!(applied, expected_after(), "change-catalog-generation/bumps-the-catalog-generation-to-7: the bumped document differs from the committed after-snapshot");
    assert_eq!(applied.catalog_generation, 7, "change-catalog-generation/bumps-the-catalog-generation-to-7: the counter must land on the payload's value, not on before + 1");
}

/// ↩️ The inverse re-pins the OLD counter read out of BASE — `3`, never a structural inversion of
/// the diff.
#[semio_framework_async_macros::async_test]
async fn repinning_the_old_counter_restores_before() {
    let base = before();
    let forward = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(&mutation(), &base);
    let mut snapshot = protocol::apply_diff(forward.diff(), &base).expect("forward change-catalog-generation applies");
    let inverse = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::inverse(&mutation(), &base).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(), 1, "change-catalog-generation/bumps-the-catalog-generation-to-7: the inverse of one counter pin is exactly one counter pin back");
    for step in &inverse {
        let undo = <SHomeMutation as protocol::Mutation<SHomeSnapshot>>::diff(step, &snapshot);
        snapshot = protocol::apply_diff(undo.diff(), &snapshot).expect("the change-catalog-generation inverse step applies");
    }
    assert_eq!(snapshot, base, "change-catalog-generation/bumps-the-catalog-generation-to-7: re-pinning generation 3 did not restore the before-document");
}

/// 🔣️ Both committed launcher snapshots and the `changeCatalogGeneration` payload are canonical.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded = decode_value::<SHomeSnapshot>(text);
        let reencoded = encode_value(&decoded);
        let original = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("launcher snapshot reparses");
        assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed {label} launcher JSON is not canonical");
    }
    let reencoded = encode_value(&mutation());
    let original = semio_framework_pack_json::parse(MUTATION, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("changeCatalogGeneration payload reparses");
    assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed changeCatalogGeneration JSON is not canonical");
}

/// 🎯️ The only guard this mutation has is the equal-counter `mutation.no-op` warning; `3 != 7`, so
/// the declared `applied` outcome must be message-free.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds() {
    let declared = semio_framework_pack_json::parse(OUTCOME, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("outcome decodes");
    assert_eq!(declared.get("status").and_then(semio_framework_pack_json::Value::as_str), Some("applied"), "change-catalog-generation/bumps-the-catalog-generation-to-7: this fixture declares an applied outcome");
    let produced = built_outcome();
    assert_eq!(produced.worst_level(), None, "change-catalog-generation/bumps-the-catalog-generation-to-7: pinning a different value must not raise mutation.no-op");
    assert!(produced.messages().is_empty(), "change-catalog-generation/bumps-the-catalog-generation-to-7: an accepted counter pin emits no diagnostics");
}

/// 🔺️ `SHomeDiff` carries four optional fields; this mutation is allowed to set exactly one of
/// them — `catalogGeneration` — and must leave `schema` null.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = encode_value(built_outcome().diff());
    let committed = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff decodes");
    assert_eq!(produced, committed, "change-catalog-generation/bumps-the-catalog-generation-to-7: produced diff differs from the committed 🔺️diff/🔣️.json");
}

/// 🔣️ The committed diff decodes to `SHomeDiff` and re-encodes unchanged — including the schema
/// null, which `SHomeDiff` emits because no field carries `skip_serializing_if`.
#[semio_framework_async_macros::async_test]
async fn committed_diff_is_canonical() {
    let decoded = decode_value::<SHomeDiff>(DIFF);
    assert_eq!(decoded.catalog_generation, Some(7), "change-catalog-generation/bumps-the-catalog-generation-to-7: the committed diff must set the counter");
    assert!(decoded.schema.is_none(), "a catalog-generation delta preserves the schema");
    let reencoded = encode_value(&decoded);
    let original = semio_framework_pack_json::parse(DIFF, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("committed diff reparses");
    assert_eq!(reencoded, original, "change-catalog-generation/bumps-the-catalog-generation-to-7: committed diff JSON is not canonical");
}

/// 🩹 The committed diff alone carries the before-document to the after-document.
#[semio_framework_async_macros::async_test]
async fn committed_diff_applies_to_after() {
    let decoded = decode_value::<SHomeDiff>(DIFF);
    let produced = protocol::apply_diff(&decoded, &before()).expect("committed diff applies to the before-document");
    assert_eq!(produced, expected_after(), "change-catalog-generation/bumps-the-catalog-generation-to-7: committed diff did not carry before to after");
}

/// ⚖️ The inverse diffs sum to the negative of the forward diff: `Σ.apply(after) == before` and `canon(Σ) == canon(d.inverse(before))`.
#[semio_framework_async_macros::async_test]
async fn inverse_diffs_sum_to_the_negative_diff() {
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await;
}
