use super::*;
use semio_plugin::app::ImageWindowKit;
use semio_plugin::WindowKit;

#[semio_framework_async_macros::async_test]
async fn definition_declares_the_image_window_kit() {
    let def = definition();
    assert_eq!(def.id, ImageWindowKit::KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_produces_a_scene_node_for_the_default_document() {
    let document = DwgSnapshot::default();
    assert!(render(&document).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn real_drawing_renders_its_typed_entities_as_vector_geometry() {
    let bytes = include_bytes!("../../../../../../../🖼️assets/🏛️architectural/🏛️architectural.dwg");
    let document = semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::io::import::decode_dwg(bytes).expect("a real AC1024 file opens");
    let svg = semio_s_artifact_stdio_dwg_snapshot_drawing::standards::v_ac1024::subsets::any::schema::snapshot::drawing::preview::preview_svg(&document.drawing);
    assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"));
    assert!(svg.contains("<path") || svg.contains("<circle") || svg.contains("<polygon"), "the fixture's typed geometry draws something");
    assert!(render(&document).is_ok());
}
