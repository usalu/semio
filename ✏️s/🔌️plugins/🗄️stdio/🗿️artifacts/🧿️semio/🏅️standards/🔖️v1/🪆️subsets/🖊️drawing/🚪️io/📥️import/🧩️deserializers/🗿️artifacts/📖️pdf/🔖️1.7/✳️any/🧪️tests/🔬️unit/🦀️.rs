use super::*;

#[semio_framework_async_macros::async_test]
async fn maps_page_text_and_media_box() {
    let pdf = semio_s_artifact_stdio_pdf::io::text_document(&[(200.0, 100.0, "hello semio")]);
    let drawing = semio_framework_plugin::resolve_ready(SemioDrawingFromPdf::deserialize(&pdf)).expect("deserialize");
    assert_eq!(drawing.canvas.width, 200.0);
    assert_eq!(drawing.canvas.height, 100.0);
    assert_eq!(drawing.layers.len(), 1);
    match &drawing.layers[0].root {
        DrawNode::Group { children, .. } => match &children[0] {
            DrawNode::Text { value, .. } => assert_eq!(value, "hello semio"),
            other => panic!("expected Text, got {other:?}"),
        },
        other => panic!("expected Group, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn rejects_no_pages() {
    assert!(semio_framework_plugin::resolve_ready(SemioDrawingFromPdf::deserialize(&PdfSnapshot::default())).is_err());
}
