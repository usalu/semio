use super::args_bridge::command_from_action;
use super::*;

#[test]
fn folds_camel_case_keys_and_json_values() {
    let args = dsl::json::to_dsl_value(&dsl::json::parse(r#"{"exampleId":"demo"}"#).expect("json"));
    assert_eq!(command_from_action("setActiveExample", Some(&args)).expect("decodes"), DrawingCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }));
    let args = dsl::json::to_dsl_value(&dsl::json::parse(r#"{"layerId":"a","field":"opacity","value":0.5}"#).expect("json"));
    assert_eq!(command_from_action("patchLayer", Some(&args)).expect("decodes"), DrawingCommand::PatchLayer(patch_layer::PatchLayer { layer_id: "a".into(), field: "opacity".into(), value: "0.5".into() }));
    let args = dsl::json::to_dsl_value(&dsl::json::parse(r#"{"camera":{"x":1,"y":2,"zoom":1.5}}"#).expect("json"));
    assert!(matches!(command_from_action("setCamera", Some(&args)).expect("decodes"), DrawingCommand::SetCamera(_)));
    assert_eq!(command_from_action("addLayer", None).expect("arg-less palette row"), DrawingCommand::AddLayer(add_layer::AddLayer { kind: "path".into() }));
    assert_eq!(command_from_action("exportDocument", None).expect("arg-less palette row"), DrawingCommand::ExportDocument(export_document::ExportDocument { format: "pdf".into() }));
    assert_eq!(command_from_action("exportDocument", Some(&dsl::DslValue::Object(vec![("format".into(), dsl::DslValue::String("svg".into()))]))).expect("explicit format"), DrawingCommand::ExportDocument(export_document::ExportDocument { format: "svg".into() }));
    assert!(command_from_action("noSuchAction", None).is_err());
}
