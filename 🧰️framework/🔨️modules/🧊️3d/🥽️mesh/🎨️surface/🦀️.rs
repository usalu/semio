//! 🎨️ Retained surface work: shading flags, vertex normals, UV seams and UV unwrapping as budgeted jobs over an owned mesh.
//!
//! A job owns the mesh it edits and advances in bounded units of at most [`CHUNK`] elements, so a host can run it inside an interactive step
//! ceiling, report monotone progress and cancel it. Every ordering is by element id, so a job is a pure function of its inputs.
//!
//! 🔗️ [Least squares conformal maps](https://doi.org/10.1145/566654.566590)

use super::*;
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

const CHUNK: usize = 256;

type Complex = [f64; 2];

fn product(a: Complex, b: Complex) -> Complex {
    [a[0] * b[0] - a[1] * b[1], a[0] * b[1] + a[1] * b[0]]
}

fn conjugate_product(a: Complex, b: Complex) -> Complex {
    [a[0] * b[0] + a[1] * b[1], a[0] * b[1] - a[1] * b[0]]
}

fn combine(a: Complex, scale: f64, b: Complex) -> Complex {
    [a[0] + scale * b[0], a[1] + scale * b[1]]
}

/// 🧵️ A budgeted surface job; completed geometry is transferred exactly once.
pub struct MeshSurfaceJob {
    work: Work,
    progress: MeshModelingProgress,
    cancelled: bool,
    retired: bool,
}

enum Work {
    Basic(Box<Basic>),
    Unwrap(Box<Unwrap>),
}

impl MeshSurfaceJob {
    /// 📊 Completed units and a conservative remaining-work estimate.
    pub fn progress(&self) -> MeshModelingProgress {
        self.progress
    }

    /// 🛑 Retires the job at its next unit boundary.
    pub fn cancel(&mut self) {
        if !self.retired {
            self.cancelled = true;
        }
    }

    pub(super) fn finish_with_progress(mut self) -> MeshResult<HalfedgeMesh> {
        loop {
            match self.step(1024)? {
                MeshModelingStep::Working(_) => {}
                MeshModelingStep::Done(mesh) => return Ok(mesh),
                MeshModelingStep::Cancelled(_) => return Err(MeshKernelError::InvalidInput("operation cancelled".into())),
            }
        }
    }

    /// ⏱️ Advances at most `budget` units.
    pub fn step(&mut self, budget: usize) -> MeshResult<MeshModelingStep> {
        if self.retired {
            return Err(MeshKernelError::InvalidInput("surface job already retired".into()));
        }
        if self.cancelled {
            self.retired = true;
            return Ok(MeshModelingStep::Cancelled(self.progress));
        }
        for _ in 0..budget {
            let (output, phase, remaining) = match &mut self.work {
                Work::Basic(work) => match work.advance() {
                    Ok(output) => (output, work.phase(), work.remaining()),
                    Err(error) => {
                        self.retired = true;
                        return Err(error);
                    }
                },
                Work::Unwrap(work) => match work.advance() {
                    Ok(output) => (output, work.phase(), work.remaining()),
                    Err(error) => {
                        self.retired = true;
                        return Err(error);
                    }
                },
            };
            self.progress.units_done = self.progress.units_done.saturating_add(1);
            self.progress.phase = phase;
            if let Some(mesh) = output {
                self.progress.units_total = self.progress.units_done;
                self.progress.phase = "done";
                self.retired = true;
                return Ok(MeshModelingStep::Done(mesh));
            }
            self.progress.units_total = self.progress.units_done.saturating_add(remaining.max(1));
        }
        Ok(MeshModelingStep::Working(self.progress))
    }
}

//#region 🔖️Basic
#[derive(Clone, Copy, PartialEq, Eq)]
enum Plan {
    Shading,
    Normals,
    Seams,
}

