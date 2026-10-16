//! 👥️ Peer presence wire contract: window views, typing, tool runs, history edits and the typed interaction roster, with the borrowed reader and the peer overlay.
//!
//! Presence travels inside the lane-tagged frames only as opaque blobs; this crate owns what the blobs contain.

use std::collections::BTreeMap;
use semio_framework_replication_interaction::HoverSpec;
use semio_framework_replication_interaction::InteractionState;
use semio_framework_replication_interaction::SelectionSpec;
use semio_framework_replication_wire_core::malformed;
use semio_framework_replication_wire_core::write_opt_str;

//#region 🔖️Presence
// 🎯️ W6 kernel unification: `PayloadHash`/`MutationEnvelope`/`MutationDagError`/`MutationDag`/`InsertResult`
// (the local causal-sync types) and `HubClientFrame`/`HubServerFrame` (the local semio_hub wire frames)
// are DELETED — `store`/`store_sync` (their only consumers outside this crate) now speak
// `protocol::{MutationEnvelope, MutationDag, MutationDagError, InsertResult}`/`protocol::{ClientFrame,
// ServerFrame}` directly (W5 already made these real binary types; this wave just stops
// duplicating them here). `PresencePoint`/`PresenceViewport`/`PresencePeer` below are NOT
// duplicates of anything in `protocol` — no equivalent exists there — so they stay, kept in their
// own region since the `🔖️HubProtocol` name they used to share with the now-deleted frame enums no
// longer fits. 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
// C7.1: `PresencePoint`/`PresenceViewport` and `PresencePeer.cursor`/`.viewport` are DELETED —
// replaced by `views: Vec<PresenceWindowView>` (one entry per open window/surface, artifact-scope,
// matched by `space`) plus `ui: Option<PresenceUi>` (app-scope `data-ui-path` hover/focus/press).
//#region 🔖️PresenceView
/// 🪟️ One open window/surface's live view for a document — camera/pan-zoom plus in-view
/// pointer, broadcast so peers can render each other's viewport rectangles / camera frustums /
/// cursor markers. `window_id` disambiguates multiple windows viewing the same space; `space` is
/// the coordinate-space id the surface host reports (`"world"`/`"canvas"`/`"geo"`, or an app-declared
/// finer id) — an overlay renders a peer view only in local surfaces with the same `space`.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceWindowView {
    pub window_id: String,
    pub space: String,
    pub kind: PresenceViewKind,
    /// 📐️ The reporting surface's pixel size — needed to draw a peer's viewport rectangle.
    pub size: [f64; 2],
    /// 📍️ In view coordinates: world point (Orbit), `[x, y, 0]` canvas point (Canvas),
    /// `[lng, lat, 0]` (Geo).
    pub pointer: Option<[f64; 3]>,
    /// World-space ray origin for 3D presence (Orbit); absent for canvas/geo.
    pub ray_origin: Option<[f64; 3]>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. The real wire path is
/// `🔖️PresenceViewCodec` below (a hand-rolled binary codec); this exists for capability parity.
impl semio_framework_value::ToValue for PresenceWindowView {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("windowId".to_string(), semio_framework_value::ToValue::to_value(&self.window_id)),
            ("space".to_string(), semio_framework_value::ToValue::to_value(&self.space)),
            ("kind".to_string(), semio_framework_value::ToValue::to_value(&self.kind)),
            ("size".to_string(), semio_framework_value::ToValue::to_value(&self.size)),
        ];
        if self.pointer.is_some() {
            entries.push(("pointer".to_string(), semio_framework_value::ToValue::to_value(&self.pointer)));
        }
        if self.ray_origin.is_some() {
            entries.push(("rayOrigin".to_string(), semio_framework_value::ToValue::to_value(&self.ray_origin)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for PresenceWindowView {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceWindowView, found {value:?}")));
        };
        let mut window_id = None;
        let mut space = None;
        let mut kind = None;
        let mut size = None;
        let mut pointer = None;
        let mut ray_origin = None;
        for (key, entry) in fields {
            match key.as_str() {
                "windowId" => window_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("windowId"))?),
                "space" => space = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("space"))?),
                "kind" => kind = Some(<PresenceViewKind as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("kind"))?),
                "size" => size = Some(<[f64; 2] as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("size"))?),
                "pointer" => pointer = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("pointer"))?,
                "rayOrigin" => ray_origin = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("rayOrigin"))?,
                _ => {}
            }
        }
        Ok(PresenceWindowView {
            window_id: window_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing windowId"))?,
            space: space.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing space"))?,
            kind: kind.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing kind"))?,
            size: size.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing size"))?,
            pointer,
            ray_origin,
        })
    }
}

