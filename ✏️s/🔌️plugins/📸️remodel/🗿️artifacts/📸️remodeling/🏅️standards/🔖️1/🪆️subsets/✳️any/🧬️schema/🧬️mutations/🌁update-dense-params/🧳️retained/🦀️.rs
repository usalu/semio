//! 🧳️ Dense-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateDenseParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::DenseParams;
semio_s_artifact_remodel_remodeling_mut_params_2::retained_remodeling_parameter_field!(UpdateDenseParams, DenseParams, dense, 4);
impl semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterScalar for UpdateDenseParams {
    const WORK_DEMAND: u64 = 6;
    const NOOP_MESSAGE: &'static str = "Dense params are already up to date.";
    const REFUSAL_MESSAGE: Option<&'static str> = Some("Dense params have a non-finite confidence threshold.");
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition, u64) {
        use semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.dense;
        if !p.confidence_threshold.is_finite() { return (TargetedRefusal, 1); }
        let same = (p.resolution == old.resolution) & (p.window_radius_px == old.window_radius_px) & (p.min_view_consistency == old.min_view_consistency) & (p.confidence_threshold == old.confidence_threshold) & (p.max_points == old.max_points);
        (if same { Unchanged } else { Changed }, 6)
    }
}
semio_s_artifact_remodel_remodeling_mut_params_2::retained_parameter_scalar_vectors!(UpdateDenseParams, "dense", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.dense.clone();
    if let Some(value) = change.get("max_points") { params.max_points = value.as_u64().unwrap() as u32; }
    if change.contains_key("confidence_threshold") { params.confidence_threshold = f32::NAN; }
    UpdateDenseParams { params }
});
