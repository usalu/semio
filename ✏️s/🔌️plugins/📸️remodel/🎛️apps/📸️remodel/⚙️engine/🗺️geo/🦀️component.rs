//! 🌍️ Georeferencing and survey products: GCPs, geodetic transforms, DSM/DTM, orthomosaics, contours, volumes and quality reports.

// 🔗️ Sibling engine topic files, aliased to their pre-merge crate names so every path in
// this file is byte-identical to the crate it was moved from (see 📦️glue.rs for the wiring).
use crate::apps::remodel::engine::{camera as remodel_camera, dense as remodel_dense, images as remodel_image, mesh as remodel_mesh, sfm as remodel_sfm};

use std::collections::HashMap;

pub use math::lie::Sim3;
pub use remodel_camera::{CameraPose, Intrinsics};
pub use remodel_mesh::WatertightReport;
pub use remodel_sfm::Reconstruction;

use math::algebra::{pseudo_inverse, solve_llsq, vec3d_normalize, vec3d_sub, MatD, VecD};
use math::lie::umeyama;
use math::optimize::{camera_covariances, LmConfig, ResidualTerm, SchurResult};
use math::spatial::KdTree;
use remodel_camera::{reproject, reprojection_jacobians, reprojection_residual};
use remodel_dense::{PointClass, PointCloud};
use remodel_image::ImageRgba8;
use remodel_sfm::{apply_gcp_prior_residual, SfmBundleProblem};

// #region 🔖️Geodesy
/// 🌐️ WGS84 semi-major axis (metres). <https://en.wikipedia.org/wiki/World_Geodetic_System>
const WGS84_A: f64 = 6_378_137.0;
/// 🌐️ WGS84 flattening `f = (a - b) / a`.
const WGS84_F: f64 = 1.0 / 298.257_223_563;
/// 📐️ UTM central-meridian scale factor.
const UTM_K0: f64 = 0.9996;
/// 📐️ UTM false easting (metres), added to every zone's central-meridian-relative easting.
const UTM_FALSE_EASTING: f64 = 500_000.0;
/// 📐️ UTM false northing (metres) added south of the equator so northings stay positive.
const UTM_FALSE_NORTHING_SOUTH: f64 = 10_000_000.0;

/// 🌐️ WGS84 first eccentricity squared `e² = f(2 - f)`.
fn wgs84_e2() -> f64 {
    WGS84_F * (2.0 - WGS84_F)
}

/// 🌐️ WGS84 first eccentricity `e`.
fn wgs84_e() -> f64 {
    wgs84_e2().sqrt()
}

/// 🌐️ WGS84 semi-minor axis `b = a(1 - f)`.
fn wgs84_b() -> f64 {
    WGS84_A * (1.0 - WGS84_F)
}

/// 🌐️ WGS84 third flattening `n = f / (2 - f)`, the small parameter the Krüger series is expanded in.
fn wgs84_n() -> f64 {
    WGS84_F / (2.0 - WGS84_F)
}

/// 🌐️ Converts geodetic `(lat, lon, height)` (radians, radians, metres above the ellipsoid) to
/// Earth-Centered-Earth-Fixed Cartesian coordinates via the closed-form prime-vertical-radius formula.
pub fn geodetic_to_ecef(lat_rad: f64, lon_rad: f64, height_m: f64) -> [f64; 3] {
    let (sin_lat, cos_lat) = (lat_rad.sin(), lat_rad.cos());
    let (sin_lon, cos_lon) = (lon_rad.sin(), lon_rad.cos());
    let prime_vertical = WGS84_A / (1.0 - wgs84_e2() * sin_lat * sin_lat).sqrt();
    let x = (prime_vertical + height_m) * cos_lat * cos_lon;
    let y = (prime_vertical + height_m) * cos_lat * sin_lon;
    let z = (prime_vertical * (1.0 - wgs84_e2()) + height_m) * sin_lat;
    [x, y, z]
}

/// 🌐️ Converts ECEF Cartesian coordinates to geodetic `(lat, lon, height)` via the standard fixed-point
/// iteration on the prime-vertical radius (Hofmann-Wellenhof et al., converges to double precision in a
/// handful of iterations for Earth's eccentricity); the polar singularity (`x = y = 0`) is handled directly.
pub fn ecef_to_geodetic(ecef: [f64; 3]) -> (f64, f64, f64) {
    let [x, y, z] = ecef;
    let lon = y.atan2(x);
    let r = (x * x + y * y).sqrt();
    if r < 1e-9 {
        let lat = if z >= 0.0 { std::f64::consts::FRAC_PI_2 } else { -std::f64::consts::FRAC_PI_2 };
        return (lat, lon, z.abs() - wgs84_b());
    }
    let e2 = wgs84_e2();
    let mut lat = z.atan2(r * (1.0 - e2));
    let mut prime_vertical = WGS84_A;
    for _ in 0..10 {
        let sin_lat = lat.sin();
        prime_vertical = WGS84_A / (1.0 - e2 * sin_lat * sin_lat).sqrt();
        lat = (z + e2 * prime_vertical * sin_lat).atan2(r);
    }
    let height = r / lat.cos() - prime_vertical;
    (lat, lon, height)
}

/// 🧭️ 3x3 rotation columns mapping an ECEF delta vector into East-North-Up at the given geodetic reference.
fn enu_rotation(ref_lat: f64, ref_lon: f64) -> [[f64; 3]; 3] {
    let (sin_lat, cos_lat) = (ref_lat.sin(), ref_lat.cos());
    let (sin_lon, cos_lon) = (ref_lon.sin(), ref_lon.cos());
    [[-sin_lon, cos_lon, 0.0], [-sin_lat * cos_lon, -sin_lat * sin_lon, cos_lat], [cos_lat * cos_lon, cos_lat * sin_lon, sin_lat]]
}

/// 🧭️ Converts an ECEF point into the local East-North-Up tangent plane centred at the given geodetic reference.
pub fn ecef_to_enu(ecef: [f64; 3], ref_lat_rad: f64, ref_lon_rad: f64, ref_height_m: f64) -> [f64; 3] {
    let origin = geodetic_to_ecef(ref_lat_rad, ref_lon_rad, ref_height_m);
    let d = vec3d_sub(ecef, origin);
    let rot = enu_rotation(ref_lat_rad, ref_lon_rad);
    std::array::from_fn(|row| rot[row][0] * d[0] + rot[row][1] * d[1] + rot[row][2] * d[2])
}

/// 🧭️ Converts a local East-North-Up point back to ECEF, the inverse of [`ecef_to_enu`].
pub fn enu_to_ecef(enu: [f64; 3], ref_lat_rad: f64, ref_lon_rad: f64, ref_height_m: f64) -> [f64; 3] {
    let origin = geodetic_to_ecef(ref_lat_rad, ref_lon_rad, ref_height_m);
    let rot = enu_rotation(ref_lat_rad, ref_lon_rad);
    let d: [f64; 3] = std::array::from_fn(|col| rot[0][col] * enu[0] + rot[1][col] * enu[1] + rot[2][col] * enu[2]);
    [origin[0] + d[0], origin[1] + d[1], origin[2] + d[2]]
}

/// 🧭️ Geodetic `(lat, lon, height)` straight to a local ENU frame, composing [`geodetic_to_ecef`] and [`ecef_to_enu`].
pub fn geodetic_to_enu(lat_rad: f64, lon_rad: f64, height_m: f64, ref_lat_rad: f64, ref_lon_rad: f64, ref_height_m: f64) -> [f64; 3] {
    ecef_to_enu(geodetic_to_ecef(lat_rad, lon_rad, height_m), ref_lat_rad, ref_lon_rad, ref_height_m)
}

/// 🧭️ Local ENU straight to geodetic `(lat, lon, height)`, composing [`enu_to_ecef`] and [`ecef_to_geodetic`].
pub fn enu_to_geodetic(enu: [f64; 3], ref_lat_rad: f64, ref_lon_rad: f64, ref_height_m: f64) -> (f64, f64, f64) {
    ecef_to_geodetic(enu_to_ecef(enu, ref_lat_rad, ref_lon_rad, ref_height_m))
}

