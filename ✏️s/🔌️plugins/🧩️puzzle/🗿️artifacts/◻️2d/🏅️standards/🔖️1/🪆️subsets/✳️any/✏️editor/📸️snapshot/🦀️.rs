//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
use crate::{Puzzle2dSnapshot,Puzzle2dMutation};
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_pack_json::Value as HostValue;

//#region 🔖️PlaySnapshot
/// 🌱️ Immutable typed puzzle authority and its deferred first-party intrinsic view.
#[derive(Debug)]
pub struct Puzzle2dPlaySnapshot {
    typed: std::sync::Arc<Puzzle2dSnapshot>,
    value: std::sync::OnceLock<std::sync::Arc<HostValue>>,
}

impl Puzzle2dPlaySnapshot {
    /// 🎯️ Builds an immutable root from an admitted typed snapshot.
    pub fn new(typed: Puzzle2dSnapshot) -> Self {
        Self::from_typed(typed)
    }

    /// 🧬️ A root produced by typed mutation application; its `Value` projection is deferred.
    pub(crate) fn from_typed(typed: Puzzle2dSnapshot) -> Self {
        Self { typed: std::sync::Arc::new(typed), value: std::sync::OnceLock::new() }
    }

    /// 👁️ The intrinsic view, materialized once per immutable root.
    pub fn value(&self) -> &HostValue {
        self.value.get_or_init(|| std::sync::Arc::new(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self.typed.as_ref())))).as_ref()
    }

    /// 🤝️ The legacy play projection as a shared root — what an owned tool event carries without copying the document.
    pub fn shared_value(&self) -> std::sync::Arc<HostValue> {
        std::sync::Arc::clone(self.value.get_or_init(|| std::sync::Arc::new(semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self.typed.as_ref())))))
    }

    /// 🫱️ The typed authority as a shared root, for the same reason.
    pub fn shared_typed(&self) -> std::sync::Arc<Puzzle2dSnapshot> {
        std::sync::Arc::clone(&self.typed)
    }

    /// 🧬️ The typed authority, without materializing the legacy projection.
    pub fn typed(&self) -> &Puzzle2dSnapshot {
        self.typed.as_ref()
    }
}

impl Clone for Puzzle2dPlaySnapshot {
    fn clone(&self) -> Self {
        let value = std::sync::OnceLock::new();
        if let Some(projected) = self.value.get() {
            let _ = value.set(std::sync::Arc::clone(projected));
        }
        Self { typed: std::sync::Arc::clone(&self.typed), value }
    }
}

impl PartialEq for Puzzle2dPlaySnapshot {
    fn eq(&self, other: &Self) -> bool {
        self.typed == other.typed
    }
}

/// 🌱️ Encodes the admitted typed authority through the native value port.
impl semio_framework_value::ToValue for Puzzle2dPlaySnapshot {
    fn to_value(&self) -> semio_framework_value::DslValue {
        semio_framework_value::ToValue::to_value(self.typed())
    }
}

impl semio_framework_value::FromValue for Puzzle2dPlaySnapshot {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Puzzle2dSnapshot as semio_framework_value::FromValue>::from_value(value).map(Self::from_typed)
    }
}

/// 🧒️ Composition view of the play snapshot: a puzzle 2d document owns no child artifacts.
impl semio_framework_schema_composition::ArtifactCompositionFields for Puzzle2dPlaySnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl MutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn apply(&self, projection: &Puzzle2dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dPlaySnapshot> {
        MutationDiff::<Puzzle2dSnapshot>::apply(self, projection.typed(), capability).map(Puzzle2dPlaySnapshot::from_typed).map_err(|error| error.under(["document"]))
    }
    fn absorb(&mut self, other: Self) {
        MutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }
}

impl DiffAlgebra<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dPlaySnapshot) -> Self {
        DiffAlgebra::<Puzzle2dSnapshot>::inverse(self, base.typed())
    }
    fn is_empty(&self) -> bool {
        DiffAlgebra::<Puzzle2dSnapshot>::is_empty(self)
    }
}

impl Mutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type Diff = Puzzle2dDiff;

    /// 🧷️ Not hand-written — see the identical note on `impl Mutation<Value>` above. The metadata
    /// is projection-independent, so this forwards to the derive's own table too, same as
    /// `may_emit_foreign_steps` already does immediately below.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = <Self as Mutation<Puzzle2dSnapshot>>::DESCRIPTORS;
    const INPUT_SCHEMAS: &'static [&'static str] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMAS;
    const INPUT_SCHEMA_DOCUMENTS: &'static [&'static [&'static str]] = <Self as Mutation<Puzzle2dSnapshot>>::INPUT_SCHEMA_DOCUMENTS;

    fn input_schema(&self) -> Option<&'static str> {
        Mutation::<Puzzle2dSnapshot>::input_schema(self)
    }

    fn payload_value(&self) -> semio_framework_value::DslValue {
        Mutation::<Puzzle2dSnapshot>::payload_value(self)
    }

    fn with_payload_value(&self, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        Mutation::<Puzzle2dSnapshot>::with_payload_value(self, value)
    }

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        <Self as Mutation<Puzzle2dSnapshot>>::descriptor(self)
    }

    fn inverse_rows(&self) -> usize {
        Mutation::<Puzzle2dSnapshot>::inverse_rows(self)
    }

    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        Mutation::<Puzzle2dSnapshot>::diff(self, projection.typed())
    }

    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        Mutation::<Puzzle2dSnapshot>::inverse(self, projection.typed())?
    
    })
}
    fn may_emit_foreign_steps(&self) -> bool {
        Mutation::<Puzzle2dSnapshot>::may_emit_foreign_steps(self)
    }
    fn from_payload_value(kind: &str, value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        <Self as Mutation<Puzzle2dSnapshot>>::from_payload_value(kind, value)
    }
    fn conflict_target(&self) -> Vec<String> {
        Mutation::<Puzzle2dSnapshot>::conflict_target(self)
    }
}

/// 🪪️ `kinds`/`semantics`/`label`/`target` are projection-independent (the derive-generated
/// `SemanticMutation<Puzzle2dSnapshot>` impl above never actually reads `Puzzle2dSnapshot` data in
/// any of the four), so this bridges the same vocabulary onto `Puzzle2dPlaySnapshot` by forwarding
/// straight through — the `SemanticMutation` twin of the `Mutation<Puzzle2dPlaySnapshot>` bridge
/// immediately above, needed so `.editor_mutation_roster::<Puzzle2dPlayApp>()` can register this
/// dialect's real semantic vocabulary against the play app's own `Snapshot` type.
impl protocol::SemanticMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::kinds()
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::semantics(self)
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::label(self)
    }
    fn target(&self) -> Vec<String> {
        <Self as protocol::SemanticMutation<Puzzle2dSnapshot>>::target(self)
    }
}
//#endregion 🔖️PlaySnapshot

/// ♻️ The typed authority retires through its own owner, then the deferred first-party host view, each as its own bounded cursor turn.
impl semio_framework_value::retirement::RetireOwned for Puzzle2dPlaySnapshot {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let Self { typed, value } = self;
        semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::deferred(typed), semio_framework_value::retirement::deferred(value.into_inner())])
    }

    fn retirement_birth_bytes(&self) -> Option<usize> {
        semio_framework_value::retirement::sequence_birth_bytes(&[semio_framework_value::retirement::deferred_birth_bytes_for(&self.typed), semio_framework_value::retirement::deferred_birth_bytes::<Option<std::sync::Arc<HostValue>>>()])
    }

    fn controlled_retirement_supported() -> bool {
        true
    }
}

