//! 📇️ Directory read model — pure fold over `DirectoryEvent` (ticket 26/08/16/HUB-SPACES-LIVE-
//! PRESENCE-AND-COLLABORATIVE-STUDIOS, contract C1). `DirectoryReadModel` is the projection the hub
//! (and any client mirror) derives from the event log: `spaces` keyed by id, each carrying its
//! `SpaceView` plus `members`, and `cursor` = the last folded `seq` (idempotent replay guard). A
//! `users` side-table backfills `MemberView.email`/`display_name` from `user.created` — that event
//! never touches `spaces` itself (per the decider laws, it "only feeds member display data"). See
//! `🧬️schema/🦀️component.rs` for the wire types folded over here, and `🟦️component.ts` for the
//! byte-identical TypeScript twin (parity asserted over the golden fixture
//! `🧫️fixtures/📇️directory/🧾️events.json` in both `🧪️Tests` regions).

// 🧭️ Resolves relative to THIS file's own physical directory (📇️directory/), independent of how
// this file itself got mounted into the crate from 📦️glue.rs — see 🔣️taxonomy.json's
// `rustEntryPathRules` header comment on why that holds even under `#[path]` mounting.
#[path = "🧬️schema/🦀️component.rs"]
pub mod schema;

// 🎫️ Lane 1-D (ticket §A ownership: "none" before this wave, ours per `📋️ownership-and-handoffs.md`
// row `🔨️modules/📇️directory/**`): the hub client + native identity mint/restore helper. Both are
// additive sub-modules beside `schema`, consuming `fold`/the wire types below without touching them.
#[path = "🔌️client/🦀️component.rs"]
pub mod client;

#[path = "🪪️identity/🦀️component.rs"]
pub mod identity;

use std::collections::BTreeMap;

pub use schema::{ConnectionView, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryConnectionPhase, DirectoryPresenceActor, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentView, Hlc, InviteView};
pub use schema::{DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, MemberView, SpaceView, UserView};

//#region 🔖️ReadModel
/// 🏠️ One projected space: its `SpaceView` plus the current member roster.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectorySpace {
    pub view: SpaceView,
    pub members: Vec<MemberView>,
}

/// 📇️ The directory's whole projected state, folded from the event log.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DirectoryReadModel {
    pub spaces: BTreeMap<String, DirectorySpace>,
    pub cursor: u64,
    /// 🙋️ Side-table, not part of contract-freeze.md's C1 prose shape — backfills
    /// `MemberView.email`/`display_name` from `user.created` events (see this file's header).
    pub users: BTreeMap<String, UserView>,
}

/// 🔗️ Upserts `user_id`/`role` into `space.members`, joining `email`/`display_name` from
/// `model.users` when known (falls back to empty strings for a member added before their own
/// `user.created` — sequencing hygiene the hub's command handler guarantees never happens live).
// 🚫️async: E1 pure mutation — I/O-free, and its only caller `fold` (above) is itself E1-tagged
// (R9 propagates one hop backward: a helper consumed exclusively by a sync-by-necessity caller
// cannot be async either — there is no suspension point and no legal way to await it here).
fn upsert_member(space: &mut DirectorySpace, email: String, display_name: String, user_id: &str, role: DirectorySpaceRole, updated_at_ms: i64) {
    match space.members.iter_mut().find(|member| member.user_id == user_id) {
        Some(existing) => existing.role = role,
        None => space.members.push(MemberView { user_id: user_id.to_string(), email, display_name, role }),
    }
    space.view.member_count = space.members.len() as u32;
    space.view.updated_at_ms = updated_at_ms;
}

