//! 🏠️ Pitched roof surfaces over polygons with holes: hip, gable and mansard on arbitrary simple footprints, concave ones included.
//!
//! A roof is the weighted straight skeleton of its footprint read as heights: the edge of pitch `p` moves inward at `1 / tan(p)` per
//! metre of height, so every skeleton face is the plane of one footprint edge. A hip roof gives every edge the same pitch, a gable
//! roof makes the edges across the ridge direction vertical (pitch `pi / 2`, speed zero: a gable end), a mansard runs the skeleton to
//! the break height with the lower pitches and again from the wavefront left there with the upper pitches. Heights are relative to
//! the footprint plane; the caller adds the eave elevation. Only straight edges are supported; curved footprints are flattened first.
//!
//! Related: <https://en.wikipedia.org/wiki/Hip_roof>, <https://en.wikipedia.org/wiki/Mansard_roof>.

use crate::mesh::TriMesh;
use crate::skeleton::{straight_skeleton_until, Ring, Skeleton, SkeletonError};
use crate::triangulation::triangulate;
use crate::vector::Xyz;
use crate::Point;
use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::FRAC_PI_2;

/// 🔼 Pitch of a vertical edge (a gable end).
pub const VERTICAL: f64 = FRAC_PI_2;

/// 📐️ Largest sine of the angle between a footprint edge and the perpendicular of the ridge direction for which the edge still is a
/// gable end (about one degree).
pub const GABLE_END_TOLERANCE: f64 = 0.02;

/// 🏠️ The form of a roof over a footprint; pitches are angles above the horizontal in radians, `ridge_direction` an angle in plan.
#[derive(Clone, Debug, PartialEq)]
pub enum Roof {
    Hip { pitch: f64 },
    Gable { pitch: f64, ridge_direction: f64 },
    Mansard { lower_pitch: f64, upper_pitch: f64, break_height: f64 },
    Pitched { pitches: Vec<Vec<f64>> },
}

/// 📏️ The kind of a roof line: a ridge is horizontal, a hip descends from a ridge or apex to a convex corner, a valley runs into a
/// reflex corner, a verge follows the sloped top of a gable end, a break is the crease between the two slopes of a mansard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoofLineKind {
    Ridge,
    Hip,
    Valley,
    Verge,
    Break,
}

/// 📏️ One line of the roof between two points in metres, the third coordinate being the height above the footprint plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoofLine {
    pub from: Xyz,
    pub to: Xyz,
    pub kind: RoofLineKind,
}

/// ▭️ One planar face of the roof: the footprint edge (`ring`, `edge`) it rises from, its zone (`0` below the break of a mansard,
/// `1` above) and its corners in counter-clockwise plan order. A vertical face is a gable end.
#[derive(Clone, Debug, PartialEq)]
pub struct RoofFace {
    pub ring: usize,
    pub edge: usize,
    pub zone: u8,
    pub vertical: bool,
    pub vertices: Vec<Xyz>,
}

/// 🏠️ The surface of a roof: its faces, its lines and the height of its highest point.
#[derive(Clone, Debug, PartialEq)]
pub struct RoofSurface {
    pub faces: Vec<RoofFace>,
    pub lines: Vec<RoofLine>,
    pub height: f64,
}

/// ⚠️ Why a roof surface cannot be built.
#[derive(Clone, Debug, PartialEq)]
pub enum RoofError {
    InvalidPitch,
    InvalidBreak,
    Skeleton(SkeletonError),
}

impl From<SkeletonError> for RoofError {
    fn from(error: SkeletonError) -> Self {
        Self::Skeleton(error)
    }
}

impl std::fmt::Display for RoofError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RoofError {}

fn speed_of(pitch: f64) -> Result<f64, RoofError> {
    if !(pitch.is_finite() && pitch > 0.0 && pitch <= FRAC_PI_2) {
        return Err(RoofError::InvalidPitch);
    }
    Ok(if pitch >= FRAC_PI_2 - 1e-12 { 0.0 } else { 1.0 / pitch.tan() })
}

/// 🏠️ The pitch of every footprint edge of a gable roof: `pitch` for the edges along the ridge, [`VERTICAL`] for the gable ends
/// (edges across `ridge_direction` within `tolerance`, a sine).
pub fn gable_pitches(rings: &[Vec<Point>], pitch: f64, ridge_direction: f64, tolerance: f64) -> Vec<Vec<f64>> {
    let (sin, cos) = ridge_direction.sin_cos();
    rings
        .iter()
        .map(|ring| {
            (0..ring.len())
                .map(|i| {
                    let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                    let length = a.distance(b);
                    let along = ((b.x - a.x) * cos + (b.y - a.y) * sin) / length;
                    if along.abs() <= tolerance { VERTICAL } else { pitch }
                })
                .collect()
        })
        .collect()
}

