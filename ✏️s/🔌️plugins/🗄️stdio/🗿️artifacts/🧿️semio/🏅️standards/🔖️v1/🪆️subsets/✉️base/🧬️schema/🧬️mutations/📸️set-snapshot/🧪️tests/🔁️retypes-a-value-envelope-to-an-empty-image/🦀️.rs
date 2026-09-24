//! 🧪️ `📸️set-snapshot` fixture — `🔁️retypes-a-value-envelope-to-an-empty-image`: retypes a value envelope into an empty image envelope.
//!
//! The one change of subset kind the envelope permits, and only through set-snapshot. Source of truth is the committed JSON bundle under
//! `../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/`, read through this subset's
//! schema-derived JSON bridge.

use crate::standards::v1::subsets::base::schema::mutations::{apply_semio_mutation, decode_semio_mutation_json, inverse_semio_mutation, SemioMutation};
use crate::standards::v1::subsets::base::schema::snapshot::{decode_semio_snapshot_json, encode_semio_snapshot_json, SemioSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/📸️set-snapshot/🔁️retypes-a-value-envelope-to-an-empty-image/🔺️diff/🔣️.json");

fn before() -> SemioSnapshot {
    decode_semio_snapshot_json(BEFORE).expect("before envelope decodes")
}
fn after() -> SemioSnapshot {
    decode_semio_snapshot_json(AFTER).expect("after envelope decodes")
}
fn mutation() -> SemioMutation {
    decode_semio_mutation_json(MUTATION).expect("set-snapshot mutation decodes")
}

/// ▶️ Applying the committed mutation lands on the committed after-envelope and raises nothing.
#[semio_framework_async_macros::async_test]
async fn applies_to_committed_after() {
    let mut snapshot = before();
    let outcome = apply_semio_mutation(&mut snapshot, &mutation());
    assert!(outcome.messages().is_empty(), "semio-base/set-snapshot: an applied set-snapshot raises no message");
    assert_eq!(snapshot, after(), "semio-base/set-snapshot: applied state differs from the committed after-envelope");
}

/// ↩️ The computed inverse restores the committed before-envelope.
#[semio_framework_async_macros::async_test]
async fn inverse_restores_before() {
    let mut snapshot = before();
    apply_semio_mutation(&mut snapshot, &mutation());
    for step in &inverse_semio_mutation(&mutation(), &before()) {
        apply_semio_mutation(&mut snapshot, step);
    }
    assert_eq!(snapshot, before(), "semio-base/set-snapshot: inverse did not restore the committed before-envelope");
}

/// 🔣️ The committed envelopes are the bridge's own canonical encoding.
#[semio_framework_async_macros::async_test]
async fn committed_json_is_canonical() {
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {
        let reencoded: serde_json::Value = serde_json::from_str(&encode_semio_snapshot_json(&decode_semio_snapshot_json(text).expect("envelope decodes"))).expect("envelope re-encodes");
        assert_eq!(reencoded, serde_json::from_str::<serde_json::Value>(text).expect("envelope reparses"), "semio-base/set-snapshot: committed {label} JSON is not canonical");
    }
}

/// 🔺️ The diff is the committed whole `replace` of the after-envelope.
#[semio_framework_async_macros::async_test]
async fn produces_committed_diff() {
    let produced = <SemioMutation as protocol::Mutation<SemioSnapshot>>::diff(&mutation(), &before());
    let produced: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(produced.diff())).expect("produced envelope diff encodes");
    assert_eq!(produced, serde_json::from_str::<serde_json::Value>(DIFF).expect("committed envelope diff decodes"), "semio-base/set-snapshot: produced diff differs from the committed 🔺️diff/🔣️.json");
}
