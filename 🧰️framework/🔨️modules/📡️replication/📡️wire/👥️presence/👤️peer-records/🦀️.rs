//! 👤️ Presence peer records: the peer, the typing excerpt, the principal kind, the tool run, the history edit and the value conversions of the peer. See [`super`].

use super::*;

//#region 🔖️Presence
// 🌱️ `presence_pack_serde` (the former `#[serde(with = "…")]` base64 shim for `PresencePeer.
// presence_pack`) is deleted: `PresencePeer::to_value`/`from_value` above inline the identical
// base64-string encoding directly, and nothing else referenced this module.

/// 📡️ Presence roster entry broadcast to every peer connected to a document.
///
/// `presence_pack` carries the app's typed `ArtifactApp::Presence` encoded through `ArtifactPack`.
/// When serialised for `ViewModel.presence_peers_json`, that pack is base64-encoded under the
/// camelCase key `presencePack` (this layer has no app-specific `ArtifactPack` decoder, so the
/// renderer JSON contract keeps the opaque pack rather than a decoded `presence` object).
#[derive(Clone, Debug, PartialEq)]
pub struct PresencePeer {
    pub actor: String,
    pub connected_at_ms: i64,
    pub label: Option<String>,
    /// 👥️ App-typed presence encoded as `ArtifactPack` bytes (flag bit 1 on the wire, APP scope).
    pub presence_pack: Option<Vec<u8>>,
    /// 🪪️ Authenticated hub user id, when this peer connected with an `AuthSession` rather than an anonymous share token.
    pub user_id: Option<String>,
    /// 🎚️ The peer's resolved studio role (`"owner"`/`"member"`/`"viewer"`), present alongside `user_id`.
    pub role: Option<String>,
    /// 👻️ Serialized preview of an in-flight drag (opaque JSON, schema owned by the dragging app, APP scope).
    pub drag_ghost_json: Option<String>,
    /// 🕹️ This peer's live selection+hover roster (ARTIFACT scope), mirrored from local
    /// `InteractionState` — see `assemble_presence_interaction` below. `None` for peers on apps that
    /// declare no interaction domains.
    pub interaction: Option<PresenceInteraction>,
    /// 🎨️ Hub-assigned palette index, normalized at authenticated presence ingress.
    pub color: Option<u8>,
    /// 🪟️ Canonical plan-bound surface id, normalized by the Hub.
    pub surface: Option<String>,
    /// 🪟️ Every open window/surface's live camera + in-view pointer (ARTIFACT scope), matched
    /// by `space`. Empty when the peer has no open windows for this document.
    pub views: Vec<PresenceWindowView>,
    /// 🖱️ Live `data-ui-path` hover/focus/press state (APP scope).
    pub ui: Option<PresenceUi>,
    /// ⏯️ Summary of this peer's non-terminal or just-settled tool run (ARTIFACT scope), never provisional geometry.
    pub tool_run: Option<PresenceToolRun>,
    /// 🤖️ What kind of principal this peer is. `None` means the peer never declared one and a
    /// reader must treat it as [`PresencePrincipalKind::Human`] — the pre-agent wire shape. Like
    /// `user_id`/`role`/`color` this is an *admitted* field: the hub overwrites whatever a client
    /// sends with the kind it authenticated, so a human session can never claim to be an agent and
    /// an agent session can never hide behind a human.
    pub principal_kind: Option<PresencePrincipalKind>,
    /// 🛠️ Active editor tool/utility id (ARTIFACT scope). Distinct from `tool_run`.
    pub active_tool: Option<String>,
    /// ⏪️ Summary of this peer's open history edit (ARTIFACT scope): who edits history, on which mutation, at which stage.
    pub history_edit: Option<PresenceHistoryEdit>,
    /// ⌨️ This peer's pending typing runs, one per text window (ARTIFACT scope): the ephemeral shared preview of text it typed
    /// that has not committed yet — never history (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
    pub typing: Vec<PresenceTyping>,
}

/// ⌨️ Ephemeral shared preview of one peer's pending typing run in one text window: what the run deleted and typed so far
/// (excerpts of at most [`PRESENCE_TYPING_EXCERPT_BYTES`] bytes each, cut at a scalar boundary). Peers render it at the
/// author's caret; the run itself lands as ONE edit when it commits, and the preview disappears.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceTyping {
    pub window_id: String,
    pub deleted: String,
    pub insert: String,
}

/// 📏️ The most bytes a typing preview carries per excerpt.
pub const PRESENCE_TYPING_EXCERPT_BYTES: usize = 256;

