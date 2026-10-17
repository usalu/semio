//! 🪄️ Bounded post-processing of the mesh pipeline: the resumable outward orientation, Taubin smoothing, simplification, UV unwrapping and texturing preparations. See [`super`].

use super::*;

// #region 🔖️BoundedPostprocess
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedOutwardPreparation {
    pub(super) cursor: usize,
    pub(super) signed_volume: f64,
    flip: Option<bool>,
}

impl BoundedOutwardPreparation {
    pub(super) fn new() -> Self {
        Self { cursor: 0, signed_volume: 0.0, flip: None }
    }

    pub(super) fn advance(&mut self, mesh: &mut TriMesh, triangle_budget: usize) -> bool {
        let end = self.cursor.saturating_add(triangle_budget.max(1)).min(mesh.triangles.len());
        if let Some(flip) = self.flip {
            if flip {
                for triangle in &mut mesh.triangles[self.cursor..end] {
                    triangle.swap(1, 2);
                }
            }
        } else {
            for triangle in &mesh.triangles[self.cursor..end] {
                let [a, b, c] = triangle.map(|vertex| mesh.positions[vertex as usize]);
                self.signed_volume += dot3(a, cross3(b, c)) / 6.0;
            }
        }
        self.cursor = end;
        if end < mesh.triangles.len() {
            return false;
        }
        if self.flip.is_none() {
            self.flip = Some(self.signed_volume < 0.0);
            self.cursor = 0;
            return mesh.triangles.is_empty();
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTaubinPhase {
    AllocateNeighbors,
    Edges,
    Vertices,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedTaubinPreparation {
    pub(super) phase: BoundedTaubinPhase,
    pub(super) cursor: usize,
    neighbors: Vec<BTreeSet<u32>>,
    pub(super) next: Vec<[f64; 3]>,
    pass: usize,
    passes: usize,
    lambda: f64,
    mu: f64,
}

impl BoundedTaubinPreparation {
    pub(super) fn new(iterations: usize, lambda: f64, mu: f64) -> Self {
        Self { phase: BoundedTaubinPhase::AllocateNeighbors, cursor: 0, neighbors: Vec::new(), next: Vec::new(), pass: 0, passes: iterations.saturating_mul(2), lambda, mu }
    }

    pub(super) fn advance(&mut self, mesh: &mut TriMesh, item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedTaubinPhase::AllocateNeighbors => {
                    if self.neighbors.len() < mesh.positions.len() {
                        self.neighbors.push(BTreeSet::new());
                    } else {
                        self.phase = BoundedTaubinPhase::Edges;
                    }
                }
                BoundedTaubinPhase::Edges => {
                    let Some(triangle) = mesh.triangles.get(self.cursor).copied() else {
                        self.cursor = 0;
                        self.next = Vec::with_capacity(mesh.positions.len());
                        self.phase = if self.passes == 0 { BoundedTaubinPhase::Done } else { BoundedTaubinPhase::Vertices };
                        continue;
                    };
                    for edge in 0..3 {
                        let a = triangle[edge];
                        let b = triangle[(edge + 1) % 3];
                        self.neighbors[a as usize].insert(b);
                        self.neighbors[b as usize].insert(a);
                    }
                    self.cursor += 1;
                }
                BoundedTaubinPhase::Vertices => {
                    if self.cursor < mesh.positions.len() {
                        let position = mesh.positions[self.cursor];
                        let neighbors = &self.neighbors[self.cursor];
                        if neighbors.is_empty() {
                            self.next.push(position);
                        } else {
                            let sum = neighbors.iter().fold([0.0; 3], |sum, &neighbor| add3(sum, mesh.positions[neighbor as usize]));
                            let average = scale3(sum, 1.0 / neighbors.len() as f64);
                            let factor = if self.pass.is_multiple_of(2) { self.lambda } else { self.mu };
                            self.next.push(add3(position, scale3(sub3(average, position), factor)));
                        }
                        self.cursor += 1;
                    } else {
                        mesh.positions = std::mem::take(&mut self.next);
                        self.pass += 1;
                        self.cursor = 0;
                        if self.pass == self.passes {
                            self.phase = BoundedTaubinPhase::Done;
                        } else {
                            self.next = Vec::with_capacity(mesh.positions.len());
                        }
                    }
                }
                BoundedTaubinPhase::Done => return true,
            }
        }
        self.phase == BoundedTaubinPhase::Done
    }
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedSimplifyPreparation {
    pub(super) cursor: usize,
    pub(super) target: usize,
    total: usize,
    pub(super) triangles: Vec<[u32; 3]>,
}

impl BoundedSimplifyPreparation {
    pub(super) fn new(mesh: &TriMesh, target: usize) -> Self {
        Self { cursor: 0, target: target.min(mesh.triangles.len()), total: mesh.triangles.len(), triangles: Vec::with_capacity(target.min(mesh.triangles.len())) }
    }

    pub(super) fn advance(&mut self, mesh: &TriMesh, triangle_budget: usize) -> bool {
        let end = self.cursor.saturating_add(triangle_budget.max(1)).min(self.total);
        for index in self.cursor..end {
            if (index + 1).saturating_mul(self.target) / self.total.max(1) > index.saturating_mul(self.target) / self.total.max(1) {
                self.triangles.push(mesh.triangles[index]);
            }
        }
        self.cursor = end;
        end == self.total
    }

    pub(super) fn finish(self, mesh: &mut TriMesh) {
        mesh.triangles = self.triangles;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedUnwrapPhase {
    Bounds,
    Vertices,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedUnwrapPreparation {
    pub(super) phase: BoundedUnwrapPhase,
    pub(super) cursor: usize,
    lower: [f64; 3],
    upper: [f64; 3],
    axes: [usize; 2],
    pub(super) uvs: Vec<[f32; 2]>,
}

impl BoundedUnwrapPreparation {
    pub(super) fn new(mesh: &TriMesh) -> Self {
        Self { phase: BoundedUnwrapPhase::Bounds, cursor: 0, lower: [f64::INFINITY; 3], upper: [f64::NEG_INFINITY; 3], axes: [0, 1], uvs: Vec::with_capacity(mesh.positions.len()) }
    }

    pub(super) fn advance(&mut self, mesh: &TriMesh, vertex_budget: usize) -> bool {
        let end = self.cursor.saturating_add(vertex_budget.max(1)).min(mesh.positions.len());
        match self.phase {
            BoundedUnwrapPhase::Bounds => {
                for position in &mesh.positions[self.cursor..end] {
                    for (axis, coordinate) in position.iter().copied().enumerate() {
                        self.lower[axis] = self.lower[axis].min(coordinate);
                        self.upper[axis] = self.upper[axis].max(coordinate);
                    }
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    let mut spans = (0..3).map(|axis| (axis, self.upper[axis] - self.lower[axis])).collect::<Vec<_>>();
                    spans.sort_by(|a, b| b.1.total_cmp(&a.1));
                    self.axes = [spans[0].0, spans[1].0];
                    self.cursor = 0;
                    self.phase = BoundedUnwrapPhase::Vertices;
                }
            }
            BoundedUnwrapPhase::Vertices => {
                for position in &mesh.positions[self.cursor..end] {
                    self.uvs.push(std::array::from_fn(|component| {
                        let axis = self.axes[component];
                        ((position[axis] - self.lower[axis]) / (self.upper[axis] - self.lower[axis]).max(1e-12)) as f32
                    }));
                }
                self.cursor = end;
                if end == mesh.positions.len() {
                    self.phase = BoundedUnwrapPhase::Done;
                }
            }
            BoundedUnwrapPhase::Done => {}
        }
        self.phase == BoundedUnwrapPhase::Done
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
enum BoundedTexturePhase {
    Allocate,
    Faces,
    Done,
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct BoundedTexturePreparation {
    pub(super) phase: BoundedTexturePhase,
    pub(super) atlas_size: u32,
    pub(super) data: Vec<u8>,
    pub(super) face: usize,
    pixel: u32,
}

impl BoundedTexturePreparation {
    pub(super) fn new(atlas_size: u32) -> Self {
        Self { phase: BoundedTexturePhase::Allocate, atlas_size, data: Vec::with_capacity(atlas_size as usize * atlas_size as usize * 4), face: 0, pixel: 0 }
    }

    pub(super) fn advance(&mut self, mesh: &TriMesh, uvs: &[[f32; 2]], views: &[TextureView], item_budget: usize) -> bool {
        for _ in 0..item_budget.max(1) {
            match self.phase {
                BoundedTexturePhase::Allocate => {
                    let target = self.atlas_size as usize * self.atlas_size as usize * 4;
                    let end = self.data.len().saturating_add(4).min(target);
                    self.data.resize(end, 0);
                    if end == target {
                        self.phase = BoundedTexturePhase::Faces;
                    }
                }
                BoundedTexturePhase::Faces => {
                    let Some(triangle) = mesh.triangles.get(self.face).copied() else {
                        self.phase = BoundedTexturePhase::Done;
                        continue;
                    };
                    let atlas_pixels = self.atlas_size.saturating_mul(self.atlas_size);
                    if self.pixel >= atlas_pixels {
                        self.face += 1;
                        self.pixel = 0;
                        continue;
                    }
                    let x = self.pixel % self.atlas_size;
                    let y = self.pixel / self.atlas_size;
                    self.pixel += 1;
                    let uv = triangle.map(|vertex| uvs[vertex as usize]);
                    let p = [(f64::from(x) + 0.5) / f64::from(self.atlas_size), (f64::from(y) + 0.5) / f64::from(self.atlas_size)];
                    let denominator = f64::from((uv[1][1] - uv[2][1]) * (uv[0][0] - uv[2][0]) + (uv[2][0] - uv[1][0]) * (uv[0][1] - uv[2][1]));
                    if denominator.abs() < 1e-12 {
                        continue;
                    }
                    let w0 = (f64::from(uv[1][1] - uv[2][1]) * (p[0] - f64::from(uv[2][0])) + f64::from(uv[2][0] - uv[1][0]) * (p[1] - f64::from(uv[2][1]))) / denominator;
                    let w1 = (f64::from(uv[2][1] - uv[0][1]) * (p[0] - f64::from(uv[2][0])) + f64::from(uv[0][0] - uv[2][0]) * (p[1] - f64::from(uv[2][1]))) / denominator;
                    let w2 = 1.0 - w0 - w1;
                    if w0 < -1e-6 || w1 < -1e-6 || w2 < -1e-6 {
                        continue;
                    }
                    let positions = triangle.map(|vertex| mesh.positions[vertex as usize]);
                    let world = add3(add3(scale3(positions[0], w0), scale3(positions[1], w1)), scale3(positions[2], w2));
                    let sample = views.iter().find_map(|view| {
                        let camera = view.pose.0.act(world);
                        (camera[2] > 0.0)
                            .then(|| view.intrinsics.project(camera))
                            .flatten()
                            .and_then(|pixel| (pixel[0] >= 0.0 && pixel[1] >= 0.0 && pixel[0] < f64::from(view.image.width) && pixel[1] < f64::from(view.image.height)).then(|| view.image.sample_rgb(pixel[0] as f32, pixel[1] as f32)))
                    });
                    if let Some(rgb) = sample {
                        let index = ((y * self.atlas_size + x) * 4) as usize;
                        self.data[index] = (rgb[0].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 1] = (rgb[1].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 2] = (rgb[2].clamp(0.0, 1.0) * 255.0) as u8;
                        self.data[index + 3] = 255;
                    }
                }
                BoundedTexturePhase::Done => return true,
            }
        }
        self.phase == BoundedTexturePhase::Done
    }

    pub(super) fn finish(self) -> remodeling_image::ImageRgba8 {
        remodeling_image::ImageRgba8 { width: self.atlas_size, height: self.atlas_size, data: self.data }
    }
}
// #endregion 🔖️BoundedPostprocess
