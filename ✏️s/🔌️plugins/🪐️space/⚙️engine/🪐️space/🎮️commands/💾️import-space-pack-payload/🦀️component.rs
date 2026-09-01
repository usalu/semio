//! 💾️ 💾️ S Studio app command — `import-space-pack-payload`.

use crate::engine::space::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::host::import_os_space_from_pack;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot, OS_SPACE_SCHEMA};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};


#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-space-pack-payload")]
pub struct ImportSpacePackPayload {
    pub payload: String,
}

pub async fn handle(payload: &ImportSpacePackPayload, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let base64_part = payload.payload.split_once(',').map_or(payload.payload.as_str(), |(_, data)| data);
    if let Ok(bytes) = base64_codec::base64_standard_decode(base64_part) {
        // 🌱️ A single `.pack` file carries no separate `.spr` sidecar (unlike `exportStudioPack`'s
        // two-file output) — `store::empty_document_spr` builds a bare, edit-free op log so the
        // pack+spr-first codec path still decodes to a document with no replayed edit history, i.e.
        // its bare initial projection.
        let empty_spr = store::empty_document_spr("", OS_SPACE_SCHEMA);
        let _ = import_os_space_from_pack(&bytes, &empty_spr, crate::catalog_port());
    }
    Ok(Emit::default())
}
