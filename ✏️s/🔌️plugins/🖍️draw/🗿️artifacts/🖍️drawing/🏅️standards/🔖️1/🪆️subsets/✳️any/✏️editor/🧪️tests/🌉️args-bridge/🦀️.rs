use semio_s_artifact_draw_drawing_editor::args_bridge::command_from_action;
use semio_s_artifact_draw_drawing_editor::DrawingCommand;
use semio_s_artifact_draw_drawing_editor_commands::{add_layer, edit_fill, edit_path, edit_selection, export_document, patch_layer, set_active_example};
use semio_s_artifact_draw_drawing_model::fill::FillType;
use semio_s_artifact_draw_drawing_model::fill::FillAxis;

#[test]
fn folds_camel_case_keys_and_json_values() {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"exampleId":"demo"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("json"));
    assert_eq!(command_from_action("setActiveExample", Some(&args)).expect("decodes"), DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }));
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"a","field":"opacity","value":0.5}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("json"));
    assert_eq!(command_from_action("patchLayer", Some(&args)).expect("decodes"), DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "a".into(), field: "opacity".into(), value: "0.5".into() }));
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"camera":{"x":1,"y":2,"zoom":1.5}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("json"));
    assert!(matches!(command_from_action("setCamera", Some(&args)).expect("decodes"), DrawingCommand::SetCamera(_)));
    assert_eq!(command_from_action("addLayer", None).expect("arg-less palette row"), DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }));
    assert_eq!(command_from_action("exportDocument", None).expect("arg-less palette row"), DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into(), ..Default::default() }));
    assert_eq!(command_from_action("exportDocument", Some(&semio_framework_value::DslValue::Object(vec![("format".into(), semio_framework_value::DslValue::String("svg".into()))]))).expect("explicit format"), DrawingCommand::ExportDocument(export_document::ExportDocument { format: "svg".into(), ..Default::default() }));
    assert!(command_from_action("noSuchAction", None).is_err());
}

#[test]
fn selection_action_uses_live_selection_when_ids_are_omitted() {
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"operation":"group"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    assert_eq!(command_from_action("editSelection", Some(&args)).unwrap(), DrawingCommand::EditSelection(edit_selection::EditSelection { operation: "group".into(), ids: Vec::new() }));
}

#[test]
fn path_coordinate_input_overrides_the_rendered_value() {
    use semio_s_artifact_draw_drawing_geometry::editing::{PathEdit, PathPoint, PathAxis};
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"path","edit":{"kind":"coordinate","index":1,"point":"anchor","axis":"x","value":0},"value":"42.5"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    assert_eq!(command_from_action("editPath", Some(&args)).unwrap(), DrawingCommand::EditPath(edit_path::EditPath { layer_id: "path".into(), edit: Box::new(PathEdit::Coordinate { index: 1, point: PathPoint::Anchor, axis: PathAxis::X, value: 42.5 }) }));
    let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"path","edit":"{\"kind\":\"reverse\"}"}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    assert_eq!(command_from_action("editPath", Some(&args)).unwrap(), DrawingCommand::EditPath(edit_path::EditPath { layer_id: "path".into(), edit: Box::new(PathEdit::Reverse) }));
}

