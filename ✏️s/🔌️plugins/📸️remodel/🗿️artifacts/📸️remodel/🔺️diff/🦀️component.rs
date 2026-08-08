//! 🔺️ Remodel artifact — the durable mutation diff (`Mutation::Diff`) and its `MutationDiff` law.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::remodel::mutations::{apply_remodel_mutation, RemodelMutation};
use crate::artifacts::remodel::{
    CalibrationState, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, GeoParams, GeoProducts, GroundControlPoint, ImageAsset, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary, QcReportSnapshot,
    ReconstructionJob, RemodelMesh, RemodelProjection, SfmParams, SparseCloud,
};
use protocol::MutationDiff;
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

impl MutationDiff<RemodelProjection> for RemodelDiff {
    fn apply(&self, projection: &RemodelProjection) -> RemodelProjection {
        let operation = match self {
            RemodelDiff::Empty => return projection.clone(),
            RemodelDiff::SetStreams { streams } => RemodelMutation::SetStreams { streams: streams.clone() },
            RemodelDiff::SetAsset { key, value } => RemodelMutation::SetAsset { key: key.clone(), value: value.clone() },
            RemodelDiff::SetCalibration { calibration } => RemodelMutation::SetCalibration { calibration: calibration.clone() },
            RemodelDiff::SetGcps { gcps } => RemodelMutation::SetGcps { gcps: gcps.clone() },
            RemodelDiff::SetIngestParams { params } => RemodelMutation::SetIngestParams { params: params.clone() },
            RemodelDiff::SetFeatureParams { params } => RemodelMutation::SetFeatureParams { params: params.clone() },
            RemodelDiff::SetMatchParams { params } => RemodelMutation::SetMatchParams { params: params.clone() },
            RemodelDiff::SetSfmParams { params } => RemodelMutation::SetSfmParams { params: params.clone() },
            RemodelDiff::SetDenseParams { params } => RemodelMutation::SetDenseParams { params: params.clone() },
            RemodelDiff::SetMeshParams { params } => RemodelMutation::SetMeshParams { params: params.clone() },
            RemodelDiff::SetMotionParams { params } => RemodelMutation::SetMotionParams { params: params.clone() },
            RemodelDiff::SetGeoParams { params } => RemodelMutation::SetGeoParams { params: params.clone() },
            RemodelDiff::SetJob { job } => RemodelMutation::SetJob { job: job.clone() },
            RemodelDiff::SetSparse { sparse } => RemodelMutation::SetSparse { sparse: sparse.clone() },
            RemodelDiff::SetDense { dense } => RemodelMutation::SetDense { dense: dense.clone() },
            RemodelDiff::SetMeshResult { mesh } => RemodelMutation::SetMeshResult { mesh: mesh.clone() },
            RemodelDiff::SetTrajectory { trajectory } => RemodelMutation::SetTrajectory { trajectory: trajectory.clone() },
            RemodelDiff::SetTracks { tracks } => RemodelMutation::SetTracks { tracks: tracks.clone() },
            RemodelDiff::SetGeoProducts { geo } => RemodelMutation::SetGeoProducts { geo: geo.clone() },
            RemodelDiff::SetQc { qc } => RemodelMutation::SetQc { qc: qc.clone() },
        };
        apply_remodel_mutation(projection, &operation)
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
    /// `Mutation::diff` in the `🔧️op` node's tests).
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