/// 🎥️ A peer's live camera/pan-zoom, tagged by surface family.
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceViewKind {
    Canvas { x: f64, y: f64, zoom: f64 },
    Orbit { position: [f64; 3], target: [f64; 3], up: [f64; 3], fov: f64 },
    Geo { lng: f64, lat: f64, zoom: f64, bearing: f64, pitch: f64 },
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]` byte-for-byte.
impl semio_framework_value::ToValue for PresenceViewKind {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            PresenceViewKind::Canvas { x, y, zoom } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("canvas".to_string())),
                ("x".to_string(), semio_framework_value::ToValue::to_value(x)),
                ("y".to_string(), semio_framework_value::ToValue::to_value(y)),
                ("zoom".to_string(), semio_framework_value::ToValue::to_value(zoom)),
            ]),
            PresenceViewKind::Orbit { position, target, up, fov } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("orbit".to_string())),
                ("position".to_string(), semio_framework_value::ToValue::to_value(position)),
                ("target".to_string(), semio_framework_value::ToValue::to_value(target)),
                ("up".to_string(), semio_framework_value::ToValue::to_value(up)),
                ("fov".to_string(), semio_framework_value::ToValue::to_value(fov)),
            ]),
            PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("geo".to_string())),
                ("lng".to_string(), semio_framework_value::ToValue::to_value(lng)),
                ("lat".to_string(), semio_framework_value::ToValue::to_value(lat)),
                ("zoom".to_string(), semio_framework_value::ToValue::to_value(zoom)),
                ("bearing".to_string(), semio_framework_value::ToValue::to_value(bearing)),
                ("pitch".to_string(), semio_framework_value::ToValue::to_value(pitch)),
            ]),
        }
    }
}
impl semio_framework_value::FromValue for PresenceViewKind {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceViewKind, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(semio_framework_value::DslValue::String(s)) => s,
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind missing kind")),
        };
        match kind.as_str() {
            "canvas" => Ok(PresenceViewKind::Canvas {
                x: <f64 as semio_framework_value::FromValue>::from_value(get("x").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing x"))?).map_err(|e| e.under("x"))?,
                y: <f64 as semio_framework_value::FromValue>::from_value(get("y").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing y"))?).map_err(|e| e.under("y"))?,
                zoom: <f64 as semio_framework_value::FromValue>::from_value(get("zoom").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing zoom"))?).map_err(|e| e.under("zoom"))?,
            }),
            "orbit" => Ok(PresenceViewKind::Orbit {
                position: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("position").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing position"))?).map_err(|e| e.under("position"))?,
                target: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("target").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing target"))?).map_err(|e| e.under("target"))?,
                up: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("up").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing up"))?).map_err(|e| e.under("up"))?,
                fov: <f64 as semio_framework_value::FromValue>::from_value(get("fov").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing fov"))?).map_err(|e| e.under("fov"))?,
            }),
            "geo" => Ok(PresenceViewKind::Geo {
                lng: <f64 as semio_framework_value::FromValue>::from_value(get("lng").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing lng"))?).map_err(|e| e.under("lng"))?,
                lat: <f64 as semio_framework_value::FromValue>::from_value(get("lat").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing lat"))?).map_err(|e| e.under("lat"))?,
                zoom: <f64 as semio_framework_value::FromValue>::from_value(get("zoom").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing zoom"))?).map_err(|e| e.under("zoom"))?,
                bearing: <f64 as semio_framework_value::FromValue>::from_value(get("bearing").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing bearing"))?).map_err(|e| e.under("bearing"))?,
                pitch: <f64 as semio_framework_value::FromValue>::from_value(get("pitch").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing pitch"))?).map_err(|e| e.under("pitch"))?,
            }),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown PresenceViewKind kind `{other}`"))),
        }
    }
}

/// 🖱️ A peer's live `data-ui-path` hover/focus/press state (APP scope) — the grammar
/// `type[idx]#id/...`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresenceUi {
    pub hovered_path: Option<String>,
    pub focused_path: Option<String>,
    pub pressed_path: Option<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for PresenceUi {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = Vec::new();
        if self.hovered_path.is_some() {
            entries.push(("hoveredPath".to_string(), semio_framework_value::ToValue::to_value(&self.hovered_path)));
        }
        if self.focused_path.is_some() {
            entries.push(("focusedPath".to_string(), semio_framework_value::ToValue::to_value(&self.focused_path)));
        }
        if self.pressed_path.is_some() {
            entries.push(("pressedPath".to_string(), semio_framework_value::ToValue::to_value(&self.pressed_path)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}
impl semio_framework_value::FromValue for PresenceUi {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceUi, found {value:?}")));
        };
        let mut out = PresenceUi::default();
        for (key, entry) in fields {
            match key.as_str() {
                "hoveredPath" => out.hovered_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("hoveredPath"))?,
                "focusedPath" => out.focused_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("focusedPath"))?,
                "pressedPath" => out.pressed_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("pressedPath"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceViewCodec
async fn encode_presence_view_kind(kind: &PresenceViewKind, out: &mut Vec<u8>) {
    match kind {
        PresenceViewKind::Canvas { x, y, zoom } => {
            out.push(0);
            semio_framework_replication_wire_core::write_f64(out, *x);
            semio_framework_replication_wire_core::write_f64(out, *y);
            semio_framework_replication_wire_core::write_f64(out, *zoom);
        }
        PresenceViewKind::Orbit { position, target, up, fov } => {
            out.push(1);
            for value in position.iter().chain(target.iter()).chain(up.iter()) {
                semio_framework_replication_wire_core::write_f64(out, *value);
            }
            semio_framework_replication_wire_core::write_f64(out, *fov);
        }
        PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => {
            out.push(2);
            semio_framework_replication_wire_core::write_f64(out, *lng);
            semio_framework_replication_wire_core::write_f64(out, *lat);
            semio_framework_replication_wire_core::write_f64(out, *zoom);
            semio_framework_replication_wire_core::write_f64(out, *bearing);
            semio_framework_replication_wire_core::write_f64(out, *pitch);
        }
    }
}

async fn encode_presence_window_view(view: &PresenceWindowView, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &view.window_id);
    semio_framework_replication_wire_core::write_str(out, &view.space);
    encode_presence_view_kind(&view.kind, out).await;
    semio_framework_replication_wire_core::write_f64(out, view.size[0]);
    semio_framework_replication_wire_core::write_f64(out, view.size[1]);
    semio_framework_replication_wire_core::write_bool(out, view.pointer.is_some());
    if let Some(pointer) = view.pointer {
        for value in pointer {
            semio_framework_replication_wire_core::write_f64(out, value);
        }
    }
    semio_framework_replication_wire_core::write_bool(out, view.ray_origin.is_some());
    if let Some(ray_origin) = view.ray_origin {
        for value in ray_origin {
            semio_framework_replication_wire_core::write_f64(out, value);
        }
    }
}

async fn write_vec_presence_window_view(out: &mut Vec<u8>, values: &[PresenceWindowView]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_presence_window_view(value, out).await;
    }
}

async fn encode_presence_ui(ui: &PresenceUi, out: &mut Vec<u8>) {
    write_opt_str(out, &ui.hovered_path).await;
    write_opt_str(out, &ui.focused_path).await;
    write_opt_str(out, &ui.pressed_path).await;
}

//#endregion 🔖️PresenceViewCodec
//#endregion 🔖️PresenceView

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

/// 🎯️ Binary `PresencePeer` codec: `actor str | flags varint_u64 | connected_at_ms varint |
/// fields present per bitmask, strictly in bit order`. `protocol_wire::ClientFrame::Presence`/
/// `ServerFrame::Presence` carry the resulting bytes opaquely (that crate has no dependency on this
/// one) — this is the encode/decode pair store_sync calls on either side of the wire.
/// `presence_pack` is length-prefixed bytes in flag bit 1; `drag_ghost_json` stays opaque app-owned
/// text (never re-parsed as JSON here, same as `ArtifactDiff.payload` staying opaque bytes). Bit 5
/// carries `interaction` via `encode_presence_interaction`/`decode_presence_interaction`
/// (self-delimiting varint-counted fields — see the `🔖️PresenceInteraction` region below); bit 8
/// (`views`) is set iff non-empty; bit 9 (`ui`) carries three `opt_str` fields unconditionally once
/// present. `flags` widened from a single `u8` to a varint (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.1) now that bit 9 exceeds a byte's range. Bit 10 carries
/// `tool_run` as `tool_id str | state u8 | stage varint | completed varint | total bool+varint?`.
/// Bit 11 carries `principal_kind` as one declaration-order tag byte (`0` human, `1` agent); bit 12 carries `active_tool`;
/// bit 13 carries `history_edit` as `mutation_id str | stage u8 | drafts varint`; bit 14 (`typing`) is set iff non-empty and
/// carries `count varint | (window_id str | deleted str | insert str)*`; a peer
/// that leaves it unset is the pre-agent wire shape and reads back as `None`, so every encoder and
/// decoder written before the agent principal existed still round-trips byte-for-byte.
pub async fn encode_presence_peer(peer: &PresencePeer) -> Vec<u8> {
    let mut out = Vec::new();
    semio_framework_replication_wire_core::write_str(&mut out, &peer.actor);
    let mut flags = 0u64;
    if peer.label.is_some() {
        flags |= 1 << 0;
    }
    if peer.presence_pack.is_some() {
        flags |= 1 << 1;
    }
    if peer.user_id.is_some() {
        flags |= 1 << 2;
    }
    if peer.role.is_some() {
        flags |= 1 << 3;
    }
    if peer.drag_ghost_json.is_some() {
        flags |= 1 << 4;
    }
    if peer.interaction.is_some() {
        flags |= 1 << 5;
    }
    if peer.color.is_some() {
        flags |= 1 << 6;
    }
    if peer.surface.is_some() {
        flags |= 1 << 7;
    }
    if !peer.views.is_empty() {
        flags |= 1 << 8;
    }
    if peer.ui.is_some() {
        flags |= 1 << 9;
    }
    if peer.tool_run.is_some() {
        flags |= 1 << 10;
    }
    if peer.principal_kind.is_some() {
        flags |= 1 << 11;
    }
    if peer.active_tool.is_some() {
        flags |= 1 << 12;
    }
    if peer.history_edit.is_some() {
        flags |= 1 << 13;
    }
    if !peer.typing.is_empty() {
        flags |= 1 << 14;
    }
    semio_framework_replication_wire_core::write_varint_u64(&mut out, flags);
    semio_framework_replication_wire_core::write_varint_u64(&mut out, peer.connected_at_ms as u64);
    if let Some(label) = &peer.label {
        semio_framework_replication_wire_core::write_str(&mut out, label);
    }
    if let Some(presence_pack) = &peer.presence_pack {
        semio_framework_replication_wire_core::write_bytes(&mut out, presence_pack);
    }
    if let Some(user_id) = &peer.user_id {
        semio_framework_replication_wire_core::write_str(&mut out, user_id);
    }
    if let Some(role) = &peer.role {
        semio_framework_replication_wire_core::write_str(&mut out, role);
    }
    if let Some(drag_ghost_json) = &peer.drag_ghost_json {
        semio_framework_replication_wire_core::write_str(&mut out, drag_ghost_json);
    }
    if let Some(interaction) = &peer.interaction {
        encode_presence_interaction(interaction, &mut out).await;
    }
    if let Some(color) = peer.color {
        out.push(color);
    }
    if let Some(surface) = &peer.surface {
        semio_framework_replication_wire_core::write_str(&mut out, surface);
    }
    if !peer.views.is_empty() {
        write_vec_presence_window_view(&mut out, &peer.views).await;
    }
    if let Some(ui) = &peer.ui {
        encode_presence_ui(ui, &mut out).await;
    }
    if let Some(tool_run) = &peer.tool_run {
        encode_presence_tool_run(tool_run, &mut out);
    }
    if let Some(principal_kind) = peer.principal_kind {
        out.push(principal_kind as u8);
    }
    if let Some(active_tool) = &peer.active_tool {
        semio_framework_replication_wire_core::write_str(&mut out, active_tool);
    }
    if let Some(history_edit) = &peer.history_edit {
        encode_presence_history_edit(history_edit, &mut out);
    }
    if !peer.typing.is_empty() {
        semio_framework_replication_wire_core::write_varint_u64(&mut out, peer.typing.len() as u64);
        for typing in &peer.typing {
            semio_framework_replication_wire_core::write_str(&mut out, &typing.window_id);
            semio_framework_replication_wire_core::write_str(&mut out, &typing.deleted);
            semio_framework_replication_wire_core::write_str(&mut out, &typing.insert);
        }
    }
    out
}

/// ⏯️ Appends one `PresenceToolRun` body — the exact bytes a `PresencePeer` carries under flag bit 10.
/// `pub` so a guest's `AppFrame::Ephemeral` can publish its run summary in the same encoding the heartbeat
/// assembly re-embeds (contract `📋️tool-run-contract.md` §3.4).
pub fn encode_presence_tool_run(tool_run: &PresenceToolRun, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &tool_run.tool_id);
    out.push(tool_run.state as u8);
    semio_framework_replication_wire_core::write_varint_u64(out, u64::from(tool_run.stage));
    semio_framework_replication_wire_core::write_varint_u64(out, tool_run.completed);
    semio_framework_replication_wire_core::write_bool(out, tool_run.total.is_some());
    if let Some(total) = tool_run.total {
        semio_framework_replication_wire_core::write_varint_u64(out, total);
    }
}

/// ⏪️ Appends one `PresenceHistoryEdit` body — the exact bytes a `PresencePeer` carries under flag bit 13, and what a
/// guest's `AppFrame::Ephemeral.history_edit` publishes.
pub fn encode_presence_history_edit(history_edit: &PresenceHistoryEdit, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &history_edit.mutation_id);
    out.push(history_edit.stage as u8);
    semio_framework_replication_wire_core::write_varint_u64(out, u64::from(history_edit.drafts));
}

