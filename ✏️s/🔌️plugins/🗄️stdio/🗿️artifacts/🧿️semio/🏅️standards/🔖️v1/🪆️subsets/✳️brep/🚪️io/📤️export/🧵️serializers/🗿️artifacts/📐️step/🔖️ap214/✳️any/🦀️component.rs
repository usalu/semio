//! 📤️ Serialize `s.stdio.semio/v1/brep` to `s.stdio.step/ap214/✳️any` — the inverse of the
//! `📥️import` sibling leaf's entity-graph walk, minting a real AP214-shaped Part-21 graph
//! (`CARTESIAN_POINT`/`DIRECTION`/`AXIS2_PLACEMENT_3D`/`VERTEX_POINT`/`EDGE_CURVE`/
//! `ORIENTED_EDGE`/`EDGE_LOOP`/`FACE_BOUND`/`FACE_OUTER_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/
//! `MANIFOLD_SOLID_BREP`/`BREP_WITH_VOIDS`) from semio's explicit id-keyed b-rep graph. Zero
//! codec reimplementation: entity allocation and Part-21 text writing both stay step's own
//! (`engine::part21::Part21Builder`/`write_part21`, reused directly via `StepSnapshot::
//! from_part21_document`) — this file only maps typed snapshot to typed snapshot.
//!
//! Honest boundaries — see the mirror `📥️import` leaf's module doc comment for the full list;
//! restated briefly: `BrepShellFace.orientation` has no STEP counterpart (dropped, since
//! `CLOSED_SHELL` face membership is an unordered ref set); `AXIS2_PLACEMENT_3D.ref_direction` is
//! always emitted `$` (unset) since `BrepCurve`/`BrepSurface` don't carry an in-plane rotation to
//! round-trip it from. Semio ids are NOT preserved as STEP ids — every export mints fresh,
//! sequential Part-21 instance ids (expected/honest: neutral in-memory ids are never a real
//! exchange format's own identity scheme; the `📥️import` leaf's own round-trip test asserts
//! structural/geometric fidelity, not id equality).

use std::collections::HashMap;

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepSurface, SemioBrepSnapshot};
use crate::artifacts::step::engine::part21::{Part21Builder, Part21Header, Part21Value};
use crate::artifacts::step::schema::snapshot::StepSnapshot;

//#region 🔖️ValueBuild
async fn s(text: &str) -> Part21Value {
    Part21Value::Str(text.to_string())
}
async fn xyz(p: SemioPoint3) -> Part21Value {
    Part21Value::List(vec![Part21Value::Real(p.x.into()), Part21Value::Real(p.y.into()), Part21Value::Real(p.z.into())])
}
async fn bool_enum(b: bool) -> Part21Value {
    Part21Value::Enum(if b { "T".to_string() } else { "F".to_string() })
}
/// 🔁️ Inverse of the `📥️import` leaf's `expand_knots`: a flat knot vector -> `(multiplicities,
/// distinct_knots)`, grouping consecutive equal (within float-epsilon) values into runs.
async fn compress_knots(flat: &[f64]) -> (Vec<i64>, Vec<f64>) {
    let mut mults: Vec<i64> = Vec::new();
    let mut uniq: Vec<f64> = Vec::new();
    for &k in flat {
        if let Some(&last) = uniq.last() {
            if (k - last).abs() < 1e-9 {
                *mults.last_mut().expect("uniq and mults grow together") += 1;
                continue;
            }
        }
        uniq.push(k);
        mults.push(1);
    }
    (mults, uniq)
}
//#endregion 🔖️ValueBuild

