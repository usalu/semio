use super::*;

#[test]
fn archive_window_requires_complete_draft_arguments() {
    let definition = definition();
    assert_eq!(definition.id, WINDOW_KIND_ID);
    assert_eq!(definition.body_key, BODY_KEY);
    let action = definition.actions.iter().find(|action| action.id == "set-node").unwrap();
    assert_eq!(action.args.len(), 3);
}

#[test]
fn archive_window_renders_explicit_localized_drafts() {
    for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
        let document = ZipSnapshot { comment: "Archive comment text".into(), ..Default::default() };
        let node = render(&document, &TreeWindows::unhosted(), locale, semio_framework_plugin::UiPublicationRevision(23)).unwrap();
        assert_eq!(node.key.as_str(), "archive-fields");
        assert_eq!(node.children[0].key.as_str(), "archive-field-0");
        assert!(!node.children[0].children.is_empty());
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
    }
}
