//! 🏙️ 🏙️ S Home launcher app command — `bind-space-file`.

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};

use crate::artifacts::home::op::SHomeMutation;
use crate::artifacts::home::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "bind-space-file")]
pub struct BindSpaceFile {
    pub space_id: String,
    pub file_path: String,
}

/// @emoji 🎯️ This IS "save this draft as a real asset" for a whole space manifest — binding a studio
/// to a real file/catalog location is the natural persist moment.
#[cfg(not(target_arch = "wasm32"))]
fn bind_studio_file(space_id: &str, file_path: &str) -> Result<(), VcsError> {
    use semio_framework_os::{document_backbone_ref, encode_backbone_payload, OS_SPACE_BACKBONE_URI_PREFIX};
    let uri = format!("file://{file_path}");
    let port = semio_framework_os::open_file_space_backbone(file_path)?;
    // 🌉️ `register_studio_port`/`resolve_studio_document`/`catalog_port`/`draft_backbone_port`/
    // `ephemeral_draft_catalog`/`sync_os_space_document_helper` are plugin-root async fns (outside
    // this lease); `handle` must stay sync — bridged via `resolve_ready`, matching
    // `🏙️create-studio`'s own seam.
    semio_framework_plugin::resolve_ready(crate::register_studio_port(space_id, port.clone()));
    let mut document = semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id)).ok_or_else(|| VcsError::Backbone(format!("unknown space {space_id}")))?;
    document.backbone = Some(document_backbone_ref(&uri));
    port.write(&uri, &encode_backbone_payload(&document)?)?;
    let catalog_uri = format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}");
    semio_framework_plugin::resolve_ready(crate::sync_os_space_document_helper(&document, &catalog_uri, &semio_framework_plugin::resolve_ready(crate::catalog_port())))?;
    let draft_port = semio_framework_plugin::resolve_ready(crate::draft_backbone_port());
    semio_framework_plugin::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, space_id);
    Ok(())
}

pub fn handle(payload: &BindSpaceFile, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = bind_studio_file(&payload.space_id, &payload.file_path);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (&payload.space_id, &payload.file_path);
    }
    Ok(Emit::default())
}
