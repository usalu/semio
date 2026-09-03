//! 📥️ Deserialize `s.stdio.semio/v1/brep` from `s.stdio.step/ap214/✳️any` — a genuine walk of
//! STEP's generic Part-21 entity graph (`VERTEX_POINT`/`EDGE_CURVE`/`ORIENTED_EDGE`/`EDGE_LOOP`/
//! `FACE_BOUND`/`FACE_OUTER_BOUND`/`ADVANCED_FACE`/`CLOSED_SHELL`/`OPEN_SHELL`/
//! `MANIFOLD_SOLID_BREP`/`BREP_WITH_VOIDS`), not a reshape. Unlike step's own
//! `⚙️engine/🧱️brep::analyze_brep_mesh` (which only understands planar faces bounded by
//! straight-line polygons and discards edges/loops as first-class entities, folding them into
//! flat index lists), this walk preserves every vertex/edge/loop/face/shell/solid as its own
//! id-keyed entity and resolves the full AP214 curve/surface vocabulary `BrepCurve`/`BrepSurface`
//! cover (`LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` and `PLANE`/
//! `CYLINDRICAL_SURFACE`/`CONICAL_SURFACE`/`SPHERICAL_SURFACE`/`TOROIDAL_SURFACE`/
//! `B_SPLINE_SURFACE_WITH_KNOTS`, both with optional `RATIONAL_B_SPLINE_*` weight fragments per
//! ISO 10303-42). Zero codec reimplementation: bytes were already turned into the generic
//! `StepEntity` graph by step's own Part-21 tokenizer (`engine::part21::parse_part21` via
//! `StepSnapshot`'s `ArtifactDsl`/`ArtifactPack` impls) — this file only maps typed snapshot to
//! typed snapshot.
//!
//! Honest boundaries (never silently fabricated — see the mirror `📤️export` leaf for the same
//! list from the other direction):
//! - An edge/face whose underlying curve/surface entity is outside the vocabulary above (e.g.
//!   `SURFACE_OF_REVOLUTION`, `OFFSET_CURVE_3D`, `RECTANGULAR_TRIMMED_SURFACE`) fails the WHOLE
//!   conversion with a descriptive `PackError`, never a guessed/defaulted shape.
//! - `EDGE_CURVE.same_sense` is not modeled — `BrepEdge.start_vertex`/`end_vertex` are taken
//!   directly from `EDGE_CURVE`'s own `edge_start`/`edge_end`, matching the convention step's own
//!   `analyze_brep_mesh` already uses for its simpler planar-only case.
//! - `VECTOR.magnitude` is dropped — `BrepCurve::Line.direction` stores only the unit `DIRECTION`,
//!   matching `engine::brep::brep_mesh_to_part21`'s own existing round-trip convention.
//! - `AXIS2_PLACEMENT_3D.ref_direction` (the local X axis / in-plane rotation) is not modeled —
//!   `BrepCurve::Circle`/`Ellipse`/`BrepSurface::Cylinder`/`Cone`/`Torus` only carry `axis` (the
//!   placement's Z direction) plus radii, so a re-exported entity is re-oriented to a canonical
//!   `ref_direction`. The supporting plane/axis and every radius are exact; only the in-plane
//!   rotation around that axis is lost — this is the honest, documented STEP↔semio impedance
//!   mismatch (semio's `BrepSurface`/`BrepCurve` model rotation-invariant primitives).
//! - `BrepShellFace.orientation` and any per-shell orientation have no STEP counterpart in
//!   `CLOSED_SHELL`/`OPEN_SHELL` (shell membership there is an unordered face-ref set; face
//!   orientation is carried entirely by `ADVANCED_FACE.same_sense`, already captured as
//!   `BrepFace.orientation`) — always imported as `true`.

use std::collections::HashMap;

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

use crate::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{
    BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot, STDIO_SEMIOBREP_DOCUMENT_SCHEMA,
};
use crate::artifacts::step::schema::snapshot::{StepEntity, StepSnapshot, StepValue};

