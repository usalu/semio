use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated() -> SemioGraphSnapshot {
    demo_graph_snapshot()
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioGraphSnapshot::default();
    let bytes = <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioGraphSnapshot::default();
    let text = <SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: decode(encode(snapshot)) is byte-for-byte structurally identical
/// on a fully-populated snapshot (nodes/edges/ports/properties non-empty), not just the default.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law() {
    let snap = populated();
    let bytes = <SemioGraphSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioGraphSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
    let text = <SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back_text = <SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back_text);
}

#[test]
fn declared_graph_json_preserves_raw_geometry_and_every_intrinsic_family() {
    let neutral: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️json/🔣️.json")).expect("closed neutral intrinsic cases");
    let mut raw: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧬️mutations/🏗️create-node/🔎️appends/📸️snapshot/➡️after/🔣️.json")).expect("committed full graph");
    raw["nodes"][0]["properties"] = neutral["properties"].clone();
    for bits in [0, 0x8000000000000000, 1, 0x000fffffffffffff, 0x0010000000000000, 0x7fefffffffffffff, 0x7ff0000000000000, 0xfff0000000000000, 0x7ff8000000000011, 0x7ff0000000000001, u64::MAX] {
        let word = serde_json::json!({"bits": format!("{bits:016x}")});
        for node in raw["nodes"].as_array_mut().expect("nodes") {
            node["position"]["x"] = word.clone();
            node["position"]["y"] = word.clone();
            node["width"] = word.clone();
            node["height"] = word.clone();
        }
        let decoded = decode_semio_graph_snapshot_json(&raw.to_string()).expect("declared graph input");
        for node in &decoded.nodes {
            assert_eq!(node.position.x.to_bits(), bits);
            assert_eq!(node.position.y.to_bits(), bits);
            assert_eq!(node.width.to_bits(), bits);
            assert_eq!(node.height.to_bits(), bits);
        }
        assert_eq!(decoded.nodes[0].properties.len(), 9);
        let encoded = encode_semio_graph_snapshot_json(&decoded).expect("declared graph output");
        assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).expect("independent declared output"), raw);
    }
    raw["nodes"][0]["width"] = serde_json::json!({"bits": "INVALID"});
    assert_eq!(decode_semio_graph_snapshot_json(&raw.to_string()).expect_err("invalid word refused").kind, semio_framework_value::ValueRefusalKind::InvalidValue);
    raw["nodes"][0]["width"] = serde_json::json!({"bits": "0000000000000000", "extra": true});
    assert_eq!(decode_semio_graph_snapshot_json(&raw.to_string()).expect_err("unknown word field refused").kind, semio_framework_value::ValueRefusalKind::InvalidValue);
}
