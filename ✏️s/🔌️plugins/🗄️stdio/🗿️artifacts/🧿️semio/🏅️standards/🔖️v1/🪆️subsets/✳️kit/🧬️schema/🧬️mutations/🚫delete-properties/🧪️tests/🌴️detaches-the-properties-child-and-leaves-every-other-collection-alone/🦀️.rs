//! 🧪️ `delete-properties` fixture — `🌴️detaches-the-properties-child-and-leaves-every-other-collection-alone`.
//!
//! Transcribed from `../../🔺️diff/🦀️.rs`: a kit with no properties child is Error
//! `mutation.target-missing`; otherwise the diff sets `properties = Some(None)` — outer `Some` =
//! "this diff writes the slot", inner `None` = "clear it".
//!
//! ⚠️ KNOWN SERDE LIMITATION, pinned here rather than papered over: `SemioKitDiff::properties` is
//! `Option<Option<ArtifactChild<..>>>` with `skip_serializing_if = "Option::is_none"`, so
//! `Some(None)` encodes as `{"properties": null}` — but JSON `null` deserializes back into the
//! OUTER `None`, i.e. "untouched". The committed `🔺️diff/🔣️.json` is therefore the correct
//! ENCODING of what this mutation produces, while a decode of it is NOT the same value. The two
//! assertions that would otherwise gloss over this state the collapse explicitly and exercise the
//! apply law against the in-memory `Some(None)` diff instead.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;
use protocol::{Mutation, MutationDiff};

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> SemioKitSnapshot {
    serde_json::from_str(BEFORE).expect("delete-properties before snapshot decodes")
}
fn expected_after() -> SemioKitSnapshot {
    serde_json::from_str(AFTER).expect("delete-properties after snapshot decodes")
}
fn mutation() -> SemioKitMutation {
    serde_json::from_str(MUTATION).expect("delete-properties mutation decodes")
}

/// ▶️ The properties handle is cleared; the type catalogue, designs and both child collections all
/// survive — clearing the single optional slot must not double as a kit reset.
#[semio_framework_async_macros::async_test]
async fn clears_the_properties_slot_and_leaves_every_other_slot_alone() {
    let base = before();
    assert!(base.properties.is_some(), "the fixture needs a populated properties slot");
    let produced = mutation().diff(&base).diff().apply(&base).expect("delete-properties applies to its committed before-snapshot");
    assert_eq!(produced, expected_after(), "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: applied state differs from the committed after-snapshot");
    assert!(produced.properties.is_none(), "the properties slot must be empty afterwards");
    assert_eq!((produced.types, produced.designs), (base.types, base.designs), "types and designs must survive untouched");
    assert_eq!((produced.objects, produced.models, produced.representations), (base.objects, base.models, base.representations), "both child collections and the link collection must survive untouched");
}

/// ↩️ The undo re-creates the captured handle — the same `childId` and target, not a fresh one.
#[semio_framework_async_macros::async_test]
async fn the_undo_create_properties_reattaches_the_captured_handle() {
    let base = before();
    let mutation = mutation();
    let undo = mutation.inverse(&base);
    assert_eq!(undo.len(), 1, "deleting an existing properties child undoes as exactly one create-properties");
    let SemioKitMutation::CreateProperties(recreate) = &undo[0] else { panic!("delete-properties must undo as create-properties") };
    assert_eq!(recreate.child_id, "props-1", "the undo must recapture the ORIGINAL child id from base");
    let mut current = mutation.diff(&base).diff().apply(&base).expect("forward delete-properties applies");
    for step in &undo {
        current = step.diff(&current).diff().apply(&current).expect("the undo create-properties applies");
    }
    assert_eq!(current, base, "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: the undo did not restore the before-snapshot");
}

