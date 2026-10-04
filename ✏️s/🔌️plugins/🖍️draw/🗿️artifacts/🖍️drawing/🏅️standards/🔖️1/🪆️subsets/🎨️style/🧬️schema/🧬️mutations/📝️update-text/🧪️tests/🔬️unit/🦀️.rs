//! 📝️ Shared text editing fixtures prove semantic inverse and wire preservation.
#[test]
fn text_edits_preserve_content_size_and_history_payload() {
    use protocol::{Mutation, MutationDiff};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let layer = crate::schema::create_drawing_text_layer("Text");
    let id = crate::schema::layer_id(&layer).to_string();
    let before = crate::DrawingSnapshot { layers: vec![layer], ..Default::default() };
    for edit in fixture["edits"].as_array().unwrap() {
        let mutation = super::mutation::update_text(id.clone(), edit["content"].as_str().unwrap().into(), edit["size"].as_f64().unwrap());
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let outcome = mutation.diff(&before);
        assert!(outcome.messages().is_empty());
        let mut after = outcome.diff().apply(&before).unwrap();
        let crate::DrawingLayerNode::Text(text) = &after.layers[0] else { panic!("Text kind changed") };
        assert_eq!(text.content, edit["content"].as_str().unwrap());
        assert_eq!(text.size, edit["size"].as_f64().unwrap());
        for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture") { crate::mutations::apply_drawing_mutation(&mut after, &inverse).unwrap(); }
        assert_eq!(after, before);
    }
    for size in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let rejected = super::mutation::update_text(id.clone(), "changed".into(), size).diff(&before);
        assert!(!rejected.messages().is_empty());
        assert_eq!(rejected.diff().apply(&before).unwrap(), before);
    }
}

#[test]
fn canonical_text_scenario_matches_diff_apply_and_inverse() {
    use protocol::{Mutation, MutationDiff};
    let before: crate::DrawingSnapshot = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after: crate::DrawingSnapshot = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let mutation: crate::DrawingMutation = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🦠️mutation/🔣️.json")).unwrap();
    let expected: crate::DrawingDiff = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🔺️diff/🔣️.json")).unwrap();
    let inverse: Vec<crate::DrawingMutation> = serde_json::from_str(include_str!("../../../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/↩️inverse/🔣️.json")).unwrap();
    assert_eq!(*mutation.diff(&before).diff(), expected);
    assert_eq!(expected.apply(&before).unwrap(), after);
    assert_eq!(mutation.inverse(&before).expect("valid retained mutation inverse fixture"), inverse);
}
