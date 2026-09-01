//! 📇️ SpaceIndexEditor commands command — `fold-directory-events`. Config-only (never touches the
//! shared `SSpaceSnapshot` document — contract §C4: space name/kind/visibility/members are
//! directory-owned, never duplicated into the index document). Reuses the OS's own pure
//! `semio_framework_os::os_directory::fold_all` (contract §C1) rather than re-deriving the fold
//! logic here — the shell is expected to pass the FULL event history it holds for this space each
//! dispatch (this command carries no cursor of its own, so a partial/delta batch would silently
//! regress the folded members/visibility).

use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::SSpaceSnapshot;
use crate::editor::space_index::config::{SpaceIndexConfig, SpaceIndexConfigMutation, SpaceIndexMember};
use semio_framework_os_kernel::os_directory::{fold_all, DirectoryEvent, DirectoryReadModel, DirectorySpaceRole, DirectorySpaceVisibility};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "fold-directory-events")]
pub struct FoldDirectoryEvents {
    pub events_json: String,
}

async fn role_str(role: DirectorySpaceRole) -> &'static str {
    match role {
        DirectorySpaceRole::Author => "author",
        DirectorySpaceRole::Spectator => "spectator",
    }
}

async fn visibility_str(visibility: DirectorySpaceVisibility) -> &'static str {
    match visibility {
        DirectorySpaceVisibility::Private => "private",
        DirectorySpaceVisibility::Public => "public",
    }
}

pub async fn handle(payload: &FoldDirectoryEvents, doc: &ArtifactView<'_, SSpaceSnapshot>, cfg: &ConfigView<'_, SpaceIndexConfig>) -> Result<Emit<SSpaceMutation, SpaceIndexConfigMutation>, Fault> {
    let events: Vec<DirectoryEvent> = pack::from_json_str(&payload.events_json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("s.space.directory.decode"), error.to_string()))?;
    let model = fold_all(DirectoryReadModel::default(), &events);
    let Some(space) = model.spaces.get(&doc.snapshot.space_id) else {
        return Ok(Emit::default());
    };
    let next = SpaceIndexConfig {
        visibility: visibility_str(space.view.visibility).into(),
        members: space.members.iter().map(|member| SpaceIndexMember { user_id: member.user_id.clone(), email: member.email.clone(), display_name: member.display_name.clone(), role: role_str(member.role).into() }).collect(),
        presence: cfg.snapshot.presence.clone(),
    };
    Ok(Emit { config_mutations: vec![SpaceIndexConfigMutation::Snapshot { config: next }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_os_kernel::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEventBody, DirectorySpaceKind, Hlc};
    use semio_framework_plugin::{ArtifactView, HistoryView};

    async fn event(seq: u64, body: DirectoryEventBody, space_id: Option<&str>) -> DirectoryEvent {
        DirectoryEvent {
            seq,
            id: format!("evt-{seq}"),
            hlc: Hlc { physical_ms: seq as i64, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
            space_id: space_id.map(Into::into),
            user_id: None,
            body,
            recorded_at_ms: seq as i64,
        }
    }

    async fn view_for(space_id: &str) -> SSpaceSnapshot {
        SSpaceSnapshot { space_id: space_id.into(), ..Default::default() }
    }

    #[semio_framework_async_macros::async_test]
    async fn folds_visibility_and_members_for_this_space_into_config() {
        let snapshot = view_for("space-1");
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let config_snapshot = SpaceIndexConfig::default();
        let cfg = ConfigView { snapshot: &config_snapshot };
        let events = vec![
            event(1, DirectoryEventBody::UserCreated { user_id: "u-1".into(), email: "a@example.com".into(), display_name: "Alice".into() }, None),
            event(2, DirectoryEventBody::SpaceCreated { space_id: "space-1".into(), name: "Space 1".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-1")),
            event(3, DirectoryEventBody::MemberUpserted { space_id: "space-1".into(), user_id: "u-1".into(), role: DirectorySpaceRole::Author }, Some("space-1")),
        ];
        let events_json = pack::to_json_string(&events);
        let result = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("fold");
        assert_eq!(result.config_mutations.len(), 1);
        let SpaceIndexConfigMutation::Snapshot { config } = &result.config_mutations[0];
        assert_eq!(config.visibility, "public");
        assert_eq!(config.members.len(), 1);
        assert_eq!(config.members[0].email, "a@example.com");
        assert_eq!(config.members[0].role, "author");
    }

    #[semio_framework_async_macros::async_test]
    async fn folding_events_for_a_different_space_is_a_no_op() {
        let snapshot = view_for("space-1");
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let config_snapshot = SpaceIndexConfig::default();
        let cfg = ConfigView { snapshot: &config_snapshot };
        let events =
            vec![event(1, DirectoryEventBody::SpaceCreated { space_id: "space-2".into(), name: "Other".into(), space_kind: DirectorySpaceKind::Atelier, visibility: DirectorySpaceVisibility::Public, owner_user_id: "u-1".into() }, Some("space-2"))];
        let events_json = pack::to_json_string(&events);
        let result = handle(&FoldDirectoryEvents { events_json }, &doc, &cfg).expect("fold");
        assert!(result.config_mutations.is_empty(), "unrelated-space events never touch this space's config");
        assert!(result.artifact_mutations.is_empty());
    }
}
//#endregion 🧪️Tests
