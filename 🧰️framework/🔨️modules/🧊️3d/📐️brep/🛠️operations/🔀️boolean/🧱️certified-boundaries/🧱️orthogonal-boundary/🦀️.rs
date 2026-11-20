//! 🧱️ Bounded exact boundary of the orthogonal cell arrangement of two certified boxes.

use super::{empty_result, unsupported, ValueError};
use crate::BooleanOp;
use semio_framework_3d_brep_construction::primitives::{PrimitiveEdge, PrimitiveTrim, planar_pcurve};
use semio_framework_3d_brep_curve::Curve3;
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use semio_framework_3d_brep_vector::matrix::Frame3;
use semio_framework_3d_brep_vector::{Pnt3, Vec3};

const CELL_COUNT: usize = 27;
const VERTEX_COUNT: usize = 64;
const EDGE_COUNT: usize = 144;
const FACE_COUNT: usize = 162;
const SHELL_COUNT: usize = CELL_COUNT;
pub const MAX_ORTHOGONAL_ENTITIES: usize = VERTEX_COUNT + 2 * EDGE_COUNT + 11 * FACE_COUNT + SHELL_COUNT + 1;

#[derive(Default, Clone, Copy)]
struct Quad { vertices: [u8; 4], edges: [u8; 4], axis: u8, positive: bool }

pub struct OrthogonalBoundary {
    axes: [[f64; 4]; 3],
    vertices: [u8; VERTEX_COUNT],
    edges: [[u8; 2]; EDGE_COUNT],
    faces: [Quad; FACE_COUNT],
    shells: [usize; SHELL_COUNT],
    shells_used: usize,
    vertices_used: usize,
    edges_used: usize,
    faces_used: usize,
}

fn address([x,y,z]: [usize;3], length: usize) -> usize { x + length * (y + length * z) }
fn decode(index: usize, length: usize) -> [usize;3] { [index % length, (index / length) % length, index / (length * length)] }

fn connected_cells(occupied: &[bool; CELL_COUNT]) -> bool {
    let Some(first) = occupied.iter().position(|active| *active) else { return false; };
    let mut pending = [0; CELL_COUNT];
    let mut seen = [false; CELL_COUNT];
    pending[0] = first;
    seen[first] = true;
    let (mut index, mut total) = (0,1);
    while index < total {
        let cell = decode(pending[index],3);
        for axis in 0..3 {
            for next in [cell[axis].checked_sub(1), (cell[axis] < 2).then_some(cell[axis]+1)].into_iter().flatten() {
                let mut neighbor = cell;
                neighbor[axis] = next;
                let id = address(neighbor,3);
                if occupied[id] && !seen[id] {
                    seen[id] = true;
                    pending[total] = id;
                    total += 1;
                }
            }
        }
        index += 1;
    }
    total == occupied.iter().filter(|active| **active).count()
}

type Bounds = [[f64; 3]; 2];
type Axes = [[f64; 4]; 3];

fn ordered([lo, hi]: Bounds) -> bool {
    lo.into_iter().zip(hi).all(|(lo, hi)| lo.is_finite() && hi.is_finite() && lo < hi)
}

fn contains_cell(axes: &Axes, cell: [usize; 3], [lo, hi]: Bounds) -> bool {
    (0..3).all(|i| axes[i][cell[i]] >= lo[i] && axes[i][cell[i] + 1] <= hi[i])
}

fn axis_count(axis: &[f64; 4]) -> usize {
    axis.iter().take_while(|value| value.is_finite()).count()
}

impl OrthogonalBoundary {
    pub fn new(a: Bounds, b: Bounds, operation: BooleanOp) -> Result<Self, ValueError> {
        if !ordered(a) || !ordered(b) { return Err(unsupported()); }
        let axes = std::array::from_fn(|axis| {
            let mut planes = [a[0][axis], a[1][axis], b[0][axis], b[1][axis]];
            planes.sort_unstable_by(f64::total_cmp);
            let mut count = 1;
            for i in 1..4 {
                if planes[i] != planes[count - 1] { planes[count] = planes[i]; count += 1; }
            }
            planes[count..].fill(f64::NAN);
            planes
        });
        Self::from_cells(axes, |axes, cell| {
            let (a, b) = (contains_cell(axes, cell, a), contains_cell(axes, cell, b));
            match operation { BooleanOp::Unite => a || b, BooleanOp::Cut => a && !b, BooleanOp::Intersect => a && b }
        })
    }