impl PresenceTyping {
    /// ✂️ `text` cut to [`PRESENCE_TYPING_EXCERPT_BYTES`] at a scalar boundary.
    pub fn excerpt(text: &str) -> String {
        let end = text.char_indices().map(|(index, scalar)| index + scalar.len_utf8()).take_while(|end| *end <= PRESENCE_TYPING_EXCERPT_BYTES).last().unwrap_or(0);
        text[..end].to_string()
    }
}

impl semio_framework_value::ToValue for PresenceTyping {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("windowId".to_string(), semio_framework_value::ToValue::to_value(&self.window_id)),
            ("deleted".to_string(), semio_framework_value::ToValue::to_value(&self.deleted)),
            ("insert".to_string(), semio_framework_value::ToValue::to_value(&self.insert)),
        ])
    }
}

impl semio_framework_value::FromValue for PresenceTyping {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceTyping, found {value:?}")));
        };
        let (mut window_id, mut deleted, mut insert) = (None, None, None);
        for (key, entry) in fields {
            match key.as_str() {
                "windowId" => window_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("windowId"))?),
                "deleted" => deleted = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("deleted"))?),
                "insert" => insert = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("insert"))?),
                _ => {}
            }
        }
        Ok(PresenceTyping {
            window_id: window_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceTyping missing windowId"))?,
            deleted: deleted.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceTyping missing deleted"))?,
            insert: insert.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceTyping missing insert"))?,
        })
    }
}

/// 🤖️ Which kind of principal holds a presence slot. An `Agent` peer is an AI agent acting
/// under a credential a human delegated to it (`hub.auth`'s `AgentDelegationRecord`); it is a
/// principal in its own right, never the delegating human, so a roster shows it as its own row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresencePrincipalKind {
    Human,
    Agent,
}

impl PresencePrincipalKind {
    pub const ALL: [PresencePrincipalKind; 2] = [Self::Human, Self::Agent];

    /// 🔤️ The camelCase wire spelling, shared by the binary tag's ordinal and the JSON value.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Agent => "agent",
        }
    }

    /// 🔡️ Inverse of [`Self::wire_name`].
    pub fn from_wire_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.wire_name() == name)
    }
}

/// ⏳️ Lifecycle state of a peer's tool run, spelled like `ToolRunState` in the tool run contract §2.2.
///
/// Duplicated rather than imported: `semio-framework-tool-run` layers above this replication kernel.
/// The binary tag is the declaration order and the JSON value is the camelCase wire name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceToolRunState {
    Starting,
    Running,
    Paused,
    Complete,
    Finalizing,
    Finalized,
    Aborting,
    Aborted,
    Faulted,
}

impl PresenceToolRunState {
    pub const ALL: [PresenceToolRunState; 9] = [Self::Starting, Self::Running, Self::Paused, Self::Complete, Self::Finalizing, Self::Finalized, Self::Aborting, Self::Aborted, Self::Faulted];

    /// 🔤️ The camelCase wire spelling shared with `ToolRunState`.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Complete => "complete",
            Self::Finalizing => "finalizing",
            Self::Finalized => "finalized",
            Self::Aborting => "aborting",
            Self::Aborted => "aborted",
            Self::Faulted => "faulted",
        }
    }

    /// 🔡️ Inverse of [`Self::wire_name`].
    pub fn from_wire_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|state| state.wire_name() == name)
    }
}

/// 📶️ Ephemeral shared progress summary of a peer's tool run ("Alice · Fill · 42 %").
///
/// `stage` indexes the tool's declared stages, `total` absent means indeterminate, and the percentage
/// is derived by the renderer. `completed <= total` whenever `total` is present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceToolRun {
    pub tool_id: String,
    pub state: PresenceToolRunState,
    pub stage: u16,
    pub completed: u64,
    pub total: Option<u64>,
}

