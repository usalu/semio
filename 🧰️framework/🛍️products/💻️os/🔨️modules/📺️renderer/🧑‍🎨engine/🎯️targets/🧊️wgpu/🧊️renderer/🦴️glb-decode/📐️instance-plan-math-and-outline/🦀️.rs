//! 📐️ Instance plan, math and outline: the instance plan cursor, the world frame and matrix math, the transform helpers and the outline edge accumulator. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
impl GlbInstancePlanCursor {
    pub(super) fn new(schema: &GlbSchemaOutput) -> Self {
        let scene = schema.default_scene.or((schema.scene_len != 0).then_some(0));
        let mode = if schema.node_len == 0 { GlbPlanMode::Fallback { primitive: 0 } } else { GlbPlanMode::Roots { scene, index: 0 } };
        Self { instances: semio_framework_async::boxed_fixed_slots(|| None), instance_len: 0, stack: semio_framework_async::boxed_fixed_slots(|| None), stack_len: 0, mode, vertex_count: 0, index_count: 0, has_uvs: false,has_tangents:false, has_colors: false, face_id_instances: 0, vertex_id_instances: 0, output_bytes: 0 }
    }

    /// 🎯️ Whether EVERY instantiated primitive carries `_FACE_ID`; a table with gaps would mix producer ids with anonymous ones, so a mixed bank publishes none.
    pub(super) fn has_face_ids(&self) -> bool {
        self.instance_len != 0 && self.face_id_instances == self.instance_len
    }

    /// 🎯️ Whether EVERY instantiated primitive carries `_VERTEX_ID`.
    pub(super) fn has_vertex_ids(&self) -> bool {
        self.instance_len != 0 && self.vertex_id_instances == self.instance_len
    }