/// 🛡️ Fixed hostile-input ceilings shared with the TypeScript presence decoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PresencePeerWireLimitsV1 {
    pub maximum_entry_bytes: usize,
    pub maximum_text_bytes: usize,
    pub maximum_presence_pack_bytes: usize,
    pub maximum_views: usize,
    pub maximum_interaction_domains: usize,
    pub maximum_domain_ids: usize,
    pub maximum_connected_at_ms: u64,
    pub maximum_tool_run_units: u64,
    pub maximum_typing_runs: usize,
}

pub const PRESENCE_PEER_WIRE_LIMITS_V1: PresencePeerWireLimitsV1 = PresencePeerWireLimitsV1 {
    maximum_entry_bytes: 4_096,
    maximum_text_bytes: 1_024,
    maximum_presence_pack_bytes: 2_048,
    maximum_views: 16,
    maximum_interaction_domains: 16,
    maximum_domain_ids: 64,
    maximum_connected_at_ms: 9_007_199_254_740_991,
    maximum_tool_run_units: 9_007_199_254_740_991,
    maximum_typing_runs: 8,
};

struct PresencePeerReader<'a> {
    bytes: &'a [u8],
    position: usize,
    limits: PresencePeerWireLimitsV1,
}

impl<'a> PresencePeerReader<'a> {
    fn malformed(&self, what: &'static str, detail: impl Into<String>) -> semio_framework_replication_error::ProtocolError {
        semio_framework_replication_error::ProtocolError::Malformed { what, offset: self.position as u64, detail: detail.into() }
    }