fn rings_with(rings: &[Vec<Point>], pitches: &[Vec<f64>]) -> Result<Vec<Ring>, RoofError> {
    let mut offset = 0;
    rings
        .iter()
        .zip(pitches)
        .map(|(points, list)| {
            let speeds = list.iter().map(|&pitch| speed_of(pitch)).collect::<Result<Vec<_>, _>>()?;
            let tags = (offset..offset + points.len()).map(|tag| tag as u32).collect();
            offset += points.len();
            Ok(Ring { points: points.clone(), speeds, tags })
        })
        .collect()
}

fn locate(rings: &[Vec<Point>], tag: u32) -> (usize, usize) {
    let mut remaining = tag as usize;
    for (r, ring) in rings.iter().enumerate() {
        if remaining < ring.len() {
            return (r, remaining);
        }
        remaining -= ring.len();
    }
    (0, 0)
}

fn collect(skeleton: &Skeleton, rings: &[Vec<Point>], zone: u8, lift: f64, surface: &mut RoofSurface) {
    let corner = |n: u32| {
        let node = &skeleton.nodes[n as usize];
        [node.point.x, node.point.y, node.time + lift]
    };
    let mut verges: BTreeSet<(u32, u32)> = BTreeSet::new();
    for face in &skeleton.faces {
        let (ring, edge) = locate(rings, face.tag);
        let vertical = face.speed == 0.0;
        if vertical {
            let count = face.nodes.len();
            for i in 1..count {
                let (a, b) = (face.nodes[i], face.nodes[(i + 1) % count]);
                verges.insert((a.min(b), a.max(b)));
            }
        }
        surface.faces.push(RoofFace { ring, edge, zone, vertical, vertices: face.nodes.iter().map(|&n| corner(n)).collect() });
    }
    for arc in &skeleton.arcs {
        let (from, to) = (corner(arc.from), corner(arc.to));
        let kind = if verges.contains(&(arc.from.min(arc.to), arc.from.max(arc.to))) {
            RoofLineKind::Verge
        } else if arc.reflex {
            RoofLineKind::Valley
        } else if (from[2] - to[2]).abs() <= 1e-9 {
            RoofLineKind::Ridge
        } else {
            RoofLineKind::Hip
        };
        surface.lines.push(RoofLine { from, to, kind });
    }
    surface.height = surface.height.max(skeleton.peak() + lift);
}

/// 🏠️ The roof surface of `footprint` (outer ring first, then holes, either orientation) for `roof`; `control` is polled per skeleton
/// event and returns `false` to cancel.
pub fn roof_surface_controlled(footprint: &[Vec<Point>], roof: &Roof, control: &mut dyn FnMut() -> bool) -> Result<RoofSurface, RoofError> {
    let mut surface = RoofSurface { faces: Vec::new(), lines: Vec::new(), height: 0.0 };
    match roof {
        Roof::Mansard { lower_pitch, upper_pitch, break_height } => {
            if !(break_height.is_finite() && *break_height > 0.0) {
                return Err(RoofError::InvalidBreak);
            }
            let lower: Vec<Vec<f64>> = footprint.iter().map(|ring| vec![*lower_pitch; ring.len()]).collect();
            let first = straight_skeleton_until(&rings_with(footprint, &lower)?, Some(*break_height), control)?;
            collect(&first, footprint, 0, 0.0, &mut surface);
            let upper_speed = speed_of(*upper_pitch)?;
            let next: Vec<Ring> = first.wavefront.iter().map(|w| Ring { points: w.points.clone(), speeds: vec![upper_speed; w.points.len()], tags: w.tags.clone() }).collect();
            for wave in &first.wavefront {
                for i in 0..wave.points.len() {
                    let (a, b) = (wave.points[i], wave.points[(i + 1) % wave.points.len()]);
                    surface.lines.push(RoofLine { from: [a.x, a.y, *break_height], to: [b.x, b.y, *break_height], kind: RoofLineKind::Break });
                }
            }
            if !next.is_empty() {
                let second = straight_skeleton_until(&next, None, control)?;
                collect(&second, footprint, 1, *break_height, &mut surface);
            }
        }
        other => {
            let pitches = match other {
                Roof::Hip { pitch } => footprint.iter().map(|ring| vec![*pitch; ring.len()]).collect(),
                Roof::Gable { pitch, ridge_direction } => {
                    speed_of(*pitch)?;
                    gable_pitches(footprint, *pitch, *ridge_direction, GABLE_END_TOLERANCE)
                }
                Roof::Pitched { pitches } => pitches.clone(),
                Roof::Mansard { .. } => unreachable!("handled above"),
            };
            let skeleton = straight_skeleton_until(&rings_with(footprint, &pitches)?, None, control)?;
            collect(&skeleton, footprint, 0, 0.0, &mut surface);
        }
    }
    Ok(surface)
}

