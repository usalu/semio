//! 🔖️ Constants, examples and scene: the controller and granularity constants, the shipped example json, the scene and interaction snapshot and the selection writes. See [`super`].

use super::*;
use std::collections::HashSet;

pub const PUZZLE2D_PLAY_CONTROLLER_ID: &str = "puzzle2d-play";

pub const PUZZLE2D_PLAY_SURFACE_ID: &str = "puzzle2d.play.composite";

pub const PUZZLE2D_BOARD_SNAPSHOT_SCHEMA: &str = "board.ports.directed.v1";

pub const PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID: &str = crate::examples::puzzle2d::concrete_forest::ID;

pub const PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID: &str = crate::examples::puzzle2d::nakagin_capsule_tower::ID;

/// 🪟️ The three canvas pane KIND ids — a different id space from the window body keys (see
/// `🎮️commands/🎲️apply-board-events`'s `PUZZLE2D_WINDOW_BODY_KEYS`): these key utilities, engagements and measures.
pub const PUZZLE2D_PANES: [&str; 3] = [crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_DETAIL_WINDOW_KIND_ID, crate::editor::puzzle2d::config::PUZZLE2D_SELECTION_WINDOW_KIND_ID];

/// 🗨️ The declared dialog `openAddNodeDialog` opens — its submit action is `addNode`, whose `kind`
/// select is built from the shipped examples' LIVE node kinds (never a literal `"node"` option no
/// catalog declares, puzzle3d's own long-standing add-object-dialog bug).
pub const PUZZLE2D_ADD_NODE_DIALOG_ID: &str = "addNode";

/// 🗂️ Fixed ceiling on the node-kind rows the `addNode` arg form and the Add Node dialog offer — the
/// manifest is minted once per process, so this select is built eagerly and must stay bounded however
/// wide a catalog a future example declares.
pub const PUZZLE2D_NODE_KIND_OPTIONS_MAX: usize = 64;

/// 🖱️ The one hover channel the `vortex` domain declares.
pub const PUZZLE2D_HOVER_CHANNEL: &str = "pointer";

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the one interaction domain this app
/// declares — the deleted `Puzzle2dConfig::selected_ids` flat bag (nodes and their nested handles
/// alike) collapses into one framework-owned domain, one flat granularity (no real parent/child
/// structure was ever modeled for it).
pub const PUZZLE2D_INTERACTION_DOMAIN: &str = "vortex";

pub const PUZZLE2D_GRANULARITY_NODE: &str = "node";

pub const PUZZLE2D_GRANULARITY_EDGE: &str = "edge";

pub const PUZZLE2D_GRANULARITY_HANDLE: &str = "handle";

/// 🎯️ The fill-constraining board rectangle granularity — the 2d twin of puzzle3d's
/// `PUZZLE3D_GRANULARITY_TARGET_VOLUME`.
pub const PUZZLE2D_GRANULARITY_TARGET_REGION: &str = "targetRegion";

pub(super) const BOARD_DEFAULT_WIDTH: u32 = 1024;

pub(super) const BOARD_DEFAULT_HEIGHT: u32 = 768;

/// 🧲️ Edges ONE moved node's drop may auto-connect. A node carries a handful of handles, so this
/// ceiling is what a real drop can reach while keeping the drop's `extent` a small constant per moved
/// node instead of puzzle3d's whole-document `objects × 66` bound.
pub const PUZZLE2D_PROXIMITY_CONNECT_MAX: usize = 8;

/// 🧲️ Edges ONE gesture's auto-connect may create across every node it moved. A whole-selection move
/// therefore prices a fixed 64-edge budget instead of scaling with the selection, which is what keeps
/// a Nakagin-sized drag inside the shared `PUZZLE_COMMAND_WORK_ITEMS` envelope.
pub const PUZZLE2D_PROXIMITY_GESTURE_MAX: usize = 64;

/// 🧲️ Widest auto-connect radius the settings stepper may reach — twenty default node radii, past
/// which "nearby" stops meaning anything on a board.
pub const PUZZLE2D_PROXIMITY_RADIUS_MAX: f64 = 480.0;

