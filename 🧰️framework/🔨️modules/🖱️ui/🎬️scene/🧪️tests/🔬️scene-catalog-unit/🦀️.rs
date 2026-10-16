//! 🗃️ Catalog proof: every typed scene of the facade decodes the language-neutral retained-scene catalog and refuses the hostile rows.

use super::*;

#[test]
fn typed_scene_neutral_catalog_matches_native_serde_contracts() {
    use semio_framework_ui_scene_pack::{to_bytes, write_varint, TAG_MAP, TAG_NONE, TAG_SEQ, TAG_SOME};
    fn packet(value: &serde_json::Value, bytes: &mut Vec<u8>) {
        match value {
            serde_json::Value::Null => bytes.push(TAG_NONE),
            serde_json::Value::Bool(value) => bytes.extend(to_bytes(value).unwrap()),
            serde_json::Value::Number(value) => bytes.extend(to_bytes(&value.as_f64().unwrap()).unwrap()),
            serde_json::Value::String(value) => bytes.extend(to_bytes(value).unwrap()),
            serde_json::Value::Array(values) => {
                bytes.push(TAG_SEQ);
                write_varint(bytes, values.len() as u64);
                for value in values {
                    packet(value, bytes);
                }
            }
            serde_json::Value::Object(values) => {
                if let Some(value) = values.get("$some") {
                    bytes.push(TAG_SOME);
                    packet(value, bytes);
                } else if let Some(value) = values.get("$u64") {
                    bytes.extend(to_bytes(&value.as_u64().unwrap()).unwrap());
                } else {
                    bytes.push(TAG_MAP);
                    write_varint(bytes, values.len() as u64);
                    for (key, value) in values {
                        bytes.extend(to_bytes(key).unwrap());
                        packet(value, bytes);
                    }
                }
            }
        }
    }
    fn check<T: SceneDoc>(schema: &str, bytes: &[u8]) -> bool {
        schema == T::SCHEMA && T::decode_pack(bytes).is_ok()
    }
    fn admitted(case: &serde_json::Value) -> bool {
        let mut bytes = Vec::new();
        packet(&case["value"], &mut bytes);
        let schema = case["schema"].as_str().unwrap();
        match case["kind"].as_str().unwrap() {
            "canvas-2d" => check::<Canvas2dScene>(schema, &bytes),
            "world-3d" => check::<World3dScene>(schema, &bytes),
            "node-graph" => check::<NodeGraphScene>(schema, &bytes),
            "text-editor" => check::<TextEditorScene>(schema, &bytes),
            "table" => check::<TableScene>(schema, &bytes),
            "paint-2d" => check::<Paint2dScene>(schema, &bytes),
            "virtual-file-system" => check::<VirtualFileSystemScene>(schema, &bytes),
            "tiled-map" => check::<TiledMapScene>(schema, &bytes),
            "board-2d" => check::<Board2dScene>(schema, &bytes),
            "icon-render" => check::<IconRenderScene>(schema, &bytes),
            "ink-canvas" => check::<InkCanvasScene>(schema, &bytes),
            "graph-timeline" => check::<GraphTimelineScene>(schema, &bytes),
            "block-list" => check::<BlockListScene>(schema, &bytes),
            "diff-view" => check::<DiffViewScene>(schema, &bytes),
            "event-feed" => check::<EventFeedScene>(schema, &bytes),
            _ => false,
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧵️retained/🧪️tests/🧫️fixtures/🧾️typed-scene/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 15);
    for case in fixture["cases"].as_array().unwrap() {
        assert!(admitted(case), "{case}");
    }
    for case in fixture["hostile"].as_array().unwrap() {
        assert!(!admitted(case), "{case}");
    }
}
