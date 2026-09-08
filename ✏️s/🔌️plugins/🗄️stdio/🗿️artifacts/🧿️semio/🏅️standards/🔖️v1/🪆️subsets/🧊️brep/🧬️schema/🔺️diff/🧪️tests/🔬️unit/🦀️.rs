
use super::*;

//#region 🔖️Fixtures
/// 🧱️ Every collection carries: one "keep" item touched in EVERY sub-field, one item present
/// only in `sweep_a` (removed), and (in `sweep_b`) one item present only there (added).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_a() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![BrepVertex { tol: 1e-7, id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }, BrepVertex { tol: 1e-7, id: "v-removed".into(), point: SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 } }];
    s.edges = vec![
        BrepEdge { tol: 1e-7, id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } },
        BrepEdge { tol: 1e-7, id: "e-removed".into(), start_vertex: "v-removed".into(), end_vertex: "v-removed".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 } },
    ];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }, BrepLoop { id: "l-removed".into(), edges: vec![] }];
    s.faces = vec![
        BrepFace { tol: 1e-7, id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true },
        BrepFace { tol: 1e-7, id: "f-removed".into(), outer_loop: "l-removed".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 1.0 }, orientation: true },
    ];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }, BrepShell { id: "s-removed".into(), faces: vec![] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }, BrepSolid { id: "so-removed".into(), shells: vec![] }];
    s
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sweep_b() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![BrepVertex { tol: 1e-7, id: "v1".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }, BrepVertex { tol: 1e-7, id: "v-added".into(), point: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } }];
    s.edges = vec![
        BrepEdge { tol: 1e-7, id: "e1".into(), start_vertex: "v-added".into(), end_vertex: "v-added".into(), curve: BrepCurve::Circle { center: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }, axis: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 2.0 } },
        BrepEdge { tol: 1e-7, id: "e-added".into(), start_vertex: "v-added".into(), end_vertex: "v-added".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 } } },
    ];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] }, BrepLoop { id: "l-added".into(), edges: vec![] }];
    s.faces = vec![
        BrepFace {
            tol: 1e-7,
            id: "f1".into(),
            outer_loop: "l1-alt".into(),
            inner_loops: vec!["l-added".into()],
            surface: BrepSurface::Cylinder { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 5.0 },
            orientation: false,
        },
        BrepFace {
            tol: 1e-7,
            id: "f-added".into(),
            outer_loop: "l1".into(),
            inner_loops: vec![],
            surface: BrepSurface::Torus { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, major_radius: 3.0, minor_radius: 1.0 },
            orientation: true,
        },
    ];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: false }] }, BrepShell { id: "s-added".into(), faces: vec![] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: true }] }, BrepSolid { id: "so-added".into(), shells: vec![] }];
    s
}
//#endregion 🔖️Fixtures

//#region 🔖️between_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn between_roundtrip_law_and_field_sweep_both_directions() {
    let (a, b) = (sweep_a(), sweep_b());
    let d_ab = SemioBrepDiff::between(&a, &b);
    assert_eq!(d_ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
    let d_ba = SemioBrepDiff::between(&b, &a);
    assert_eq!(d_ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
    assert!(SemioBrepDiff::between(&a, &a).is_empty());
}
//#endregion 🔖️between_roundtrip_law

//#region 🔖️field_sweep
#[semio_framework_async_macros::async_test]
async fn field_sweep_every_field_present_in_diff() {
    let (a, b) = (sweep_a(), sweep_b());
    let d = SemioBrepDiff::between(&a, &b);

    let vertices = d.vertices.as_ref().expect("vertices diff present");
    assert_eq!(vertices.removed, vec!["v-removed".to_string()]);
    assert_eq!(vertices.added.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), vec!["v-added".to_string()]);
    assert!(vertices.modified.iter().any(|m| m.key == "v1" && m.diff.point.is_some()));

    let edges = d.edges.as_ref().expect("edges diff present");
    assert_eq!(edges.removed, vec!["e-removed".to_string()]);
    assert_eq!(edges.added.iter().map(|e| e.id.clone()).collect::<Vec<_>>(), vec!["e-added".to_string()]);
    let e1 = edges.modified.iter().find(|m| m.key == "e1").expect("e1 modified");
    assert!(e1.diff.start_vertex.is_some() && e1.diff.end_vertex.is_some() && e1.diff.curve.is_some());

    let loops = d.loops.as_ref().expect("loops diff present");
    assert_eq!(loops.removed, vec!["l-removed".to_string()]);
    assert_eq!(loops.added.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), vec!["l-added".to_string()]);
    assert!(loops.modified.iter().any(|m| m.key == "l1" && m.diff.edges.is_some()));

    let faces = d.faces.as_ref().expect("faces diff present");
    assert_eq!(faces.removed, vec!["f-removed".to_string()]);
    assert_eq!(faces.added.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), vec!["f-added".to_string()]);
    let f1 = faces.modified.iter().find(|m| m.key == "f1").expect("f1 modified");
    assert!(f1.diff.outer_loop.is_some() && f1.diff.inner_loops.is_some() && f1.diff.surface.is_some() && f1.diff.orientation.is_some());

    let shells = d.shells.as_ref().expect("shells diff present");
    assert_eq!(shells.removed, vec!["s-removed".to_string()]);
    assert_eq!(shells.added.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["s-added".to_string()]);
    assert!(shells.modified.iter().any(|m| m.key == "s1" && m.diff.faces.is_some()));

    let solids = d.solids.as_ref().expect("solids diff present");
    assert_eq!(solids.removed, vec!["so-removed".to_string()]);
    assert_eq!(solids.added.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["so-added".to_string()]);
    assert!(solids.modified.iter().any(|m| m.key == "so1" && m.diff.shells.is_some()));
}
//#endregion 🔖️field_sweep

