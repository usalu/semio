//! 🪝️ `shell-window-actions` staging of `identity-and-types`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

pub(crate) fn optional_dsl_value_as_json(value: Option<DslValue>) -> Option<Value> {
    value.map(|entry| dsl_value_as_json(&entry))
}

//#region ShellTypes
//#region 🔖️NativeSyncChannel
pub(crate) fn route_document_backbone_effects(actor_uri: &str, cmd_tx: &ArtifactMailboxSender, effects: Vec<semio_framework::kernel::Effect>) -> Result<Vec<semio_framework::kernel::Effect>, String> {
    let mut remaining = Vec::new();
    for effect in effects {
        match effect {
            semio_framework::kernel::Effect::SendMessage { target: semio_framework::kernel::MessageEndpoint::Backbone { uri }, payload } => {
                if uri != actor_uri {
                    return Err("plugin document-backbone effect targeted a different actor owner".into());
                }
                match store_sync::os_store::backbone::decode_hot_backbone_message_exact(&payload).map_err(|error| error.to_string())? {
                    store_sync::os_store::backbone::BackboneMessage::Mutations { .. } => cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: payload }).map_err(|_| "document-backbone mailbox refused the canonical mutation message".to_string())?,
                    store_sync::os_store::backbone::BackboneMessage::Ack { .. } => {}
                    store_sync::os_store::backbone::BackboneMessage::Genesis { .. } => return Err("hot document-backbone egress cannot publish genesis".into()),
                    store_sync::os_store::backbone::BackboneMessage::Member { .. } => return Err("a composed member requires its exact member transport lane".into()),
                    store_sync::os_store::backbone::BackboneMessage::Retract { .. } => return Err("a retraction flows from the actor to its store, never back".into()),
                }
            }
            other => remaining.push(other),
        }
    }
    Ok(remaining)
}

//#endregion 🔖️NativeSyncChannel
//#endregion ShellTypes

pub(crate) fn scope_action_to_window(action: &mut ActionDescriptor, window_id: &str) {
    let mut args = match action.args.take() {
        Some(semio_framework_value::DslValue::Object(entries)) => entries,
        _ => Vec::new(),
    };
    args.retain(|(key, _)| key != "windowId");
    args.push(("windowId".into(), semio_framework_value::DslValue::String(window_id.into())));
    action.args = Some(semio_framework_value::DslValue::Object(args));
}

//#region 🔖️IdentityPure
/// 🏛️ The persistence transports the document host this shell build links actually serves. Both
/// builds link the kernel's `ArtifactHost`; its native actor owns the folder event log and the hub
/// WebSocket, while its browser actor has no filesystem and dials its hub through the page's socket door.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ShellDocumentTransports {
    pub(crate) folder: bool,
    pub(crate) hub: bool,
}