    /// 🔪️ Subtracts a bounded union of box tools from one certified rectangular stock.
    pub fn cut_tools(stock: Bounds, tools: &[Bounds]) -> Result<Self, ValueError> {
        if !(1..=2).contains(&tools.len()) || !ordered(stock) || !tools.iter().copied().all(ordered) { return Err(unsupported()); }
        let mut clipped = [stock; 2];
        let mut used = 0;
        for &[lo, hi] in tools {
            let bounds = [std::array::from_fn(|i| lo[i].max(stock[0][i])), std::array::from_fn(|i| hi[i].min(stock[1][i]))];
            if ordered(bounds) { clipped[used] = bounds; used += 1; }
        }
        let mut relevant = &clipped[..used];
        if let [a, b] = relevant {
            let contains = |outer: &Bounds, inner: &Bounds| (0..3).all(|i| outer[0][i] <= inner[0][i] && outer[1][i] >= inner[1][i]);
            if contains(a, b) { relevant = std::slice::from_ref(a); }
            else if contains(b, a) { relevant = std::slice::from_ref(b); }
        }
        let mut axes = std::array::from_fn(|i| [stock[0][i], stock[1][i], f64::NAN, f64::NAN]);
        for tool in relevant {
            for (axis, planes) in axes.iter_mut().enumerate() {
                for value in [tool[0][axis], tool[1][axis]] {
                    if planes.contains(&value) { continue; }
                    let next = planes.iter_mut().find(|value| value.is_nan()).ok_or_else(unsupported)?;
                    *next = value;
                }
            }
        }
        axes.iter_mut().for_each(|planes| planes.sort_unstable_by(f64::total_cmp));
        Self::from_cells(axes, |axes, cell| !relevant.iter().any(|&tool| contains_cell(axes, cell, tool)))
    }

    fn from_cells(axes: Axes, keeps: impl Fn(&Axes, [usize; 3]) -> bool) -> Result<Self, ValueError> {
        let lengths = axes.each_ref().map(|axis| axis_count(axis) - 1);
        let occupied: [bool; CELL_COUNT] = std::array::from_fn(|index| {
            let cell = decode(index, 3);
            (0..3).all(|i| cell[i] < lengths[i]) && keeps(&axes, cell)
        });
        if !occupied.contains(&true) { return Err(empty_result()); }
        if !connected_cells(&occupied) { return Err(unsupported()); }
        let mut result = Self {
            axes, vertices:[0;VERTEX_COUNT], edges:[[0;2];EDGE_COUNT], faces:[Quad::default();FACE_COUNT],
            vertices_used:0, edges_used:0, faces_used:0, shells:[0;SHELL_COUNT], shells_used:0,
        };
        let mut vertex_ids = [u8::MAX; VERTEX_COUNT];
        let mut edge_ids = [u8::MAX; 3 * VERTEX_COUNT];
        let mut edge_uses = [[0_u8; 2]; EDGE_COUNT];
        for (index, active) in occupied.into_iter().enumerate() {
            if !active { continue; }
            let cell = decode(index,3);
            if (0..3).any(|axis| {
                let size = axes[axis][cell[axis]+1] - axes[axis][cell[axis]];
                !size.is_finite() || size <= Tol::DEFAULT.value()
            }) { return Err(unsupported()); }
            for axis in 0..3 {
                for positive in [false, true] {
                    let next = if positive { (cell[axis]+1 < lengths[axis]).then_some(cell[axis]+1) } else { cell[axis].checked_sub(1) };
                    if next.is_some_and(|value| {
                        let mut adjacent = cell;
                        adjacent[axis] = value;
                        occupied[address(adjacent,3)]
                    }) { continue; }
                    let (u,v) = ((axis+1)%3,(axis+2)%3);
                    let mut corners = [[0_u8;3];4];
                    let mut quad = Quad { axis:axis as u8, positive, ..Quad::default() };
                    for (member, [du,dv]) in [[0,0],[1,0],[1,1],[0,1]].into_iter().enumerate() {
                        let (du,dv) = if positive { (du,dv) } else { (dv,du) };
                        let mut point = cell;
                        point[axis] += usize::from(positive);
                        point[u] += du;
                        point[v] += dv;
                        corners[member] = point.map(|x| x as u8);
                        let index = address(point,4);
                        let id = &mut vertex_ids[index];
                        if *id == u8::MAX {
                            *id = result.vertices_used as u8;
                            result.vertices[result.vertices_used] = index as u8;
                            result.vertices_used += 1;
                        }
                        quad.vertices[member] = *id;
                    }
                    for member in 0..4 {
                        let next = (member+1)%4;
                        let p = corners[member];
                        let q = corners[next];
                        let direction = (0..3).find(|&i| p[i] != q[i]).expect("orthogonal quad edge");
                        let forward = p[direction] < q[direction];
                        let (start, end) = if forward { (member,next) } else { (next,member) };
                        let global = direction * VERTEX_COUNT + address(corners[start].map(usize::from),4);
                        let id = &mut edge_ids[global];
                        if *id == u8::MAX {
                            *id = result.edges_used as u8;
                            result.edges[result.edges_used] = [quad.vertices[start], quad.vertices[end]];
                            result.edges_used += 1;
                        }
                        quad.edges[member] = *id;
                        edge_uses[usize::from(*id)][usize::from(forward)] += 1;
                    }
                    result.faces[result.faces_used] = quad;
                    result.faces_used += 1;
                }
            }
        }
        if edge_uses[..result.edges_used].iter().any(|count| *count != [1,1]) { return Err(unsupported()); }
        result.partition_shells()?;
        Ok(result)
    }