//#region 🔖️Build
async fn point_to_part21(b: &mut Part21Builder, p: SemioPoint3) -> u64 {
    b.alloc("CARTESIAN_POINT", vec![s("").await, xyz(p).await]).await
}
async fn direction_to_part21(b: &mut Part21Builder, d: SemioPoint3) -> u64 {
    b.alloc("DIRECTION", vec![s("").await, xyz(d).await]).await
}
/// 📐️ `AXIS2_PLACEMENT_3D` with the ref_direction (in-plane rotation) always `$` — see module
/// doc comment.
async fn axis_placement_to_part21(b: &mut Part21Builder, origin: SemioPoint3, axis: SemioPoint3) -> u64 {
    let origin_id = point_to_part21(b, origin);
    let axis_id = direction_to_part21(b, axis);
    b.alloc("AXIS2_PLACEMENT_3D", vec![s("").await, Part21Value::Ref(origin_id.await), Part21Value::Ref(axis_id.await), Part21Value::Unset]).await
}

async fn curve_to_part21(b: &mut Part21Builder, curve: &BrepCurve) -> u64 {
    match curve {
        BrepCurve::Line { origin, direction } => {
            let point_id = point_to_part21(b, *origin);
            let dir_id = direction_to_part21(b, *direction);
            let vector_id = b.alloc("VECTOR", vec![s("").await, Part21Value::Ref(dir_id.await), Part21Value::Real(1.0.into())]);
            b.alloc("LINE", vec![s("").await, Part21Value::Ref(point_id.await), Part21Value::Ref(vector_id.await)]).await
        }
        BrepCurve::Circle { center, axis, radius } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("CIRCLE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*radius).into())]).await
        }
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("ELLIPSE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*radius_major).into()), Part21Value::Real((*radius_minor).into())]).await
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => {
            let cp_ids: Vec<Part21Value> = control_points.iter().map(|p| Part21Value::Ref(point_to_part21(b, *p))).collect();
            let (mults, uniq_knots) = compress_knots(knots).await;
            let base_args = vec![
                s(""),
                Part21Value::Int(*degree as i64),
                Part21Value::List(cp_ids),
                Part21Value::Enum("UNSPECIFIED".into()),
                Part21Value::Enum("F".into()),
                Part21Value::Enum("F".into()),
                Part21Value::List(mults.iter().map(|m| Part21Value::Int(*m)).collect()),
                Part21Value::List(uniq_knots.iter().map(|k| Part21Value::Real((*k).into())).collect()),
                Part21Value::Enum("UNSPECIFIED".into()),
            ];
            let id = b.alloc("B_SPLINE_CURVE_WITH_KNOTS", base_args);
            let uniform = weights.iter().all(|w| (w - 1.0).abs() < 1e-12);
            if !uniform {
                let weight_args = vec![Part21Value::List(weights.iter().map(|w| Part21Value::Real((*w).into())).collect())];
                b.instances.last_mut().expect("just allocated above").entities.push(("RATIONAL_B_SPLINE_CURVE".to_string(), weight_args));
            }
            id.await
        }
    }
}

async fn surface_to_part21(b: &mut Part21Builder, surface: &BrepSurface) -> u64 {
    match surface {
        BrepSurface::Plane { origin, normal } => {
            let pos_id = axis_placement_to_part21(b, *origin, *normal);
            b.alloc("PLANE", vec![s("").await, Part21Value::Ref(pos_id.await)]).await
        }
        BrepSurface::Cylinder { origin, axis, radius } => {
            let pos_id = axis_placement_to_part21(b, *origin, *axis);
            b.alloc("CYLINDRICAL_SURFACE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*radius).into())]).await
        }
        BrepSurface::Cone { origin, axis, radius, half_angle } => {
            let pos_id = axis_placement_to_part21(b, *origin, *axis);
            b.alloc("CONICAL_SURFACE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*radius).into()), Part21Value::Real((*half_angle).into())]).await
        }
        BrepSurface::Sphere { center, radius } => {
            let pos_id = axis_placement_to_part21(b, *center, SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 });
            b.alloc("SPHERICAL_SURFACE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*radius).into())]).await
        }
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("TOROIDAL_SURFACE", vec![s("").await, Part21Value::Ref(pos_id.await), Part21Value::Real((*major_radius).into()), Part21Value::Real((*minor_radius).into())]).await
        }
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => {
            let (u, v) = (*u_count as usize, *v_count as usize);
            let mut rows = Vec::with_capacity(u);
            for ui in 0..u {
                let mut row = Vec::with_capacity(v);
                for vi in 0..v {
                    row.push(Part21Value::Ref(point_to_part21(b, control_points[ui * v + vi]).await));
                }
                rows.push(Part21Value::List(row));
            }
            let (u_mults, u_uniq) = compress_knots(knots_u).await;
            let (v_mults, v_uniq) = compress_knots(knots_v).await;
            let base_args = vec![
                s(""),
                Part21Value::Int(*degree_u as i64),
                Part21Value::Int(*degree_v as i64),
                Part21Value::List(rows),
                Part21Value::Enum("UNSPECIFIED".into()),
                Part21Value::Enum("F".into()),
                Part21Value::Enum("F".into()),
                Part21Value::Enum("F".into()),
                Part21Value::List(u_mults.iter().map(|m| Part21Value::Int(*m)).collect()),
                Part21Value::List(v_mults.iter().map(|m| Part21Value::Int(*m)).collect()),
                Part21Value::List(u_uniq.iter().map(|k| Part21Value::Real((*k).into())).collect()),
                Part21Value::List(v_uniq.iter().map(|k| Part21Value::Real((*k).into())).collect()),
                Part21Value::Enum("UNSPECIFIED".into()),
            ];
            let id = b.alloc("B_SPLINE_SURFACE_WITH_KNOTS", base_args);
            let uniform = weights.iter().all(|w| (w - 1.0).abs() < 1e-12);
            if !uniform {
                let mut wrows = Vec::with_capacity(u);
                for ui in 0..u {
                    let mut wrow = Vec::with_capacity(v);
                    for vi in 0..v {
                        wrow.push(Part21Value::Real(weights[ui * v + vi].into()));
                    }
                    wrows.push(Part21Value::List(wrow));
                }
                b.instances.last_mut().expect("just allocated above").entities.push(("RATIONAL_B_SPLINE_SURFACE".to_string(), vec![Part21Value::List(wrows)]));
            }
            id.await
        }
    }
}

async fn build_part21(snapshot: &SemioBrepSnapshot) -> Result<crate::artifacts::step::engine::part21::Part21Document, String> {
    let mut b = Part21Builder::new().await;

    let mut vertex_ids: HashMap<&str, u64> = HashMap::new();
    for v in &snapshot.vertices {
        let point_id = point_to_part21(&mut b, v.point);
        let vertex_id = b.alloc("VERTEX_POINT", vec![s("").await, Part21Value::Ref(point_id.await)]).await;
        vertex_ids.insert(v.id.as_str(), vertex_id);
    }

    let mut edge_ids: HashMap<&str, u64> = HashMap::new();
    for e in &snapshot.edges {
        let &start = vertex_ids.get(e.start_vertex.as_str()).ok_or_else(|| format!("edge {:?}: dangling start_vertex {:?}", e.id, e.start_vertex))?;
        let &end = vertex_ids.get(e.end_vertex.as_str()).ok_or_else(|| format!("edge {:?}: dangling end_vertex {:?}", e.id, e.end_vertex))?;
        let curve_id = curve_to_part21(&mut b, &e.curve);
        let edge_id = b.alloc("EDGE_CURVE", vec![s("").await, Part21Value::Ref(start), Part21Value::Ref(end), Part21Value::Ref(curve_id.await), bool_enum(true).await]).await;
        edge_ids.insert(e.id.as_str(), edge_id);
    }

    let mut loop_ids: HashMap<&str, u64> = HashMap::new();
    for l in &snapshot.loops {
        let mut members = Vec::with_capacity(l.edges.len());
        for le in &l.edges {
            let &edge_ref = edge_ids.get(le.edge.as_str()).ok_or_else(|| format!("loop {:?}: dangling edge {:?}", l.id, le.edge))?;
            let oe_id = b.alloc("ORIENTED_EDGE", vec![s("").await, Part21Value::Derived, Part21Value::Derived, Part21Value::Ref(edge_ref), bool_enum(le.orientation).await]).await;
            members.push(Part21Value::Ref(oe_id));
        }
        let loop_id = b.alloc("EDGE_LOOP", vec![s("").await, Part21Value::List(members)]).await;
        loop_ids.insert(l.id.as_str(), loop_id);
    }

    let mut face_ids: HashMap<&str, u64> = HashMap::new();
    for f in &snapshot.faces {
        let &outer_ref = loop_ids.get(f.outer_loop.as_str()).ok_or_else(|| format!("face {:?}: dangling outer_loop {:?}", f.id, f.outer_loop))?;
        let outer_bound_id = b.alloc("FACE_OUTER_BOUND", vec![s("").await, Part21Value::Ref(outer_ref), bool_enum(true).await]).await;
        let mut bounds = vec![Part21Value::Ref(outer_bound_id)];
        for il in &f.inner_loops {
            let &loop_ref = loop_ids.get(il.as_str()).ok_or_else(|| format!("face {:?}: dangling inner_loop {:?}", f.id, il))?;
            let bound_id = b.alloc("FACE_BOUND", vec![s("").await, Part21Value::Ref(loop_ref), bool_enum(true).await]).await;
            bounds.push(Part21Value::Ref(bound_id));
        }
        let surface_id = surface_to_part21(&mut b, &f.surface);
        let face_id = b.alloc("ADVANCED_FACE", vec![s("").await, Part21Value::List(bounds), Part21Value::Ref(surface_id.await), bool_enum(f.orientation).await]).await;
        face_ids.insert(f.id.as_str(), face_id);
    }

    let mut shell_ids: HashMap<&str, u64> = HashMap::new();
    for sh in &snapshot.shells {
        let mut members = Vec::with_capacity(sh.faces.len());
        for sf in &sh.faces {
            let &face_ref = face_ids.get(sf.face.as_str()).ok_or_else(|| format!("shell {:?}: dangling face {:?}", sh.id, sf.face))?;
            members.push(Part21Value::Ref(face_ref));
        }
        let shell_id = b.alloc("CLOSED_SHELL", vec![s("").await, Part21Value::List(members)]).await;
        shell_ids.insert(sh.id.as_str(), shell_id);
    }

    for so in &snapshot.solids {
        let outer = so.shells.iter().find(|m| !m.is_void).ok_or_else(|| format!("solid {:?}: has no non-void outer shell", so.id))?;
        let &outer_ref = shell_ids.get(outer.shell.as_str()).ok_or_else(|| format!("solid {:?}: dangling outer shell {:?}", so.id, outer.shell))?;
        b.alloc("MANIFOLD_SOLID_BREP", vec![s("").await, Part21Value::Ref(outer_ref)]).await;
        let void_refs: Vec<Part21Value> =
            so.shells.iter().filter(|m| m.is_void).map(|m| shell_ids.get(m.shell.as_str()).copied().map(Part21Value::Ref).ok_or_else(|| format!("solid {:?}: dangling void shell {:?}", so.id, m.shell))).collect::<Result<Vec<_>, _>>()?;
        if !void_refs.is_empty() {
            b.instances.last_mut().expect("just allocated MANIFOLD_SOLID_BREP above").entities.push(("BREP_WITH_VOIDS".to_string(), vec![Part21Value::List(void_refs)]));
        }
    }

    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![s("").await]), s("2;1").await],
        file_name: vec![s("semio.step").await, s("").await, Part21Value::List(vec![s("").await]), Part21Value::List(vec![s("").await]), s("semio").await, s("").await, s("").await],
        file_schema: vec![Part21Value::List(vec![s("AUTOMOTIVE_DESIGN").await])],
    };
    Ok(b.build(header).await)
}
//#endregion 🔖️Build

