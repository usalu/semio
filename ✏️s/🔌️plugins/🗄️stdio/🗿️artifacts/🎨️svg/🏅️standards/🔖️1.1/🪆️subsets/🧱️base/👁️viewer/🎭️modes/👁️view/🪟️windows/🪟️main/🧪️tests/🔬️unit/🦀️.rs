use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = SvgSnapshot::default();
    let _node = render(&document).expect("render");
}

#[semio_framework_async_macros::async_test]
async fn render_refuses_invalid_declaration_without_panicking() {
    let mut document = SvgSnapshot::default();
    document.doc.declaration = Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration::new("1.0", Some("ISO-8859-1".into()), None));
    assert!(render(&document).is_err());
}