/// 🧵 The shipped examples' document JSON, decoded from their authored DSL EXACTLY ONCE per process
/// — the same `LazyLock` shape `🧊️3d` and `🖐️5d` use for their own `*_EXAMPLE_JSON`.
///
/// 🐛️ These two accessors used to call `ExampleSource::document_json()` per call. That was a cheap
/// `String` clone while the bodies were inline, but every puzzle example is now
/// `ExampleSource::deferred`, and a deferred body's `document_json()` RUNS ITS PRODUCER — a full
/// parse of the authored `.dsl.semio` (93 779 B for nakagin) plus a JSON re-serialisation — on every
/// single call. `puzzle2d_node_kind_arg()` is built twice (the `addNode` arg form and the Add Node
/// dialog) and each build reads both examples, so ASSEMBLING THE APP DEFINITION decoded ~384 KB of
/// DSL where 96 KB is needed. `AppDefinition` assembly happens on the `describe()` path, inside the
/// owned interpreter, under a 1 800 s guest epoch — this was pure multiplier on the cost that put
/// `🧩️puzzle` over that epoch.
static CONCRETE_FOREST_EXAMPLE_JSON: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| crate::examples::puzzle2d::concrete_forest::SOURCE.document_json());

static NAKAGIN_EXAMPLE_JSON: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE.document_json());

/// 🧵 Reuses the manifest's canonical, initialization-owned example payload so an interactive
/// command never repeats DSL decoding inside its bounded worker step.
pub fn concrete_forest_example_json() -> String {
    CONCRETE_FOREST_EXAMPLE_JSON.clone()
}

pub fn nakagin_example_json() -> String {
    NAKAGIN_EXAMPLE_JSON.clone()
}

/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare snapshot json) with
/// the app's view config. It is never persisted — the `VcsArtifactApp` store owns the snapshot as its
/// projection and `Puzzle2dConfig` owns the runtime — but rebuilding it per call lets the panel,
/// canvas and engagement helpers keep one `&Puzzle2dScene` signature.
pub struct Puzzle2dScene {
    pub board_snapshot: Value,
    pub runtime: Puzzle2dPlayRuntime,
    /// 🧰️ The host-owned active utility for this render/mutation.
    pub active_utility: String,
    /// 🕹️ The framework-owned `vortex` selection this render/mutation reads — the board engine's own
    /// selection echoes back through it, so the panes, the inspector and the context menu all agree.
    pub interaction: Puzzle2dInteractionSnapshot,
}

/// 🕹️ One render's read of the live `vortex` domain: the selected ids (any granularity), the `"pointer"` hover and
/// the ids the open time-travel draft references — the 2d twin of `Puzzle3dInteractionSnapshot`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Puzzle2dInteractionSnapshot {
    pub granularity: String,
    pub selected: Vec<String>,
    pub hovered: Vec<String>,
    pub referenced: Vec<String>,
}

impl Puzzle2dInteractionSnapshot {
    pub fn from_interaction(interaction: &InteractionView<'_>) -> Self {
        let selection = interaction.selection(PUZZLE2D_INTERACTION_DOMAIN);
        let hover = interaction.hover(PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_HOVER_CHANNEL);
        let leftover_ids = interaction.leftover_selected_ids();
        let selected = if selection.ids.is_empty() { leftover_ids } else { selection.ids.clone() };
        let granularity = if !selection.granularity.is_empty() { selection.granularity.clone() } else if !selected.is_empty() { PUZZLE2D_GRANULARITY_NODE.to_string() } else { String::new() };
        Self { granularity, selected, hovered: hover.ids.clone(), referenced: interaction.draft_references(PUZZLE2D_INTERACTION_DOMAIN).to_vec() }
    }

