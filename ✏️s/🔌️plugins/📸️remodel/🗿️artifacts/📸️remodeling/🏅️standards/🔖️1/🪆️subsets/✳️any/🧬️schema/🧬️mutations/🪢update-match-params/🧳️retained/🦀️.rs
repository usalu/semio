//! 🧳️ Matching-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateMatchParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::MatchParams;
semio_s_artifact_remodel_remodeling_mut_params_2::retained_remodeling_parameter_field!(UpdateMatchParams, MatchParams, matching, 2);
impl semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterScalar for UpdateMatchParams {
    const WORK_DEMAND: u64 = 9;
    const NOOP_MESSAGE: &'static str = "Matching params are unchanged.";
    fn retained_parameter_numeric_source(source: semio_framework_value::retained_clone::RetainedCloneRef<'_, Self>) -> Option<semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterTextSource<'_>> { Some(semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterTextSource::Matching(source.project(0, |value| &value.params))) }
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition, u64) {
        use semio_s_artifact_remodel_remodeling_mut_params_2::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.matching;
        if !p.ratio_test.is_finite() | (p.ratio_test <= 0.0) | (p.ratio_test > 1.0) { return (NumericRefusal, 3); }
        let same = (p.matcher == old.matcher) & (p.ratio_test == old.ratio_test) & (p.cross_check == old.cross_check) & (p.sequential_window == old.sequential_window) & (p.max_pairs_per_frame == old.max_pairs_per_frame) & (p.loop_closure == old.loop_closure);
        (if same { Unchanged } else { Changed }, 9)
    }
}
semio_s_artifact_remodel_remodeling_mut_params_2::retained_parameter_scalar_vectors!(UpdateMatchParams, "matching", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.matching.clone();
    if let Some(value) = change.get("sequential_window") { params.sequential_window = value.as_u64().unwrap() as u32; }
    if let Some(value) = change.get("ratio_test") { params.ratio_test = value.as_f64().unwrap() as f32; }
    UpdateMatchParams { params }
});