/// 📐️ UTM zone number (1-60) covering a longitude, ignoring the Svalbard/Norway irregular-zone exceptions.
pub fn utm_zone_number(lon_deg: f64) -> u32 {
    let z = ((lon_deg + 180.0) / 6.0).floor() as i64 + 1;
    z.clamp(1, 60) as u32
}

/// 📐️ Central meridian (degrees) of a UTM zone.
pub fn utm_central_meridian_deg(zone: u32) -> f64 {
    f64::from(zone) * 6.0 - 183.0
}

/// 📐️ A projected UTM coordinate: easting/northing in metres plus the zone/hemisphere it was computed in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UtmCoord {
    pub easting: f64,
    pub northing: f64,
    pub zone: u32,
    pub northern: bool,
}

/// 📐️ Krüger meridional-arc scale constant `A = a/(1+n)(1 + n²/4 + n⁴/64 + n⁶/256)`.
/// <https://en.wikipedia.org/wiki/Transverse_Mercator_projection>
fn kruger_meridian_constant() -> f64 {
    let n = wgs84_n();
    let n2 = n * n;
    WGS84_A / (1.0 + n) * (1.0 + n2 / 4.0 + n2 * n2 / 64.0 + n2 * n2 * n2 / 256.0)
}

/// 📐️ The six Krüger forward-series coefficients `alpha_1..alpha_6`, hard-coded to 6th order in `n`
/// (Karney's "Transverse Mercator with an accuracy of a few nanometers" formulation).
fn kruger_alpha() -> [f64; 6] {
    let n = wgs84_n();
    let n2 = n * n;
    let n3 = n2 * n;
    let n4 = n3 * n;
    let n5 = n4 * n;
    let n6 = n5 * n;
    [
        n / 2.0 - 2.0 / 3.0 * n2 + 5.0 / 16.0 * n3 + 41.0 / 180.0 * n4 - 127.0 / 288.0 * n5 + 7891.0 / 37800.0 * n6,
        13.0 / 48.0 * n2 - 3.0 / 5.0 * n3 + 557.0 / 1440.0 * n4 + 281.0 / 630.0 * n5 - 1_983_433.0 / 1_935_360.0 * n6,
        61.0 / 240.0 * n3 - 103.0 / 140.0 * n4 + 15061.0 / 26880.0 * n5 + 167_603.0 / 181_440.0 * n6,
        49561.0 / 161_280.0 * n4 - 179.0 / 168.0 * n5 + 6_601_661.0 / 7_257_600.0 * n6,
        34729.0 / 80640.0 * n5 - 3_418_889.0 / 1_995_840.0 * n6,
        212_378_941.0 / 319_334_400.0 * n6,
    ]
}

/// 📐️ Ellipsoidal transverse-Mercator forward projection (pre false-easting/northing, pre-`k0` origin
/// shift already folded in) via the conformal-latitude + Krüger series construction: `(easting_local,
/// northing_local)` relative to the given central meridian.
fn tm_forward_raw(lat_rad: f64, lon_rad: f64, lon0_rad: f64) -> (f64, f64) {
    let e = wgs84_e();
    let sin_lat = lat_rad.sin();
    let psi = sin_lat.atanh() - e * (e * sin_lat).atanh();
    let t = psi.sinh();
    let omega = lon_rad - lon0_rad;
    let xi_p = t.atan2(omega.cos());
    let eta_p = (omega.sin() / (t * t + omega.cos().powi(2)).sqrt()).asinh();
    let alpha = kruger_alpha();
    let mut xi = xi_p;
    let mut eta = eta_p;
    for (k0, &a_k) in alpha.iter().enumerate() {
        let k = (k0 + 1) as f64;
        xi += a_k * (2.0 * k * xi_p).sin() * (2.0 * k * eta_p).cosh();
        eta += a_k * (2.0 * k * xi_p).cos() * (2.0 * k * eta_p).sinh();
    }
    let scale = UTM_K0 * kruger_meridian_constant();
    (scale * eta, scale * xi)
}

/// 📐️ Inverts [`tm_forward_raw`] via Newton polishing (central-difference 2x2 Jacobian) seeded from a
/// crude spherical approximation — guarantees the round trip stays tight regardless of the forward
/// series' truncation order, the same defensive-Newton idiom [`remodel_camera`]'s lens-undistortion uses.
fn tm_inverse_raw(x: f64, y: f64, lon0_rad: f64) -> (f64, f64) {
    let scale = UTM_K0 * kruger_meridian_constant();
    let mut lat = (y / scale).clamp(-1.5, 1.5);
    let mut lon = lon0_rad + x / (scale * lat.cos().max(0.05));
    let eps = 1e-6;
    for _ in 0..25 {
        let (fx, fy) = tm_forward_raw(lat, lon, lon0_rad);
        let rx = fx - x;
        let ry = fy - y;
        if rx.abs() < 1e-10 && ry.abs() < 1e-10 {
            break;
        }
        let (fx_lat, fy_lat) = tm_forward_raw(lat + eps, lon, lon0_rad);
        let (fx_lon, fy_lon) = tm_forward_raw(lat, lon + eps, lon0_rad);
        let j00 = (fx_lat - fx) / eps;
        let j01 = (fx_lon - fx) / eps;
        let j10 = (fy_lat - fy) / eps;
        let j11 = (fy_lon - fy) / eps;
        let det = j00 * j11 - j01 * j10;
        if det.abs() < 1e-300 {
            break;
        }
        lat -= (j11 * rx - j01 * ry) / det;
        lon -= (j00 * ry - j10 * rx) / det;
    }
    (lat, lon)
}

/// 📐️ Projects a geodetic point (radians) to UTM, auto-selecting the zone from the longitude.
pub fn geodetic_to_utm(lat_rad: f64, lon_rad: f64) -> UtmCoord {
    let zone = utm_zone_number(lon_rad.to_degrees());
    let lon0 = utm_central_meridian_deg(zone).to_radians();
    let (x, y) = tm_forward_raw(lat_rad, lon_rad, lon0);
    let northern = lat_rad >= 0.0;
    let northing = if northern { y } else { y + UTM_FALSE_NORTHING_SOUTH };
    UtmCoord { easting: x + UTM_FALSE_EASTING, northing, zone, northern }
}

/// 📐️ Inverse of [`geodetic_to_utm`]: recovers `(lat_rad, lon_rad)` from a UTM coordinate.
pub fn utm_to_geodetic(coord: &UtmCoord) -> (f64, f64) {
    let lon0 = utm_central_meridian_deg(coord.zone).to_radians();
    let x = coord.easting - UTM_FALSE_EASTING;
    let y = if coord.northern { coord.northing } else { coord.northing - UTM_FALSE_NORTHING_SOUTH };
    tm_inverse_raw(x, y, lon0)
}
// #endregion 🔖️Geodesy

// #region 🔖️Gcp
/// 📍️ A ground control point: a known world/geodetic-or-local survey position plus the pixel
/// observations (indexed by position into the caller's camera list) that pin it into the SfM frame.
#[derive(Clone, Debug, PartialEq)]
pub struct GroundControlPoint {
    pub id: String,
    pub world_position_enu_or_local: [f64; 3],
    /// 📸️ `(camera_index, pixel)` pairs, `camera_index` indexing directly into the `cameras` slice
    /// passed to [`georeference`]/[`refine_gcp_scene_points`].
    pub observations: Vec<(usize, [f64; 2])>,
}

const GCP_REFINE_OUTER_ITERS: usize = 3;
const GCP_REFINE_INNER_ITERS: usize = 6;