/// 📇️ ticket §C6 — maps an `os.directory.<verb>` action id + its relayed JSON args onto a
/// `DirectoryCommand`, mirroring the React shell's `directoryCommandFromAction` verb-for-verb
/// (`📓️w2-c-report.md`) — same action ids, same field names, same `share-link` → `create-invite`
/// sugar (contract §C1 has no directory-schema command kind of its own for it). `None` for an
/// unrecognized verb, defensive against a foreign/future caller.
pub(crate) fn directory_command_from_action(action_id: &str, args: Option<&Value>) -> Option<DirectoryCommand> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| value.as_str()).unwrap_or("").to_string();
    let str_field_or = |key: &str, default: &str| args.and_then(|value| value.get(key)).and_then(|value| value.as_str()).unwrap_or(default).to_string();
    let space_kind = |raw: &str| match raw {
        "studio" => DirectorySpaceKind::Studio,
        "archive" => DirectorySpaceKind::Archive,
        _ => DirectorySpaceKind::Atelier,
    };
    let visibility = |raw: &str| if raw == "public" { DirectorySpaceVisibility::Public } else { DirectorySpaceVisibility::Private };
    let role = |raw: &str| if raw == "author" { DirectorySpaceRole::Author } else { DirectorySpaceRole::Spectator };
    match action_id {
        "os.directory.create-space" => Some(DirectoryCommand::CreateSpace { name: str_field("name"), space_kind: space_kind(&str_field_or("spaceKind", "atelier")), visibility: visibility(&str_field_or("visibility", "private")) }),
        "os.directory.delete-space" => Some(DirectoryCommand::DeleteSpace { space_id: str_field("spaceId") }),
        "os.directory.rename-space" => Some(DirectoryCommand::RenameSpace { space_id: str_field("spaceId"), name: str_field("name") }),
        "os.directory.set-visibility" => Some(DirectoryCommand::SetVisibility { space_id: str_field("spaceId"), visibility: visibility(&str_field_or("visibility", "private")) }),
        "os.directory.upsert-member" => Some(DirectoryCommand::UpsertMember { space_id: str_field("spaceId"), email: str_field("email"), role: role(&str_field_or("role", "spectator")) }),
        "os.directory.remove-member" => Some(DirectoryCommand::RemoveMember { space_id: str_field("spaceId"), user_id: str_field("userId") }),
        "os.directory.share-link" => {
            Some(DirectoryCommand::CreateInvite { space_id: str_field("spaceId"), role: role(&str_field_or("role", "spectator")), ttl_secs: args.and_then(|value| value.get("ttlSecs")).and_then(|value| value.as_u64()).unwrap_or(3600) })
        }
        _ => None,
    }
}

pub(crate) fn open_artifact_relay_target(action_id: &str, args: Option<&Value>) -> Result<OpenArtifactRelayTarget, &'static str> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let args = args.and_then(Value::as_object).ok_or("opening.invalid-args")?;
    let text = |field: &str| args.get(field).and_then(Value::as_str).filter(|value| !value.trim().is_empty()).map(str::to_string);
    let raw_artifact_ref = text("artifactRef").ok_or("opening.invalid-artifact-ref")?;
    let (dialect, surface_role) = if raw_artifact_ref.contains('#') {
        let (dialect, role) = semio_framework::parse_surface_app_id(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?;
        (dialect, Some(role))
    } else {
        (semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(&raw_artifact_ref).map_err(|_| "opening.invalid-artifact-ref")?, None)
    };
    let wire_role = match args.get("role") {
        None => None,
        Some(Value::Number(number)) if number.as_u64() == Some(0) => Some(semio_framework::AppRole::Viewer),
        Some(Value::Number(number)) if number.as_u64() == Some(1) => Some(semio_framework::AppRole::Editor),
        Some(Value::String(role)) if role == "viewer" => Some(semio_framework::AppRole::Viewer),
        Some(Value::String(role)) if role == "editor" => Some(semio_framework::AppRole::Editor),
        _ => return Err("opening.invalid-role"),
    };
    if surface_role.is_some() && wire_role.is_some() && surface_role != wire_role {
        return Err("opening.role-mismatch");
    }
    let role = wire_role.or(surface_role).unwrap_or(semio_framework::AppRole::Editor);
    let plugin_id = text("pluginId");
    let app_id = text("appId");
    if plugin_id.is_some() != app_id.is_some() {
        return Err("opening.partial-app-ref");
    }
    if action_id == "os.open-artifact-with" && plugin_id.is_none() {
        return Err("opening.explicit-app-required");
    }
    if let Some(app_id) = &app_id {
        let (app_dialect, app_role) = semio_framework::parse_surface_app_id(app_id).map_err(|_| "opening.app-mismatch")?;
        if app_dialect != dialect || app_role != role {
            return Err("opening.app-mismatch");
        }
    }
    let document_id = text("documentId");
    let schema = text("schema");
    if document_id.is_some() != schema.is_some() {
        return Err("opening.partial-document-ref");
    }
    Ok(OpenArtifactRelayTarget { artifact_ref: dialect.to_coordinate(), dialect, role, plugin_id, app_id, document_id, space_id: text("spaceId"), schema })
}

