//! 🔭️ Intrinsics of the camera model: the pinhole parameters and the lens distortion models. See [`super`].


// #region 🔖️Intrinsics
/// 📷️ Pinhole camera intrinsic parameters: focal lengths, principal point, pixel skew, and a lens distortion model.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct Intrinsics {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub skew: f64,
    pub distortion: Distortion,
}

/// 🌀️ Lens distortion model applied to normalized camera-plane coordinates before the linear intrinsic map.
#[derive(Clone, Copy, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum Distortion {
    None,
    BrownConrady { k1: f64, k2: f64, k3: f64, p1: f64, p2: f64 },
    FisheyeEquidistant { k1: f64, k2: f64, k3: f64, k4: f64 },
}

impl Distortion {
    pub(super) fn distort(&self, p: [f64; 2]) -> [f64; 2] {
        match *self {
            Self::None => p,
            Self::BrownConrady { k1, k2, k3, p1, p2 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let radial = 1.0 + k1 * r2 + k2 * r2 * r2 + k3 * r2 * r2 * r2;
                let dx = 2.0 * p1 * x * y + p2 * (r2 + 2.0 * x * x);
                let dy = p1 * (r2 + 2.0 * y * y) + 2.0 * p2 * x * y;
                [x * radial + dx, y * radial + dy]
            }
            Self::FisheyeEquidistant { k1, k2, k3, k4 } => {
                let (x, y) = (p[0], p[1]);
                let r = (x * x + y * y).sqrt();
                if r < 1e-12 {
                    return [x, y];
                }
                let theta = r.atan();
                let t2 = theta * theta;
                let theta_d = theta * (1.0 + k1 * t2 + k2 * t2 * t2 + k3 * t2 * t2 * t2 + k4 * t2 * t2 * t2 * t2);
                let scale = theta_d / r;
                [x * scale, y * scale]
            }
        }
    }
}

impl Distortion {
    /// 🔢️ Free distortion-parameter count feeding the `[fx, fy, cx, cy, skew, <distortion params>]`
    /// intrinsics-Jacobian ordering used by [`reprojection_jacobians`].
    pub(super) fn param_count(&self) -> usize {
        match self {
            Self::None => 0,
            Self::BrownConrady { .. } => 5,
            Self::FisheyeEquidistant { .. } => 4,
        }
    }

    /// 🔬️ Distorts `p` and returns alongside it the 2x2 Jacobian `d(distorted)/d(p)` and the per-parameter
    /// derivatives `d(distorted)/d(param)` (empty for [`Distortion::None`], ordered `[k1,k2,k3,p1,p2]` for
    /// [`Distortion::BrownConrady`], `[k1,k2,k3,k4]` for [`Distortion::FisheyeEquidistant`]) — the shared
    /// chain-rule kernel [`reprojection_jacobians`] builds its intrinsics/point/pose blocks on top of.
    pub(super) fn distort_with_jacobian(&self, p: [f64; 2]) -> ([f64; 2], [[f64; 2]; 2], Vec<[f64; 2]>) {
        match *self {
            Self::None => (p, [[1.0, 0.0], [0.0, 1.0]], Vec::new()),
            Self::BrownConrady { k1, k2, k3, p1, p2 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let r4 = r2 * r2;
                let r6 = r4 * r2;
                let radial = 1.0 + k1 * r2 + k2 * r4 + k3 * r6;
                let kr = k1 + 2.0 * k2 * r2 + 3.0 * k3 * r4;
                let xd = x * radial + 2.0 * p1 * x * y + p2 * (r2 + 2.0 * x * x);
                let yd = y * radial + p1 * (r2 + 2.0 * y * y) + 2.0 * p2 * x * y;
                let dxd_dx = radial + 2.0 * x * x * kr + 2.0 * p1 * y + 6.0 * p2 * x;
                let dxd_dy = 2.0 * x * y * kr + 2.0 * p1 * x + 2.0 * p2 * y;
                let dyd_dx = 2.0 * x * y * kr + 2.0 * p1 * x + 2.0 * p2 * y;
                let dyd_dy = radial + 2.0 * y * y * kr + 6.0 * p1 * y + 2.0 * p2 * x;
                let params = vec![[x * r2, y * r2], [x * r4, y * r4], [x * r6, y * r6], [2.0 * x * y, r2 + 2.0 * y * y], [r2 + 2.0 * x * x, 2.0 * x * y]];
                ([xd, yd], [[dxd_dx, dxd_dy], [dyd_dx, dyd_dy]], params)
            }
            Self::FisheyeEquidistant { k1, k2, k3, k4 } => {
                let (x, y) = (p[0], p[1]);
                let r2 = x * x + y * y;
                let r = r2.sqrt();
                if r < 1e-12 {
                    return ([x, y], [[1.0, 0.0], [0.0, 1.0]], vec![[0.0, 0.0]; 4]);
                }
                let theta = r.atan();
                let t2 = theta * theta;
                let t3 = t2 * theta;
                let t5 = t3 * t2;
                let t7 = t5 * t2;
                let t9 = t7 * t2;
                let poly = 1.0 + k1 * t2 + k2 * t2 * t2 + k3 * t2 * t2 * t2 + k4 * t2 * t2 * t2 * t2;
                let theta_d = theta * poly;
                let scale = theta_d / r;
                let dpoly_dtheta2 = k1 + 2.0 * k2 * t2 + 3.0 * k3 * t2 * t2 + 4.0 * k4 * t2 * t2 * t2;
                let dthetad_dtheta = poly + 2.0 * t2 * dpoly_dtheta2;
                let dtheta_dr = 1.0 / (1.0 + r2);
                let dthetad_dr = dthetad_dtheta * dtheta_dr;
                let dscale_dr = (dthetad_dr * r - theta_d) / r2;
                let dscale_dx = dscale_dr * x / r;
                let dscale_dy = dscale_dr * y / r;
                let dxd_dx = scale + x * dscale_dx;
                let dxd_dy = x * dscale_dy;
                let dyd_dx = y * dscale_dx;
                let dyd_dy = scale + y * dscale_dy;
                let dscale_dk = |theta_pow: f64| theta_pow / r;
                let params = vec![[x * dscale_dk(t3), y * dscale_dk(t3)], [x * dscale_dk(t5), y * dscale_dk(t5)], [x * dscale_dk(t7), y * dscale_dk(t7)], [x * dscale_dk(t9), y * dscale_dk(t9)]];
                ([x * scale, y * scale], [[dxd_dx, dxd_dy], [dyd_dx, dyd_dy]], params)
            }
        }
    }
}