/// 🧷️ Refines each GCP's scene-frame 3D position and the scene→world [`Sim3`] jointly: each outer pass
/// (a) per-GCP, runs a small Gauss-Newton (3 unknowns) stacking multi-view reprojection residuals
/// (Jacobians from [`remodel_camera::reprojection_jacobians`]) with the GCP world-position prior residual
/// ([`remodel_sfm::apply_gcp_prior_residual`] — the `PosePrior::Gcp` primitive, delegated to wholesale
/// rather than reimplemented), pulling the point toward the current [`Sim3`] estimate's local-frame
/// image of the known world position; then (b) re-solves the closed-form [`umeyama`] similarity between
/// all refined points and their known world positions. No multi-camera bundle adjustment is
/// reimplemented here — the only per-observation math is the single-point reprojection residual/Jacobian
/// `remodel_camera` already exposes, and the prior residual `remodel_sfm` already exposes.
pub fn refine_gcp_scene_points(scene_points: &[[f64; 3]], gcps: &[GroundControlPoint], cameras: &[(CameraPose, Intrinsics)]) -> (Vec<[f64; 3]>, Sim3) {
    let world: Vec<[f64; 3]> = gcps.iter().map(|g| g.world_position_enu_or_local).collect();
    let mut refined = scene_points.to_vec();
    let mut sim3 = umeyama(&refined, &world, true).unwrap_or_else(Sim3::identity);
    for _ in 0..GCP_REFINE_OUTER_ITERS {
        let sim3_inv = sim3.inverse();
        for (i, gcp) in gcps.iter().enumerate() {
            if i >= refined.len() {
                continue;
            }
            let views: Vec<((CameraPose, Intrinsics), [f64; 2])> = gcp.observations.iter().filter_map(|&(ci, px)| cameras.get(ci).map(|&cam| (cam, px))).collect();
            if views.is_empty() {
                continue;
            }
            let target_local = sim3_inv.act(world[i]);
            let mut point = refined[i];
            for _ in 0..GCP_REFINE_INNER_ITERS {
                let rows = views.len() * 2 + 3;
                let mut j = MatD::zeros(rows, 3);
                let mut r = VecD::zeros(rows);
                for (k, ((pose, intr), px)) in views.iter().enumerate() {
                    let res = reprojection_residual(intr, pose, point, *px);
                    let (_, jpoint, _) = reprojection_jacobians(intr, pose, point, *px);
                    r.set(2 * k, res[0]);
                    r.set(2 * k + 1, res[1]);
                    for c in 0..3 {
                        j.set(2 * k, c, jpoint.get(0, c));
                        j.set(2 * k + 1, c, jpoint.get(1, c));
                    }
                }
                let (prior_r, prior_j) = apply_gcp_prior_residual(point, target_local, 1.0);
                let base = views.len() * 2;
                for c in 0..3 {
                    r.set(base + c, prior_r.get(c));
                    for cc in 0..3 {
                        j.set(base + c, cc, prior_j.get(c, cc));
                    }
                }
                let neg_r = r.scale(-1.0);
                let Ok(delta) = solve_llsq(&j, &neg_r) else { break };
                point = [point[0] + delta.get(0), point[1] + delta.get(1), point[2] + delta.get(2)];
                if delta.norm2() < 1e-12 {
                    break;
                }
            }
            refined[i] = point;
        }
        sim3 = umeyama(&refined, &world, true).unwrap_or(sim3);
    }
    (refined, sim3)
}

/// 🧷️ Registers an SfM reconstruction (arbitrary monocular gauge/scale) into a known world/geodetic or
/// local survey frame via ground control points: [`refine_gcp_scene_points`]'s converged [`Sim3`].
/// `cameras[i]` supplies the pose/intrinsics for observations tagged camera index `i`; `scene_points[i]`
/// is `gcps[i]`'s initial (e.g. triangulated) position in the SfM frame. Falls back to [`Sim3::identity`]
/// for degenerate input (fewer than three usable GCPs, or collinear/coincident positions) rather than
/// panicking — callers should sanity-check [`gcp_checkpoint_rmse`] afterward.
pub fn georeference(scene_points: &[[f64; 3]], gcps: &[GroundControlPoint], cameras: &[(CameraPose, Intrinsics)]) -> Sim3 {
    refine_gcp_scene_points(scene_points, gcps, cameras).1
}

/// 📏️ Root-mean-square checkpoint residual of a converged georeferencing fit: for each `(refined scene
/// point, gcp)` pair, [`apply_gcp_prior_residual`] scores `sim3.act(point) - gcp.world_position` and this
/// aggregates the residual norms into one RMSE (metres, or whatever unit `world_position` is in).
pub fn gcp_checkpoint_rmse(sim3: &Sim3, refined_scene_points: &[[f64; 3]], gcps: &[GroundControlPoint]) -> f64 {
    let mut sum_sq = 0.0;
    let mut n = 0usize;
    for (point, gcp) in refined_scene_points.iter().zip(gcps) {
        let transformed = sim3.act(*point);
        let (r, _jb) = apply_gcp_prior_residual(transformed, gcp.world_position_enu_or_local, 1.0);
        sum_sq += r.norm2() * r.norm2();
        n += 1;
    }
    if n == 0 {
        0.0
    } else {
        (sum_sq / n as f64).sqrt()
    }
}
// #endregion 🔖️Gcp

// #region 🔖️Raster
/// 🗺️ Regular 2D grid of `f32` cell values with a parallel `valid` mask (rather than `Option<f32>` per
/// cell, to keep `values` a dense contiguous buffer usable directly as image/heightfield data); `origin`
/// is the world `(x, y)` coordinate of cell `(0, 0)`'s lower-left corner, and elevation/DSM-style
/// consumers treat the world frame as Z-up. Shared by DSM/DTM, orthomosaic-support rasters, contours and volumes.
#[derive(Clone, Debug, PartialEq)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub cell_size: f64,
    pub origin: [f64; 2],
    pub values: Vec<f32>,
    pub valid: Vec<bool>,
}

impl Raster {
    /// 🗺️ An all-invalid raster of the given shape.
    pub fn new(width: u32, height: u32, cell_size: f64, origin: [f64; 2]) -> Self {
        let n = width as usize * height as usize;
        Self { width, height, cell_size, origin, values: vec![0.0; n], valid: vec![false; n] }
    }

    /// 🔢️ Row-major flat index of cell `(x, y)`.
    pub fn index(&self, x: u32, y: u32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    /// 🔍️ The cell's value, or `None` if out of bounds or not yet written.
    pub fn get(&self, x: u32, y: u32) -> Option<f32> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let i = self.index(x, y);
        self.valid[i].then_some(self.values[i])
    }

    /// ✏️ Writes a cell value and marks it valid.
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        let i = self.index(x, y);
        self.values[i] = value;
        self.valid[i] = true;
    }

    /// 📍️ World-space `(x, y)` of a cell's center.
    pub fn cell_center(&self, x: u32, y: u32) -> [f64; 2] {
        [self.origin[0] + (f64::from(x) + 0.5) * self.cell_size, self.origin[1] + (f64::from(y) + 0.5) * self.cell_size]
    }

    /// 🎯️ The integer cell containing a world-space point, or `None` if outside the raster's extent.
    pub fn cell_of(&self, p: [f64; 2]) -> Option<(u32, u32)> {
        let fx = (p[0] - self.origin[0]) / self.cell_size;
        let fy = (p[1] - self.origin[1]) / self.cell_size;
        if fx < 0.0 || fy < 0.0 {
            return None;
        }
        let (x, y) = (fx.floor() as u32, fy.floor() as u32);
        (x < self.width && y < self.height).then_some((x, y))
    }
}
// #endregion 🔖️Raster

// #region 🔖️Dsm
/// ⛰️ Per-cell max-elevation Digital Surface Model, binning every cloud point into the raster cell
/// containing its `(x, y)` and keeping the highest `z` seen per cell.
pub fn build_dsm(cloud: &PointCloud, cell_size: f64, origin: [f64; 2], width: u32, height: u32) -> Raster {
    let mut raster = Raster::new(width, height, cell_size, origin);
    for &p in &cloud.positions {
        if let Some((x, y)) = raster.cell_of([p[0], p[1]]) {
            let i = raster.index(x, y);
            if !raster.valid[i] || p[2] as f32 > raster.values[i] {
                raster.set(x, y, p[2] as f32);
            }
        }
    }
    raster
}

