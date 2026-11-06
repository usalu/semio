//! ⚙️ Reconstruction parameter patches retain authored and original facet ownership through funded exchange and close.
use super::*;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;

protocol::retained_empty_option_patch!(RemodelingParamsDiff { ingest, feature, matching, sfm, dense, mesh, motion, geo });
protocol::retained_row_patch!(pub fields for RemodelingParamsDiff => ReconstructionParams {
    ingest: IngestParams,
    feature: FeatureParams,
    matching: MatchParams,
    sfm: SfmParams,
    dense: DenseParams,
    mesh: MeshParams,
    motion: MotionParams,
    geo: GeoParams,
});
