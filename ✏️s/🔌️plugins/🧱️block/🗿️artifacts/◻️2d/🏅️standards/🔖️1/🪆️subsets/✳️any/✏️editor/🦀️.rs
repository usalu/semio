//! 🩻️ Block 2D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the board window
//! in `🎭️modes/✏️edit/🪟️windows/📋️board`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`,
//! view state in `🦀️config.rs`, document-side compute in `crate::standards::v1::subsets::any::schema`, and this
//! app's own typed media I/O surface + plugin registration (below — constitutional: general, an
//! artifact must never depend on an app, so both live here rather than under `🗿️artifacts`).

use crate::standards::v1::subsets::any::schema::mutations::text::Block2dMutation;
use crate::{artifact_kind, Block2dSnapshot, BLOCK_2D_SCHEMA};
use crate::editor::block2d::commands::patch_node_kind;
use crate::editor::block2d::commands::{add_compatibility_rule, remove_compatibility_rule};
use crate::editor::block2d::commands::{add_handle, remove_handle};
use crate::editor::block2d::commands::{add_handle_kind, remove_handle_kind};
use crate::editor::block2d::commands::{edit, set_active_example};
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use crate::editor::block2d::modes::edit as edit_mode;
use crate::editor::block2d::modes::edit::windows::board;
use crate::editor::block2d::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::block2d::terminology::block2d_labels;
use semio_framework::{DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, TopologyNode};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppIo, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactPresentation, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, Dialect, DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload,
    MediaPortDirection, MediaPortSpec, MediaType, NoDraft, NoDraftMutation, PortMultiplicity,
};
use std::collections::BTreeMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const BLOCK2D_PLAY_APP_ID: &str = "block2d-play";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this app's rim-handle templates ("handle" granularity, the default) and handle-kind
/// catalog ("handleKind" granularity) — replaces the deleted `Block2dConfig.selected_ids`.
pub const BLOCK2D_INTERACTION_HANDLE: &str = "handle";
pub const BLOCK2D_GRANULARITY_HANDLE: &str = "handle";
pub const BLOCK2D_GRANULARITY_HANDLE_KIND: &str = "handleKind";
/// 🗂️ The `s/plugin/puzzle` 2d catalog artifact kind block2d's `"catalog:out"` port produces — see
/// `block2d_io` and `Block2dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎮️commands/*`)? builds its `on_change`/item actions with.
pub fn block2d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(BLOCK2D_PLAY_APP_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}


/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🔤️ Admits one dynamic string into the fixed-capacity UI text owner (control values, icons,
/// commit conventions) — the `UiValue`-free twin of [`ui_value_text`].
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block2d text admission failed"))
}

/// 🏷️ Admits one dynamic string into the fixed-capacity UI label owner — every panel/window label in
/// this subset goes through here rather than the renderer's unbounded `Label`.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref().to_string()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block2d label admission failed"))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ `Block2dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"2d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle2d_manifest_fragment` a real caller (see `export_media` above).
pub fn block2d_io() -> AppIo {
    let io = semio_framework::io::resolve_ready(AppIo::from_document(BLOCK_2D_SCHEMA, MediaType { class: MediaClass::Kit, form: MediaForm::Type }, ArtifactPresentation { id: "2d.block".into(), name: "Node Kind".into(), dimension: "2d".into(), component_kind: "block2d".into() }));
    semio_framework::io::resolve_ready(io.with_ports(vec![MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: PortMultiplicity::Many,
    }]))
}
//#endregion 🔖️Io