/// 🪐️ ticket §C4/§6 — the space's own artifact-index document: its id is always the literal `"index"`
/// (one per hub space). Which plugin app edits it, and the document schema it is opened with, are named
/// by that plugin's own `os.space-index` topic contribution ([`program_space_index`]), never by this shell.
pub(crate) const SPACE_INDEX_DOCUMENT_ID: &str = "index";

/// 🪐️ The app of `role` that serves the space index `program` names.
pub(crate) fn find_space_index_app(program: &ProgramBridgeEntry, role: semio_framework::manifest::AppRole) -> Option<&AppDefinition> {
    let index = program_space_index(program)?;
    find_dialect_app(program, &index.dialect, role)
}

/// 🌐️ The three parts a `remote://host:port/space/document` sync-card uri names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RemoteBackboneUri {
    pub(crate) host_port: String,
    pub(crate) space_id: String,
    pub(crate) document_id: String,
}

pub(crate) fn bind_wgpu_document_socket_surface(host: &ArtifactHost, document_id: &str, artifact_schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {
    let mut hub_spaces = bindings.iter().filter_map(|binding| match binding {
        PersistenceBinding::Hub { space_id, .. } => Some(space_id.as_str()),
        PersistenceBinding::Folder { .. } => None,
    });
    let Some(space_id) = hub_spaces.next() else {
        return Ok(());
    };
    if hub_spaces.any(|candidate| candidate != space_id) {
        return Err("document open cannot span hub spaces".into());
    }
    if app.io.artifact_schema != artifact_schema {
        return Err("document open local artifact schema does not match the selected app".into());
    }
    let surface_id = wgpu_document_socket_surface(program, app, window_kind_id)?;
    if bindings.iter().any(|binding| matches!(binding, PersistenceBinding::Hub { surface: Some(selected), .. } if selected != &surface_id)) {
        return Err("document open local surface does not match the hub binding".into());
    }
    let _ = (host, space_id, document_id);
    Ok(())
}
//#endregion 🔖️IdentityPure

//#region 🔖️IdentityPure
//#region 📣️ReplayRefusal
/// 🌱️ The one kind an `os.create-space-artifact` replay may create — ShellHost's `spaceArtifactCreationRequestFromAction`
/// over this shell's creation catalog: exactly `{kindChoice, name}`, the choice a canonical (decode ∘ encode)
/// `ArtifactKindChoice` equal to one kind the ready catalog of `space_id` offers. Answers `(kindId, name)`.
pub(crate) async fn space_artifact_creation_replay_choice(args: Option<&Value>, catalog: Option<&SpaceArtifactCreationCatalogV1>, space_id: &str) -> Result<(String, String), ReplayRefusalReasonV1> {
    let invalid = ReplayRefusalReasonV1::InvalidRequest;
    let args = args.and_then(Value::as_object).filter(|args| args.len() == 2).ok_or(invalid)?;
    let (Some(choice), Some(name)) = (args.get("kindChoice").and_then(Value::as_str), args.get("name").and_then(Value::as_str)) else { return Err(invalid) };
    let decoded = semio_framework::manifest::decode_artifact_kind_choice(choice).await.map_err(|_| invalid)?;
    if semio_framework::manifest::encode_artifact_kind_choice(&decoded).await != choice {
        return Err(invalid);
    }
    let catalog = catalog.filter(|catalog| catalog.space_id == space_id).ok_or(ReplayRefusalReasonV1::RouterNotReady)?;
    for kind in &catalog.kinds {
        let offered = semio_framework::manifest::ArtifactKindChoice {
            kind_id: kind.kind_id.clone(),
            schema: kind.schema.clone(),
            dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.dialect.artifact_kind.clone(), standard: kind.dialect.standard.clone(), subset: kind.dialect.subset.clone() },
            label: LocalizedLabel::native(&kind.label.en, &kind.label.de),
        };
        if semio_framework::manifest::encode_artifact_kind_choice(&offered).await == choice {
            return Ok((kind.kind_id.clone(), name.to_string()));
        }
    }
    Err(invalid)
}
//#endregion 📣️ReplayRefusal
//#endregion 🔖️IdentityPure

