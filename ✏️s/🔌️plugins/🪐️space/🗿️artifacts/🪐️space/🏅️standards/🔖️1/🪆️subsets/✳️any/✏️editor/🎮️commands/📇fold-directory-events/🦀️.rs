//! 📇️ SpaceIndexEditor commands command — `fold-directory-events`. Config-only (never touches the
//! shared `SSpaceSnapshot` document — contract §C4: space name/kind/visibility/members are
//! directory-owned, never duplicated into the index document). Reuses the OS's own pure
//! `semio_framework_os::os_directory::fold_all` (contract §C1) rather than re-deriving the fold
//! logic here — the shell is expected to pass the FULL event history it holds for this space each
//! dispatch (this command carries no cursor of its own, so a partial/delta batch would silently
//! regress the folded members/visibility).

use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation, SpaceIndexMember};
use semio_framework_os_kernel::os_directory::{fold_all, DirectoryEvent, DirectoryReadModel, DirectorySpaceRole, DirectorySpaceVisibility};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "fold-directory-events")]
pub struct FoldDirectoryEvents {
    pub events_json: String,
}

fn role_str(role: DirectorySpaceRole) -> &'static str {
    match role {
        DirectorySpaceRole::Author => "author",
        DirectorySpaceRole::Spectator => "spectator",
    }
}

fn visibility_str(visibility: DirectorySpaceVisibility) -> &'static str {
    match visibility {
        DirectorySpaceVisibility::Private => "private",
        DirectorySpaceVisibility::Public => "public",
    }
}

pub fn handle(payload: &FoldDirectoryEvents, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let events: Vec<DirectoryEvent> = pack::from_json_str(&payload.events_json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.directory.decode"), error.to_string()))?;
    let model = semio_framework_plugin::resolve_ready(fold_all(DirectoryReadModel::default(), &events));
    let Some(space) = model.spaces.get(&doc.snapshot.space_id) else {
        return Ok(Emit::default());
    };
    let next = SpaceIndexConfig {
        visibility: visibility_str(space.view.visibility).into(),
        members: space.members.iter().map(|member| SpaceIndexMember { user_id: member.user_id.clone(), email: member.email.clone(), display_name: member.display_name.clone(), role: role_str(member.role).into() }).collect(),
        indexed_artifacts: space.indexed_documents.iter().filter_map(SpaceIndexConfig::indexed_artifact_from_directory).collect(),
        presence: cfg.snapshot.presence.clone(),
    };
    Ok(Emit { config_mutations: vec![SpaceIndexConfigMutation::Snapshot { config: next }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
