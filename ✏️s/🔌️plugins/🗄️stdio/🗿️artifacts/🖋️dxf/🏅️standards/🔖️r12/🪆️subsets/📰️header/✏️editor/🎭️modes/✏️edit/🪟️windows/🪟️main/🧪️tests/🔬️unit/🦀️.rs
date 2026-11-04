use crate::standards::v_r12::subsets::any::schema::snapshot::{DxfArc, DxfCircle, DxfEntity, DxfLine, DxfSnapshot};
use semio_framework_plugin::app::ImageWindowKit;
use semio_framework_plugin::WindowKit;
use crate::editor::dxf::modes::edit::windows::main::*;
use crate::standards::v_r12::subsets::any::io::text::snapshot::preview::preview_svg;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_image_window_kit() {
    assert_eq!(definition().id, ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_node_for_the_default_document() {
    assert!(render(&DxfSnapshot::default()).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn preview_draws_every_typed_entity_and_follows_edits() {
    let mut document = DxfSnapshot::default();
    document.entities.push(DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [4.0, 0.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }));
    document.entities.push(DxfEntity::Circle(DxfCircle { center: [2.0, 2.0, 0.0], radius: 1.0, layer: "0".into(), unknown_group_codes: vec![] }));
    document.entities.push(DxfEntity::Arc(DxfArc { center: [0.0, 0.0, 0.0], radius: 2.0, start_angle: 0.0, end_angle: 90.0, layer: "0".into(), unknown_group_codes: vec![] }));
    let before = preview_svg(&document);
    assert!(before.contains("<path d=\"M0 0L4 0\"/>") && before.contains("<circle cx=\"2\" cy=\"2\" r=\"1\"/>") && before.contains("A2 2 0 0 1 0 2"));
    let DxfEntity::Line(line) = &mut document.entities[0] else { unreachable!() };
    line.end = [4.0, 3.0, 0.0];
    assert!(preview_svg(&document).contains("<path d=\"M0 0L4 3\"/>"));
    assert_ne!(before, preview_svg(&document));
}
