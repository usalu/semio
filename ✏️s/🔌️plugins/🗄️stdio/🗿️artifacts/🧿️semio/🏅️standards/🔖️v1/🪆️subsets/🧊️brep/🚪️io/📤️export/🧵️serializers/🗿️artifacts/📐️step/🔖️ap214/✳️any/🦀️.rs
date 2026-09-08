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
//! always emitted ` (unset) since `BrepCurve`/`BrepSurface` don't carry an in-plane rotation to
//! round-trip it from. Semio ids are NOT preserved as STEP ids — every export mints fresh,
//! sequential Part-21 instance ids (expected/honest: neutral in-memory ids are never a real
//! exchange format's own identity scheme; the `📥️import` leaf's own round-trip test asserts
//! structural/geometric fidelity, not id equality).

use std::collections::HashMap;

use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepSurface, SemioBrepSnapshot};
use semio_s_artifact_stdio_step::engine::part21::{Part21Builder, Part21Header, Part21Value};
use semio_s_artifact_stdio_step::schema::snapshot::StepSnapshot;

//#region 🔖️ValueBuild
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn s(text: &str) -> Part21Value {
    Part21Value::Str(text.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn xyz(p: SemioPoint3) -> Part21Value {
    Part21Value::List(vec![Part21Value::Real(p.x.into()), Part21Value::Real(p.y.into()), Part21Value::Real(p.z.into())])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bool_enum(b: bool) -> Part21Value {
    Part21Value::Enum(if b { "T".to_string() } else { "F".to_string() })
}
/// 🔁️ Inverse of the `📥️import` leaf's `expand_knots`: a flat knot vector -> `(multiplicities,
/// distinct_knots)`, grouping consecutive equal (within float-epsilon) values into runs.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn compress_knots(flat: &[f64]) -> (Vec<i64>, Vec<f64>) {
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn point_to_part21(b: &mut Part21Builder, p: SemioPoint3) -> u64 {
    b.alloc("CARTESIAN_POINT", vec![s(""), xyz(p)])
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn direction_to_part21(b: &mut Part21Builder, d: SemioPoint3) -> u64 {
    b.alloc("DIRECTION", vec![s(""), xyz(d)])
}
/// 📐️ `AXIS2_PLACEMENT_3D` with the ref_direction (in-plane rotation) always ` — see module
/// doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn axis_placement_to_part21(b: &mut Part21Builder, origin: SemioPoint3, axis: SemioPoint3) -> u64 {
    let origin_id = point_to_part21(b, origin);
    let axis_id = direction_to_part21(b, axis);
    b.alloc("AXIS2_PLACEMENT_3D", vec![s(""), Part21Value::Ref(origin_id), Part21Value::Ref(axis_id), Part21Value::Unset])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn curve_to_part21(b: &mut Part21Builder, curve: &BrepCurve) -> u64 {
    match curve {
        BrepCurve::Line { origin, direction } => {
            let point_id = point_to_part21(b, *origin);
            let dir_id = direction_to_part21(b, *direction);
            let vector_id = b.alloc("VECTOR", vec![s(""), Part21Value::Ref(dir_id), Part21Value::Real(1.0.into())]);
            b.alloc("LINE", vec![s(""), Part21Value::Ref(point_id), Part21Value::Ref(vector_id)])
        }
        BrepCurve::Circle { center, axis, radius } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("CIRCLE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*radius).into())])
        }
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("ELLIPSE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*radius_major).into()), Part21Value::Real((*radius_minor).into())])
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => {
            let cp_ids: Vec<Part21Value> = control_points.iter().map(|p| Part21Value::Ref(point_to_part21(b, *p))).collect();
            let (mults, uniq_knots) = compress_knots(knots);
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
            id
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn surface_to_part21(b: &mut Part21Builder, surface: &BrepSurface) -> u64 {
    match surface {
        BrepSurface::Plane { origin, normal } => {
            let pos_id = axis_placement_to_part21(b, *origin, *normal);
            b.alloc("PLANE", vec![s(""), Part21Value::Ref(pos_id)])
        }
        BrepSurface::Cylinder { origin, axis, radius } => {
            let pos_id = axis_placement_to_part21(b, *origin, *axis);
            b.alloc("CYLINDRICAL_SURFACE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*radius).into())])
        }
        BrepSurface::Cone { origin, axis, radius, half_angle } => {
            let pos_id = axis_placement_to_part21(b, *origin, *axis);
            b.alloc("CONICAL_SURFACE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*radius).into()), Part21Value::Real((*half_angle).into())])
        }
        BrepSurface::Sphere { center, radius } => {
            let pos_id = axis_placement_to_part21(b, *center, SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 });
            b.alloc("SPHERICAL_SURFACE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*radius).into())])
        }
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => {
            let pos_id = axis_placement_to_part21(b, *center, *axis);
            b.alloc("TOROIDAL_SURFACE", vec![s(""), Part21Value::Ref(pos_id), Part21Value::Real((*major_radius).into()), Part21Value::Real((*minor_radius).into())])
        }
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => {
            let (u, v) = (*u_count as usize, *v_count as usize);
            let mut rows = Vec::with_capacity(u);
            for ui in 0..u {
                let mut row = Vec::with_capacity(v);
                for vi in 0..v {
                    row.push(Part21Value::Ref(point_to_part21(b, control_points[ui * v + vi])));
                }
                rows.push(Part21Value::List(row));
            }
            let (u_mults, u_uniq) = compress_knots(knots_u);
            let (v_mults, v_uniq) = compress_knots(knots_v);
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
            id
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build_part21(snapshot: &SemioBrepSnapshot) -> Result<semio_s_artifact_stdio_step::engine::part21::Part21Document, String> {
    let mut b = Part21Builder::new();

    let mut vertex_ids: HashMap<&str, u64> = HashMap::new();
    for v in &snapshot.vertices {
        let point_id = point_to_part21(&mut b, v.point);
        let vertex_id = b.alloc("VERTEX_POINT", vec![s(""), Part21Value::Ref(point_id)]);
        vertex_ids.insert(v.id.as_str(), vertex_id);
    }

    let mut edge_ids: HashMap<&str, u64> = HashMap::new();
    for e in &snapshot.edges {
        let &start = vertex_ids.get(e.start_vertex.as_str()).ok_or_else(|| format!("edge {:?}: dangling start_vertex {:?}", e.id, e.start_vertex))?;
        let &end = vertex_ids.get(e.end_vertex.as_str()).ok_or_else(|| format!("edge {:?}: dangling end_vertex {:?}", e.id, e.end_vertex))?;
        let curve_id = curve_to_part21(&mut b, &e.curve);
        let edge_id = b.alloc("EDGE_CURVE", vec![s(""), Part21Value::Ref(start), Part21Value::Ref(end), Part21Value::Ref(curve_id), bool_enum(true)]);
        edge_ids.insert(e.id.as_str(), edge_id);
    }

    let mut loop_ids: HashMap<&str, u64> = HashMap::new();
    for l in &snapshot.loops {
        let mut members = Vec::with_capacity(l.edges.len());
        for le in &l.edges {
            let &edge_ref = edge_ids.get(le.edge.as_str()).ok_or_else(|| format!("loop {:?}: dangling edge {:?}", l.id, le.edge))?;
            let oe_id = b.alloc("ORIENTED_EDGE", vec![s(""), Part21Value::Derived, Part21Value::Derived, Part21Value::Ref(edge_ref), bool_enum(le.orientation)]);
            members.push(Part21Value::Ref(oe_id));
        }
        let loop_id = b.alloc("EDGE_LOOP", vec![s(""), Part21Value::List(members)]);
        loop_ids.insert(l.id.as_str(), loop_id);
    }

    let mut face_ids: HashMap<&str, u64> = HashMap::new();
    for f in &snapshot.faces {
        let &outer_ref = loop_ids.get(f.outer_loop.as_str()).ok_or_else(|| format!("face {:?}: dangling outer_loop {:?}", f.id, f.outer_loop))?;
        let outer_bound_id = b.alloc("FACE_OUTER_BOUND", vec![s(""), Part21Value::Ref(outer_ref), bool_enum(true)]);
        let mut bounds = vec![Part21Value::Ref(outer_bound_id)];
        for il in &f.inner_loops {
            let &loop_ref = loop_ids.get(il.as_str()).ok_or_else(|| format!("face {:?}: dangling inner_loop {:?}", f.id, il))?;
            let bound_id = b.alloc("FACE_BOUND", vec![s(""), Part21Value::Ref(loop_ref), bool_enum(true)]);
            bounds.push(Part21Value::Ref(bound_id));
        }
        let surface_id = surface_to_part21(&mut b, &f.surface);
        let face_id = b.alloc("ADVANCED_FACE", vec![s(""), Part21Value::List(bounds), Part21Value::Ref(surface_id), bool_enum(f.orientation)]);
        face_ids.insert(f.id.as_str(), face_id);
    }

    let mut shell_ids: HashMap<&str, u64> = HashMap::new();
    for sh in &snapshot.shells {
        let mut members = Vec::with_capacity(sh.faces.len());
        for sf in &sh.faces {
            let &face_ref = face_ids.get(sf.face.as_str()).ok_or_else(|| format!("shell {:?}: dangling face {:?}", sh.id, sf.face))?;
            members.push(Part21Value::Ref(face_ref));
        }
        let shell_id = b.alloc("CLOSED_SHELL", vec![s(""), Part21Value::List(members)]);
        shell_ids.insert(sh.id.as_str(), shell_id);
    }

    for so in &snapshot.solids {
        let outer = so.shells.iter().find(|m| !m.is_void).ok_or_else(|| format!("solid {:?}: has no non-void outer shell", so.id))?;
        let &outer_ref = shell_ids.get(outer.shell.as_str()).ok_or_else(|| format!("solid {:?}: dangling outer shell {:?}", so.id, outer.shell))?;
        b.alloc("MANIFOLD_SOLID_BREP", vec![s(""), Part21Value::Ref(outer_ref)]);
        let void_refs: Vec<Part21Value> =
            so.shells.iter().filter(|m| m.is_void).map(|m| shell_ids.get(m.shell.as_str()).copied().map(Part21Value::Ref).ok_or_else(|| format!("solid {:?}: dangling void shell {:?}", so.id, m.shell))).collect::<Result<Vec<_>, _>>()?;
        if !void_refs.is_empty() {
            b.instances.last_mut().expect("just allocated MANIFOLD_SOLID_BREP above").entities.push(("BREP_WITH_VOIDS".to_string(), vec![Part21Value::List(void_refs)]));
        }
    }

    let header = Part21Header {
        file_description: vec![Part21Value::List(vec![s("")]), s("2;1")],
        file_name: vec![s("semio.step"), s(""), Part21Value::List(vec![s("")]), Part21Value::List(vec![s("")]), s("semio"), s(""), s("")],
        file_schema: vec![Part21Value::List(vec![s("AUTOMOTIVE_DESIGN")])],
    };
    Ok(b.build(header))
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
        let doc = build_part21(from).map_err(|m| store::PackError::Schema(format!("semio brep -> step: {m}")))?;
        Ok(StepSnapshot::from_part21_document(&doc))
    }
}
//#endregion 🔖️Serializer

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
