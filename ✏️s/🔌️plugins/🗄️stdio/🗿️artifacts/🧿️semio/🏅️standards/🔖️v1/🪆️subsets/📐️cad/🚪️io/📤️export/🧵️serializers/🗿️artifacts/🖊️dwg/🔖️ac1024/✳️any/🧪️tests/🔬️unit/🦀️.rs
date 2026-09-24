use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use crate::standards::v1::subsets::cad::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioCadFromDwg;
use crate::standards::v1::subsets::cad::schema::snapshot::{CadLayer, STDIO_SEMIOCAD_DOCUMENT_SCHEMA};
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn p(x: f64, y: f64) -> SemioPoint2 {
    SemioPoint2 { x, y }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn plan() -> SemioCadSnapshot {
    let record = |handle: &str, layer: &str, entity: CadEntity| CadEntityRecord { handle: handle.into(), layer: layer.into(), entity };
    SemioCadSnapshot {
        schema: STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "WALLS".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "NOTES".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true }],
        blocks: Vec::new(),
        entities: vec![
            record("1", "WALLS", CadEntity::Line { a: p(0.0, 0.0), b: p(4000.0, 0.0) }),
            record("2", "WALLS", CadEntity::Polyline { vertices: vec![p(0.0, 0.0), p(4000.0, 0.0), p(4000.0, 3000.0), p(0.0, 3000.0)], closed: true }),
            record("3", "WALLS", CadEntity::Circle { center: p(2000.0, 1500.0), radius: 250.0 }),
            record("4", "WALLS", CadEntity::Arc { center: p(1000.0, 1000.0), radius: 900.0, start_angle: 0.0, end_angle: 90.0 }),
            record("5", "NOTES", CadEntity::Text { position: p(100.0, 3100.0), height: 150.0, rotation: 0.0, content: "Living".into() }),
        ],
    }
}

/// 🔁️ Plan → DWG bytes (the codec's own AC1015 writer) → decode → plan keeps every entity, layer and
/// coordinate. No third-party DWG reader is available as a test dependency; the byte round trip
/// through the independent decoder is the evidence.
#[semio_framework_async_macros::async_test]
async fn a_plan_survives_real_dwg_bytes() {
    let dwg = SemioCadToDwg::serialize(&plan()).await.expect("export");
    let bytes = semio_s_artifact_stdio_dwg::dwg_to_bytes(&dwg.drawing.to_native().expect("native")).expect("dwg bytes");
    assert!(bytes.starts_with(b"AC10"), "a DWG version sentinel opens the file");
    let decoded = semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::snapshot::decode_dwg(&bytes).expect("decode");
    let back = SemioCadFromDwg::deserialize(&decoded).await.expect("import");
    let entities: Vec<(&str, &CadEntity)> = back.entities.iter().map(|record| (record.layer.as_str(), &record.entity)).collect();
    let expected = plan();
    let wanted: Vec<(&str, &CadEntity)> = expected.entities.iter().map(|record| (record.layer.as_str(), &record.entity)).collect();
    assert_eq!(entities.len(), wanted.len());
    for ((layer, entity), (want_layer, want)) in entities.iter().zip(&wanted) {
        assert_eq!(layer, want_layer);
        match (entity, want) {
            (CadEntity::Arc { start_angle, end_angle, .. }, CadEntity::Arc { start_angle: s, end_angle: e, .. }) => assert!((start_angle - s).abs() < 1e-9 && (end_angle - e).abs() < 1e-9, "degrees survive the radian trip"),
            _ => assert_eq!(entity, want),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn inserts_have_no_logical_entity_and_are_dropped() {
    let mut cad = plan();
    cad.entities.push(CadEntityRecord { handle: "9".into(), layer: "0".into(), entity: CadEntity::Insert { block_name: "DOOR".into(), insertion_point: p(0.0, 0.0), scale: p(1.0, 1.0), rotation: 0.0 } });
    assert_eq!(cad_to_dwg_drawing(&cad).entities.len(), 5);
}