/// 🏞️ Ground-only Digital Terrain Model: the same max-elevation binning as [`build_dsm`], restricted to
/// points labeled [`PointClass::Ground`] (an unlabeled cloud is treated as entirely ground).
pub fn build_dtm(cloud: &PointCloud, cell_size: f64, origin: [f64; 2], width: u32, height: u32) -> Raster {
    let mut raster = Raster::new(width, height, cell_size, origin);
    for (idx, &p) in cloud.positions.iter().enumerate() {
        if !cloud.classification.is_empty() && cloud.classification[idx] != PointClass::Ground {
            continue;
        }
        if let Some((x, y)) = raster.cell_of([p[0], p[1]]) {
            let i = raster.index(x, y);
            if !raster.valid[i] || p[2] as f32 > raster.values[i] {
                raster.set(x, y, p[2] as f32);
            }
        }
    }
    raster
}

/// 🕳️ Fills every invalid cell via inverse-distance weighting over valid cells within `radius_cells`
/// (grid units), searched through a [`KdTree`] over valid-cell centers rather than a linear scan; cells
/// with no valid neighbour within radius stay invalid.
pub fn idw_fill(raster: &Raster, radius_cells: f64, power: f64) -> Raster {
    let mut valid_pts: Vec<[f64; 2]> = Vec::new();
    let mut valid_idx: Vec<usize> = Vec::new();
    for y in 0..raster.height {
        for x in 0..raster.width {
            let i = raster.index(x, y);
            if raster.valid[i] {
                valid_pts.push([f64::from(x), f64::from(y)]);
                valid_idx.push(i);
            }
        }
    }
    let mut out = raster.clone();
    if valid_pts.is_empty() {
        return out;
    }
    let tree = KdTree::<2>::build(&valid_pts);
    let radius_sq = radius_cells * radius_cells;
    let k = (valid_pts.len()).min(16);
    for y in 0..raster.height {
        for x in 0..raster.width {
            let i = raster.index(x, y);
            if raster.valid[i] {
                continue;
            }
            let q = [f64::from(x), f64::from(y)];
            let mut w_sum = 0.0_f64;
            let mut v_sum = 0.0_f64;
            for (id, dist_sq) in tree.k_nearest(&q, k) {
                if dist_sq > radius_sq {
                    continue;
                }
                let d = dist_sq.sqrt().max(1e-6);
                let w = 1.0 / d.powf(power);
                v_sum += w * f64::from(raster.values[valid_idx[id as usize]]);
                w_sum += w;
            }
            if w_sum > 0.0 {
                out.set(x, y, (v_sum / w_sum) as f32);
            }
        }
    }
    out
}

/// 🌄️ Per-cell density raster: number of point-cloud points binned into each cell.
pub fn density_map(cloud: &PointCloud, cell_size: f64, origin: [f64; 2], width: u32, height: u32) -> Raster {
    let mut raster = Raster::new(width, height, cell_size, origin);
    for &p in &cloud.positions {
        if let Some((x, y)) = raster.cell_of([p[0], p[1]]) {
            let i = raster.index(x, y);
            raster.values[i] += 1.0;
            raster.valid[i] = true;
        }
    }
    raster
}

/// 🔆️ Standard illumination-angle hillshade (Horn's central-difference gradient, `shade = sin(alt)
/// cos(slope) + cos(alt) sin(slope) cos(azimuth - aspect)`), values clamped to `[0, 1]`; invalid cells
/// stay invalid and neighbouring invalid samples fall back to the center cell's own value so a hole
/// doesn't tilt the local gradient estimate.
pub fn hillshade(raster: &Raster, azimuth_deg: f64, altitude_deg: f64) -> Raster {
    let az = azimuth_deg.to_radians();
    let alt = altitude_deg.to_radians();
    let mut out = Raster::new(raster.width, raster.height, raster.cell_size, raster.origin);
    for y in 0..raster.height {
        for x in 0..raster.width {
            let i = raster.index(x, y);
            if !raster.valid[i] {
                continue;
            }
            let center = raster.values[i];
            let z = |dx: i64, dy: i64| -> f64 {
                let xx = (i64::from(x) + dx).clamp(0, i64::from(raster.width) - 1) as u32;
                let yy = (i64::from(y) + dy).clamp(0, i64::from(raster.height) - 1) as u32;
                f64::from(raster.get(xx, yy).unwrap_or(center))
            };
            let dzdx = ((z(1, -1) + 2.0 * z(1, 0) + z(1, 1)) - (z(-1, -1) + 2.0 * z(-1, 0) + z(-1, 1))) / (8.0 * raster.cell_size);
            let dzdy = ((z(-1, 1) + 2.0 * z(0, 1) + z(1, 1)) - (z(-1, -1) + 2.0 * z(0, -1) + z(1, -1))) / (8.0 * raster.cell_size);
            let slope = (dzdx * dzdx + dzdy * dzdy).sqrt().atan();
            let aspect = (-dzdy).atan2(-dzdx);
            let shade = alt.sin() * slope.cos() + alt.cos() * slope.sin() * (az - aspect).cos();
            out.set(x, y, shade.clamp(0.0, 1.0) as f32);
        }
    }
    out
}
// #endregion 🔖️Dsm

// #region 🔖️Ortho
/// 🧵️ Per-cell overlap count: how many cameras' image bounds a DSM cell's height re-projects into.
pub fn overlap_map(dsm: &Raster, cameras: &[(CameraPose, Intrinsics)], image_size: (u32, u32)) -> Raster {
    let mut out = Raster::new(dsm.width, dsm.height, dsm.cell_size, dsm.origin);
    for y in 0..dsm.height {
        for x in 0..dsm.width {
            let Some(z) = dsm.get(x, y) else { continue };
            let [wx, wy] = dsm.cell_center(x, y);
            let world = [wx, wy, f64::from(z)];
            let mut count = 0.0_f32;
            for (pose, intr) in cameras {
                if let Some(px) = reproject(intr, pose, world) {
                    if px[0] >= 0.0 && px[1] >= 0.0 && px[0] < f64::from(image_size.0) && px[1] < f64::from(image_size.1) {
                        count += 1.0;
                    }
                }
            }
            out.set(x, y, count);
        }
    }
    out
}

/// 🖼️ Orthomosaic generation: for each DSM cell, samples every camera whose reprojection of that cell's
/// `(x, y, dsm_height)` lands in-bounds, weights each candidate by `feather * nadir` — `feather` is the
/// reprojected pixel's distance to the nearest image edge (so seams fade out smoothly toward frame
/// borders rather than cutting hard) and `nadir` is the cosine of the viewing ray's angle from vertical
/// (favouring the most overhead camera) — and blends the weighted colors; cells with no in-bounds camera
/// stay transparent black.
pub fn build_orthomosaic(dsm: &Raster, cameras: &[(CameraPose, Intrinsics)], images: &[ImageRgba8]) -> ImageRgba8 {
    let mut out = ImageRgba8::new(dsm.width, dsm.height);
    for y in 0..dsm.height {
        for x in 0..dsm.width {
            let Some(z) = dsm.get(x, y) else { continue };
            let [wx, wy] = dsm.cell_center(x, y);
            let world = [wx, wy, f64::from(z)];
            let mut w_rgb = [0.0_f64; 3];
            let mut w_sum = 0.0_f64;
            for (ci, (pose, intr)) in cameras.iter().enumerate() {
                let Some(img) = images.get(ci) else { continue };
                let Some(px) = reproject(intr, pose, world) else { continue };
                if px[0] < 0.0 || px[1] < 0.0 || px[0] > f64::from(img.width) - 1.0 || px[1] > f64::from(img.height) - 1.0 {
                    continue;
                }
                let cam_center = pose.0.inverse().t;
                let dir = vec3d_normalize(vec3d_sub(cam_center, world));
                let nadir = dir[2].max(0.0);
                let edge_dist = px[0].min(f64::from(img.width) - 1.0 - px[0]).min(px[1]).min(f64::from(img.height) - 1.0 - px[1]);
                let feather = edge_dist.max(0.0) + 1.0;
                let weight = feather * (nadir + 0.05);
                if weight <= 0.0 {
                    continue;
                }
                let rgb = img.sample_rgb(px[0] as f32, px[1] as f32);
                for c in 0..3 {
                    w_rgb[c] += weight * f64::from(rgb[c]);
                }
                w_sum += weight;
            }
            if w_sum > 0.0 {
                let didx = (y as usize * out.width as usize + x as usize) * 4;
                for (c, &channel) in w_rgb.iter().enumerate() {
                    out.data[didx + c] = ((channel / w_sum) * 255.0).clamp(0.0, 255.0) as u8;
                }
                out.data[didx + 3] = 255;
            }
        }
    }
    out
}
// #endregion 🔖️Ortho

