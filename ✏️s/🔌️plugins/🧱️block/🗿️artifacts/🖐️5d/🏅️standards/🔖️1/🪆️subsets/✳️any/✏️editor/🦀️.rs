//! 👯️ Block 5D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the board/world
//! windows in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`,
//! view state in `🦀️config.rs`, document-side compute in `crate::standards::v1::subsets::any::schema`/
//! `crate::standards::v1::subsets::any::schema::inferences`, and this app's own typed media I/O surface (below —
//! constitutional: general, an artifact must never depend on an app, so it lives here rather than under
//! `🗿️artifacts`).

use crate::standards::v1::subsets::any::schema::mutations::text::Block5dMutation;
use crate::{artifact_kind, Block5dSnapshot, BLOCK_5D_SCHEMA};
use crate::editor::block5d::commands::patch_part_kind;
use crate::editor::block5d::commands::{add_grip, remove_grip};
use crate::editor::block5d::commands::{add_grip_kind, remove_grip_kind};
use crate::editor::block5d::commands::{edit, set_active_example};
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use crate::editor::block5d::modes::edit as edit_mode;
use crate::editor::block5d::modes::edit::windows::{board, world};
use crate::editor::block5d::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::block5d::terminology::block5d_labels;
use semio_framework::{DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError, TopologyNode};
use semio_framework_plugin::app::{Dialect, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, DraftView, Editor, EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation,
};
use dsl::os_pack::json::Value;
use std::collections::BTreeMap;
use store::EngineHandles;

//#region 🔖️Constants
pub const BLOCK5D_PLAY_APP_ID: &str = "block5d-play";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this app's rim-grip templates ("grip" granularity, the default) and grip-kind catalog
/// ("gripKind" granularity) — replaces the deleted `Block5dConfig.selected_ids`.
pub const BLOCK5D_INTERACTION_GRIP: &str = "grip";
pub const BLOCK5D_GRANULARITY_GRIP: &str = "grip";
pub const BLOCK5D_GRANULARITY_GRIP_KIND: &str = "gripKind";
/// 🗂️ The `s/plugin/puzzle` 5d catalog artifact kind block5d's `"catalog:out"` port produces — see
/// `block5d_io` and `Block5dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ One action binding addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎮️commands/*`)? builds its `on_change`/item actions with.
pub fn block5d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(BLOCK5D_PLAY_APP_ID).action(action, args)
}


