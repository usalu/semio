
use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dwg() -> DwgSnapshot {
    let bytes = semio_s_artifact_stdio_dwg::dwg_to_bytes(&semio_s_artifact_stdio_dwg::DwgDrawing::default()).expect("encode sample");
    semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::snapshot::decode_dwg(&bytes).expect("decode sample")
}

#[semio_framework_async_macros::async_test]
async fn produces_empty_but_valid_cad_snapshot() {
    let cad = semio_framework_plugin::resolve_ready(SemioCadFromDwg::deserialize(&sample_dwg())).expect("deserialize");
    assert!(cad.layers.is_empty());
    assert!(cad.blocks.is_empty());
    assert!(cad.entities.is_empty());
    assert_eq!(cad.schema, STDIO_SEMIOCAD_DOCUMENT_SCHEMA);
}

#[semio_framework_async_macros::async_test]
async fn rejects_missing_version() {
    let bad = DwgSnapshot { version: String::new(), ..DwgSnapshot::default() };
    assert!(semio_framework_plugin::resolve_ready(SemioCadFromDwg::deserialize(&bad)).is_err());
}
