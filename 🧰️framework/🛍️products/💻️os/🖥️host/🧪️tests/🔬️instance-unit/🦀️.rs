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
    fn materializes_instance_documents_with_parameter_overrides() {
        let json = materialize_os_app_instance_document_json(r#"{"schema":"draw.document","id":"semio"}"#, "app-draw-1", &[], &[]);
        let parsed: Value = serde_json::from_str(&json).expect("json");
        assert_eq!(parsed["schema"], "draw.document");
        assert_eq!(parsed["id"], "semio");
    }

    fn sample_config_spec() -> ConfigSpec {
        ConfigSpec {
            fields: vec![
                semio_framework::ActionArgDef::number("zoom", semio_framework_ui_locale::LocalizedLabel::native("Zoom", "Zoom")).default_value(&1.0),
                semio_framework::ActionArgDef::select("mode", semio_framework_ui_locale::LocalizedLabel::native("Mode", "Modus"), vec![semio_framework::ActionArgOption::new("A", semio_framework_ui_locale::LocalizedLabel::data("A")), semio_framework::ActionArgOption::new("B", semio_framework_ui_locale::LocalizedLabel::data("B"))]).default_value(&"A"),
                semio_framework::ActionArgDef::toggle("flag", semio_framework_ui_locale::LocalizedLabel::native("Flag", "Markierung")),
                semio_framework::ActionArgDef::text("label", semio_framework_ui_locale::LocalizedLabel::native("Label", "Beschriftung")),
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