impl semio_framework_value::ToValue for PresenceToolRun {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("toolId".to_string(), semio_framework_value::ToValue::to_value(&self.tool_id)),
            ("state".to_string(), semio_framework_value::DslValue::String(self.state.wire_name().to_string())),
            ("stage".to_string(), semio_framework_value::ToValue::to_value(&self.stage)),
            ("completed".to_string(), semio_framework_value::ToValue::to_value(&self.completed)),
        ];
        if let Some(total) = &self.total {
            entries.push(("total".to_string(), semio_framework_value::ToValue::to_value(total)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for PresenceToolRun {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceToolRun, found {value:?}")));
        };
        let mut tool_id = None;
        let mut state = None;
        let mut stage = None;
        let mut completed = None;
        let mut total = None;
        for (key, entry) in fields {
            match key.as_str() {
                "toolId" => tool_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("toolId"))?),
                "state" => {
                    let name = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("state"))?;
                    state = Some(PresenceToolRunState::from_wire_name(&name).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown tool run state {name}")).under("state"))?);
                }
                "stage" => stage = Some(<u16 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("stage"))?),
                "completed" => completed = Some(<u64 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("completed"))?),
                "total" => total = <Option<u64> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("total"))?,
                _ => {}
            }
        }
        Ok(PresenceToolRun {
            tool_id: tool_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceToolRun missing toolId"))?,
            state: state.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceToolRun missing state"))?,
            stage: stage.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceToolRun missing stage"))?,
            completed: completed.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceToolRun missing completed"))?,
            total,
        })
    }
}

/// ⏪️ Stage of a peer's open history edit, spelled like `HistoryTimeTravelStage` on the history wire. The binary tag
/// is the declaration order and the JSON value is the camelCase wire name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceHistoryEditStage {
    Editing,
    Replaying,
    Reviewing,
    Choosing,
    Finalizing,
}

impl PresenceHistoryEditStage {
    pub const ALL: [PresenceHistoryEditStage; 5] = [Self::Editing, Self::Replaying, Self::Reviewing, Self::Choosing, Self::Finalizing];

    /// 🔤️ The camelCase wire spelling shared with `HistoryTimeTravelStage`.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Editing => "editing",
            Self::Replaying => "replaying",
            Self::Reviewing => "reviewing",
            Self::Choosing => "choosing",
            Self::Finalizing => "finalizing",
        }
    }

    /// 🔡️ Inverse of [`Self::wire_name`].
    pub fn from_wire_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|stage| stage.wire_name() == name)
    }
}

/// 👥️ Ephemeral shared summary of a peer's open history edit ("Alice is editing history: Set count to 2"): the
/// replica-independent id of the mutation whose inputs it drafts — every replica labels it from its own history rows,
/// in its own locale — the session stage and how many drafts it accepted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresenceHistoryEdit {
    pub mutation_id: String,
    pub stage: PresenceHistoryEditStage,
    pub drafts: u32,
}

impl semio_framework_value::ToValue for PresenceHistoryEdit {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("mutationId".to_string(), semio_framework_value::ToValue::to_value(&self.mutation_id)),
            ("stage".to_string(), semio_framework_value::DslValue::String(self.stage.wire_name().to_string())),
            ("drafts".to_string(), semio_framework_value::ToValue::to_value(&self.drafts)),
        ])
    }
}

