//! 🧭️ The directory command decider: one command in, events and a result out.

use semio_hub_directory_contract::{ARTIFACT_CHECKPOINT_LINEAGE_MAX, ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES, AUTH_TEXT_MAX_BYTES, HubClock, NewDirectoryEvent, actor_user_id, role_from_wire, validate_bounded_auth_text};
use directory::os_directory::{ArtifactCheckpoint, ArtifactRetention, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, DocumentScope};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_directory_contract::model::{SpaceRecord, SpaceRole};
use directory::os_directory::io::binary::descriptor_digest::descriptor_digest_v1;
use semio_hub_directory::{frontier_strictly_advances, published_artifact_checkpoint, validate_checkpoint_shape, validate_document_descriptor, validate_retention_shape};
use directory::os_identity::time_ordered_id;

//#region 🔖️Decider
/// 🎁️ The extra payload a `decide`d command can carry alongside its events — today only
/// `create-invite` produces one (contract C1's `-> { inviteToken }`); the secret token itself is
/// never an event field (invites are not event-sourced, only their `invite.redeemed` outcome is),
/// so it has to travel back to the caller some other way than the event stream.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandResult {
    pub invite_token: Option<String>,
}

/// 🧾️ What `decide` computed for one `DirectoryCommand`: the events to persist (empty for the two
/// not-event-sourced invite commands, see `decide`'s own doc) plus an optional `CommandResult`.
#[derive(Clone, Debug)]
pub struct Decision {
    pub events: Vec<NewDirectoryEvent>,
    pub result: Option<CommandResult>,
}

pub fn new_event(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> NewDirectoryEvent {
    NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id, user_id, body }
}

fn single(clock: &mut HubClock, actor: &DirectoryActor, space_id: Option<String>, user_id: Option<String>, body: DirectoryEventBody) -> Decision {
    Decision { events: vec![new_event(clock, actor, space_id, user_id, body)], result: None }
}

pub(super) async fn require_space(dir: &HubDirectories, space_id: &str) -> DirectoryResult<SpaceRecord> {
    dir.get_space(space_id).await?.ok_or_else(|| DirectoryError::NotFound(format!("space '{space_id}' not found")))
}