struct Basic {
    mesh: HalfedgeMesh,
    plan: Plan,
    faces: Vec<FaceId>,
    edges: Vec<EdgeId>,
    flag: bool,
    phase: u8,
    cursor: usize,
    flat: Vec<Option<Vec3>>,
    sums: Vec<Vec3>,
}

impl Basic {
    fn phase(&self) -> &'static str {
        match self.phase {
            0 => "surface-validate",
            1 => "surface-apply",
            2 => "normals-faces",
            _ => "normals-vertices",
        }
    }

    fn remaining(&self) -> usize {
        let chunks = |total: usize| total.saturating_sub(self.cursor).div_ceil(CHUNK);
        match self.phase {
            0 | 1 => chunks(self.faces.len().max(self.edges.len())) + 1,
            2 => chunks(self.mesh.face_count()) + self.mesh.vertex_count().div_ceil(CHUNK),
            _ => chunks(self.mesh.vertex_count()),
        }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match (self.plan, self.phase) {
            (Plan::Shading, 0) => {
                let end = (self.cursor + CHUNK).min(self.faces.len());
                for face in &self.faces[self.cursor..end] {
                    self.mesh.face_vertex_ids(*face)?;
                }
                self.cursor = end;
                if self.cursor == self.faces.len() {
                    self.phase = 1;
                    self.cursor = 0;
                }
            }
            (Plan::Shading, 1) => {
                let end = (self.cursor + CHUNK).min(self.faces.len());
                for face in &self.faces[self.cursor..end] {
                    self.mesh.faces[face.0 as usize].smooth = self.flag;
                }
                self.cursor = end;
                if self.cursor == self.faces.len() {
                    self.phase = 2;
                    self.cursor = 0;
                }
            }
            (Plan::Seams, 0) => {
                let end = (self.cursor + CHUNK).min(self.edges.len());
                if self.edges[self.cursor..end].iter().any(|edge| edge.0 as usize >= self.mesh.halfedges.len()) {
                    return Err(MeshKernelError::InvalidHandle);
                }
                self.cursor = end;
                if self.cursor == self.edges.len() {
                    self.phase = 1;
                    self.cursor = 0;
                }
            }
            (Plan::Seams, 1) => {
                let end = (self.cursor + CHUNK).min(self.edges.len());
                for edge in self.edges[self.cursor..end].to_vec() {
                    self.mesh.set_seam_pair(edge, self.flag);
                }
                self.cursor = end;
                if self.cursor == self.edges.len() {
                    return Ok(Some(std::mem::replace(&mut self.mesh, HalfedgeMesh::empty())));
                }
            }
            (_, 2) => {
                if self.cursor == 0 {
                    self.flat = vec![None; self.mesh.vertex_count()];
                    self.sums = vec![Vec3::ZERO; self.mesh.vertex_count()];
                }
                let end = (self.cursor + CHUNK).min(self.mesh.face_count());
                self.mesh.accumulate_vertex_normals(self.cursor..end, &mut self.flat, &mut self.sums)?;
                self.cursor = end;
                if self.cursor >= self.mesh.face_count() {
                    self.phase = 3;
                    self.cursor = 0;
                }
            }
            _ => {
                let end = (self.cursor + CHUNK).min(self.mesh.vertex_count());
                self.mesh.apply_vertex_normals(self.cursor..end, &self.flat, &self.sums);
                self.cursor = end;
                if self.cursor >= self.mesh.vertex_count() {
                    return Ok(Some(std::mem::replace(&mut self.mesh, HalfedgeMesh::empty())));
                }
            }
        }
        Ok(None)
    }
}
//#endregion 🔖️Basic

