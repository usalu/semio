//! 🧮️ Affine representation geometry matrices.
#[derive(Clone,Debug,PartialEq)]
pub struct Mat4(pub [[f64; 4]; 4]);

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::identity()
    }
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4([[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]])
    }
    /// ✖️ `self * other` (self applied after other — i.e. `self` is the outer/parent transform).
    pub fn mul(&self, other: &Mat4) -> Mat4 {
        let (a, b) = (&self.0, &other.0);
        let mut out = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                out[i][j] = (0..4).map(|k| a[i][k] * b[k][j]).sum();
            }
        }
        Mat4(out)
    }
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.0;
        [m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3], m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3], m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3]]
    }
}


/// 🧭️ Pure finite normalization lets retained workers refuse invalid input without allocating prose.
pub fn normalize_direction(value:[f64;3])->Option<[f64;3]>{let largest=value.map(f64::abs).into_iter().fold(0.,f64::max);if !largest.is_finite()||largest==0.{return None}let unit=value.map(|v|v/largest);let length=unit[0].hypot(unit[1]).hypot(unit[2]);Some(unit.map(|v|v/length))}