impl Intrinsics {
    /// 🎯️ Projects a camera-space point to pixel coordinates, applying distortion to the normalized coordinates first; `None` when the point is behind the camera (`z <= 0`).
    pub fn project(&self, p_cam: [f64; 3]) -> Option<[f64; 2]> {
        if p_cam[2] <= 0.0 {
            return None;
        }
        let x = p_cam[0] / p_cam[2];
        let y = p_cam[1] / p_cam[2];
        let [xd, yd] = self.distortion.distort([x, y]);
        Some([self.fx * xd + self.skew * yd + self.cx, self.fy * yd + self.cy])
    }

    /// 🔬️ Newton iteration (finite-difference Jacobian, 8 steps) undoing the forward distortion map: finds normalized coordinates `p` such that `distortion.distort(p) == distorted`.
    fn newton_undistort(&self, distorted: [f64; 2]) -> [f64; 2] {
        if matches!(self.distortion, Distortion::None) {
            return distorted;
        }
        let mut p = distorted;
        let eps = 1e-6;
        for _ in 0..8 {
            let fp = self.distortion.distort(p);
            let residual = [fp[0] - distorted[0], fp[1] - distorted[1]];
            if residual[0].abs() < 1e-14 && residual[1].abs() < 1e-14 {
                break;
            }
            let fx0 = self.distortion.distort([p[0] + eps, p[1]]);
            let fy0 = self.distortion.distort([p[0], p[1] + eps]);
            let j = [[(fx0[0] - fp[0]) / eps, (fy0[0] - fp[0]) / eps], [(fx0[1] - fp[1]) / eps, (fy0[1] - fp[1]) / eps]];
            let det = j[0][0] * j[1][1] - j[0][1] * j[1][0];
            if det.abs() < 1e-300 {
                break;
            }
            let dx = (j[1][1] * residual[0] - j[0][1] * residual[1]) / det;
            let dy = (j[0][0] * residual[1] - j[1][0] * residual[0]) / det;
            p = [p[0] - dx, p[1] - dy];
        }
        p
    }

    /// ↩️ Inverse of the linear intrinsic map followed by Newton undistortion, returning a ray direction with `z = 1`.
    pub fn unproject_ray(&self, p_px: [f64; 2]) -> [f64; 3] {
        let yd = (p_px[1] - self.cy) / self.fy;
        let xd = (p_px[0] - self.cx - self.skew * yd) / self.fx;
        let [x, y] = self.newton_undistort([xd, yd]);
        [x, y, 1.0]
    }

    /// ↩️ Undistorts a normalized (post-linear-map) point via fixed-point/Newton iteration, undoing [`Distortion::distort`].
    pub fn undistort_point(&self, p_norm: [f64; 2]) -> [f64; 2] {
        self.newton_undistort(p_norm)
    }
}
// #endregion 🔖️Intrinsics
