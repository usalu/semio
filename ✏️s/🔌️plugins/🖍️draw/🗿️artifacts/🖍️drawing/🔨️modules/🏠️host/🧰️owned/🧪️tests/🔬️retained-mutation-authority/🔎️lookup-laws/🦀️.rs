//! ✏️ The registered Drawing rename retains the selected identity and original inverse name.

use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_s_artifact_draw_drawing_model::{find_drawing_layer, layer_base};
use semio_s_artifact_draw_drawing_mutations::inverse_drawing_mutation;

#[test]
fn drawing_rename_and_inverse_keep_the_original_name_and_id() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧰️owned/🧪️tests/🧫️fixtures/📋️native-owner/🔣️.json")).unwrap();
    let mut input: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🏅️standards/🔖️1/🪆️subsets/🧱️structure/🧪️tests/🧫️fixtures/🧬️mutations/🗑️delete-layer/🚫️removes/📸️snapshot/⬅️before/🔣️.json")).unwrap();
    let expansion = law["repeat"].as_u64().unwrap() as usize;
    let long = format!("{}\0", law["text"].as_str().unwrap().repeat(expansion));
    input["layers"][1]["children"][0]["name"] = long.clone().into();
    let mut long_id = input["layers"][0].clone();
    long_id["id"] = long.clone().into();
    long_id["name"] = "Long Identity".into();
    let mut deep = input["layers"][0].clone();
    deep["id"] = "deep-rename".into();
    deep["name"] = "Deep Original".into();
    let group_template = input["layers"][1].clone();
    let deep_case = law["rename"]["cases"].as_array().unwrap().iter().find(|case| case["deep"] == true).unwrap();
    for depth in 0..deep_case["path"].as_array().unwrap().len() - 1 {
        let mut group = group_template.clone();
        group["id"] = format!("nested-{depth}").into();
        group["children"] = serde_json::json!([deep]);
        deep = group;
    }
    let mut duplicate = input["layers"][0].clone();
    duplicate["name"] = "Later Duplicate".into();
    input["layers"].as_array_mut().unwrap().extend([long_id, duplicate, deep]);
    let snapshot: DrawingSnapshot = serde_json::from_value(input).unwrap();
    for case in law["rename"]["cases"].as_array().unwrap() {
        let target = if case["repeatTarget"].as_bool() == Some(true) { long.as_str() } else { case["target"].as_str().unwrap() };
        let mut name = case["newName"].as_str().unwrap().repeat(if case["repeatNewName"].as_bool() == Some(true) { expansion } else { 1 });
        if case["disposition"] == "no-op" && case["repeatNewName"] == true || case["suffix"].is_string() {
            name.push('\0');
        }
        name.push_str(case["suffix"].as_str().unwrap_or_default());
        let original_name = case["inverseName"].as_str().map(|value| if value == "long" { long.as_str() } else { value });
        assert_eq!(
            find_drawing_layer(&snapshot, target).map(|node| layer_base(node).name.to_string_owner()),
            original_name.map(str::to_owned)
        );
        let mutation = DrawingMutation::RenameLayer(RenameLayer { layer_id: target.into(), new_name: name.as_str().into() });
        let outcome = <DrawingMutation as protocol::ColdMutation<DrawingSnapshot>>::diff(&mutation, &snapshot);
        assert_eq!(outcome.diff().layers.is_some(), case["disposition"] == "changed");
        let code = outcome.messages().first().map(|message| message.code.0.as_str());
        match case["disposition"].as_str().unwrap() {
            "missing" => assert_eq!(code, Some("mutation.target-missing")),
            "no-op" => assert_eq!(code, Some("mutation.no-op")),
            "changed" => assert_eq!(code, None),
            _ => panic!("unsupported fixture disposition"),
        }
        let inverse = inverse_drawing_mutation(&snapshot, &mutation).unwrap();
        let inverse_name = original_name.filter(|_| case["disposition"] == "changed");
        assert_eq!(inverse.len(), usize::from(inverse_name.is_some()));
        if let Some(original_name) = inverse_name {
            let DrawingMutation::RenameLayer(inverse) = &inverse[0] else { panic!("rename inverse kind") };
            assert_eq!(inverse.layer_id.to_string_owner(), target);
            assert_eq!(inverse.new_name.to_string_owner(), original_name);
        }
    }
}