// #region 🔖️Contours
/// 🧩️ A grid-edge crossing's canonical identity, shared by the two marching-squares cells that border
/// it — keying by edge identity (not interpolated position) makes adjacent cells agree exactly, so
/// polyline chaining never needs epsilon-tolerant point matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum EdgeKey {
    /// ↔ Horizontal edge between `(x, y)` and `(x + 1, y)`.
    H(u32, u32),
    /// ↕️ Vertical edge between `(x, y)` and `(x, y + 1)`.
    V(u32, u32),
}

/// 📐️ Linear-interpolation fraction along an edge where the field crosses `level`.
fn crossing_fraction(a: f32, b: f32, level: f32) -> f64 {
    if (b - a).abs() < 1e-12 {
        0.5
    } else {
        f64::from((level - a) / (b - a))
    }
}

/// 📍️ World-space position of an edge crossing at the given iso level.
fn edge_position(raster: &Raster, level: f32, key: EdgeKey) -> [f64; 2] {
    match key {
        EdgeKey::H(x, y) => {
            let t = crossing_fraction(raster.values[raster.index(x, y)], raster.values[raster.index(x + 1, y)], level);
            [raster.origin[0] + (f64::from(x) + t) * raster.cell_size, raster.origin[1] + f64::from(y) * raster.cell_size]
        }
        EdgeKey::V(x, y) => {
            let t = crossing_fraction(raster.values[raster.index(x, y)], raster.values[raster.index(x, y + 1)], level);
            [raster.origin[0] + f64::from(x) * raster.cell_size, raster.origin[1] + (f64::from(y) + t) * raster.cell_size]
        }
    }
}

/// 🧵️ Marching-squares segments for one cell, keyed on canonical [`EdgeKey`]s; saddle cases 5/10 resolve
/// via the average of the four corners against `level` (Montani-style consistent disambiguation).
fn cell_segments(bl: f32, br: f32, tr: f32, tl: f32, level: f32, x: u32, y: u32) -> Vec<(EdgeKey, EdgeKey)> {
    let bottom = EdgeKey::H(x, y);
    let right = EdgeKey::V(x + 1, y);
    let top = EdgeKey::H(x, y + 1);
    let left = EdgeKey::V(x, y);
    let case = u8::from(bl > level) | (u8::from(br > level) << 1) | (u8::from(tr > level) << 2) | (u8::from(tl > level) << 3);
    match case {
        0 | 15 => vec![],
        1 | 14 => vec![(left, bottom)],
        2 | 13 => vec![(bottom, right)],
        3 | 12 => vec![(left, right)],
        4 | 11 => vec![(right, top)],
        6 | 9 => vec![(bottom, top)],
        7 | 8 => vec![(left, top)],
        5 => {
            if (bl + br + tr + tl) / 4.0 > level {
                vec![(left, top), (bottom, right)]
            } else {
                vec![(left, bottom), (right, top)]
            }
        }
        10 => {
            if (bl + br + tr + tl) / 4.0 > level {
                vec![(bottom, left), (top, right)]
            } else {
                vec![(bottom, right), (top, left)]
            }
        }
        _ => vec![],
    }
}

/// 🧵️ Chains a bag of `(EdgeKey, position)`-terminated segments into polylines by endpoint matching:
/// each unused edge key with a not-yet-used incident segment extends the current chain, walking forward
/// then backward from the seed segment until no unused neighbour remains (an open polyline, if the
/// level set touches the raster border) or the chain closes back on its start key (a closed loop).
fn chain_segments(segments: &[(EdgeKey, EdgeKey)], positions: &HashMap<EdgeKey, [f64; 2]>) -> Vec<Vec<[f64; 2]>> {
    let mut adjacency: HashMap<EdgeKey, Vec<usize>> = HashMap::new();
    for (idx, &(a, b)) in segments.iter().enumerate() {
        adjacency.entry(a).or_default().push(idx);
        adjacency.entry(b).or_default().push(idx);
    }
    let mut used = vec![false; segments.len()];
    let mut polylines = Vec::new();
    for start in 0..segments.len() {
        if used[start] {
            continue;
        }
        used[start] = true;
        let (k0, k1) = segments[start];
        let mut chain_keys = vec![k0, k1];

        let mut cur = k1;
        while let Some(next) = adjacency.get(&cur).and_then(|cands| cands.iter().copied().find(|&i| !used[i])) {
            used[next] = true;
            let (a, b) = segments[next];
            let other = if a == cur { b } else { a };
            chain_keys.push(other);
            cur = other;
            if cur == chain_keys[0] {
                break;
            }
        }
        let mut head = chain_keys[0];
        while let Some(next) = adjacency.get(&head).and_then(|cands| cands.iter().copied().find(|&i| !used[i])) {
            used[next] = true;
            let (a, b) = segments[next];
            let other = if a == head { b } else { a };
            chain_keys.insert(0, other);
            head = other;
        }
        polylines.push(chain_keys.iter().map(|k| positions[k]).collect());
    }
    polylines
}

/// 🗻️ Marching-squares isocontour extraction at each of `levels`, chaining segments into connected
/// polylines; returns `(level, polylines)` pairs, one per input level. Cells touching an invalid raster
/// sample are skipped (honest gaps rather than a fabricated crossing).
pub fn extract_contours(raster: &Raster, levels: &[f32]) -> Vec<(f32, Vec<Vec<[f64; 2]>>)> {
    levels
        .iter()
        .map(|&level| {
            let mut segments = Vec::new();
            let mut positions: HashMap<EdgeKey, [f64; 2]> = HashMap::new();
            if raster.width > 0 && raster.height > 0 {
                for y in 0..raster.height - 1 {
                    for x in 0..raster.width - 1 {
                        let (i00, i10, i11, i01) = (raster.index(x, y), raster.index(x + 1, y), raster.index(x + 1, y + 1), raster.index(x, y + 1));
                        if !(raster.valid[i00] && raster.valid[i10] && raster.valid[i11] && raster.valid[i01]) {
                            continue;
                        }
                        let (bl, br, tr, tl) = (raster.values[i00], raster.values[i10], raster.values[i11], raster.values[i01]);
                        for (a, b) in cell_segments(bl, br, tr, tl, level, x, y) {
                            positions.entry(a).or_insert_with(|| edge_position(raster, level, a));
                            positions.entry(b).or_insert_with(|| edge_position(raster, level, b));
                            segments.push((a, b));
                        }
                    }
                }
            }
            (level, chain_segments(&segments, &positions))
        })
        .collect()
}
// #endregion 🔖️Contours

// #region 🔖️Volume
/// 📦️ Cut/fill/net volume between two elevation states, with cut and fill always non-negative and `net = fill - cut`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VolumeReport {
    pub cut_m3: f64,
    pub fill_m3: f64,
    pub net_m3: f64,
}

