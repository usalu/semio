//! 👁️ Previews and products of the reconstruction engine: the scene preview, the terminal sparse, quality and geo chunks and the geo products. See [`super`].

use super::*;

// #region 🔖️Preview
/// 🔭️ A lightweight incremental-progress snapshot for downstream UI rendering, callable mid-reconstruction
/// (not just once [`EngineStatus::Done`]): every currently-known camera pose, and every currently
/// triangulated point packed as a flat `[x0, y0, z0, x1, y1, z1, ..]` `f32` buffer.
#[derive(Clone, Debug, PartialEq)]
pub struct ScenePreview {
    pub camera_poses: Vec<remodeling_camera::CameraPose>,
    pub packed_points: Vec<f32>,
}

/// 🧱️ One bounded terminal sparse-output slice with explicit next cursors.
#[derive(Clone, Debug, PartialEq)]
pub struct TerminalSparseChunk {
    pub camera_poses: Vec<remodeling_camera::CameraPose>,
    pub packed_points: Vec<f32>,
    pub next_camera: usize,
    pub next_point: usize,
    pub complete: bool,
}

/// 🧱️ One bounded terminal QC observation slice.
#[derive(Clone, Debug, PartialEq)]
pub struct TerminalQualityChunk {
    pub squared_error_sum: f64,
    pub observation_count: usize,
    pub point_indices: Vec<usize>,
    pub next_observation: usize,
    pub complete: bool,
}

/// 🗺️ Incremental terminal geo-product state; each advance consumes a capped point slice.
#[derive(semio_framework_value::RetireOwned)]
pub struct TerminalGeoPreparation {
    pub cursor: usize,
    bounds_complete: bool,
    allocation_complete: bool,
    pub(super) bounds_min: [f64; 3],
    pub(super) bounds_max: [f64; 3],
    dsm: Option<remodeling_geo::Raster>,
    dtm: Option<remodeling_geo::Raster>,
}

pub(super) fn bounded_terminal_end(cursor: usize, len: usize, budget: usize) -> usize {
    cursor.min(len).saturating_add(budget).min(len)
}

impl ReconstructionEngine {
    /// 🔭️ Snapshots whichever reconstruction state is currently available: the finalized
    /// `Reconstruction` once bundle adjustment has run, else the in-progress `IncrementalSfm`'s own
    /// snapshot, else empty (before `EstimatingPoses` has produced anything).
    #[cfg(test)]
    pub fn sparse_preview(&self) -> ScenePreview {
        if let Some(r) = &self.reconstruction {
            return pack_reconstruction(r);
        }
        if let Some(sfm) = &self.sfm {
            let camera_poses = sfm.camera_pose_prefix(64);
            let packed_points = sfm.point_prefix(512).into_iter().flat_map(|point| point.map(|coordinate| coordinate as f32)).collect();
            return ScenePreview { camera_poses, packed_points };
        }
        ScenePreview { camera_poses: Vec::new(), packed_points: Vec::new() }
    }

    /// 🪟️ Caps incremental UI publication without changing the full terminal product snapshot.
    pub fn sparse_preview_bounded(&self, max_cameras: usize, max_points: usize) -> ScenePreview {
        if let Some(reconstruction) = &self.reconstruction {
            return pack_reconstruction_bounded(reconstruction, max_cameras, max_points);
        }
        if let Some(sfm) = &self.sfm {
            let camera_poses = sfm.camera_pose_prefix(max_cameras);
            let packed_points = sfm.point_prefix(max_points).into_iter().flat_map(|point| point.map(|coordinate| coordinate as f32)).collect();
            return ScenePreview { camera_poses, packed_points };
        }
        ScenePreview { camera_poses: Vec::new(), packed_points: Vec::new() }
    }

    /// ⏱️ Copies at most the requested camera and point counts from finished reconstruction state.
    pub fn terminal_sparse_chunk(&self, camera_cursor: usize, point_cursor: usize, max_cameras: usize, max_points: usize) -> TerminalSparseChunk {
        let Some(reconstruction) = &self.reconstruction else {
            return TerminalSparseChunk { camera_poses: Vec::new(), packed_points: Vec::new(), next_camera: camera_cursor, next_point: point_cursor, complete: true };
        };
        let camera_start = camera_cursor.min(reconstruction.cameras.len());
        let camera_end = bounded_terminal_end(camera_start, reconstruction.cameras.len(), max_cameras);
        let point_start = point_cursor.min(reconstruction.points.len());
        let point_end = bounded_terminal_end(point_start, reconstruction.points.len(), max_points);
        let camera_poses = reconstruction.cameras[camera_start..camera_end].iter().map(|&(_, pose)| pose).collect();
        let packed_points = reconstruction.points[point_start..point_end].iter().flat_map(|point| point.iter().map(|&coordinate| coordinate as f32)).collect();
        TerminalSparseChunk { camera_poses, packed_points, next_camera: camera_end, next_point: point_end, complete: camera_end == reconstruction.cameras.len() && point_end == reconstruction.points.len() }
    }

