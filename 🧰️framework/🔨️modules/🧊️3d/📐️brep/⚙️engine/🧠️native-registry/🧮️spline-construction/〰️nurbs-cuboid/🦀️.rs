//! 〰️ Exact degree-one spline storage for the existing cuboid topology recipe.

use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_vector::Pnt3;
use semio_framework_value::{RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use std::mem::size_of;
use semio_framework_3d_brep_curve::{Curve3, bspline::KnotVector};

/// 📐️ Fixed degree-one spline parameters for one cuboid edge or face.
pub enum Recipe {
    Edge { knots: [f64; 4], controls: [Pnt3; 2] },
    Face { knots: [[f64; 4]; 2], controls: [[Pnt3; 2]; 2] },
}

fn knots(lo: f64, hi: f64) -> [f64; 4] { [lo, lo, hi, hi] }

impl Recipe {
    pub fn is_finite(&self) -> bool {
        let axis = |knots: &[f64; 4]| knots[0].is_finite() && knots[3].is_finite() && knots[0] < knots[3];
        let point = |point: &Pnt3| point.to_array().into_iter().all(f64::is_finite);
        match self {
            Self::Edge { knots, controls } => axis(knots) && controls.iter().all(point),
            Self::Face { knots, controls } => knots.iter().all(axis) && controls.iter().flatten().all(point),
        }
    }

    pub fn edge(box_: SolidPrimitive, index: usize) -> Self {
        let edge = box_.edge(index);
        Self::Edge {
            knots: knots(edge.range.0, edge.range.1),
            controls: edge.vertices.map(|vertex| box_.vertex(vertex)),
        }
    }

    pub fn face(box_: SolidPrimitive, index: usize) -> Self {
        let surface = box_.surface(index);
        let (mut lo, mut hi) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
        for row in 0..4 {
            let trim = box_.trim(index, row);
            for t in [trim.range.0, trim.range.1] {
                let uv = trim.curve.eval(t);
                lo = [lo[0].min(uv.x), lo[1].min(uv.y)];
                hi = [hi[0].max(uv.x), hi[1].max(uv.y)];
            }
        }
        Self::Face {
            knots: [knots(lo[0], hi[0]), knots(lo[1], hi[1])],
            controls: [lo[0], hi[0]].map(|u| [lo[1], hi[1]].map(|v| surface.eval(u, v))),
        }
    }
}

/// 📦️ Funded edge or face buffers retained until topology publication.
#[derive(Default)]
pub enum Payload {
    #[default]
    Empty,
    Edge(Curve3),
    Face(Surface),
}

fn vector_bytes<T>(rows: &Vec<T>) -> usize { rows.capacity() * size_of::<T>() }

fn grid_bytes<T>(rows: &Vec<Vec<T>>) -> usize {
    vector_bytes(rows) + rows.iter().map(vector_bytes).sum::<usize>()
}

pub fn curve_bytes(curve: &Curve3) -> usize {
    match curve {
        Curve3::Nurbs { knots, controls, weights } => vector_bytes(&knots.knots) + vector_bytes(controls) + vector_bytes(weights),
        _ => 0,
    }
}

pub fn surface_bytes(surface: &Surface) -> usize {
    match surface {
        Surface::Nurbs { u_knots, v_knots, controls, weights } =>
            vector_bytes(&u_knots.knots) + vector_bytes(&v_knots.knots) + grid_bytes(controls) + grid_bytes(weights),
        _ => 0,
    }
}

fn demand<T>(count: usize, copy: bool) -> RetirementDemand {
    let capacity_bytes = count * size_of::<T>();
    RetirementDemand { capacity_bytes, copy_bytes: if copy { capacity_bytes } else { 0 }, depth: 1, ..Default::default() }
}

fn fill<T: Copy>(rows: &mut Vec<T>, values: &[T]) -> Result<usize, ValueError> {
    reserve(rows, values.len())?;
    rows.extend_from_slice(values);
    Ok(vector_bytes(rows))
}

fn reserve<T>(rows: &mut Vec<T>, count: usize) -> Result<usize, ValueError> {
    rows.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "cuboid spline storage allocation failed"))?;
    Ok(vector_bytes(rows))
}

