//! 🧬️ Jack diff schema — sparse field delta over the artifact.
//!
//! `content: Option<JackContentChild>` is the always-present composed child slot; scene edits never pass through it, they
//! are child-lane leaves of the shared graph vocabulary (design §20.15).

use crate::{Camera, JackContentChild};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the jack artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.trinity.jack")]
pub struct JackDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub name: Option<String>,
    #[state(artifact)]
    pub manifest_id: Option<Option<String>>,
    #[state(artifact)]
    pub manifest: Option<crate::Manifest>,
    #[state(artifact)]
    pub camera: Option<Camera>,
    #[state(artifact)]
    pub content: Option<JackContentChild>,
    #[state(artifact)]
    pub root_node_id: Option<Option<String>>,
    #[state(artifact)]
    pub query: Option<String>,
}
//#endregion 🔖️Diff