    pub(super) fn step(&mut self, schema: &GlbSchemaOutput) -> Result<bool, &'static str> {
        match self.mode {
            GlbPlanMode::Roots { scene, index } => {
                let root = if let Some(scene) = scene {
                    let scene = schema.scenes.get(usize::from(scene)).and_then(Option::as_ref).ok_or("GLB plan lost its scene")?;
                    if index >= scene.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    schema.scene_roots.get(usize::from(scene.node_start) + usize::from(index)).and_then(Option::as_ref).copied().ok_or("GLB plan lost a scene root")?
                } else {
                    if index >= schema.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    index
                };
                self.push_node(root, glb_world_frame())?;
                self.mode = GlbPlanMode::Nodes { scene, next_root: index + 1 };
                Ok(false)
            }
            GlbPlanMode::Nodes { scene, next_root } => {
                if self.stack_len == 0 {
                    self.mode = GlbPlanMode::Roots { scene, index: next_root };
                    return Ok(false);
                }
                let slot = usize::from(self.stack_len - 1);
                let mut frame = self.stack[slot].take().ok_or("GLB node traversal lost its retained frame")?;
                let node = *schema.nodes.get(usize::from(frame.node)).and_then(Option::as_ref).ok_or("GLB node traversal referenced a missing node")?;
                let local = node.matrix.map(glb_matrix_from_array).unwrap_or_else(|| glb_trs(node.translation, node.rotation, node.scale));
                let world = glb_matrix_mul(frame.parent, local);
                if frame.phase == 0 {
                    frame.phase = 1;
                    frame.item = 0;
                }
                if frame.phase == 1 {
                    if let Some(mesh) = node.mesh {
                        let mesh = schema.meshes.get(usize::from(mesh)).and_then(Option::as_ref).ok_or("GLB node traversal lost a mesh")?;
                        if frame.item < mesh.primitive_len {
                            let primitive = mesh.primitive_start.checked_add(frame.item).ok_or("GLB primitive range overflowed")?;
                            frame.item += 1;
                            self.stack[slot] = Some(frame);
                            self.admit_instance(schema, primitive, world)?;
                            return Ok(false);
                        }
                    }
                    frame.phase = 2;
                    frame.item = 0;
                }
                if frame.item < node.child_len {
                    let child = schema.node_edges.get(usize::from(node.child_start) + usize::from(frame.item)).and_then(Option::as_ref).copied().ok_or("GLB node traversal lost a child edge")?;
                    frame.item += 1;
                    self.stack[slot] = Some(frame);
                    self.push_node(child, world)?;
                    return Ok(false);
                }
                self.stack_len -= 1;
                Ok(false)
            }
            GlbPlanMode::Fallback { primitive } => {
                if primitive >= schema.primitive_len {
                    self.mode = GlbPlanMode::Terminal;
                    return Ok(false);
                }
                self.admit_instance(schema, primitive, glb_world_frame())?;
                self.mode = GlbPlanMode::Fallback { primitive: primitive + 1 };
                Ok(false)
            }
            GlbPlanMode::Terminal => Ok(true),
        }
    }

    fn push_node(&mut self, node: u16, parent: GlbMatrix) -> Result<(), &'static str> {
        let slot = usize::from(self.stack_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node traversal exceeded fixed depth credits");
        }
        if self.stack[..slot].iter().flatten().any(|frame| frame.node == node) {
            return Err("GLB node traversal contained a cycle");
        }
        self.stack[slot] = Some(GlbNodePlanFrame { node, parent, phase: 0, item: 0 });
        self.stack_len += 1;
        Ok(())
    }

    fn admit_instance(&mut self, schema: &GlbSchemaOutput, primitive: u16, matrix: GlbMatrix) -> Result<(), &'static str> {
        let primitive_schema = schema.primitives.get(usize::from(primitive)).and_then(Option::as_ref).ok_or("GLB plan referenced a missing primitive")?;
        if !matches!(primitive_schema.mode, 4..=6) {
            return Ok(());
        }
        let position = schema.accessor(primitive_schema.position)?;
        let source_indices = primitive_schema.indices.map(|index| schema.accessor(index).map(|accessor| accessor.count)).transpose()?.unwrap_or(position.count);
        let index_count = match primitive_schema.mode {
            4 => source_indices,
            5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
            _ => 0,
        };
        if index_count == 0 || !index_count.is_multiple_of(3) {
            return Err("GLB instance had no complete triangle output");
        }
        let vertex_base = self.vertex_count;
        let index_base = self.index_count;
        let vertex_count = vertex_base.checked_add(position.count).ok_or("GLB instantiated vertex count overflowed")?;
        let output_index_count = index_base.checked_add(index_count).ok_or("GLB instantiated index count overflowed")?;
        let has_uvs = self.has_uvs || primitive_schema.uv_sets.iter().any(Option::is_some);
        let has_tangents=self.has_tangents || primitive_schema.tangent.is_some();
        let has_colors = self.has_colors || primitive_schema.color.is_some();
        let has_face_ids = self.face_id_instances != 0 || primitive_schema.face_id.is_some();
        let has_vertex_ids = self.vertex_id_instances != 0 || primitive_schema.vertex_id.is_some();
        let vertex_stride = 24usize.checked_add(if has_uvs { 40 } else { 0 }).and_then(|stride|stride.checked_add(if has_tangents{16}else{0})).and_then(|stride| stride.checked_add(if has_colors { 16 } else { 0 })).and_then(|stride| stride.checked_add(if has_vertex_ids { 4 } else { 0 })).ok_or("GLB instantiated vertex stride overflowed")?;
        let vertex_bytes = usize::try_from(vertex_count).ok().and_then(|count| count.checked_mul(vertex_stride)).ok_or("GLB instantiated vertex bytes overflowed")?;
        let index_bytes = usize::try_from(output_index_count).ok().and_then(|count| count.checked_mul(4).and_then(|bytes| bytes.checked_add(if has_face_ids { count / 3 * 4 } else { 0 }))).ok_or("GLB instantiated index bytes overflowed")?;
        let output_bytes = vertex_bytes.checked_add(index_bytes).ok_or("GLB instantiated output bytes overflowed")?;
        if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
            return Err("GLB instantiated output exceeded fixed byte credits");
        }
        let slot = usize::from(self.instance_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB instantiated primitive count exceeded fixed item credits");
        }
        self.instances[slot] = Some(GlbInstanceOutput { primitive, matrix, vertex_base, index_base, vertex_count: position.count, index_count, explicit_normals: primitive_schema.normal.is_some() });
        self.instance_len += 1;
        self.face_id_instances += u16::from(primitive_schema.face_id.is_some());
        self.vertex_id_instances += u16::from(primitive_schema.vertex_id.is_some());
        self.vertex_count = vertex_count;
        self.index_count = output_index_count;
        self.has_uvs = has_uvs;self.has_tangents=has_tangents;
        self.has_colors = has_colors;
        self.output_bytes = output_bytes;
        Ok(())
    }
}

