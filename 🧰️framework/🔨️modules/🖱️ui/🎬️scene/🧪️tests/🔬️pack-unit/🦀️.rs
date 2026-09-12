
use super::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Nested {
    label: String,
    weight: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Sample {
    name: String,
    count: u32,
    active: bool,
    tag: Option<String>,
    items: Vec<Nested>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sparse {
    first_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_value: Option<String>,
    last_value: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Choice {
    Unit,
    Newtype(String),
    Record { value: String },
}

#[test]
fn round_trips_struct_with_option_and_seq() {
    let value = Sample { name: "hello".into(), count: 42, active: true, tag: Some("v1".into()), items: vec![Nested { label: "a".into(), weight: 1.5 }, Nested { label: "b".into(), weight: -2.25 }] };
    let bytes = to_bytes(&value).expect("encode");
    let back: Sample = from_bytes(&bytes).expect("decode");
    assert_eq!(value, back);
}

#[test]
fn round_trips_none_option() {
    let value = Sample { name: String::new(), count: 0, active: false, tag: None, items: vec![] };
    let bytes = to_bytes(&value).expect("encode");
    let back: Sample = from_bytes(&bytes).expect("decode");
    assert_eq!(value, back);
}

#[test]
fn self_describing_struct_preserves_a_field_after_a_skipped_middle_field() {
    let value = Sparse { first_value: "first".into(), middle_value: None, last_value: "last".into() };
    let bytes = to_bytes(&value).expect("encode");
    let back: Sparse = from_bytes(&bytes).expect("decode");
    assert_eq!(value, back);
    assert!(bytes.windows("lastValue".len()).any(|window| window == b"lastValue"));
}

#[test]
fn self_describing_enum_round_trips_every_payload_shape() {
    for value in [Choice::Unit, Choice::Newtype("newtype".into()), Choice::Record { value: "record".into() }] {
        let bytes = to_bytes(&value).expect("encode");
        let back: Choice = from_bytes(&bytes).expect("decode");
        assert_eq!(value, back);
    }
}

#[test]
fn truncated_input_errs_not_panics() {
    let result: Result<Sample, PackError> = from_bytes(&[TAG_SEQ]);
    assert!(result.is_err());
}

#[test]
fn viewport_projection_pack_round_trips_every_shared_mode_orientation_pair() {
    use semio_framework_ui_viewport::{Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionPreferences, Viewport3dProjectionSpec};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🪟️viewport/🧪️tests/🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    let preferences: Viewport3dProjectionPreferences = serde_json::from_value(fixture["defaultPreferences"].clone()).unwrap();
    let bytes = to_bytes(&preferences).unwrap();
    assert_eq!(from_bytes::<Viewport3dProjectionPreferences>(&bytes).unwrap(), preferences);
    let mut combinations = 0;
    for mode in fixture["modes"].as_array().unwrap() {
        let mode_value: Viewport3dProjectionMode = serde_json::from_value(mode.clone()).unwrap();
        let mode_bytes = to_bytes(&mode_value).unwrap();
        from_bytes::<Viewport3dProjectionMode>(&mode_bytes).unwrap_or_else(|error| panic!("mode={mode}: {error:?}"));
        for orientation in fixture["orientations"].as_array().unwrap() {
            let orientation_value: Viewport3dProjectionOrientation = serde_json::from_value(orientation.clone()).unwrap();
            let orientation_bytes = to_bytes(&orientation_value).unwrap();
            from_bytes::<Viewport3dProjectionOrientation>(&orientation_bytes).unwrap_or_else(|error| panic!("orientation={orientation}: {error:?}"));
            let spec: Viewport3dProjectionSpec = serde_json::from_value(serde_json::json!({ "mode": mode, "orientation": orientation })).unwrap();
            let bytes = to_bytes(&spec).unwrap();
            let decoded = from_bytes::<Viewport3dProjectionSpec>(&bytes).unwrap_or_else(|error| panic!("mode={mode} orientation={orientation}: {error:?}"));
            assert_eq!(decoded, spec);
            combinations += 1;
        }
    }
    eprintln!("[DEBUG] Shared viewport projection Pack codec round-tripped the full preference bank and {combinations} mode-orientation pairs including outside-control active values");
}

#[test]
fn viewport_projection_pack_outside_control_fov_matches_native_projection_math() {
    use semio_framework_ui_viewport::{Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};

    let spec = Viewport3dProjectionSpec { mode: Viewport3dProjectionMode::ThreePoint { fov: 130.0 }, orientation: Viewport3dProjectionOrientation::Free {} };
    let decoded: Viewport3dProjectionSpec = from_bytes(&to_bytes(&spec).unwrap()).unwrap();
    let mut camera = crate::math::Camera3d::default();
    let Viewport3dProjectionMode::ThreePoint { fov } = decoded.mode else { panic!("unexpected projection mode") };
    camera.fov_y = (fov as f32).to_radians();
    let matrix = camera.view_proj(16.0 / 9.0);
    assert!(matrix.cols.into_iter().flatten().all(f32::is_finite));
    eprintln!("[DEBUG] Shared viewport projection Pack preserved FOV 130 and native projection math produced a finite matrix");
}

#[test]
fn viewport_projection_pack_rejects_nonfinite_active_values() {
    use semio_framework_ui_viewport::{Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};

    let invalid = Viewport3dProjectionSpec { mode: Viewport3dProjectionMode::ThreePoint { fov: f64::NAN }, orientation: Viewport3dProjectionOrientation::Free {} };
    let bytes = to_bytes(&invalid).unwrap();
    assert!(from_bytes::<Viewport3dProjectionSpec>(&bytes).is_err());
    eprintln!("[DEBUG] Shared viewport projection Pack decode rejected a nonfinite active lens");
}

//#region 🎬️RetainedSceneOracle
#[test]
fn owned_scene_neutral_vectors_match_native_serde_packet() {
    #[derive(Serialize)]
    enum FixtureVariant {
        Idle,
        Scale(u64),
    }
    #[derive(Serialize)]
    struct NestedMap {
        a: (bool, u64),
        b: TextMap,
    }
    #[derive(Serialize)]
    struct TextMap {
        x: &'static str,
    }
    #[derive(Serialize)]
    struct PrototypeMap {
        #[serde(rename = "__proto__")]
        value: bool,
    }
    #[derive(Serialize)]
    struct CollisionMap {
        costarring: bool,
        liquid: bool,
    }
    #[derive(Serialize)]
    struct EmptyMap {}
    struct Bytes<'a>(&'a [u8]);
    impl Serialize for Bytes<'_> {
        fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            serializer.serialize_bytes(self.0)
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧵️retained/🧫️fixtures/🎬️owned-scene/🔣️.json")).unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 19);
    for case in cases {
        let name = case["name"].as_str().unwrap();
        let expected: Vec<u8> = case["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|digits| u8::from_str_radix(std::str::from_utf8(digits).unwrap(), 16).unwrap()).collect();
        let actual = match name {
            "unit" => to_bytes(&()).unwrap(),
            "false" => to_bytes(&false).unwrap(),
            "true" => to_bytes(&true).unwrap(),
            "unsigned" => to_bytes(&300u64).unwrap(),
            "negative" => to_bytes(&-3i64).unwrap(),
            "double" => to_bytes(&1.5f64).unwrap(),
            "unicode" | "bom-preserved" => to_bytes(case["value"].as_str().unwrap()).unwrap(),
            "bytes" => to_bytes(&Bytes(&[0, 128, 255])).unwrap(),
            "none" => to_bytes(&Option::<bool>::None).unwrap(),
            "some" => to_bytes(&Some(true)).unwrap(),
            "char" => to_bytes(&'🧹').unwrap(),
            "unit-variant" => to_bytes(&FixtureVariant::Idle).unwrap(),
            "data-variant" => to_bytes(&FixtureVariant::Scale(2)).unwrap(),
            "sequence" => to_bytes(&case["value"]).unwrap(),
            "nested-map" => to_bytes(&NestedMap { a: (false, 7), b: TextMap { x: "雪" } }).unwrap(),
            "prototype-key" => to_bytes(&PrototypeMap { value: true }).unwrap(),
            "empty-containers" => to_bytes(&(Vec::<u8>::new(), EmptyMap {}, "")).unwrap(),
            "fnv-collision-exact-keys" => to_bytes(&CollisionMap { costarring: true, liquid: false }).unwrap(),
            _ => panic!("Unmatched neutral scene case: {name}"),
        };
        assert_eq!(actual, expected, "{name}");
    }
}
#[test]
fn typed_scene_neutral_catalog_matches_native_serde_contracts() {
    use crate::*;
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧵️retained/🧫️fixtures/🧾️typed-scene/🔣️.json")).unwrap();
    assert_eq!(fixture["cases"].as_array().unwrap().len(), 15);
    for case in fixture["cases"].as_array().unwrap() {
        assert!(admitted(case), "{case}");
    }
    for case in fixture["hostile"].as_array().unwrap() {
        assert!(!admitted(case), "{case}");
    }
}
#[test]
fn scene_pack_numeric_widths_do_not_wrap() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧵️retained/🧫️fixtures/🔢️scene-numeric/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let bytes: Vec<u8> = case["hex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|digits| u8::from_str_radix(std::str::from_utf8(digits).unwrap(), 16).unwrap()).collect();
        let admitted = match case["type"].as_str().unwrap() {
            "u8" => from_bytes::<u8>(&bytes).is_ok(),
            "u32" => from_bytes::<u32>(&bytes).is_ok(),
            "u64" => from_bytes::<u64>(&bytes).is_ok(),
            "i64" => from_bytes::<i64>(&bytes).is_ok(),
            "usize" => from_bytes::<usize>(&bytes).is_ok(),
            "f64" => {
                let decoded = from_bytes::<f64>(&bytes);
                if let Ok(value) = decoded {
                    assert_eq!(value.is_finite(), case["finite"].as_bool().unwrap());
                    assert_eq!(value.to_bits().to_le_bytes(), bytes[1..]);
                }
                decoded.is_ok()
            }
            _ => panic!("Unknown numeric fixture"),
        };
        let expected = case[if usize::BITS == 32 { "native32" } else { "native64" }].as_bool().unwrap();
        assert_eq!(admitted, expected, "{}", case["name"]);
    }
}
//#endregion 🎬️RetainedSceneOracle
