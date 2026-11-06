//! 🧳️ Motion-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateMotionParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MotionParams;
semio_s_artifact_remodel_remodeling_mut_params_2::retained_remodeling_parameter_field!(UpdateMotionParams, MotionParams, motion, 6);
impl semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterScalar for UpdateMotionParams {
    const WORK_DEMAND: u64 = 6;
    const NOOP_MESSAGE: &'static str = "Motion params are already up to date.";
    const REFUSAL_MESSAGE: Option<&'static str> = Some("Motion params have a non-finite minimum track quality.");
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition, u64) {
        use semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.motion;
        if !p.min_track_quality.is_finite() { return (TargetedRefusal, 1); }
        let same = (p.enabled == old.enabled) & (p.max_tracks == old.max_tracks) & (p.track_window_px == old.track_window_px) & (p.min_track_quality == old.min_track_quality) & (p.min_track_length_frames == old.min_track_length_frames);
        (if same { Unchanged } else { Changed }, 6)
    }
}
semio_s_artifact_remodel_remodeling_mut_params_2::retained_parameter_scalar_vectors!(UpdateMotionParams, "motion", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.motion.clone();
    if let Some(value) = change.get("enabled") { params.enabled = value.as_bool().unwrap(); }
    if change.contains_key("min_track_quality") { params.min_track_quality = f32::NAN; }
    UpdateMotionParams { params }
});
