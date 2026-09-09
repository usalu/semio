use super::*;

fn assert_geometry_contract<T>(input: &serde_json::Value, valid: bool)
where
    T: dsl::FromValue + dsl::ToValue + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let text = serde_json::to_string(input).expect("fixture JSON");
    let native = dsl::json::from_json_str::<T>(&text);
    let oracle = serde_json::from_str::<T>(&text);
    assert_eq!(native.is_ok(), valid, "native admission: {input}");
    assert_eq!(oracle.is_ok(), valid, "independent admission: {input}");
    if valid {
        let native = native.expect("valid native geometry");
        assert_eq!(native, oracle.expect("valid oracle geometry"));
        let encoded = dsl::json::to_json_string(&native);
        assert_eq!(native, serde_json::from_str::<T>(&encoded).expect("independent geometry decode"));
    }
}

#[semio_framework_async_macros::async_test]
async fn stdio_document_contract_shared_geometry_matches_independent_oracle() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("neutral geometry vectors");
    let cases = fixtures["cases"].as_array().expect("geometry cases");
    for case in cases {
        let input = &case["input"];
        let valid = case["valid"].as_bool().expect("expected admission");
        match case["type"].as_str().expect("geometry type") {
            "SemioPoint3" => assert_geometry_contract::<SemioPoint3>(input, valid),
            "SemioPoint2" => assert_geometry_contract::<SemioPoint2>(input, valid),
            "SemioUv" => assert_geometry_contract::<SemioUv>(input, valid),
            "SemioRgba" => assert_geometry_contract::<SemioRgba>(input, valid),
            "SemioQuaternion" => assert_geometry_contract::<SemioQuaternion>(input, valid),
            "SemioTransform" => assert_geometry_contract::<SemioTransform>(input, valid),
            other => panic!("unknown geometry fixture type {other}"),
        }
    }
    eprintln!("[DEBUG] Shared Semio geometry matched {} neutral vectors and independent Serde admission", cases.len());
}

#[semio_framework_async_macros::async_test]
async fn identity_transform_round_trips_through_json() {
    let t = SemioTransform::identity();
    let json = serde_json::to_string(&t).expect("serialize");
    let back: SemioTransform = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(t, back);
    assert_eq!(back.rotation.w, 1.0);
    assert_eq!(back.scale, SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 });
}

#[semio_framework_async_macros::async_test]
async fn rgba_and_uv_default_to_zero() {
    assert_eq!(SemioRgba::default(), SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
    assert_eq!(SemioUv::default(), SemioUv { u: 0.0, v: 0.0 });
}

#[semio_framework_async_macros::async_test]
async fn point3_and_point2_are_plain_structs_not_tuples() {
    // 🧪️ Structural proof against the f6 §4.3 bare-tuple `DslField` gap: field ACCESS by
    // name, not `.0`/`.1` positional tuple indexing.
    let p3 = SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = SemioPoint2 { x: p3.x, y: p3.y };
    assert_eq!(p2, SemioPoint2 { x: 1.0, y: 2.0 });
}