/// 🧠️ Decides what a `DirectoryCommand` means: reads projections through `dir` (`get_space`,
/// `list_members`, `get_user_by_email`) and returns the events that would record it — it never
/// writes to `dir`, with one deliberate exception: `create-invite`/`revoke-invite` are NOT
/// event-sourced (contract's decider laws — only an invite's `invite.redeemed` outcome is an
/// event), so for those two variants this function performs the (non-event) write itself, under
/// the same write-lock serialization `DirectoryService::execute` already holds while calling it.
///
/// Authorization is **not** this function's job — `bin.rs` (lane 1-B) checks whether `actor` may
/// issue `command` against the named space *before* calling this; `decide` trusts `actor` as given
/// and only enforces the contract's structural laws:
/// - `create-space`/`upsert-member` derive the atelier ⇒ ≤1-author law.
/// - `archive-space` emits one intrinsically demoting event; backend transactions enforce
///   archive ⇒ nobody-writes even when an independent writer invalidates this decision.
/// - `remove-member` naming the space's own owner ⇒ `DirectoryError::Conflict` (never removable).
/// - `delete-space` removes every membership (owner included) before the space: raw directory events are member-only, so
///   `space.deleted` alone reaches no reader, while each `member.removed` owes its member's global socket an
///   `access-changed: revoked` frame and that reader re-reads its directory from the origin without the space.
/// - Any command naming a missing/deleted space ⇒ `DirectoryError::NotFound`.
/// - `upsert-member` with an email that has no `UserRecord` yet emits `user.created` first, using
///   a freshly minted user id the following `member.upserted` also uses.
pub async fn decide(dir: &HubDirectories, actor: &DirectoryActor, command: DirectoryCommand, clock: &mut HubClock) -> DirectoryResult<Decision> {
    match command {
        DirectoryCommand::CreateSpace { name, space_kind, visibility } => {
            let space_id = time_ordered_id();
            decide_create_space(actor, space_id, name, space_kind, visibility, clock)
        }
        DirectoryCommand::RenameSpace { space_id, name } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceRenamed { space_id, name }))
        }
        DirectoryCommand::SetVisibility { space_id, visibility } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility }))
        }
        DirectoryCommand::ArchiveSpace { space_id } => {
            require_space(dir, &space_id).await?;
            Ok(single(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceArchived { space_id }))
        }
        DirectoryCommand::DeleteSpace { space_id } => {
            require_space(dir, &space_id).await?;
            let mut events: Vec<NewDirectoryEvent> = dir.list_members(&space_id).await?.into_iter().map(|(user, _)| new_event(clock, actor, Some(space_id.clone()), Some(user.id.clone()), DirectoryEventBody::MemberRemoved { space_id: space_id.clone(), user_id: user.id })).collect();
            events.push(new_event(clock, actor, Some(space_id.clone()), None, DirectoryEventBody::SpaceDeleted { space_id }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::UpsertMember { space_id, email, role } => {
            let space = require_space(dir, &space_id).await?;
            let mut events = Vec::new();
            let user_id = match dir.get_user_by_email(&email).await? {
                Some(existing) => existing.id,
                None => {
                    let user_id = time_ordered_id();
                    let display_name = email.split('@').next().unwrap_or(&email).to_string();
                    events.push(new_event(clock, actor, None, Some(user_id.clone()), DirectoryEventBody::UserCreated { user_id: user_id.clone(), email: email.clone(), display_name }));
                    user_id
                }
            };
            if role == DirectorySpaceRole::Author {
                if space.kind == "archive" {
                    return Err(DirectoryError::Conflict(format!("space '{space_id}' is an archive; no author memberships are allowed")));
                }
                if space.kind == "atelier" {
                    let has_other_author = dir.list_members(&space_id).await?.into_iter().any(|(user, existing_role)| existing_role == SpaceRole::Author && user.id != user_id);
                    if has_other_author {
                        return Err(DirectoryError::Conflict(format!("space '{space_id}' is an atelier; it already has a distinct author")));
                    }
                }
            }
            events.push(new_event(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id, role }));
            Ok(Decision { events, result: None })
        }
        DirectoryCommand::RemoveMember { space_id, user_id } => {
            let space = require_space(dir, &space_id).await?;
            if space.owner_user_id == user_id {
                return Err(DirectoryError::Conflict(format!("space '{space_id}' owner membership cannot be removed")));
            }
            Ok(single(clock, actor, Some(space_id.clone()), Some(user_id.clone()), DirectoryEventBody::MemberRemoved { space_id, user_id }))
        }
        DirectoryCommand::CreateInvite { space_id, role, ttl_secs } => {
            require_space(dir, &space_id).await?;
            let ttl_secs = i64::try_from(ttl_secs).map_err(|_| DirectoryError::Conflict("invite ttl exceeds the signed storage boundary".into()))?;
            let actor_user_id = Some(actor_user_id(actor)?);
            let correlation_id = time_ordered_id();
            let issued = dir.issue_invite_as(&space_id, role_from_wire(role), ttl_secs, actor_user_id, &correlation_id).await?;
            Ok(Decision { events: Vec::new(), result: Some(CommandResult { invite_token: Some(issued.capability.expose_once()) }) })
        }
        DirectoryCommand::RevokeInvite { space_id, invite_id } => {
            require_space(dir, &space_id).await?;
            let actor_user_id = Some(actor_user_id(actor)?);
            let correlation_id = time_ordered_id();
            dir.revoke_invite_as(&space_id, &invite_id, "directory-command", actor_user_id, &correlation_id).await?;
            Ok(Decision { events: Vec::new(), result: None })
        }
        DirectoryCommand::RecordUserPreference { schema, mutation } => {
            if !::directory::os_directory::io::text::valid_user_preference_record_v1(&schema, &mutation) {
                return Err(DirectoryError::Conflict("user preference record is invalid".into()));
            }
            let user_id = actor_user_id(actor)?.to_string();
            Ok(single(clock, actor, None, Some(user_id.clone()), DirectoryEventBody::UserPreferenceRecorded { user_id, schema, mutation }))
        }
        DirectoryCommand::AnnounceDocument { descriptor } => {
            validate_document_descriptor(&descriptor)?;
            require_space(dir, &descriptor.space_id).await?;
            let scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
            match dir.get_document_descriptor(&scope).await? {
                Some(existing) if existing == *descriptor => Ok(Decision { events: Vec::new(), result: None }),
                Some(_) => Err(DirectoryError::Conflict(format!("document descriptor for '{}/{}' is immutable", descriptor.space_id, descriptor.document_id))),
                None => Ok(single(clock, actor, Some(descriptor.space_id.clone()), None, DirectoryEventBody::DocumentAnnounced { descriptor: *descriptor })),
            }
        }
    }
}

pub(super) fn decide_create_space(actor: &DirectoryActor, space_id: String, name: String, space_kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility, clock: &mut HubClock) -> DirectoryResult<Decision> {
    validate_bounded_auth_text(&space_id, "space id", AUTH_TEXT_MAX_BYTES)?;
    let owner_user_id = actor_user_id(actor)?.to_string();
    let owner_role = if space_kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
    let events = vec![
        new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name, space_kind, visibility, owner_user_id: owner_user_id.clone() }),
        new_event(clock, actor, Some(space_id.clone()), Some(owner_user_id.clone()), DirectoryEventBody::MemberUpserted { space_id, user_id: owner_user_id, role: owner_role }),
    ];
    Ok(Decision { events, result: None })
}

/// 🛡️ Server-only retention policy intent; it is deliberately absent from `DirectoryCommand`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactDirectoryCommand {
    AdvanceRetention { retention: ArtifactRetention },
}

