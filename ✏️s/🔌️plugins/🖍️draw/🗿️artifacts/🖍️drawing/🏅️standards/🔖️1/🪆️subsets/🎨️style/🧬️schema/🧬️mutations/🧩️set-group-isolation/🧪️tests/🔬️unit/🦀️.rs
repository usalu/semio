//! 🧩️ Canonical group-isolation changes preserve geometry, history, and operation codecs.

use protocol::{ColdMutation, Mutation};
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_diff::DrawingDiff;
static BEFORE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/⬅️before/🔣️.json"));
static AFTER: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/📸️snapshot/➡️after/🔣️.json"));
static DIFF: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/🔺️diff/🔣️.json"));
static MUTATION: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧫️fixtures/🧬️mutations/🧩️set-group-isolation/🧩️pass/🦠️mutation/🔣️.json"));
#[test]
fn group_isolation_fixture_applies_and_inverts_through_all_owned_codecs() {
    let before:DrawingSnapshot=serde_json::from_str(*BEFORE).unwrap();
    let after:DrawingSnapshot=serde_json::from_str(*AFTER).unwrap();
    store::os_store::test_support::assert_dsl_pack_equivalence(&before);
    store::os_store::test_support::assert_dsl_pack_equivalence(&after);
    let mutation:DrawingMutation=serde_json::from_str(*MUTATION).unwrap();
    let result=mutation.diff(&before);
    assert!(result.messages().is_empty());
    assert_eq!(serde_json::to_value(result.diff()).unwrap(),serde_json::from_str::<serde_json::Value>(*DIFF).unwrap());
    assert_eq!(protocol::apply_diff_cold(result.diff(),&before).unwrap(),after);
    let committed:DrawingDiff=serde_json::from_str(*DIFF).unwrap();
    assert_eq!(protocol::apply_diff_cold(&committed,&before).unwrap(),after);
    let mut restored=after;
    let inverse=mutation.inverse(&before).expect("valid retained mutation inverse fixture");
    assert_eq!(inverse.len(),1);
    for step in inverse {semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut restored,&step).unwrap();}
    assert_eq!(restored,before);
    for (snapshot,text) in [(&before,*BEFORE),(&restored,*BEFORE)] {assert_eq!(serde_json::to_value(snapshot).unwrap(),serde_json::from_str::<serde_json::Value>(text).unwrap());}
    store::os_store::test_support::assert_op_line_round_trip(&mutation);
    store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
}
#[test]
fn unchanged_and_missing_group_isolation_targets_do_not_mutate() {
    let before:DrawingSnapshot=serde_json::from_str(*BEFORE).unwrap();
    let same: semio_s_artifact_draw_drawing_mutations::DrawingMutation=semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation("group-a".into(),false);
    let result=same.diff(&before);
    assert!(!result.messages().is_empty());
    assert_eq!(protocol::apply_diff_cold(result.diff(),&before).unwrap(),before);
    let missing=semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation::<semio_s_artifact_draw_drawing_mutations::DrawingMutation>("missing".into(),true);
    assert!(!missing.diff(&before).messages().is_empty());
    assert!(missing.inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
    let wrong_kind=semio_s_artifact_draw_drawing_style::set_group_isolation::mutation::set_group_isolation::<semio_s_artifact_draw_drawing_mutations::DrawingMutation>("shape-a".into(),true);
    assert!(!wrong_kind.diff(&before).messages().is_empty());assert!(wrong_kind.inverse(&before).expect("valid retained mutation inverse fixture").is_empty());
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff, restoring the committed before-document.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    let before:DrawingSnapshot=serde_json::from_str(*BEFORE).unwrap();
    let mutation:DrawingMutation=serde_json::from_str(*MUTATION).unwrap();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation,&before).await;
}