//#region 🔖️Normals
impl HalfedgeMesh {
    pub(super) fn accumulate_vertex_normals(&self, faces: std::ops::Range<usize>, flat: &mut [Option<Vec3>], sums: &mut [Vec3]) -> MeshResult<()> {
        for face in faces {
            let normal = self.face_normal(FaceId(face as u32))?;
            for vertex in self.face_vertex_ids(FaceId(face as u32))? {
                if flat[vertex.0 as usize].is_none() {
                    flat[vertex.0 as usize] = Some(normal);
                }
                if self.faces[face].smooth {
                    sums[vertex.0 as usize] = sums[vertex.0 as usize].add(normal);
                }
            }
        }
        Ok(())
    }

    pub(super) fn apply_vertex_normals(&mut self, vertices: std::ops::Range<usize>, flat: &[Option<Vec3>], sums: &[Vec3]) {
        for id in vertices {
            self.vertices[id].normal = if sums[id].length() > 0.0 { Some(sums[id].normalize().0) } else { flat[id].map(|normal| normal.0) };
        }
    }

    /// 🔗️ Marks or clears the seam on an edge and its twin as one pair, so either half-edge names the seam.
    pub(super) fn set_seam_pair(&mut self, edge: EdgeId, seam: bool) {
        let twin = self.halfedges[edge.0 as usize].twin;
        let canonical = twin.map_or(edge.0, |twin| twin.min(edge.0));
        if seam {
            self.uv_seams.insert(canonical);
        } else {
            self.uv_seams.remove(&edge.0);
            if let Some(twin) = twin {
                self.uv_seams.remove(&twin);
            }
        }
    }

    pub(super) fn is_seam_pair(&self, halfedge: u32) -> bool {
        self.uv_seams.contains(&halfedge) || self.halfedges[halfedge as usize].twin.is_some_and(|twin| self.uv_seams.contains(&twin))
    }

    fn surface_job(self, plan: Plan, faces: Vec<FaceId>, edges: Vec<EdgeId>, flag: bool) -> MeshSurfaceJob {
        let phase = if plan == Plan::Normals { 2 } else { 0 };
        let total = faces.len().max(edges.len()).div_ceil(CHUNK) * 2 + self.face_count().div_ceil(CHUNK) + self.vertex_count().div_ceil(CHUNK) + 1;
        MeshSurfaceJob {
            work: Work::Basic(Box::new(Basic { mesh: self, plan, faces, edges, flag, phase, cursor: 0, flat: Vec::new(), sums: Vec::new() })),
            progress: MeshModelingProgress { units_done: 0, units_total: total, phase: "surface-validate" },
            cancelled: false,
            retired: false,
        }
    }

    /// 🌗️ Marks the faces smooth or flat and rebuilds every vertex normal, as a retained job over a copy of this mesh.
    pub fn set_shading_job(&self, faces: &[FaceId], smooth: bool) -> MeshResult<MeshSurfaceJob> {
        self.clone().set_shading_job_owned(faces, smooth)
    }

    /// 🌗️ [`Self::set_shading_job`] over this mesh itself.
    pub fn set_shading_job_owned(self, faces: &[FaceId], smooth: bool) -> MeshResult<MeshSurfaceJob> {
        if faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        Ok(self.surface_job(Plan::Shading, faces.to_vec(), Vec::new(), smooth))
    }

    /// 🧭️ Rebuilds every vertex normal from the faces and their smooth or flat marks, as a retained job over a copy of this mesh.
    pub fn recompute_normals_job(&self) -> MeshResult<MeshSurfaceJob> {
        self.clone().recompute_normals_job_owned()
    }

    /// 🧭️ [`Self::recompute_normals_job`] over this mesh itself.
    pub fn recompute_normals_job_owned(self) -> MeshResult<MeshSurfaceJob> {
        Ok(self.surface_job(Plan::Normals, Vec::new(), Vec::new(), false))
    }

    /// 🪡️ Marks or clears the edges as UV seams (an edge and its twin are one seam), as a retained job over a copy of this mesh.
    pub fn set_uv_seams_job(&self, edges: &[EdgeId], seam: bool) -> MeshResult<MeshSurfaceJob> {
        self.clone().set_uv_seams_job_owned(edges, seam)
    }

