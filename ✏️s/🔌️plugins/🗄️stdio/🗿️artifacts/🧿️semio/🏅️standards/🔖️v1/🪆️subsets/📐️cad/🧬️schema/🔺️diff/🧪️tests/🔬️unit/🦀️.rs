
use super::*;
use protocol::DiffCodec;

//#region Fixtures
/// 🧪️ Every field/collection mutable, incl. a nested `blocks[].entities` add/remove/modify and
/// an `entities` entry whose `entity` variant itself changes kind (Circle -> Ellipse).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: crate::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }, CadLayer { name: "layer-remove".into(), color_index: 2, line_type: "DASHED".into(), visible: false }],
        blocks: vec![
            CadBlock {
                name: "keep-block".into(),
                base_point: SemioPoint2 { x: 0.0, y: 0.0 },
                entities: vec![
                    CadEntityRecord { handle: "be-keep".into(), layer: "keep".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 1.0, y: 1.0 } } },
                    CadEntityRecord { handle: "be-remove".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 2.0, y: 2.0 }, radius: 3.0 } },
                ],
            },
            CadBlock { name: "block-remove".into(), base_point: SemioPoint2 { x: 5.0, y: 5.0 }, entities: Vec::new() },
        ],
        entities: vec![
            CadEntityRecord { handle: "e-keep".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } },
            CadEntityRecord { handle: "e-remove".into(), layer: "layer-remove".into(), entity: CadEntity::Line { a: SemioPoint2 { x: 0.0, y: 0.0 }, b: SemioPoint2 { x: 9.0, y: 9.0 } } },
        ],
    }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioCadSnapshot {
    SemioCadSnapshot {
        schema: crate::standards::v1::subsets::cad::schema::snapshot::STDIO_SEMIOCAD_DOCUMENT_SCHEMA.into(),
        layers: vec![CadLayer { name: "keep".into(), color_index: 9, line_type: "DASHDOT".into(), visible: false }, CadLayer { name: "layer-add".into(), color_index: 4, line_type: "HIDDEN".into(), visible: true }],
        blocks: vec![
            CadBlock {
                name: "keep-block".into(),
                base_point: SemioPoint2 { x: 10.0, y: 10.0 },
                entities: vec![
                    CadEntityRecord { handle: "be-keep".into(), layer: "layer-add".into(), entity: CadEntity::Arc { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0, start_angle: 0.0, end_angle: 90.0 } },
                    CadEntityRecord { handle: "be-add".into(), layer: "keep".into(), entity: CadEntity::Text { position: SemioPoint2 { x: 1.0, y: 1.0 }, height: 2.0, rotation: 0.0, content: "hi".into() } },
                ],
            },
            CadBlock { name: "block-add".into(), base_point: SemioPoint2 { x: 7.0, y: 7.0 }, entities: Vec::new() },
        ],
        entities: vec![
            CadEntityRecord { handle: "e-keep".into(), layer: "layer-add".into(), entity: CadEntity::Ellipse { center: SemioPoint2 { x: 0.0, y: 0.0 }, major_axis_end: SemioPoint2 { x: 1.0, y: 0.0 }, ratio: 0.5, start_param: 0.0, end_param: 6.28 } },
            CadEntityRecord { handle: "e-add".into(), layer: "keep".into(), entity: CadEntity::Insert { block_name: "keep-block".into(), insertion_point: SemioPoint2 { x: 0.0, y: 0.0 }, scale: SemioPoint2 { x: 1.0, y: 1.0 }, rotation: 0.0 } },
        ],
    }
}
//#endregion Fixtures

