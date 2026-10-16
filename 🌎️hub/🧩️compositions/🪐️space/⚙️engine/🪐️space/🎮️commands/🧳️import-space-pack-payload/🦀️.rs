//! 💾️ 💾️ S Studio app command — `import-space-pack-payload`.

use semio_hub_space_foundation::config::{SpaceConfig, SpaceConfigMutation};
use semio_framework_os::host::import_os_space_from_pack;
use semio_framework_os::{WorkflowMutation, WorkflowSnapshot, OS_SPACE_SCHEMA};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "import-space-pack-payload")]
pub struct ImportSpacePackPayload {
    pub payload: String,
}

pub fn handle(payload: &ImportSpacePackPayload, _doc: &ArtifactView<'_, WorkflowSnapshot>, _cfg: &ConfigView<'_, SpaceConfig>) -> Result<Emit<WorkflowMutation, SpaceConfigMutation>, Fault> {
    let base64_part = payload.payload.split_once(',').map_or(payload.payload.as_str(), |(_, data)| data);
    if let Ok(bytes) = base64_codec::base64_standard_decode(base64_part) {
        // 🌱️ A single `.pack` file carries no separate `.spr` sidecar (unlike `exportStudioPack`'s
        // two-file output) — `store::os_store::text_format::empty_document_spr` builds a bare, edit-free op log so the
        // pack+spr-first codec path still decodes to a document with no replayed edit history, i.e.
        // its bare initial projection.
        let empty_spr = semio_hub_space_operations::resolve_future(store::os_store::text_format::empty_document_spr("", OS_SPACE_SCHEMA));
        let port = semio_hub_space_operations::resolve_future(crate::catalog_port());
        let _ = import_os_space_from_pack(&bytes, &empty_spr, &port);
    }
    Ok(Emit::default())
}