/// 🔣️ Snapshots and the empty-payload `{"DeleteProperties":{}}` mutation are canonical — the
/// after-snapshot OMITS the `properties` key entirely, which is a different encoding from the
/// diff's explicit `null`.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let decoded: SemioKitSnapshot = serde_json::from_str(text).expect("snapshot decodes");
        let reencoded = serde_json::to_value(&decoded).expect("snapshot encodes");
        let original: serde_json::Value = serde_json::from_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: committed {label} JSON is not canonical");
    }
    let after_json: serde_json::Value = serde_json::from_str(AFTER).expect("after reparses");
    assert!(after_json.get("properties").is_none(), "a cleared snapshot slot is an ABSENT key, never an explicit null");
    let reencoded = serde_json::to_value(mutation()).expect("delete-properties mutation encodes");
    let original: serde_json::Value = serde_json::from_str(MUTATION).expect("delete-properties mutation reparses");
    assert_eq!(reencoded, original, "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: committed mutation JSON is not canonical");
}

/// 🎯️ Declared `applied`: the kit really has a properties child, so `mutation.target-missing` must
/// not fire.
#[semio_framework_async_macros::async_test]
async fn declared_outcome_holds_without_a_target_missing_rejection() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("applied"), "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: this case is declared applied");
    let produced = mutation().diff(&before());
    assert!(produced.messages().is_empty(), "clearing a populated slot must raise no diagnostics");
}

/// 🔺️ The produced delta ENCODES to exactly `{"properties": null}` and nothing else — the
/// assertion that pins "clear this one slot" as distinct from both "leave it alone" and "rebuild
/// the kit".
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let base = before();
    let outcome = <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(&mutation(), &base);
    assert!(matches!(outcome.diff().properties, Some(None)), "the in-memory diff must be Some(None) — write the slot, clear it");
    let produced = serde_json::to_value(outcome.diff()).expect("produced diff encodes");
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: produced diff differs from the committed 🔺️diff/🔣️.json");
    assert_eq!(committed.as_object().map(|map| map.len()), Some(1), "exactly one key — no other kit slot may appear");
    assert!(committed.get("properties").expect("the committed diff names the properties slot").is_null(), "a cleared DIFF slot is an explicit null, never an absent key");
}

/// 🔣️ The committed diff is NOT a decode→encode fixed point, and this test says so on purpose:
/// `Option<Option<T>>` collapses `{"properties":null}` back to the outer `None`. Asserting the
/// usual canonicality law here would assert something false, so what is pinned instead is the
/// exact shape of the collapse.
#[semio_framework_async_macros::async_test]
async fn committed_diff_json_pins_the_option_option_collapse() {
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed delete-properties diff decodes");
    assert!(decoded.properties.is_none(), "decoding {{\"properties\":null}} yields the OUTER None — the clear intent is lost on the JSON round trip");
    assert_eq!(serde_json::to_value(&decoded).expect("re-encode"), serde_json::json!({}), "so re-encoding the decoded value drops the key entirely");
    let authored = SemioKitDiff { properties: Some(None), ..Default::default() };
    let committed: serde_json::Value = serde_json::from_str(DIFF).expect("committed diff reparses");
    assert_eq!(serde_json::to_value(&authored).expect("authored diff encodes"), committed, "the committed JSON IS the canonical encoding of the Some(None) diff, even though it cannot be decoded back into one");
}

/// 🩹 The diff carries `before` to `after` — exercised against the in-memory `Some(None)` diff,
/// because the JSON-decoded one collapses to a no-op (see the test above). Both halves are
/// asserted so the collapse cannot silently start passing for the wrong reason.
#[semio_framework_async_macros::async_test]
async fn authored_diff_applies_to_after_while_the_decoded_one_is_inert() {
    let authored = SemioKitDiff { properties: Some(None), ..Default::default() };
    let produced = authored.apply(&before()).expect("the Some(None) diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "delete-properties/detaches-the-properties-child-and-leaves-every-other-collection-alone: the Some(None) diff did not carry before to after");
    let decoded: SemioKitDiff = serde_json::from_str(DIFF).expect("committed delete-properties diff decodes");
    let inert = decoded.apply(&before()).expect("the collapsed diff still applies, it just does nothing");
    assert_eq!(inert, before(), "the JSON-decoded diff is inert — that is exactly the limitation being pinned");
}
