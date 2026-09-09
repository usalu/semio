use super::*;

#[test]
fn invalid_collection_targets_are_rejected_before_mutation() {
    let base = ObjSnapshot::default();
    let diff = ObjDiff { vertices: Some(ObjVerticesDiff { removed: vec![0], ..Default::default() }), ..Default::default() };
    let error = diff.apply(&base).expect_err("missing vertex target must be rejected");
    assert_eq!(error.code, "invalid-remove-index");
    assert_eq!(error.target, vec!["vertices", "0"]);
    assert_eq!(base, ObjSnapshot::default());
}

/// 🧪️ F6: `DiffCodec` round-trip laws for the hand-rolled `ObjDiff` text/binary grammar —
/// exercises every scalar, both tri-states (`mtllib` at the top level, `texcoords[1].w`
/// inside a modified item), and all three collection-triple kinds — index-keyed
/// (`vertices`/`texcoords`/`normals`/`faces`) AND name-keyed (`groups`/`objects`).await — via a real
/// `between()` result in both directions.
#[test]
fn diff_codec_text_binary_roundtrip_law() {
    let a = sweep_a();
    let b = sweep_b();
    let ab = <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&a, &b);
    assert_eq!(ab.mtllib, Some(None), "mtllib tri-state must exercise Some(None)");
    let td = ab.texcoords.as_ref().expect("texcoords diff populated");
    assert_eq!(td.modified[0].diff.w, Some(None), "texcoord w tri-state must exercise Some(None)");
    assert!(!ab.groups.as_ref().unwrap().removed.is_empty() && !ab.groups.as_ref().unwrap().modified.is_empty() && !ab.groups.as_ref().unwrap().added.is_empty(), "groups triple must exercise all 3 kinds");

    let cases = vec![ObjDiff::default(), ab, <ObjDiff as DiffAlgebra<ObjSnapshot>>::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = ObjDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = ObjDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