//#region 🚪️Codecs
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::{SqliteDatabase, SqliteSnapshotControl, SnapshotEncoding}};
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v1::subsets::any::io::sqlite::snapshot::{semantic_cells, restore_authority};

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle2dSnapshot`'s derived record
/// layout and pack-schema identity.
impl pack::snapshot::ArtifactPack for Puzzle2dPlaySnapshot {
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactPack>::record_spec()
    }
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle2dPlaySnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::serialization::ArtifactSqliteSnapshotHost>::sqlite_codec())
    }
}

impl pack::snapshot::ArtifactDsl for Puzzle2dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle2d-play";

    fn envelope_id() -> &'static str {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id()
    }

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map(Self::from_typed)
    }

    fn print_dsl(&self) -> String {
        <Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(self.typed())
    }
}

/// 🎮️ Persists the editor's typed authority through the declared board schema.
impl ArtifactSqliteSnapshotHost for crate::Puzzle2dPlaySnapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{
  let limits=control.limits();semantic_cells::extent(limits)?;
  store::decode_sqlite_snapshot_record_native(payload,<Puzzle2dSnapshot as pack::snapshot::ArtifactDsl>::envelope_id(),Puzzle2dSnapshot::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {
   semantic_cells::admit_record(record,limits,native)?;
   native.charge(size_of::<Puzzle2dSnapshot>()+2*size_of::<usize>())?;
   Puzzle2dSnapshot::__dsl_from_record_controlled(record,native).map(Self::from_typed)
  })(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
 }
 fn preflight_sqlite_snapshot_encoding(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{self.typed().preflight_sqlite_snapshot_encoding(encoding,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,control,native_owner)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,database,control)}
}

impl pack::snapshot::ArtifactSqliteSnapshot for crate::Puzzle2dPlaySnapshot {
const SQLITE_SCHEMA:&'static str=<Puzzle2dSnapshot as ArtifactSqliteSnapshotHost>::SQLITE_SCHEMA;
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.typed().to_sqlite_database(control)}
fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let bytes=size_of::<Puzzle2dSnapshot>().checked_add(2*size_of::<usize>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Play authority allocation overflow"))?;
  restore_authority(database,control,bytes).map(Self::from_typed)
 }
}
//#endregion 🚪️Codecs

use crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC;
use {crate::editor::puzzle2d::commands::apply_board_events, crate::editor::puzzle2d::commands::set_brush_kind_weights, crate::editor::puzzle2d::commands::set_grid_factor, crate::editor::puzzle2d::commands::set_grid_snap_enabled, crate::editor::puzzle2d::commands::set_grid_visible, crate::editor::puzzle2d::commands::set_suggestion_offset};
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use {crate::editor::puzzle2d::engine::handle_position_on_circle, crate::editor::puzzle2d::engine::handle_position_on_rectangle, crate::editor::puzzle2d::engine::BoardHost, crate::editor::puzzle2d::engine::Point, crate::editor::puzzle2d::engine::Puzzle2dExtension};
use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::select as select_utility;
use crate::editor::puzzle2d::modes::edit::windows::selection;
use {crate::editor::puzzle2d::panels::artifact, crate::editor::puzzle2d::panels::inspection};
use crate::editor::puzzle2d::terminology::puzzle2d_localized;
use crate::editor::puzzle2d::window::self;
use crate::editor::puzzle2d::snapshot::Puzzle2dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::editor::puzzle2d::recorder::Puzzle2dRecorder;
use semio_framework::kernel::UiDirtyScope;
use {semio_framework_plugin::kernel::ClipboardError, semio_framework_plugin::kernel::ClipboardFragment, semio_framework_plugin::kernel::Effect, semio_framework_plugin::kernel::PastePlacement};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionTarget;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::app::InteractionView;
use {semio_framework_pack_json::json, semio_framework_pack_json::Value};
#[cfg(test)] use semio_framework_pack_json::Object;
use {std::collections::BTreeSet, std::collections::HashSet};
use crate::editor::puzzle2d::{PUZZLE2D_TOOL_JOB_IDS, Puzzle2dActionCtx, recorder, snapshot};

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

const BOARD_DEFAULT_WIDTH: u32 = 1024;

const BOARD_DEFAULT_HEIGHT: u32 = 768;

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
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr("puzzle2d", crate::standards::v1::subsets::any::schema::PUZZLE_2D_SCHEMA));
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
fn puzzle2d_interaction_definition() -> InteractionDefinition {
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
    view_state.and_then(|view| view.active_utility_id.as_deref()).filter(|utility| !utility.is_empty()).unwrap_or(select_utility::UTILITY_ID)
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

fn manifest_catalog_rows(kinds: &[semio_framework_graph_manifest::KindDef]) -> Value {
    Value::Array(
        kinds
            .iter()
            .map(|kind| {
                let mut row = semio_framework_pack_json::Object::new();
                row.insert("id".to_string(), json!(kind.id));
                row.insert("name".to_string(), json!(kind.name));
                if let Some(semio_framework_value::DslValue::Object(presentation)) = kind.presentation.as_ref() {
                    for (key, value) in presentation {
                        row.insert(key.clone(), semio_framework_pack_json::from_dsl_value(value));
                    }
                }
                Value::Object(row)
            })
            .collect(),
    )
}

/// 🗂️ Board engine catalogs for a manifest id. Every shipped puzzle2d document names a
/// `meta.manifestId` and carries no catalogs of its own, so this — not `meta.kindCatalogs` — is where
/// their node/handle kinds actually come from. Each row is the manifest row's `id`/`name` merged with
/// its flattened `presentation`. Port kinds without a `presentation.color` are dropped because the
/// engine rejects a colourless handle kind outright, which would discard the whole catalog push.
pub fn manifest_board_kind_catalogs_json(manifest_id: &str) -> Option<String> {
    let manifest = crate::graph_manifest::manifest_by_id(manifest_id)?;
    let visual_port_kinds: Vec<semio_framework_graph_manifest::KindDef> = manifest.port_kinds.iter().filter(|kind| kind.presentation.as_ref().is_some_and(|p| p.get("color").is_some())).cloned().collect();
    Some(
        json!({
            "handleKinds": manifest_catalog_rows(&visual_port_kinds),
            "wireKinds": manifest_catalog_rows(&manifest.wire_kinds),
            "nodeKinds": manifest_catalog_rows(&manifest.node_kinds),
            "edgeKinds": manifest_catalog_rows(&manifest.edge_kinds),
        })
        .to_string(),
    )
}

fn catalog_row_subset(row: &Value, keys: &[&str]) -> Value {
    let mut out = semio_framework_pack_json::Object::new();
    for key in keys {
        match row.get(*key) {
            Some(value) if !value.is_null() => {
                out.insert((*key).to_string(), value.clone());
            }
            _ => {}
        }
    }
    Value::Object(out)
}

fn catalog_rows_subset(catalogs: &Value, slice: &str, keys: &[&str]) -> Option<Value> {
    let rows = catalogs.get(slice).and_then(Value::as_array)?;
    Some(Value::Array(rows.iter().map(|row| catalog_row_subset(row, keys)).collect()))
}

/// 🗂️ Projects the document's `meta.kindCatalogs` onto the board engine's catalog contract. The
/// document owns `nodes`/`🐙️handles`/`edges`/`wires` ([`Puzzle2dKindCatalogs`]) while
/// `BoardHost::set_board_kind_catalogs_from_json` reads `nodeKinds`/`handleKinds`/`edgeKinds`/`wireKinds`
/// and rejects any row still carrying the document's `label`, so each row is narrowed to the keys the
/// engine actually reads. Without this the engine's `node_kinds` map stays empty and every
/// brush/fill candidate lookup silently yields nothing. A slice absent from the document is left out
/// entirely rather than emitted empty, because the engine reads an omitted array as "leave that
/// slice alone" and an empty one as "clear it".
///
/// Documents that carry no `meta.kindCatalogs` of their own — which is every shipped example, they
/// name a `meta.manifestId` instead — resolve their catalogs from the compile-time manifest registry
/// via [`manifest_board_kind_catalogs_json`].
pub fn board_kind_catalogs_json(snapshot: &Value) -> Option<String> {
    let meta = snapshot.get("meta");
    meta.and_then(|meta| meta.get("kindCatalogs")).and_then(document_board_kind_catalogs_json).or_else(|| meta.and_then(|meta| meta.get("manifestId")).and_then(Value::as_str).and_then(manifest_board_kind_catalogs_json))
}

/// 🗂️ The `meta.kindCatalogs` half of [`board_kind_catalogs_json`]. Returns `None` when the document
/// contributes no node kinds, so a document carrying an empty catalog bundle still falls through to
/// its manifest rather than clearing the engine's catalogs.
fn document_board_kind_catalogs_json(catalogs: &Value) -> Option<String> {
    let node_kinds = catalogs.get("nodes").and_then(Value::as_array).map(|rows| {
        Value::Array(
            rows.iter()
                .map(|row| {
                    let mut node = catalog_row_subset(row, &["id", "name", "icon", "color", "shape", "scale"]);
                    let handles: Vec<Value> = row.get("handles").and_then(Value::as_array).map_or_else(Vec::new, |templates| {
                        templates.iter().filter(|template| template.get("handleKind").and_then(Value::as_str).is_some_and(|kind| !kind.trim().is_empty())).map(|template| catalog_row_subset(template, &["handleKind", "angle", "radius"])).collect()
                    });
                    set_member(&mut node, "handles", Value::Array(handles));
                    node
                })
                .collect(),
        )
    });
    let mut out = semio_framework_pack_json::Object::new();
    for (key, rows) in [
        ("handleKinds", catalog_rows_subset(catalogs, "handles", &["id", "name", "color", "defaultWireKind", "scale"])),
        ("wireKinds", catalog_rows_subset(catalogs, "wires", &["id", "name", "defaultEdgeKind"])),
        ("nodeKinds", node_kinds),
        ("edgeKinds", catalog_rows_subset(catalogs, "edges", &["id", "name", "color", "stroke", "pattern", "sourceTip", "targetTip", "directed"])),
    ] {
        if let Some(rows) = rows {
            out.insert(key.to_string(), rows);
        }
    }
    let contributes_node_kinds = out.get("nodeKinds").and_then(Value::as_array).is_some_and(|rows| !rows.is_empty());
    contributes_node_kinds.then(|| Value::Object(out).to_string())
}

/// 🗂️ [`board_kind_catalogs_json`] under the SAME node-kind resolution law the fill run already uses
/// (`precompute::fill::fill_kind_rows`): a brush or suggestion candidate can only be built from a node
/// kind that carries handle templates, so a resolved catalog whose `nodeKinds` are all template-less —
/// Concrete Forest's manifest rows are presentation only — falls back to the kinds the document itself
/// implies ([`inferred_node_kind_rows`]), and a document resolving no catalog at all uses them outright.
/// Without this the engine's usable `node_kinds` map stays empty and every candidate lookup silently
/// yields nothing: a picker that opens on a free handle and lists nothing forever.
pub fn board_kind_catalogs_json_or_inferred(snapshot: &Value) -> Option<String> {
    let inferred = || {
        let rows = inferred_node_kind_rows(snapshot);
        (!rows.is_empty()).then_some(rows)
    };
    let Some(json) = board_kind_catalogs_json(snapshot) else {
        return inferred().map(|rows| json!({ "nodeKinds": Value::Array(rows) }).to_string());
    };
    let Some(mut catalogs) = semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok().filter(|value| value.as_object().is_some()) else {
        return Some(json);
    };
    let templated = catalogs.get("nodeKinds").and_then(Value::as_array).is_some_and(|rows| rows.iter().any(|row| row.get("handles").and_then(Value::as_array).is_some_and(|handles| !handles.is_empty())));
    if templated {
        return Some(json);
    }
    match inferred() {
        Some(rows) => {
            set_member(&mut catalogs, "nodeKinds", Value::Array(rows));
            Some(catalogs.to_string())
        }
        None => Some(json),
    }
}

/// 🗂️ The kind ids present in the document itself, used whenever the snapshot carries no explicit
/// `meta.kindCatalogs` slice.
pub fn inferred_kind_entries(snapshot: &Value, field: &str) -> Vec<Value> {
    let mut ids = BTreeSet::new();
    match field {
        "nodes" => return inferred_node_kind_rows(snapshot),
        "handles" => {
            for node in board_snapshot_nodes(snapshot) {
                if let Some(handles) = node.get("handles").and_then(|value| value.as_array()) {
                    for handle in handles {
                        if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                            ids.insert(kind.to_string());
                        }
                    }
                }
            }
        }
        "edges" => {
            for edge in board_snapshot_edges(snapshot) {
                if let Some(kind) = edge.get("edgeKind").and_then(|value| value.as_str()) {
                    ids.insert(kind.to_string());
                }
            }
        }
        _ => {}
    }
    ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect()
}

/// 🧬️ The node kind rows a catalog-less document IMPLIES: one row per distinct `nodeKind`, shaped like
/// the first node carrying it — shape, size (as the engine's `scale` of its 96-unit kind footprint),
/// icon and that node's handles as templates. A document that came from no manifest (Concrete Forest,
/// or any board a user drew) has exactly the kinds it shows, so the catalogue panel offers them and
/// the fill can place them; a document whose meta carries `kindCatalogs` never reaches this.
pub fn inferred_node_kind_rows(snapshot: &Value) -> Vec<Value> {
    let mut rows: Vec<Value> = Vec::new();
    for node in board_snapshot_nodes(snapshot) {
        let Some(kind) = node.get("nodeKind").and_then(Value::as_str).filter(|kind| !kind.is_empty()) else { continue };
        if rows.iter().any(|row| row.get("id").and_then(Value::as_str) == Some(kind)) {
            continue;
        }
        let rectangle = node.get("shape").and_then(Value::as_str) == Some("rectangle");
        let size = if rectangle {
            node.get("width").and_then(Value::as_f64).unwrap_or(48.0).max(node.get("height").and_then(Value::as_f64).unwrap_or(48.0))
        } else {
            node.get("radius").and_then(Value::as_f64).unwrap_or(24.0) * 2.0
        };
        let handles: Vec<Value> = node
            .get("handles")
            .and_then(Value::as_array)
            .map(|handles| {
                handles
                    .iter()
                    .map(|handle| {
                        let mut template = json!({ "handleKind": handle.get("handleKind").and_then(Value::as_str).unwrap_or("port"), "angle": handle.get("angle").and_then(Value::as_f64).unwrap_or(0.0) });
                        if let Some(radius) = handle.get("radius").and_then(Value::as_f64) {
                            set_member(&mut template, "radius", json!(radius));
                        }
                        template
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut row = json!({ "id": kind, "name": kind, "shape": if rectangle { "rectangle" } else { "circle" }, "scale": size / 96.0, "radius": size * 0.5, "width": size, "height": size, "handles": handles });
        if let Some(icon) = node.get("iconKind").and_then(Value::as_str) {
            set_member(&mut row, "iconKind", json!(icon));
        }
        rows.push(row);
    }
    rows
}

/// 🗂️ The node-kind rows a document actually offers: its own `meta.kindCatalogs.nodes` when it
/// declares them, else the rows its `meta.manifestId` declares, else the rows its own nodes imply.
/// One seam so the catalogue panel, the `addNode` arg form and the Add Node dialog can never drift
/// onto different kind sets.
pub fn puzzle2d_node_kind_rows(snapshot: &Value) -> Vec<Value> {
    if let Some(rows) = kind_catalog_entries(snapshot, "nodes").filter(|rows| !rows.is_empty()) {
        return rows.to_vec();
    }
    let manifest_rows = snapshot
        .get("meta")
        .and_then(|meta| meta.get("manifestId"))
        .and_then(Value::as_str)
        .and_then(manifest_board_kind_catalogs_json)
        .and_then(|json| semio_framework_pack_json::parse(&json,semio_framework_pack_json::JsonMemberPolicy::Reject).ok())
        .and_then(|catalogs| catalogs.get("nodeKinds").and_then(Value::as_array).cloned())
        .unwrap_or_default();
    if !manifest_rows.is_empty() {
        return manifest_rows;
    }
    inferred_kind_entries(snapshot, "nodes")
}

/// 🗂️ The `kind` options the `addNode` arg form and the Add Node dialog offer. `AppDefinition` is
/// minted once per process and never sees the live document, so the union of the SHIPPED examples'
/// own node kinds is the reachable kind set — never the literal `"node"` option this select used to
/// hardcode, which could not add a single real kind of either example (puzzle3d's own §23 bug).
///
/// 🖐️ Those rows are AUTHORED below rather than derived, in the two shipped examples' own catalog
/// order, and pinned to the documents by `shipped_node_kinds_are_the_two_examples_own_catalog_rows`.
///
/// 🐛️ Deriving it dereferenced `CONCRETE_FOREST_EXAMPLE_JSON` and `NAKAGIN_EXAMPLE_JSON`, i.e.
/// parsed 96 005 B of authored DSL, re-serialised it to JSON and parsed that back into two
/// `semio_framework_pack_json::Value`s — on the `AppDefinition` path, which is the `describe()` path AND every
/// actor boot. Measured natively on 2026-09-22 (slice PZ2,
/// `🗑️generated/pz2-native-profile-*.txt`): `create_puzzle2d_app()` cost 157 ms cold against ~1 ms
/// with those statics warm, so ALL of it was this one select.
pub const PUZZLE2D_SHIPPED_NODE_KINDS: &[(&str, &str)] = &[
    ("Hexagonal Cut Concrete Forest Left", "Hexagonal Cut Concrete Forest Left"),
    ("Balcony", "Balcony"),
    ("Base", "Base"),
    ("Base Blob", "Base Blob"),
    ("Bridge", "Bridge"),
    ("Capital", "Capital"),
    ("Capsule", "Capsule"),
    ("Capsule Backslash", "Capsule Backslash"),
    ("Capsule J", "Capsule J"),
    ("Capsule L", "Capsule L"),
    ("Capsule P", "Capsule P"),
    ("Capsule q", "Capsule q"),
    ("Capsule S", "Capsule S"),
    ("Capsule Slash", "Capsule Slash"),
    ("Capsule With Balcony Backslash", "Capsule With Balcony Backslash"),
    ("Capsule With Balcony J", "Capsule With Balcony J"),
    ("Capsule With Balcony L", "Capsule With Balcony L"),
    ("Capsule With Balcony P", "Capsule With Balcony P"),
    ("Capsule With Balcony Q", "Capsule With Balcony Q"),
    ("Capsule With Balcony S", "Capsule With Balcony S"),
    ("Capsule With Balcony Slash", "Capsule With Balcony Slash"),
    ("Capsule With Balcony Z", "Capsule With Balcony Z"),
    ("Capsule Z", "Capsule Z"),
    ("Cylindric Capital", "Cylindric Capital"),
    ("Cylindric First Storey Tambour", "Cylindric First Storey Tambour"),
    ("Cylindric Last Storey Tambour", "Cylindric Last Storey Tambour"),
    ("Cylindric Single Storey Tambour", "Cylindric Single Storey Tambour"),
    ("Cylindric Tambour", "Cylindric Tambour"),
    ("Ellipsoid", "Ellipsoid"),
    ("First Storey Tambour", "First Storey Tambour"),
    ("Last Storey Tambour", "Last Storey Tambour"),
    ("Single Storey Tambour", "Single Storey Tambour"),
    ("Tambour", "Tambour"),
    ("Trapezoid", "Trapezoid"),
    ("Trapezoid Capsule Backslash", "Trapezoid Capsule Backslash"),
    ("Trapezoid Capsule J", "Trapezoid Capsule J"),
    ("Trapezoid Capsule L", "Trapezoid Capsule L"),
    ("Trapezoid Capsule P", "Trapezoid Capsule P"),
    ("Trapezoid Capsule Q", "Trapezoid Capsule Q"),
    ("Trapezoid Capsule S", "Trapezoid Capsule S"),
    ("Trapezoid Capsule Slash", "Trapezoid Capsule Slash"),
    ("Trapezoid Capsule Z", "Trapezoid Capsule Z"),
    ("Piece", "Piece"),
];

/// 🗂️ The `kind` select's options, mapped from [`PUZZLE2D_SHIPPED_NODE_KINDS`].
pub fn puzzle2d_node_kind_options() -> Vec<ActionArgOption> {
    PUZZLE2D_SHIPPED_NODE_KINDS.iter().take(PUZZLE2D_NODE_KIND_OPTIONS_MAX).map(|(id, label)| ActionArgOption::new(*id, LocalizedLabel::data(*label))).collect()
}

/// 🗂️ The one `kind` select both the standalone `addNode` arg form and the Add Node dialog declare —
/// built twice from the same catalog so the two forms can never drift apart.
fn puzzle2d_node_kind_arg() -> ActionArgDef {
    let options = puzzle2d_node_kind_options();
    let default = options.first().map(|option| option.value.clone()).unwrap_or_default();
    ActionArgDef::select("kind", puzzle2d_localized(|l| l.kind), options).required().default_value(&default)
}

/// 📐️ Upper bound a placement-tuning measure (contact tolerance, overlap budget) is clamped to —
/// half the default node footprint, past which every candidate would collide or none would.
pub const PUZZLE2D_PLACEMENT_MEASURE_MAX: f64 = 48.0;

/// 🔢️ A stepper's absolute `value`, else `current + delta`; `None` when the args name neither a
/// finite absolute nor a finite delta — the 2d twin of `puzzle3d_absolute_or_delta`.
pub fn puzzle2d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    if let Some(value) = args.and_then(|args| args.get("value")).and_then(Value::as_f64).filter(|value| value.is_finite()) {
        return Some(value);
    }
    args.and_then(|args| args.get("delta")).and_then(Value::as_f64).filter(|delta| delta.is_finite()).map(|delta| current + delta)
}

/// 🏷️ One node-kind catalog row's display name — the same `name` / `id` precedence the catalogue
/// panel renders, resolved against the document's own `meta.kindCatalogs` else its inferred rows.
pub fn puzzle2d_kind_catalog_label(snapshot: &Value, kind_id: &str) -> String {
    let inferred = inferred_kind_entries(snapshot, "nodes");
    let entries = kind_catalog_entries(snapshot, "nodes").unwrap_or(inferred.as_slice());
    entries
        .iter()
        .find(|entry| entry.get("id").and_then(Value::as_str) == Some(kind_id))
        .and_then(|entry| entry.get("label").or_else(|| entry.get("name")).and_then(Value::as_str))
        .filter(|label| !label.is_empty())
        .map_or_else(|| kind_id.to_string(), str::to_string)
}

/// 🏷️ What one outliner row and one board glyph should read — authored label (`label`, else the
/// board's own `text`), else the kind's catalog name, else the raw id. Ported from puzzle3d's
/// `puzzle3d_object_display_label` (ticket 26/09/15/PUZZLE3D-OBJECT-TREE-LABELS).
pub fn puzzle2d_node_display_label(node: &Value, snapshot: &Value) -> String {
    for key in ["label", "text"] {
        if let Some(label) = node.get(key).and_then(Value::as_str).filter(|label| !label.is_empty()) {
            return label.to_string();
        }
    }
    node.get("nodeKind")
        .and_then(Value::as_str)
        .map(|kind| puzzle2d_kind_catalog_label(snapshot, kind))
        .filter(|label| !label.is_empty())
        .unwrap_or_else(|| node.get("id").and_then(Value::as_str).unwrap_or("node").to_string())
}

/// 🏷️ What a reference to the board entity `id` (one of `kinds`: `node`, `targetRegion`, `edge`, `handle`; every kind when
/// none is declared) reads in `snapshot` — a node its display label ([`puzzle2d_node_display_label`]), a target region its
/// authored label, an edge its two endpoint nodes, a handle its node and handle kind. Authored and catalogued names are
/// data, the same in every locale; `None` when only the raw id would name the entity.
pub fn puzzle2d_entity_label(snapshot: &Value, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
    let wants = |kind: &str| kinds.is_empty() || kinds.iter().any(|declared| declared == kind);
    let by_id = |entities: &[Value]| entities.iter().find(|entity| entity.get("id").and_then(Value::as_str) == Some(id)).cloned();
    let node_of = |node_id: &str| board_snapshot_nodes(snapshot).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(node_id)).map(|node| puzzle2d_node_display_label(node, snapshot)).unwrap_or_else(|| node_id.to_string());
    let label = if let Some(node) = wants(PUZZLE2D_GRANULARITY_NODE).then(|| by_id(board_snapshot_nodes(snapshot))).flatten() {
        Some(puzzle2d_node_display_label(&node, snapshot))
    } else if let Some(region) = wants(PUZZLE2D_GRANULARITY_TARGET_REGION).then(|| by_id(snapshot_target_regions(snapshot))).flatten() {
        region.get("label").and_then(Value::as_str).filter(|label| !label.is_empty()).map(str::to_string)
    } else if let Some(edge) = wants(PUZZLE2D_GRANULARITY_EDGE).then(|| by_id(board_snapshot_edges(snapshot))).flatten() {
        let endpoint = |key: &str| edge.get(key).and_then(Value::as_str).map(|handle| node_of(handle.split_once(':').map_or(handle, |(node, _)| node)));
        endpoint("source").zip(endpoint("target")).map(|(source, target)| format!("{source} → {target}"))
    } else if wants(PUZZLE2D_GRANULARITY_HANDLE) {
        board_snapshot_nodes(snapshot).iter().find_map(|node| {
            let handle = node.get("handles").and_then(Value::as_array)?.iter().find(|handle| handle.get("id").and_then(Value::as_str) == Some(id))?;
            Some(format!("{} \u{b7} {}", puzzle2d_node_display_label(node, snapshot), handle.get("handleKind").and_then(Value::as_str).unwrap_or(id)))
        })
    } else {
        None
    };
    label.filter(|label| label != id).map(|label| LocalizedLabel::data(&label))
}

fn puzzle2d_label_root(label: &str) -> String {
    let Some((base, suffix)) = label.rsplit_once(' ') else {
        return label.to_string();
    };
    if suffix.parse::<u32>().is_ok() {
        base.to_string()
    } else {
        label.to_string()
    }
}

fn puzzle2d_label_number(label: &str, root: &str) -> Option<u32> {
    if label == root {
        return Some(1);
    }
    label.strip_prefix(root)?.strip_prefix(' ')?.parse().ok()
}

/// 🔢️ The next distinct node label for one kind — the first instance takes the catalog name, further
/// ones append ` 2`, ` 3`, … to the root taken from its peers. Every creation path (`addNode`, the
/// brush commit, the fill placement, duplicate, paste) stamps this so a document never shows two rows
/// reading the same word. Ported from `puzzle3d_next_object_label`.
pub fn puzzle2d_next_node_label(nodes: &[Value], snapshot: &Value, kind_id: &str) -> String {
    let catalog_base = puzzle2d_kind_catalog_label(snapshot, kind_id);
    let peers: Vec<&Value> = nodes.iter().filter(|node| node.get("nodeKind").and_then(Value::as_str) == Some(kind_id)).collect();
    if peers.is_empty() {
        return catalog_base;
    }
    let authored = |node: &&Value| node.get("label").or_else(|| node.get("text")).and_then(Value::as_str).filter(|label| !label.is_empty()).map(str::to_string);
    let root = peers.iter().find_map(authored).map(|label| puzzle2d_label_root(&label)).unwrap_or(catalog_base);
    let mut max = 0u32;
    let mut unlabeled = 0u32;
    for node in peers {
        match authored(&node) {
            Some(label) => {
                if let Some(number) = puzzle2d_label_number(&label, &root) {
                    max = max.max(number);
                }
            }
            None => unlabeled += 1,
        }
    }
    max = max.max(unlabeled);
    if max == 0 {
        root
    } else {
        format!("{root} {}", max + 1)
    }
}

pub fn puzzle2d_kind_ids(snapshot: &Value, field: &str) -> Vec<String> {
    let inferred = inferred_kind_entries(snapshot, field);
    let entries = kind_catalog_entries(snapshot, field).unwrap_or(inferred.as_slice());
    entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn new_node_id(prefix: &str) -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(1);
    format!("{prefix}-{}", NEXT.fetch_add(1, Ordering::Relaxed))
}

pub fn puzzle_extension_id() -> &'static str {
    let _extension = Puzzle2dExtension;
    "puzzle.2d"
}

/// 🙈️ Whether a node, handle or edge is hidden. The document stores the inverse, `visible` (absent = shown): a `hidden`
/// key written onto one of them never reaches the typed model, so it would neither persist nor reach a peer.
pub fn puzzle2d_entity_hidden(entity: &Value) -> bool {
    entity.get("visible").and_then(Value::as_bool) == Some(false)
}

/// 🎯️ Every node/handle id sharing a `nodeKind`/`handleKind` with anything currently selected.
pub fn select_same_kind_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected_set: HashSet<&str> = selected.iter().map(String::as_str).collect();
    let mut node_kinds: HashSet<&str> = HashSet::new();
    let mut handle_kinds: HashSet<&str> = HashSet::new();
    for node in board_snapshot_nodes(snapshot) {
        if node.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
            if let Some(kind) = node.get("nodeKind").and_then(|value| value.as_str()) {
                node_kinds.insert(kind);
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("id").and_then(|value| value.as_str()).is_some_and(|id| selected_set.contains(id)) {
                if let Some(kind) = handle.get("handleKind").and_then(|value| value.as_str()) {
                    handle_kinds.insert(kind);
                }
            }
        }
    }
    let mut ids: Vec<String> = Vec::new();
    for node in board_snapshot_nodes(snapshot) {
        if node.get("nodeKind").and_then(|value| value.as_str()).is_some_and(|kind| node_kinds.contains(kind)) {
            if let Some(id) = node.get("id").and_then(|value| value.as_str()) {
                ids.push(id.to_string());
            }
        }
        for handle in node.get("handles").and_then(|value| value.as_array()).into_iter().flatten() {
            if handle.get("handleKind").and_then(|value| value.as_str()).is_some_and(|kind| handle_kinds.contains(kind)) {
                if let Some(id) = handle.get("id").and_then(|value| value.as_str()) {
                    ids.push(id.to_string());
                }
            }
        }
    }
    ids
}

/// 🎥️ Writes an `{ x, y, zoom }` camera payload into the config — session-only view state, never the snapshot.
pub fn set_runtime_camera(runtime: &mut Puzzle2dPlayRuntime, camera: &Value) {
    if let Some(x) = camera.get("x").and_then(Value::as_f64) {
        runtime.camera_x = x;
    }
    if let Some(y) = camera.get("y").and_then(Value::as_f64) {
        runtime.camera_y = y;
    }
    if let Some(zoom) = camera.get("zoom").and_then(Value::as_f64) {
        runtime.camera_zoom = zoom;
    }
}

/// 📐️ The value one inspector row asks an entity's `field` to take: an absolute `value` wins,
/// otherwise a numeric `delta` rides on that entity's own current reading — offset-preserving across
/// a multi-select whose members start apart.
fn patched_field_value(entity: &Value, field: &str, value: Option<&Value>, delta: Option<&Value>) -> Option<Value> {
    if let Some(absolute) = value {
        return Some(absolute.clone());
    }
    delta.and_then(Value::as_f64).map(|delta| json!(entity.get(field).and_then(Value::as_f64).unwrap_or(0.0) + delta))
}

/// 🎯️ The document's fill-constraining rectangles, or an empty slice when the board declares none.
pub fn snapshot_target_regions(snapshot: &Value) -> &[Value] {
    snapshot.get("targetRegions").and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

/// 🎯️ The region ids inside `selected` — the framework selection is one flat id list per domain, so a
/// region is simply a selected id the document holds a region for.
pub fn puzzle2d_selected_target_region_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    snapshot_target_regions(snapshot).iter().filter_map(|region| region.get("id").and_then(Value::as_str)).filter(|id| selected.contains(id)).map(str::to_string).collect()
}

/// 📐️ Normalized `[min_x, min_y, max_x, max_y]` of one region entry — the `Value` twin of
/// [`crate::Puzzle2dTargetRegion::bounds`], so the brush's corner order never reaches a reader.
pub fn puzzle2d_region_bounds(region: &Value) -> [f64; 4] {
    let read = |key: &str| region.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    let (x, y, width, height) = (read("x"), read("y"), read("width"), read("height"));
    let (min_x, max_x) = if width < 0.0 { (x + width, x) } else { (x, x + width) };
    let (min_y, max_y) = if height < 0.0 { (y + height, y) } else { (y, y + height) };
    [min_x, min_y, max_x, max_y]
}

/// 🎯️ Whether ANY visible region of this document contains the axis-aligned box — the fill rule, on
/// the `Value` snapshot. An empty visible set is unconstrained, exactly as in puzzle3d.
pub fn puzzle2d_snapshot_regions_admit(snapshot: &Value, aabb: [f64; 4]) -> bool {
    let visible: Vec<&Value> = snapshot_target_regions(snapshot).iter().filter(|region| region.get("hidden").and_then(Value::as_bool) != Some(true)).collect();
    if visible.is_empty() {
        return true;
    }
    visible.into_iter().any(|region| {
        let bounds = puzzle2d_region_bounds(region);
        aabb[0] >= bounds[0] && aabb[1] >= bounds[1] && aabb[2] <= bounds[2] && aabb[3] <= bounds[3]
    })
}

/// 🖍️ The Area Brush's W/H steppers as WORLD extent: the steppers count grid cells, the engine and
/// the board both work in board units, and [`Puzzle2dRecorder::paint_region_cells`] uses the same product —
/// so a click on the canvas and a dispatch of `addTargetRegion` paint the identical rectangle.
pub fn puzzle2d_area_brush_extent_world(runtime: &Puzzle2dPlayRuntime) -> (f64, f64) {
    let grid = runtime.grid_factor.abs().max(0.1);
    (runtime.area_brush_width.max(1.0) * grid, runtime.area_brush_height.max(1.0) * grid)
}

/// 🎲️ Re-mints a node id when it collides with an existing one — client-side brush serials restart every session.
fn unique_node_id(snapshot: &Value, candidate: String) -> String {
    if board_snapshot_nodes(snapshot).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("node")
    } else {
        candidate
    }
}

fn unique_edge_id(snapshot: &Value, candidate: String) -> String {
    if board_snapshot_edges(snapshot).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(candidate.as_str())) {
        new_node_id("edge")
    } else {
        candidate
    }
}

/// 🔖️ The handles of a brush-placed node, each carrying the `"{node}:v{index}"` id the placed edge
/// already addresses ([`Puzzle2dRecorder::place_brush`]'s `target`). The engine's `brushPlace` payload
/// describes handles by kind/angle/radius only; an id-less handle is not a `Puzzle2dHandle` at all, so
/// the placed node would make the whole board fail to decode.
fn puzzle2d_placed_handles(node_id: &str, handles: Option<&Value>) -> Value {
    let rows = handles.and_then(Value::as_array).map(|rows| rows.iter().enumerate().map(|(index, handle)| {
        let mut handle = handle.clone();
        if let Some(object) = handle.as_object_mut() {
            if object.get("id").and_then(Value::as_str).is_none_or(str::is_empty) {
                object.insert("id", json!(format!("{node_id}:v{index}")));
            }
        }
        handle
    }).collect::<Vec<_>>()).unwrap_or_default();
    Value::Array(rows)
}

/// ➡️ A fresh edge id no edge of `snapshot` already carries.
pub fn new_edge_id(snapshot: &Value) -> String {
    unique_edge_id(snapshot, new_node_id("edge"))
}

/// ➡️ Appends one edge to a SCRATCH host view, creating the `edges` array when a hand-written document omitted it — only
/// the select tool's proximity bookkeeping calls it, never a command that edits the document.
pub fn puzzle2d_push_edge(snapshot: &mut Value, edge: Value) {
    puzzle2d_push_entity(snapshot, "edges", edge);
}

fn puzzle2d_push_entity(snapshot: &mut Value, key: &str, entity: Value) {
    let Some(object) = snapshot.as_object_mut() else {
        return;
    };
    if !object.contains_key(key) {
        object.insert(key, json!([]));
    }
    if let Some(entities) = object.get_mut(key).and_then(Value::as_array_mut) {
        entities.push(entity);
    }
}

/// 📐️ Board units from a node's centre to its furthest possible handle — the circle's radius, or the
/// rectangle's half-diagonal. Every handle of the node lies inside this circle whatever its angle, so
/// a proximity search can reject a whole node on its centre alone.
pub fn puzzle2d_node_reach(node: &Value) -> f64 {
    if node.get("shape").and_then(Value::as_str) == Some("rectangle") {
        let half_width = node.get("width").and_then(Value::as_f64).unwrap_or(48.0) / 2.0;
        let half_height = node.get("height").and_then(Value::as_f64).unwrap_or(48.0) / 2.0;
        return half_width.hypot(half_height);
    }
    node.get("radius").and_then(Value::as_f64).unwrap_or(24.0).abs()
}

/// 📍️ World position of one handle on its node — the SAME rim geometry the board engine draws with
/// (`handle_position_on_circle`'s east-zero angle, `handle_position_on_rectangle`'s north-zero one),
/// so a proximity hit is exactly a visual touch.
pub fn puzzle2d_handle_world_position(node: &Value, handle: &Value) -> (f64, f64) {
    let centre = Point::new(node.get("x").and_then(Value::as_f64).unwrap_or(0.0), node.get("y").and_then(Value::as_f64).unwrap_or(0.0));
    let angle = handle.get("angle").and_then(Value::as_f64).unwrap_or(0.0);
    let point = if node.get("shape").and_then(Value::as_str) == Some("rectangle") {
        handle_position_on_rectangle(centre, node.get("width").and_then(Value::as_f64).unwrap_or(48.0), node.get("height").and_then(Value::as_f64).unwrap_or(48.0), angle)
    } else {
        // 🧭️ The rim distance is the NODE's radius; a handle's own `radius` is its glyph size.
        handle_position_on_circle(centre, node.get("radius").and_then(Value::as_f64).unwrap_or(24.0), angle)
    };
    (point.x, point.y)
}

/// 🔌️ The `handleKind` of one handle id (empty string when the handle declares none), `None` when
/// the document carries no such handle at all.
pub fn puzzle2d_handle_kind(snapshot: &Value, handle_id: &str) -> Option<String> {
    board_snapshot_nodes(snapshot)
        .iter()
        .flat_map(|node| node.get("handles").and_then(Value::as_array).into_iter().flatten())
        .find(|handle| handle.get("id").and_then(Value::as_str) == Some(handle_id))
        .map(|handle| handle.get("handleKind").and_then(Value::as_str).unwrap_or_default().to_string())
}

/// 🤝️ Whether `meta.kindCompatibility` admits a `source → target` handle-kind link. A document that
/// declares no rules is permissive (the board engine's own linking gate behaves the same), and a
/// handle carrying no kind is admitted by every rule set.
pub fn puzzle2d_kinds_compatible(snapshot: &Value, source_kind: &str, target_kind: &str) -> bool {
    let rows = snapshot.get("meta").and_then(|meta| meta.get("kindCompatibility")).or_else(|| snapshot.get("kindCompatibility")).and_then(Value::as_array);
    let Some(rows) = rows.filter(|rows| !rows.is_empty()) else {
        return true;
    };
    if source_kind.is_empty() || target_kind.is_empty() {
        return true;
    }
    rows.iter().any(|row| {
        let (Some(source), Some(target)) = (row.get("source").and_then(Value::as_str), row.get("target").and_then(Value::as_str)) else {
            return false;
        };
        let bidirectional = row.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}

/// 🔗️ Every handle id an edge already ends on — an occupied handle refuses a second connection.
pub fn puzzle2d_occupied_handles(snapshot: &Value) -> HashSet<String> {
    board_snapshot_edges(snapshot)
        .iter()
        .flat_map(|edge| [edge.get("source"), edge.get("target")])
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// 📋️ The fragment schema a puzzle 2d copy writes and a puzzle 2d paste accepts.
pub const PUZZLE2D_CLIPBOARD_SCHEMA: &str = "puzzle.2d.clipboard.v1";

/// 📋️ Board units a paste with no explicit placement offsets the clone by — the same nudge
/// `duplicateSelection` uses, so a pasted copy never lands exactly on its source.
pub const PUZZLE2D_PASTE_OFFSET: f64 = 24.0;

fn puzzle2d_clipboard_media_type() -> MediaType {
    MediaType { class: MediaClass::TwoD, form: MediaForm::Design }
}

/// 🕹️ The node ids one selection copies: selected node ids, plus the parent node of every selected
/// handle id — a handle-granularity selection still copies the whole node it belongs to, exactly as
/// puzzle3d's `puzzle3d_selected_objects_from` resolves a vortex selection to its object.
pub fn puzzle2d_selected_node_ids(snapshot: &Value, selected: &[String]) -> Vec<String> {
    let selected: HashSet<&str> = selected.iter().map(String::as_str).collect();
    board_snapshot_nodes(snapshot)
        .iter()
        .filter(|node| {
            let own = node.get("id").and_then(Value::as_str).is_some_and(|id| selected.contains(id));
            own || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| handle.get("id").and_then(Value::as_str).is_some_and(|id| selected.contains(id)))
        })
        .filter_map(|node| node.get("id").and_then(Value::as_str).map(str::to_string))
        .collect()
}

/// 🔐️ Whether ANY addressed entity — node, handle, edge or target region — carries `locked: true`.
/// The one predicate every destructive/moving verb asks before it touches the document, so a lock is
/// the same promise for `deleteSelection`, a board drag, the rotate ring, the three transform verbs
/// and an inspector patch. Wider than [`puzzle2d_selection_is_locked`], which only answers for the
/// node/handle pair the clipboard's cut fragment is built from.
pub fn puzzle2d_addresses_locked_entity(snapshot: &Value, ids: &[String]) -> bool {
    if ids.is_empty() {
        return false;
    }
    let addressed: HashSet<&str> = ids.iter().map(String::as_str).collect();
    let locked = |entity: &Value| entity.get("locked").and_then(Value::as_bool) == Some(true);
    let hits = |entity: &Value| entity.get("id").and_then(Value::as_str).is_some_and(|id| addressed.contains(id));
    let node_locked = board_snapshot_nodes(snapshot).iter().any(|node| {
        (hits(node) && locked(node)) || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| hits(handle) && locked(handle))
    });
    node_locked || board_snapshot_edges(snapshot).iter().any(|edge| hits(edge) && locked(edge)) || snapshot_target_regions(snapshot).iter().any(|region| hits(region) && locked(region))
}

/// 🔒️ Whether any of these nodes (or one of their handles) refuses to be cut.
pub fn puzzle2d_selection_is_locked(snapshot: &Value, node_ids: &[String]) -> bool {
    let ids: HashSet<&str> = node_ids.iter().map(String::as_str).collect();
    board_snapshot_nodes(snapshot).iter().filter(|node| node.get("id").and_then(Value::as_str).is_some_and(|id| ids.contains(id))).any(|node| {
        node.get("locked").and_then(Value::as_bool) == Some(true) || node.get("handles").and_then(Value::as_array).into_iter().flatten().any(|handle| handle.get("locked").and_then(Value::as_bool) == Some(true))
    })
}

/// 📋️ The copied nodes with their handles, plus every edge whose BOTH endpoints were copied, as a
/// snapshot-shaped fragment.
pub fn puzzle2d_copy_fragment_from(snapshot: &Value, node_ids: &[String]) -> Result<ClipboardFragment, ClipboardError> {
    if node_ids.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let ids: HashSet<&str> = node_ids.iter().map(String::as_str).collect();
    let nodes: Vec<Value> = board_snapshot_nodes(snapshot).iter().filter(|node| node.get("id").and_then(Value::as_str).is_some_and(|id| ids.contains(id))).cloned().collect();
    if nodes.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let handle_ids: HashSet<&str> = nodes.iter().flat_map(|node| node.get("handles").and_then(Value::as_array).into_iter().flatten()).filter_map(|handle| handle.get("id").and_then(Value::as_str)).collect();
    let endpoint_copied = |endpoint: Option<&str>| endpoint.is_some_and(|id| handle_ids.contains(id) || ids.contains(id));
    let edges: Vec<Value> = board_snapshot_edges(snapshot)
        .iter()
        .filter(|edge| endpoint_copied(edge.get("source").and_then(Value::as_str)) && endpoint_copied(edge.get("target").and_then(Value::as_str)))
        .cloned()
        .collect();
    let label = format!("{} nodes", nodes.len());
    let clip = json!({ "schema": PUZZLE2D_CLIPBOARD_SCHEMA, "nodes": nodes, "edges": edges });
    Ok(ClipboardFragment { schema: PUZZLE2D_CLIPBOARD_SCHEMA.into(), media_type: puzzle2d_clipboard_media_type(), dsl_text: clip.to_string(), pack_bytes: None, source_app: PUZZLE2D_PLAY_CONTROLLER_ID.into(), label })
}

/// ✂️ Copy-then-delete as ONE mutation list: a `delete-node` per cut node (each severs the edges on its handles) and a
/// `disconnect-handles` per edge still hanging off a cut id, so a cut is a single history edit one undo restores.
pub fn puzzle2d_cut_operations_from(snapshot: &Puzzle2dPlaySnapshot, node_ids: &[String]) -> Vec<Puzzle2dMutation> {
    let mut recorder = Puzzle2dRecorder::new(snapshot);
    recorder.cut(node_ids);
    recorder.into_rows()
}

/// 📋️ Clones a copied fragment with fresh node, handle and edge ids: the edges between copied nodes
/// come back rewired onto the clones, and every clone is offset so it never lands under its source. The mutation list is
/// a `create-node` per clone and a `connect-handles` per rewired edge. Returns it and the pasted node ids the caller
/// re-selects.
pub fn puzzle2d_paste_operations_on(snapshot: &Puzzle2dPlaySnapshot, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<(Vec<Puzzle2dMutation>, Vec<String>), ClipboardError> {
    let expected = puzzle2d_clipboard_media_type();
    if fragment.media_type != expected {
        return Err(ClipboardError::IncompatibleMediaType(fragment.media_type.clone()));
    }
    let clip: Value = semio_framework_pack_json::parse(&fragment.dsl_text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
    let nodes = board_snapshot_nodes(&clip);
    if nodes.is_empty() {
        return Err(ClipboardError::EmptySelection);
    }
    let [offset_x, offset_y] = placement.position.map_or([PUZZLE2D_PASTE_OFFSET, PUZZLE2D_PASTE_OFFSET], |position| [position[0], position[1]]);
    let mut recorder = Puzzle2dRecorder::new(snapshot);
    let pasted = recorder.clone_fragment(nodes, board_snapshot_edges(&clip), (offset_x, offset_y));
    Ok((recorder.into_rows(), pasted))
}

/// 🧱️ The expensive half of syncing `host` from `envelope`: a full `clear_scene()` + rebuild of
/// every node/handle/edge plus the kind-catalog/kind-compat re-push. Only needed when the snapshot
/// content actually changed — gated by `last_synced_fixture` in `handle`.
fn sync_host_fixture_content(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    let _ = {let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(64*1024*1024,&mut accepted);semio_framework_os_infinite::board::io::text::snapshot_assembly::load_board_snapshot_json(host,&envelope.board_snapshot.to_string(),&mut control)};
    if let Some(json) = board_kind_catalogs_json_or_inferred(&envelope.board_snapshot) {
        let _ = host.set_board_kind_catalogs_from_json(&json);
    }
    if let Some(compat) = envelope.board_snapshot.get("meta").and_then(|value| value.get("kindCompatibility")).or_else(|| envelope.board_snapshot.get("kindCompatibility")) {
        let json = semio_framework_pack_json::to_json_string(compat); {
            let _ = host.set_handle_link_compat_from_json(&json);
        }
    }
}

/// 🪶️ The cheap half of syncing `host` from `envelope`: plain setters mirroring ephemeral view state
/// (selection/utility/grid/LOD/…) — must run on every action regardless of whether the snapshot
/// content changed, since this state itself changes every action.
fn sync_host_runtime_state(host: &mut BoardHost, envelope: &Puzzle2dScene, selected_ids: &[String]) {
    host.set_size(BOARD_DEFAULT_WIDTH, BOARD_DEFAULT_HEIGHT, 1.0);
    host.set_selection_ids(selected_ids);
    host.set_active_utility(&envelope.active_utility);
    let overview_lod_mode = envelope.runtime.lod_mode_by_pane.get(crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID).map_or(PUZZLE2D_LOD_MODE_AUTOMATIC, String::as_str);
    if overview_lod_mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
        host.set_automatic_lod(true);
    } else {
        host.set_automatic_lod(false);
        host.set_forced_draw_lod_label(overview_lod_mode);
    }
    host.set_grid_visible(envelope.runtime.grid_visible);
    host.set_grid_snap_enabled(envelope.runtime.grid_snap_enabled);
    host.set_transform_flags(envelope.runtime.transform_move, envelope.runtime.transform_rotate);
    let _ = host.set_grid_factor(envelope.runtime.grid_factor);
    let (brush_width, brush_height) = puzzle2d_area_brush_extent_world(&envelope.runtime);
    host.set_area_brush_extent(brush_width, brush_height);
    host.set_suggestion_offset(envelope.runtime.suggestion_offset);
    let weights_json = semio_framework_pack_json::to_json_string(&json!({
        "nodeWeights": envelope.runtime.node_kind_weights,
        "handleWeights": envelope.runtime.handle_kind_weights,
    })); {
        host.set_brush_kind_weights(&weights_json);
    }
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the marquee method is
    // framework-owned now (`interactionSelect`'s `method` arg) — the board engine still needs SOME
    // default to hit-test with, so it keeps "rectangle" rather than reading a deleted config field.
    // 🎯️ …but WHICH granularities a hit-test may return is app config now (`setSelectableKind`), so
    // a filtered pane cannot silently pick the kind the user switched off. Argument order on the
    // normal port is `(nodes, edges, handles)`.
    let kinds = envelope.runtime.selectable_kinds;
    host.set_selection_options("rectangle", "replace", kinds.nodes, kinds.edges, kinds.handles);
}

fn sync_host_from_envelope(host: &mut BoardHost, envelope: &Puzzle2dScene) {
    sync_host_fixture_content(host, envelope);
    sync_host_runtime_state(host, envelope, &[]);
}

/// 🪞️ Replays the board host's owned events: a board-tool row (an area painted, a brush stamped, a wire dropped or cut, a
/// delete) is recorded as the concrete kinds it consists of, a candidate page is window-transient runtime state, and a
/// guest-side host never receives a pointer, so it never records a gesture. Selection is framework-owned and camera is
/// deliberately NOT mirrored: every action that moves the camera already writes the config's camera fields directly.
pub fn apply_host_events(host: &mut BoardHost, recorder: &mut Puzzle2dRecorder, runtime: &mut Puzzle2dPlayRuntime) {
    let events_raw = drain_board_events_json(host);
    apply_board_events::replay_board_events(&events_raw, recorder, runtime);
}

/// 🖌️ Re-enters the board host's brush slot on the handle this window transient remembers. The guest
/// host is rebuilt from the document on EVERY dispatch (`puzzle2d_dispatch_emit`), so the slot
/// `openHandleSuggestions` opened is gone by the next verb — every candidate verb after it restores the
/// slot before it reads an index. The rebuild is deterministic (same snapshot, same kind catalogs, same
/// weights), so the candidate page is the one the client painted and the index the popup names is the
/// candidate that gets placed. An open popup claims the slot's hover; the armed brush only targets it.
/// `requested` is the handle the CALLER named — the popup's own rows carry it, so a client that still
/// has the menu on screen never depends on the transient having round-tripped back to the guest yet.
pub fn puzzle2d_restore_brush_slot(ctx: &mut Puzzle2dActionCtx<'_>, requested: Option<&str>) -> Option<String> {
    let popup = ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.handle_id.clone()).filter(|handle_id| !handle_id.is_empty());
    let named = requested.filter(|handle_id| !handle_id.is_empty()).map(str::to_string);
    let opened = named.clone().or_else(|| popup.clone());
    let handle_id = opened.clone().or_else(|| Some(ctx.scene.runtime.brush_candidate_source_handle_id.clone()).filter(|handle_id| !handle_id.is_empty()))?;
    let mut host = ctx.host.borrow_mut();
    if named.is_some() || popup.is_some() {
        host.brush_open_slot(&handle_id);
    } else {
        host.brush_target_slot(Some(&handle_id));
    }
    Some(handle_id)
}

/// 📨️ Retires the board host's bounded owned-event queue into its public JSON envelope.
pub fn drain_board_events_json(host: &mut BoardHost) -> String {
    let mut output = String::from("[");
    let mut first = true;
    while let Some(event) = host.pop_owned_event() {
        if !first {
            output.push(',');
        }
        first = false;
        event.write_json(&mut output);
    }
    output.push(']');
    output
}

/// 🐢️ Narrow `UiDirtyScope` shared by pure view/selection/camera actions that only touch the 3
/// canvas panes (never a panel or engagement/measure/utility refresh).
pub fn puzzle2d_window_only_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for actions that additionally change the engagement bar (active utility,
/// brush weights, LOD/grid settings, engagement text input) but never touch document content.
pub fn puzzle2d_window_and_engagements_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for settings surfaced in the measures sidebar (LOD mode, grid, brush
/// weights, suggestion offset) but that never touch document content or the engagement bar.
pub fn puzzle2d_window_and_measures_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🐢️ Narrow `UiDirtyScope` for a runtime-only selection change: the 3 canvas panes plus the
/// layers/properties panels (which highlight the selection) and the engagement bar.
pub fn puzzle2d_select_scope() -> UiDirtyScope {
    UiDirtyScope::Partial {
        window_bodies: apply_board_events::PUZZLE2D_WINDOW_BODY_KEYS.iter().map(|body_key| body_key.to_string()).collect(),
        panel_bodies: vec![artifact::PUZZLE2D_PLAY_BODY_LAYERS.to_string(), inspection::PUZZLE2D_PLAY_BODY_PROPERTIES.to_string()],
        utilities: false,
        tools: false,
        engagements: true,
        measures: false,
        labels: false,
    }
}

/// 🎯️ B1: `Puzzle2dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.mutation(...)`/`.view_action(...)`/`.action_with(...)` id
/// `create_puzzle2d_app` registers below, plus the framework-injected `setActiveUtility` and the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple every `🎮️commands/*` arm expects, so each arm's internal
/// `args.get("field")` extraction stays byte-for-byte identical to the pre-B1 implementation.
///
/// ⚠️ `OpBinary` is a plain JSON-bytes bridge (NOT `#[derive(dsl::DslOps)]`, and NOT the framework's
/// `app_commands!` macro): a generic `args: Value` field is not representable in the DSL grammar
/// those target, so adopting them would silently rewrite this app's wire format. Keep this macro's
/// variant list byte-for-byte stable.
macro_rules! puzzle2d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
        pub enum Puzzle2dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle2dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle` dispatches on.
            pub(crate) fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle2dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle2dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            pub(crate) fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle2dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            fn try_from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Option<Self> {
                match action {
                    $($id => Some(Puzzle2dCommand::$Variant { window_id, args })),*,
                    _ => None,
                }
            }

            fn to_host_value(&self)->Value {
                match self {
                    $(Self::$Variant{window_id,args}=>json!({"action":$id,"windowId":window_id,"args":args})),*
                }
            }
            fn from_host_value(value:&Value)->Option<Self>{
                let object=value.as_object()?;
                if object.iter().any(|(key,_)|!["action","windowId","args"].contains(&key)){return None;}
                let action=value.get("action")?.as_str()?;
                let window_id=match value.get("windowId"){Some(Value::String(value))=>Some(value.clone()),None|Some(Value::Null)=>None,_=>return None};
                let args=value.get("args").cloned().filter(|value|!value.is_null());
                Self::try_from_action(action,args,window_id)
            }
            #[cfg(test)]
            pub(crate) fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                Self::try_from_action(action, args, window_id)
                    .unwrap_or_else(|| panic!("unknown puzzle2d action id in test: {action}"))
            }
        }
    };
}

puzzle2d_command_variants! {
    AddNode = "addNode",
    SetActiveExample = "setActiveExample",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    ForceLayout = "forceLayout",
    FocusSelection = "focusSelection",
    SelectSameKind = "selectSameKind",
    CreateEdge = "createEdge",
    DeleteEdge = "deleteEdge",
    ProximityConnect = "proximityConnect",
    SetProximityRadius = "setProximityRadius",
    SetSelectionFlag = "setSelectionFlag",
    SetSelectionHidden = "setSelectionHidden",
    SetSelectionLocked = "setSelectionLocked",
    PatchInspectorNodes = "patchInspectorNodes",
    RedrawHandles = "redrawHandles",
    Reorganize = "reorganize",
    ApplyBoardEvents = "applyBoardEvents",
    SetFillCount = "setFillCount",
    AcceptSuggestion = "acceptSuggestion",
    SetCamera = "setCamera",
    EngagementInput = "engagementInput",
    EngagementSubmit = "engagementSubmit",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    SetLodModeForPane = "setLodModeForPane",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    SetGridVisible = "setGridVisible",
    SetSelectableKind = "setSelectableKind",
    SetBrushPlacementContactTolerance = "setBrushPlacementContactTolerance",
    SetBrushPlacementOverlapBudget = "setBrushPlacementOverlapBudget",
    EngagementRepeatLast = "engagementRepeatLast",
    OpenAddNodeDialog = "openAddNodeDialog",
    SetBrushKindWeights = "setBrushKindWeights",
    SetBrushNodeSize = "setBrushNodeSize",
    SetSuggestionOffset = "setSuggestionOffset",
    SetTransformGumballFlag = "setTransformGumballFlag",
    AddTargetRegion = "addTargetRegion",
    DeleteTargetRegion = "deleteTargetRegion",
    RelocateTargetRegion = "relocateTargetRegion",
    SetTargetRegionFlag = "setTargetRegionFlag",
    SetTargetRegionHidden = "setTargetRegionHidden",
    SetTargetRegionLocked = "setTargetRegionLocked",
    SetAreaBrushSize = "setAreaBrushSize",
    CycleBrushCandidate = "cycleBrushCandidate",
    CycleBrushCandidateBack = "cycleBrushCandidateBack",
    TargetBrushSuggestions = "targetBrushSuggestions",
    HoverSuggestion = "hoverSuggestion",
    OpenHandleSuggestions = "openHandleSuggestions",
    CloseHandleSuggestions = "closeHandleSuggestions",
    LodScaleJson = "lodScaleJson",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    ExportSnapshot = "exportSnapshot",
    ImportSnapshot = "importSnapshot",
    OpenImportSnapshot = "openImportSnapshot",
}

impl protocol::OpBinary for Puzzle2dCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &PUZZLE2D_TOOL_JOB_IDS;
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(semio_framework_pack_json::to_string(&self.to_host_value()).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text=std::str::from_utf8(bytes).map_err(|error|protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,error.to_string()))))?;
        let value=semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error|protocol::ProtocolError::Pack(store::PackError::from(error.into_value_error())))?;
        Self::from_host_value(&value).ok_or_else(||protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid puzzle2d command"))))
    }
}
