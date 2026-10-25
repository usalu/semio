//! 📝️ Shared text editing fixtures prove semantic inverse and wire preservation.
use semio_framework_test_fixtures::fixture_text;

#[test]
fn text_edits_preserve_content_size_and_history_payload() {
    use protocol::{ColdMutation, Mutation};
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/🔣️.json")).unwrap();
    let layer = semio_s_artifact_draw_drawing_model::create_drawing_text_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Text")).to_string().into()).expect("nonempty authored identity"), "Text");
    let id = semio_s_artifact_draw_drawing_model::layer_id(&layer).to_string();
    let before = semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot { layers: vec![layer].into(), ..Default::default() };
    for (index,edit) in fixture["edits"].as_array().unwrap().iter().enumerate() {
        let mutation: semio_s_artifact_draw_drawing_mutations::DrawingMutation = semio_s_artifact_draw_drawing_style::update_text::mutation::update_text(id.clone().into(), edit["content"].as_str().unwrap().into(), edit["size"].as_f64().unwrap(),semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::parse(edit["fontFamily"].as_str().unwrap()).unwrap());
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
        let outcome = mutation.diff(&before);
        assert!(outcome.messages().is_empty());
        if let Some(delta)=&outcome.diff().layers{let patch=&delta.modified[0].patch;assert_eq!(patch.text_content.as_deref(),fixture["patches"][index]["textContent"].as_str());assert_eq!(patch.text_size,fixture["patches"][index]["textSize"].as_f64());assert_eq!(patch.font_family.map(|family|family.as_str()),fixture["patches"][index]["fontFamily"].as_str());}else{assert_eq!(fixture["patches"][index],serde_json::json!({}));}
        let mut after = protocol::apply_diff_cold(outcome.diff(), &before).unwrap();
        let semio_s_artifact_draw_drawing_model::DrawingLayerNode::Text(text) = &after.layers[0] else { panic!("Text kind changed") };
        assert_eq!(text.content, edit["content"].as_str().unwrap());
        assert_eq!(text.size, edit["size"].as_f64().unwrap());
        assert_eq!(text.font_family, semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::parse(edit["fontFamily"].as_str().unwrap()).unwrap());
        for inverse in mutation.inverse(&before).expect("valid retained mutation inverse fixture") { semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut after, &inverse).unwrap(); }
        assert_eq!(after, before);
    }
    for size in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        let rejected = semio_s_artifact_draw_drawing_style::update_text::mutation::update_text::<semio_s_artifact_draw_drawing_mutations::DrawingMutation>(id.clone().into(), "changed".into(), size,semio_s_artifact_draw_drawing_model::font_family::DrawingFontFamily::Anta).diff(&before);
        assert!(!rejected.messages().is_empty());
        assert_eq!(protocol::apply_diff_cold(rejected.diff(), &before).unwrap(), before);
    }
}

#[test]
fn canonical_text_scenario_matches_diff_apply_and_inverse() {
    use protocol::{ColdMutation, Mutation};
    let before: semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let after: semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/➡️after/🔣️.json")).unwrap();
    let mutation: semio_s_artifact_draw_drawing_mutations::DrawingMutation = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🦠️mutation/🔣️.json")).unwrap();
    let expected: semio_s_artifact_draw_drawing_diff::DrawingDiff = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🔺️diff/🔣️.json")).unwrap();
    let inverse: Vec<semio_s_artifact_draw_drawing_mutations::DrawingMutation> = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/↩️inverse/🔣️.json")).unwrap();
    assert_eq!(*mutation.diff(&before).diff(), expected);
    assert_eq!(protocol::apply_diff_cold(&expected, &before).unwrap(), after);
    assert_eq!(mutation.inverse(&before).expect("valid retained mutation inverse fixture"), inverse);
}

/// ⚖️ The concrete inverse's diffs sum to exactly the negative of the forward diff on the committed caption scenario.
#[semio_framework_async_macros::async_test]
async fn inverse_sums_to_the_negative_diff() {
    let before: semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let mutation: semio_s_artifact_draw_drawing_mutations::DrawingMutation = serde_json::from_str(fixture_text!("../../../🧫️fixtures/🧬️mutations/📝️update-text/📝️edit-caption/🦠️mutation/🔣️.json")).unwrap();
    protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &before).await;
}
