//! 〰️ Exact STEP proof for the native unit-weight cuboid charts.

use super::{AuthoredCoedge, Bound, Edge, Face, StepSolidDecode, boolean, fields, invalid, list, number, only_axis, refs, unsupported};
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Vec3;
use semio_framework_value::ValueError;

type Point = [f64; 3];

#[derive(Clone, Copy, Default)]
pub(super) struct Chart {
    knots: [[f64; 2]; 2],
    controls: [[Point; 2]; 2],
    edges: [[Point; 2]; 4],
}

fn outside() -> ValueError { unsupported("STEP spline import requires the original clamped degree-one cuboid charts") }

fn span(degree: &str, multiplicities: &str, coordinates: &str) -> Result<[f64; 2], ValueError> {
    if degree != "1" || fields::<2>(list(multiplicities)?)? != ["2"; 2] { return Err(outside()); }
    let [lower, upper] = fields(list(coordinates)?)?;
    let [a, b] = [number(lower)?, number(upper)?];
    if a >= b { return Err(invalid("STEP spline knot span is collapsed")); }
    Ok([a, b])
}

fn all_false(flags: &[&str]) -> Result<bool, ValueError> {
    for flag in flags {
        if boolean(flag)? { return Ok(false); }
    }
    Ok(true)
}

impl Chart {
    pub(super) fn read(reader: &StepSolidDecode, source: &str, id: u64) -> Result<(Face, Self), ValueError> {
        let Bound { surface, same, forward, members, count } = reader.bound(source, id)?;
        if count != 4 { return Err(outside()); }
        let [_, degree_u, degree_v, controls, _, closed_u, closed_v, crossing, mult_u, mult_v, knots_u, knots_v, _] =
            fields(reader.row(source, surface, "B_SPLINE_SURFACE_WITH_KNOTS")?)?;
        if !all_false(&[closed_u, closed_v, crossing])? { return Err(outside()); }
        let knots = [span(degree_u, mult_u, knots_u)?, span(degree_v, mult_v, knots_v)?];
        let [first, second] = fields(list(controls)?)?;
        let controls = [read_points(reader, source, first)?, read_points(reader, source, second)?];
        let [[a, c], [b, d]] = controls;
        let (delta_u, delta_v) = (offset(a,b), offset(a,c));
        let u = only_axis(delta_u).ok_or_else(outside)?;
        let v = only_axis(delta_v).ok_or_else(outside)?;
        if u == v || (0..3).any(|i| d[i] != if i == u { b[i] } else { c[i] }) { return Err(outside()); }
        let signer = |axis: usize, delta: Point| Vec3::from_array(std::array::from_fn(|i| if i == axis { delta[i].signum() } else { 0.0 }));
        let normal = signer(u,delta_u).cross(signer(v,delta_v));
        let normal = if same { normal } else { -normal };

        let mut ring = [Edge::default(); 4];
        let mut edges = [[[0.0; 3]; 2]; 4];
        for index in 0..4 {
            let authored = reader.coedge(source, members[index])?;
            (ring[index], edges[index]) = edge(reader, source, authored)?;
        }
        reader.close_ring(source, &mut ring, forward)?;
        if ring.iter().any(|edge| !controls.iter().flatten().any(|point| *point == edge.point)) {
            return Err(invalid("STEP spline face trim leaves its four control corners"));
        }
        Ok((Face { origin: a, normal: normal.to_array(), edges: ring, count }, Self { knots, controls, edges }))
    }

    pub(super) fn certify(&self, primitive: SolidPrimitive, face: &Face) -> Result<(), ValueError> {
        let plane = (0..6).map(|i| primitive.surface(i)).find(|surface| {
            matches!(surface, Surface::Plane { frame } if frame.z.to_array() == face.normal)
        }).ok_or_else(outside)?;
        let [u, v] = self.knots;
        let canonical = u.map(|u| v.map(|v| plane.eval(u,v).to_array()));
        if canonical != self.controls { return Err(outside()); }
        for controls in self.edges {
            let agrees = (0..12).any(|i| primitive.edge(i).vertices.map(|vertex| primitive.vertex(vertex).to_array()) == controls);
            if !agrees { return Err(outside()); }
        }
        Ok(())
    }
}

fn offset(a: Point, b: Point) -> Point { std::array::from_fn(|i| b[i] - a[i]) }

fn read_points(reader: &StepSolidDecode, source: &str, token: &str) -> Result<[Point; 2], ValueError> {
    let [a, b] = refs(token)?;
    Ok([reader.vector(source,a,"CARTESIAN_POINT")?, reader.vector(source,b,"CARTESIAN_POINT")?])
}

fn edge(reader: &StepSolidDecode, source: &str, authored: AuthoredCoedge) -> Result<(Edge, [Point; 2]), ValueError> {
    let AuthoredCoedge { edge, from, to, curve, same, forward } = authored;
    let [_, degree, controls, _, closed, crossing, mults, knots, _] =
        fields(reader.row(source, curve, "B_SPLINE_CURVE_WITH_KNOTS")?)?;
    if !all_false(&[closed, crossing])? || span(degree, mults, knots)? != [0.0, 1.0] { return Err(outside()); }
    let ends = [reader.vertex(source, from)?, reader.vertex(source, to)?];
    let [first,last] = read_points(reader, source, controls)?;
    let ordered = if same { [first,last] } else { [last,first] };
    if ends != ordered || only_axis(offset(ends[0], ends[1])).is_none() {
        return Err(invalid("STEP spline edge control span disagrees with its vertices"));
    }
    let record = if forward { Edge { id: edge, from, to, point: ends[0] } }
        else { Edge { id: edge, from: to, to: from, point: ends[1] } };
    Ok((record, [first,last]))
}