impl semio_framework_value::FromValue for PresenceHistoryEdit {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceHistoryEdit, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut stage = None;
        let mut drafts = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "stage" => {
                    let name = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("stage"))?;
                    stage = Some(PresenceHistoryEditStage::from_wire_name(&name).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown history edit stage {name}")).under("stage"))?);
                }
                "drafts" => drafts = Some(<u32 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("drafts"))?),
                _ => {}
            }
        }
        Ok(PresenceHistoryEdit {
            mutation_id: mutation_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceHistoryEdit missing mutationId"))?,
            stage: stage.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceHistoryEdit missing stage"))?,
            drafts: drafts.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceHistoryEdit missing drafts"))?,
        })
    }
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. `presence_pack` mirrors
/// the pre-existing `#[serde(with = "presence_pack_serde")]` base64-string wire shape byte-for-byte
/// (rather than the default `Vec<u8>` numeric-array shape) via this crate's own
/// `semio-framework-io-base64` dependency. The real wire path is `encode_presence_peer`/
/// `decode_presence_peer` below (a hand-rolled binary codec); this exists for capability parity.
impl semio_framework_value::ToValue for PresencePeer {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![("actor".to_string(), semio_framework_value::ToValue::to_value(&self.actor)), ("connectedAtMs".to_string(), semio_framework_value::ToValue::to_value(&self.connected_at_ms))];
        if let Some(label) = &self.label {
            entries.push(("label".to_string(), semio_framework_value::ToValue::to_value(label)));
        }
        if let Some(pack) = &self.presence_pack {
            entries.push(("presencePack".to_string(), semio_framework_value::DslValue::String(semio_framework_io_base64::base64_standard_encode(pack))));
        }
        if let Some(user_id) = &self.user_id {
            entries.push(("userId".to_string(), semio_framework_value::ToValue::to_value(user_id)));
        }
        if let Some(role) = &self.role {
            entries.push(("role".to_string(), semio_framework_value::ToValue::to_value(role)));
        }
        if let Some(drag_ghost_json) = &self.drag_ghost_json {
            entries.push(("dragGhostJson".to_string(), semio_framework_value::ToValue::to_value(drag_ghost_json)));
        }
        if let Some(interaction) = &self.interaction {
            entries.push(("interaction".to_string(), semio_framework_value::ToValue::to_value(interaction)));
        }
        if let Some(color) = &self.color {
            entries.push(("color".to_string(), semio_framework_value::ToValue::to_value(color)));
        }
        if let Some(surface) = &self.surface {
            entries.push(("surface".to_string(), semio_framework_value::ToValue::to_value(surface)));
        }
        if !self.views.is_empty() {
            entries.push(("views".to_string(), semio_framework_value::ToValue::to_value(&self.views)));
        }
        if let Some(ui) = &self.ui {
            entries.push(("ui".to_string(), semio_framework_value::ToValue::to_value(ui)));
        }
        if let Some(tool_run) = &self.tool_run {
            entries.push(("toolRun".to_string(), semio_framework_value::ToValue::to_value(tool_run)));
        }
        if let Some(principal_kind) = self.principal_kind {
            entries.push(("principalKind".to_string(), semio_framework_value::DslValue::String(principal_kind.wire_name().to_string())));
        }
        if let Some(active_tool) = &self.active_tool {
            entries.push(("activeTool".to_string(), semio_framework_value::ToValue::to_value(active_tool)));
        }
        if let Some(history_edit) = &self.history_edit {
            entries.push(("historyEdit".to_string(), semio_framework_value::ToValue::to_value(history_edit)));
        }
        if !self.typing.is_empty() {
            entries.push(("typing".to_string(), semio_framework_value::ToValue::to_value(&self.typing)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for PresencePeer {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresencePeer, found {value:?}")));
        };
        let mut actor = None;
        let mut connected_at_ms = None;
        let mut label = None;
        let mut presence_pack = None;
        let mut user_id = None;
        let mut role = None;
        let mut drag_ghost_json = None;
        let mut interaction = None;
        let mut color = None;
        let mut surface = None;
        let mut views = Vec::new();
        let mut ui = None;
        let mut tool_run = None;
        let mut principal_kind = None;
        let mut active_tool = None;
        let mut history_edit = None;
        let mut typing = Vec::new();
        for (key, entry) in fields {
            match key.as_str() {
                "actor" => actor = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "connectedAtMs" => connected_at_ms = Some(<i64 as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("connectedAtMs"))?),
                "label" => label = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("label"))?,
                "presencePack" => {
                    let semio_framework_value::DslValue::String(encoded) = entry else {
                        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresencePeer.presencePack must be a string").under("presencePack"));
                    };
                    presence_pack = Some(semio_framework_io_base64::base64_standard_decode(encoded.as_bytes()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()).under("presencePack"))?);
                }
                "userId" => user_id = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("userId"))?,
                "role" => role = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("role"))?,
                "dragGhostJson" => drag_ghost_json = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("dragGhostJson"))?,
                "interaction" => interaction = <Option<PresenceInteraction> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("interaction"))?,
                "color" => color = <Option<u8> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("color"))?,
                "surface" => surface = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("surface"))?,
                "views" => views = <Vec<PresenceWindowView> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("views"))?,
                "ui" => ui = <Option<PresenceUi> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("ui"))?,
                "toolRun" => tool_run = <Option<PresenceToolRun> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("toolRun"))?,
                "principalKind" => {
                    principal_kind = match entry {
                        semio_framework_value::DslValue::Null => None,
                        semio_framework_value::DslValue::String(name) => {
                            Some(PresencePrincipalKind::from_wire_name(&name).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown PresencePeer.principalKind `{name}`")).under("principalKind"))?)
                        }
                        other => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("PresencePeer.principalKind must be a string, found {other:?}")).under("principalKind")),
                    }
                }
                "activeTool" => active_tool = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("activeTool"))?,
                "historyEdit" => history_edit = <Option<PresenceHistoryEdit> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("historyEdit"))?,
                "typing" => typing = <Vec<PresenceTyping> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("typing"))?,
                _ => {}
            }
        }
        Ok(PresencePeer {
            actor: actor.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresencePeer missing actor"))?,
            connected_at_ms: connected_at_ms.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresencePeer missing connectedAtMs"))?,
            label,
            presence_pack,
            user_id,
            role,
            drag_ghost_json,
            interaction,
            color,
            surface,
            views,
            ui,
            tool_run,
            principal_kind,
            active_tool,
            history_edit,
            typing,
        })
    }
}
//#endregion 🔖️Presence
