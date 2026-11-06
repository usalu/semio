//! 🧳️ Ingest-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateIngestParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::IngestParams;
semio_s_artifact_remodel_remodeling_mut_params_2::retained_remodeling_parameter_field!(UpdateIngestParams, IngestParams, ingest, 0);
impl semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterScalar for UpdateIngestParams {
    const WORK_DEMAND: u64 = 8;
    const NOOP_MESSAGE: &'static str = "Ingest params are unchanged.";
    fn retained_parameter_numeric_source(source: semio_framework_value::retained_clone::RetainedCloneRef<'_, Self>) -> Option<semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterTextSource<'_>> { Some(semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterTextSource::Ingest(source.project(0, |value| &value.params))) }
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition, u64) {
        use semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.ingest;
        if !p.min_sharpness.is_finite() | (p.min_sharpness < 0.0) | (p.max_frames == 0) | (p.frame_sample_stride == 0) { return (NumericRefusal, 4); }
        let same = (p.frame_sample_stride == old.frame_sample_stride) & (p.max_frames == old.max_frames) & (p.downscale_long_edge_px == old.downscale_long_edge_px) & (p.min_sharpness == old.min_sharpness);
        (if same { Unchanged } else { Changed }, 8)
    }
}
semio_s_artifact_remodel_remodeling_mut_params_2::retained_parameter_scalar_vectors!(UpdateIngestParams, "ingest", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.ingest.clone();
    if let Some(value) = change.get("frame_sample_stride") { params.frame_sample_stride = value.as_u64().unwrap() as u32; }
    UpdateIngestParams { params }
});