/// 🏠️ The roof surface of `footprint` for `roof`, run to the end.
pub fn roof_surface(footprint: &[Vec<Point>], roof: &Roof) -> Result<RoofSurface, RoofError> {
    roof_surface_controlled(footprint, roof, &mut || true)
}

impl RoofSurface {
    /// 🕸️ The faces as a triangle mesh; sloped faces face up, gable ends face outward.
    pub fn mesh(&self) -> TriMesh {
        let mut mesh = TriMesh::new();
        for face in &self.faces {
            let (origin, next) = (face.vertices[0], face.vertices[1]);
            let length = (next[0] - origin[0]).hypot(next[1] - origin[1]);
            let (dx, dy) = ((next[0] - origin[0]) / length, (next[1] - origin[1]) / length);
            let flat: Vec<Point> = if face.vertical {
                face.vertices.iter().map(|v| Point::new((v[0] - origin[0]) * dx + (v[1] - origin[1]) * dy, v[2])).collect()
            } else {
                face.vertices.iter().map(|v| Point::new(v[0], v[1])).collect()
            };
            for tri in &triangulate(&flat, &[]).triangles {
                let [a, b, c] = tri.map(|i| face.vertices[i as usize]);
                mesh.push_triangle(a, b, c);
            }
        }
        mesh
    }

    /// 🧱️ The sloped faces thickened `thickness` vertically downward into a closed solid (a roof layer): the top surface, the bottom
    /// surface and a vertical wall along every boundary edge (eaves and verges). Its volume is `thickness * plan_area()`. A layer
    /// below another one is this solid translated down by the thickness of the layers above it.
    pub fn shell(&self, thickness: f64) -> TriMesh {
        let quantise = |p: Xyz| [(p[0] / 1e-9).round() as i64, (p[1] / 1e-9).round() as i64, (p[2] / 1e-9).round() as i64];
        let down = |p: Xyz| [p[0], p[1], p[2] - thickness];
        let mut mesh = TriMesh::new();
        let mut edges: BTreeMap<([i64; 3], [i64; 3]), (Xyz, Xyz)> = BTreeMap::new();
        for face in self.faces.iter().filter(|face| !face.vertical) {
            let plan: Vec<Point> = face.vertices.iter().map(|v| Point::new(v[0], v[1])).collect();
            for tri in &triangulate(&plan, &[]).triangles {
                let [a, b, c] = tri.map(|i| face.vertices[i as usize]);
                mesh.push_triangle(a, b, c);
                mesh.push_triangle(down(c), down(b), down(a));
            }
            for i in 0..face.vertices.len() {
                let (p, q) = (face.vertices[i], face.vertices[(i + 1) % face.vertices.len()]);
                edges.insert((quantise(p), quantise(q)), (p, q));
            }
        }
        for ((from, to), (p, q)) in &edges {
            if !edges.contains_key(&(*to, *from)) {
                mesh.push_quad(p.to_owned(), down(*p), down(*q), q.to_owned());
            }
        }
        mesh
    }

    /// 📐️ Plan area covered by the sloped faces (the gable ends have none).
    pub fn plan_area(&self) -> f64 {
        self.faces.iter().filter(|face| !face.vertical).map(|face| plan_area_of(&face.vertices)).sum()
    }

    /// 📐️ Area of the roof surface in space: sloped faces and gable ends.
    pub fn surface_area(&self) -> f64 {
        self.mesh().surface_area()
    }
}

fn plan_area_of(vertices: &[Xyz]) -> f64 {
    (0..vertices.len()).map(|i| vertices[i][0] * vertices[(i + 1) % vertices.len()][1] - vertices[(i + 1) % vertices.len()][0] * vertices[i][1]).sum::<f64>() / 2.0
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
