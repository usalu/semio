use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_uses_the_frozen_window_kit_kind_id() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_projects_canonical_bmp_pixels_to_browser_png() {
    let document = crate::standards::v_v3::subsets::any::schema::demo_bmp_snapshot();
    let view = image_view(&document).expect("preview");
    assert_eq!((view.width, view.height, view.mime.as_str()), (4, 2, "image/png"));
    let preview = crate::standards::v_v3::subsets::any::io::bmp_png_preview(&document).expect("PNG");
    assert_eq!(view.base64, semio_s_artifact_stdio_contract::base64_standard(&preview.bytes));
    let decoded = semio_framework_pixels::decode_png(&preview.bytes).expect("PNG");
    assert_eq!(decoded.pixels, crate::standards::v_v3::subsets::any::schema::operations::bmp_rgba8_preview(&document).expect("RGBA"));
    render(&document, Locale::En).expect("image window");
}

#[semio_framework_async_macros::async_test]
async fn invalid_projection_keeps_the_artifact_mounted_with_a_localized_state() {
    let mut document = BmpSnapshot::default();
    document.image.pixels = crate::schema::snapshot::BmpPixels::Direct { samples: Vec::new() };
    assert!(image_view(&document).is_err());
    let node = render(&document, Locale::De).expect("invalid display capability does not reject the artifact window");
    assert_eq!(node.key.as_str(), ImageWindowKit::KIND_ID);
    assert_eq!(node.children.len(), 1);
    let semio_framework_plugin::Component::Text(text) = &node.children[0].component else { panic!("explicit unavailable state") };
    assert_eq!(text.value.0.as_str(), "Vorschau nicht verfügbar");
}