    /// 🪡️ [`Self::set_uv_seams_job`] over this mesh itself.
    pub fn set_uv_seams_job_owned(self, edges: &[EdgeId], seam: bool) -> MeshResult<MeshSurfaceJob> {
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        Ok(self.surface_job(Plan::Seams, Vec::new(), edges.to_vec(), seam))
    }

    /// 🗺️ Cuts the surface along its seams into islands, flattens each island with a least squares conformal map and packs the islands into the unit square, as a retained job over a copy of this mesh.
    pub fn unwrap_uv_job(&self) -> MeshResult<MeshSurfaceJob> {
        self.clone().unwrap_uv_job_owned()
    }

    /// 🗺️ [`Self::unwrap_uv_job`] over this mesh itself; a mesh with an authored UV channel is refused because the unwrapped coordinates would be hidden behind it.
    pub fn unwrap_uv_job_owned(self) -> MeshResult<MeshSurfaceJob> {
        if self.attributes.values().any(|attribute| attribute.semantic == MeshAttributeSemantic::Uv) {
            return Err(MeshKernelError::InvalidInput("the mesh carries an authored UV channel".into()));
        }
        if self.faces.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        let total = self.face_count().div_ceil(CHUNK) * 3 + 8;
        Ok(MeshSurfaceJob { work: Work::Unwrap(Box::new(Unwrap::new(self))), progress: MeshModelingProgress { units_done: 0, units_total: total, phase: "unwrap-islands" }, cancelled: false, retired: false })
    }
}
//#endregion 🔖️Normals

//#region 🔖️Unwrap
struct Triangle {
    corners: [usize; 3],
    weights: [Complex; 3],
}

struct Solve {
    vertices: Vec<u32>,
    triangles: Vec<Triangle>,
    pins: [usize; 2],
    scale: f64,
    diagonal: Vec<f64>,
    x: Vec<Complex>,
    r: Vec<Complex>,
    p: Vec<Complex>,
    z: Vec<Complex>,
    ap: Vec<Complex>,
    stage: u8,
    cursor: usize,
    accumulator: f64,
    rz: f64,
    next: (f64, f64),
    alpha: f64,
    rr_target: f64,
    iterations: usize,
    limit: usize,
}

struct Unwrap {
    mesh: HalfedgeMesh,
    phase: u8,
    cursor: usize,
    visited: Vec<bool>,
    stack: Vec<u32>,
    current: Vec<u32>,
    face_island: Vec<u32>,
    islands: Vec<Vec<u32>>,
    island: usize,
    build: Option<(Vec<u32>, BTreeMap<u32, usize>, Vec<Triangle>, usize)>,
    solve: Option<Solve>,
    local: Vec<BTreeMap<u32, [f64; 2]>>,
    packed: Vec<BTreeMap<u32, [f32; 2]>>,
}

impl Unwrap {
    fn new(mesh: HalfedgeMesh) -> Self {
        let faces = mesh.face_count();
        Self { mesh, phase: 0, cursor: 0, visited: vec![false; faces], stack: Vec::new(), current: Vec::new(), face_island: vec![0; faces], islands: Vec::new(), island: 0, build: None, solve: None, local: Vec::new(), packed: Vec::new() }
    }

