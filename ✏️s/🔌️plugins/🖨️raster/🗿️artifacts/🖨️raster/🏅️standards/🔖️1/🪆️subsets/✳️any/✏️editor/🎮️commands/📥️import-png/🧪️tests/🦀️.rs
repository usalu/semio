use super::*;
use semio_framework_value::DslValue;
use semio_plugin::{Effect, HistoryView};

#[test]
fn picker_and_action_share_the_png_file_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("fixture.json")).unwrap();
    let document = semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();
    let history = HistoryView::empty();
    let config = RasterConfig::default();
    let emit = handle(&ImportPng {}, &ArtifactView::new(&document, &history), &ConfigView { snapshot: &config, window: None }).unwrap();
    let [Effect::RequestFileOpen { accept, read_as, import_action, multiple, max_file_bytes, .. }] = emit.effects.as_slice() else {
        panic!("one PNG picker effect required");
    };
    assert_eq!(serde_json::json!({"accept":accept,"readAs":read_as,"importAction":import_action,"maxFileBytes":max_file_bytes}), fixture["picker"]);
    assert!(!*multiple);
    for row in fixture["cases"].as_array().unwrap() {
        let mut payload = row["payload"].as_str().unwrap().to_owned();
        payload.reserve(1);
        let pointer = payload.as_ptr();
        let mut args = Some(DslValue::object([("payload".into(), DslValue::String(payload)), ("name".into(), DslValue::String("image.png".into()))]));
        let expected = row["offset"].as_u64().map(|offset| offset as usize);
        assert_eq!(base64_offset(row["payload"].as_str().unwrap()), expected, "{}", row["name"]);
        let result = file::ImportPngFile::receive(&mut args);
        if expected.is_some() {
            assert_eq!(result.unwrap().payload.as_ptr(), pointer, "file source moves without copying");
        } else {
            assert!(result.is_err());
            assert_eq!(args.as_ref().unwrap().get("payload").unwrap().as_str().unwrap().as_ptr(), pointer);
        }
    }
}