/// 📦️ Cut/fill volume between two same-shaped rasters, summing `cell_area * (after - before)` over
/// cells valid in both — positive differences accumulate as fill, negative as cut.
pub fn cut_fill_volume(before: &Raster, after: &Raster) -> VolumeReport {
    assert_eq!(before.width, after.width, "cut_fill_volume: raster width mismatch");
    assert_eq!(before.height, after.height, "cut_fill_volume: raster height mismatch");
    let cell_area = before.cell_size * before.cell_size;
    let mut cut = 0.0;
    let mut fill = 0.0;
    for i in 0..before.values.len() {
        if before.valid[i] && after.valid[i] {
            let d = f64::from(after.values[i]) - f64::from(before.values[i]);
            if d > 0.0 {
                fill += d * cell_area;
            } else {
                cut += -d * cell_area;
            }
        }
    }
    VolumeReport { cut_m3: cut, fill_m3: fill, net_m3: fill - cut }
}

/// 📦️ Cut/fill volume of a raster against a flat reference plane at `plane_z`.
pub fn cut_fill_volume_vs_plane(raster: &Raster, plane_z: f64) -> VolumeReport {
    let cell_area = raster.cell_size * raster.cell_size;
    let mut cut = 0.0;
    let mut fill = 0.0;
    for i in 0..raster.values.len() {
        if raster.valid[i] {
            let d = f64::from(raster.values[i]) - plane_z;
            if d > 0.0 {
                fill += d * cell_area;
            } else {
                cut += -d * cell_area;
            }
        }
    }
    VolumeReport { cut_m3: cut, fill_m3: fill, net_m3: fill - cut }
}
// #endregion 🔖️Volume

// #region 🔖️Quality
/// 🧵️ Aggregate feature-track length statistics.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TrackStats {
    pub track_count: usize,
    pub mean_track_length: f64,
    pub min_track_length: usize,
    pub max_track_length: usize,
}

/// 📊️ Whole-reconstruction quality/QC report: reprojection accuracy, GCP checkpoint agreement, track
/// health, spatial coverage rasters, camera-pose and point-position uncertainty, and mesh watertightness.
#[derive(Clone, Debug, PartialEq)]
pub struct QualityReport {
    pub reprojection_rms_px: f64,
    pub per_camera_rms_px: Vec<f64>,
    pub gcp_checkpoint_rmse: Option<f64>,
    pub track_stats: TrackStats,
    pub density_map: Option<Raster>,
    pub overlap_map: Option<Raster>,
    /// 🧮️ Flattened row-major 6x6 marginal covariance per camera (from [`camera_covariance_diagonals`]).
    pub per_camera_covariance: Vec<[f64; 36]>,
    pub per_point_sigma: Vec<f64>,
    pub watertight: Option<WatertightReport>,
}

/// 📏️ Overall and per-camera reprojection RMS (pixels) of `observations` — `(camera_index, point_index,
/// pixel)` triples, `camera_index`/`point_index` indexing directly into `recon.cameras`/`recon.points` —
/// against `recon`'s current camera poses and triangulated points.
pub fn reprojection_stats(recon: &Reconstruction, observations: &[(usize, usize, [f64; 2])]) -> (f64, Vec<f64>) {
    let mut per_camera_sq = vec![0.0_f64; recon.cameras.len()];
    let mut per_camera_n = vec![0usize; recon.cameras.len()];
    let mut total_sq = 0.0;
    let mut total_n = 0usize;
    for &(ci, pi, px) in observations {
        if ci >= recon.cameras.len() || pi >= recon.points.len() {
            continue;
        }
        let (_, pose) = recon.cameras[ci];
        let res = reprojection_residual(&recon.intrinsics, &pose, recon.points[pi], px);
        let sq = res[0] * res[0] + res[1] * res[1];
        per_camera_sq[ci] += sq;
        per_camera_n[ci] += 1;
        total_sq += sq;
        total_n += 1;
    }
    let overall = if total_n > 0 { (total_sq / total_n as f64).sqrt() } else { 0.0 };
    let per_camera = per_camera_sq.iter().zip(&per_camera_n).map(|(&sq, &n)| if n > 0 { (sq / n as f64).sqrt() } else { 0.0 }).collect();
    (overall, per_camera)
}

/// 🧵️ [`TrackStats`] derived from `observations` by grouping observation counts per `point_index`.
pub fn track_stats_from_observations(observations: &[(usize, usize, [f64; 2])], num_points: usize) -> TrackStats {
    let mut counts = vec![0usize; num_points];
    for &(_, pi, _) in observations {
        if pi < num_points {
            counts[pi] += 1;
        }
    }
    let observed: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
    if observed.is_empty() {
        return TrackStats::default();
    }
    let total: usize = observed.iter().sum();
    TrackStats { track_count: observed.len(), mean_track_length: total as f64 / observed.len() as f64, min_track_length: *observed.iter().min().unwrap_or(&0), max_track_length: *observed.iter().max().unwrap_or(&0) }
}

/// 🧮️ Per-camera marginal 6x6 covariance (flattened row-major), estimated by constructing a
/// [`SfmBundleProblem`] from `recon`+`observations` and running [`math::optimize::schur_lm`] for a
/// single (already-near-optimal) iteration — the covariance-diagonal machinery `schur_lm`'s Schur
/// complement always computes, just reused at the current solution rather than a fresh optimization.
/// Empty when there are no cameras, points or observations to build a problem from.
pub fn camera_covariance_diagonals(recon: &Reconstruction, observations: &[(usize, usize, [f64; 2])]) -> Vec<[f64; 36]> {
    if recon.cameras.is_empty() || recon.points.is_empty() || observations.is_empty() {
        return Vec::new();
    }
    let mut terms = Vec::new();
    let mut obs_map = HashMap::new();
    for &(ci, pi, px) in observations {
        if ci >= recon.cameras.len() || pi >= recon.points.len() {
            continue;
        }
        terms.push(ResidualTerm { a_index: Some(ci), b_index: Some(pi), dim: 2 });
        obs_map.insert((ci, pi), px);
    }
    if terms.is_empty() {
        return Vec::new();
    }
    let problem = SfmBundleProblem { intrinsics: recon.intrinsics, num_cameras: recon.cameras.len(), num_points: recon.points.len(), terms, observations: obs_map };
    let a0: Vec<VecD> = recon.cameras.iter().map(|&(_, pose)| VecD::from_vec(pose.0.log().to_vec())).collect();
    let b0: Vec<VecD> = recon.points.iter().map(|&p| VecD::from_vec(p.to_vec())).collect();
    let cfg = LmConfig { max_iters: 1, ..LmConfig::default() };
    let result: SchurResult = math::optimize::schur_lm(&problem, a0, b0, &cfg);
    camera_covariances(&result)
        .iter()
        .map(|m| {
            let mut flat = [0.0; 36];
            for r in 0..m.rows.min(6) {
                for c in 0..m.cols.min(6) {
                    flat[r * 6 + c] = m.get(r, c);
                }
            }
            flat
        })
        .collect()
}

/// 🧮️ Per-point isotropic position-uncertainty proxy: accumulates the fixed-camera reprojection normal
/// equations `sum(JbᵀJb)` for every observation of that point (Jacobians from
/// [`remodel_camera::reprojection_jacobians`]), inverts via [`pseudo_inverse`], and reports
/// `sqrt(trace(cov) / 3)`. `f64::INFINITY` marks an unobserved-or-singular (e.g. single-observation) point.
pub fn estimate_per_point_sigma(recon: &Reconstruction, observations: &[(usize, usize, [f64; 2])]) -> Vec<f64> {
    let mut h_blocks: Vec<MatD> = (0..recon.points.len()).map(|_| MatD::zeros(3, 3)).collect();
    for &(ci, pi, px) in observations {
        if ci >= recon.cameras.len() || pi >= recon.points.len() {
            continue;
        }
        let (_, pose) = recon.cameras[ci];
        let (_, jpoint, _) = reprojection_jacobians(&recon.intrinsics, &pose, recon.points[pi], px);
        let gram = jpoint.transpose().matmul(&jpoint);
        for r in 0..3 {
            for c in 0..3 {
                h_blocks[pi].add_at(r, c, gram.get(r, c));
            }
        }
    }
    h_blocks
        .iter()
        .map(|h| match pseudo_inverse(h, 1e-9) {
            Ok(cov) => ((0..3).map(|k| cov.get(k, k)).sum::<f64>() / 3.0).max(0.0).sqrt(),
            Err(_) => f64::INFINITY,
        })
        .collect()
}

