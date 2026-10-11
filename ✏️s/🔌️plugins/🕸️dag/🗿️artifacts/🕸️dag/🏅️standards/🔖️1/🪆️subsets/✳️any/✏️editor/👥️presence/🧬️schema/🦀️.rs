//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.dag.dag.presence")]
pub struct DagPresence {
    #[state(presence)]
    pub camera_x: f64,
    #[state(presence)]
    pub camera_y: f64,
    #[state(presence)]
    pub camera_zoom: f64,
}

impl store::ArtifactPresenceSnapshot for DagPresence {}
