//! 🧬️ schema leaf
use crate::artifacts::jack::Camera;
use schema::ArtifactSchema;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.rewrite.presence")]
pub struct RewritePresence {
    #[state(presence)]
    pub before_pane_camera: Camera,
    #[state(presence)]
    pub lod_mode_by_window: BTreeMap<String, String>,
}
