//! 🧳️ Feature-parameter changed-value projection and inverse, after its retained decision.

use super::UpdateFeatureParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::FeatureParams;

crate::retained_remodeling_parameter_field!(UpdateFeatureParams, FeatureParams, feature, 1);

impl crate::RetainedParameterScalar for UpdateFeatureParams {
    const WORK_DEMAND: u64 = 7;
    const NOOP_MESSAGE: &'static str = "Feature params are unchanged.";
    fn retained_parameter_numeric_source(source: semio_framework_value::retained_clone::RetainedCloneRef<'_, Self>) -> Option<crate::RetainedParameterTextSource<'_>> { Some(crate::RetainedParameterTextSource::Feature(source.project(0, |value| &value.params))) }
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (crate::RetainedParameterDisposition, u64) {
        use crate::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.feature;
        if (p.target_count == 0) | !p.edge_threshold.is_finite() | (p.edge_threshold < 0.0) { return (NumericRefusal, 3); }
        let same = (p.detector == old.detector) & (p.target_count == old.target_count) & (p.octaves == old.octaves) & (p.edge_threshold == old.edge_threshold);
        (if same { Unchanged } else { Changed }, 7)
    }
}

crate::retained_parameter_scalar_vectors!(UpdateFeatureParams, "feature", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.feature.clone();
    if let Some(value) = change.get("target_count") { params.target_count = value.as_u64().unwrap() as u32; }
    UpdateFeatureParams { params }
});