//#region 🔖️CheckInPure
/// 📌️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END §C5 — auto check-in policy
/// constants, byte-identical to the React shell's `AUTO_CHECKIN_IDLE_MS`/`AUTO_CHECKIN_EDIT_THRESHOLD`
/// (`ShellHelpers/🟦️.tsx`).
pub(crate) const AUTO_CHECKIN_IDLE_MS: i64 = 20_000;

pub(crate) const AUTO_CHECKIN_EDIT_THRESHOLD: u32 = 200;

/// 🧾️ The folded rows oldest-first — by `seq`, the command-log order every walk over the ledger reads.
pub(crate) fn history_rows_oldest_first(entries: &BTreeMap<String, semio_framework::kernel::HistoryEntry>) -> Vec<&semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<&semio_framework::kernel::HistoryEntry> = entries.values().collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 👁️✏️ ticket §C5 item 5 — "viewers never checkpoint", the wgpu twin of the React shell's
/// `canCheckIn` (`ShellHelpers/🟦️.tsx`): the one predicate gating both the `#s-checkin`
/// affordance's presence and the auto check-in poll's arming.
pub(crate) fn can_check_in(role: semio_framework::manifest::AppRole) -> bool {
    role == semio_framework::manifest::AppRole::Editor
}

/// 📌️ ticket §C5 item 2 — pure decision for the per-frame auto check-in poll. This shell has no timer
/// wheel (see `render_sync_status_and_checkin`'s doc note on `Effect::DispatchAction`'s `delay_ms`
/// collapsing to "next tick"), so `AutoCheckinScheduler`'s `setTimeout`-based debounce
/// (`ShellHelpers/🟦️.tsx`) is reproduced as a poll instead: fires once uncommitted edits
/// reach `threshold` (never waiting out the idle window once crossed), or once `idle_ms` have elapsed
/// since the last uncommitted-count change — and never again while `pending` is already set (mirrors
/// `AutoCheckinScheduler::notify`'s own storm guard: a fire that's already pending never re-fires
/// until the caller observes the count return to 0 and clears `pending`).
pub(crate) fn auto_checkin_should_fire(uncommitted_count: u32, pending: bool, last_edit_at_ms: Option<i64>, now_ms: i64, idle_ms: i64, threshold: u32) -> bool {
    if uncommitted_count == 0 || pending {
        return false;
    }
    if uncommitted_count >= threshold {
        return true;
    }
    match last_edit_at_ms {
        Some(last) => now_ms.saturating_sub(last) >= idle_ms,
        None => false,
    }
}

/// 📌️ ticket §C5 item 6 — the pure decision `observe_invocation_history` uses to fire `TouchArtifact`:
/// a checkpoint id change only counts when THIS shell itself dispatched the checkpoint that produced
/// it (`checkpoint_dispatched`) — a checkpoint id present from the very first snapshot (nothing
/// "landed", the session just mounted with a pre-existing one) or one made by a REMOTE peer must never
/// trigger a redundant `TouchArtifact` of our own.
pub(crate) fn checkpoint_landed(previous_checkpoint_id: Option<&str>, next_checkpoint_id: Option<&str>, checkpoint_dispatched: bool) -> bool {
    checkpoint_dispatched && next_checkpoint_id.is_some() && next_checkpoint_id != previous_checkpoint_id
}
//#endregion 🔖️CheckInPure

//#region ShellTypes
pub(crate) const SHELL_DISPATCH_ORIGIN_GESTURE: &str = "gesture";

pub(crate) fn custom_ui_driver_id(label: &str) -> Option<String> {
    let mut slug = String::new();
    let mut separator = false;
    for character in label.trim().to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character);
            separator = false;
        } else if !slug.is_empty() {
            separator = true;
        }
    }
    (!slug.is_empty()).then(|| format!("custom.{slug}"))
}
//#endregion ShellTypes

//#region ShellTypes
//#region 🔖️NativeSyncChannel
pub(crate) fn shell_sync_owner_matches(active: &ShellSyncOwner, completion: &ShellSyncOwner) -> bool {
    active == completion
}
//#endregion 🔖️NativeSyncChannel
//#endregion ShellTypes
