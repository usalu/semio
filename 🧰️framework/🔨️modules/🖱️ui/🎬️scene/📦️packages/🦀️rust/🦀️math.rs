//! 🌐️ Generic 3D scene math, orbit camera, mesh instances, screen picking, and draw descriptors —
//! the `math` region of `semio-framework-ui-scene`. Relocated verbatim from the old
//! `🖱️ui/🎬️scene/🦀️component.rs` (which `ui_wgpu` used to mount directly via a relative `#[path]`,
//! ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave MESH) into this
//! standalone crate, per ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet
//! `scene-surface`. `ui_wgpu` now re-exports this module instead of mounting the file.
//!
//! 🚫️async: E6 sync frame construction — every fn here arrived `async` from an old blind codemod
//! with zero internal `.await` call sites (none of these fns actually suspend), which is also why
//! the pre-existing file did not compile under `--features wgpu-engine`. Stripped to plain sync
//! `fn` wholesale rather than tagging each one individually.

pub use semio_framework_geometry::{Mat4, Vec3};

//#region 🔖️SyncAlgebra
// 🚫️async: E6 sync frame construction — `semio_framework_geometry::{Vec3, Mat4}`'s own inherent
// methods are `pub async fn` (that crate's own design, correctly `.await`-chained internally, not a
// blind-codemod bug — see its `⚙️engine/🦀️component.rs`). `📐️geometry` is outside this packet's path
// scope, so rather than either violate E6 by staying `async fn` here or editing a foreign crate, this
// region ports the exact same formulas (verified against `📐️geometry`'s own algebra tests) as plain
// sync functions/methods on the same public `{x,y,z}`/`{cols}` fields, suffixed `_m` so call sites
// never collide with `Vec3`/`Mat4`'s inherent async names.
pub trait Vec3Math: Sized {
    fn add_m(self, other: Self) -> Self;
    fn sub_m(self, other: Self) -> Self;
    fn scale_m(self, s: f32) -> Self;
    fn dot_m(self, other: Self) -> f32;
    fn cross_m(self, other: Self) -> Self;
    fn length_m(self) -> f32;
    fn normalize_m(self) -> Self;
    fn to_array_m(self) -> [f32; 3];
}

