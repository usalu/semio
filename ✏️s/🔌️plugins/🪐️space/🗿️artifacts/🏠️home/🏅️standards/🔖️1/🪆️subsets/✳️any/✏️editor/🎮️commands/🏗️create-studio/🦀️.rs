//! 🏙️ 🏙️ S Home launcher app command — `create-studio`.
//! Temporary studios are classified `ephemeralLocalOnly` (no backbone);
//! share/collaboration stay blocked until `promote-to-hub-space` or `persist-locally`. A folder studio is refused by name
//! when no folder is named, when its folder cannot be written, or on a host that gives studios no filesystem (every
//! `wasm32` guest) — never answered with an empty success.

use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os::VcsError;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
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
    ::semio_framework_async::poll::resolve_ready(semio_s_space_core::register_studio_port(&entry.id, port));
    Ok(entry)
}

/// 🧭️ Builds the typed emit for a freshly-created studio: bump the catalog counter (operation)
/// and navigate the shell to the new studio route (host effect).
fn created_studio_emit(catalog_generation: u64, space_id: &str) -> Emit<SHomeMutation, HomeConfigMutation> {
    Emit { artifact_mutations: vec![change_catalog_generation(catalog_generation + 1)], effects: vec![Effect::Navigate { uri: format!("/spaces/{space_id}") }], ..Default::default() }
}

pub fn handle(_payload: &CreateStudio, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::from("s.home.create-studio.effect-authority-required"))
}

pub fn handle_with_identity(
    payload: &CreateStudio,
    doc: &ArtifactView<'_, SHomeSnapshot>,
    _cfg: &ConfigView<'_, HomeConfig>,
    session: &semio_framework_plugin::ViewSessionIdentity,
    identity: &mut store::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,
) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let generation = doc.snapshot.catalog_generation;
    let owner_id = session.user_id.as_str();
    let owner_name = session.display_name.as_str();
    match payload.kind.as_str() {
        "folder" => {
            let folder_path = payload.folder_path.as_deref().map(str::trim).filter(|path| !path.is_empty()).ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.create-studio.folder-path-required", "a folder studio names the folder it is kept in"))?;
            #[cfg(not(target_arch = "wasm32"))]
            {
                let entry = create_folder_studio(&payload.name, folder_path, owner_id, owner_name).map_err(|error| Fault::new(FaultOrigin::App, "s.home.create-studio.io-failed", format!("creating a studio in {folder_path} failed: {error:?}")))?;
                Ok(created_studio_emit(generation, &entry.id))
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = (folder_path, owner_id, owner_name);
                Err(Fault::new(FaultOrigin::App, "s.home.create-studio.filesystem-unavailable", "this host gives studios no filesystem; create a temporary studio or a hub space instead"))
            }
        }
        _ => {
            let space_id = ::semio_framework_async::poll::resolve_ready(semio_s_space_core::create_and_register_ephemeral_studio(&payload.name, owner_id, owner_name, identity)).map_err(|error| Fault::new(FaultOrigin::App, "s.home.create-studio.io-failed", format!("creating a temporary studio failed: {error}")))?;
            Ok(created_studio_emit(generation, &space_id))
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