/// 🧊️🌍️ The frame every GLB scene root is mounted under: a +90° rotation about X, which carries
/// glTF's Y-up convention (https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#coordinate-system-and-units)
/// into the world's Z-up one. Column-major, so `(x, y, z)` becomes `(x, -z, y)`.
///
/// This is `GLB_MESH_FRAME_ROTATION_X` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`), which React applies as the
/// `<group rotation={[π/2, 0, 0]}>` wrapping every `GlbInstanceMesh` — and, crucially, also inside
/// `extractGlbCollisionMesh`, so the collision bodies an app registers for these meshes are already
/// in this frame. Baking it into the decoded mesh (rather than into each instance model) keeps the
/// wire's INLINE geometry untouched, exactly the split React makes: only a GLB gets the frame.
fn glb_world_frame() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, -1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn glb_matrix_from_array(values: [f32; 16]) -> GlbMatrix {
    [values[0..4].try_into().expect("fixed GLB matrix column"), values[4..8].try_into().expect("fixed GLB matrix column"), values[8..12].try_into().expect("fixed GLB matrix column"), values[12..16].try_into().expect("fixed GLB matrix column")]
}

fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            for axis in 0..4 {
                result[column][row] += left[axis][row] * right[column][axis];
            }
        }
    }
    result
}

fn glb_trs(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> GlbMatrix {
    let [x, y, z, w] = rotation;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    [
        [(1.0 - (yy + zz)) * scale[0], (xy + wz) * scale[0], (xz - wy) * scale[0], 0.0],
        [(xy - wz) * scale[1], (1.0 - (xx + zz)) * scale[1], (yz + wx) * scale[1], 0.0],
        [(xz + wy) * scale[2], (yz - wx) * scale[2], (1.0 - (xx + yy)) * scale[2], 0.0],
        [translation[0], translation[1], translation[2], 1.0],
    ]
}

pub(super) fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

pub(super) fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    glb_normalize(transformed)
}

pub(super) fn glb_normalize(value: [f32; 3]) -> [f32; 3] {
    let length = value.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        value
    } else {
        value.map(|value| value / length)
    }
}

pub const GLB_OUTLINE_SCALE: f32 = 1.001;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct GlbOutlineEdgeKey {
    pub(super) instance: u16,
    pub(super) from: [i64; 3],
    to: [i64; 3],
}

#[derive(Clone, Copy)]
struct GlbOutlineEdgeRecord {
    pub(super) points: [[f32; 3]; 2],
    pub(super) matrix: GlbMatrix,
    pub(super) normal: [f32; 3],
    live: bool,
}

pub struct GlbOutlineAccumulator {
    lookup: HashMap<GlbOutlineEdgeKey, usize>,
    records: Vec<GlbOutlineEdgeRecord>,
    pub segments: Vec<[[f32; 3]; 2]>,
    pub(super) maximum_edges: usize,
    flush: usize,
}