//#region 🔖️ValueAccess
/// 🔎️ True if `e`'s primary type OR any of its complex-instance fragments match `name`
/// (case-insensitively, matching Part-21 keyword casing conventions).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn has_type(e: &StepEntity, name: &str) -> bool {
    e.name.eq_ignore_ascii_case(name) || e.complex.iter().any(|c| c.name.eq_ignore_ascii_case(name))
}
/// 🔎️ The argument list of the fragment named `name` on `e` (primary or complex), if present.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn args_for_type<'a>(e: &'a StepEntity, name: &str) -> Option<&'a [StepValue]> {
    if e.name.eq_ignore_ascii_case(name) {
        return Some(&e.args);
    }
    e.complex.iter().find(|c| c.name.eq_ignore_ascii_case(name)).map(|c| c.args.as_slice())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_real(v: &StepValue) -> Option<f64> {
    match v {
        StepValue::Real(r) => Some(*r),
        StepValue::Integer(i) => Some(*i as f64),
        _ => None,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_int(v: &StepValue) -> Option<i64> {
    match v {
        StepValue::Integer(i) => Some(*i),
        StepValue::Real(r) => Some(*r as i64),
        _ => None,
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_ref_id(v: &StepValue) -> Option<u64> {
    if let StepValue::Reference(id) = v {
        Some(*id)
    } else {
        None
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn as_agg(v: &StepValue) -> Option<&Vec<StepValue>> {
    if let StepValue::Aggregate(items) = v {
        Some(items)
    } else {
        None
    }
}
/// 🔁️ `(multiplicities, distinct_knots)` -> a flat knot vector, per ISO 10303-42's
/// `b_spline_curve_with_knots`/`b_spline_surface_with_knots` convention (each distinct knot value
/// repeated `multiplicity` times). Mirrored by `compress_knots` in the `📤️export` leaf.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expand_knots(mults: &[StepValue], vals: &[StepValue]) -> Option<Vec<f64>> {
    if mults.len() != vals.len() {
        return None;
    }
    let mut out = Vec::new();
    for (m, v) in mults.iter().zip(vals.iter()) {
        let m = as_int(m)?;
        let v = as_real(v)?;
        if m < 0 {
            return None;
        }
        for _ in 0..m {
            out.push(v);
        }
    }
    Some(out)
}
//#endregion 🔖️ValueAccess

//#region 🔖️Resolver
/// 🔎️ Id-keyed entity lookup + typed geometry resolvers over one `StepSnapshot`'s entity graph.
struct Resolver<'a> {
    by_id: HashMap<u64, &'a StepEntity>,
}

impl<'a> Resolver<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn new(step: &'a StepSnapshot) -> Self {
        Self { by_id: step.entities.iter().map(|e| (e.id, e)).collect() }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn get(&self, id: u64) -> Option<&'a StepEntity> {
        self.by_id.get(&id).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn point(&self, id: u64) -> Result<SemioPoint3, String> {
        let e = self.get(id).ok_or_else(|| format!("dangling CARTESIAN_POINT reference #{id}"))?;
        let args = args_for_type(e, "CARTESIAN_POINT").ok_or_else(|| format!("#{id} is not a CARTESIAN_POINT"))?;
        let coords = args.get(1).and_then(as_agg).ok_or_else(|| format!("CARTESIAN_POINT #{id}: coordinates not a list"))?;
        Ok(SemioPoint3 {
            x: coords.first().and_then(as_real).ok_or_else(|| format!("CARTESIAN_POINT #{id}: missing x"))?,
            y: coords.get(1).and_then(as_real).ok_or_else(|| format!("CARTESIAN_POINT #{id}: missing y"))?,
            z: coords.get(2).and_then(as_real).unwrap_or(0.0),
        })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn direction(&self, id: u64) -> Result<SemioPoint3, String> {
        let e = self.get(id).ok_or_else(|| format!("dangling DIRECTION reference #{id}"))?;
        let args = args_for_type(e, "DIRECTION").ok_or_else(|| format!("#{id} is not a DIRECTION"))?;
        let coords = args.get(1).and_then(as_agg).ok_or_else(|| format!("DIRECTION #{id}: ratios not a list"))?;
        Ok(SemioPoint3 {
            x: coords.first().and_then(as_real).ok_or_else(|| format!("DIRECTION #{id}: missing x"))?,
            y: coords.get(1).and_then(as_real).ok_or_else(|| format!("DIRECTION #{id}: missing y"))?,
            z: coords.get(2).and_then(as_real).unwrap_or(0.0),
        })
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn vector_direction(&self, id: u64) -> Result<SemioPoint3, String> {
        let e = self.get(id).ok_or_else(|| format!("dangling VECTOR reference #{id}"))?;
        let args = args_for_type(e, "VECTOR").ok_or_else(|| format!("#{id} is not a VECTOR"))?;
        let dir_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("VECTOR #{id}: orientation not a reference"))?;
        self.direction(dir_ref)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn axis_placement(&self, id: u64) -> Result<(SemioPoint3, SemioPoint3), String> {
        let e = self.get(id).ok_or_else(|| format!("dangling AXIS2_PLACEMENT_3D reference #{id}"))?;
        let args = args_for_type(e, "AXIS2_PLACEMENT_3D").ok_or_else(|| format!("#{id} is not an AXIS2_PLACEMENT_3D"))?;
        let loc_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("AXIS2_PLACEMENT_3D #{id}: location not a reference"))?;
        let origin = self.point(loc_ref)?;
        let axis = match args.get(2).and_then(as_ref_id) {
            Some(r) => self.direction(r)?,
            None => SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 },
        };
        Ok((origin, axis))
    }

    /// 🧵️ `LINE`/`CIRCLE`/`ELLIPSE`/`B_SPLINE_CURVE_WITH_KNOTS` (+ `RATIONAL_B_SPLINE_CURVE`) ->
    /// `BrepCurve`. `Err` for any other curve entity — see module doc comment.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn curve(&self, id: u64) -> Result<BrepCurve, String> {
        let e = self.get(id).ok_or_else(|| format!("dangling curve reference #{id}"))?;
        if let Some(args) = args_for_type(e, "LINE") {
            let point_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("LINE #{id}: pnt not a reference"))?;
            let vector_ref = args.get(2).and_then(as_ref_id).ok_or_else(|| format!("LINE #{id}: dir not a reference"))?;
            return Ok(BrepCurve::Line { origin: self.point(point_ref)?, direction: self.vector_direction(vector_ref)? });
        }
        if let Some(args) = args_for_type(e, "CIRCLE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("CIRCLE #{id}: position not a reference"))?;
            let (center, axis) = self.axis_placement(pos_ref)?;
            let radius = args.get(2).and_then(as_real).ok_or_else(|| format!("CIRCLE #{id}: radius not numeric"))?;
            return Ok(BrepCurve::Circle { center, axis, radius });
        }
        if let Some(args) = args_for_type(e, "ELLIPSE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("ELLIPSE #{id}: position not a reference"))?;
            let (center, axis) = self.axis_placement(pos_ref)?;
            let semi1 = args.get(2).and_then(as_real).ok_or_else(|| format!("ELLIPSE #{id}: semi_axis_1 not numeric"))?;
            let semi2 = args.get(3).and_then(as_real).ok_or_else(|| format!("ELLIPSE #{id}: semi_axis_2 not numeric"))?;
            return Ok(BrepCurve::Ellipse { center, axis, radius_major: semi1.max(semi2), radius_minor: semi1.min(semi2) });
        }
        if let Some(args) = args_for_type(e, "B_SPLINE_CURVE_WITH_KNOTS") {
            let degree = args.get(1).and_then(as_int).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: degree not integer"))? as u32;
            let cp_refs = args.get(2).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: control_points_list not a list"))?;
            let mut control_points = Vec::with_capacity(cp_refs.len());
            for cp in cp_refs {
                let r = as_ref_id(cp).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: control point not a reference"))?;
                control_points.push(self.point(r)?);
            }
            let mults = args.get(6).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: knot_multiplicities not a list"))?;
            let vals = args.get(7).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: knots not a list"))?;
            let knots = expand_knots(mults, vals).ok_or_else(|| format!("B_SPLINE_CURVE_WITH_KNOTS #{id}: malformed knot vector"))?;
            let weights = match args_for_type(e, "RATIONAL_B_SPLINE_CURVE") {
                Some(wargs) => {
                    let wagg = wargs.first().and_then(as_agg).ok_or_else(|| format!("RATIONAL_B_SPLINE_CURVE #{id}: weights_data not a list"))?;
                    wagg.iter().map(|w| as_real(w).ok_or_else(|| format!("RATIONAL_B_SPLINE_CURVE #{id}: weight not numeric"))).collect::<Result<Vec<_>, _>>()?
                }
                None => vec![1.0; control_points.len()],
            };
            return Ok(BrepCurve::Nurbs { control_points, weights, degree, knots });
        }
        Err(format!("edge references unsupported curve entity #{id} ({:?}) -- no BrepCurve variant covers it", e.name))
    }

    /// 🗺️ `PLANE`/`CYLINDRICAL_SURFACE`/`CONICAL_SURFACE`/`SPHERICAL_SURFACE`/
    /// `TOROIDAL_SURFACE`/`B_SPLINE_SURFACE_WITH_KNOTS` (+ `RATIONAL_B_SPLINE_SURFACE`) ->
    /// `BrepSurface`. `Err` for any other surface entity — see module doc comment.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn surface(&self, id: u64) -> Result<BrepSurface, String> {
        let e = self.get(id).ok_or_else(|| format!("dangling surface reference #{id}"))?;
        if let Some(args) = args_for_type(e, "PLANE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("PLANE #{id}: position not a reference"))?;
            let (origin, normal) = self.axis_placement(pos_ref)?;
            return Ok(BrepSurface::Plane { origin, normal });
        }
        if let Some(args) = args_for_type(e, "CYLINDRICAL_SURFACE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("CYLINDRICAL_SURFACE #{id}: position not a reference"))?;
            let (origin, axis) = self.axis_placement(pos_ref)?;
            let radius = args.get(2).and_then(as_real).ok_or_else(|| format!("CYLINDRICAL_SURFACE #{id}: radius not numeric"))?;
            return Ok(BrepSurface::Cylinder { origin, axis, radius });
        }
        if let Some(args) = args_for_type(e, "CONICAL_SURFACE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("CONICAL_SURFACE #{id}: position not a reference"))?;
            let (origin, axis) = self.axis_placement(pos_ref)?;
            let radius = args.get(2).and_then(as_real).ok_or_else(|| format!("CONICAL_SURFACE #{id}: radius not numeric"))?;
            let half_angle = args.get(3).and_then(as_real).ok_or_else(|| format!("CONICAL_SURFACE #{id}: semi_angle not numeric"))?;
            return Ok(BrepSurface::Cone { origin, axis, radius, half_angle });
        }
        if let Some(args) = args_for_type(e, "SPHERICAL_SURFACE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("SPHERICAL_SURFACE #{id}: position not a reference"))?;
            let (center, _axis) = self.axis_placement(pos_ref)?;
            let radius = args.get(2).and_then(as_real).ok_or_else(|| format!("SPHERICAL_SURFACE #{id}: radius not numeric"))?;
            return Ok(BrepSurface::Sphere { center, radius });
        }
        if let Some(args) = args_for_type(e, "TOROIDAL_SURFACE") {
            let pos_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| format!("TOROIDAL_SURFACE #{id}: position not a reference"))?;
            let (center, axis) = self.axis_placement(pos_ref)?;
            let major_radius = args.get(2).and_then(as_real).ok_or_else(|| format!("TOROIDAL_SURFACE #{id}: major_radius not numeric"))?;
            let minor_radius = args.get(3).and_then(as_real).ok_or_else(|| format!("TOROIDAL_SURFACE #{id}: minor_radius not numeric"))?;
            return Ok(BrepSurface::Torus { center, axis, major_radius, minor_radius });
        }
        if let Some(args) = args_for_type(e, "B_SPLINE_SURFACE_WITH_KNOTS") {
            let degree_u = args.get(1).and_then(as_int).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: u_degree not integer"))? as u32;
            let degree_v = args.get(2).and_then(as_int).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: v_degree not integer"))? as u32;
            let rows = args.get(3).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: control_points_list not a list"))?;
            let mut control_points = Vec::new();
            let mut v_count = 0u32;
            for row in rows {
                let row_agg = as_agg(row).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: control point row not a list"))?;
                v_count = row_agg.len() as u32;
                for cell in row_agg {
                    let r = as_ref_id(cell).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: control point not a reference"))?;
                    control_points.push(self.point(r)?);
                }
            }
            let u_count = rows.len() as u32;
            let u_mults = args.get(8).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: u_multiplicities not a list"))?;
            let v_mults = args.get(9).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: v_multiplicities not a list"))?;
            let u_knot_vals = args.get(10).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: u_knots not a list"))?;
            let v_knot_vals = args.get(11).and_then(as_agg).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: v_knots not a list"))?;
            let knots_u = expand_knots(u_mults, u_knot_vals).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: malformed u knot vector"))?;
            let knots_v = expand_knots(v_mults, v_knot_vals).ok_or_else(|| format!("B_SPLINE_SURFACE_WITH_KNOTS #{id}: malformed v knot vector"))?;
            let weights = match args_for_type(e, "RATIONAL_B_SPLINE_SURFACE") {
                Some(wargs) => {
                    let wrows = wargs.first().and_then(as_agg).ok_or_else(|| format!("RATIONAL_B_SPLINE_SURFACE #{id}: weights_data not a list"))?;
                    let mut flat = Vec::new();
                    for wrow in wrows {
                        let wrow_agg = as_agg(wrow).ok_or_else(|| format!("RATIONAL_B_SPLINE_SURFACE #{id}: weight row not a list"))?;
                        for w in wrow_agg {
                            flat.push(as_real(w).ok_or_else(|| format!("RATIONAL_B_SPLINE_SURFACE #{id}: weight not numeric"))?);
                        }
                    }
                    flat
                }
                None => vec![1.0; control_points.len()],
            };
            return Ok(BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v });
        }
        Err(format!("face references unsupported surface entity #{id} ({:?}) -- no BrepSurface variant covers it", e.name))
    }
}
//#endregion 🔖️Resolver

