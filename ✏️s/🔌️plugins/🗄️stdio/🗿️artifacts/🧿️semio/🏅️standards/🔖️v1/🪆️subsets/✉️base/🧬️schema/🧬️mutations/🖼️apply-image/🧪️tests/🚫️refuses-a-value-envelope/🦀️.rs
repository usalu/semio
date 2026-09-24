//! 🧪️ `🖼️apply-image` fixture — `🚫️refuses-a-value-envelope`: a wrapped image set-dimensions
//! mutation against a value envelope names another arm, so the envelope refuses it with
//! `mutation.target-missing` and stays exactly as it stood. Source of truth is the committed JSON
//! bundle under `../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/`,
//! whose `🔺️diff/🚫️.absent` records that no diff exists.

use crate::standards::v1::subsets::base::schema::mutations::{apply_semio_mutation, decode_semio_mutation_json, semio_mutation_refusal_codes, SemioMutation};
use crate::standards::v1::subsets::base::schema::snapshot::{decode_semio_snapshot_json, SemioSnapshot};

const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/🖼️apply-image/🚫️refuses-a-value-envelope/🎯️outcome/🔣️.json");

fn before() -> SemioSnapshot {
    decode_semio_snapshot_json(BEFORE).expect("before envelope decodes")
}
fn mutation() -> SemioMutation {
    decode_semio_mutation_json(MUTATION).expect("wrapped image mutation decodes")
}

/// 🚫️ The mismatched arm is refused with exactly the committed code and leaves the envelope untouched.
#[semio_framework_async_macros::async_test]
async fn refuses_with_the_committed_code() {
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(serde_json::Value::as_str), Some("rejected"), "semio-base/apply-image: this fixture declares a rejected outcome");
    let mut snapshot = before();
    let produced = apply_semio_mutation(&mut snapshot, &mutation());
    assert_eq!(semio_mutation_refusal_codes(&produced), vec![outcome.get("code").and_then(serde_json::Value::as_str).expect("outcome carries a code").to_string()], "semio-base/apply-image: the refusal code differs from the committed outcome");
    assert_eq!(snapshot, decode_semio_snapshot_json(AFTER).expect("after envelope decodes"), "semio-base/apply-image: a refused mutation must leave the committed envelope as it stood");
    assert_eq!(snapshot, before(), "semio-base/apply-image: the committed after-envelope must equal the before-envelope");
}
