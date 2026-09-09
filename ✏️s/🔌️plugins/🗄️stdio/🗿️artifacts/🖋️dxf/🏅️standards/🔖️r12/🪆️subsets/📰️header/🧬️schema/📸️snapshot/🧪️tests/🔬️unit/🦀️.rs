use super::*;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_dxf_text() -> String {
    concat!(
        "0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1009\n9\n$INSBASE\n10\n1\n20\n2\n30\n3\n0\nENDSEC\n",
        "0\nSECTION\n2\nTABLES\n",
        "0\nTABLE\n2\nLAYER\n70\n1\n0\nLAYER\n2\n0\n70\n0\n62\n7\n6\nCONTINUOUS\n0\nENDTAB\n",
        "0\nTABLE\n2\nSTYLE\n70\n1\n0\nSTYLE\n2\nSTANDARD\n70\n0\n3\ntxt\n0\nENDTAB\n",
        "0\nTABLE\n2\nLTYPE\n70\n1\n0\nLTYPE\n2\nCONTINUOUS\n70\n0\n3\nSolid\n0\nENDTAB\n",
        "0\nTABLE\n2\nVPORT\n70\n1\n0\nVPORT\n2\n*ACTIVE\n0\nENDTAB\n",
        "0\nENDSEC\n",
        "0\nSECTION\n2\nBLOCKS\n",
        "0\nBLOCK\n2\nMYBLOCK\n70\n0\n10\n0\n20\n0\n30\n0\n0\nLINE\n8\n0\n10\n0\n20\n0\n30\n0\n11\n1\n21\n1\n31\n0\n0\nENDBLK\n",
        "0\nENDSEC\n",
        "0\nSECTION\n2\nENTITIES\n",
        "0\nLINE\n8\n0\n10\n1\n20\n2\n30\n3\n11\n4\n21\n5\n31\n6\n",
        "0\nCIRCLE\n8\n0\n10\n10\n20\n20\n30\n30\n40\n5\n",
        "0\nARC\n8\n0\n10\n1\n20\n2\n30\n3\n40\n7\n50\n0\n51\n180\n",
        "0\nTEXT\n8\n0\n10\n1\n20\n1\n30\n0\n40\n2.5\n1\nHello\n",
        "0\nSOLID\n8\n0\n10\n0\n20\n0\n30\n0\n11\n1\n21\n0\n31\n0\n12\n1\n22\n1\n32\n0\n13\n0\n23\n1\n33\n0\n",
        "0\nINSERT\n8\n0\n2\nMYBLOCK\n10\n5\n20\n5\n30\n0\n41\n1\n42\n1\n43\n1\n50\n0\n",
        "0\nPOLYLINE\n8\n0\n66\n1\n70\n1\n0\nVERTEX\n8\n0\n10\n0\n20\n0\n30\n0\n0\nVERTEX\n8\n0\n10\n1\n20\n0\n30\n0\n0\nSEQEND\n",
        "0\n3DFACE\n8\n0\n10\n0\n20\n0\n30\n0\n",
        "0\nENDSEC\n0\nEOF\n",
    )
    .to_string()
}

#[semio_framework_async_macros::async_test]
async fn parses_every_section_and_entity_kind() {
    let snap = parse_dxf_document(&sample_dxf_text()).expect("parse");
    assert_eq!(snap.header_vars.len(), 2);
    assert_eq!(snap.header_vars[0].name, "$ACADVER");
    assert_eq!(snap.header_vars[1].name, "$INSBASE");
    assert_eq!(snap.header_vars[1].value, DxfValue::Point { value: [1.0, 2.0, 3.0] });

    assert_eq!(snap.tables.layers.len(), 1);
    assert_eq!(snap.tables.layers[0].name, "0");
    assert_eq!(snap.tables.styles.len(), 1);
    assert_eq!(snap.tables.linetypes.len(), 1);
    assert_eq!(snap.other_tables.len(), 1, "VPORT retained raw");
    assert_eq!(snap.other_tables[0].name, "VPORT");

    assert_eq!(snap.blocks.len(), 1);
    assert_eq!(snap.blocks[0].name, "MYBLOCK");
    assert_eq!(snap.blocks[0].entities.len(), 1);
    assert!(matches!(snap.blocks[0].entities[0], DxfEntity::Line { .. }));

    assert_eq!(snap.entities.len(), 8);
    assert!(matches!(snap.entities[0], DxfEntity::Line { .. }));
    assert!(matches!(snap.entities[1], DxfEntity::Circle { .. }));
    assert!(matches!(snap.entities[2], DxfEntity::Arc { .. }));
    assert!(matches!(snap.entities[3], DxfEntity::Text { .. }));
    assert!(matches!(snap.entities[4], DxfEntity::Solid { .. }));
    assert!(matches!(snap.entities[5], DxfEntity::Insert { .. }));
    match &snap.entities[6] {
        DxfEntity::Polyline { vertices, closed, .. } => {
            assert_eq!(vertices.len(), 2);
            assert!(*closed);
        }
        other => panic!("expected Polyline, got {other:?}"),
    }
    match &snap.entities[7] {
        DxfEntity::Other { kind, .. } => assert_eq!(kind, "3DFACE"),
        other => panic!("expected Other(3DFACE), got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn codec_retention_is_a_fixed_point_from_generation_two() {
    let snap1 = parse_dxf_document(&sample_dxf_text()).expect("parse");
    let text2 = print_dxf_document(&snap1);
    let snap2 = parse_dxf_document(&text2).expect("re-parse");
    assert_eq!(snap1, snap2, "decode(encode(decode(text))) must be a fixed point");
}

#[semio_framework_async_macros::async_test]
async fn snapshot_parse_dsl_print_dsl_round_trips() {
    let snap = parse_dxf_document(&sample_dxf_text()).expect("parse");
    let printed = store::ArtifactDsl::print_dsl(&snap);
    let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("parse");
    assert_eq!(parsed, snap);

    let packed = store::ArtifactPack::encode_pack(&snap);
    let unpacked = <DxfSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("unpack");
    assert_eq!(unpacked, snap);
}
