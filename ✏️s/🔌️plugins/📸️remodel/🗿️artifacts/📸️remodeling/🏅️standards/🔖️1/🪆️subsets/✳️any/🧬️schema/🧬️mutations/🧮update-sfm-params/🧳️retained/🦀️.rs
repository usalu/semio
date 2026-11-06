//! 🧳️ SfM-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateSfmParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::SfmParams;
crate::retained_remodeling_parameter_field!(UpdateSfmParams, SfmParams, sfm, 3);
impl crate::RetainedParameterScalar for UpdateSfmParams {
    const WORK_DEMAND: u64 = 8;
    const NOOP_MESSAGE: &'static str = "SfM params are already up to date.";
    const REFUSAL_MESSAGE: Option<&'static str> = Some("SfM params have non-finite thresholds.");
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (crate::RetainedParameterDisposition, u64) {
        use crate::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.sfm;
        if !p.ransac_threshold_px.is_finite() | !p.huber_delta_px.is_finite() { return (TargetedRefusal, 2); }
        let same = (p.ransac_iterations == old.ransac_iterations) & (p.ransac_threshold_px == old.ransac_threshold_px) & (p.min_track_length == old.min_track_length) & (p.ba_max_iterations == old.ba_max_iterations) & (p.robust_loss == old.robust_loss) & (p.huber_delta_px == old.huber_delta_px);
        (if same { Unchanged } else { Changed }, 8)
    }
}
crate::retained_parameter_scalar_vectors!(UpdateSfmParams, "sfm", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.sfm.clone();
    if let Some(value) = change.get("ransac_iterations") { params.ransac_iterations = value.as_u64().unwrap() as u32; }
    if change.contains_key("ransac_threshold_px") { params.ransac_threshold_px = f32::NAN; }
    UpdateSfmParams { params }
});
