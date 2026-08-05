//! 🔺️ Remodel artifact — the durable operation diff (`Operation::Diff`) and its `OperationDiff` law.

use crate::artifacts::remodel::op::{apply_remodel_operation, RemodelOperation};
use crate::artifacts::remodel::{
    CalibrationState, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary, QcReportSnapshot,
    ReconstructionJob, RemodelMesh, RemodelScene, SfmParams, SparseCloud,
};
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum RemodelDiff {
    #[default]
    Empty,
    SetStreams {
        streams: Vec<MediaStream>,
    },
    SetAsset {
        key: String,
        #[serde(default)]
        value: Option<ImageAsset>,
    },
    SetCalibration {
        calibration: CalibrationState,
    },
    SetGcps {
        gcps: Vec<GroundControlPoint>,
    },
    SetIngestParams {
        params: IngestParams,
    },
    SetFeatureParams {
        params: FeatureParams,
    },
    SetMatchParams {
        params: MatchParams,
    },
    SetSfmParams {
        params: SfmParams,
    },
    SetDenseParams {
        params: DenseParams,
    },
    SetMeshParams {
        params: MeshParams,
    },
    SetMotionParams {
        params: MotionParams,
    },
    SetGeoParams {
        params: GeoParams,
    },
    SetJob {
        job: ReconstructionJob,
    },
    SetSparse {
        #[serde(default)]
        sparse: Option<SparseCloud>,
    },
    SetDense {
        #[serde(default)]
        dense: Option<DenseCloud>,
    },
    SetMeshResult {
        mesh: Box<RemodelMesh>,
    },
    SetTrajectory {
        #[serde(default)]
        trajectory: Option<CameraTrajectory>,
    },
    SetTracks {
        tracks: Vec<MotionTrackSummary>,
    },
    SetGeoProducts {
        #[serde(default)]
        geo: Option<GeoProducts>,
    },
    SetQc {
        #[serde(default)]
        qc: Option<QcReportSnapshot>,
    },
}

impl OperationDiff<RemodelScene> for RemodelDiff {
    fn apply(&self, projection: &RemodelScene) -> RemodelScene {
        let operation = match self {
            RemodelDiff::Empty => return projection.clone(),
            RemodelDiff::SetStreams { streams } => RemodelOperation::SetStreams { streams: streams.clone() },
            RemodelDiff::SetAsset { key, value } => RemodelOperation::SetAsset { key: key.clone(), value: value.clone() },
            RemodelDiff::SetCalibration { calibration } => RemodelOperation::SetCalibration { calibration: calibration.clone() },
            RemodelDiff::SetGcps { gcps } => RemodelOperation::SetGcps { gcps: gcps.clone() },
            RemodelDiff::SetIngestParams { params } => RemodelOperation::SetIngestParams { params: params.clone() },
            RemodelDiff::SetFeatureParams { params } => RemodelOperation::SetFeatureParams { params: params.clone() },
            RemodelDiff::SetMatchParams { params } => RemodelOperation::SetMatchParams { params: params.clone() },
            RemodelDiff::SetSfmParams { params } => RemodelOperation::SetSfmParams { params: params.clone() },
            RemodelDiff::SetDenseParams { params } => RemodelOperation::SetDenseParams { params: params.clone() },
            RemodelDiff::SetMeshParams { params } => RemodelOperation::SetMeshParams { params: params.clone() },
            RemodelDiff::SetMotionParams { params } => RemodelOperation::SetMotionParams { params: params.clone() },
            RemodelDiff::SetGeoParams { params } => RemodelOperation::SetGeoParams { params: params.clone() },
            RemodelDiff::SetJob { job } => RemodelOperation::SetJob { job: job.clone() },
            RemodelDiff::SetSparse { sparse } => RemodelOperation::SetSparse { sparse: sparse.clone() },
            RemodelDiff::SetDense { dense } => RemodelOperation::SetDense { dense: dense.clone() },
            RemodelDiff::SetMeshResult { mesh } => RemodelOperation::SetMeshResult { mesh: mesh.clone() },
            RemodelDiff::SetTrajectory { trajectory } => RemodelOperation::SetTrajectory { trajectory: trajectory.clone() },
            RemodelDiff::SetTracks { tracks } => RemodelOperation::SetTracks { tracks: tracks.clone() },
            RemodelDiff::SetGeoProducts { geo } => RemodelOperation::SetGeoProducts { geo: geo.clone() },
            RemodelDiff::SetQc { qc } => RemodelOperation::SetQc { qc: qc.clone() },
        };
        apply_remodel_operation(projection, &operation)
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, RemodelDiff::Empty) {
            *self = other;
        }
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::remodel::default_remodel_scene;

    /// 🫙️ `Empty` is the identity diff, and `absorb` is last-writer-wins over any non-`Empty` value —
    /// the two laws that are the diff node's own (every other diff behaviour is exercised through
    /// `Operation::diff` in the `🔧️op` node's tests).
    #[test]
    fn empty_diff_is_the_identity_and_absorb_is_last_writer_wins() {
        let scene = default_remodel_scene();
        assert_eq!(RemodelDiff::Empty.apply(&scene), scene);

        let mut diff = RemodelDiff::Empty;
        diff.absorb(RemodelDiff::SetGcps { gcps: Vec::new() });
        assert!(matches!(diff, RemodelDiff::SetGcps { .. }));
        diff.absorb(RemodelDiff::Empty);
        assert!(matches!(diff, RemodelDiff::SetGcps { .. }), "absorbing Empty never clobbers a real diff");
    }
}
//#endregion 🧪️Tests