//#region 🧪️Law6_FieldSweep
/// ⚖️ Law 6 — `field_sweep`: `sweep_a`/`sweep_b` differ in every mutable field, incl. per
/// collection one removed/one modified-in-every-field/one added, at BOTH the top level and the
/// nested `blocks[].entities` level.
#[semio_framework_async_macros::async_test]
async fn field_sweep() {
    let a = sweep_a();
    let b = sweep_b();

    let forward = SemioCadDiff::between(&a, &b);
    assert_eq!(forward.apply(&a).expect("apply must succeed for a well-formed fixture"), b, "between(a,b).apply(a) must equal b");
    let backward = SemioCadDiff::between(&b, &a);
    assert_eq!(backward.apply(&b).expect("apply must succeed for a well-formed fixture"), a, "between(b,a).apply(b) must equal a");
    assert!(SemioCadDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

    let layers_diff = forward.layers.as_ref().expect("layers diff present");
    assert!(!layers_diff.removed.is_empty(), "layers.removed not swept");
    assert!(!layers_diff.added.is_empty(), "layers.added not swept");
    let keep_layer_diff = &layers_diff.modified.iter().find(|m| m.key == "keep").expect("keep layer modified").diff;
    assert!(keep_layer_diff.color_index.is_some(), "layer.color_index not swept");
    assert!(keep_layer_diff.line_type.is_some(), "layer.line_type not swept");
    assert!(keep_layer_diff.visible.is_some(), "layer.visible not swept");

    let blocks_diff = forward.blocks.as_ref().expect("blocks diff present");
    assert!(!blocks_diff.removed.is_empty(), "blocks.removed not swept");
    assert!(!blocks_diff.added.is_empty(), "blocks.added not swept");
    let keep_block_diff = &blocks_diff.modified.iter().find(|m| m.key == "keep-block").expect("keep-block modified").diff;
    assert!(keep_block_diff.base_point.is_some(), "block.base_point not swept");
    let nested_entities_diff = keep_block_diff.entities.as_ref().expect("nested block entities diff present");
    assert!(!nested_entities_diff.removed.is_empty(), "block.entities.removed not swept");
    assert!(!nested_entities_diff.added.is_empty(), "block.entities.added not swept");
    let be_keep_diff = &nested_entities_diff.modified.iter().find(|m| m.key == "be-keep").expect("be-keep modified").diff;
    assert!(be_keep_diff.layer.is_some(), "block entity.layer not swept");
    assert!(be_keep_diff.entity.is_some(), "block entity.entity not swept");

    let entities_diff = forward.entities.as_ref().expect("entities diff present");
    assert!(!entities_diff.removed.is_empty(), "entities.removed not swept");
    assert!(!entities_diff.added.is_empty(), "entities.added not swept");
    let e_keep_diff = &entities_diff.modified.iter().find(|m| m.key == "e-keep").expect("e-keep modified").diff;
    assert!(e_keep_diff.layer.is_some(), "entity.layer not swept");
    assert!(e_keep_diff.entity.is_some(), "entity.entity not swept");
}
//#endregion

//#region 🧪️Law3_AbsorbLaw
/// ⚖️ Law 3 — `absorb_law`: curated op list (Insert+Remove-before, Add+SetField-patches-into-
/// added, Modify+Remove-annihilates) plus associativity — same canonical cases as bcf's.
#[semio_framework_async_macros::async_test]
async fn absorb_law() {
    let base = sweep_a();

    // Insert+Remove-before: add a layer, then remove an unrelated layer — independent, net
    // effect must match sequential application.
    let d1 = SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CadLayer { name: "fresh".into(), color_index: 3, line_type: "CONTINUOUS".into(), visible: true }] }), blocks: None, entities: None };
    let d2 = SemioCadDiff { layers: Some(CadLayersDiff { removed: vec!["layer-remove".into()], modified: Vec::new(), added: Vec::new() }), blocks: None, entities: None };
    assert_absorb_matches_sequential(&base, d1, d2);

    // Add+SetField: insert an entity, then immediately edit that SAME entity's layer -- must
    // patch into the carried `added` payload, not become a dangling `modified` entry.
    let new_entity = CadEntityRecord { handle: "e-fresh".into(), layer: "keep".into(), entity: CadEntity::Circle { center: SemioPoint2 { x: 0.0, y: 0.0 }, radius: 1.0 } };
    let d1 = SemioCadDiff { layers: None, blocks: None, entities: Some(CadEntitiesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![new_entity] }) };
    let d2 = wrap_entity_diff("e-fresh", CadEntityRecordDiff { layer: Some("layer-remove".into()), entity: None });
    let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
    let entities_diff = absorbed.entities.as_ref().expect("entities diff");
    assert!(entities_diff.modified.is_empty(), "edit-after-insert must patch into added, not appear as modified");
    let added_entity = entities_diff.added.iter().find(|e| e.handle == "e-fresh").expect("e-fresh still in added");
    assert_eq!(added_entity.layer, "layer-remove");

    // Modify+Remove: edit a block's base_point, then remove that same block -- must annihilate
    // to a plain removal, not a dangling modify+remove pair.
    let d1 = wrap_block_diff("keep-block", CadBlockDiff { base_point: Some(SemioPoint2 { x: 99.0, y: 99.0 }), entities: None });
    let d2 = SemioCadDiff { layers: None, blocks: Some(CadBlocksDiff { removed: vec!["keep-block".into()], modified: Vec::new(), added: Vec::new() }), entities: None };
    let absorbed = assert_absorb_matches_sequential(&base, d1, d2);
    let blocks_diff = absorbed.blocks.as_ref().expect("blocks diff");
    assert_eq!(blocks_diff.removed, vec!["keep-block".to_string()]);
    assert!(blocks_diff.modified.is_empty());

    // Associativity: absorb(absorb(d1,d2),d3) == absorb(d1,absorb(d2,d3)).
    let d1 = wrap_layer_diff("keep", CadLayerDiff { color_index: Some(42), line_type: None, visible: None });
    let mid1 = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
    let d2 = SemioCadDiff { layers: Some(CadLayersDiff { removed: Vec::new(), modified: Vec::new(), added: vec![CadLayer { name: "assoc".into(), color_index: 1, line_type: "CONTINUOUS".into(), visible: true }] }), blocks: None, entities: None };
    let _mid2 = d2.apply(&mid1).expect("apply must succeed for a well-formed fixture");
    let d3 = wrap_layer_diff("assoc", CadLayerDiff { color_index: None, line_type: Some("DASHED".into()), visible: None });

    let mut left = d1.clone();
    MutationDiff::absorb(&mut left, d2.clone());
    MutationDiff::absorb(&mut left, d3.clone());

    let mut d2_d3 = d2;
    MutationDiff::absorb(&mut d2_d3, d3);
    let mut right = d1;
    MutationDiff::absorb(&mut right, d2_d3);

    assert_eq!(left.apply(&base).expect("apply must succeed for a well-formed fixture"), right.apply(&base).expect("apply must succeed for a well-formed fixture"), "absorb must be associative");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_absorb_matches_sequential(base: &SemioCadSnapshot, d1: SemioCadDiff, d2: SemioCadDiff) -> SemioCadDiff {
    let sequential = d2.apply(&d1.apply(base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    let mut absorbed = d1;
    MutationDiff::absorb(&mut absorbed, d2);
    assert_eq!(absorbed.apply(base).expect("apply must succeed for a well-formed fixture"), sequential, "absorb(d1,d2).apply(base) must equal sequential application");
    absorbed
}
//#endregion

//#region 🧪️Law4_BetweenRoundtripLaw
/// ⚖️ Law 4 — `between_roundtrip_law`: `between(a,b).apply(a) == b` on fixtures.
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let d = SemioCadDiff::between(&a, &b);
    assert_eq!(d.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let d_back = SemioCadDiff::between(&b, &a);
    assert_eq!(d_back.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(SemioCadDiff::between(&a, &a).is_empty());
}
//#endregion

//#region 🧪️Law8_DiffCodecTextBinaryRoundtripLaw
/// ⚖️ Law 8 — `diff_codec_text_binary_roundtrip_law`: hand-rolled `DiffCodec` text/binary
/// round-trip, exercising every collection triple (top-level AND the nested
/// `blocks[].entities`) plus all 9 `CadEntity` variants across `between()` results.
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let mut cases = vec![SemioCadDiff::default(), SemioCadDiff::between(&a, &b), SemioCadDiff::between(&b, &a), SemioCadDiff::between(&a, &a)];
    // Exercise every remaining CadEntity variant not already covered by sweep_a/sweep_b.
    cases.push(wrap_entity_diff("h", CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Polyline { vertices: vec![SemioPoint2 { x: 0.0, y: 0.0 }, SemioPoint2 { x: 1.0, y: 1.0 }], closed: true }) }));
    cases.push(wrap_entity_diff(
        "h",
        CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Solid { p1: SemioPoint2 { x: 0.0, y: 0.0 }, p2: SemioPoint2 { x: 1.0, y: 0.0 }, p3: SemioPoint2 { x: 1.0, y: 1.0 }, p4: SemioPoint2 { x: 0.0, y: 1.0 } }) },
    ));
    cases.push(wrap_entity_diff("h", CadEntityRecordDiff { layer: None, entity: Some(CadEntity::Dimension { def_point: SemioPoint2 { x: 0.0, y: 0.0 }, text_position: SemioPoint2 { x: 1.0, y: 1.0 }, measurement: 4.2, text: "4.2m".into() }) }));

    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioCadDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioCadDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion
