//! 🌳️ Funded active-scene traversal and embedded dense accessor projection.
use super::{GlbMatrix, MeshData, glb_identity, glb_matrix_mul, glb_transform_normal, glb_transform_point, gltf_node_matrix, json::Value};
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneProgress};
use std::mem::size_of;

#[path="🔢️accessor/🦀️.rs"]
mod accessor;
use accessor::Accessor;
#[path="📚️buffers/🦀️.rs"]
mod buffers;
use buffers::{BufferCursor, BufferStep, Buffers};
#[path="🗂️metadata/🦀️.rs"]
mod metadata;
use metadata::{MetadataCursor, field};

const SCENE_DEPTH: usize = 128;
const SCENE_VISITS: usize = 100_000;
type SceneResult<T> = std::result::Result<T, String>;

fn rows<'a>(value: &'a Value, name: &str) -> &'a [Value] {
    field(value, name).and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

fn index(value: &Value, name: &str) -> Option<usize> {
    field(value, name).and_then(Value::as_u64).and_then(|x| usize::try_from(x).ok())
}

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
struct Frame {
    node: Option<usize>,
    mesh: Option<usize>,
    matrix: GlbMatrix,
    primitive: usize,
    child: usize,
}

#[derive(Default, semio_framework_value_derive::RetireOwned)]
struct Walker {
    stack: Vec<Frame>,
    root: usize,
    visits: usize,
}

enum Event<'a> {
    Walk,
    Primitive(&'a Value, GlbMatrix),
    Complete,
}

impl Walker {
    fn reset(&mut self) {
        self.root = 0;
        self.visits = 0;
    }

    fn enter(&mut self, nodes: &[Value], node: usize, parent: GlbMatrix) -> SceneResult<()> {
        if self.visits == SCENE_VISITS || self.stack.len() == SCENE_DEPTH || self.stack.iter().any(|frame| frame.node == Some(node)) {
            return Err("glb: active scene is cyclic or exceeds its capacity".into());
        }
        let value = nodes.get(node).ok_or("glb: invalid node index")?;
        let matrix = glb_matrix_mul(parent, gltf_node_matrix(|key| field(value, key)));
        self.stack.push(Frame { node: Some(node), mesh: index(value, "mesh"), matrix, primitive: 0, child: 0 });
        self.visits += 1;
        Ok(())
    }

    fn step<'a>(&mut self, document: &'a Value) -> SceneResult<Event<'a>> {
        let meshes = rows(document, "meshes");
        let nodes = rows(document, "nodes");
        if let Some(frame) = self.stack.last_mut() {
            if let Some(mesh) = frame.mesh {
                let value = meshes.get(mesh).ok_or("glb: invalid mesh index")?;
                if let Some(primitive) = rows(value, "primitives").get(frame.primitive) {
                    frame.primitive += 1;
                    return Ok(Event::Primitive(primitive, frame.matrix));
                }
            }
            if let Some(child) = frame.node.and_then(|i| rows(&nodes[i], "children").get(frame.child)) {
                frame.child += 1;
                let matrix = frame.matrix;
                if let Some(node) = child.as_u64().and_then(|x| usize::try_from(x).ok()) {
                    self.enter(nodes, node, matrix)?;
                }
            } else {
                self.stack.pop();
            }
            return Ok(Event::Walk);
        }
        let scenes = rows(document, "scenes");
        if let Some(scene) = index(document, "scene").and_then(|i| scenes.get(i)).or_else(|| scenes.first()) {
            let Some(node) = rows(scene, "nodes").get(self.root) else { return Ok(Event::Complete); };
            self.root += 1;
            if let Some(node) = node.as_u64().and_then(|x| usize::try_from(x).ok()) {
                self.enter(nodes, node, glb_identity())?;
            }
        } else if self.root == meshes.len() {
            return Ok(Event::Complete);
        } else {
            self.stack.push(Frame { node: None, mesh: Some(self.root), matrix: glb_identity(), primitive: 0, child: 0 });
            self.root += 1;
        }
        Ok(Event::Walk)
    }
}

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
struct Primitive {
    matrix: GlbMatrix,
    mode: u64,
    positions: Accessor,
    normals: Option<Accessor>,
    uvs: Option<Accessor>,
    indices: Option<Accessor>,
    triangles: usize,
}

enum Intake {
    Skip,
    Dense(Primitive),
}

impl Primitive {
    fn layout(document: &Value, buffers: Buffers<'_>, value: &Value, matrix: GlbMatrix, remaining: usize) -> SceneResult<Intake> {
        let mode = field(value, "mode").and_then(Value::as_u64).unwrap_or(4);
        if !matches!(mode, 4..=6) { return Ok(Intake::Skip); }
        let attributes = field(value, "attributes").ok_or("glb: primitive missing attributes")?;
        let position = index(attributes, "POSITION").ok_or("glb: triangle primitive missing POSITION")?;
        let positions = Accessor::layout(document, buffers, position, "VEC3", 3)?;
        let normals = Self::optional(document, buffers, attributes, "NORMAL", "VEC3", 3)?;
        let uvs = Self::optional(document, buffers, attributes, "TEXCOORD_0", "VEC2", 2)?;
        let indices = Self::optional(document, buffers, value, "indices", "SCALAR", 1)?;
        let corners = indices.map_or(positions.count, |a| a.count);
        let triangles = if mode == 4 {
            if corners % 3 != 0 { return Err("glb: triangle list is incomplete".into()); }
            corners / 3
        } else { corners.saturating_sub(2) };
        let uv_type = |a: &Accessor| matches!((a.component, a.normalized), (5126, false) | (5121 | 5123, true));
        if triangles > remaining || corners > remaining.saturating_mul(3) || positions.count > remaining.saturating_mul(3) {
            return Err("glb: triangles or accessor count exceed capacity".into());
        }
        if normals.as_ref().is_some_and(|a| a.count != positions.count) || uvs.as_ref().is_some_and(|a| a.count != positions.count || !uv_type(a)) {
            return Err("glb: NORMAL or TEXCOORD_0 layout differs from POSITION".into());
        }
        Ok(Intake::Dense(Self { matrix, mode, positions, normals, uvs, indices, triangles }))
    }

    fn optional(document: &Value, buffers: Buffers<'_>, values: &Value, name: &str, kind: &str, width: usize) -> SceneResult<Option<Accessor>> {
        match index(values, name) {
            Some(id) => Accessor::layout(document, buffers, id, kind, width).map(Some),
            None if name == "TEXCOORD_0" && field(values, name).is_some() => Err("glb: invalid TEXCOORD_0 index".into()),
            None => Ok(None),
        }
    }

    fn accessors(&self) -> [Option<&Accessor>; 4] {
        [Some(&self.positions), self.normals.as_ref(), self.uvs.as_ref(), self.indices.as_ref()]
    }

    fn triangle(&self, buffers: Buffers<'_>, row: usize, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<[usize; 3]> {
        let indices = match self.mode {
            4 => [row * 3, row * 3 + 1, row * 3 + 2],
            5 if row % 2 == 0 => [row, row + 1, row + 2],
            5 => [row + 1, row, row + 2],
            _ => [0, row + 1, row + 2],
        };
        let mut output = indices;
        if let Some(accessor) = self.indices.as_ref() {
            for (index, unit) in output.iter_mut().zip(indices) {
                *index = accessor.sample::<1>(buffers, unit, effect)?[0] as u32 as usize;
            }
        }
        if output.iter().any(|index| *index >= self.positions.count) {
            return Err("glb triangle index outside POSITION accessor".into());
        }
        Ok(output)
    }
}

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
enum Phase {
    Metadata, Buffers, Walker, Count, Sparse, PositionsAllocation, NormalsAllocation, IndicesAllocation, UvAllocation,
    Draw, Vertices, Triangles, Normals, Complete,
}

/// ⏱️ Stages embedded accessors over certified ranges and physically admitted output arrays.
#[derive(semio_framework_value_derive::RetireOwned)]
pub(super) struct SceneCursor {
    phase: Phase,
    metadata: MetadataCursor,
    buffers: BufferCursor,
    walker: Walker,
    primitive: Option<Primitive>,
    row: usize,
    sparse_channel: usize,
    sparse_previous: Option<usize>,
    base: usize,
    vertices: usize,
    triangles: usize,
    has_uvs: bool,
    mesh: MeshData,
    receipt: [usize; 3],
}

pub(super) enum SceneStep {
    Working,
    Complete(MeshData),
}

impl Default for SceneCursor {
    fn default() -> Self {
        Self { phase: Phase::Metadata, metadata: MetadataCursor::default(), buffers: BufferCursor::default(), walker: Walker::default(), primitive: None, row: 0, sparse_channel: 0, sparse_previous: None, base: 0,
            vertices: 0, triangles: 0, has_uvs: false, mesh: MeshData::default(), receipt: [0; 3] }
    }
}

fn allocate<T>(out: &mut Vec<T>, count: usize, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<usize> {
    if count == 0 { return Ok(0); }
    if !out.is_empty() || out.capacity() != 0 || effect.actual().is_some() {
        return Err("glb: invalid fresh output allocation".into());
    }
    let result = out.try_reserve_exact(count);
    let bytes = out.capacity() * size_of::<T>();
    *effect.actual() = Some(bytes);
    result.map_err(|_| "glb: output allocation refused".to_owned())?;
    Ok(bytes)
}

impl SceneCursor {
    pub(super) fn phase(&self) -> &'static str {
        match self.phase {
            Phase::Metadata => "mesh-glb-metadata",
            Phase::Buffers => self.buffers.phase(),
            Phase::Walker | Phase::Count => "mesh-glb-scene-count",
            Phase::Sparse => "mesh-glb-sparse-indices",
            Phase::PositionsAllocation | Phase::NormalsAllocation | Phase::IndicesAllocation | Phase::UvAllocation => "mesh-glb-output-reserve",
            Phase::Draw => "mesh-glb-scene",
            Phase::Vertices => "mesh-glb-accessor-vertices",
            Phase::Triangles => "mesh-glb-accessor-triangles",
            Phase::Normals => "mesh-glb-generated-normals",
            Phase::Complete => "mesh-glb-scene-complete",
        }
    }

    pub(super) fn take_progress(&mut self) -> RetainedCloneProgress {
        let [copied_items, copied_bytes, retained_capacity_bytes] = std::mem::take(&mut self.receipt);
        RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes: 0 }
    }

    pub(super) fn step(&mut self, document: &mut Value, bin: Option<&[u8]>, maximum_triangles: usize, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<SceneStep> {
        self.receipt = [0; 3];
        if matches!(self.phase, Phase::Metadata) {
            let done = self.metadata.step(document, effect);
            let progress = self.metadata.take_progress();
            self.receipt = [progress.copied_items, progress.copied_bytes, 0];
            if done? { self.phase = Phase::Buffers; }
            return Ok(SceneStep::Working);
        }
        if matches!(self.phase, Phase::Complete) { return Err("glb: scene already taken".into()); }
        if matches!(self.phase, Phase::Buffers) {
            let result = self.buffers.step(document, effect);
            let progress = self.buffers.take_progress();
            self.receipt = [progress.copied_items, progress.copied_bytes, progress.retained_capacity_bytes];
            match result? {
                BufferStep::Working => {},
                BufferStep::Ready => self.phase = Phase::Walker,
            }
            return Ok(SceneStep::Working);
        }
        let grant = effect.grant();
        let (capacity, copy) = match self.phase {
            Phase::Walker => (SCENE_DEPTH * size_of::<Frame>(), 0),
            Phase::PositionsAllocation | Phase::NormalsAllocation => (self.vertices * 12, 0),
            Phase::IndicesAllocation => (self.triangles * 12, 0),
            Phase::UvAllocation => (usize::from(self.has_uvs) * self.vertices * 8, 0),
            Phase::Count | Phase::Draw => (0, size_of::<Frame>().max(size_of::<Primitive>())),
            Phase::Vertices => (0, if self.has_uvs { 32 } else { 24 }),
            Phase::Triangles => (0, if self.primitive.as_ref().unwrap().normals.is_none() { 48 } else { 12 }),
            Phase::Normals => (0, 24),
            _ => (0, 0),
        };
        let additional_work = match self.phase {
            Phase::Vertices if self.row < self.primitive.as_ref().unwrap().positions.count => {
                self.primitive.as_ref().unwrap().accessors()[..3].iter().flatten().map(|accessor| accessor.lookup_work()).sum()
            }
            Phase::Triangles if self.row < self.primitive.as_ref().unwrap().triangles => {
                self.primitive.as_ref().unwrap().indices.as_ref().map_or(0, |accessor| 3 * accessor.lookup_work())
            }
            _ => 0,
        };
        let admits = grant.maximum_items > 0 && grant.maximum_depth > 0 && grant.maximum_copy_bytes >= copy
            && grant.maximum_capacity_bytes >= capacity
            && effect.remaining_work_units().map_err(|error| error.to_string())? >= 1 + additional_work;
        if !admits || !effect.performed(1).map_err(|error| error.to_string())? { return Ok(SceneStep::Working); }
        self.receipt[0] = 1;
        match self.phase {
            Phase::Walker => {
                self.receipt[2] = allocate(&mut self.walker.stack, SCENE_DEPTH, effect)?;
                self.phase = Phase::Count;
            }
            Phase::Count | Phase::Draw => return self.walk(document, bin, maximum_triangles),
            Phase::Sparse => self.check_sparse(document, bin)?,
            Phase::PositionsAllocation => {
                self.receipt[2] = allocate(&mut self.mesh.positions, self.vertices * 3, effect)?;
                self.phase = Phase::NormalsAllocation;
            }
            Phase::NormalsAllocation => {
                self.receipt[2] = allocate(&mut self.mesh.normals, self.vertices * 3, effect)?;
                self.phase = Phase::IndicesAllocation;
            }
            Phase::IndicesAllocation => {
                self.receipt[2] = allocate(&mut self.mesh.indices, self.triangles * 3, effect)?;
                self.phase = Phase::UvAllocation;
            }
            Phase::UvAllocation => {
                self.receipt[2] = allocate(&mut self.mesh.uvs, usize::from(self.has_uvs) * self.vertices * 2, effect)?;
                self.walker.reset();
                self.phase = Phase::Draw;
            }
            Phase::Vertices => self.vertex(document, bin, effect)?,
            Phase::Triangles => self.triangle(document, bin, effect)?,
            Phase::Normals => self.finalize_normal(),
            Phase::Metadata | Phase::Buffers | Phase::Complete => unreachable!(),
        }
        Ok(SceneStep::Working)
    }

    fn walk(&mut self, document: &Value, bin: Option<&[u8]>, maximum: usize) -> SceneResult<SceneStep> {
        let counting = matches!(self.phase, Phase::Count);
        let depth = self.walker.stack.len();
        let event = self.walker.step(document)?;
        if self.walker.stack.len() > depth { self.receipt[1] = size_of::<Frame>(); }
        match event {
            Event::Walk => {},
            Event::Complete if counting => {
                if self.triangles == 0 { return Err("glb contains no triangle primitives".into()); }
                self.phase = Phase::PositionsAllocation;
            }
            Event::Complete => {
                self.phase = Phase::Complete;
                return Ok(SceneStep::Complete(std::mem::take(&mut self.mesh)));
            }
            Event::Primitive(value, matrix) => {
                let consumed = if counting { self.triangles } else { self.mesh.indices.len() / 3 };
                match Primitive::layout(document, self.buffers.view(document, bin), value, matrix, maximum.saturating_sub(consumed))? {
                    Intake::Skip => {},
                    Intake::Dense(primitive) if counting => {
                        self.vertices = self.vertices.checked_add(primitive.positions.count)
                            .filter(|count| *count <= maximum.saturating_mul(3) && *count < u32::MAX as usize)
                            .ok_or("glb: vertices exceed output capacity")?;
                        self.triangles += primitive.triangles;
                        self.has_uvs |= primitive.uvs.is_some() && primitive.positions.count != 0;
                        if primitive.accessors().into_iter().flatten().any(|accessor| accessor.sparse_count() != 0) {
                            self.primitive = Some(primitive);
                            self.receipt[1] = size_of::<Primitive>();
                            self.sparse_channel = 0;
                            self.sparse_previous = None;
                            self.row = 0;
                            self.phase = Phase::Sparse;
                        }
                    }
                    Intake::Dense(primitive) => {
                        self.base = self.mesh.positions.len() / 3;
                        self.primitive = Some(primitive);
                        self.receipt[1] = size_of::<Primitive>();
                        self.row = 0;
                        self.phase = Phase::Vertices;
                    }
                }
            }
        }
        Ok(SceneStep::Working)
    }

    fn check_sparse(&mut self, document: &Value, bin: Option<&[u8]>) -> SceneResult<()> {
        let accessors = self.primitive.as_ref().unwrap().accessors();
        if self.sparse_channel == accessors.len() {
            self.primitive = None;
            self.phase = Phase::Count;
            return Ok(());
        }
        if let Some(accessor) = accessors[self.sparse_channel].filter(|accessor| self.row < accessor.sparse_count()) {
            let index = accessor.sparse_index(self.buffers.view(document, bin), self.row)?;
            if index >= accessor.count || self.sparse_previous.is_some_and(|previous| previous >= index) {
                return Err("glb: sparse indices must increase strictly within accessor count".into());
            }
            self.sparse_previous = Some(index);
            self.row += 1;
        } else {
            self.sparse_channel += 1;
            self.row = 0;
            self.sparse_previous = None;
        }
        Ok(())
    }

    fn vertex(&mut self, document: &Value, bin: Option<&[u8]>, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<()> {
        let primitive = self.primitive.as_ref().unwrap();
        if self.row == primitive.positions.count {
            self.row = 0;
            self.phase = Phase::Triangles;
            return Ok(());
        }
        let buffers = self.buffers.view(document, bin);
        let p = primitive.positions.sample::<3>(buffers, self.row, effect)?.map(|x| x as f32);
        let (p, n) = if let Some(normals) = primitive.normals.as_ref() {
            let n = normals.sample::<3>(buffers, self.row, effect)?.map(|x| x as f32);
            (glb_transform_point(primitive.matrix, p), glb_transform_normal(primitive.matrix, n))
        } else { (p, [0.0; 3]) };
        let uv = primitive.uvs.as_ref().map(|accessor| accessor.sample::<2>(buffers, self.row, effect)).transpose()?.unwrap_or_default().map(|x| x as f32);
        if uv.iter().any(|x| !x.is_finite()) { return Err("glb TEXCOORD_0 contains non-finite values".into()); }
        self.mesh.positions.extend_from_slice(&p);
        self.mesh.normals.extend_from_slice(&n);
        if self.has_uvs { self.mesh.uvs.extend_from_slice(&uv); }
        self.receipt[1] = if self.has_uvs { 32 } else { 24 };
        self.row += 1;
        Ok(())
    }

    fn triangle(&mut self, document: &Value, bin: Option<&[u8]>, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<()> {
        let primitive = self.primitive.as_ref().unwrap();
        if self.row == primitive.triangles {
            self.row = 0;
            self.phase = if primitive.normals.is_some() { Phase::Draw } else { Phase::Normals };
            return Ok(());
        }
        let indices = primitive.triangle(self.buffers.view(document, bin), self.row, effect)?.map(|index| self.base + index);
        self.mesh.indices.extend(indices.map(|index| index as u32));
        self.receipt[1] = 12;
        if primitive.normals.is_none() {
            let [a, b, c] = indices.map(|index| [0, 1, 2].map(|axis| self.mesh.positions[index * 3 + axis]));
            let x = [0, 1, 2].map(|axis| b[axis] - a[axis]);
            let y = [0, 1, 2].map(|axis| c[axis] - a[axis]);
            let normal = [x[1]*y[2]-x[2]*y[1], x[2]*y[0]-x[0]*y[2], x[0]*y[1]-x[1]*y[0]];
            for index in indices {
                for (axis, value) in normal.into_iter().enumerate() { self.mesh.normals[index * 3 + axis] += value; }
            }
            self.receipt[1] = 48;
        }
        self.row += 1;
        Ok(())
    }

    fn finalize_normal(&mut self) {
        let primitive = self.primitive.as_ref().unwrap();
        if self.row == primitive.positions.count {
            self.phase = Phase::Draw;
            return;
        }
        let at = (self.base + self.row) * 3;
        let position = self.mesh.positions[at..at + 3].try_into().unwrap();
        let mut normal: [f32; 3] = self.mesh.normals[at..at + 3].try_into().unwrap();
        let length = normal.iter().map(|value| value * value).sum::<f32>().sqrt();
        if length > 1e-8 { normal = normal.map(|value| value / length); }
        self.mesh.positions[at..at + 3].copy_from_slice(&glb_transform_point(primitive.matrix, position));
        self.mesh.normals[at..at + 3].copy_from_slice(&glb_transform_normal(primitive.matrix, normal));
        self.receipt[1] = 24;
        self.row += 1;
    }
}