    fn phase(&self) -> &'static str {
        match self.phase {
            0 => "unwrap-islands",
            1 => "unwrap-triangles",
            2 => "unwrap-solve",
            3 => "unwrap-pack",
            _ => "unwrap-apply",
        }
    }

    fn remaining(&self) -> usize {
        let faces = self.mesh.face_count();
        match self.phase {
            0 => faces.saturating_sub(self.cursor).div_ceil(CHUNK) * 3 + 8,
            1 | 2 => {
                let pending: usize = self.islands.iter().skip(self.island).map(|island| island.len().div_ceil(CHUNK) + 1).sum();
                let solving = self.solve.as_ref().map_or(0, |solve| solve.limit.saturating_sub(solve.iterations) * (solve.triangles.len().div_ceil(CHUNK) + solve.vertices.len().div_ceil(CHUNK) * 3 + 1));
                pending + solving.min(1 << 20) + faces.div_ceil(CHUNK) + 2
            }
            3 => faces.div_ceil(CHUNK) + 2,
            _ => faces.saturating_sub(self.cursor).div_ceil(CHUNK),
        }
    }

    fn advance(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        match self.phase {
            0 => self.advance_islands(),
            1 => self.advance_build()?,
            2 => self.advance_solve(),
            3 => {
                self.pack();
                self.phase = 4;
                self.cursor = 0;
            }
            _ => return self.advance_apply(),
        }
        Ok(None)
    }

    fn advance_islands(&mut self) {
        let faces = self.mesh.face_count();
        for _ in 0..CHUNK {
            let Some(face) = self.stack.pop() else {
                if !self.current.is_empty() {
                    let mut island = std::mem::take(&mut self.current);
                    island.sort_unstable();
                    self.islands.push(island);
                }
                while self.cursor < faces && self.visited[self.cursor] {
                    self.cursor += 1;
                }
                if self.cursor == faces {
                    self.phase = 1;
                    self.cursor = 0;
                    self.island = 0;
                    return;
                }
                self.visited[self.cursor] = true;
                self.stack.push(self.cursor as u32);
                continue;
            };
            self.face_island[face as usize] = self.islands.len() as u32;
            self.current.push(face);
            let mut halfedge = self.mesh.faces[face as usize].halfedge;
            loop {
                let corner = &self.mesh.halfedges[halfedge as usize];
                if !self.mesh.is_seam_pair(halfedge) {
                    if let Some(neighbour) = corner.twin.and_then(|twin| self.mesh.halfedges[twin as usize].face) {
                        if !self.visited[neighbour as usize] {
                            self.visited[neighbour as usize] = true;
                            self.stack.push(neighbour);
                        }
                    }
                }
                halfedge = corner.next;
                if halfedge == self.mesh.faces[face as usize].halfedge {
                    break;
                }
            }
        }
    }

    fn advance_build(&mut self) -> MeshResult<()> {
        if self.island == self.islands.len() {
            self.phase = 3;
            return Ok(());
        }
        let island = &self.islands[self.island];
        let (vertices, index, triangles, next) = self.build.get_or_insert_with(|| (Vec::new(), BTreeMap::new(), Vec::new(), 0));
        let end = (*next + CHUNK).min(island.len());
        for face in &island[*next..end] {
            let corners = self.mesh.face_vertex_ids(FaceId(*face))?;
            let positions: Vec<Vec3> = corners.iter().map(|vertex| Vec3(self.mesh.vertices[vertex.0 as usize].position)).collect();
            let local: Vec<usize> = corners
                .iter()
                .map(|vertex| {
                    *index.entry(vertex.0).or_insert_with(|| {
                        vertices.push(vertex.0);
                        vertices.len() - 1
                    })
                })
                .collect();
            for triangle in triangulate_polygon(&positions) {
                if let Some(weights) = conformal_weights([positions[triangle[0]], positions[triangle[1]], positions[triangle[2]]]) {
                    triangles.push(Triangle { corners: [local[triangle[0]], local[triangle[1]], local[triangle[2]]], weights });
                }
            }
        }
        *next = end;
        if *next < island.len() {
            return Ok(());
        }
        let (vertices, _, triangles, _) = self.build.take().expect("build state");
        self.solve = Some(Solve::new(&self.mesh, vertices, triangles));
        self.phase = 2;
        Ok(())
    }

    fn advance_solve(&mut self) {
        let solve = self.solve.as_mut().expect("solve state");
        if solve.advance() {
            let solve = self.solve.take().expect("solve state");
            self.local.push(solve.coordinates());
            self.island += 1;
            self.phase = 1;
        }
    }

    fn pack(&mut self) {
        let mut boxes: Vec<(usize, [f64; 2], [f64; 2])> = self
            .local
            .iter()
            .enumerate()
            .map(|(island, uvs)| {
                let mut low = [f64::INFINITY; 2];
                let mut high = [f64::NEG_INFINITY; 2];
                for uv in uvs.values() {
                    for axis in 0..2 {
                        low[axis] = low[axis].min(uv[axis]);
                        high[axis] = high[axis].max(uv[axis]);
                    }
                }
                if uvs.is_empty() {
                    low = [0.0; 2];
                    high = [0.0; 2];
                }
                (island, low, high)
            })
            .collect();
        let size = |entry: &(usize, [f64; 2], [f64; 2])| [(entry.2[0] - entry.1[0]).max(1e-9), (entry.2[1] - entry.1[1]).max(1e-9)];
        let area: f64 = boxes.iter().map(|entry| size(entry)[0] * size(entry)[1]).sum();
        let widest = boxes.iter().map(|entry| size(entry)[0]).fold(0.0, f64::max);
        let padding = 0.02 * area.sqrt().max(1e-9);
        let row_width = (area * 1.25).sqrt().max(widest);
        boxes.sort_by(|a, b| size(b)[1].total_cmp(&size(a)[1]).then(a.0.cmp(&b.0)));
        let mut placements = vec![[0.0f64; 2]; boxes.len()];
        let (mut x, mut y, mut row_height, mut extent) = (0.0f64, 0.0f64, 0.0f64, [0.0f64; 2]);
        for (slot, entry) in boxes.iter().enumerate() {
            let [width, height] = size(entry);
            if x > 0.0 && x + width > row_width {
                x = 0.0;
                y += row_height + padding;
                row_height = 0.0;
            }
            placements[slot] = [x, y];
            extent = [extent[0].max(x + width), extent[1].max(y + height)];
            x += width + padding;
            row_height = row_height.max(height);
        }
        let scale = 1.0 / extent[0].max(extent[1]).max(1e-9);
        self.packed = vec![BTreeMap::new(); self.local.len()];
        for (slot, entry) in boxes.iter().enumerate() {
            for (vertex, uv) in &self.local[entry.0] {
                let u = ((placements[slot][0] + uv[0] - entry.1[0]) * scale).clamp(0.0, 1.0);
                let v = ((placements[slot][1] + uv[1] - entry.1[1]) * scale).clamp(0.0, 1.0);
                self.packed[entry.0].insert(*vertex, [u as f32, v as f32]);
            }
        }
    }

    fn advance_apply(&mut self) -> MeshResult<Option<HalfedgeMesh>> {
        let end = (self.cursor + CHUNK).min(self.mesh.face_count());
        for face in self.cursor..end {
            let island = self.face_island[face] as usize;
            let start = self.mesh.faces[face].halfedge;
            let mut halfedge = start;
            loop {
                let vertex = self.mesh.halfedges[halfedge as usize].vertex;
                if let Some(uv) = self.packed[island].get(&vertex) {
                    self.mesh.halfedges[halfedge as usize].uv = *uv;
                }
                halfedge = self.mesh.halfedges[halfedge as usize].next;
                if halfedge == start {
                    break;
                }
            }
        }
        self.cursor = end;
        Ok((self.cursor == self.mesh.face_count()).then(|| std::mem::replace(&mut self.mesh, HalfedgeMesh::empty())))
    }
}

