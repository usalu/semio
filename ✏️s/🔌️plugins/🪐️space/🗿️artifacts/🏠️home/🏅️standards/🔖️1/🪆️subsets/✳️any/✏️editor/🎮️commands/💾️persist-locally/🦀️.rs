//! 💾 S Home launcher app command — `persist-locally`.
//! Event-sourced persistence of an ephemeral local-only studio onto the local catalog
//! (folder backbone). Share/collaboration stay blocked; hub promotion is a separate command.

use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "persist-locally")]
pub struct PersistLocally {
    pub space_id: String,
    pub folder_path: Option<String>,
}

/// @emoji 🎯️ This IS "save this draft as a real asset" for a whole space — binding
/// an ephemeral studio to a folder backbone is the natural local persist moment (sibling of
/// `bind-space-file` for file backbones).
#[cfg(not(target_arch = "wasm32"))]
fn bind_studio_folder(space_id: &str, folder_path: &str) -> Result<(), VcsError> {
    use semio_framework_os::{document_backbone_ref, encode_backbone_payload, OsBackbonePort as _, OS_SPACE_BACKBONE_URI_PREFIX};
    let uri = format!("folder://{folder_path}");
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    semio_framework_plugin::resolve_ready(crate::register_studio_port(space_id, port.clone()));
    let mut document = semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id)).ok_or_else(|| VcsError::Backbone(format!("unknown space {space_id}")))?;
    document.backbone = Some(semio_framework_plugin::resolve_ready(document_backbone_ref(&uri)));
    port.write(&uri, &encode_backbone_payload(&document)?)?;
    let catalog_uri = format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}");
    semio_framework_plugin::resolve_ready(crate::sync_os_space_document_helper(&document, &catalog_uri, &semio_framework_plugin::resolve_ready(crate::catalog_port())))?;
    let draft_port = semio_framework_plugin::resolve_ready(crate::draft_backbone_port());
    semio_framework_plugin::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, space_id);
    Ok(())
}

pub fn handle(payload: &PersistLocally, doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    if payload.folder_path.as_ref().map(|path| path.trim().is_empty()).unwrap_or(true) {
        let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": payload.space_id.clone() })));
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(127), dialog_id: "persistLocally".into(), args }));
    }
    let folder_path = payload.folder_path.clone().unwrap_or_default();
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = bind_studio_folder(&payload.space_id, &folder_path);
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (&payload.space_id, &folder_path);
    }
    Ok(Emit {
        artifact_mutations: vec![change_catalog_generation(doc.snapshot.catalog_generation + 1)],
        ..Default::default()
    })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