    /// 🕹️ The retained-reducer twin over the raw `InteractionState`/hover map a retained job is handed.
    pub fn from_state(state: &protocol::InteractionState, hover: &semio_framework_plugin::app::InteractionHoverState) -> Self {
        let selection = state.selection.get(PUZZLE2D_INTERACTION_DOMAIN);
        let hovered = hover.get(PUZZLE2D_INTERACTION_DOMAIN).filter(|hover| hover.channel == PUZZLE2D_HOVER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
        let leftover_ids: Vec<String> = state.selection.values().flat_map(|selection| selection.ids.iter().cloned()).collect();
        let selected = selection.filter(|selection| !selection.ids.is_empty()).map(|selection| selection.ids.clone()).unwrap_or(leftover_ids);
        let granularity = selection.map(|selection| selection.granularity.clone()).filter(|granularity| !granularity.is_empty()).unwrap_or_else(|| if selected.is_empty() { String::new() } else { PUZZLE2D_GRANULARITY_NODE.to_string() });
        Self { granularity, selected, hovered, referenced: Vec::new() }
    }

    pub fn selected_ids(&self) -> &[String] {
        &self.selected
    }

    pub fn selection_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.selected)
    }

    /// 🔗️ The ids the board paints highlighted while a time-travel draft targets them, as the scene's JSON id array.
    pub fn referenced_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.referenced)
    }

    /// 🐁️ The one `"pointer"`-channel hover id this render paints, whatever granularity resolved it
    /// (a node, a `node:handle` or an edge) — the board scene's `hovered_id` and the 2d twin of
    /// puzzle3d's `hovered_object_id`. A canvas pointermove, an outliner row and a catalogue row all
    /// write the same framework-owned hover, so every pane paints the same id.
    pub fn hovered_id(&self) -> Option<String> {
        self.hovered.first().cloned()
    }
}

/// 🕹️ Classifies board ids into `(granularity, id)` targets by document membership — a node id, an
/// edge id, or a `node:handle` id nested under a node — so one engine `select` event becomes one
/// framework selection write whatever it picked. Ids the document does not carry are kept as nodes:
/// the framework prunes them on the next document change, and dropping them here would silently
/// unselect a just-created entity the engine already painted.
pub fn puzzle2d_selection_targets(snapshot: &Value, ids: &[String]) -> Vec<InteractionTarget> {
    let nodes = board_snapshot_nodes(snapshot);
    let node_ids: HashSet<&str> = nodes.iter().filter_map(|node| node.get("id").and_then(Value::as_str)).collect();
    let edge_ids: HashSet<&str> = board_snapshot_edges(snapshot).iter().filter_map(|edge| edge.get("id").and_then(Value::as_str)).collect();
    let handle_ids: HashSet<&str> = nodes.iter().filter_map(|node| node.get("handles").and_then(Value::as_array)).flatten().filter_map(|handle| handle.get("id").and_then(Value::as_str)).collect();
    ids.iter()
        .map(|id| {
            let granularity = if node_ids.contains(id.as_str()) {
                PUZZLE2D_GRANULARITY_NODE
            } else if edge_ids.contains(id.as_str()) {
                PUZZLE2D_GRANULARITY_EDGE
            } else if handle_ids.contains(id.as_str()) {
                PUZZLE2D_GRANULARITY_HANDLE
            } else {
                PUZZLE2D_GRANULARITY_NODE
            };
            InteractionTarget { granularity: granularity.into(), id: id.clone() }
        })
        .collect()
}

/// 🕹️ The one selection write every 2d reducer expresses "select exactly these" through.
pub fn puzzle2d_selection_write(snapshot: &Value, ids: &[String]) -> semio_framework_plugin::InteractionWrite {
    semio_framework_plugin::InteractionWrite { domain: PUZZLE2D_INTERACTION_DOMAIN.into(), targets: puzzle2d_selection_targets(snapshot, ids), merge: MergeMode::Replace }
}

/// 🕹️ Empties the live selection: a SUBTRACTIVE write of exactly the ids that are selected (a `Replace`
/// with no targets selects nothing and therefore changes nothing). `snapshot` is the document BEFORE the
/// deletion, so a just-deleted id still classifies to its granularity.
pub fn puzzle2d_clear_selection_write(snapshot: &Value, selected: &[String]) -> Option<semio_framework_plugin::InteractionWrite> {
    if selected.is_empty() {
        return None;
    }
    Some(semio_framework_plugin::InteractionWrite { domain: PUZZLE2D_INTERACTION_DOMAIN.into(), targets: puzzle2d_selection_targets(snapshot, selected), merge: MergeMode::Subtractive })
}