#[test]
fn path_conversion_actions_round_trip_the_typed_command() {
    use semio_s_artifact_draw_drawing_geometry::editing::{PathEdit, SegmentType};
    for (name,target) in [("line",SegmentType::Line),("cubic",SegmentType::Cubic)] {
        let value = format!(r#"{{"layerId":"path","edit":{{"kind":"convert","index":1,"target":"{name}"}}}}"#);
        let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        let command = command_from_action("editPath",Some(&args)).unwrap();
        assert_eq!(command,DrawingCommand::EditPath(edit_path::EditPath { layer_id:"path".into(),edit:Box::new(PathEdit::Convert { index:1,target }) }));
        semio_os_store_test_support::assert_op_line_round_trip(&command);
        semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
    }
}

#[test]
fn path_join_action_round_trips_the_typed_command() {
    use semio_s_artifact_draw_drawing_geometry::editing::PathEdit;
    let args=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"path","edit":{"kind":"join","index":1,"other":2}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let command=command_from_action("editPath",Some(&args)).unwrap();
    assert_eq!(command,DrawingCommand::EditPath(edit_path::EditPath { layer_id:"path".into(),edit:Box::new(PathEdit::Join { index:1,other:2 }) }));
    semio_os_store_test_support::assert_op_line_round_trip(&command);
    semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
}

#[test]
fn fill_inputs_override_the_rendered_edit_with_their_typed_value() {
    use semio_s_artifact_draw_drawing_model::fill::FillEdit;
    for (json,edit) in [
        (r#"{"layerId":"a","edit":{"kind":"type","value":"solid"},"value":"radialGradient"}"#,FillEdit::Type { value:FillType::RadialGradient }),
        (r##"{"layerId":"a","edit":{"kind":"color","index":1,"value":[0,0,0,1]},"value":"#abc"}"##,FillEdit::Color { index:Some(1),value:[170.0/255.0,187.0/255.0,204.0/255.0,1.0] }),
        (r#"{"layerId":"a","edit":{"kind":"coordinate","axis":"r","value":50},"value":"24.5"}"#,FillEdit::Coordinate { axis:FillAxis::R,value:24.5 }),
    ] {
        let args = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
        assert_eq!(command_from_action("editFill",Some(&args)).unwrap(),DrawingCommand::EditFill(edit_fill::EditFill { layer_id:"a".into(),edit:Box::new(edit) }));
    }
}

#[test]
fn path_position_action_round_trips_one_two_axis_edit() {
    use semio_s_artifact_draw_drawing_geometry::editing::{PathEdit, PathPoint};
    let args=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"path","edit":{"kind":"position","index":1,"point":"anchor","to":[15,5]}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let command=command_from_action("editPath",Some(&args)).unwrap();
    assert_eq!(command,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Position {index:1,point:PathPoint::Anchor,to:[15.0,5.0]})}));
    semio_os_store_test_support::assert_op_line_round_trip(&command);
    semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
}

#[test]
fn path_translation_action_round_trips_the_typed_point_set() {
    use semio_s_artifact_draw_drawing_geometry::editing::{PathEdit, PathPoint, PathPointRef};
    let args=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(r#"{"layerId":"path","edit":{"kind":"translate","points":[{"index":0,"point":"anchor"},{"index":1,"point":"control2"}],"delta":[3,-2]}}"#, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
    let command=command_from_action("editPath",Some(&args)).unwrap();
    assert_eq!(command,DrawingCommand::EditPath(edit_path::EditPath {layer_id:"path".into(),edit:Box::new(PathEdit::Translate {points:vec![PathPointRef {index:0,point:PathPoint::Anchor},PathPointRef {index:1,point:PathPoint::Control2}],delta:[3.0,-2.0]})}));
    semio_os_store_test_support::assert_op_line_round_trip(&command);
    semio_os_store_test_support::assert_op_text_binary_equivalence(&command);
}

#[test]
fn action_bridge_matches_edit_input_fixture() {
    let source = include_str!("🔣️.json");
    let cases: serde_json::Value = serde_json::from_str(source).expect("edit input fixture");
    for case in cases.as_array().expect("case list") {
        let action = case["action"].as_str().expect("action name");
        let args = semio_framework_pack_json::from_json_str(&case["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject)
            .expect("action arguments");
        let command = command_from_action(action, Some(&args)).expect("typed action");
        let expected = case["expectedEdit"].clone();
        match command {
            DrawingCommand::EditPath(payload) => assert_eq!(*payload.edit, serde_json::from_value::<semio_s_artifact_draw_drawing_geometry::editing::PathEdit>(expected).expect("path edit")),
            DrawingCommand::EditFill(payload) => assert_eq!(*payload.edit, serde_json::from_value::<semio_s_artifact_draw_drawing_model::fill::FillEdit>(expected).expect("fill edit")),
            command => panic!("unexpected fixture action {command:?}"),
        }
    }
}

#[test]
fn host_pointer_numbers_and_nullable_export_fields_are_typed() {
    let source = include_str!("🔣️host-actions.json");
    let cases: serde_json::Value = serde_json::from_str(source).expect("host action fixture");
    for case in cases.as_array().expect("case list") {
        let action = case["action"].as_str().expect("action");
        let input = &case["args"];
        let args = semio_framework_pack_json::from_json_str(&input.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject)
            .expect("host action arguments");
        let decoded = command_from_action(action, Some(&args)).expect("typed host command");
        match decoded {
            DrawingCommand::CanvasPointerDown(payload) => {
                assert_eq!(payload.app_instance_id.map(u64::from), case["instance"].as_u64());
                assert_eq!(payload.operation_id, case["operation"].as_u64());
                assert_eq!(payload.checkpoint_pending_work, case["pending"].as_u64());
                assert_eq!(payload.world_x, case["worldX"].as_f64());
                assert_eq!(payload.world_y, case["worldY"].as_f64());
            }
            DrawingCommand::CanvasPointerMove(payload) => {
                let expected_samples: Vec<[f64; 2]> = serde_json::from_value(case["samples"].clone()).expect("samples");
                assert_eq!(payload.samples, expected_samples);
                assert_eq!([payload.x, payload.y], expected_samples.last().copied().expect("current sample"));
            }
            DrawingCommand::ExportDocument(payload) => {
                assert_eq!(payload.format, case["format"].as_str().expect("format"));
                assert_eq!(payload.width.map(u64::from), case["width"].as_u64());
                assert_eq!(payload.height.map(u64::from), case["height"].as_u64());
                assert_eq!(payload.transparent, case["transparent"].as_bool());
            }
            command => panic!("unexpected host command {command:?}"),
        }
    }
}
