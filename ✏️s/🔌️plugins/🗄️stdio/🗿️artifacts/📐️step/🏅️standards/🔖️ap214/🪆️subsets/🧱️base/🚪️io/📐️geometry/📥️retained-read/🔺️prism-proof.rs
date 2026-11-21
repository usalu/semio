//! 🔺 Exact recognition of an authored triangular shell as a native straight prism.

use super::*;
use semio_framework_3d_brep_construction::prism_boundary::{MAX_PRISM_CORNERS, PrismBoundary};
use semio_framework_3d_brep_surface::Surface;

pub(super) fn certify_prism(faces: &[Face]) -> Result<StepSolid, ValueError> {
    let mut caps = [0; 2];
    let mut count = 0;
    for (index, face) in faces.iter().enumerate() {
        if face.count == 3 {
            *caps.get_mut(count).ok_or_else(prism_required)? = index;
            count += 1;
        }
    }
    if count != 2 { return Err(prism_required()); }
    let uses = || faces.iter().flat_map(|face| face.edges[..face.count].iter());
    for edge in uses() {
        if uses().any(|other| other.point == edge.point && other.from != edge.from)
            || uses().filter(|other| other.id == edge.id).count() != 2
            || uses().filter(|other| other.id == edge.id && other.from == edge.to && other.to == edge.from).count() != 1 {
            return Err(invalid("STEP prism does not have shared opposite trim instances"));
        }
    }
    let first = faces[caps[0]];
    let second = faces[caps[1]];
    let cap = std::array::from_fn(|index| first.edges[index].point);
    let start = Pnt3::from_array(cap[0]);
    for candidate in &second.edges[..3] {
        let vector = Pnt3::from_array(candidate.point) - start;
        let Some(prism) = prism_recipe(cap, vector) else { continue; };
        if matches_boundary(&prism, faces) {
            return Ok(StepSolid::TriangularPrism { cap, extrusion: vector.to_array() });
        }
    }
    Err(prism_required())
}

fn prism_required() -> ValueError { unsupported("STEP faces do not certify one exact triangular prism") }

pub(super) fn prism_recipe(cap: [[f64; 3]; 3], vector: Vec3) -> Option<PrismBoundary> {
    let mut corners = [Pnt3::from_array([0.0; 3]); MAX_PRISM_CORNERS];
    for (corner, point) in corners.iter_mut().zip(cap) { *corner = Pnt3::from_array(point); }
    let a = corners[1] - corners[0];
    let b = corners[2] - corners[0];
    let normal = a.cross(b);
    let displacement = normal.normalized()?.dot(vector);
    if !displacement.is_finite() || displacement == 0.0 || !(a.norm().is_finite() && b.norm().is_finite()) {
        return None;
    }
    for index in 0..3 {
        let tangent = corners[(index + 1)%3] - corners[index];
        tangent.normalized()?;
        tangent.cross(vector).normalized()?;
        if !(corners[index] + vector).to_array().into_iter().all(f64::is_finite) {
            return None;
        }
    }
    PrismBoundary::from_planar_ring(corners, 3, vector, normal)
}

fn matches_boundary(prism: &PrismBoundary, faces: &[Face]) -> bool {
    let mut used = 0_u8;
    for (native, &count) in prism.ring_sizes().iter().enumerate() {
        let Surface::Plane { frame } = prism.surface(native) else { return false; };
        let mut ring = [[0.0;3];4];
        for (member, target) in ring[..count].iter_mut().enumerate() {
            let trim = prism.trim(native, member);
            let vertices = prism.edge(trim.edge).vertices;
            *target = prism.vertex(vertices[usize::from(!trim.forward)]).to_array();
        }
        let found = faces.iter().enumerate().position(|(index, face)| {
            used & (1 << index) == 0 && count == face.count
                && same_direction(frame.z.to_array(), face.normal)
                && ring[..count].contains(&face.origin)
                && (0..count).any(|shift| (0..count).all(|member| ring[member] == face.edges[(member + shift) % count].point))
        });
        let Some(index) = found else { return false; };
        used |= 1 << index;
    }
    true
}

fn same_direction(a: [f64; 3], b: [f64; 3]) -> bool {
    a == b || only_axis(a).is_some_and(|axis| only_axis(b) == Some(axis) && (a[axis] > 0.0) == (b[axis] > 0.0))
}
