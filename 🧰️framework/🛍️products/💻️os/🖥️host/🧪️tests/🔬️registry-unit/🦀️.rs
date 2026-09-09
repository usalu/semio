mod tests {
    use super::*;

    #[test]
    fn owned_artifact_kind_formats_survive_host_registry_projection() {
        let value: semio_framework::DslValue = dsl::os_pack::json::from_json_str(include_str!("../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json")).unwrap();
        let spec: ArtifactKindSpec = semio_framework::from_dsl_value(value).unwrap();
        assert_eq!(spec.export_stdio_kinds, ["stdio.svg", "stdio.png"]);
        assert_eq!(spec.import_stdio_kinds, ["stdio.dwg", "stdio.svg"]);
        register_artifact_descriptor(&spec);
        let projected = try_os_artifact_descriptor(&spec.id).unwrap();
        assert_eq!(projected.export_stdio_kinds, spec.export_stdio_kinds);
        assert_eq!(projected.import_stdio_kinds, spec.import_stdio_kinds);
        assert_eq!(projected.schema, spec.schema);
        assert_eq!(projected.media_type, spec.media_type);
    }

    #[test]
    fn registers_app_io_and_resolves_registration() {
        let app = AppDefinition {
            id: "draw".into(),
            role: AppRole::Editor,
            dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
            label: LocalizedLabel::data("Draw"),
            breadcrumb: vec!["semio".into(), "draw".into()],
            icon_id: None,
            controller_id: "draw-play".into(),
            modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
            default_mode_id: "edit".into(),
            window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                id: "draw".into(),
                label: LocalizedLabel::data("Draw"),
                body_key: "draw".into(),
                surface_kind: SurfaceKind::Canvas2d,
                icon_id: "pen-tool".into(),
                options: ui_wgpu::wgpu::WindowOptions::default(),
                actions: Vec::new(),
                utilities: Vec::new(),
                interactions: Vec::new(),
                params_schema: None,
                artifact_snapshot_schema: None,
                input_event_schema: None,
                output_schema: None,
                capabilities: Vec::new(),
            }),
            panel_tabs: vec![],
            keybindings: vec![],
            utilities: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            interactions: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: HashMap::new(),
            introduction: None,
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: crate::host::resolve_kernel_future(ConfigSpec::empty()),
            command_grammar: crate::host::resolve_kernel_future(semio_framework::CommandGrammar::empty()),
            io: crate::host::resolve_kernel_future(semio_framework::AppIo::from_document(
                "draw.document",
                MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                semio_framework::ArtifactPresentation { id: "draw".into(), name: "Draw".into(), dimension: "2d".into(), component_kind: "draw".into() },
            )),
            tutorials: Vec::new(),
        };
        register_app_io("draw", &app);
        let registration = os_app_registration("draw", "draw").expect("registration");
        assert_eq!(registration.source_format, "draw.document");
        let palette = workflow_palette();
        assert!(palette.iter().any(|entry| entry.plugin_id == "draw" && entry.app_id == "draw"));
    }
}