fn row<T: Copy>(outer: &mut Vec<Vec<T>>, values: [T; 2]) -> Result<usize, ValueError> {
    let mut rows = Vec::new();
    let bytes = fill(&mut rows, &values)?;
    outer.push(rows);
    Ok(bytes)
}

impl Payload {
    pub fn demand(&self) -> Option<RetirementDemand> {
        match self {
            Self::Empty => Some(demand::<()>(0, false)),
            Self::Edge(Curve3::Nurbs { knots, .. }) if knots.knots.is_empty() => Some(demand::<f64>(4, true)),
            Self::Edge(Curve3::Nurbs { controls, .. }) if controls.is_empty() => Some(demand::<Pnt3>(2, true)),
            Self::Edge(Curve3::Nurbs { weights, .. }) if weights.is_empty() => Some(demand::<f64>(2, true)),
            Self::Face(Surface::Nurbs { u_knots, v_knots, .. }) if u_knots.knots.is_empty() || v_knots.knots.is_empty() => Some(demand::<f64>(4, true)),
            Self::Face(Surface::Nurbs { controls, .. }) if controls.capacity() == 0 => Some(demand::<Vec<Pnt3>>(2, false)),
            Self::Face(Surface::Nurbs { controls, .. }) if controls.len() < 2 => Some(demand::<Pnt3>(2, true)),
            Self::Face(Surface::Nurbs { weights, .. }) if weights.capacity() == 0 => Some(demand::<Vec<f64>>(2, false)),
            Self::Face(Surface::Nurbs { weights, .. }) if weights.len() < 2 => Some(demand::<f64>(2, true)),
            _ => None,
        }
    }

    pub fn retained_bytes(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Edge(value) => curve_bytes(value),
            Self::Face(value) => surface_bytes(value),
        }
    }

    pub fn prepare(&mut self, recipe: Recipe) -> Result<Progress, ValueError> {
        let quoted = self.demand().ok_or_else(|| ValueError::invariant("cuboid spline row is already complete"))?;
        let degree_one = || KnotVector { degree: 1, knots: Vec::new() };
        let retained_capacity_bytes = match (self, recipe) {
            (state @ Self::Empty, Recipe::Edge { .. }) => {
                *state = Self::Edge(Curve3::Nurbs { knots: degree_one(), controls: Vec::new(), weights: Vec::new() });
                0
            }
            (state @ Self::Empty, Recipe::Face { .. }) => {
                *state = Self::Face(Surface::Nurbs { u_knots: degree_one(), v_knots: degree_one(), controls: Vec::new(), weights: Vec::new() });
                0
            }
            (Self::Edge(Curve3::Nurbs { knots, controls, weights }), Recipe::Edge { knots: values, controls: points }) => {
                if knots.knots.is_empty() { fill(&mut knots.knots, &values)? }
                else if controls.is_empty() { fill(controls, &points)? }
                else { fill(weights, &[1.0; 2])? }
            }
            (Self::Face(Surface::Nurbs { u_knots, v_knots, controls, weights }), Recipe::Face { knots, controls: points }) => {
                if u_knots.knots.is_empty() { fill(&mut u_knots.knots, &knots[0])? }
                else if v_knots.knots.is_empty() { fill(&mut v_knots.knots, &knots[1])? }
                else if controls.capacity() == 0 { reserve(controls, 2)? }
                else if controls.len() < 2 { let index = controls.len(); row(controls, points[index])? }
                else if weights.capacity() == 0 { reserve(weights, 2)? }
                else { row(weights, [1.0; 2])? }
            }
            _ => return Err(ValueError::invariant("cuboid spline row and topology step diverged")),
        };
        Ok(Progress { copied_items: 1, copied_bytes: quoted.copy_bytes, retained_capacity_bytes, ..Default::default() })
    }
}
