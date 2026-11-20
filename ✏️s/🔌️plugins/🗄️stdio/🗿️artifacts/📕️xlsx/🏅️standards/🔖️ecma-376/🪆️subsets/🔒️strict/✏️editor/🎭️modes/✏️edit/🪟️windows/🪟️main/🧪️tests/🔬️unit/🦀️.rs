use super::*;
use semio_plugin::{Component, Trigger};

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn strict_grid_binds_blank_cell_creation_to_the_strict_controller() {
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSheet;
    use semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxWorkbook;
    fn visit(node: &BuiltNode, found: &mut bool) {
        if matches!(node.component, Component::Input(_)) {
            *found |= node.bindings.iter().any(|binding| binding.trigger == Trigger::Commit && binding.action.scope.as_str() == "s.stdio.xlsx@ecma-376/strict#editor" && binding.action.name.as_str() == "set-cell");
        }
        for child in &node.children {
            visit(child, found);
        }
    }
    let document = semio_s_artifact_stdio_xlsx_base_model::standards::v_ecma_376::subsets::base::schema::construction::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Blank".into(), cells: Vec::new() }], ..Default::default() });
    let node = render(&document, Locale::En, &TreeWindows::unhosted(), semio_plugin::UiPublicationRevision(23)).unwrap();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node), |node| {
        let mut found = false;
        visit(node, &mut found);
        assert!(found);
    });
}
