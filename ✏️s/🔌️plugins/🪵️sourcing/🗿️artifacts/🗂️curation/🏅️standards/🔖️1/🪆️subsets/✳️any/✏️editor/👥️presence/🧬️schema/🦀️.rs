//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.sourcing.curation.presence")]
pub struct SourcingCurationPresence {
    #[state(presence)]
    pub world_camera_position: [f64; 3],
    #[state(presence)]
    pub world_camera_target: [f64; 3],
    #[state(presence)]
    pub world_camera_fov: f64,
}
