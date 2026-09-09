use super::*;

#[semio_framework_async_macros::async_test]
async fn catalogue_includes_control_kinds() {
    let raw = catalogue_json();
    assert!(raw.contains("control.if"));
    assert!(raw.contains("control.while"));
    assert!(raw.contains("control.repeat"));
}