    fn varint(&mut self, what: &'static str) -> Result<u64, semio_framework_replication_error::ProtocolError> {
        let start = self.position;
        let mut value = 0u64;
        for index in 0..10 {
            let byte = *self.bytes.get(self.position).ok_or_else(|| self.malformed(what, "truncated varint"))?;
            self.position += 1;
            if index == 9 && byte > 1 { return Err(self.malformed(what, "varint exceeds u64")); }
            value |= u64::from(byte & 0x7f) << (index * 7);
            if byte & 0x80 == 0 {
                if self.position - start > 1 && byte == 0 { return Err(self.malformed(what, "noncanonical varint")); }
                return Ok(value);
            }
        }
        Err(self.malformed(what, "overlong varint"))
    }

    fn length(&mut self, maximum: usize, what: &'static str) -> Result<usize, semio_framework_replication_error::ProtocolError> {
        let value = self.varint(what)?;
        if value > maximum as u64 { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded(what)); }
        let length = usize::try_from(value).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded(what))?;
        if length > self.bytes.len().saturating_sub(self.position) { return Err(self.malformed(what, "truncated field")); }
        Ok(length)
    }

    fn count(&mut self, maximum: usize, what: &'static str) -> Result<usize, semio_framework_replication_error::ProtocolError> {
        let count = self.length(maximum, what)?;
        Ok(count)
    }

    fn text(&mut self, what: &'static str) -> Result<String, semio_framework_replication_error::ProtocolError> {
        let length = self.length(self.limits.maximum_text_bytes, what)?;
        let end = self.position + length;
        let value = std::str::from_utf8(&self.bytes[self.position..end]).map_err(|_| self.malformed(what, "invalid utf8"))?.to_owned();
        self.position = end;
        Ok(value)
    }

    fn blob(&mut self, what: &'static str) -> Result<Vec<u8>, semio_framework_replication_error::ProtocolError> {
        let length = self.length(self.limits.maximum_presence_pack_bytes, what)?;
        let end = self.position + length;
        let value = self.bytes[self.position..end].to_vec();
        self.position = end;
        Ok(value)
    }

    fn byte(&mut self, what: &'static str) -> Result<u8, semio_framework_replication_error::ProtocolError> {
        let value = *self.bytes.get(self.position).ok_or_else(|| self.malformed(what, "truncated byte"))?;
        self.position += 1;
        Ok(value)
    }

    fn principal_kind(&mut self) -> Result<PresencePrincipalKind, semio_framework_replication_error::ProtocolError> {
        let tag = self.byte("presence peer principal kind")?;
        PresencePrincipalKind::ALL.get(usize::from(tag)).copied().ok_or_else(|| self.malformed("presence peer principal kind", format!("unknown principal kind tag {tag}")))
    }

    fn boolean(&mut self, what: &'static str) -> Result<bool, semio_framework_replication_error::ProtocolError> {
        match self.byte(what)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(self.malformed(what, "boolean must be zero or one")),
        }
    }

    fn number(&mut self, what: &'static str) -> Result<f64, semio_framework_replication_error::ProtocolError> {
        let end = self.position.checked_add(8).ok_or_else(|| self.malformed(what, "length overflow"))?;
        let slice = self.bytes.get(self.position..end).ok_or_else(|| self.malformed(what, "truncated f64"))?;
        let value = f64::from_le_bytes(slice.try_into().map_err(|_| self.malformed(what, "invalid f64"))?);
        if !value.is_finite() { return Err(self.malformed(what, "non-finite f64")); }
        self.position = end;
        Ok(value)
    }

    fn triple(&mut self, what: &'static str) -> Result<[f64; 3], semio_framework_replication_error::ProtocolError> {
        Ok([self.number(what)?, self.number(what)?, self.number(what)?])
    }

    fn strings(&mut self, what: &'static str) -> Result<Vec<String>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_domain_ids, what)?;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count { values.push(self.text(what)?); }
        Ok(values)
    }

    fn interaction(&mut self) -> Result<PresenceInteraction, semio_framework_replication_error::ProtocolError> {
        let app_id = self.text("presence interaction app id")?;
        let count = self.count(self.limits.maximum_interaction_domains, "presence interaction domains")?;
        let mut domains = Vec::with_capacity(count);
        for _ in 0..count {
            domains.push(PresenceDomain {
                domain: self.text("presence interaction domain")?,
                granularity: self.text("presence interaction granularity")?,
                selected: self.strings("presence interaction selected")?,
                hovered: self.strings("presence interaction hovered")?,
            });
        }
        Ok(PresenceInteraction { app_id, domains })
    }

    fn view_kind(&mut self) -> Result<PresenceViewKind, semio_framework_replication_error::ProtocolError> {
        match self.byte("presence view kind")? {
            0 => Ok(PresenceViewKind::Canvas { x: self.number("presence canvas x")?, y: self.number("presence canvas y")?, zoom: self.number("presence canvas zoom")? }),
            1 => Ok(PresenceViewKind::Orbit { position: self.triple("presence orbit position")?, target: self.triple("presence orbit target")?, up: self.triple("presence orbit up")?, fov: self.number("presence orbit fov")? }),
            2 => Ok(PresenceViewKind::Geo { lng: self.number("presence geo longitude")?, lat: self.number("presence geo latitude")?, zoom: self.number("presence geo zoom")?, bearing: self.number("presence geo bearing")?, pitch: self.number("presence geo pitch")? }),
            tag => Err(self.malformed("presence view kind", format!("unknown tag {tag:#x}"))),
        }
    }

    fn views(&mut self) -> Result<Vec<PresenceWindowView>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_views, "presence views")?;
        let mut views = Vec::with_capacity(count);
        for _ in 0..count {
            let window_id = self.text("presence view window id")?;
            let space = self.text("presence view space")?;
            let kind = self.view_kind()?;
            let size = [self.number("presence view width")?, self.number("presence view height")?];
            let pointer = if self.boolean("presence view pointer")? { Some(self.triple("presence view pointer")?) } else { None };
            let ray_origin = if self.boolean("presence view ray origin")? { Some(self.triple("presence view ray origin")?) } else { None };
            views.push(PresenceWindowView { window_id, space, kind, size, pointer, ray_origin });
        }
        Ok(views)
    }

    fn optional_text(&mut self, what: &'static str) -> Result<Option<String>, semio_framework_replication_error::ProtocolError> {
        if self.boolean(what)? { Ok(Some(self.text(what)?)) } else { Ok(None) }
    }

    fn tool_run_units(&mut self, what: &'static str) -> Result<u64, semio_framework_replication_error::ProtocolError> {
        let value = self.varint(what)?;
        if value > self.limits.maximum_tool_run_units { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded(what)); }
        Ok(value)
    }

    fn tool_run(&mut self) -> Result<PresenceToolRun, semio_framework_replication_error::ProtocolError> {
        let tool_id = self.text("presence tool run id")?;
        let tag = self.byte("presence tool run state")?;
        let state = *PresenceToolRunState::ALL.get(usize::from(tag)).ok_or_else(|| self.malformed("presence tool run state", format!("unknown tag {tag:#x}")))?;
        let stage = u16::try_from(self.varint("presence tool run stage")?).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded("presence tool run stage"))?;
        let completed = self.tool_run_units("presence tool run completed")?;
        let total = if self.boolean("presence tool run total")? { Some(self.tool_run_units("presence tool run total")?) } else { None };
        if total.is_some_and(|total| completed > total) { return Err(self.malformed("presence tool run completed", "completed exceeds total")); }
        Ok(PresenceToolRun { tool_id, state, stage, completed, total })
    }

    fn history_edit(&mut self) -> Result<PresenceHistoryEdit, semio_framework_replication_error::ProtocolError> {
        let mutation_id = self.text("presence history edit mutation")?;
        let tag = self.byte("presence history edit stage")?;
        let stage = *PresenceHistoryEditStage::ALL.get(usize::from(tag)).ok_or_else(|| self.malformed("presence history edit stage", format!("unknown tag {tag:#x}")))?;
        let drafts = u32::try_from(self.varint("presence history edit drafts")?).map_err(|_| semio_framework_replication_error::ProtocolError::LimitExceeded("presence history edit drafts"))?;
        Ok(PresenceHistoryEdit { mutation_id, stage, drafts })
    }

    fn ui(&mut self) -> Result<PresenceUi, semio_framework_replication_error::ProtocolError> {
        Ok(PresenceUi { hovered_path: self.optional_text("presence ui hovered path")?, focused_path: self.optional_text("presence ui focused path")?, pressed_path: self.optional_text("presence ui pressed path")? })
    }

    fn typing(&mut self) -> Result<Vec<PresenceTyping>, semio_framework_replication_error::ProtocolError> {
        let count = self.count(self.limits.maximum_typing_runs, "presence typing runs")?;
        if count == 0 {
            return Err(self.malformed("presence typing runs", "an empty typing list is never flagged"));
        }
        let mut runs = Vec::with_capacity(count);
        for _ in 0..count {
            let (window_id, deleted, insert) = (self.text("presence typing window")?, self.text("presence typing deleted")?, self.text("presence typing insert")?);
            if deleted.len() > PRESENCE_TYPING_EXCERPT_BYTES || insert.len() > PRESENCE_TYPING_EXCERPT_BYTES {
                return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence typing excerpt"));
            }
            runs.push(PresenceTyping { window_id, deleted, insert });
        }
        Ok(runs)
    }
}