//#region 🔌️Registration
// 🗂️ `Block2dSnapshot`'s pack↔dsl codec, `block2d`'s artifact schema/inference descriptors, its
// composer table and its pilot-language grammars now register declaratively via
// `crate::artifact()` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
// `descriptor-prep`; previously `declaration()`, ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
// M1/W1d), consumed by `.declare_artifact(crate::artifact())` in the plugin root
// (`🧱️block/🦀️.rs`) — replacing this app's former side-effecting `register()`. Nothing
// app-scope-only remains here: `Block2dPlayApp::app_schema()` now returns
// `crate::editor::block2d::config::schema::app_schema_descriptor()` directly (ticket W1c), so the
// plugin root's `.setup()` escape hatch is gone entirely.
//#endregion 🔌️Registration

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block2dPlayApp::Command` — the SOLE dispatch surface for block2d's own behavior, covering
    /// every action `create_block2d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair here is IDENTICAL (the pre-migration
    /// `#[dsl(key)]` already used the camelCase action id, not kebab-case) — preserved verbatim, not
    /// "fixed" to kebab, so the wire format stays byte-identical.
    pub enum Block2dCommand for Block2dSnapshot, Block2dMutation, Block2dConfig, Block2dConfigMutation {
        "patchNodeKind" as "patchNodeKind" => patch_node_kind::PatchNodeKind,
        "addHandleKind" as "addHandleKind" => add_handle_kind::AddHandleKind,
        "removeHandleKind" as "removeHandleKind" => remove_handle_kind::RemoveHandleKind,
        "addHandle" as "addHandle" => add_handle::AddHandle,
        "removeHandle" as "removeHandle" => remove_handle::RemoveHandle,
        "addCompatibilityRule" as "addCompatibilityRule" => add_compatibility_rule::AddCompatibilityRule,
        "removeCompatibilityRule" as "removeCompatibilityRule" => remove_compatibility_rule::RemoveCompatibilityRule,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧵️ Every `Block2dCommand` row, without exception — block2d declares no host-only/config-only verb,
/// so the retained route table and `create_block2d_app`'s `Migrated` classification list are the same
/// nine ids (pinned by `retained_route_dispositions_are_exact_and_exhaustive`).
const BLOCK2D_RETAINED_TOOL_IDS: &[&str] = &["patchNodeKind", "addHandleKind", "removeHandleKind", "addHandle", "removeHandle", "addCompatibilityRule", "removeCompatibilityRule", "setActiveExample", "edit"];
const BLOCK2D_RETAINED_PAYLOAD_SCHEMA: &str = "block.2d.tool-command.v1";
const BLOCK2D_RETAINED_RAW_BYTES: usize = 65_536;
const BLOCK2D_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🛣️ Publication lanes per route — every block2d handler emits `Emit::mutations(..)`/`Emit::default()`
/// over `Block2dMutation` only (`🎮️commands/*/🦀️.rs`), never a `Block2dConfigMutation`, so every route
/// publishes into the artifact lane and nothing else.
const BLOCK2D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchNodeKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addHandleKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeHandleKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addHandle", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeHandle", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addCompatibilityRule", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeCompatibilityRule", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn block2d_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BLOCK2D_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500)
}

fn block2d_retained_extent(command: &Block2dCommand, snapshot: &Block2dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !BLOCK2D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let collections = [snapshot.handle_kinds.len(), snapshot.handles.len(), snapshot.compatibility.len(), snapshot.attributes.len(), snapshot.authors.len()];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= BLOCK2D_RETAINED_WORK_ITEMS).then_some(1)
}