/// 🧮️ The complex conformal weights of one triangle: the energy of its image `z` is `|w0 z0 + w1 z1 + w2 z2|²`; `None` for a degenerate triangle.
fn conformal_weights(corners: [Vec3; 3]) -> Option<[Complex; 3]> {
    let point = |vertex: Vec3| vertex.0.map(f64::from);
    let (p0, p1, p2) = (point(corners[0]), point(corners[1]), point(corners[2]));
    let e1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
    let length = (e1[0] * e1[0] + e1[1] * e1[1] + e1[2] * e1[2]).sqrt();
    let normal = [e1[1] * v[2] - e1[2] * v[1], e1[2] * v[0] - e1[0] * v[2], e1[0] * v[1] - e1[1] * v[0]];
    let normal_length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if length < 1e-12 || normal_length < 1e-12 * length * length {
        return None;
    }
    let x_axis = [e1[0] / length, e1[1] / length, e1[2] / length];
    let x3 = v[0] * x_axis[0] + v[1] * x_axis[1] + v[2] * x_axis[2];
    let y3 = normal_length / length;
    let root = (length * y3).sqrt();
    Some([[(x3 - length) / root, y3 / root], [-x3 / root, -y3 / root], [length / root, 0.0]])
}

impl Solve {
    fn new(mesh: &HalfedgeMesh, vertices: Vec<u32>, triangles: Vec<Triangle>) -> Self {
        let count = vertices.len();
        let position = |local: usize| mesh.vertices[vertices[local] as usize].position.map(f64::from);
        let mut far = (0usize, 0.0f64);
        if count > 1 {
            let origin = position(0);
            for local in 1..count {
                let other = position(local);
                let distance = (0..3).map(|axis| (other[axis] - origin[axis]).powi(2)).sum::<f64>();
                if distance > far.1 {
                    far = (local, distance);
                }
            }
        }
        let pins = [0, far.0];
        let scale = far.1.sqrt();
        let mut diagonal = vec![0.0f64; count];
        for triangle in &triangles {
            for (corner, weight) in triangle.corners.iter().zip(&triangle.weights) {
                diagonal[*corner] += weight[0] * weight[0] + weight[1] * weight[1];
            }
        }
        let mut solve = Self {
            vertices,
            triangles,
            pins,
            scale,
            diagonal,
            x: vec![[0.0; 2]; count],
            r: vec![[0.0; 2]; count],
            p: vec![[0.0; 2]; count],
            z: vec![[0.0; 2]; count],
            ap: vec![[0.0; 2]; count],
            stage: 0,
            cursor: 0,
            accumulator: 0.0,
            rz: 0.0,
            next: (0.0, 0.0),
            alpha: 0.0,
            rr_target: 0.0,
            iterations: 0,
            limit: (4 * count).clamp(100, 20_000),
        };
        solve.initialise();
        solve
    }