//#region 🔖️Serializer
const SEMIO_BREP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };
const STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId::ANY };

/// 🧵️ `s.stdio.semio/v1/brep` -> `s.stdio.step/ap214/✳️any`. Real graph builder (module doc
/// comment) — never a `Default::default()` stub.
pub struct SemioBrepToStep;

impl ArtifactSerializer for SemioBrepToStep {
    type From = SemioBrepSnapshot;
    type Into = StepSnapshot;
    const FROM: Dialect = SEMIO_BREP_DIALECT;
    const INTO: Dialect = STEP_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let doc = build_part21(from).await.map_err(|m| store::PackError::Schema(format!("semio brep -> step: {m}")))?;
        Ok(StepSnapshot::from_part21_document(doc).await)
    }
}
//#endregion 🔖️Serializer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::brep::io::import::deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep;
    use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepVertex};
    use semio_framework_plugin::ArtifactDeserializer;

    /// 🧱️ Exercises every `BrepCurve`/`BrepSurface` variant (Line/Circle/Ellipse/Nurbs curves;
    /// Plane/Cylinder/Cone/Sphere/Torus/Nurbs surfaces) plus a face with an inner (hole) loop and
    /// a solid with a void shell — real-world-shaped coverage of the full AP214 vocabulary this
    /// bridge supports, not a minimal degenerate case.
    async fn full_vocabulary_snapshot() -> SemioBrepSnapshot {
        let mut snap = SemioBrepSnapshot::default();
        snap.vertices = vec![
            BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } },
            BrepVertex { id: "v2".into(), point: SemioPoint3 { x: 4.0, y: 0.0, z: 0.0 } },
            BrepVertex { id: "v3".into(), point: SemioPoint3 { x: 4.0, y: 3.0, z: 0.0 } },
            BrepVertex { id: "v4".into(), point: SemioPoint3 { x: 0.0, y: 3.0, z: 0.0 } },
        ];
        snap.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v2".into(), curve: BrepCurve::Line { origin: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } },
            BrepEdge { id: "e2".into(), start_vertex: "v2".into(), end_vertex: "v3".into(), curve: BrepCurve::Circle { center: SemioPoint3 { x: 4.0, y: 1.5, z: 0.0 }, axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.5 } },
            BrepEdge {
                id: "e3".into(),
                start_vertex: "v3".into(),
                end_vertex: "v4".into(),
                curve: BrepCurve::Ellipse { center: SemioPoint3 { x: 2.0, y: 3.0, z: 0.0 }, axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius_major: 2.0, radius_minor: 1.0 },
            },
            BrepEdge {
                id: "e4".into(),
                start_vertex: "v4".into(),
                end_vertex: "v1".into(),
                curve: BrepCurve::Nurbs {
                    control_points: vec![SemioPoint3 { x: 0.0, y: 3.0, z: 0.0 }, SemioPoint3 { x: -1.0, y: 1.5, z: 0.0 }, SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }],
                    weights: vec![1.0, 0.7, 1.0],
                    degree: 2,
                    knots: vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                },
            },
        ];
        snap.loops = vec![
            BrepLoop {
                id: "l1".into(),
                edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }, BrepLoopEdge { edge: "e2".into(), orientation: true }, BrepLoopEdge { edge: "e3".into(), orientation: true }, BrepLoopEdge { edge: "e4".into(), orientation: true }],
            },
            BrepLoop { id: "l2".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] },
        ];
        snap.faces = vec![
            BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec!["l2".into()], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true },
            BrepFace { id: "f2".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Cylinder { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 2.0 }, orientation: true },
            BrepFace { id: "f3".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Cone { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 2.0, half_angle: 0.4 }, orientation: false },
            BrepFace { id: "f4".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3 { x: 1.0, y: 1.0, z: 0.0 }, radius: 3.0 }, orientation: true },
            BrepFace {
                id: "f5".into(),
                outer_loop: "l1".into(),
                inner_loops: vec![],
                surface: BrepSurface::Torus { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, major_radius: 5.0, minor_radius: 1.0 },
                orientation: true,
            },
            BrepFace {
                id: "f6".into(),
                outer_loop: "l1".into(),
                inner_loops: vec![],
                surface: BrepSurface::Nurbs {
                    control_points: vec![SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 0.0, z: 1.0 }, SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }],
                    weights: vec![1.0, 0.5, 1.0, 0.5],
                    u_count: 2,
                    v_count: 2,
                    degree_u: 1,
                    degree_v: 1,
                    knots_u: vec![0.0, 0.0, 1.0, 1.0],
                    knots_v: vec![0.0, 0.0, 1.0, 1.0],
                },
                orientation: true,
            },
        ];
        snap.shells = vec![
            BrepShell {
                id: "sh1".into(),
                faces: vec![
                    BrepShellFace { face: "f1".into(), orientation: true },
                    BrepShellFace { face: "f2".into(), orientation: true },
                    BrepShellFace { face: "f3".into(), orientation: true },
                    BrepShellFace { face: "f4".into(), orientation: true },
                    BrepShellFace { face: "f5".into(), orientation: true },
                    BrepShellFace { face: "f6".into(), orientation: true },
                ],
            },
            BrepShell { id: "sh2".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] },
        ];
        snap.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "sh1".into(), is_void: false }, BrepSolidShell { shell: "sh2".into(), is_void: true }] }];
        snap
    }

    async fn assert_curve_matches(o: &BrepCurve, r: &BrepCurve) {
        match (o, r) {
            (BrepCurve::Line { origin: oo, direction: od }, BrepCurve::Line { origin: ro, direction: rd }) => {
                assert_eq!(oo, ro);
                assert_eq!(od, rd);
            }
            (BrepCurve::Circle { center: oc, axis: oa, radius: or_ }, BrepCurve::Circle { center: rc, axis: ra, radius: rr }) => {
                assert_eq!(oc, rc);
                assert_eq!(oa, ra);
                assert_eq!(or_, rr);
            }
            (BrepCurve::Ellipse { center: oc, axis: oa, radius_major: oma, radius_minor: omi }, BrepCurve::Ellipse { center: rc, axis: ra, radius_major: rma, radius_minor: rmi }) => {
                assert_eq!(oc, rc);
                assert_eq!(oa, ra);
                assert_eq!(oma, rma);
                assert_eq!(omi, rmi);
            }
            (BrepCurve::Nurbs { control_points: ocp, weights: ow, degree: od, knots: ok }, BrepCurve::Nurbs { control_points: rcp, weights: rw, degree: rd, knots: rk }) => {
                assert_eq!(ocp, rcp);
                assert_eq!(ow, rw);
                assert_eq!(od, rd);
                assert_eq!(ok, rk);
            }
            (o, r) => panic!("curve kind changed across round trip: {o:?} -> {r:?}"),
        }
    }

    async fn assert_surface_matches(o: &BrepSurface, r: &BrepSurface) {
        match (o, r) {
            (BrepSurface::Plane { origin: oo, normal: on }, BrepSurface::Plane { origin: ro, normal: rn }) => {
                assert_eq!(oo, ro);
                assert_eq!(on, rn);
            }
            (BrepSurface::Cylinder { origin: oo, axis: oa, radius: or_ }, BrepSurface::Cylinder { origin: ro, axis: ra, radius: rr }) => {
                assert_eq!(oo, ro);
                assert_eq!(oa, ra);
                assert_eq!(or_, rr);
            }
            (BrepSurface::Cone { origin: oo, axis: oa, radius: or_, half_angle: oh }, BrepSurface::Cone { origin: ro, axis: ra, radius: rr, half_angle: rh }) => {
                assert_eq!(oo, ro);
                assert_eq!(oa, ra);
                assert_eq!(or_, rr);
                assert_eq!(oh, rh);
            }
            (BrepSurface::Sphere { center: oc, radius: or_ }, BrepSurface::Sphere { center: rc, radius: rr }) => {
                assert_eq!(oc, rc);
                assert_eq!(or_, rr);
            }
            (BrepSurface::Torus { center: oc, axis: oa, major_radius: oma, minor_radius: omi }, BrepSurface::Torus { center: rc, axis: ra, major_radius: rma, minor_radius: rmi }) => {
                assert_eq!(oc, rc);
                assert_eq!(oa, ra);
                assert_eq!(oma, rma);
                assert_eq!(omi, rmi);
            }
            (
                BrepSurface::Nurbs { control_points: ocp, weights: ow, u_count: ou, v_count: ov, degree_u: odu, degree_v: odv, knots_u: oku, knots_v: okv },
                BrepSurface::Nurbs { control_points: rcp, weights: rw, u_count: ru, v_count: rv, degree_u: rdu, degree_v: rdv, knots_u: rku, knots_v: rkv },
            ) => {
                assert_eq!(ocp, rcp);
                assert_eq!(ow, rw);
                assert_eq!(ou, ru);
                assert_eq!(ov, rv);
                assert_eq!(odu, rdu);
                assert_eq!(odv, rdv);
                assert_eq!(oku, rku);
                assert_eq!(okv, rkv);
            }
            (o, r) => panic!("surface kind changed across round trip: {o:?} -> {r:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn round_trips_full_curve_and_surface_vocabulary_through_step() {
        let original = full_vocabulary_snapshot();
        let step = semio_framework_plugin::resolve_ready(SemioBrepToStep::serialize(&original)).expect("serialize to step");
        let reimported = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step)).expect("deserialize back");

        assert_eq!(reimported.vertices.len(), original.vertices.len());
        assert_eq!(reimported.edges.len(), original.edges.len());
        assert_eq!(reimported.loops.len(), original.loops.len());
        assert_eq!(reimported.faces.len(), original.faces.len());
        assert_eq!(reimported.shells.len(), original.shells.len());
        assert_eq!(reimported.solids.len(), original.solids.len());

        for (o, r) in original.vertices.iter().zip(reimported.vertices.iter()) {
            assert_eq!(o.point, r.point, "vertex point drifted across round trip");
        }
        for (o, r) in original.edges.iter().zip(reimported.edges.iter()) {
            assert_curve_matches(&o.curve, &r.curve);
        }
        for (o, r) in original.faces.iter().zip(reimported.faces.iter()) {
            assert_eq!(o.orientation, r.orientation, "face orientation drifted");
            assert_eq!(o.inner_loops.len(), r.inner_loops.len(), "inner loop count drifted");
            assert_surface_matches(&o.surface, &r.surface);
        }

        let void_count = |shells: &[BrepSolidShell]| shells.iter().filter(|m| m.is_void).count();
        assert_eq!(void_count(&original.solids[0].shells), void_count(&reimported.solids[0].shells));
        assert!(reimported.solids[0].shells.iter().any(|m| m.is_void), "void shell must survive the round trip");
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_reference_errors_rather_than_fabricating() {
        let mut snap = SemioBrepSnapshot::default();
        snap.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "nonexistent".into(), end_vertex: "also-nonexistent".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
        let result = semio_framework_plugin::resolve_ready(SemioBrepToStep::serialize(&snap));
        assert!(result.is_err(), "an edge referencing a nonexistent vertex must error, not silently drop the edge");
    }
}
//#endregion 🧪️Tests