/// 🎞️ Exact inverse of [`encode_presence_tool_run`] over a standalone body: the peer decoder's limits
/// (state tag, `u16` stage, exact-integer units, `completed ≤ total`) and no trailing bytes.
pub fn decode_presence_tool_run(bytes: &[u8]) -> Result<PresenceToolRun, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence tool run bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let tool_run = reader.tool_run()?;
    if reader.position != bytes.len() { return Err(reader.malformed("presence tool run", "trailing bytes")); }
    Ok(tool_run)
}

/// 🎞️ Exact inverse of [`encode_presence_history_edit`] over a standalone body: the peer decoder's limits (stage tag,
/// `u32` drafts) and no trailing bytes.
pub fn decode_presence_history_edit(bytes: &[u8]) -> Result<PresenceHistoryEdit, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence history edit bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let history_edit = reader.history_edit()?;
    if reader.position != bytes.len() { return Err(reader.malformed("presence history edit", "trailing bytes")); }
    Ok(history_edit)
}

/// 🎯️ Exact, allocation-bounded inverse of [`encode_presence_peer`]. Unknown flags,
/// noncanonical varints, non-finite view values, hostile collection counts, and trailing bytes are
/// rejected before a peer can cross a network authority boundary.
pub async fn decode_presence_peer(bytes: &[u8]) -> Result<PresencePeer, semio_framework_replication_error::ProtocolError> {
    let limits = PRESENCE_PEER_WIRE_LIMITS_V1;
    if bytes.len() > limits.maximum_entry_bytes { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence peer entry bytes")); }
    let mut reader = PresencePeerReader { bytes, position: 0, limits };
    let actor = reader.text("presence peer actor")?;
    let flags = reader.varint("presence peer flags")?;
    if flags >> 15 != 0 { return Err(reader.malformed("presence peer flags", format!("unknown flag bits set: {flags:#x}"))); }
    let connected_at = reader.varint("presence peer connected at")?;
    if connected_at > limits.maximum_connected_at_ms { return Err(semio_framework_replication_error::ProtocolError::LimitExceeded("presence peer connected at")); }
    let connected_at_ms = connected_at as i64;
    let label = if flags & (1 << 0) != 0 { Some(reader.text("presence peer label")?) } else { None };
    let presence_pack = if flags & (1 << 1) != 0 { Some(reader.blob("presence peer pack")?) } else { None };
    let user_id = if flags & (1 << 2) != 0 { Some(reader.text("presence peer user id")?) } else { None };
    let role = if flags & (1 << 3) != 0 { Some(reader.text("presence peer role")?) } else { None };
    let drag_ghost_json = if flags & (1 << 4) != 0 { Some(reader.text("presence peer drag ghost")?) } else { None };
    let interaction = if flags & (1 << 5) != 0 { Some(reader.interaction()?) } else { None };
    let color = if flags & (1 << 6) != 0 { Some(reader.byte("presence peer color")?) } else { None };
    let surface = if flags & (1 << 7) != 0 { Some(reader.text("presence peer surface")?) } else { None };
    let views = if flags & (1 << 8) != 0 { reader.views()? } else { Vec::new() };
    let ui = if flags & (1 << 9) != 0 { Some(reader.ui()?) } else { None };
    let tool_run = if flags & (1 << 10) != 0 { Some(reader.tool_run()?) } else { None };
    let principal_kind = if flags & (1 << 11) != 0 { Some(reader.principal_kind()?) } else { None };
    let active_tool = if flags & (1 << 12) != 0 { Some(reader.text("presence peer active tool")?) } else { None };
    let history_edit = if flags & (1 << 13) != 0 { Some(reader.history_edit()?) } else { None };
    let typing = if flags & (1 << 14) != 0 { reader.typing()? } else { Vec::new() };
    if reader.position != bytes.len() { return Err(reader.malformed("presence peer", "trailing bytes")); }
    Ok(PresencePeer { actor, connected_at_ms, label, presence_pack, user_id, role, drag_ghost_json, interaction, color, surface, views, ui, tool_run, principal_kind, active_tool, history_edit, typing })
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️presence-codec/🦀️.rs"]
mod presence_codec_tests;


//#endregion 🔖️Presence

//#region 🔖️PresenceInteraction
/// 📡️ One peer's interaction roster for one app instance, mirrored onto `PresencePeer.interaction`
/// (bit 5) on the heartbeat — typed (not app-opaque `presence_pack`) so the Shell renders every peer's
/// selection/hover generically. Only explicit ids broadcast; receivers expand transitive closures via
/// their own topology.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PresenceInteraction {
    pub app_id: String,
    pub domains: Vec<PresenceDomain>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. This crate's own
/// hand-rolled binary codec (`🔖️PresenceInteractionCodec` below) is the real wire path; this impl
/// exists for capability parity with the type's former `Serialize`/`Deserialize`.
impl semio_framework_value::ToValue for PresenceInteraction {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![("appId".to_string(), semio_framework_value::ToValue::to_value(&self.app_id)), ("domains".to_string(), semio_framework_value::ToValue::to_value(&self.domains))])
    }
}
impl semio_framework_value::FromValue for PresenceInteraction {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceInteraction, found {value:?}")));
        };
        let mut out = PresenceInteraction::default();
        for (key, entry) in fields {
            match key.as_str() {
                "appId" => out.app_id = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("appId"))?,
                "domains" => out.domains = <Vec<PresenceDomain> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("domains"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

/// 📡️ One domain's broadcast slice of `PresenceInteraction` — the peer-facing mirror of a domain's
/// `DomainSelection`/`DomainHover`, flattened to raw explicit ids (no transitive expansion on the wire).
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PresenceDomain {
    pub domain: String,
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for PresenceDomain {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::DslValue::object(vec![
            ("domain".to_string(), semio_framework_value::ToValue::to_value(&self.domain)),
            ("granularity".to_string(), semio_framework_value::ToValue::to_value(&self.granularity)),
            ("selected".to_string(), semio_framework_value::ToValue::to_value(&self.selected)),
            ("hovered".to_string(), semio_framework_value::ToValue::to_value(&self.hovered)),
        ])
    }
}
impl semio_framework_value::FromValue for PresenceDomain {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceDomain, found {value:?}")));
        };
        let mut out = PresenceDomain::default();
        for (key, entry) in fields {
            match key.as_str() {
                "domain" => out.domain = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("domain"))?,
                "granularity" => out.granularity = <String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("granularity"))?,
                "selected" => out.selected = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("selected"))?,
                "hovered" => out.hovered = <Vec<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("hovered"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceInteractionCodec
// 🎯️ Binary codec for the two structs above — what `encode_presence_peer`/`decode_presence_peer`
// (in the `🔖️Presence` region) call for bit 5. Self-delimiting throughout (varint counts, exactly
// `write_vec_bytes`/`write_vec_envelope`'s own convention in `🔖️Combinators` up top), so no outer
// length prefix is needed around the whole payload.
async fn write_vec_str(out: &mut Vec<u8>, values: &[String]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        semio_framework_replication_wire_core::write_str(out, value);
    }
}

async fn read_vec_str(bytes: &[u8], pos: &mut usize) -> Result<Vec<String>, semio_framework_replication_error::ProtocolError> {
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(semio_framework_replication_wire_core::read_str(bytes, pos)?);
    }
    Ok(out)
}