/// 🧮️ Pure fold: `model × event -> model`. Idempotent — an event whose `seq` does not strictly
/// advance `model.cursor` (already-applied or out-of-order-old) is ignored wholesale.
// 🚫️async: E1 pure accessor — the only real caller is `Iterator::fold` below (`FnMut(B, Item) -> B`,
// signature fixed outside this repo); the TS twin (`🟦️component.ts` `export function fold`) is
// already sync too — see R9, R10 residue #1.
pub fn fold(model: DirectoryReadModel, event: &DirectoryEvent) -> DirectoryReadModel {
    let mut next = model;
    if event.seq <= next.cursor {
        return next;
    }
    next.cursor = event.seq;
    match &event.body {
        DirectoryEventBody::UserCreated { user_id, email, display_name } => {
            next.users.insert(user_id.clone(), UserView { id: user_id.clone(), email: email.clone(), display_name: display_name.clone(), created_at_ms: event.recorded_at_ms });
        }
        DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
            next.spaces.insert(
                space_id.clone(),
                DirectorySpace {
                    view: SpaceView {
                        id: space_id.clone(),
                        name: name.clone(),
                        kind: *space_kind,
                        visibility: *visibility,
                        owner_user_id: owner_user_id.clone(),
                        role: None,
                        member_count: 0,
                        document_count: 0,
                        active_connections: 0,
                        created_at_ms: event.recorded_at_ms,
                        updated_at_ms: event.recorded_at_ms,
                    },
                    members: Vec::new(),
                },
            );
        }
        DirectoryEventBody::SpaceRenamed { space_id, name } => {
            if let Some(space) = next.spaces.get_mut(space_id) {
                space.view.name = name.clone();
                space.view.updated_at_ms = event.recorded_at_ms;
            }
        }
        DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
            if let Some(space) = next.spaces.get_mut(space_id) {
                space.view.visibility = *visibility;
                space.view.updated_at_ms = event.recorded_at_ms;
            }
        }
        DirectoryEventBody::SpaceArchived { space_id } => {
            if let Some(space) = next.spaces.get_mut(space_id) {
                space.view.kind = DirectorySpaceKind::Archive;
                space.view.updated_at_ms = event.recorded_at_ms;
            }
        }
        DirectoryEventBody::SpaceDeleted { space_id } => {
            next.spaces.remove(space_id);
        }
        DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
            let (email, display_name) = next.users.get(user_id).map(|u| (u.email.clone(), u.display_name.clone())).unwrap_or_default();
            if let Some(space) = next.spaces.get_mut(space_id) {
                upsert_member(space, email, display_name, user_id, *role, event.recorded_at_ms);
            }
        }
        DirectoryEventBody::MemberRemoved { space_id, user_id } => {
            if let Some(space) = next.spaces.get_mut(space_id) {
                space.members.retain(|member| &member.user_id != user_id);
                space.view.member_count = space.members.len() as u32;
                space.view.updated_at_ms = event.recorded_at_ms;
            }
        }
        DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
            let (email, display_name) = next.users.get(user_id).map(|u| (u.email.clone(), u.display_name.clone())).unwrap_or_default();
            if let Some(space) = next.spaces.get_mut(space_id) {
                upsert_member(space, email, display_name, user_id, *role, event.recorded_at_ms);
            }
        }
    }
    next
}

/// 🔁️ Folds every event in order — `events.iter().fold(model, fold)` spelled out for callers that
/// do not want to import `Iterator::fold` alongside this crate's own `fold`.
pub async fn fold_all(model: DirectoryReadModel, events: &[DirectoryEvent]) -> DirectoryReadModel {
    events.iter().fold(model, fold)
}
//#endregion 🔖️ReadModel

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn fixture_events() -> Vec<DirectoryEvent> {
        #[derive(serde::Deserialize)]
        struct Fixture {
            events: Vec<DirectoryEvent>,
        }
        let raw = include_str!("../../🧫️fixtures/📇️directory/🧾️events.json");
        serde_json::from_str::<Fixture>(raw).expect("fixture decodes").events
    }

    #[semio_framework_async_macros::async_test]
    async fn folds_the_golden_fixture_into_the_expected_projection() {
        let model = fold_all(DirectoryReadModel::default(), &fixture_events().await).await;

        assert_eq!(model.cursor, 16, "cursor tracks the last folded seq");
        assert_eq!(model.spaces.len(), 1, "the deleted atelier leaves only the studio");
        assert!(!model.spaces.contains_key("sp-atelier-amara"), "space.deleted removes the entry entirely");

        let studio = model.spaces.get("sp-studio-fabrication").expect("studio survives");
        assert_eq!(studio.view.name, "Fabrication Studio");
        assert_eq!(studio.view.visibility, DirectorySpaceVisibility::Public);
        assert_eq!(studio.view.kind, DirectorySpaceKind::Archive, "space.archived sets kind archive");
        assert_eq!(studio.view.member_count, 2);

        let mut roles: Vec<(&str, DirectorySpaceRole)> = studio.members.iter().map(|m| (m.user_id.as_str(), m.role)).collect();
        roles.sort();
        assert_eq!(roles, vec![("u-amara", DirectorySpaceRole::Spectator), ("u-devon", DirectorySpaceRole::Spectator)], "member.removed dropped u-noor; the archive law demoted every remaining author to spectator");

        let devon = studio.members.iter().find(|m| m.user_id == "u-devon").expect("devon is a member");
        assert_eq!(devon.email, "devon@semio.dev", "member display data is backfilled from user.created");
    }

    #[semio_framework_async_macros::async_test]
    async fn folding_is_idempotent_on_replay() {
        let events = fixture_events().await;
        let once = fold_all(DirectoryReadModel::default(), &events).await;
        let twice = fold_all(once.clone(), &events);
        assert_eq!(once, twice.await, "re-folding the same events changes nothing (seq <= cursor is ignored)");
    }
}
//#endregion 🧪️Tests
