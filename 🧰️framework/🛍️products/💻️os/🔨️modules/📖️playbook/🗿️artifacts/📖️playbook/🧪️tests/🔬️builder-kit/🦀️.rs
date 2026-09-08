mod builder_kit_tests {
    use super::super::empty_playbook_snapshot;
    use super::*;

    fn sample_config() -> PlaybookBuilderConfig {
        PlaybookBuilderConfig { action_namespace: "playbook-play", controller_id: "playbook-play", labels: PLAYBOOK_BUILDER_LABELS_EN }
    }

    #[test]
    fn render_playbook_builder_emits_block_list_component_scene() {
        let spec = empty_playbook_snapshot();
        let config = sample_config();
        let node = render_playbook_builder("surface", &spec, &[], None, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"componentKind\":\"block-list\""));
        assert!(json.contains("\"blockList\""));
    }

    fn open_topic_entry() -> ProgramContributionEntry {
        ProgramContributionEntry {
            plugin_id: "playbook-module-procedural".into(),
            topic_contribution: Some(semio_framework::TopicContribution::new(
                "playbook.blockKind",
                DslValue::object([
                    ("appId".to_string(), DslValue::String("playbook-module-procedural".to_string())),
                    ("blockKind".to_string(), DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), DslValue::String("building".to_string())),
                    ("defaultValueJson".to_string(), DslValue::String("{}".to_string())),
                    ("paramsBodyKey".to_string(), DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), DslValue::String("preview".to_string())),
                ]),
            )),
        }
    }

    #[test]
    fn resolve_block_kind_extensions_reads_open_topic_contribution() {
        let extensions = resolve_block_kind_extensions(&[open_topic_entry()]);
        assert_eq!(extensions, vec![("buildingComponent".to_string(), "Building Component".to_string(), "building".to_string())]);
    }

    #[test]
    fn resolve_block_kind_extensions_ignores_unrelated_topics() {
        let mut entry = open_topic_entry();
        entry.topic_contribution = Some(semio_framework::TopicContribution::new("cad.computer", DslValue::object([("unrelated".to_string(), DslValue::Bool(true))])));
        let extensions = resolve_block_kind_extensions(&[entry]);
        assert!(extensions.is_empty());
    }
}