//#region 🔖️Deserializer
const STEP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.step", standard: StandardId("ap214"), subset: SubsetId::ANY };
const SEMIO_BREP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("brep") };

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn step_err(message: String) -> store::PackError {
    store::PackError::Schema(format!("semio brep <- step: {message}"))
}

/// 🧩️ `s.stdio.step/ap214/✳️any` -> `s.stdio.semio/v1/brep`. Real entity-graph walk (module doc
/// comment) — never a `Default::default()` stub.
pub struct SemioBrepFromStep;

impl ArtifactDeserializer for SemioBrepFromStep {
    type From = StepSnapshot;
    type Into = SemioBrepSnapshot;
    const FROM: Dialect = STEP_DIALECT;
    const INTO: Dialect = SEMIO_BREP_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let resolver = Resolver::new(from);
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut loops = Vec::new();
        let mut faces = Vec::new();
        let mut shells = Vec::new();
        let mut solids = Vec::new();

        for e in &from.entities {
            if has_type(e, "VERTEX_POINT") {
                let args = args_for_type(e, "VERTEX_POINT").expect("has_type just confirmed VERTEX_POINT");
                let point_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| step_err(format!("VERTEX_POINT #{}: vertex_geometry not a reference", e.id)))?;
                let point = resolver.point(point_ref).map_err(step_err)?;
                vertices.push(BrepVertex { id: format!("v{}", e.id), point, tol: 0.0 });
            }
            if has_type(e, "EDGE_CURVE") {
                let args = args_for_type(e, "EDGE_CURVE").expect("has_type just confirmed EDGE_CURVE");
                let start_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| step_err(format!("EDGE_CURVE #{}: edge_start not a reference", e.id)))?;
                let end_ref = args.get(2).and_then(as_ref_id).ok_or_else(|| step_err(format!("EDGE_CURVE #{}: edge_end not a reference", e.id)))?;
                let curve_ref = args.get(3).and_then(as_ref_id).ok_or_else(|| step_err(format!("EDGE_CURVE #{}: edge_geometry not a reference", e.id)))?;
                let curve = resolver.curve(curve_ref).map_err(step_err)?;
                edges.push(BrepEdge { id: format!("e{}", e.id), start_vertex: format!("v{start_ref}"), end_vertex: format!("v{end_ref}"), curve, tol: 0.0 });
            }
            if has_type(e, "EDGE_LOOP") {
                let args = args_for_type(e, "EDGE_LOOP").expect("has_type just confirmed EDGE_LOOP");
                let edge_refs = args.get(1).and_then(as_agg).ok_or_else(|| step_err(format!("EDGE_LOOP #{}: edge_list not a list", e.id)))?;
                let mut members = Vec::with_capacity(edge_refs.len());
                for oe in edge_refs {
                    let oe_id = as_ref_id(oe).ok_or_else(|| step_err(format!("EDGE_LOOP #{}: edge_list entry not a reference", e.id)))?;
                    let oe_entity = resolver.get(oe_id).ok_or_else(|| step_err(format!("EDGE_LOOP #{}: dangling ORIENTED_EDGE #{oe_id}", e.id)))?;
                    let oe_args = args_for_type(oe_entity, "ORIENTED_EDGE").ok_or_else(|| step_err(format!("EDGE_LOOP #{}: #{oe_id} is not an ORIENTED_EDGE", e.id)))?;
                    let edge_ref = oe_args.get(3).and_then(as_ref_id).ok_or_else(|| step_err(format!("ORIENTED_EDGE #{oe_id}: edge_element not a reference")))?;
                    let orientation = matches!(oe_args.get(4), Some(StepValue::Enum(s)) if s == "T");
                    members.push(BrepLoopEdge { edge: format!("e{edge_ref}"), orientation });
                }
                loops.push(BrepLoop { id: format!("l{}", e.id), edges: members });
            }
            if has_type(e, "ADVANCED_FACE") {
                let args = args_for_type(e, "ADVANCED_FACE").expect("has_type just confirmed ADVANCED_FACE");
                let bound_refs = args.get(1).and_then(as_agg).ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: bounds not a list", e.id)))?;
                let mut outer_loop: Option<String> = None;
                let mut inner_loops = Vec::new();
                for bound in bound_refs {
                    let bound_id = as_ref_id(bound).ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: bound entry not a reference", e.id)))?;
                    let bound_entity = resolver.get(bound_id).ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: dangling bound #{bound_id}", e.id)))?;
                    let is_outer = has_type(bound_entity, "FACE_OUTER_BOUND");
                    let bound_args =
                        args_for_type(bound_entity, "FACE_OUTER_BOUND").or_else(|| args_for_type(bound_entity, "FACE_BOUND")).ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: #{bound_id} is neither FACE_BOUND nor FACE_OUTER_BOUND", e.id)))?;
                    let loop_ref = bound_args.get(1).and_then(as_ref_id).ok_or_else(|| step_err(format!("bound #{bound_id}: bound not a reference")))?;
                    let loop_id = format!("l{loop_ref}");
                    if is_outer || outer_loop.is_none() {
                        if outer_loop.is_some() {
                            inner_loops.push(loop_id);
                        } else {
                            outer_loop = Some(loop_id);
                        }
                    } else {
                        inner_loops.push(loop_id);
                    }
                }
                let outer_loop = outer_loop.ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: no bound resolved to an outer loop", e.id)))?;
                let surface_ref = args.get(2).and_then(as_ref_id).ok_or_else(|| step_err(format!("ADVANCED_FACE #{}: face_geometry not a reference", e.id)))?;
                let surface = resolver.surface(surface_ref).map_err(step_err)?;
                let orientation = matches!(args.get(3), Some(StepValue::Enum(s)) if s == "T");
                faces.push(BrepFace { id: format!("f{}", e.id), outer_loop, inner_loops, surface, orientation, tol: 0.0 });
            }
            if has_type(e, "CLOSED_SHELL") || has_type(e, "OPEN_SHELL") {
                let args = args_for_type(e, "CLOSED_SHELL").or_else(|| args_for_type(e, "OPEN_SHELL")).expect("has_type just confirmed a shell type");
                let face_refs = args.get(1).and_then(as_agg).ok_or_else(|| step_err(format!("shell #{}: cfs_faces not a list", e.id)))?;
                let mut members = Vec::with_capacity(face_refs.len());
                for f in face_refs {
                    let f_ref = as_ref_id(f).ok_or_else(|| step_err(format!("shell #{}: face entry not a reference", e.id)))?;
                    members.push(BrepShellFace { face: format!("f{f_ref}"), orientation: true });
                }
                shells.push(BrepShell { id: format!("s{}", e.id), faces: members });
            }
            if has_type(e, "MANIFOLD_SOLID_BREP") {
                let args = args_for_type(e, "MANIFOLD_SOLID_BREP").expect("has_type just confirmed MANIFOLD_SOLID_BREP");
                let outer_ref = args.get(1).and_then(as_ref_id).ok_or_else(|| step_err(format!("MANIFOLD_SOLID_BREP #{}: outer not a reference", e.id)))?;
                let mut members = vec![BrepSolidShell { shell: format!("s{outer_ref}"), is_void: false }];
                if let Some(vargs) = args_for_type(e, "BREP_WITH_VOIDS") {
                    let void_refs = vargs.first().and_then(as_agg).ok_or_else(|| step_err(format!("BREP_WITH_VOIDS on #{}: voids not a list", e.id)))?;
                    for v in void_refs {
                        let v_ref = as_ref_id(v).ok_or_else(|| step_err(format!("BREP_WITH_VOIDS on #{}: void entry not a reference", e.id)))?;
                        members.push(BrepSolidShell { shell: format!("s{v_ref}"), is_void: true });
                    }
                }
                solids.push(BrepSolid { id: format!("so{}", e.id), shells: members });
            }
        }

        Ok(SemioBrepSnapshot { schema: STDIO_SEMIOBREP_DOCUMENT_SCHEMA.into(), vertices, edges, loops, faces, shells, solids, coedges: Vec::new(), next_label: 0 })
    }
}
//#endregion 🔖️Deserializer

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🧱️ Same single-triangular-planar-face box fixture used by step's own
    /// `⚙️engine/🧱️brep` tests — a real-world-shaped AP214 exchange snippet (3 vertices, 3
    /// straight edges, 1 planar face, 1 shell, 1 solid), not a synthetic degenerate case.
    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn fixture_step_snapshot() -> StepSnapshot {
        let doc = crate::artifacts::step::engine::part21::parse_part21(FIXTURE).expect("parse real AP214 fixture");
        StepSnapshot::from_part21_document(doc)
    }

    #[semio_framework_async_macros::async_test]
    async fn deserializes_real_step_fixture_into_topologically_faithful_brep() {
        let step = fixture_step_snapshot();
        let brep = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step)).expect("deserialize real fixture");

        assert_eq!(brep.vertices.len(), 3);
        assert_eq!(brep.edges.len(), 3);
        assert_eq!(brep.loops.len(), 1);
        assert_eq!(brep.faces.len(), 1);
        assert_eq!(brep.shells.len(), 1);
        assert_eq!(brep.solids.len(), 1);

        let v2 = brep.vertices.iter().find(|v| v.id == "v6").expect("VERTEX_POINT #6 imported as v6");
        assert_eq!(v2.point, SemioPoint3 { x: 10.0, y: 0.0, z: 0.0 });

        for e in &brep.edges {
            assert!(matches!(e.curve, BrepCurve::Line { .. }), "fixture edges are all straight LINEs, got {:?}", e.curve);
        }

        let face = &brep.faces[0];
        assert!(face.inner_loops.is_empty());
        match &face.surface {
            BrepSurface::Plane { normal, .. } => assert_eq!(*normal, SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }),
            other => panic!("expected Plane, got {other:?}"),
        }

        assert_eq!(brep.solids[0].shells.len(), 1);
        assert!(!brep.solids[0].shells[0].is_void);
    }

    #[semio_framework_async_macros::async_test]
    async fn dangling_curve_reference_errors_rather_than_fabricating() {
        // A LINE whose `dir` points at a nonexistent VECTOR must fail loudly, not silently
        // produce a zero direction.
        let bad = FIXTURE.replace("#20=LINE('',#1,#30);", "#20=LINE('',#1,#999);");
        let doc = crate::artifacts::step::engine::part21::parse_part21(&bad).expect("parse");
        let step = StepSnapshot::from_part21_document(doc);
        let result = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step));
        assert!(result.is_err(), "dangling VECTOR reference must surface as an error, not a fabricated direction");
    }

    #[semio_framework_async_macros::async_test]
    async fn unsupported_surface_kind_errors_rather_than_fabricating() {
        // Swap PLANE for a surface kind outside this leaf's supported vocabulary.
        let bad = FIXTURE.replace("#16=PLANE('',#40);", "#16=SURFACE_OF_REVOLUTION('',#20,#40);");
        let doc = crate::artifacts::step::engine::part21::parse_part21(&bad).expect("parse");
        let step = StepSnapshot::from_part21_document(doc);
        let result = semio_framework_plugin::resolve_ready(SemioBrepFromStep::deserialize(&step));
        assert!(result.is_err(), "an unsupported surface entity must error, never silently become a Plane");
    }
}
//#endregion 🧪️Tests