/// 🧷️ Writes `member` under `key` of a JSON object `Value`; any other `Value` is left untouched.
pub fn set_member(target: &mut Value, key: &str, member: Value) {
    if let Some(object) = target.as_object_mut() {
        object.insert(key, member);
    }
}

pub fn empty_board_snapshot() -> Value {
    json!({
        "schema": PUZZLE2D_BOARD_SNAPSHOT_SCHEMA,
        "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
        "nodes": [],
        "edges": []
    })
}

/// 📂️ The artifact's load path for a whole-document import: an `Effect::LoadDocument` (outside history) carrying the
/// `document` pack and a fresh, edit-free op-log. A whole-document replacement is never a mutation and is never stated as
/// the difference of two boards. The spr is built from bytes (`store::os_store::text_format::empty_document_spr`), never from a live envelope,
/// whose bounded retirement authority would trap the guest when dropped.
pub fn puzzle2d_load_document_effect(document: &crate::Puzzle2dSnapshot) -> Effect {
    let pack = <crate::Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("puzzle2d", crate::standards::v1::subsets::any::schema::records::PUZZLE_2D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}

pub fn puzzle2d_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: builds a framework `interactionSelect`
/// action targeting one `(granularity, id)` pair in the `vortex` domain — replaces the deleted
/// `setSelection` action builders.
pub fn puzzle2d_interaction_select(granularity: &str, id: &str) -> ActionDescriptor {
    let targets = semio_framework_pack_json::to_json_string(&vec![InteractionTarget { granularity: granularity.into(), id: id.into() }]);
    puzzle2d_action(INTERACTION_SELECT_ACTION_ID, Some(semio_framework::dsl_value!({ "domainId": PUZZLE2D_INTERACTION_DOMAIN, "targets": targets, "merge": "replace", "method": "pick" })))
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain declaration —
/// the deleted `Puzzle2dConfig::selected_ids` flat bag collapses into one framework-owned domain,
/// whose topology the app supplies (`interaction_topology`): nodes, their handles, edges and target regions.
pub fn puzzle2d_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: PUZZLE2D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Vortex", "Vortex"),
        granularities: vec![
            GranularityDefinition { id: PUZZLE2D_GRANULARITY_NODE.into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle-dot".into() },
            GranularityDefinition { id: PUZZLE2D_GRANULARITY_EDGE.into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "link".into() },
            GranularityDefinition { id: PUZZLE2D_GRANULARITY_HANDLE.into(), label: LocalizedLabel::native("Handle", "Anschluss"), icon_id: "target".into() },
        ],
        hierarchy: HierarchyProvider::Topology,
        hover: HoverSpec { enabled: true, transitive: false, channels: vec![PUZZLE2D_HOVER_CHANNEL.into()], broadcast: true },
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🧰️ Resolves the canonical host-owned utility for the current window.
pub fn puzzle2d_active_utility(view_state: Option<&semio_framework_plugin::ViewModel>) -> &str {
    view_state.and_then(|view| view.active_utility_id.as_deref()).filter(|utility| !utility.is_empty()).unwrap_or(crate::editor::puzzle2d::config::PUZZLE2D_SELECT_UTILITY_ID)
}

/// 🎥️ The camera lives on `Puzzle2dConfig` — session-only view state, never a snapshot field.
pub fn runtime_camera(runtime: &Puzzle2dPlayRuntime) -> (f64, f64, f64) {
    (runtime.camera_x, runtime.camera_y, runtime.camera_zoom)
}

pub fn board_snapshot_nodes(snapshot: &Value) -> &[Value] {
    snapshot.get("nodes").and_then(|value| value.as_array()).map_or(&[][..], |values| values.as_slice())
}

pub fn board_snapshot_edges(snapshot: &Value) -> &[Value] {
    snapshot.get("edges").and_then(|value| value.as_array()).map_or(&[][..], |values| values.as_slice())
}

pub fn kind_catalog_entries<'a>(snapshot: &'a Value, key: &str) -> Option<&'a [Value]> {
    snapshot.get("meta").and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.as_slice())
}
