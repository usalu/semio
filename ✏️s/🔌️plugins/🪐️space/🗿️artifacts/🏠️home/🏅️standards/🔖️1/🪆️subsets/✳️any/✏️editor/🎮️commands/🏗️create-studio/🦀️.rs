//! 🏙️ 🏙️ S Home launcher app command — `create-studio`.

use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "create-studio")]
pub struct CreateStudio {
    pub name: String,
    pub kind: String,
    pub folder_path: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
fn create_folder_studio(name: &str, folder_path: &str, owner_id: &str, owner_name: &str) -> Result<semio_framework_os::OsSpaceCatalogEntry, VcsError> {
    use semio_framework_artifact_space_space::{SpaceKind, SpaceRole, SpaceUser, SpaceVisibility};
    use semio_framework_os::create_os_space;
    let port = semio_framework_os::open_folder_space_backbone(folder_path)?;
    let owner = SpaceUser { id: if owner_id.is_empty() { "local".into() } else { owner_id.into() }, name: if owner_name.is_empty() { name.into() } else { owner_name.into() }, avatar: None, role: SpaceRole::Author };
    let entry = create_os_space(name, SpaceKind::Atelier, SpaceVisibility::Private, owner, &port)?;
    semio_framework_plugin::resolve_ready(crate::register_studio_port(&entry.id, port));
    Ok(entry)
}

/// @emoji 🧭️ Builds the typed emit for a freshly-created studio: bump the catalog counter (operation)
/// and navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeMutation, HomeConfigMutation> {
    Emit { artifact_mutations: vec![change_catalog_generation(catalog_generation + 1)], effects: vec![Effect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
}

pub fn handle(payload: &CreateStudio, doc: &ArtifactView<'_, SHomeSnapshot>, cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    let owner_id = cfg.snapshot.client_id.as_str();
    let owner_name = cfg.snapshot.client_name.as_str();
    match payload.kind.as_str() {
        "folder" => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Some(folder_path) = &payload.folder_path {
                    if let Ok(entry) = create_folder_studio(&payload.name, folder_path, owner_id, owner_name) {
                        eprintln!("[DEBUG] createStudio folder id={}", entry.id);
                        return Ok(created_studio_emit(generation, &entry.id));
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = &payload.folder_path;
            }
            Ok(Emit::default())
        }
        _ => {
            // 🌉️ `crate::create_and_register_ephemeral_studio` is a plugin-root async fn (outside
            // this lease); `handle` must stay sync (the `app_commands!` dispatch contract), so the
            // call is bridged via `resolve_ready` — the same poll-once bridge the framework's own
            // `composer_entry_of`/`deserializer_entry_of` use for an identical sync/async seam.
            let space_id = semio_framework_plugin::resolve_ready(crate::create_and_register_ephemeral_studio(&payload.name, owner_id, owner_name));
            eprintln!("[DEBUG] createStudio ephemeral id={space_id}");
            Ok(created_studio_emit(generation, &space_id))
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