pub(super) async fn decide_artifact_authority(dir: &HubDirectories, actor: &DirectoryActor, command: ArtifactDirectoryCommand, clock: &mut HubClock) -> DirectoryResult<Decision> {
    if !matches!(actor.kind, DirectoryActorKind::System | DirectoryActorKind::Admin) {
        return Err(DirectoryError::Unauthorized);
    }
    match command {
        ArtifactDirectoryCommand::AdvanceRetention { retention } => {
            validate_retention_shape(&retention)?;
            let active = dir.get_active_artifact_checkpoint(&retention.scope).await?.ok_or_else(|| DirectoryError::NotFound("active artifact checkpoint".into()))?;
            if retention.checkpoint_lineage_head != active.checkpoint_id {
                return Err(DirectoryError::Conflict("artifact retention lineage head is not the active checkpoint".into()));
            }
            let retained = dir.get_artifact_checkpoint(&retention.scope, retention.retained_checkpoint_id).await?.ok_or_else(|| DirectoryError::NotFound("retained artifact checkpoint".into()))?;
            if retained.baseline_frontier != retention.retained_floor {
                return Err(DirectoryError::Conflict("artifact retention floor is not the retained checkpoint baseline".into()));
            }
            let lineage = dir.list_artifact_checkpoint_lineage(&retention.scope, ARTIFACT_CHECKPOINT_LINEAGE_MAX as usize).await?;
            let retained_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == retention.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("retained artifact checkpoint is outside the active lineage".into()))?;
            let active_index = lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == active.checkpoint_id).ok_or_else(|| DirectoryError::Conflict("active artifact checkpoint is outside its lineage".into()))?;
            if retained_index > active_index {
                return Err(DirectoryError::Conflict("artifact retention floor is ahead of the active baseline".into()));
            }
            if let Some(previous) = dir.get_artifact_retention(&retention.scope).await? {
                if previous == retention {
                    return Ok(Decision { events: Vec::new(), result: None });
                }
                let previous_index =
                    lineage.iter().position(|checkpoint| checkpoint.checkpoint_id == previous.retained_checkpoint_id).ok_or_else(|| DirectoryError::Conflict("existing artifact retention floor is outside the active lineage".into()))?;
                if retained_index < previous_index {
                    return Err(DirectoryError::Conflict("artifact retention floor cannot move backward".into()));
                }
            }
            Ok(single(clock, actor, Some(retention.scope.space_id.clone()), None, DirectoryEventBody::ArtifactRetentionAdvanced { retention }))
        }
    }
}

pub(super) async fn decide_verified_checkpoint(dir: &HubDirectories, actor: &DirectoryActor, checkpoint: &ArtifactCheckpoint, clock: &mut HubClock) -> DirectoryResult<Decision> {
    if actor.kind != DirectoryActorKind::System {
        return Err(DirectoryError::Unauthorized);
    }
    let published = published_artifact_checkpoint(checkpoint);
    validate_checkpoint_shape(&published)?;
    if checkpoint.pack.storage_key.trim().is_empty() || checkpoint.spr.storage_key.trim().is_empty() || checkpoint.pack.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES || checkpoint.spr.storage_key.len() > ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES {
        return Err(DirectoryError::Conflict(format!("verified artifact checkpoint locator must contain 1..={ARTIFACT_PRIVATE_LOCATOR_MAX_BYTES} UTF-8 bytes")));
    }
    let descriptor = dir.get_document_descriptor(&published.scope).await?.ok_or_else(|| DirectoryError::NotFound(format!("document descriptor for '{}/{}'", published.scope.space_id, published.scope.document_id)))?;
    let digest = descriptor_digest_v1(&descriptor).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    if digest != published.descriptor_digest_v1 {
        return Err(DirectoryError::Conflict("artifact checkpoint descriptor digest differs from the durable descriptor".into()));
    }
    if let Some(existing) = dir.get_verified_artifact_checkpoint(&published.scope, published.checkpoint_id).await? {
        return if &existing == checkpoint { Ok(Decision { events: Vec::new(), result: None }) } else { Err(DirectoryError::Conflict("artifact checkpoint id already names different public or private metadata".into())) };
    }
    if dir.get_artifact_checkpoint(&published.scope, published.checkpoint_id).await?.is_some() {
        return Err(DirectoryError::Conflict("artifact checkpoint public projection has no matching private authority record".into()));
    }
    if dir.artifact_checkpoint_count(&published.scope).await? >= ARTIFACT_CHECKPOINT_LINEAGE_MAX {
        return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage exceeds fixed maximum {ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
    }
    match dir.get_active_artifact_checkpoint(&published.scope).await? {
        None => return Err(DirectoryError::Conflict("ordinary artifact publication requires a committed genesis parent".into())),
        Some(ref current) if published.parent_checkpoint_id != Some(current.checkpoint_id) => return Err(DirectoryError::Conflict("artifact checkpoint parent is not the active lineage head".into())),
        Some(ref current) if !frontier_strictly_advances(&current.baseline_frontier, &published.baseline_frontier) => {
            return Err(DirectoryError::Conflict("artifact checkpoint frontier does not strictly advance the active baseline".into()));
        }
        _ => {}
    }
    Ok(single(clock, actor, Some(published.scope.space_id.clone()), None, DirectoryEventBody::ArtifactCheckpointPublished { checkpoint: published }))
}
//#endregion 🔖️Decider