impl Vec3Math for Vec3 {
    fn add_m(self, other: Self) -> Self {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
    fn sub_m(self, other: Self) -> Self {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
    fn scale_m(self, s: f32) -> Self {
        Vec3 { x: self.x * s, y: self.y * s, z: self.z * s }
    }
    fn dot_m(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross_m(self, other: Self) -> Self {
        Vec3 { x: self.y * other.z - self.z * other.y, y: self.z * other.x - self.x * other.z, z: self.x * other.y - self.y * other.x }
    }
    fn length_m(self) -> f32 {
        self.dot_m(self).sqrt()
    }
    fn normalize_m(self) -> Self {
        let len = self.length_m();
        if len < 1e-8 {
            return Vec3::ZERO;
        }
        self.scale_m(1.0 / len)
    }
    fn to_array_m(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

fn vec3_new_m(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}
fn vec3_from_array_m(v: [f32; 3]) -> Vec3 {
    Vec3 { x: v[0], y: v[1], z: v[2] }
}

pub trait Mat4Math: Sized {
    fn mul_m(self, other: Self) -> Self;
    fn transform_point_m(self, p: Vec3) -> Vec3;
    fn inverse_m(self) -> Self;
    fn to_cols_array_m(self) -> [f32; 16];
}

impl Mat4Math for Mat4 {
    // 🚫️async: E6 sync mirror of `semio_framework_geometry::Mat4::to_cols_array` (that crate's
    // own `pub async fn`, out of packet scope per this file's header) — identical column-major
    // flatten, verified against its `mat4_to_cols_array_matches_column_major_layout` test.
    fn to_cols_array_m(self) -> [f32; 16] {
        let mut out = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                out[col * 4 + row] = self.cols[col][row];
            }
        }
        out
    }
    fn mul_m(self, other: Self) -> Self {
        let mut out = mat4_identity_m();
        for col in 0..4 {
            for row in 0..4 {
                out.cols[col][row] = self.cols[0][row] * other.cols[col][0] + self.cols[1][row] * other.cols[col][1] + self.cols[2][row] * other.cols[col][2] + self.cols[3][row] * other.cols[col][3];
            }
        }
        out
    }
    fn transform_point_m(self, p: Vec3) -> Vec3 {
        let x = p.x * self.cols[0][0] + p.y * self.cols[1][0] + p.z * self.cols[2][0] + self.cols[3][0];
        let y = p.x * self.cols[0][1] + p.y * self.cols[1][1] + p.z * self.cols[2][1] + self.cols[3][1];
        let z = p.x * self.cols[0][2] + p.y * self.cols[1][2] + p.z * self.cols[2][2] + self.cols[3][2];
        let w = p.x * self.cols[0][3] + p.y * self.cols[1][3] + p.z * self.cols[2][3] + self.cols[3][3];
        if w.abs() < 1e-8 {
            return vec3_new_m(x, y, z);
        }
        vec3_new_m(x / w, y / w, z / w)
    }
    fn inverse_m(self) -> Self {
        let mut a = [[0.0f32; 8]; 4];
        for (row, arow) in a.iter_mut().enumerate() {
            for (col, slot) in arow.iter_mut().take(4).enumerate() {
                *slot = self.cols[col][row];
            }
            arow[4 + row] = 1.0;
        }
        for pivot in 0..4 {
            let (mut best_row, mut best_val) = (pivot, a[pivot][pivot].abs());
            for (row, arow) in a.iter().enumerate().skip(pivot + 1) {
                if arow[pivot].abs() > best_val {
                    best_row = row;
                    best_val = arow[pivot].abs();
                }
            }
            if best_val < 1e-8 {
                return mat4_identity_m();
            }
            if best_row != pivot {
                a.swap(pivot, best_row);
            }
            let pivot_value = a[pivot][pivot];
            for slot in a[pivot].iter_mut() {
                *slot /= pivot_value;
            }
            let pivot_row = a[pivot];
            for (row, arow) in a.iter_mut().enumerate() {
                if row == pivot {
                    continue;
                }
                let factor = arow[pivot];
                if factor == 0.0 {
                    continue;
                }
                for (col, slot) in arow.iter_mut().enumerate() {
                    *slot -= factor * pivot_row[col];
                }
            }
        }
        let mut inv = [[0.0f32; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                inv[col][row] = a[row][4 + col];
            }
        }
        Mat4 { cols: inv }
    }
}

fn mat4_identity_m() -> Mat4 {
    Mat4 { cols: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
}

fn mat4_perspective_m(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y * 0.5).tan();
    let gl_z = (far + near) / (near - far);
    let gl_w = (2.0 * far * near) / (near - far);
    Mat4 { cols: [[f / aspect, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, 0.5 * gl_z - 0.5, -1.0], [0.0, 0.0, 0.5 * gl_w, 0.0]] }
}

fn mat4_look_at_m(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = target.sub_m(eye).normalize_m();
    let s = f.cross_m(up).normalize_m();
    let u = s.cross_m(f);
    Mat4 { cols: [[s.x, u.x, -f.x, 0.0], [s.y, u.y, -f.y, 0.0], [s.z, u.z, -f.z, 0.0], [-s.dot_m(eye), -u.dot_m(eye), f.dot_m(eye), 1.0]] }
}

fn mat4_translation_m(v: Vec3) -> Mat4 {
    let mut m = mat4_identity_m();
    m.cols[3] = [v.x, v.y, v.z, 1.0];
    m
}

fn mat4_scale_vec_m(v: Vec3) -> Mat4 {
    Mat4 { cols: [[v.x, 0.0, 0.0, 0.0], [0.0, v.y, 0.0, 0.0], [0.0, 0.0, v.z, 0.0], [0.0, 0.0, 0.0, 1.0]] }
}

fn mat4_from_quat_m(x: f32, y: f32, z: f32, w: f32) -> Mat4 {
    let xx = x * x;
    let yy = y * y;
    let zz = z * z;
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;
    let wx = w * x;
    let wy = w * y;
    let wz = w * z;
    Mat4 { cols: [[1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0], [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0], [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0], [0.0, 0.0, 0.0, 1.0]] }
}
//#endregion 🔖️SyncAlgebra

//#region Camera
#[derive(Clone, Debug)]
pub struct Camera3d {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Camera3d {
    fn default() -> Self {
        Self { position: vec3_new_m(4.0, -4.0, 3.0), target: Vec3::ZERO, up: vec3_new_m(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 }
    }
}

impl Camera3d {
    pub fn view_proj(&self, aspect: f32) -> Mat4 {
        mat4_perspective_m(self.fov_y, aspect, self.near, self.far).mul_m(mat4_look_at_m(self.position, self.target, self.up))
    }

    pub fn ray_from_screen(&self, aspect: f32, x: f32, y: f32, width: f32, height: f32) -> (Vec3, Vec3) {
        let ndc_x = (x / width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / height) * 2.0;
        let view = mat4_look_at_m(self.position, self.target, self.up);
        let proj = mat4_perspective_m(self.fov_y, aspect, self.near, self.far);
        let inv = proj.mul_m(view).inverse_m();
        let near = inv.transform_point_m(vec3_new_m(ndc_x, ndc_y, 0.0));
        let far = inv.transform_point_m(vec3_new_m(ndc_x, ndc_y, 1.0));
        let dir = far.sub_m(near).normalize_m();
        (self.position, dir)
    }
}

#[derive(Clone, Debug)]
pub struct OrbitController {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl Default for OrbitController {
    fn default() -> Self {
        Self { target: Vec3::ZERO, distance: 8.0, yaw: 0.8, pitch: 0.5, fov_y: 45.0_f32.to_radians() }
    }
}

impl OrbitController {
    pub fn from_camera(camera: &Camera3d) -> Self {
        let offset = camera.position.sub_m(camera.target);
        let distance = offset.length_m().max(0.5);
        Self { target: camera.target, distance, yaw: offset.y.atan2(offset.x), pitch: (offset.z / distance).asin(), fov_y: camera.fov_y }
    }

    pub fn to_camera(&self) -> Camera3d {
        let cp = self.pitch.cos();
        let position = vec3_new_m(self.target.x + self.distance * cp * self.yaw.cos(), self.target.y + self.distance * cp * self.yaw.sin(), self.target.z + self.distance * self.pitch.sin());
        Camera3d { position, target: self.target, up: vec3_new_m(0.0, 0.0, 1.0), fov_y: self.fov_y, near: 0.1, far: 1000.0 }
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * 0.01;
        self.pitch = (self.pitch + dy * 0.01).clamp(-1.5, 1.5);
    }

    pub fn pan(&mut self, dx: f32, dy: f32) {
        let camera = self.to_camera();
        let right = camera.position.sub_m(camera.target).cross_m(camera.up).normalize_m();
        let up = right.cross_m(camera.position.sub_m(camera.target)).normalize_m();
        let scale = self.distance * 0.001;
        self.target = self.target.add_m(right.scale_m(-dx * scale)).add_m(up.scale_m(dy * scale));
    }

    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance * (1.0 - delta * 0.001)).clamp(0.5, 500.0);
    }
}
//#endregion Camera

//#region Mesh
#[derive(Clone, Debug)]
pub struct Mesh3d {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub aabb_min: [f32; 3],
    pub aabb_max: [f32; 3],
    pub face_ids: Vec<u32>,
    pub vertex_ids: Vec<u32>,
    pub edge_positions: Vec<f32>,
    pub edge_ids: Vec<u32>,
    pub uvs: Vec<f32>,
    pub colors: Vec<f32>,
}

impl Mesh3d {
    pub fn from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> Self {
        let mut aabb_min = [f32::INFINITY; 3];
        let mut aabb_max = [f32::NEG_INFINITY; 3];
        for chunk in positions.as_chunks::<3>().0 {
            for axis in 0..3 {
                aabb_min[axis] = aabb_min[axis].min(chunk[axis]);
                aabb_max[axis] = aabb_max[axis].max(chunk[axis]);
            }
        }
        Self { positions, normals, indices, aabb_min, aabb_max, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
    }

    pub fn has_vertex_colors(&self) -> bool {
        self.colors.len() == self.positions.len()
    }
}

impl From<(&[f32], &[f32], &[u32])> for Mesh3d {
    fn from((positions, normals, indices): (&[f32], &[f32], &[u32])) -> Self {
        Self::from_buffers(positions.to_vec(), normals.to_vec(), indices.to_vec())
    }
}

#[derive(Clone, Debug)]
pub struct Instance3d {
    pub id: String,
    pub model: Mat4,
    pub color: [f32; 4],
    pub selected: bool,
    pub hovered: bool,
}

impl Instance3d {
    pub fn model_from_trs(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Mat4 {
        mat4_translation_m(vec3_from_array_m(position)).mul_m(mat4_from_quat_m(rotation[0], rotation[1], rotation[2], rotation[3])).mul_m(mat4_scale_vec_m(vec3_from_array_m(scale)))
    }
}
//#endregion Mesh

//#region ScenePass
#[derive(Clone, Debug)]
pub struct SceneDraw3d {
    pub mesh_key: String,
    pub mesh_version: u64,
    pub instances: Vec<Instance3d>,
}

#[derive(Clone, Debug, Default)]
pub struct ScenePass3d {
    pub viewport: [f32; 4],
    pub view_proj: [f32; 16],
    pub light_dir: [f32; 3],
    pub draws: Vec<SceneDraw3d>,
    pub line_draws: Vec<LineDraw3d>,
    pub translucent_draws: Vec<SceneDraw3d>,
    pub textured_draws: Vec<TexturedDraw3d>,
    pub layer_index: usize,
    pub ui_watermark: usize,
    pub vector_watermark: usize,
}
//#endregion ScenePass

//#region LineDraw
#[derive(Clone, Copy, Debug)]
pub struct LineVertex3d {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Clone, Debug, Default)]
pub struct LineDraw3d {
    pub vertices: Vec<LineVertex3d>,
}

#[derive(Clone, Debug)]
pub struct TexturedInstance3d {
    pub texture_key: String,
    pub model: Mat4,
    pub tint: [f32; 4],
}

#[derive(Clone, Debug, Default)]
pub struct TexturedDraw3d {
    pub instances: Vec<TexturedInstance3d>,
}
//#endregion LineDraw

//#region Culling
#[derive(Clone, Copy, Debug)]
pub struct FrustumPlane {
    pub normal: Vec3,
    pub distance: f32,
}

pub fn frustum_planes(view_proj: Mat4) -> [FrustumPlane; 6] {
    let m = view_proj.cols;
    let rows = [
        [m[0][0] + m[0][3], m[1][0] + m[1][3], m[2][0] + m[2][3], m[3][0] + m[3][3]],
        [m[0][3] - m[0][0], m[1][3] - m[1][0], m[2][3] - m[2][0], m[3][3] - m[3][0]],
        [m[0][3] + m[0][1], m[1][3] + m[1][1], m[2][3] + m[2][1], m[3][3] + m[3][1]],
        [m[0][3] - m[0][1], m[1][3] - m[1][1], m[2][3] - m[2][1], m[3][3] - m[3][1]],
        [m[0][3] + m[0][2], m[1][3] + m[1][2], m[2][3] + m[2][2], m[3][3] + m[3][2]],
        [m[0][3] - m[0][2], m[1][3] - m[1][2], m[2][3] - m[2][2], m[3][3] - m[3][2]],
    ];
    let mut planes = [FrustumPlane { normal: Vec3::ZERO, distance: 0.0 }; 6];
    for (index, row) in rows.into_iter().enumerate() {
        let normal = vec3_new_m(row[0], row[1], row[2]);
        let length = normal.length_m();
        if length > 1e-8 {
            planes[index] = FrustumPlane { normal: normal.scale_m(1.0 / length), distance: row[3] / length };
        }
    }
    planes
}

pub fn transform_aabb(model: Mat4, min: [f32; 3], max: [f32; 3]) -> ([f32; 3], [f32; 3]) {
    let corners = [[min[0], min[1], min[2]], [max[0], min[1], min[2]], [min[0], max[1], min[2]], [max[0], max[1], min[2]], [min[0], min[1], max[2]], [max[0], min[1], max[2]], [min[0], max[1], max[2]], [max[0], max[1], max[2]]];
    let mut out_min = [f32::INFINITY; 3];
    let mut out_max = [f32::NEG_INFINITY; 3];
    for corner in corners {
        let world = model.transform_point_m(vec3_from_array_m(corner)).to_array_m();
        for axis in 0..3 {
            out_min[axis] = out_min[axis].min(world[axis]);
            out_max[axis] = out_max[axis].max(world[axis]);
        }
    }
    (out_min, out_max)
}

pub fn aabb_intersects_frustum(planes: &[FrustumPlane; 6], min: [f32; 3], max: [f32; 3]) -> bool {
    for plane in planes {
        let positive = vec3_new_m(if plane.normal.x >= 0.0 { max[0] } else { min[0] }, if plane.normal.y >= 0.0 { max[1] } else { min[1] }, if plane.normal.z >= 0.0 { max[2] } else { min[2] });
        if plane.normal.dot_m(positive) + plane.distance < 0.0 {
            return false;
        }
    }
    true
}
//#endregion Culling

//#region Picking
pub fn ray_aabb_slab(origin: Vec3, dir: Vec3, min: [f32; 3], max: [f32; 3]) -> Option<f32> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let axes = [origin.x, origin.y, origin.z];
    let dirs = [dir.x, dir.y, dir.z];
    let bounds = [(min[0], max[0]), (min[1], max[1]), (min[2], max[2])];
    for axis in 0..3 {
        if dirs[axis].abs() < 1e-8 {
            if axes[axis] < bounds[axis].0 || axes[axis] > bounds[axis].1 {
                return None;
            }
            continue;
        }
        let inv = 1.0 / dirs[axis];
        let mut t0 = (bounds[axis].0 - axes[axis]) * inv;
        let mut t1 = (bounds[axis].1 - axes[axis]) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max < t_min {
            return None;
        }
    }
    if t_max < 0.0 {
        return None;
    }
    Some(if t_min >= 0.0 { t_min } else { t_max })
}

pub fn ray_pick_instance(origin: Vec3, dir: Vec3, mesh: &Mesh3d, instance: &Instance3d) -> Option<f32> {
    let (world_min, world_max) = transform_aabb(instance.model, mesh.aabb_min, mesh.aabb_max);
    ray_aabb_slab(origin, dir, world_min, world_max)?;
    let mut best = None;
    for tri in mesh.indices.as_chunks::<3>().0 {
        let a = instance.model.transform_point_m(vertex(mesh, tri[0]));
        let b = instance.model.transform_point_m(vertex(mesh, tri[1]));
        let c = instance.model.transform_point_m(vertex(mesh, tri[2]));
        if let Some(t) = ray_triangle(origin, dir, a, b, c) {
            best = Some(best.map_or(t, |prev: f32| prev.min(t)));
        }
    }
    best
}

pub struct RayMeshHit {
    pub distance: f32,
    pub triangle_index: usize,
    pub bary_u: f32,
    pub bary_v: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub fn ray_pick_mesh_detail(origin: Vec3, dir: Vec3, mesh: &Mesh3d, instance: &Instance3d) -> Option<RayMeshHit> {
    let (world_min, world_max) = transform_aabb(instance.model, mesh.aabb_min, mesh.aabb_max);
    ray_aabb_slab(origin, dir, world_min, world_max)?;
    let mut best: Option<RayMeshHit> = None;
    for (triangle_index, tri) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
        let a = instance.model.transform_point_m(vertex(mesh, tri[0]));
        let b = instance.model.transform_point_m(vertex(mesh, tri[1]));
        let c = instance.model.transform_point_m(vertex(mesh, tri[2]));
        if let Some((t, u, v)) = ray_triangle_barycentric(origin, dir, a, b, c) {
            if best.as_ref().is_none_or(|hit| t < hit.distance) {
                let point = origin.add_m(dir.scale_m(t));
                let edge1 = b.sub_m(a);
                let edge2 = c.sub_m(a);
                let mut normal = edge1.cross_m(edge2);
                if normal.length_m() > 1e-6 {
                    normal = normal.normalize_m();
                }
                best = Some(RayMeshHit { distance: t, triangle_index, bary_u: u, bary_v: v, point, normal });
            }
        }
    }
    best
}

fn ray_triangle_barycentric(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let edge1 = b.sub_m(a);
    let edge2 = c.sub_m(a);
    let h = dir.cross_m(edge2);
    let det = edge1.dot_m(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / det;
    let s = origin.sub_m(a);
    let u = f * s.dot_m(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross_m(edge1);
    let v = f * dir.dot_m(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * edge2.dot_m(q);
    if t > 1e-4 {
        Some((t, u, v))
    } else {
        None
    }
}

pub fn interpolate_mesh_uv(mesh: &Mesh3d, triangle_index: usize, bary_u: f32, bary_v: f32) -> Option<(f32, f32)> {
    if mesh.uvs.len() < 6 {
        return None;
    }
    let base = triangle_index * 3;
    let tri = mesh.indices.get(base..base + 3)?;
    let uv = |index: u32| {
        let i = index as usize * 2;
        (mesh.uvs[i], mesh.uvs[i + 1])
    };
    let (u0, v0) = uv(tri[0]);
    let (u1, v1) = uv(tri[1]);
    let (u2, v2) = uv(tri[2]);
    let w = 1.0 - bary_u - bary_v;
    Some((u0 * w + u1 * bary_u + u2 * bary_v, v0 * w + v1 * bary_u + v2 * bary_v))
}

pub const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f32 = 2.0;

pub fn marquee_is_crossing(start_x: f32, end_x: f32) -> bool {
    end_x < start_x
}

pub fn marquee_is_crossing_from_path(path: &[[f32; 2]], is_lasso: bool) -> bool {
    let Some(start) = path.first() else {
        return false;
    };
    if is_lasso {
        for point in path.iter().skip(1) {
            let dx = point[0] - start[0];
            if dx.abs() >= SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                return dx < 0.0;
            }
        }
    }
    let end = path.last().copied().unwrap_or(*start);
    marquee_is_crossing(start[0], end[0])
}

fn orient2d(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn point_on_segment(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> bool {
    p[0] >= a[0].min(b[0]) && p[0] <= a[0].max(b[0]) && p[1] >= a[1].min(b[1]) && p[1] <= a[1].max(b[1])
}

fn segments_intersect(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
    let o1 = orient2d(a0, a1, b0);
    let o2 = orient2d(a0, a1, b1);
    let o3 = orient2d(b0, b1, a0);
    let o4 = orient2d(b0, b1, a1);
    if o1 == 0.0 && point_on_segment(b0, a0, a1) {
        return true;
    }
    if o2 == 0.0 && point_on_segment(b1, a0, a1) {
        return true;
    }
    if o3 == 0.0 && point_on_segment(a0, b0, b1) {
        return true;
    }
    if o4 == 0.0 && point_on_segment(a1, b0, b1) {
        return true;
    }
    (o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0)
}

fn rect_corners(rect: [f32; 4]) -> [[f32; 2]; 4] {
    let min_x = rect[0].min(rect[2]);
    let max_x = rect[0].max(rect[2]);
    let min_y = rect[1].min(rect[3]);
    let max_y = rect[1].max(rect[3]);
    [[min_x, min_y], [max_x, min_y], [max_x, max_y], [min_x, max_y]]
}

fn segment_intersects_rect(a: [f32; 2], b: [f32; 2], rect: [f32; 4]) -> bool {
    let corners = rect_corners(rect);
    for index in 0..corners.len() {
        let c0 = corners[index];
        let c1 = corners[(index + 1) % corners.len()];
        if segments_intersect(a, b, c0, c1) {
            return true;
        }
    }
    false
}

fn segment_intersects_polygon(a: [f32; 2], b: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    if point_in_polygon(a, polygon) || point_in_polygon(b, polygon) {
        return true;
    }
    for index in 0..polygon.len() {
        let j = if index == 0 { polygon.len() - 1 } else { index - 1 };
        if segments_intersect(a, b, polygon[index], polygon[j]) {
            return true;
        }
    }
    false
}

fn marquee_contains_point(point: [f32; 2], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>) -> bool {
    if rectangle {
        rect_bounds.is_some_and(|bounds| rect_contains(bounds, point))
    } else {
        point_in_polygon(point, polygon)
    }
}

fn marquee_segment_selected(a: [f32; 2], b: [f32; 2], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>, crossing: bool) -> bool {
    if crossing {
        if marquee_contains_point(a, polygon, rectangle, rect_bounds) || marquee_contains_point(b, polygon, rectangle, rect_bounds) {
            return true;
        }
        if rectangle {
            rect_bounds.is_some_and(|bounds| segment_intersects_rect(a, b, bounds))
        } else {
            segment_intersects_polygon(a, b, polygon)
        }
    } else {
        marquee_contains_point(a, polygon, rectangle, rect_bounds) && marquee_contains_point(b, polygon, rectangle, rect_bounds)
    }
}

fn marquee_triangle_selected(points: &[[f32; 2]; 3], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>, crossing: bool) -> bool {
    if crossing {
        if points.iter().any(|point| marquee_contains_point(*point, polygon, rectangle, rect_bounds)) {
            return true;
        }
        for index in 0..3 {
            let next = (index + 1) % 3;
            if marquee_segment_selected(points[index], points[next], polygon, rectangle, rect_bounds, true) {
                return true;
            }
        }
        false
    } else {
        points.iter().all(|point| marquee_contains_point(*point, polygon, rectangle, rect_bounds))
    }
}

fn marquee_rect_bounds(polygon: &[[f32; 2]]) -> Option<[f32; 4]> {
    let first = polygon.first()?;
    let mut min_x = first[0];
    let mut max_x = first[0];
    let mut min_y = first[1];
    let mut max_y = first[1];
    for point in polygon.iter().skip(1) {
        min_x = min_x.min(point[0]);
        max_x = max_x.max(point[0]);
        min_y = min_y.min(point[1]);
        max_y = max_y.max(point[1]);
    }
    Some([min_x, min_y, max_x, max_y])
}

/// 🎯️ Screen-space component (vertex/edge/face) picking within a marquee/lasso polygon; kept as a flat argument
/// list rather than a params struct because both `infinite/world/rs` call sites pass through the same shape
/// positionally and this crate must not restructure a signature consumed outside its own scope.
#[allow(clippy::too_many_arguments, reason = "flat picking-context args match the two infinite/world/rs call sites; a params struct would be a cross-crate signature change out of this crate's scope")]
pub fn screen_select_components(
    mesh_lookup: &std::collections::HashMap<String, Mesh3d>,
    draws: &[SceneDraw3d],
    view_proj: Mat4,
    width: f32,
    height: f32,
    polygon: &[[f32; 2]],
    rectangle: bool,
    granularity: &str,
    active_instance_id: Option<&str>,
    crossing: bool,
) -> Vec<String> {
    use std::collections::HashSet;
    let mut selected = HashSet::new();
    let local_polygon: Vec<[f32; 2]> = polygon.iter().map(|point| [point[0], point[1]]).collect();
    let rect_bounds = marquee_rect_bounds(&local_polygon);
    for draw in draws {
        let Some(mesh) = mesh_lookup.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if active_instance_id.is_some_and(|active| instance.id != active) {
                continue;
            }
            match granularity {
                "vertex" if !mesh.vertex_ids.is_empty() => {
                    for (vertex_index, chunk) in mesh.positions.as_chunks::<3>().0.iter().enumerate() {
                        let world = instance.model.transform_point_m(vec3_new_m(chunk[0], chunk[1], chunk[2]));
                        let Some(screen) = project_point(view_proj, world, width, height) else {
                            continue;
                        };
                        let point = [screen[0], screen[1]];
                        let inside = marquee_contains_point(point, &local_polygon, rectangle, rect_bounds);
                        if inside {
                            let id = mesh.vertex_ids.get(vertex_index).map(|value| value.to_string()).unwrap_or_else(|| vertex_index.to_string());
                            selected.insert(id);
                        }
                    }
                }
                "edge" if !mesh.edge_positions.is_empty() => {
                    for (edge_index, chunk) in mesh.edge_positions.as_chunks::<6>().0.iter().enumerate() {
                        let a_world = instance.model.transform_point_m(vec3_new_m(chunk[0], chunk[1], chunk[2]));
                        let b_world = instance.model.transform_point_m(vec3_new_m(chunk[3], chunk[4], chunk[5]));
                        let (Some(a_screen), Some(b_screen)) = (project_point(view_proj, a_world, width, height), project_point(view_proj, b_world, width, height)) else {
                            continue;
                        };
                        if !marquee_segment_selected(a_screen, b_screen, &local_polygon, rectangle, rect_bounds, crossing) {
                            continue;
                        }
                        let id = mesh.edge_ids.get(edge_index).map(|value| value.to_string()).unwrap_or_else(|| edge_index.to_string());
                        selected.insert(id);
                    }
                }
                "face" => {
                    for (tri_index, tri) in mesh.indices.as_chunks::<3>().0.iter().enumerate() {
                        let mut screens = [[0.0_f32; 2]; 3];
                        let mut visible = 0usize;
                        for (slot, index) in tri.iter().enumerate() {
                            let world = instance.model.transform_point_m(vertex(mesh, *index));
                            if let Some(screen) = project_point(view_proj, world, width, height) {
                                screens[slot] = screen;
                                visible += 1;
                            }
                        }
                        if visible < 3 {
                            continue;
                        }
                        if !marquee_triangle_selected(&screens, &local_polygon, rectangle, rect_bounds, crossing) {
                            continue;
                        }
                        let id = mesh.face_ids.get(tri_index).map(|value| value.to_string()).unwrap_or_else(|| tri_index.to_string());
                        selected.insert(id);
                    }
                }
                _ => {
                    let Some(projected) = projected_aabb_bounds(view_proj, instance.model, mesh.aabb_min, mesh.aabb_max, width, height) else {
                        continue;
                    };
                    if aabb_overlaps_marquee(projected, &local_polygon, rectangle) {
                        selected.insert(instance.id.clone());
                    }
                }
            }
        }
    }
    selected.into_iter().collect()
}

fn vertex(mesh: &Mesh3d, index: u32) -> Vec3 {
    let i = index as usize * 3;
    vec3_new_m(mesh.positions[i], mesh.positions[i + 1], mesh.positions[i + 2])
}

pub fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<f32> {
    let edge1 = b.sub_m(a);
    let edge2 = c.sub_m(a);
    let h = dir.cross_m(edge2);
    let det = edge1.dot_m(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / det;
    let s = origin.sub_m(a);
    let u = f * s.dot_m(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross_m(edge1);
    let v = f * dir.dot_m(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * edge2.dot_m(q);
    if t > 1e-4 {
        Some(t)
    } else {
        None
    }
}

pub fn project_point(view_proj: Mat4, point: Vec3, width: f32, height: f32) -> Option<[f32; 2]> {
    let clip = view_proj.transform_point_m(point);
    if clip.z < 0.0 || clip.z > 1.0 {
        return None;
    }
    Some([(clip.x * 0.5 + 0.5) * width, (1.0 - (clip.y * 0.5 + 0.5)) * height])
}

pub fn screen_segment_distance(px: f32, py: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let abx = bx - ax;
    let aby = by - ay;
    let len_sq = abx * abx + aby * aby;
    if len_sq < 1e-6 {
        let dx = px - ax;
        let dy = py - ay;
        return (dx * dx + dy * dy).sqrt();
    }
    let t = ((px - ax) * abx + (py - ay) * aby) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let cx = ax + abx * t;
    let cy = ay + aby * t;
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy).sqrt()
}

pub fn point_in_polygon(point: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut winding = 0;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        if a[1] <= point[1] {
            if b[1] > point[1] && cross2(a, b, point) > 0.0 {
                winding += 1;
            }
        } else if b[1] <= point[1] && cross2(a, b, point) < 0.0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn cross2(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

pub fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
    let min_x = rect[0].min(rect[2]);
    let max_x = rect[0].max(rect[2]);
    let min_y = rect[1].min(rect[3]);
    let max_y = rect[1].max(rect[3]);
    point[0] >= min_x && point[0] <= max_x && point[1] >= min_y && point[1] <= max_y
}

pub fn projected_aabb_bounds(view_proj: Mat4, model: Mat4, min: [f32; 3], max: [f32; 3], width: f32, height: f32) -> Option<[f32; 4]> {
    let corners = [[min[0], min[1], min[2]], [max[0], min[1], min[2]], [min[0], max[1], min[2]], [max[0], max[1], min[2]], [min[0], min[1], max[2]], [max[0], min[1], max[2]], [min[0], max[1], max[2]], [max[0], max[1], max[2]]];
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut visible = false;
    for corner in corners {
        let world = model.transform_point_m(vec3_from_array_m(corner));
        if let Some(screen) = project_point(view_proj, world, width, height) {
            visible = true;
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
    }
    visible.then_some([min_x, min_y, max_x, max_y])
}

fn aabb_overlaps_marquee(projected: [f32; 4], polygon: &[[f32; 2]], rectangle: bool) -> bool {
    if rectangle {
        let Some(marquee) = marquee_rect_bounds(polygon) else {
            return false;
        };
        let marquee_min_x = marquee[0].min(marquee[2]);
        let marquee_max_x = marquee[0].max(marquee[2]);
        let marquee_min_y = marquee[1].min(marquee[3]);
        let marquee_max_y = marquee[1].max(marquee[3]);
        return projected[2] >= marquee_min_x && projected[0] <= marquee_max_x && projected[3] >= marquee_min_y && projected[1] <= marquee_max_y;
    }
    let corners = [[projected[0], projected[1]], [projected[2], projected[1]], [projected[2], projected[3]], [projected[0], projected[3]]];
    corners.iter().any(|corner| point_in_polygon(*corner, polygon)) || polygon.iter().any(|point| point[0] >= projected[0] && point[0] <= projected[2] && point[1] >= projected[1] && point[1] <= projected[3])
}

/// 🎯️ Screen-space whole-instance picking within a marquee/lasso polygon; kept as a flat argument list rather
/// than a params struct for the same cross-crate-scope reason as `screen_select_components`.
#[allow(clippy::too_many_arguments, reason = "flat picking-context args match the two infinite/world/rs call sites; a params struct would be a cross-crate signature change out of this crate's scope")]
pub fn screen_select_instances(mesh_lookup: &std::collections::HashMap<String, Mesh3d>, draws: &[SceneDraw3d], view_proj: Mat4, width: f32, height: f32, polygon: &[[f32; 2]], rectangle: bool, crossing: bool) -> Vec<String> {
    let rect_bounds = marquee_rect_bounds(polygon);
    let mut selected = Vec::new();
    for draw in draws {
        let Some(mesh) = mesh_lookup.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if !crossing {
                let mut all_inside = true;
                let mut any_visible = false;
                for chunk in mesh.positions.as_chunks::<3>().0 {
                    let world = instance.model.transform_point_m(vec3_new_m(chunk[0], chunk[1], chunk[2]));
                    if let Some(screen) = project_point(view_proj, world, width, height) {
                        any_visible = true;
                        if !marquee_contains_point(screen, polygon, rectangle, rect_bounds) {
                            all_inside = false;
                            break;
                        }
                    }
                }
                if any_visible && all_inside {
                    selected.push(instance.id.clone());
                }
                continue;
            }
            let Some(projected) = projected_aabb_bounds(view_proj, instance.model, mesh.aabb_min, mesh.aabb_max, width, height) else {
                continue;
            };
            if !aabb_overlaps_marquee(projected, polygon, rectangle) {
                continue;
            }
            let mut covered = false;
            for tri in mesh.indices.as_chunks::<3>().0 {
                let mut screens = [[0.0_f32; 2]; 3];
                let mut visible = 0usize;
                for (slot, &index) in tri.iter().enumerate() {
                    let world = instance.model.transform_point_m(vertex(mesh, index));
                    if let Some(screen) = project_point(view_proj, world, width, height) {
                        screens[slot] = screen;
                        visible += 1;
                    }
                }
                if visible < 3 {
                    continue;
                }
                if marquee_triangle_selected(&screens, polygon, rectangle, rect_bounds, true) {
                    covered = true;
                    break;
                }
            }
            if covered {
                selected.push(instance.id.clone());
            }
        }
    }
    selected
}
//#endregion Picking

//#region GumballMath
pub fn vec3_from_f64(values: [f64; 3]) -> Vec3 {
    vec3_new_m(values[0] as f32, values[1] as f32, values[2] as f32)
}

pub fn gumball_extent(camera_distance: f32) -> f32 {
    (camera_distance * 0.15).clamp(0.25, 2.5)
}

pub fn gumball_eye(camera: &Camera3d, pivot: Vec3) -> Vec3 {
    camera.position.sub_m(pivot).normalize_m()
}

pub fn ray_plane_point(origin: Vec3, dir: Vec3, plane_point: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let denom = plane_normal.dot_m(dir);
    if denom.abs() < 1e-6 {
        return None;
    }
    let t = plane_normal.dot_m(plane_point.sub_m(origin)) / denom;
    if t < 0.0 {
        return None;
    }
    Some(origin.add_m(dir.scale_m(t)))
}

pub fn gumball_axis_drag_plane_normal(axis: Vec3, eye: Vec3) -> Vec3 {
    let axis = axis.normalize_m();
    let align = eye.cross_m(axis);
    if align.length_m() > 1e-6 {
        return align.cross_m(axis).normalize_m();
    }
    if axis.z.abs() < 0.9 {
        vec3_new_m(0.0, 0.0, 1.0).cross_m(axis).normalize_m()
    } else {
        vec3_new_m(0.0, 1.0, 0.0).cross_m(axis).normalize_m()
    }
}

pub fn gumball_project_ray_onto_axis(origin: Vec3, dir: Vec3, pivot: Vec3, axis: Vec3, eye: Vec3) -> Option<f32> {
    let plane_normal = gumball_axis_drag_plane_normal(axis, eye);
    let hit = ray_plane_point(origin, dir, pivot, plane_normal)?;
    Some(hit.sub_m(pivot).dot_m(axis.normalize_m()))
}

pub fn ray_segment_distance(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3) -> Option<f32> {
    let ab = b.sub_m(a);
    let len_sq = ab.dot_m(ab);
    if len_sq < 1e-8 {
        return None;
    }
    let t = origin.sub_m(a).dot_m(ab) / len_sq;
    let t_clamped = t.clamp(0.0, 1.0);
    let closest = a.add_m(ab.scale_m(t_clamped));
    let w = origin.sub_m(closest);
    let b_val = dir.dot_m(w);
    let c = w.dot_m(w);
    let denom = 1.0 - b_val * b_val;
    let dist_sq = if denom.abs() < 1e-6 { c } else { c - b_val * b_val / denom };
    Some(dist_sq.max(0.0).sqrt())
}

pub fn quat_from_basis(x: Vec3, y: Vec3, z: Vec3) -> [f32; 4] {
    let m00 = x.x;
    let m01 = y.x;
    let m02 = z.x;
    let m10 = x.y;
    let m11 = y.y;
    let m12 = z.y;
    let m20 = x.z;
    let m21 = y.z;
    let m22 = z.z;
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, 0.25 * s]
    } else if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        [0.25 * s, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    } else if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        [(m01 + m10) / s, 0.25 * s, (m12 + m21) / s, (m02 - m20) / s]
    } else {
        let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
        [(m02 + m20) / s, (m12 + m21) / s, 0.25 * s, (m10 - m01) / s]
    }
}

pub fn rotate_vector(vector: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let axis = axis.normalize_m();
    let cos = angle.cos();
    let sin = angle.sin();
    vector.scale_m(cos).add_m(axis.cross_m(vector).scale_m(sin)).add_m(axis.scale_m(axis.dot_m(vector) * (1.0 - cos)))
}

pub fn axis_rotate_angle(start: Vec3, current: Vec3, axis: Vec3) -> f32 {
    let axis = axis.normalize_m();
    let project = |v: Vec3| v.sub_m(axis.scale_m(v.dot_m(axis)));
    let a = project(start).normalize_m();
    let b = project(current).normalize_m();
    let mut angle = a.dot_m(b).clamp(-1.0, 1.0).acos();
    if axis.dot_m(a.cross_m(b)) < 0.0 {
        angle = -angle;
    }
    angle
}
//#endregion GumballMath

//#region LodGrid
pub const LOD_GRID_MAJOR_QUANTUM: f64 = 10.0;
pub const LOD_GRID_MEDIUM_QUANTUM: f64 = 2.5;
pub const LOD_GRID_SMALL_QUANTUM: f64 = 0.5;
pub const LOD_GRID_MICRO_QUANTUM: f64 = 0.1;
pub const WORLD_LOD_GRID_MAX_LOD: f64 = 1000.0;
pub const WORLD_LOD_GRID_MEDIUM_MAX_LOD: f64 = 50.0;
pub const WORLD_LOD_GRID_SMALL_MAX_LOD: f64 = 10.0;
pub const WORLD_LOD_GRID_MICRO_MAX_LOD: f64 = 2.0;
pub const LOD_GRID_LAYER_OPACITY: [f32; 4] = [1.0, 0.72, 0.48, 0.32];

pub fn lod_from_camera_distance(distance: f64, reference: f64) -> f64 {
    let d = distance.max(1e-6);
    let reference = reference.max(1e-6);
    d / reference
}

pub fn pick_closest_lod(available: &[f64], desired: f64) -> Option<f64> {
    if available.is_empty() || !desired.is_finite() || desired <= 0.0 {
        return None;
    }
    let mut best = available[0];
    if !best.is_finite() || best <= 0.0 {
        return None;
    }
    let mut best_dist = (best.ln() - desired.ln()).abs();
    for &rep in available.iter().skip(1) {
        if !rep.is_finite() || rep <= 0.0 {
            continue;
        }
        let dist = (rep.ln() - desired.ln()).abs();
        if dist < best_dist - 1e-12 || ((dist - best_dist).abs() <= 1e-12 && rep < best) {
            best = rep;
            best_dist = dist;
        }
    }
    Some(best)
}

pub fn pick_closest_mesh_url<'a>(entries: &'a [(f64, &'a str)], desired: f64, fallback: Option<&'a str>) -> Option<&'a str> {
    if entries.is_empty() {
        return fallback;
    }
    let lods: Vec<f64> = entries.iter().filter_map(|(lod, _)| (*lod > 0.0 && lod.is_finite()).then_some(*lod)).collect();
    let picked = pick_closest_lod(&lods, desired)?;
    entries.iter().find(|(lod, _)| (*lod - picked).abs() < 1e-12).map(|(_, url)| *url).or(fallback)
}

pub fn lod_grid_band_steps_world(grid_factor: f64) -> [f64; 4] {
    [LOD_GRID_MAJOR_QUANTUM * grid_factor, LOD_GRID_MEDIUM_QUANTUM * grid_factor, LOD_GRID_SMALL_QUANTUM * grid_factor, LOD_GRID_MICRO_QUANTUM * grid_factor]
}

pub fn lod_progressive_grid_layers(lod: f64, grid_factor: f64) -> Vec<(f64, f32)> {
    if !lod.is_finite() || lod <= 0.0 || lod > WORLD_LOD_GRID_MAX_LOD {
        return Vec::new();
    }
    let [large, medium, small, micro] = lod_grid_band_steps_world(grid_factor);
    let mut layers = vec![(large, LOD_GRID_LAYER_OPACITY[0])];
    if lod <= WORLD_LOD_GRID_MEDIUM_MAX_LOD {
        layers.push((medium, LOD_GRID_LAYER_OPACITY[1]));
    }
    if lod <= WORLD_LOD_GRID_SMALL_MAX_LOD {
        layers.push((small, LOD_GRID_LAYER_OPACITY[2]));
    }
    if lod <= WORLD_LOD_GRID_MICRO_MAX_LOD {
        layers.push((micro, LOD_GRID_LAYER_OPACITY[3]));
    }
    layers
}

pub fn lod_progressive_grid_layer_key(lod: f64, grid_factor: f64) -> String {
    let layers = lod_progressive_grid_layers(lod, grid_factor);
    if layers.is_empty() {
        return String::new();
    }
    layers.iter().map(|(step, _)| step.to_string()).collect::<Vec<_>>().join("|")
}

pub fn lod_grid_step_world(lod: f64, grid_factor: f64) -> Option<f64> {
    let layers = lod_progressive_grid_layers(lod, grid_factor);
    layers.last().map(|(step, _)| *step)
}

pub fn floating_origin_rebase(world: Vec3, anchor: Vec3) -> Vec3 {
    vec3_new_m(world.x - anchor.x, world.y - anchor.y, world.z - anchor.z)
}

pub fn grid_placement_anchor(orbit_target: Vec3, datum: [f64; 3]) -> Vec3 {
    vec3_new_m(orbit_target.x, orbit_target.y, datum[2] as f32)
}
//#endregion LodGrid

#[cfg(test)]
mod tests {
    use super::*;

    fn test_box_mesh() -> Mesh3d {
        Mesh3d::from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
    }

    #[test]
    fn orbit_round_trip() {
        let camera = Camera3d::default();
        let orbit = OrbitController::from_camera(&camera);
        let next = orbit.to_camera();
        assert!((next.position.x - camera.position.x).abs() < 0.5);
    }

    #[test]
    fn point_in_square() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        assert!(point_in_polygon([5.0, 5.0], &square));
        assert!(!point_in_polygon([20.0, 5.0], &square));
    }

    #[test]
    fn mat4_inverse_round_trips_to_identity() {
        let m = mat4_translation_m(vec3_new_m(3.0, -2.0, 5.0)).mul_m(mat4_from_quat_m(0.1, 0.2, 0.05, 0.9701425)).mul_m(mat4_scale_vec_m(vec3_new_m(2.0, 1.5, 0.5)));
        let round_trip = m.mul_m(m.inverse_m());
        let identity = mat4_identity_m();
        for col in 0..4 {
            for row in 0..4 {
                assert!((round_trip.cols[col][row] - identity.cols[col][row]).abs() < 1e-4, "mismatch at col={col} row={row}: {} vs {}", round_trip.cols[col][row], identity.cols[col][row]);
            }
        }
    }

    #[test]
    fn mat4_inverse_undoes_view_projection() {
        let camera = Camera3d::default();
        let view = mat4_look_at_m(camera.position, camera.target, camera.up);
        let proj = mat4_perspective_m(camera.fov_y, 1.5, camera.near, camera.far);
        let view_proj = proj.mul_m(view);
        let inv = view_proj.inverse_m();
        let world_point = vec3_new_m(1.0, 2.0, 0.5);
        let clip_point = view_proj.transform_point_m(world_point);
        let unprojected = inv.transform_point_m(clip_point);
        assert!((unprojected.x - world_point.x).abs() < 1e-3, "x mismatch: {}", unprojected.x);
        assert!((unprojected.y - world_point.y).abs() < 1e-3, "y mismatch: {}", unprojected.y);
        assert!((unprojected.z - world_point.z).abs() < 1e-3, "z mismatch: {}", unprojected.z);
    }

    #[test]
    fn ray_from_screen_center_points_at_target() {
        let camera = Camera3d::default();
        let aspect = 1.6;
        let (origin, dir) = camera.ray_from_screen(aspect, 400.0, 300.0, 800.0, 600.0);
        assert!((origin.x - camera.position.x).abs() < 1e-4);
        let to_target = camera.target.sub_m(camera.position).normalize_m();
        let dot = dir.dot_m(to_target);
        assert!(dot > 0.999, "ray from screen center should point at target, dot={dot}");
    }

    #[test]
    fn ray_hits_triangle_direct() {
        let hit = ray_triangle(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(-1.0, -1.0, 0.0), vec3_new_m(1.0, -1.0, 0.0), vec3_new_m(0.0, 1.0, 0.0));
        assert!(hit.is_some());
    }

    #[test]
    fn ray_hits_box() {
        let mesh = test_box_mesh();
        let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
        let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), &mesh, &instance);
        assert!(hit.is_some());
    }

    #[test]
    fn ray_aabb_misses_offset_box() {
        let mesh = test_box_mesh();
        let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(100.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
        let hit = ray_pick_instance(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), &mesh, &instance);
        assert!(hit.is_none());
    }

    #[test]
    fn frustum_contains_origin_box() {
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let planes = frustum_planes(view_proj);
        assert!(aabb_intersects_frustum(&planes, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]));
    }

    #[test]
    fn frustum_culls_behind_camera_box() {
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let planes = frustum_planes(view_proj);
        assert!(aabb_intersects_frustum(&planes, [-0.5, -0.5, -0.5], [0.5, 0.5, 0.5]));
        let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
        let min = [behind.x - 0.1, behind.y - 0.1, behind.z - 0.1];
        let max = [behind.x + 0.1, behind.y + 0.1, behind.z + 0.1];
        assert!(!aabb_intersects_frustum(&planes, min, max));
    }

    fn concrete_forest_camera() -> Camera3d {
        Camera3d { position: vec3_new_m(30.0, -30.0, 20.0), target: vec3_new_m(7.0, 0.0, 3.0), up: vec3_new_m(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 }
    }

    #[test]
    fn concrete_forest_frustum_contains_target_box() {
        let camera = concrete_forest_camera();
        let view_proj = camera.view_proj(1.0);
        let planes = frustum_planes(view_proj);
        let target = camera.target;
        for plane in &planes {
            let distance = plane.normal.dot_m(target) + plane.distance;
            assert!(distance >= -1e-3, "look-at must be inside frustum, distance={distance}");
        }
        assert!(aabb_intersects_frustum(&planes, [6.0, -1.0, 2.0], [8.0, 1.0, 4.0]));
    }

    #[test]
    fn concrete_forest_frustum_culls_off_axis_boxes() {
        let camera = concrete_forest_camera();
        let view_proj = camera.view_proj(1.0);
        let planes = frustum_planes(view_proj);
        assert!(!aabb_intersects_frustum(&planes, [7.0, 0.0, 200.0], [8.0, 1.0, 201.0]));
        let behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(4.0));
        let min = [behind.x - 0.25, behind.y - 0.25, behind.z - 0.25];
        let max = [behind.x + 0.25, behind.y + 0.25, behind.z + 0.25];
        assert!(!aabb_intersects_frustum(&planes, min, max));
    }

    #[test]
    fn perspective_maps_depth_to_wgpu_ndc() {
        let near = 0.1_f32;
        let far = 100.0_f32;
        let proj = mat4_perspective_m(45.0_f32.to_radians(), 1.0, near, far);
        let near_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -near));
        let far_pt = proj.transform_point_m(vec3_new_m(0.0, 0.0, -far));
        assert!((near_pt.z - 0.0).abs() < 1e-4, "near z={}", near_pt.z);
        assert!((far_pt.z - 1.0).abs() < 1e-3, "far z={}", far_pt.z);
    }

    #[test]
    fn rectangle_marquee_bounds_use_start_and_end_corners() {
        let bounds = marquee_rect_bounds(&[[10.0, 10.0], [200.0, 10.0], [200.0, 200.0], [10.0, 200.0]]).expect("bounds");
        assert!(rect_contains(bounds, [100.0, 100.0]));
        assert!(!rect_contains(bounds, [5.0, 100.0]));
    }

    #[test]
    fn projected_aabb_skips_far_instance() {
        let mesh = test_box_mesh();
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "far".into(), model: mat4_translation_m(vec3_new_m(0.0, 0.0, -500.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let ids = screen_select_instances(&lookup, &draws, view_proj, 200.0, 200.0, &[[0.0, 0.0], [200.0, 0.0], [200.0, 200.0], [0.0, 200.0]], true, true);
        assert!(ids.is_empty());
    }

    #[test]
    fn marquee_is_crossing_follows_drag_direction() {
        assert!(marquee_is_crossing(100.0, 80.0));
        assert!(!marquee_is_crossing(80.0, 100.0));
    }

    #[test]
    fn marquee_is_crossing_from_path_lasso_uses_first_horizontal_step() {
        let left_first = [[100.0, 100.0], [80.0, 100.0], [120.0, 100.0]];
        let right_first = [[100.0, 100.0], [120.0, 100.0], [80.0, 100.0]];
        assert!(marquee_is_crossing_from_path(&left_first, true));
        assert!(!marquee_is_crossing_from_path(&right_first, true));
    }

    #[test]
    fn screen_select_instances_window_requires_full_vertex_enclosure() {
        let mesh = test_box_mesh();
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "partial".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let width = 800.0;
        let height = 600.0;
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        for corner in [[-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], [0.0, 1.0, 0.0]] {
            let screen = project_point(view_proj, vec3_from_array_m(corner), width, height).expect("screen");
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
        let center_x = (min_x + max_x) * 0.5;
        let center_y = (min_y + max_y) * 0.5;
        let partial = [[min_x, min_y], [center_x, center_y]];
        let window_ids = screen_select_instances(&lookup, &draws, view_proj, width, height, &partial, true, false);
        let crossing_ids = screen_select_instances(&lookup, &draws, view_proj, width, height, &partial, true, true);
        assert!(window_ids.is_empty());
        assert_eq!(crossing_ids, vec!["partial".to_string()]);
    }

    #[test]
    fn lod_from_camera_distance_scales() {
        assert!((lod_from_camera_distance(100.0, 100.0) - 1.0).abs() < 1e-6);
        assert!((lod_from_camera_distance(20000.0, 100.0) - 200.0).abs() < 1e-6);
    }

    #[test]
    fn lod_progressive_grid_layers_adds_bands() {
        assert!(lod_progressive_grid_layers(5000.0, 10.0).is_empty());
        assert_eq!(lod_progressive_grid_layers(500.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0]);
        assert_eq!(lod_progressive_grid_layers(50.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0]);
        assert_eq!(lod_progressive_grid_layers(2.0, 10.0).iter().map(|(step, _)| *step).collect::<Vec<_>>(), vec![100.0, 25.0, 5.0, 1.0]);
    }

    #[test]
    fn lod_progressive_grid_layer_key_stable_within_band() {
        let key_a = lod_progressive_grid_layer_key(50.0, 10.0);
        let key_b = lod_progressive_grid_layer_key(49.2, 10.0);
        let key_c = lod_progressive_grid_layer_key(11.4, 10.0);
        assert_eq!(key_a, key_b);
        assert_eq!(key_a, key_c);
        assert_eq!(lod_progressive_grid_layer_key(9.8, 10.0), "100|25|5");
    }

    #[test]
    fn pick_closest_lod_prefers_more_detailed_on_tie() {
        let picked = pick_closest_lod(&[1.0, 2.0, 4.0], 2.0).unwrap();
        assert!((picked - 2.0).abs() < 1e-6);
    }

    #[test]
    fn floating_origin_rebase_subtracts_anchor() {
        let rebased = floating_origin_rebase(vec3_new_m(10.0, 20.0, 30.0), vec3_new_m(1.0, 2.0, 3.0));
        assert_eq!(rebased, vec3_new_m(9.0, 18.0, 27.0));
    }

    #[test]
    fn mesh_has_vertex_colors_requires_matching_length() {
        let mut mesh = test_box_mesh();
        assert!(!mesh.has_vertex_colors());
        mesh.colors = vec![1.0; mesh.positions.len()];
        assert!(mesh.has_vertex_colors());
    }

    #[test]
    fn instance_model_from_trs_translates_and_scales_point() {
        let model = Instance3d::model_from_trs([1.0, 2.0, 3.0], [0.0, 0.0, 0.0, 1.0], [2.0, 2.0, 2.0]);
        let point = model.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
        assert!((point.x - 3.0).abs() < 1e-5, "x={}", point.x);
        assert!((point.y - 2.0).abs() < 1e-5, "y={}", point.y);
        assert!((point.z - 3.0).abs() < 1e-5, "z={}", point.z);
    }

    #[test]
    fn orbit_controller_orbit_clamps_pitch() {
        let mut orbit = OrbitController::default();
        orbit.pitch = 1.49;
        orbit.orbit(0.0, 1000.0);
        assert!((orbit.pitch - 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
        orbit.pitch = -1.49;
        orbit.orbit(0.0, -1000.0);
        assert!((orbit.pitch + 1.5).abs() < 1e-5, "pitch={}", orbit.pitch);
    }

    #[test]
    fn orbit_controller_pan_moves_target_away_from_origin() {
        let mut orbit = OrbitController::default();
        let start = orbit.target;
        orbit.pan(50.0, 0.0);
        assert!(orbit.target.sub_m(start).length_m() > 0.0);
    }

    #[test]
    fn orbit_controller_zoom_clamps_distance_bounds() {
        let mut orbit = OrbitController::default();
        orbit.distance = 1.0;
        orbit.zoom(100_000.0);
        assert!((orbit.distance - 0.5).abs() < 1e-4, "distance={}", orbit.distance);
        orbit.distance = 400.0;
        orbit.zoom(-100_000.0);
        assert!((orbit.distance - 500.0).abs() < 1e-4, "distance={}", orbit.distance);
    }

    #[test]
    fn ray_pick_mesh_detail_returns_triangle_index_and_barycentrics() {
        let mesh = test_box_mesh();
        let instance = Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
        let hit = ray_pick_mesh_detail(vec3_new_m(0.0, -0.5, -5.0), vec3_new_m(0.0, 0.0, 1.0), &mesh, &instance).expect("hit");
        assert_eq!(hit.triangle_index, 0);
        assert!(hit.bary_u >= 0.0 && hit.bary_v >= 0.0 && hit.bary_u + hit.bary_v <= 1.0);
    }

    #[test]
    fn ray_pick_mesh_detail_misses_when_aabb_not_hit() {
        let mesh = test_box_mesh();
        let instance = Instance3d { id: "box".into(), model: mat4_translation_m(vec3_new_m(50.0, 0.0, 0.0)), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false };
        assert!(ray_pick_mesh_detail(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), &mesh, &instance).is_none());
    }

    #[test]
    fn interpolate_mesh_uv_none_when_uvs_missing() {
        let mesh = test_box_mesh();
        assert!(interpolate_mesh_uv(&mesh, 0, 0.25, 0.25).is_none());
    }

    #[test]
    fn interpolate_mesh_uv_blends_triangle_corners() {
        let mut mesh = test_box_mesh();
        mesh.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let (u, v) = interpolate_mesh_uv(&mesh, 0, 0.0, 0.0).expect("uv");
        assert!((u - 0.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
        let (u, v) = interpolate_mesh_uv(&mesh, 0, 1.0, 0.0).expect("uv");
        assert!((u - 1.0).abs() < 1e-6 && (v - 0.0).abs() < 1e-6);
    }

    #[test]
    fn interpolate_mesh_uv_none_when_triangle_out_of_range() {
        let mut mesh = test_box_mesh();
        mesh.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        assert!(interpolate_mesh_uv(&mesh, 5, 0.0, 0.0).is_none());
    }

    #[test]
    fn marquee_is_crossing_from_path_window_mode_uses_endpoints() {
        let path = [[100.0, 100.0], [50.0, 100.0]];
        assert!(marquee_is_crossing_from_path(&path, false));
        let path = [[50.0, 100.0], [100.0, 100.0]];
        assert!(!marquee_is_crossing_from_path(&path, false));
    }

    #[test]
    fn marquee_is_crossing_from_path_empty_defaults_to_false() {
        let path: [[f32; 2]; 0] = [];
        assert!(!marquee_is_crossing_from_path(&path, true));
    }

    #[test]
    fn segments_intersect_detects_proper_crossing() {
        assert!(segments_intersect([0.0, 0.0], [10.0, 10.0], [0.0, 10.0], [10.0, 0.0]));
        assert!(!segments_intersect([0.0, 0.0], [10.0, 0.0], [0.0, 5.0], [10.0, 5.0]));
    }

    #[test]
    fn segments_intersect_detects_collinear_touch() {
        assert!(segments_intersect([0.0, 0.0], [10.0, 0.0], [5.0, 0.0], [20.0, 0.0]));
    }

    #[test]
    fn point_on_segment_checks_bounding_box() {
        assert!(point_on_segment([5.0, 0.0], [0.0, 0.0], [10.0, 0.0]));
        assert!(!point_on_segment([20.0, 0.0], [0.0, 0.0], [10.0, 0.0]));
    }

    #[test]
    fn segment_intersects_rect_detects_boundary_crossing() {
        let rect = [0.0, 0.0, 10.0, 10.0];
        assert!(segment_intersects_rect([-5.0, 5.0], [5.0, 5.0], rect));
        assert!(!segment_intersects_rect([-5.0, 20.0], [-1.0, 20.0], rect));
    }

    #[test]
    fn segment_intersects_polygon_true_when_endpoint_inside() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        assert!(segment_intersects_polygon([5.0, 5.0], [50.0, 50.0], &square));
        assert!(!segment_intersects_polygon([50.0, 50.0], [60.0, 60.0], &square));
    }

    #[test]
    fn marquee_contains_point_rectangle_vs_polygon_modes() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let bounds = marquee_rect_bounds(&square);
        assert!(marquee_contains_point([5.0, 5.0], &square, true, bounds));
        assert!(!marquee_contains_point([5.0, 5.0], &square, true, None));
        assert!(marquee_contains_point([5.0, 5.0], &square, false, bounds));
    }

    #[test]
    fn marquee_segment_selected_window_mode_requires_both_endpoints_inside() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let bounds = marquee_rect_bounds(&square);
        assert!(marquee_segment_selected([2.0, 2.0], [8.0, 8.0], &square, true, bounds, false));
        assert!(!marquee_segment_selected([2.0, 2.0], [20.0, 20.0], &square, true, bounds, false));
    }

    #[test]
    fn marquee_segment_selected_crossing_mode_detects_rect_edge_crossing() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let bounds = marquee_rect_bounds(&square);
        assert!(marquee_segment_selected([-5.0, 5.0], [15.0, 5.0], &square, true, bounds, true));
        assert!(!marquee_segment_selected([-5.0, 20.0], [-1.0, 20.0], &square, true, bounds, true));
    }

    #[test]
    fn marquee_triangle_selected_window_mode_requires_all_points_inside() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let bounds = marquee_rect_bounds(&square);
        let inside = [[1.0, 1.0], [2.0, 2.0], [3.0, 1.0]];
        let partial = [[1.0, 1.0], [2.0, 2.0], [30.0, 30.0]];
        assert!(marquee_triangle_selected(&inside, &square, true, bounds, false));
        assert!(!marquee_triangle_selected(&partial, &square, true, bounds, false));
    }

    #[test]
    fn marquee_triangle_selected_crossing_mode_true_on_partial_overlap() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let bounds = marquee_rect_bounds(&square);
        let straddling = [[5.0, 5.0], [20.0, 20.0], [20.0, 5.0]];
        assert!(marquee_triangle_selected(&straddling, &square, true, bounds, true));
    }

    #[test]
    fn aabb_overlaps_marquee_polygon_mode_detects_corner_containment() {
        let square = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        assert!(aabb_overlaps_marquee([2.0, 2.0, 8.0, 8.0], &square, false));
        assert!(!aabb_overlaps_marquee([100.0, 100.0, 110.0, 110.0], &square, false));
    }

    #[test]
    fn aabb_overlaps_marquee_rectangle_mode_returns_false_without_bounds() {
        let empty: [[f32; 2]; 0] = [];
        assert!(!aabb_overlaps_marquee([0.0, 0.0, 5.0, 5.0], &empty, true));
    }

    #[test]
    fn screen_select_components_face_granularity_selects_visible_triangle() {
        let mut mesh = test_box_mesh();
        mesh.face_ids = vec![42];
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "face", None, false);
        assert_eq!(selected, vec!["42".to_string()]);
    }

