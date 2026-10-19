//! 🧮️ Synchronous `Vec3`/`Mat4` algebra shared by every scene math crate — the plain `fn` mirror of `semio_framework_geometry`'s async inherent methods (ruling E6), verified against that crate's own algebra tests.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

pub use semio_framework_geometry::{Mat4, Vec3};

//#region 🔖️SyncAlgebra
// 🚫️async: E6 sync frame construction — `semio_framework_geometry::{Vec3, Mat4}`'s own inherent
// methods are `pub async fn` (that crate's own design, correctly `.await`-chained internally, not a
// blind-codemod bug — see its `⚙️engine/🦀️.rs`). `📐️geometry` is outside this packet's path
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

pub fn vec3_new_m(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}
pub fn vec3_from_array_m(v: [f32; 3]) -> Vec3 {
    Vec3 { x: v[0], y: v[1], z: v[2] }
}

pub trait Mat4Math: Sized {
    fn mul_m(self, other: Self) -> Self;
    fn transform_point_m(self, p: Vec3) -> Vec3;
    fn inverse_m(self) -> Self;
    fn to_cols_array_m(self) -> [f32; 16];
}

impl Mat4Math for Mat4 {
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

pub fn mat4_identity_m() -> Mat4 {
    Mat4 { cols: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
}

pub fn mat4_perspective_m(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y * 0.5).tan();
    let gl_z = (far + near) / (near - far);
    let gl_w = (2.0 * far * near) / (near - far);
    Mat4 { cols: [[f / aspect, 0.0, 0.0, 0.0], [0.0, f, 0.0, 0.0], [0.0, 0.0, 0.5 * gl_z - 0.5, -1.0], [0.0, 0.0, 0.5 * gl_w, 0.0]] }
}

/// 📐️ Symmetric parallel frustum in the same `z ∈ [0, 1]` clip convention [`mat4_perspective_m`]
/// emits — the GL form halved and biased, so both families share one depth pipeline.
pub fn mat4_orthographic_m(half_width: f32, half_height: f32, near: f32, far: f32) -> Mat4 {
    let depth = (far - near).max(1e-6);
    Mat4 { cols: [[1.0 / half_width.max(1e-6), 0.0, 0.0, 0.0], [0.0, 1.0 / half_height.max(1e-6), 0.0, 0.0], [0.0, 0.0, -1.0 / depth, 0.0], [0.0, 0.0, -near / depth, 1.0]] }
}

pub fn mat4_look_at_m(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let f = target.sub_m(eye).normalize_m();
    let s = f.cross_m(up).normalize_m();
    let u = s.cross_m(f);
    Mat4 { cols: [[s.x, u.x, -f.x, 0.0], [s.y, u.y, -f.y, 0.0], [s.z, u.z, -f.z, 0.0], [-s.dot_m(eye), -u.dot_m(eye), f.dot_m(eye), 1.0]] }
}

pub fn mat4_translation_m(v: Vec3) -> Mat4 {
    let mut m = mat4_identity_m();
    m.cols[3] = [v.x, v.y, v.z, 1.0];
    m
}

pub fn mat4_scale_vec_m(v: Vec3) -> Mat4 {
    Mat4 { cols: [[v.x, 0.0, 0.0, 0.0], [0.0, v.y, 0.0, 0.0], [0.0, 0.0, v.z, 0.0], [0.0, 0.0, 0.0, 1.0]] }
}

pub fn mat4_from_quat_m(x: f32, y: f32, z: f32, w: f32) -> Mat4 {
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

#[cfg(test)]
#[path = "../../🧪️tests/🔬️algebra-unit/🦀️.rs"]
mod tests;
