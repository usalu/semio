use super::*;

/// 🧱️ A small but fully-populated, self-referentially-consistent b-rep: one triangular face
/// bounding one shell bounding one solid. Reused by the codec_retention_law test below.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn populated_snapshot() -> SemioBrepSnapshot {
    let mut s = SemioBrepSnapshot::default();
    s.vertices = vec![
        BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
        BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 }, tol: 1e-7 },
        BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 }, tol: 1e-7 },
    ];
    s.edges = vec![
        BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: s.vertices[0].point, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } }, tol: 1e-7 },
        BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Line { origin: s.vertices[1].point, direction: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 } }, tol: 1e-7 },
        BrepEdge {
            id: "e3".into(),
            start_vertex: "v3".into(),
            end_vertex: "v1".into(),
            curve: BrepCurve::Nurbs { control_points: vec![s.vertices[2].point, s.vertices[0].point], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] },
            tol: 1e-7,
        },
    ];
    s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }] }];
    s.coedges = vec![
        BrepCoedge { id: "co1".into(), edge: "e1".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co2".into(), prev: "co3".into() },
        BrepCoedge { id: "co2".into(), edge: "e2".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co3".into(), prev: "co1".into() },
        BrepCoedge { id: "co3".into(), edge: "e3".into(), forward: true, pcurve: None, prange: (0.0, 1.0), loop_id: "l1".into(), next: "co1".into(), prev: "co2".into() },
    ];
    s.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true, tol: 1e-7 }];
    s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];
    s.next_label = 42;
    s
}

#[semio_framework_async_macros::async_test]
async fn json_pack_round_trips() {
    let snap = SemioBrepSnapshot::default();
    let bytes = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
    let back = <SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(snap, back);
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips() {
    let snap = SemioBrepSnapshot::default();
    let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    let back = <SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(snap, back);
}

/// 🧪️ codec_retention_law: a fully-populated snapshot (every collection non-empty, every
/// `BrepSurface`/`BrepCurve` variant represented at least once) survives a pack AND a dsl
/// round trip byte-for-byte (structurally — every field, incl. every `Nurbs` variant's
/// `Vec<SemioPoint3>`/`Vec<f64>` runs, round-trips exactly).
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_populated_snapshot_round_trips_pack_and_dsl() {
    let snap = populated_snapshot();
    let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&snap);
    assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), snap);
    let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&snap);
    assert_eq!(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), snap);
}

/// 🧪️ Every `BrepCurve`/`BrepSurface` variant (incl. both `Nurbs` shapes) round-trips through
/// both the pack binary and the dsl text codec — the demo fixture used by the fixture-honesty
/// conformance law.
#[semio_framework_async_macros::async_test]
async fn demo_snapshot_round_trips_pack_and_dsl() {
    let demo = demo_brep_snapshot();
    let packed = <SemioBrepSnapshot as store::ArtifactPack>::encode_pack(&demo);
    assert_eq!(<SemioBrepSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("decode"), demo);
    let text = <SemioBrepSnapshot as store::ArtifactDsl>::print_dsl(&demo);
    assert_eq!(<SemioBrepSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse"), demo);
}