    /// ⏱️ Reduces at most `max_observations` terminal reprojection rows for incremental QC.
    pub fn terminal_quality_chunk(&self, observation_cursor: usize, max_observations: usize) -> TerminalQualityChunk {
        let Some(reconstruction) = &self.reconstruction else {
            return TerminalQualityChunk { squared_error_sum: 0.0, observation_count: 0, point_indices: Vec::new(), next_observation: observation_cursor, complete: true };
        };
        let start = observation_cursor.min(self.observations.len());
        let end = bounded_terminal_end(start, self.observations.len(), max_observations);
        let mut squared_error_sum = 0.0;
        let mut observation_count = 0;
        let mut point_indices = Vec::with_capacity(end - start);
        for &(camera_index, point_index, pixel) in &self.observations[start..end] {
            if camera_index >= reconstruction.cameras.len() || point_index >= reconstruction.points.len() {
                continue;
            }
            let (_, pose) = reconstruction.cameras[camera_index];
            let residual = remodeling_camera::reprojection_residual(&reconstruction.intrinsics, &pose, reconstruction.points[point_index], pixel);
            squared_error_sum += residual[0] * residual[0] + residual[1] * residual[1];
            observation_count += 1;
            point_indices.push(point_index);
        }
        TerminalQualityChunk { squared_error_sum, observation_count, point_indices, next_observation: end, complete: end == self.observations.len() }
    }

    /// 🔬️ Which bounded unit the engine is on, for the worker-law diagnostics: the stage plus the
    /// cursors of whichever preparation is active.
    pub fn unit_label(&self) -> String {
        match self.stage {
            EngineStage::EstimatingPoses => format!("poses cursor={} bundle={} registration={} seed={}", self.pose_cursor, self.bundle_iterations.is_some(), self.registration_preparation.is_some(), self.seed_pair_preparation.is_some()),
            EngineStage::BundleAdjusting => format!("bundle substep={} adjustment={} cleanup={} finalization={}", self.ba_substep, self.bundle_iterations.is_some(), self.bundle_preparation.is_some(), self.finalization_preparation.as_ref().map_or("-".to_string(), |preparation| format!("{:?}", preparation.phase))),
            stage => format!("{stage:?} cursor={}", self.stage_cursor),
        }
    }

    /// 📊️ Returns the already-computed mesh watertight report without rebuilding full QC.
    pub fn terminal_watertight_report(&self) -> Option<semio_s_artifact_remodel_remodeling_mesh_repair::editor::remodeling::engine::mesh::repair::WatertightReport> {
        self.watertight_report.clone()
    }

    /// 🌍️ Creates incremental geo state only when geo output and a dense cloud exist.
    pub fn begin_terminal_geo(&self) -> Option<TerminalGeoPreparation> {
        if !self.params.geo_enabled || self.dense_cloud.as_ref().is_none_or(|cloud| cloud.positions.is_empty()) {
            return None;
        }
        Some(TerminalGeoPreparation { cursor: 0, bounds_complete: false, allocation_complete: false, bounds_min: [f64::INFINITY; 3], bounds_max: [f64::NEG_INFINITY; 3], dsm: None, dtm: None })
    }

    /// ⏱️ Advances terminal bounds or DSM/DTM binning by at most `point_budget` cloud points.
    pub fn advance_terminal_geo(&self, preparation: &mut TerminalGeoPreparation, point_budget: usize) -> bool {
        let Some(cloud) = &self.dense_cloud else { return true };
        let end = bounded_terminal_end(preparation.cursor, cloud.positions.len(), point_budget);
        if !preparation.bounds_complete {
            for &point in &cloud.positions[preparation.cursor..end] {
                for (axis, coordinate) in point.iter().copied().enumerate() {
                    preparation.bounds_min[axis] = preparation.bounds_min[axis].min(coordinate);
                    preparation.bounds_max[axis] = preparation.bounds_max[axis].max(coordinate);
                }
            }
            preparation.cursor = end;
            if end == cloud.positions.len() {
                let cell = self.params.geo_cell_size.max(1e-6);
                let width = (((preparation.bounds_max[0] - preparation.bounds_min[0]) / cell).ceil() as u32 + 1).clamp(1, 512);
                let height = (((preparation.bounds_max[1] - preparation.bounds_min[1]) / cell).ceil() as u32 + 1).clamp(1, 512);
                let origin = [preparation.bounds_min[0], preparation.bounds_min[1]];
                preparation.dsm = Some(remodeling_geo::Raster::empty(width, height, cell, origin));
                preparation.dtm = Some(remodeling_geo::Raster::empty(width, height, cell, origin));
                preparation.bounds_complete = true;
                preparation.cursor = 0;
            }
            return false;
        }
        if !preparation.allocation_complete {
            let dsm_complete = preparation.dsm.as_mut().expect("terminal DSM initialized").extend_invalid(point_budget);
            let dtm_complete = preparation.dtm.as_mut().expect("terminal DTM initialized").extend_invalid(point_budget);
            preparation.allocation_complete = dsm_complete && dtm_complete;
            return false;
        }
        let dsm = preparation.dsm.as_mut().expect("terminal DSM initialized");
        let dtm = preparation.dtm.as_mut().expect("terminal DTM initialized");
        for index in preparation.cursor..end {
            let point = cloud.positions[index];
            if let Some((x, y)) = dsm.cell_of([point[0], point[1]]) {
                let cell = dsm.index(x, y);
                if !dsm.valid[cell] || point[2] as f32 > dsm.values[cell] {
                    dsm.set(x, y, point[2] as f32);
                }
                if (cloud.classification.is_empty() || cloud.classification[index] == remodeling_dense::PointClass::Ground) && (!dtm.valid[cell] || point[2] as f32 > dtm.values[cell]) {
                    dtm.set(x, y, point[2] as f32);
                }
            }
        }
        preparation.cursor = end;
        end == cloud.positions.len()
    }