    fn free(&self, local: usize) -> bool {
        local != self.pins[0] && local != self.pins[1] && self.diagonal[local] > 0.0
    }

    fn initialise(&mut self) {
        let count = self.vertices.len();
        let mut pinned = vec![[0.0f64; 2]; count];
        if self.pins[1] != self.pins[0] {
            pinned[self.pins[1]] = [1.0, 0.0];
        }
        let mut right = vec![[0.0f64; 2]; count];
        for triangle in &self.triangles {
            let sum = (0..3).fold([0.0; 2], |sum, k| combine(sum, 1.0, product(triangle.weights[k], pinned[triangle.corners[k]])));
            for k in 0..3 {
                right[triangle.corners[k]] = combine(right[triangle.corners[k]], -1.0, conjugate_product(triangle.weights[k], sum));
            }
        }
        let mut rz = 0.0;
        let mut rr = 0.0;
        for local in 0..count {
            if !self.free(local) {
                continue;
            }
            self.r[local] = right[local];
            self.z[local] = [right[local][0] / self.diagonal[local], right[local][1] / self.diagonal[local]];
            self.p[local] = self.z[local];
            rz += right[local][0] * self.z[local][0] + right[local][1] * self.z[local][1];
            rr += right[local][0] * right[local][0] + right[local][1] * right[local][1];
        }
        self.rz = rz;
        self.rr_target = rr * 1e-18;
        self.stage = if rr == 0.0 { 4 } else { 0 };
    }