//#region 🔖️inverse_law
#[semio_framework_async_macros::async_test]
async fn inverse_law_diff_level_round_trips() {
    let (a, b) = (sweep_a(), sweep_b());
    let d = SemioBrepDiff::between(&a, &b);
    let inv = d.inverse(&a);
    assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
}
//#endregion 🔖️inverse_law

//#region 🔖️absorb_law
#[semio_framework_async_macros::async_test]
async fn absorb_law_add_then_remove_of_same_added_key_cancels() {
    let base = SemioBrepSnapshot::default();
    let mut d1 = SemioBrepDiff::default();
    d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { tol: 1e-7, id: "v-new".into(), point: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } }] });
    let mut d2 = SemioBrepDiff::default();
    d2.vertices = Some(BrepVerticesDiff { removed: vec!["v-new".into()], modified: vec![], added: vec![] });
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    d1.absorb(d2);
    assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
    assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), base, "add-then-remove-of-same-key must net to a no-op");
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_add_then_setfield_patches_added_payload() {
    let base = SemioBrepSnapshot::default();
    let mut d1 = SemioBrepDiff::default();
    d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { tol: 1e-7, id: "v-new".into(), point: SemioPoint3::default() }] });
    let mut d2 = SemioBrepDiff::default();
    d2.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![NamedModified { key: "v-new".into(), diff: BrepVertexDiff { point: Some(SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 }) } }], added: vec![] });
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    d1.absorb(d2);
    let result = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(result, sequential);
    assert_eq!(result.vertices[0].point, SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 });
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_modify_then_remove_drops_pending_patch() {
    let mut base = SemioBrepSnapshot::default();
    base.vertices.push(BrepVertex { tol: 1e-7, id: "v1".into(), point: SemioPoint3::default() });
    let mut d1 = SemioBrepDiff::default();
    d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![NamedModified { key: "v1".into(), diff: BrepVertexDiff { point: Some(SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }) } }], added: vec![] });
    let mut d2 = SemioBrepDiff::default();
    d2.vertices = Some(BrepVerticesDiff { removed: vec!["v1".into()], modified: vec![], added: vec![] });
    let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
    d1.absorb(d2);
    let result = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
    assert_eq!(result, sequential);
    assert!(result.vertices.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn absorb_law_associativity() {
    let base = sweep_a();
    let mid = sweep_b();
    let mut after = sweep_b();
    after.vertices.push(BrepVertex { tol: 1e-7, id: "v-extra".into(), point: SemioPoint3 { x: 7.0, y: 8.0, z: 9.0 } });
    let d1 = SemioBrepDiff::between(&base, &mid);
    let d2 = SemioBrepDiff::between(&mid, &after);
    let mut absorbed = d1.clone();
    absorbed.absorb(d2.clone());
    assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
    assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"));
}
//#endregion 🔖️absorb_law

//#region 🔖️diff_codec_text_binary_roundtrip_law
#[semio_framework_async_macros::async_test]
async fn diff_codec_text_binary_roundtrip_law() {
    use protocol::DiffCodec;
    let (a, b) = (sweep_a(), sweep_b());
    let cases = vec![SemioBrepDiff::default(), SemioBrepDiff::between(&a, &b), SemioBrepDiff::between(&b, &a)];
    for d in cases {
        let printed = d.print_diff();
        assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
        let parsed = SemioBrepDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
        assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

        let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
        let decoded = SemioBrepDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
        assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
    }
}
//#endregion 🔖️diff_codec_text_binary_roundtrip_law