    #[test]
    fn screen_select_components_vertex_granularity_selects_ids() {
        let mut mesh = test_box_mesh();
        mesh.vertex_ids = vec![10, 11, 12];
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let mut selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "vertex", None, false);
        selected.sort();
        assert_eq!(selected, vec!["10".to_string(), "11".to_string(), "12".to_string()]);
    }

    #[test]
    fn screen_select_components_edge_granularity_selects_ids() {
        let mut mesh = test_box_mesh();
        mesh.edge_positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0];
        mesh.edge_ids = vec![99];
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "box".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "edge", None, false);
        assert_eq!(selected, vec!["99".to_string()]);
    }

    #[test]
    fn screen_select_components_default_granularity_selects_whole_instance() {
        let mesh = test_box_mesh();
        let draws = vec![SceneDraw3d { mesh_key: "box".into(), mesh_version: 0, instances: vec![Instance3d { id: "whole".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "unknown", None, false);
        assert_eq!(selected, vec!["whole".to_string()]);
    }

    #[test]
    fn screen_select_components_filters_by_active_instance_id() {
        let mesh = test_box_mesh();
        let draws = vec![SceneDraw3d {
            mesh_key: "box".into(),
            mesh_version: 0,
            instances: vec![
                Instance3d { id: "keep".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false },
                Instance3d { id: "skip".into(), model: mat4_identity_m(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false },
            ],
        }];
        let mut lookup = std::collections::HashMap::new();
        lookup.insert("box".into(), mesh);
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "unknown", Some("keep"), false);
        assert_eq!(selected, vec!["keep".to_string()]);
    }

    #[test]
    fn screen_select_components_skips_missing_mesh_lookup() {
        let draws = vec![SceneDraw3d { mesh_key: "missing".into(), mesh_version: 0, instances: vec![] }];
        let lookup = std::collections::HashMap::new();
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let full_screen = [[0.0, 0.0], [800.0, 0.0], [800.0, 600.0], [0.0, 600.0]];
        let selected = screen_select_components(&lookup, &draws, view_proj, 800.0, 600.0, &full_screen, true, "face", None, false);
        assert!(selected.is_empty());
    }

    #[test]
    fn screen_segment_distance_projects_point_onto_segment() {
        let dist = screen_segment_distance(5.0, 5.0, 0.0, 0.0, 10.0, 0.0);
        assert!((dist - 5.0).abs() < 1e-4, "dist={dist}");
        let dist_beyond_end = screen_segment_distance(20.0, 0.0, 0.0, 0.0, 10.0, 0.0);
        assert!((dist_beyond_end - 10.0).abs() < 1e-4, "dist={dist_beyond_end}");
    }

    #[test]
    fn screen_segment_distance_degenerate_segment_falls_back_to_point_distance() {
        let dist = screen_segment_distance(3.0, 4.0, 0.0, 0.0, 0.0, 0.0);
        assert!((dist - 5.0).abs() < 1e-4, "dist={dist}");
    }

    #[test]
    fn project_point_rejects_points_outside_near_far_clip() {
        let camera = Camera3d::default();
        let view_proj = camera.view_proj(1.0);
        let far_behind = camera.position.add_m(camera.position.sub_m(camera.target).normalize_m().scale_m(2.0));
        assert!(project_point(view_proj, far_behind, 800.0, 600.0).is_none());
        assert!(project_point(view_proj, camera.target, 800.0, 600.0).is_some());
    }

    #[test]
    fn ray_aabb_slab_axis_parallel_ray_outside_bounds_misses() {
        let origin = vec3_new_m(5.0, 0.0, 0.0);
        let dir = vec3_new_m(0.0, 0.0, 1.0);
        assert!(ray_aabb_slab(origin, dir, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]).is_none());
    }

    #[test]
    fn ray_aabb_slab_returns_none_when_box_entirely_behind_origin() {
        let origin = vec3_new_m(0.0, 0.0, -10.0);
        let dir = vec3_new_m(0.0, 0.0, -1.0);
        assert!(ray_aabb_slab(origin, dir, [-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]).is_none());
    }

    #[test]
    fn vec3_from_f64_converts_components() {
        let v = vec3_from_f64([1.5, -2.5, 3.5]);
        assert_eq!(v, vec3_new_m(1.5, -2.5, 3.5));
    }

    #[test]
    fn gumball_extent_clamps_to_bounds() {
        assert!((gumball_extent(0.0) - 0.25).abs() < 1e-6);
        assert!((gumball_extent(1000.0) - 2.5).abs() < 1e-6);
        assert!((gumball_extent(10.0) - 1.5).abs() < 1e-6);
    }

    #[test]
    fn gumball_eye_points_from_pivot_to_camera() {
        let camera = Camera3d { position: vec3_new_m(0.0, 0.0, 10.0), target: Vec3::ZERO, up: vec3_new_m(0.0, 1.0, 0.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 100.0 };
        let eye = gumball_eye(&camera, Vec3::ZERO);
        assert!((eye.length_m() - 1.0).abs() < 1e-5);
        assert!(eye.z > 0.99);
    }

    #[test]
    fn ray_plane_point_hits_plane_ahead_and_misses_parallel() {
        let hit = ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).expect("hit");
        assert!((hit.z - 0.0).abs() < 1e-5, "z={}", hit.z);
        assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(1.0, 0.0, 0.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
    }

    #[test]
    fn ray_plane_point_rejects_intersection_behind_origin() {
        assert!(ray_plane_point(vec3_new_m(0.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, -1.0), Vec3::ZERO, vec3_new_m(0.0, 0.0, 1.0)).is_none());
    }

    #[test]
    fn gumball_axis_drag_plane_normal_is_perpendicular_to_axis() {
        let axis = vec3_new_m(1.0, 0.0, 0.0);
        let eye = vec3_new_m(0.0, 0.0, 1.0);
        let normal = gumball_axis_drag_plane_normal(axis, eye);
        assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
    }

    #[test]
    fn gumball_axis_drag_plane_normal_handles_axis_aligned_with_eye() {
        let axis = vec3_new_m(0.0, 0.0, 1.0);
        let eye = vec3_new_m(0.0, 0.0, 1.0);
        let normal = gumball_axis_drag_plane_normal(axis, eye);
        assert!(normal.dot_m(axis).abs() < 1e-5, "dot={}", normal.dot_m(axis));
        assert!((normal.length_m() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn gumball_project_ray_onto_axis_measures_signed_offset() {
        let pivot = Vec3::ZERO;
        let axis = vec3_new_m(1.0, 0.0, 0.0);
        let eye = vec3_new_m(0.0, 0.0, 1.0);
        let offset = gumball_project_ray_onto_axis(vec3_new_m(3.0, 0.0, -5.0), vec3_new_m(0.0, 0.0, 1.0), pivot, axis, eye).expect("offset");
        assert!((offset - 3.0).abs() < 1e-4, "offset={offset}");
    }

    #[test]
    fn ray_segment_distance_measures_perpendicular_gap() {
        let dist = ray_segment_distance(vec3_new_m(3.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), vec3_new_m(0.0, -5.0, 0.0), vec3_new_m(0.0, 5.0, 0.0)).expect("distance");
        assert!((dist - 3.0).abs() < 1e-4, "dist={dist}");
    }

    #[test]
    fn ray_segment_distance_none_for_degenerate_segment() {
        assert!(ray_segment_distance(Vec3::ZERO, vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(5.0, 5.0, 5.0), vec3_new_m(5.0, 5.0, 5.0)).is_none());
    }

    #[test]
    fn quat_from_basis_identity_axes_yields_identity_quaternion() {
        let q = quat_from_basis(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
        assert!((q[0]).abs() < 1e-5 && (q[1]).abs() < 1e-5 && (q[2]).abs() < 1e-5, "q={q:?}");
        assert!((q[3] - 1.0).abs() < 1e-5, "q={q:?}");
    }

    #[test]
    fn quat_from_basis_round_trips_through_mat4_from_quat() {
        let q = quat_from_basis(vec3_new_m(0.0, 1.0, 0.0), vec3_new_m(-1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0));
        let m = mat4_from_quat_m(q[0], q[1], q[2], q[3]);
        let rotated = m.transform_point_m(vec3_new_m(1.0, 0.0, 0.0));
        assert!((rotated.x - 0.0).abs() < 1e-4 && (rotated.y - 1.0).abs() < 1e-4, "rotated={rotated:?}");
    }

    #[test]
    fn rotate_vector_by_90_degrees_around_z_axis() {
        let rotated = rotate_vector(vec3_new_m(1.0, 0.0, 0.0), vec3_new_m(0.0, 0.0, 1.0), std::f32::consts::FRAC_PI_2);
        assert!((rotated.x - 0.0).abs() < 1e-4, "x={}", rotated.x);
        assert!((rotated.y - 1.0).abs() < 1e-4, "y={}", rotated.y);
    }

    #[test]
    fn axis_rotate_angle_measures_signed_rotation() {
        let axis = vec3_new_m(0.0, 0.0, 1.0);
        let start = vec3_new_m(1.0, 0.0, 0.0);
        let current = vec3_new_m(0.0, 1.0, 0.0);
        let angle = axis_rotate_angle(start, current, axis);
        assert!((angle - std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle}");
        let angle_reverse = axis_rotate_angle(start, vec3_new_m(0.0, -1.0, 0.0), axis);
        assert!((angle_reverse + std::f32::consts::FRAC_PI_2).abs() < 1e-3, "angle={angle_reverse}");
    }

    #[test]
    fn pick_closest_mesh_url_returns_fallback_when_no_entries() {
        let entries: [(f64, &str); 0] = [];
        assert_eq!(pick_closest_mesh_url(&entries, 5.0, Some("fallback.glb")), Some("fallback.glb"));
    }

    #[test]
    fn pick_closest_mesh_url_selects_nearest_lod_entry() {
        let entries = [(1.0, "hi.glb"), (10.0, "mid.glb"), (100.0, "lo.glb")];
        assert_eq!(pick_closest_mesh_url(&entries, 8.0, None), Some("mid.glb"));
    }

    #[test]
    fn pick_closest_mesh_url_filters_out_non_finite_and_negative_lods() {
        let entries = [(-1.0, "bad.glb"), (f64::NAN, "nan.glb"), (5.0, "good.glb")];
        assert_eq!(pick_closest_mesh_url(&entries, 3.0, None), Some("good.glb"));
    }

    #[test]
    fn lod_grid_step_world_returns_finest_active_band() {
        assert_eq!(lod_grid_step_world(5000.0, 10.0), None);
        let step = lod_grid_step_world(1.0, 10.0).expect("step");
        assert!((step - 1.0).abs() < 1e-9, "step={step}");
    }

    #[test]
    fn grid_placement_anchor_uses_orbit_xy_and_datum_z() {
        let anchor = grid_placement_anchor(vec3_new_m(3.0, 4.0, 999.0), [0.0, 0.0, 12.5]);
        assert_eq!(anchor, vec3_new_m(3.0, 4.0, 12.5));
    }
}