    fn partition_shells(&mut self) -> Result<(), ValueError> {
        let faces = &self.faces[..self.faces_used];
        let mut neighbors = [[u8::MAX;2]; EDGE_COUNT];
        for (id, face) in faces.iter().enumerate() {
            for &edge in &face.edges {
                let row = &mut neighbors[usize::from(edge)];
                if row[0] == u8::MAX { row[0] = id as u8; } else { row[1] = id as u8; }
            }
        }
        let walk = |first: usize, vertex: Option<u8>, seen: &mut [bool; FACE_COUNT], pending: &mut [u8; FACE_COUNT]| {
            pending[0] = first as u8;
            seen[first] = true;
            let (mut index,mut total) = (0,1);
            while index < total {
                let id = pending[index];
                for &edge in &faces[usize::from(id)].edges {
                    if vertex.is_some_and(|v| !self.edges[usize::from(edge)].contains(&v)) { continue; }
                    let [a,b] = neighbors[usize::from(edge)];
                    let next = usize::from(if a == id { b } else { a });
                    if !seen[next] {
                        seen[next] = true;
                        pending[total] = next as u8;
                        total += 1;
                    }
                }
                index += 1;
            }
            total
        };
        let mut pending = [0_u8; FACE_COUNT];
        for vertex in 0..self.vertices_used as u8 {
            let counts = faces.iter().filter(|face| face.vertices.contains(&vertex)).count();
            let first = faces.iter().position(|face| face.vertices.contains(&vertex)).expect("owned orthogonal vertex");
            if walk(first,Some(vertex),&mut [false; FACE_COUNT],&mut pending) != counts { return Err(unsupported()); }
        }
        let mut ordered = [Quad::default(); FACE_COUNT];
        let mut seen = [false; FACE_COUNT];
        let mut start = 0;
        for first in 0..faces.len() {
            if seen[first] { continue; }
            let count = walk(first,None,&mut seen,&mut pending);
            let shell = self.shells.get_mut(self.shells_used).ok_or_else(unsupported)?;
            *shell = count;
            self.shells_used += 1;
            for index in 0..count { ordered[start+index] = faces[usize::from(pending[index])]; }
            start += count;
        }
        self.faces = ordered;
        Ok(())
    }

    pub fn shell_sizes(&self) -> &[usize] { &self.shells[..self.shells_used] }
    pub fn vertex_count(&self) -> usize { self.vertices_used }
    pub fn edge_count(&self) -> usize { self.edges_used }
    pub fn ring_sizes(&self) -> &[usize] { &[4; FACE_COUNT][..self.faces_used] }
    pub fn vertex(&self, i: usize) -> Pnt3 {
        let [x,y,z] = decode(usize::from(self.vertices[i]),4);
        Pnt3::new(self.axes[0][x],self.axes[1][y],self.axes[2][z])
    }
    pub fn edge(&self, i: usize) -> PrimitiveEdge {
        let vertices = self.edges[i].map(usize::from);
        let [a,b] = vertices.map(|id| self.vertex(id));
        PrimitiveEdge { vertices, curve: Curve3::Line { origin:a, dir:b-a }, range:(0.0,1.0) }
    }
    pub fn surface(&self, i: usize) -> Surface {
        let face = self.faces[i];
        let axis = [Vec3::X,Vec3::Y,Vec3::Z][usize::from(face.axis)];
        let normal = if face.positive { axis } else { -axis };
        Surface::Plane { frame: Frame3::from_normal(self.vertex(usize::from(face.vertices[0])),normal).expect("grid normal") }
    }
    pub fn trim(&self, i: usize, member: usize) -> PrimitiveTrim {
        let face = self.faces[i];
        let edge = usize::from(face.edges[member]);
        let spec = self.edge(edge);
        let Surface::Plane { frame } = self.surface(i) else { unreachable!("planar orthogonal face") };
        PrimitiveTrim {
            edge, forward: spec.vertices[0] == usize::from(face.vertices[member]),
            curve: planar_pcurve(&spec.curve, frame), range: spec.range,
        }
    }
}