fn block2d_retained_reduce(
    command: &Block2dCommand,
    snapshot: &Block2dSnapshot,
    config: &Block2dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::ArtifactOwnedToolJobContext<EditorApp<Block2dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Block2dMutation, Block2dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Block2dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Block2dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BLOCK2D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Block2dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Block2dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Block2dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { BLOCK2D_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { block2d_retained_contract() }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BLOCK2D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Block2d retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Block2dRetainedCommandJobFactory {
    type Owner = EditorApp<Block2dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = BLOCK2D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_2D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BLOCK2D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
struct Block2dStorePreparationFactory;

struct Block2dStorePreparation {
    base: Option<store::SnapshotRead<Block2dSnapshot>>,
    mutation: Option<Block2dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Block2dSnapshot, Block2dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Block2dSnapshot, Block2dMutation> for Block2dStorePreparationFactory {
    fn preflight(&self, _mutation: &Block2dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Block2d Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Block2dSnapshot, Block2dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Block2dSnapshot, Block2dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Block2dSnapshot, Block2dMutation>> {
        let item_count = request
            .base
            .get()
            .handle_kinds
            .len()
            .saturating_add(request.base.get().handles.len())
            .saturating_add(request.base.get().compatibility.len())
            .saturating_add(request.base.get().attributes.len())
            .saturating_add(request.base.get().authors.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || item_count > BLOCK2D_RETAINED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(Block2dStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Block2dSnapshot, Block2dMutation> for Block2dStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Block2d preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Block2d preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Block2d preparation lost its Store authority".to_string())?;
        let id = format!("block2d-retained-{}", authority.next_sequence_number());
        let edit = protocol::Edit {
            id: id.clone(),
            actor: Some(authority.actor().to_string()),
            forwards: vec![mutation],
            inverse,
            mutation_meta: vec![protocol::MutationMeta {
                mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
                dependencies: Vec::new(),
                base_version: authority.base_applied_edit_count() as u64,
                author_id: Some(protocol::ActorId(authority.actor().to_string())),
                timestamp: authority.next_clock(),
                undo_policy: protocol::UndoPolicy::ExactBaseOnly,
                payload_hash: None,
                semantic_kind: None,
                label: None,
                group_id: None,
                origin: Default::default(),
            }],
            description: self.description.take(),
            coalesce_key: None,
            sequence_number: authority.next_sequence_number(),
            started_at: String::new(),
            finished_at: None,
        };
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Block2dSnapshot, Block2dMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Block2dSnapshot, Block2dMutation>> { self.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() { return Err("Block2d preparation could not return its exact base root".into()); }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() { return Ok(store::SnapshotRetirementStep::Blocked); }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🔖️Block2dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `crate::editor::block2d::config::Block2dConfig`, written through `Block2dConfigMutation`s.
#[derive(Default)]
pub struct Block2dPlayApp;

impl ArtifactEditor for Block2dPlayApp {
    type Snapshot = Block2dSnapshot;
    type Mutation = Block2dMutation;
    type Config = Block2dConfig;
    type ConfigMutation = Block2dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::block2d::presence::Block2dPresence;
    type PresenceMutation = crate::editor::block2d::presence::Block2dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block2dCommand;

    const DIALECT: Dialect = crate::BLOCK2D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_2D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Block2dStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Block2dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.block.block2d@1/*#editor",
        document_schema: "block.2d",
        factory: "Block2dRetainedCommandJobFactory",
        factory_type: Block2dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: ["patchNodeKind", "addHandleKind", "removeHandleKind", "addHandle", "removeHandle", "addCompatibilityRule", "removeCompatibilityRule", "setActiveExample", "edit"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Block2dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BLOCK2D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || block2d_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("block2d-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, block2d_retained_reduce, block2d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            Block2dCommand::command_id,
            BLOCK2D_RETAINED_RAW_BYTES,
            BLOCK2D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::block2d::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the bundled `hexagonal-cut-concrete-forest-left` example document (the same DSL
    /// `setActiveExample` parses), so every window renders real content instead of the all-`Default`
    /// empty node kind — see `crate::standards::v1::subsets::any::schema::default_block2d_snapshot`.
    fn initial_snapshot() -> Block2dSnapshot {
        crate::standards::v1::subsets::any::schema::default_block2d_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(block2d_io())
    }

    fn command_id(command: &Block2dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block2dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string);
        match action {
            "patchNodeKind" => Ok(Block2dCommand::PatchNodeKind(patch_node_kind::PatchNodeKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addHandleKind" => Ok(Block2dCommand::AddHandleKind(add_handle_kind::AddHandleKind {})),
            "removeHandleKind" => Ok(Block2dCommand::RemoveHandleKind(remove_handle_kind::RemoveHandleKind { id: str_field("id").unwrap_or_default() })),
            "addHandle" => Ok(Block2dCommand::AddHandle(add_handle::AddHandle {})),
            "removeHandle" => Ok(Block2dCommand::RemoveHandle(remove_handle::RemoveHandle { id: str_field("id").unwrap_or_default() })),
            "addCompatibilityRule" => Ok(Block2dCommand::AddCompatibilityRule(add_compatibility_rule::AddCompatibilityRule { source: str_field("source").unwrap_or_default(), target: str_field("target").unwrap_or_default() })),
            "removeCompatibilityRule" => Ok(Block2dCommand::RemoveCompatibilityRule(remove_compatibility_rule::RemoveCompatibilityRule { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block2dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block2dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Block2dCommand,
        doc: &ArtifactView<'_, Block2dSnapshot>,
        cfg: &ConfigView<'_, Block2dConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Block2dMutation, Block2dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain's
    /// `HierarchyProvider::Topology` — every handle-kind is a root (`handleKind` granularity), every
    /// handle nests under its own `handle_kind` (`handle` granularity), so a stale selection is
    /// pruned the moment `removeHandleKind`/`removeHandle` deletes its target, and hovering/selecting
    /// a kind can transitively reach its handles.
    fn interaction_topology(doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> InteractionTopology {
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for kind in &doc.snapshot.handle_kinds {
            ordered.push(TopologyNode { id: format!("handleKind:{}", kind.id), granularity: BLOCK2D_GRANULARITY_HANDLE_KIND.into(), parent: None });
        }
        for handle in &doc.snapshot.handles {
            ordered.push(TopologyNode { id: format!("handle:{}", handle.id), granularity: BLOCK2D_GRANULARITY_HANDLE.into(), parent: Some(format!("handleKind:{}", handle.handle_kind)) });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK2D_INTERACTION_HANDLE.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block2dSnapshot>, cfg: &ConfigView<'_, Block2dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = block2d_labels(view_state);
        let node = match body_key {
            board::BLOCK2D_BODY_BOARD => board::render(doc.snapshot, labels)?,
            document_panel::BLOCK2D_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels)?,
            inspection_panel::BLOCK2D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block2d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🌉️ `puzzle2d_manifest_fragment`'s first real caller — wraps the block-2d document's
    /// puzzle2d-shaped catalog fragment (`portKinds`/`wireKinds`/`edgeKinds`/`nodeKinds`/
    /// `kindCompatibility`) as a `kit.catalog`-schema `Media` value for the `"catalog:out"` port
    /// declared in `block2d_io`. Falls through to the default whole-document pack export for every
    /// other port (`"document:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block2dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `ArtifactEditor::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this app.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.document_media_type);
            let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = crate::standards::v1::subsets::any::schema::inferences::puzzle2d_manifest_fragment(doc.snapshot);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block2dPlayApp

//#region 🔖️Manifest
pub fn create_block2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::BLOCK2D_DIALECT)
            .document(["semio", "block", "2d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle2d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block2d_io`/`Block2dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "2d".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("layout-grid")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK2D_PLAY_MODE_EDIT)
            .window_kind_def(board::definition())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `handle` domain
            // replaces the deleted `setSelection` view action — the framework auto-injects
            // `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity` for it.
            .interaction(InteractionDefinition {
                id: BLOCK2D_INTERACTION_HANDLE.into(),
                label: LocalizedLabel::native("Handles", "Griffe"),
                granularities: vec![
                    GranularityDefinition { id: BLOCK2D_GRANULARITY_HANDLE.into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "circle-dot".into() },
                    GranularityDefinition { id: BLOCK2D_GRANULARITY_HANDLE_KIND.into(), label: LocalizedLabel::native("Handle Kind", "Griffart"), icon_id: "circle".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace, MergeMode::Additive], transitive: false, broadcast: true },
            })
            .window_kind_interactions(board::BLOCK2D_WINDOW_BOARD, vec![InteractionRef::new(BLOCK2D_INTERACTION_HANDLE)])
            .default_layout(edit_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("patchNodeKind", LocalizedLabel::native("Patch Node Kind", "Knotenart bearbeiten"))
            .mutation("addHandleKind", LocalizedLabel::native("Add Handle Kind", "Griffart hinzufügen"))
            .mutation("removeHandleKind", LocalizedLabel::native("Remove Handle Kind", "Griffart entfernen"))
            .mutation("addHandle", LocalizedLabel::native("Add Handle", "Griff hinzufügen"))
            .mutation("removeHandle", LocalizedLabel::native("Remove Handle", "Griff entfernen"))
            .mutation("addCompatibilityRule", LocalizedLabel::native("Add Compatibility Rule", "Kompatibilitätsregel hinzufügen"))
            .mutation("removeCompatibilityRule", LocalizedLabel::native("Remove Compatibility Rule", "Kompatibilitätsregel entfernen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .action_interactive_job("patchNodeKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("addHandleKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeHandleKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("addHandle", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeHandle", InteractiveJobClassification::Migrated)
            .action_interactive_job("addCompatibilityRule", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeCompatibilityRule", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("edit", InteractiveJobClassification::Migrated)
            .io(block2d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `BLOCK2D_EXAMPLE_LEFT`/`BLOCK2D_EXAMPLE_RIGHT` app-level example registrations and the
            // no-op `.workflow("block2d", …)` call are dropped here (not silently: reported in the
            // packet's migration report). The subset's own pre-existing
            // `🗿️artifacts/◻️2d/…/📚️examples/🎬️hexagonal-cut-concrete-forest-{left,right}` facet is
            // the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