/// 📊️ Builds a full [`QualityReport`] from a reconstruction and its observations, computing
/// [`reprojection_stats`], [`track_stats_from_observations`], [`camera_covariance_diagonals`] and
/// [`estimate_per_point_sigma`] internally; the georeferencing, coverage-raster and watertight-mesh
/// fields are supplied by the caller (they depend on inputs — GCPs, cameras+images, a closed mesh — this
/// function doesn't otherwise need).
pub fn build_quality_report(recon: &Reconstruction, observations: &[(usize, usize, [f64; 2])], gcp_checkpoint_rmse: Option<f64>, density_map: Option<Raster>, overlap_map: Option<Raster>, watertight: Option<WatertightReport>) -> QualityReport {
    let (reprojection_rms_px, per_camera_rms_px) = reprojection_stats(recon, observations);
    QualityReport {
        reprojection_rms_px,
        per_camera_rms_px,
        gcp_checkpoint_rmse,
        track_stats: track_stats_from_observations(observations, recon.points.len()),
        density_map,
        overlap_map,
        per_camera_covariance: camera_covariance_diagonals(recon, observations),
        per_point_sigma: estimate_per_point_sigma(recon, observations),
        watertight,
    }
}
// #endregion 🔖️Quality

#[cfg(test)]
mod tests {
    use super::*;
    use math::lie::So3;
    use geometry::random::{normal, Rng};

    // #region 🔖️GeodesyTests
    #[test]
    fn ecef_enu_round_trip_sub_millimeter() {
        let cases =
            [(0.0_f64, 0.0_f64, 100.0_f64), (89.9_f64.to_radians(), 45.0_f64.to_radians(), 10.0), (-33.9_f64.to_radians(), 151.2_f64.to_radians(), 50.0), (47.3769_f64.to_radians(), 8.5417_f64.to_radians(), 500.0), (-90.0_f64.to_radians(), 0.0, 0.0)];
        for &(lat, lon, h) in &cases {
            let ecef = geodetic_to_ecef(lat, lon, h);
            let (lat2, lon2, h2) = ecef_to_geodetic(ecef);
            let ecef2 = geodetic_to_ecef(lat2, lon2, h2);
            let err = ((ecef[0] - ecef2[0]).powi(2) + (ecef[1] - ecef2[1]).powi(2) + (ecef[2] - ecef2[2]).powi(2)).sqrt();
            assert!(err < 1e-3, "ecef round trip err {err} at lat={lat} lon={lon}");

            let ref_lat = 47.0_f64.to_radians();
            let ref_lon = 8.0_f64.to_radians();
            let enu = ecef_to_enu(ecef, ref_lat, ref_lon, 0.0);
            let back = enu_to_ecef(enu, ref_lat, ref_lon, 0.0);
            let enu_err = ((ecef[0] - back[0]).powi(2) + (ecef[1] - back[1]).powi(2) + (ecef[2] - back[2]).powi(2)).sqrt();
            assert!(enu_err < 1e-3, "enu round trip err {enu_err} at lat={lat} lon={lon}");
        }
    }

    #[test]
    fn utm_round_trip_sub_millimeter() {
        let cases = [(47.3769_f64, 8.5417_f64), (0.0001_f64, 5.9999_f64), (0.0001_f64, 6.0001_f64), (-33.9_f64, 151.2_f64), (60.0_f64, -1.0_f64), (10.0_f64, 100.0_f64)];
        for &(lat_deg, lon_deg) in &cases {
            let lat = lat_deg.to_radians();
            let lon = lon_deg.to_radians();
            let coord = geodetic_to_utm(lat, lon);
            let (lat2, lon2) = utm_to_geodetic(&coord);
            let back = geodetic_to_ecef(lat2, lon2, 0.0);
            let fwd = geodetic_to_ecef(lat, lon, 0.0);
            let err = ((back[0] - fwd[0]).powi(2) + (back[1] - fwd[1]).powi(2) + (back[2] - fwd[2]).powi(2)).sqrt();
            assert!(err < 1e-3, "utm round trip err {err} at lat={lat_deg} lon={lon_deg}");
        }
    }
    // #endregion 🔖️GeodesyTests

    // #region 🔖️GcpTests
    #[test]
    fn georeference_recovers_planted_similarity_under_noise() {
        let mut rng = Rng::from_seed(9001);
        let scene = remodel_sfm::synthetic_scene(9001, 6, 12, false);
        let scene_obs = remodel_sfm::project_observations(&scene, 0.0, 0.0, 9001);
        let cameras: Vec<(CameraPose, Intrinsics)> = scene.cameras.iter().map(|&(intr, pose)| (pose, intr)).collect();
        let points = scene.points_world;
        let mut by_point: Vec<Vec<(usize, [f64; 2])>> = vec![Vec::new(); points.len()];
        for o in &scene_obs {
            by_point[o.point_index].push((o.camera_index, o.pixel));
        }
        let truth = Sim3 { s: 2.3, r: So3::exp([0.1, -0.2, 0.05]), t: [5.0, -3.0, 1.5] };
        let world_points: Vec<[f64; 3]> = points.iter().map(|&p| truth.act(p)).collect();
        let pos_noise_std = 0.02;
        let noisy_scene_points: Vec<[f64; 3]> = points.iter().map(|&p| [p[0] + normal(&mut rng, 0.0, pos_noise_std), p[1] + normal(&mut rng, 0.0, pos_noise_std), p[2] + normal(&mut rng, 0.0, pos_noise_std)]).collect();
        let gcps: Vec<GroundControlPoint> = (0..points.len()).map(|i| GroundControlPoint { id: format!("gcp{i}"), world_position_enu_or_local: world_points[i], observations: by_point[i].clone() }).collect();

        let (refined, sim3) = refine_gcp_scene_points(&noisy_scene_points, &gcps, &cameras);
        assert!((sim3.s - truth.s).abs() < 0.1, "scale err {} vs truth {}", sim3.s, truth.s);
        let t_err = ((sim3.t[0] - truth.t[0]).powi(2) + (sim3.t[1] - truth.t[1]).powi(2) + (sim3.t[2] - truth.t[2]).powi(2)).sqrt();
        assert!(t_err < 0.1, "translation err {t_err}");

        let rmse = gcp_checkpoint_rmse(&sim3, &refined, &gcps);
        assert!(rmse < 2.0 * pos_noise_std, "checkpoint rmse {rmse} exceeds 2x noise std {}", 2.0 * pos_noise_std);
    }
    // #endregion 🔖️GcpTests

    // #region 🔖️DsmTests
    fn bump(x: f64, y: f64) -> f64 {
        10.0 + 5.0 * (-(x * x + y * y) / 8.0).exp()
    }

    #[test]
    fn dsm_dtm_match_known_terrain_and_idw_fills_holes() {
        let mut rng = Rng::from_seed(77);
        let mut positions = Vec::new();
        let mut labels = Vec::new();
        for _ in 0..4000 {
            let x = (rng.next_f64() - 0.5) * 20.0;
            let y = (rng.next_f64() - 0.5) * 20.0;
            let ground_z = bump(x, y);
            positions.push([x, y, ground_z]);
            labels.push(PointClass::Ground);
            if rng.next_bool(0.3) {
                positions.push([x, y, ground_z + 2.0 + rng.next_f64()]);
                labels.push(PointClass::Vegetation);
            }
        }
        let cloud = PointCloud { positions, classification: labels, ..PointCloud::default() };
        let cell = 1.0;
        let origin = [-10.0, -10.0];
        let dtm = build_dtm(&cloud, cell, origin, 20, 20);
        let mut checked = 0;
        for y in 2..18 {
            for x in 2..18 {
                if let Some(z) = dtm.get(x, y) {
                    let [wx, wy] = dtm.cell_center(x, y);
                    let expected = bump(wx, wy);
                    assert!((f64::from(z) - expected).abs() < 1.5, "dtm cell ({x},{y}) = {z} vs expected {expected}");
                    checked += 1;
                }
            }
        }
        assert!(checked > 100, "expected many populated dtm cells, got {checked}");

        let mut holey = dtm;
        let hole_idx = holey.index(10, 10);
        let true_val = holey.values[hole_idx];
        holey.valid[hole_idx] = false;
        let filled = idw_fill(&holey, 5.0, 2.0);
        let filled_val = filled.get(10, 10).expect("idw should fill the deliberately emptied cell");
        assert!((f64::from(filled_val) - f64::from(true_val)).abs() < 1.5, "idw fill {filled_val} vs true {true_val}");
    }
    // #endregion 🔖️DsmTests