/// 🏷️ Admits one fixed UI contract label — the type every contract node builder takes, distinct from
/// the SDK's retained authoring `Label`.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block5d UI label admission failed"))
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
/// 🔌️ `Block5dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"5d.block"` artifact kind) plus a `"catalog:out"` port giving
/// `puzzle5d_catalog_fragment` a real caller (see `export_media` below).
pub fn block5d_io() -> semio_framework_plugin::AppIo {
    let io = semio_framework_plugin::resolve_ready(semio_framework_plugin::AppIo::from_document(
        BLOCK_5D_SCHEMA,
        MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        semio_framework_plugin::ArtifactPresentation { id: "5d.block".into(), name: "Part Kind".into(), dimension: "5d".into(), component_kind: "block5d".into() },
    ));
    semio_framework_plugin::resolve_ready(io.with_ports(vec![semio_framework_plugin::MediaPortSpec {
        id: "catalog:out".into(),
        label: "Kit Catalog".into(),
        direction: semio_framework_plugin::MediaPortDirection::Out,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        kind_id: Some("kit.catalog".into()),
        required: false,
        multiplicity: semio_framework_plugin::PortMultiplicity::Many,
    }]))
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block5dPlayApp::Command` — the SOLE dispatch surface for block5d's own behavior, covering
    /// every action `create_block5d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair here is IDENTICAL (the pre-migration
    /// `#[dsl(key)]` already used the camelCase action id, not kebab-case) — preserved verbatim.
    pub enum Block5dCommand for Block5dSnapshot, Block5dMutation, Block5dConfig, Block5dConfigMutation {
        "patchPartKind" as "patchPartKind" => patch_part_kind::PatchPartKind,
        "addGripKind" as "addGripKind" => add_grip_kind::AddGripKind,
        "removeGripKind" as "removeGripKind" => remove_grip_kind::RemoveGripKind,
        "addGrip" as "addGrip" => add_grip::AddGrip,
        "removeGrip" as "removeGrip" => remove_grip::RemoveGrip,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const BLOCK5D_RETAINED_TOOL_IDS: &[&str] = &["patchPartKind", "addGripKind", "removeGripKind", "addGrip", "removeGrip", "setActiveExample", "edit"];
const BLOCK5D_RETAINED_PAYLOAD_SCHEMA: &str = "block.5d.tool-command.v1";
const BLOCK5D_RETAINED_RAW_BYTES: usize = 65_536;
const BLOCK5D_RETAINED_WORK_ITEMS: usize = 4_096;
const BLOCK5D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchPartKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addGripKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeGripKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addGrip", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeGrip", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn block5d_retained_extent(command: &Block5dCommand, snapshot: &Block5dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !BLOCK5D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let collections = [snapshot.representations.len(), snapshot.grip_kinds.len(), snapshot.grips.len(), snapshot.compatibility.len(), snapshot.attributes.len(), snapshot.authors.len()];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= BLOCK5D_RETAINED_WORK_ITEMS).then_some(1)
}

fn block5d_retained_reduce(
    command: &Block5dCommand,
    snapshot: &Block5dSnapshot,
    config: &Block5dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Block5dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Block5dMutation, Block5dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Block5dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Block5dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BLOCK5D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Block5dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Block5dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Block5dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { BLOCK5D_RETAINED_PAYLOAD_SCHEMA }
    fn classification(&self) -> InteractiveJobClassification { InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::bounded_first_step(BLOCK5D_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { Ok(ArtifactRetainedCommandJob::new(payload)) }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BLOCK5D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Block5d retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Block5dRetainedCommandJobFactory {
    type Owner = EditorApp<Block5dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = BLOCK5D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_5D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BLOCK5D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
struct Block5dStorePreparationFactory;

struct Block5dStorePreparation {
    base: Option<store::SnapshotRead<Block5dSnapshot>>,
    mutation: Option<Block5dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Block5dSnapshot, Block5dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Block5dSnapshot, Block5dMutation> for Block5dStorePreparationFactory {
    fn preflight(&self, _mutation: &Block5dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Block5d Store preparation rejected its lane or description envelope".into());
        }
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Block5dSnapshot, Block5dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Block5dSnapshot, Block5dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Block5dSnapshot, Block5dMutation>> {
        let item_count = request
            .base
            .get()
            .representations
            .len()
            .saturating_add(request.base.get().grip_kinds.len())
            .saturating_add(request.base.get().grips.len())
            .saturating_add(request.base.get().compatibility.len())
            .saturating_add(request.base.get().attributes.len())
            .saturating_add(request.base.get().authors.len());
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || item_count > BLOCK5D_RETAINED_WORK_ITEMS
        {
            return Err(request);
        }
        Ok(Box::new(Block5dStorePreparation {
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

impl store::ArtifactStoreOneItemPreparation<Block5dSnapshot, Block5dMutation> for Block5dStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::Mutation as _;
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Block5d preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Block5d preparation lost its mutation owner".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Block5d preparation lost its Store authority".to_string())?;
        let id = format!("block5d-retained-{}", authority.next_sequence_number());
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
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Block5dSnapshot, Block5dMutation>> { self.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Block5dSnapshot, Block5dMutation>> { self.prepared.take() }
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
            if !base.return_to_registry() { return Err("Block5d preparation could not return its exact base root".into()); }
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

//#region 🔖️Block5dPlayApp
/// 🧪️ B1: unit struct — the former `selected_ids` `RefCell` field now lives in
/// `crate::editor::block5d::config::Block5dConfig`, written through `Block5dConfigMutation`s.
#[derive(Default)]
pub struct Block5dPlayApp;

impl ArtifactEditor for Block5dPlayApp {
    type Snapshot = Block5dSnapshot;
    type Mutation = Block5dMutation;
    type Config = Block5dConfig;
    type ConfigMutation = Block5dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::block5d::presence::Block5dPresence;
    type PresenceMutation = crate::editor::block5d::presence::Block5dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block5dCommand;

    const DIALECT: Dialect = crate::BLOCK5D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_5D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Block5dStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Block5dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.block.block5d@1/*#editor",
        document_schema: "block.5d",
        factory: "Block5dRetainedCommandJobFactory",
        factory_type: Block5dRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: ["patchPartKind", "addGripKind", "removeGripKind", "addGrip", "removeGrip", "setActiveExample", "edit"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Block5dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BLOCK5D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || block5d_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("block5d-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, block5d_retained_reduce, block5d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: None, operation: operation_context, completion: request.completion },
            Block5dCommand::command_id,
            BLOCK5D_RETAINED_RAW_BYTES,
            BLOCK5D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::block5d::config::schema::app_schema_descriptor())
    }

    /// 📄️ Boots on the bundled `hexagonal-cut-concrete-forest-left` example document (the same DSL
    /// `setActiveExample` parses), so the board window shows a part kind and the World3d window a
    /// `mesh_url` instead of the all-`Default` empty part kind — see
    /// `crate::standards::v1::subsets::any::schema::default_block5d_snapshot`.
    fn initial_snapshot() -> Block5dSnapshot {
        crate::standards::v1::subsets::any::schema::default_block5d_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(block5d_io())
    }

    fn command_id(command: &Block5dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block5dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map(dsl::os_pack::json::from_dsl_value);
        let str_field = |key: &str| args.as_ref().and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "patchPartKind" => Ok(Block5dCommand::PatchPartKind(patch_part_kind::PatchPartKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addGripKind" => Ok(Block5dCommand::AddGripKind(add_grip_kind::AddGripKind {})),
            "removeGripKind" => Ok(Block5dCommand::RemoveGripKind(remove_grip_kind::RemoveGripKind { id: str_field("id").unwrap_or_default() })),
            "addGrip" => Ok(Block5dCommand::AddGrip(add_grip::AddGrip {})),
            "removeGrip" => Ok(Block5dCommand::RemoveGrip(remove_grip::RemoveGrip { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block5dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block5dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            other => Err(Fault::from(format!(
                "action '{other}' is not a framework-reserved action (history/clipboard/revert/filter/noteShellCommand) — \
                 app actions are dispatched exclusively through the typed command channel now (see `dispatch_typed_command`)"
            ))),
        }
    }

    fn handle(
        command: &Block5dCommand,
        doc: &ArtifactView<'_, Block5dSnapshot>,
        cfg: &ConfigView<'_, Block5dConfig>,
        _interaction: &InteractionView<'_>, _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Block5dMutation, Block5dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain's
    /// `HierarchyProvider::Topology` — every grip-kind is a root (`gripKind` granularity), every grip
    /// nests under its own `grip_kind` (`grip` granularity), so a stale selection is pruned the moment
    /// `removeGripKind`/`removeGrip` deletes its target, and hovering/selecting a kind can transitively
    /// reach its grips.
    fn interaction_topology(doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> InteractionTopology {
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for kind in &doc.snapshot.grip_kinds {
            ordered.push(TopologyNode { id: format!("gripKind:{}", kind.id), granularity: BLOCK5D_GRANULARITY_GRIP_KIND.into(), parent: None });
        }
        for grip in &doc.snapshot.grips {
            ordered.push(TopologyNode { id: format!("grip:{}", grip.id), granularity: BLOCK5D_GRANULARITY_GRIP.into(), parent: Some(format!("gripKind:{}", grip.grip_kind)) });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK5D_INTERACTION_GRIP.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block5dSnapshot>, cfg: &ConfigView<'_, Block5dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = block5d_labels(view_state);
        let node = match body_key {
            board::BLOCK5D_BODY_BOARD => board::render(doc.snapshot, labels)?,
            world::BLOCK5D_BODY_WORLD => world::render(doc.snapshot, labels)?,
            document_panel::BLOCK5D_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels)?,
            inspection_panel::BLOCK5D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}")))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block5d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🌉️ `puzzle5d_catalog_fragment`'s first real caller — wraps the block-5d document's
    /// puzzle5d-shaped catalog fragment (`parts`/`grips`/`fasteners`/`ropes`/`kindCompatibility`) as
    /// a `kit.catalog`-schema `Media` value for the `"catalog:out"` port declared in `block5d_io`.
    /// Falls through to the default whole-document pack export for every other port
    /// (`"document:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block5dSnapshot>) -> Result<Media, MediaError> {
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
        let fragment = crate::standards::v1::subsets::any::schema::inferences::puzzle5d_catalog_fragment(doc.snapshot);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block5dPlayApp

//#region 🔖️Manifest
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4: `Editor::builder`
/// takes only `BLOCK5D_DIALECT` (the surface id/label are derived, never hand-written) and the
/// chain ends with `.build_definition()` returning `AppDefinition` directly — no more
/// `App::from_builder(...)` wrapper. `EditorBuilder` has neither `.example(...)` nor `.workflow(...)`
/// (contract §2.4's `App { definition, examples }` split — `.editor::<E>(def)` only ever takes the
/// bare definition), so the two `.example(BLOCK5D_EXAMPLE_*, …)` calls and the no-op
/// `.workflow("block5d", …)` call this app used to end with are DROPPED here, not silently ported —
/// the subset's own `📚️examples/🎬️{hexagonal-cut-concrete-forest-left,nakagin-capsule}` facet
/// (untouched, already wired in `🦀️.rs`'s Examples region) is the modern, role-agnostic
/// replacement surface for app-level example registration.
pub fn create_block5d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::BLOCK5D_DIALECT)
            .document(["semio", "block", "5d"])
            .artifact_kind(artifact_kind())
            // 🗂️ The puzzle5d catalog artifact this app's new `"catalog:out"` port produces — see
            // `block5d_io`/`Block5dPlayApp::export_media`.
            .artifact_kind(ArtifactKindSpec {
                id: KIT_CATALOG_ARTIFACT_ID.into(),
                name: "Kit Catalog".into(),
                source_format: KIT_CATALOG_ARTIFACT_ID.into(),
                component_kind: "kit-catalog".into(),
                dimension: "5d".into(),
                media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                schema: KIT_CATALOG_ARTIFACT_ID.into(),
                export_formats: vec![],
                import_formats: vec![],
                    export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    })
            .icon_id("layers")
            .mode_def(edit_mode::definition())
            .default_mode_id(edit_mode::BLOCK5D_PLAY_MODE_EDIT)
            .window_kind_def(board::definition())
            .window_kind_def(world::definition())
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `grip` domain
            // replaces the deleted `setSelection` view action — the framework auto-injects
            // `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/`setSelectionMode`/
            // `setInteractionGranularity` for it.
            .interaction(InteractionDefinition {
                id: BLOCK5D_INTERACTION_GRIP.into(),
                label: LocalizedLabel::native("Grips", "Griffe"),
                granularities: vec![
                    GranularityDefinition { id: BLOCK5D_GRANULARITY_GRIP.into(), label: LocalizedLabel::native("Grip", "Griff"), icon_id: "circle-dot".into() },
                    GranularityDefinition { id: BLOCK5D_GRANULARITY_GRIP_KIND.into(), label: LocalizedLabel::native("Grip Kind", "Griffart"), icon_id: "circle".into() },
                ],
                hierarchy: HierarchyProvider::Topology,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace, MergeMode::Additive], transitive: false, broadcast: true },
            })
            .window_kind_interactions(board::BLOCK5D_WINDOW_BOARD, vec![InteractionRef::new(BLOCK5D_INTERACTION_GRIP)])
            .window_kind_interactions(world::BLOCK5D_WINDOW_WORLD, vec![InteractionRef::new(BLOCK5D_INTERACTION_GRIP)])
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .mutation("patchPartKind", LocalizedLabel::native("Patch Part Kind", "Teilart bearbeiten"))
            .mutation("addGripKind", LocalizedLabel::native("Add Grip Kind", "Griffart hinzufügen"))
            .mutation("removeGripKind", LocalizedLabel::native("Remove Grip Kind", "Griffart entfernen"))
            .mutation("addGrip", LocalizedLabel::native("Add Grip", "Griff hinzufügen"))
            .mutation("removeGrip", LocalizedLabel::native("Remove Grip", "Griff entfernen"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
            .action_interactive_job("patchPartKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGripKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGripKind", InteractiveJobClassification::Migrated)
            .action_interactive_job("addGrip", InteractiveJobClassification::Migrated)
            .action_interactive_job("removeGrip", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
            .action_interactive_job("edit", InteractiveJobClassification::Migrated)
            .default_layout(edit_mode::layout())
            .io(block5d_io())
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