    /// 🔁️ One unit of the conjugate gradient solve; true once the island is solved.
    fn advance(&mut self) -> bool {
        let count = self.vertices.len();
        match self.stage {
            0 => {
                let end = (self.cursor + CHUNK).min(self.triangles.len());
                for triangle in &self.triangles[self.cursor..end] {
                    let masked = |k: usize| {
                        let local = triangle.corners[k];
                        if local != self.pins[0] && local != self.pins[1] && self.diagonal[local] > 0.0 { self.p[local] } else { [0.0; 2] }
                    };
                    let sum = (0..3).fold([0.0; 2], |sum, k| combine(sum, 1.0, product(triangle.weights[k], masked(k))));
                    for k in 0..3 {
                        let local = triangle.corners[k];
                        self.ap[local] = combine(self.ap[local], 1.0, conjugate_product(triangle.weights[k], sum));
                    }
                }
                self.cursor = end;
                if self.cursor == self.triangles.len() {
                    self.stage = 1;
                    self.cursor = 0;
                    self.accumulator = 0.0;
                }
            }
            1 => {
                let end = (self.cursor + CHUNK).min(count);
                for local in self.cursor..end {
                    if self.free(local) {
                        self.accumulator += self.p[local][0] * self.ap[local][0] + self.p[local][1] * self.ap[local][1];
                    }
                }
                self.cursor = end;
                if self.cursor == count {
                    if self.accumulator <= 0.0 || !self.accumulator.is_finite() {
                        return true;
                    }
                    self.alpha = self.rz / self.accumulator;
                    self.stage = 2;
                    self.cursor = 0;
                    self.accumulator = 0.0;
                    self.next = (0.0, 0.0);
                }
            }
            2 => {
                let end = (self.cursor + CHUNK).min(count);
                let (mut rz, mut rr) = (self.next.0, self.next.1);
                for local in self.cursor..end {
                    if !self.free(local) {
                        continue;
                    }
                    self.x[local] = combine(self.x[local], self.alpha, self.p[local]);
                    self.r[local] = combine(self.r[local], -self.alpha, self.ap[local]);
                    self.z[local] = [self.r[local][0] / self.diagonal[local], self.r[local][1] / self.diagonal[local]];
                    rz += self.r[local][0] * self.z[local][0] + self.r[local][1] * self.z[local][1];
                    rr += self.r[local][0] * self.r[local][0] + self.r[local][1] * self.r[local][1];
                }
                self.next = (rz, rr);
                self.cursor = end;
                if self.cursor == count {
                    self.iterations += 1;
                    if rr <= self.rr_target || self.iterations >= self.limit || !rz.is_finite() {
                        return true;
                    }
                    self.alpha = rz / self.rz;
                    self.rz = rz;
                    self.stage = 3;
                    self.cursor = 0;
                }
            }
            3 => {
                let end = (self.cursor + CHUNK).min(count);
                for local in self.cursor..end {
                    self.p[local] = if self.free(local) { combine(self.z[local], self.alpha, self.p[local]) } else { [0.0; 2] };
                    self.ap[local] = [0.0; 2];
                }
                self.cursor = end;
                if self.cursor == count {
                    self.stage = 0;
                    self.cursor = 0;
                }
            }
            _ => return true,
        }
        false
    }

    fn coordinates(&self) -> BTreeMap<u32, [f64; 2]> {
        let mut uvs = BTreeMap::new();
        let scale = if self.scale > 0.0 { self.scale } else { 1.0 };
        for (local, vertex) in self.vertices.iter().enumerate() {
            let mut value = self.x[local];
            if local == self.pins[1] && self.pins[1] != self.pins[0] {
                value = [1.0, 0.0];
            }
            uvs.insert(*vertex, [value[0] * scale, value[1] * scale]);
        }
        uvs
    }
}
//#endregion 🔖️Unwrap
