mod tests {
    use super::*;

    #[test]
    fn patches_numeric_parameter_with_constraints() {
        let parameter = create_default_os_parameter(&OsParameterType::Numeric, "Zoom", None);
        let patched = patch_os_parameter(&parameter, &serde_json::json!({ "value": 12.0, "max": 10.0 }));
        match patched {
            OsParameter::Numeric { value, .. } => assert_eq!(value, 10.0),
            _ => panic!("expected numeric"),
        }
    }

    #[test]
    fn applies_json_pointer_parameter_overrides() {
        let snapshot = serde_json::json!({ "brushSize": 8 });
        let overridden = apply_parameter_values_to_snapshot(
            snapshot,
            &[OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "/brushSize".into() }],
            &[OsParameter::Numeric { id: "p1".into(), name: "Brush".into(), value: 42.0, min: None, max: None, step: None }],
            "i1",
        );
        assert_eq!(overridden["brushSize"], 42.0);
    }

    #[test]
    fn resolves_fixture_json_by_slug() {
        register_os_fixture_json("🖍️semio.draw.json", r#"{"schema":"draw.document","id":"semio"}"#);
        let json = os_fixture_json("🖍️semio.draw.json").expect("registered fixture");
        let parsed: Value = serde_json::from_str(&json).expect("json");
        assert_eq!(parsed["schema"], "draw.document");
        assert_eq!(parsed["id"], "semio");
    }

    #[test]
    fn materializes_instance_documents_with_parameter_overrides() {
        let json = materialize_os_app_instance_document_json(r#"{"schema":"draw.document","id":"semio"}"#, "app-draw-1", &[], &[]);
        let parsed: Value = serde_json::from_str(&json).expect("json");
        assert_eq!(parsed["schema"], "draw.document");
        assert_eq!(parsed["id"], "semio");
    }

    fn sample_config_spec() -> ConfigSpec {
        ConfigSpec {
            fields: vec![
                semio_framework::ConfigFieldSpec { key: "zoom".into(), label: "Zoom".into(), shape: semio_framework::ConfigFieldShape::Number { min: None, max: None, step: None }, default: Some(dsl::DslValue::from(&serde_json::json!(1.0))) },
                semio_framework::ConfigFieldSpec { key: "mode".into(), label: "Mode".into(), shape: semio_framework::ConfigFieldShape::Select { options: vec!["A".into(), "B".into()] }, default: Some(dsl::DslValue::from(&serde_json::json!("A"))) },
                semio_framework::ConfigFieldSpec { key: "flag".into(), label: "Flag".into(), shape: semio_framework::ConfigFieldShape::Toggle, default: None },
                semio_framework::ConfigFieldSpec { key: "label".into(), label: "Label".into(), shape: semio_framework::ConfigFieldShape::Text, default: None },
            ],
        }
    }

    // 🪦️ `validates_matching_parameter_config_bindings`/`rejects_mismatched_parameter_config_bindings`/
    // `rejects_parameter_config_binding_to_unknown_field` DELETED alongside the dead
    // `validate_parameter_config_binding` they exclusively exercised (see that deletion's note
    // above) — the type-check logic they asserted is duplicated verbatim (per its own doc
    // comment, "ported from os-core's `validate_parameter_config_binding`") in the live
    // `workflow::validate_workflow_parameter_config_binding`, which these tests never called.

    #[test]
    fn build_configure_config_starts_from_config_spec_defaults() {
        let config_spec = sample_config_spec();
        let config = build_configure_config("i1", &[], &[], &config_spec);
        let config: Value = Value::from(config);
        assert_eq!(config["zoom"], 1.0);
        assert_eq!(config["mode"], "A");
    }

    #[test]
    fn build_configure_config_overlays_bound_parameter_values() {
        let config_spec = sample_config_spec();
        let parameters = vec![OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 42.0, min: None, max: None, step: None }];
        let bindings = vec![OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "zoom".into() }];
        let config = build_configure_config("i1", &parameters, &bindings, &config_spec);
        let config: Value = Value::from(config);
        assert_eq!(config["zoom"], 42.0);
        assert_eq!(config["mode"], "A");
    }
}
