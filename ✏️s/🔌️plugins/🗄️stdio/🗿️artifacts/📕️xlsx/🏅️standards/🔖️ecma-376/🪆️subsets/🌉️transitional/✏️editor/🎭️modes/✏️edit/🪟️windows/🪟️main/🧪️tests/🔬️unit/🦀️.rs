use super::*;
use semio_framework_plugin::{Component, Trigger};

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_table_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[semio_framework_async_macros::async_test]
async fn transitional_grid_binds_blank_cell_creation_to_the_transitional_controller() {
    use crate::standards::v_ecma_376::subsets::base::schema::snapshot::{XlsxSheet, XlsxWorkbook};
    fn visit(node: &BuiltNode, found: &mut bool) {
        if matches!(node.component, Component::Input(_)) {
            *found |= node.bindings.iter().any(|binding| binding.trigger == Trigger::Commit && binding.action.scope.as_str() == "s.stdio.xlsx@ecma-376/transitional#editor" && binding.action.name.as_str() == "set-cell");
        }
        for child in &node.children {
            visit(child, found);
        }
    }
    let document = crate::standards::v_ecma_376::subsets::base::io::export::serializers::build_minimal_xlsx(XlsxWorkbook { sheets: vec![XlsxSheet { name: "Blank".into(), cells: Vec::new() }], ..Default::default() });
    let node = render(&document, Locale::En, &TreeWindows::unhosted(), semio_framework_plugin::UiPublicationRevision(23)).unwrap();
    let mut found = false;
    visit(&node, &mut found);
    assert!(found);
}