async fn encode_presence_domain(domain: &PresenceDomain, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &domain.domain);
    semio_framework_replication_wire_core::write_str(out, &domain.granularity);
    write_vec_str(out, &domain.selected).await;
    write_vec_str(out, &domain.hovered).await;
}

async fn decode_presence_domain(bytes: &[u8], pos: &mut usize) -> Result<PresenceDomain, semio_framework_replication_error::ProtocolError> {
    Ok(PresenceDomain { domain: semio_framework_replication_wire_core::read_str(bytes, pos)?, granularity: semio_framework_replication_wire_core::read_str(bytes, pos)?, selected: read_vec_str(bytes, pos).await?, hovered: read_vec_str(bytes, pos).await? })
}

/// 🎯️ Encodes one `PresenceInteraction` — `pub` (ticket 26/08/17/SHARED-PRESENCE-SESSION-
/// COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4): guests never enable the kernel's `sync` feature, and
/// `VcsArtifactApp` (the plugin ABI's presence adoption path) must be able to call this directly.
pub async fn encode_presence_interaction(interaction: &PresenceInteraction, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &interaction.app_id);
    semio_framework_replication_wire_core::write_varint_u64(out, interaction.domains.len() as u64);
    for domain in &interaction.domains {
        encode_presence_domain(domain, out).await;
    }
}

