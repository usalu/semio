mod generation_forms_tests {
    use super::super::{DslValue, PLAYBOOK_DOCUMENT_SCHEMA, PlaybookStep};
    use super::*;

    fn sample_spec() -> PlaybookSpec {
        PlaybookSpec {
            schema: PLAYBOOK_DOCUMENT_SCHEMA.into(),
            id: "sample".into(),
            version: "1".into(),
            title: None,
            steps: vec![PlaybookStep {
                id: "s".into(),
                title: "Inputs".into(),
                description: None,
                blocks: vec![PlaybookBlock {
                    id: "width".into(),
                    label: "Width".into(),
                    kind: "slider".into(),
                    description: None,
                    required: None,
                    placeholder: None,
                    default: Some(DslValue::float(1.0)),
                    min: Some(0.0),
                    max: Some(10.0),
                    step: Some(0.5),
                    unit: None,
                    text: None,
                    options: None,
                    fields: None,
                    schema: None,
                    src: None,
                    accept: None,
                    fixture_slug: None,
                    params: None,
                    condition: None,
                }],
            }],
        }
    }

    #[test]
    fn generation_crud_round_trip() {
        let spec = sample_spec();
        let mut state = GenerationPlayState::default();
        let id = add_generation(&mut state, &spec);
        assert_eq!(state.generations.len(), 1);
        rename_generation(&mut state, &id, "Variant A");
        update_generation_values(&mut state, &id, "width", DslValue::float(4.0));
        assert_eq!(selected_generation(&state).unwrap().name, "Variant A");
        remove_generation(&mut state, &id);
        assert!(state.generations.is_empty());
    }

    #[test]
    fn render_generations_tree_contains_add_action() {
        let json = serde_json::to_string(&render_generations_tree("flow-play", "flow-generate", &[], None, Locale::En, Terminology::Native)).unwrap();
        assert!(json.contains("addGeneration"));
    }
}