impl GlbOutlineAccumulator {
    pub fn new(maximum_edges: usize) -> Result<Self, &'static str> {
        let mut lookup = HashMap::new();
        let mut records = Vec::new();
        let mut segments = Vec::new();
        lookup.try_reserve(maximum_edges).map_err(|_| "GLB outline lookup exceeded allocation credits")?;
        records.try_reserve_exact(maximum_edges).map_err(|_| "GLB outline records exceeded allocation credits")?;
        segments.try_reserve_exact(maximum_edges).map_err(|_| "GLB outline segments exceeded allocation credits")?;
        Ok(Self { lookup, records, segments, maximum_edges, flush: 0 })
    }

    fn point_key(point: [f32; 3]) -> Result<[i64; 3], &'static str> {
        if !point.iter().all(|value| value.is_finite()) {
            return Err("GLB outline position was not finite");
        }
        let scaled = point.map(|value| (f64::from(value) * 10_000.0 + 0.5).floor());
        if !scaled.iter().all(|value| *value >= i64::MIN as f64 && *value <= i64::MAX as f64) {
            return Err("GLB outline position exceeded weld-key credits");
        }
        Ok(scaled.map(|value| value as i64))
    }

    fn transformed(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
        glb_transform_point(matrix, point.map(|value| value * GLB_OUTLINE_SCALE))
    }

    fn push_segment(&mut self, matrix: GlbMatrix, points: [[f32; 3]; 2]) -> Result<(), &'static str> {
        if self.segments.len() == self.maximum_edges {
            return Err("GLB outline output exceeded fixed edge credits");
        }
        self.segments.push([Self::transformed(matrix, points[0]), Self::transformed(matrix, points[1])]);
        Ok(())
    }

    pub fn push_triangle(&mut self, instance: u16, matrix: GlbMatrix, points: [[f32; 3]; 3]) -> Result<(), &'static str> {
        let keys = [Self::point_key(points[0])?, Self::point_key(points[1])?, Self::point_key(points[2])?];
        if keys[0] == keys[1] || keys[1] == keys[2] || keys[2] == keys[0] {
            return Ok(());
        }
        let a = [points[1][0] - points[0][0], points[1][1] - points[0][1], points[1][2] - points[0][2]];
        let b = [points[2][0] - points[0][0], points[2][1] - points[0][1], points[2][2] - points[0][2]];
        let normal = glb_normalize([a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]);
        let threshold = 1.0_f32.to_radians().cos();
        for edge in 0..3 {
            let next = (edge + 1) % 3;
            let key = GlbOutlineEdgeKey { instance, from: keys[edge], to: keys[next] };
            let reverse = GlbOutlineEdgeKey { instance, from: keys[next], to: keys[edge] };
            if let Some(index) = self.lookup.get(&reverse).copied().filter(|index| self.records[*index].live) {
                if normal.iter().zip(self.records[index].normal.iter()).map(|(left, right)| *left * *right).sum::<f32>() <= threshold {
                    self.push_segment(matrix, [points[edge], points[next]])?;
                }
                self.records[index].live = false;
            } else if !self.lookup.contains_key(&key) {
                if self.records.len() == self.maximum_edges {
                    return Err("GLB outline topology exceeded fixed edge credits");
                }
                let index = self.records.len();
                self.records.push(GlbOutlineEdgeRecord { points: [points[edge], points[next]], matrix, normal, live: true });
                self.lookup.insert(key, index);
            }
        }
        Ok(())
    }

    pub fn flush_step(&mut self) -> Result<bool, &'static str> {
        let Some(record) = self.records.get(self.flush).copied() else { return Ok(true) };
        self.flush += 1;
        if record.live {
            self.push_segment(record.matrix, record.points)?;
        }
        Ok(self.flush == self.records.len())
    }
}
//#endregion 📡️RendererAssetAuthority