    // #region 🔖️OrthoTests
    #[test]
    fn orthomosaic_blends_smoothly_across_camera_overlap() {
        let dsm = Raster { width: 40, height: 4, cell_size: 0.5, origin: [0.0, 0.0], values: vec![0.0; 160], valid: vec![true; 160] };
        let intr = Intrinsics { fx: 200.0, fy: 200.0, cx: 100.0, cy: 100.0, skew: 0.0, distortion: remodel_camera::Distortion::None };
        // `p_cam = p_world + t` (identity rotation): a raster point at world `(wx, wy, 0)` needs
        // `t` chosen so `p_cam.z = t.z > 0` (in front of the camera) and `(wx, wy) + (t.x, t.y) = (0,
        // 0)` when the camera is centered over `(cx_world, cy_world)` — i.e. `t = (-cx_world,
        // -cy_world, height)`. Camera A centers over world x=5, camera B over world x=15, giving a
        // ~10m-wide overlap band in the middle of the 20m-wide raster.
        let pose_a = CameraPose(math::lie::Se3 { r: So3::identity(), t: [-5.0, -1.0, 20.0] });
        let pose_b = CameraPose(math::lie::Se3 { r: So3::identity(), t: [-15.0, -1.0, 20.0] });
        let mut img_a = ImageRgba8::new(200, 200);
        let mut img_b = ImageRgba8::new(200, 200);
        for px in img_a.data.as_chunks_mut::<4>().0.iter_mut() {
            *px = [220, 20, 20, 255];
        }
        for px in img_b.data.as_chunks_mut::<4>().0.iter_mut() {
            *px = [20, 20, 220, 255];
        }
        let cameras = vec![(pose_a, intr), (pose_b, intr)];
        let images = vec![img_a, img_b];
        let ortho = build_orthomosaic(&dsm, &cameras, &images);

        let mut max_step = 0_i32;
        let mut prev_r: Option<i32> = None;
        let mut saw_mid_tone = false;
        for x in 0..dsm.width {
            let idx = (2 * ortho.width + x) as usize * 4;
            let r = i32::from(ortho.data[idx]);
            if r > 60 && r < 190 {
                saw_mid_tone = true;
            }
            if let Some(pr) = prev_r {
                max_step = max_step.max((r - pr).abs());
            }
            prev_r = Some(r);
        }
        assert!(saw_mid_tone, "expected an intermediate blended tone across the overlap band");
        assert!(max_step < 150, "single-step color jump {max_step} too large for a feathered blend");
    }
    // #endregion 🔖️OrthoTests

    // #region 🔖️ContoursTests
    #[test]
    fn circular_feature_extracts_closed_contour_with_expected_perimeter() {
        let n = 61_u32;
        let cell = 0.1;
        let origin = [-3.0, -3.0];
        let mut raster = Raster::new(n, n, cell, origin);
        let radius = 2.0_f64;
        for y in 0..n {
            for x in 0..n {
                let [wx, wy] = raster.cell_center(x, y);
                let z = 10.0 - (wx * wx + wy * wy).sqrt();
                raster.set(x, y, z as f32);
            }
        }
        let level = 10.0 - radius as f32;
        let contours = extract_contours(&raster, &[level]);
        let (_, polylines) = &contours[0];
        assert!(!polylines.is_empty(), "expected at least one contour polyline");
        let closed: Vec<&Vec<[f64; 2]>> = polylines.iter().filter(|p| p.len() > 3 && dist(p[0], p[p.len() - 1]) < 2.0 * cell).collect();
        assert!(!closed.is_empty(), "expected a closed loop among the extracted polylines");
        let loop_line = closed.iter().max_by_key(|p| p.len()).unwrap();
        let perimeter: f64 = loop_line.windows(2).map(|w| dist(w[0], w[1])).sum();
        let expected_perimeter = 2.0 * std::f64::consts::PI * radius;
        assert!((perimeter - expected_perimeter).abs() / expected_perimeter < 0.05, "perimeter {perimeter} vs expected {expected_perimeter}");
    }

    fn dist(a: [f64; 2], b: [f64; 2]) -> f64 {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
    }
    // #endregion 🔖️ContoursTests

    // #region 🔖️VolumeTests
    #[test]
    fn cut_fill_matches_planted_block_volume() {
        let n = 20_u32;
        let cell = 1.0;
        let mut before = Raster::new(n, n, cell, [0.0, 0.0]);
        let mut after = Raster::new(n, n, cell, [0.0, 0.0]);
        for y in 0..n {
            for x in 0..n {
                before.set(x, y, 0.0);
                let raised = (5..15).contains(&x) && (5..15).contains(&y);
                after.set(x, y, if raised { 2.0 } else { 0.0 });
            }
        }
        let report = cut_fill_volume(&before, &after);
        let expected = 10.0 * 10.0 * 2.0 * cell * cell;
        assert!((report.fill_m3 - expected).abs() / expected < 0.005, "fill {} vs expected {}", report.fill_m3, expected);
        assert!(report.cut_m3.abs() < 1e-9, "expected zero cut, got {}", report.cut_m3);
        assert!((report.net_m3 - expected).abs() / expected < 0.005);

        let vs_plane = cut_fill_volume_vs_plane(&after, 0.0);
        assert!((vs_plane.fill_m3 - expected).abs() / expected < 0.005);
    }
    // #endregion 🔖️VolumeTests

    // #region 🔖️QualityTests
    #[test]
    fn quality_report_populates_sane_fields_from_real_sfm_output() {
        let scene = remodel_sfm::synthetic_scene(4242, 5, 40, false);
        let scene_obs = remodel_sfm::project_observations(&scene, 0.3, 0.0, 4242);
        let points = scene.points_world;
        let recon_cameras: Vec<(usize, CameraPose)> = scene.cameras.iter().enumerate().map(|(i, &(_, pose))| (i, pose)).collect();
        let intrinsics = scene.cameras[0].0;
        let recon = Reconstruction { cameras: recon_cameras, points: points.clone(), point_track_ids: (0..points.len()).collect(), intrinsics };

        let observations: Vec<(usize, usize, [f64; 2])> = scene_obs.iter().map(|o| (o.camera_index, o.point_index, o.pixel)).collect();
        assert!(observations.len() > points.len(), "expected multi-view observations in the fixture");

        let report = build_quality_report(&recon, &observations, None, None, None, None);
        assert!(report.reprojection_rms_px.is_finite() && report.reprojection_rms_px < 5.0, "rms {}", report.reprojection_rms_px);
        assert_eq!(report.per_camera_rms_px.len(), scene.cameras.len());
        assert!(report.per_camera_rms_px.iter().all(|r| r.is_finite()));
        assert!(report.track_stats.track_count > 0);
        assert!(report.track_stats.mean_track_length >= 1.0);
        assert!(!report.per_camera_covariance.is_empty(), "expected non-empty camera covariance diagonals");
        assert_eq!(report.per_point_sigma.len(), points.len());
        let finite_sigmas = report.per_point_sigma.iter().filter(|s| s.is_finite()).count();
        assert!(finite_sigmas > 0, "expected at least some well-observed points with finite sigma");
    }
    // #endregion 🔖️QualityTests
}