    /// 🌍️ Releases fully binned terminal rasters.
    pub fn finish_terminal_geo(preparation: TerminalGeoPreparation) -> Option<GeoProducts> {
        Some(GeoProducts { dsm: preparation.dsm?, dtm: preparation.dtm? })
    }
}

/// 📦️ Packs a `Reconstruction`'s camera poses and points into a [`ScenePreview`].
#[cfg(test)]
fn pack_reconstruction(r: &semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::Reconstruction) -> ScenePreview {
    ScenePreview { camera_poses: r.cameras.iter().map(|&(_, p)| p).collect(), packed_points: r.points.iter().flat_map(|p| p.iter().map(|&c| c as f32)).collect() }
}

/// 📦️ Packs finite camera and point prefixes for per-continuation previews.
fn pack_reconstruction_bounded(r: &semio_s_artifact_remodel_remodeling_sfm::editor::remodeling::engine::sfm::Reconstruction, max_cameras: usize, max_points: usize) -> ScenePreview {
    ScenePreview { camera_poses: r.cameras.iter().take(max_cameras).map(|&(_, pose)| pose).collect(), packed_points: r.points.iter().take(max_points).flat_map(|point| point.iter().map(|&coordinate| coordinate as f32)).collect() }
}
// #endregion 🔖️Preview

// #region 🔖️Products
/// 🌍️ Optional georeferencing-adjacent rasters, populated only when `EngineParams::geo_enabled` and
/// there's a fused dense point cloud to derive them from.
#[derive(Clone, Debug, PartialEq, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GeoProducts {
    pub dsm: remodeling_geo::Raster,
    pub dtm: remodeling_geo::Raster,
}

/// 📦️ World-space `(x, y)`/`(z)` bounding box of a point cloud's positions, or `None` when empty.
#[cfg(test)]
fn point_cloud_bbox(cloud: &remodeling_dense::PointCloud) -> Option<([f64; 3], [f64; 3])> {
    let mut it = cloud.positions.iter();
    let first = *it.next()?;
    let mut lo = first;
    let mut hi = first;
    for &p in it {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    Some((lo, hi))
}

impl ReconstructionEngine {
    /// 🕸️ The finished, watertight-guaranteed `MeshData`, once [`EngineStatus::Done`] — consumes it (a
    /// second call returns `None`), mirroring `crate::editor::remodeling::engine::mesh::MeshPipeline::result`'s own take-once shape
    /// at the product-extraction boundary.
    pub fn take_mesh(&mut self) -> Option<semio_framework::MeshData> {
        self.mesh_data.take()
    }

    /// 📊️ The whole-reconstruction quality report: reprojection accuracy, track health, camera/point
    /// uncertainty, and (once the mesh pipeline has run) the watertight report — available as soon as
    /// bundle adjustment has produced a `Reconstruction`, not only at `Done`, since it's cheap to
    /// recompute from already-finished data.
    #[cfg(test)]
    pub fn take_quality(&mut self) -> Option<remodeling_geo::QualityReport> {
        let recon = self.reconstruction.as_ref()?;
        Some(remodeling_geo::build_quality_report(recon, &self.observations, None, None, None, self.watertight_report.clone()))
    }

    /// 🌍️ DSM/DTM rasters derived from the fused dense point cloud, when `EngineParams::geo_enabled`.
    /// `None` when geo products weren't requested, or there's no (or an empty) dense cloud yet.
    #[cfg(test)]
    pub fn take_geo_products(&mut self) -> Option<GeoProducts> {
        if !self.params.geo_enabled {
            return None;
        }
        let cloud = self.dense_cloud.as_ref()?;
        if cloud.is_empty() {
            return None;
        }
        let (lo, hi) = point_cloud_bbox(cloud)?;
        let cell = self.params.geo_cell_size.max(1e-6);
        let width = (((hi[0] - lo[0]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let height = (((hi[1] - lo[1]) / cell).ceil() as u32 + 1).clamp(1, 512);
        let origin = [lo[0], lo[1]];
        let dsm = remodeling_geo::build_dsm(cloud, cell, origin, width, height);
        let dtm = remodeling_geo::build_dtm(cloud, cell, origin, width, height);
        Some(GeoProducts { dsm, dtm })
    }
}
// #endregion 🔖️Products