/// 🎯️ Inverse of [`encode_presence_interaction`] — see its doc for why this is `pub`.
pub async fn decode_presence_interaction(bytes: &[u8], pos: &mut usize) -> Result<PresenceInteraction, semio_framework_replication_error::ProtocolError> {
    let app_id = semio_framework_replication_wire_core::read_str(bytes, pos)?;
    let count = semio_framework_replication_wire_core::read_varint_u64(bytes, pos)?;
    let mut domains = Vec::with_capacity(count as usize);
    for _ in 0..count {
        domains.push(decode_presence_domain(bytes, pos).await?);
    }
    Ok(PresenceInteraction { app_id, domains })
}
//#endregion 🔖️PresenceInteractionCodec

//#region 🔖️Assemble
/// 📡️ Assembles `PresencePeer.interaction` from local `InteractionState` plus each domain's
/// declared hover/selection behavior — the ONE place this logic lives, so every app broadcasts
/// selection+hover with ZERO app-side code (call this wherever a `PresenceHeartbeat`'s `peer` is
/// built, right before `presence_to_bytes`/`encode_presence_peer`, at the same cadence cursor updates
/// already get throttled at — this fn is pure/stateless, so it invents no throttle of its own).
/// Forwards each domain's already-computed `ids` verbatim (never re-derives a closure itself) —
/// EXPLICIT ids only cross the wire; a receiver re-expands any transitive closure via its own
/// `DomainTopology`. Only the `"pointer"` hover channel ever broadcasts (any other channel, e.g. a
/// drag-only one, stays local); a domain whose `HoverSpec::broadcast`/`SelectionSpec::broadcast` is
/// `false` contributes nothing for that half. A domain that ends up with both halves empty is omitted
/// from `domains` entirely.
///
/// 🎯️ Moved here (ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION
/// C7.4) from `store_sync` — guests never enable the kernel's `sync` feature, and `VcsArtifactApp`
/// (the plugin ABI's presence-adoption path, §C7.6) must be able to call this pure fn without pulling
/// in the whole actor layer.
pub async fn assemble_presence_interaction(app_id: &str, state: &InteractionState, hover_specs: &BTreeMap<String, HoverSpec>, selection_specs: &BTreeMap<String, SelectionSpec>) -> PresenceInteraction {
    let mut domain_ids: std::collections::BTreeSet<&String> = std::collections::BTreeSet::new();
    domain_ids.extend(state.selection.keys());
    domain_ids.extend(state.hover.keys());

    let mut domains = Vec::new();
    for domain_id in domain_ids {
        let selected = if selection_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.selection.get(domain_id).map(|selection| selection.ids.clone()).unwrap_or_default() } else { Vec::new() };
        let hovered = if hover_specs.get(domain_id).is_some_and(|spec| spec.broadcast) { state.hover.get(domain_id).filter(|hover| hover.channel == "pointer").map(|hover| hover.ids.clone()).unwrap_or_default() } else { Vec::new() };
        if selected.is_empty() && hovered.is_empty() {
            continue;
        }
        let granularity = state.active_granularity.get(domain_id).cloned().unwrap_or_default();
        domains.push(PresenceDomain { domain: domain_id.clone(), granularity, selected, hovered });
    }

    PresenceInteraction { app_id: app_id.to_string(), domains }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️assemble-presence-interaction/🦀️.rs"]
mod assemble_presence_interaction_tests;
//#endregion 🔖️Assemble
//#endregion 🔖️PresenceInteraction

#[path = "../../🚪️io/👥️presence/🔍️borrowed/🦀️.rs"]
pub mod borrowed;
pub use borrowed::{borrow_presence_metadata, BorrowedPresenceMetadata, BorrowedPresenceInteraction, BorrowedPresenceDomain, BorrowedPresenceStrings};

#[path = "../../👕️peer-overlay/🦀️.rs"]
pub mod peer_overlay;
pub use peer_overlay::*;
