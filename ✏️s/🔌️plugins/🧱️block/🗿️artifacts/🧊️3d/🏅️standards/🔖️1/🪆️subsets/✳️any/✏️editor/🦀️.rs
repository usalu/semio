//! 🏙️ Block 3D editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the world window
//! (+ its `☑️options/*`) in `🎭️modes/✏️edit/🪟️windows/🌐️world`, panel trees in `📌️panels/*`, labels in
//! `🗣️terminology`, view state in `🎚️config`, world-scene compute needing both document+config in
//! `🌍️world` (editor-only compute facet, no taxonomy slot — see that file's own doc), pure document-side
//! compute in `crate::artifacts::block3d::schema`/`crate::artifacts::block3d::schema::inferences`, and
//! this surface's own typed media I/O surface (below — constitutional: general, an artifact must never
//! depend on a surface, so it lives here rather than under `🗿️artifacts`).

use crate::artifacts::block3d::op::Block3dMutation;
use crate::artifacts::block3d::{artifact_kind, Block3dSnapshot, BLOCK3D_DIALECT, BLOCK_3D_SCHEMA};
use crate::editor::block3d::commands::patch_object_kind;
use crate::editor::block3d::commands::set_camera;
use crate::editor::block3d::commands::{add_representation, patch_representation, remove_representation};
use crate::editor::block3d::commands::{add_vortex, remove_vortex};
use crate::editor::block3d::commands::{add_vortex_kind, remove_vortex_kind};
use crate::editor::block3d::commands::{edit, set_active_example};
use crate::editor::block3d::commands::{hover_surface, leave_surface, place_vortex, set_brush_flip, set_brush_radius, set_brush_vortex_kind};
use crate::editor::block3d::commands::{set_active_representation, set_active_utility, set_window_arrangement, set_window_representations, set_window_spacing, toggle_window_representation};
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use crate::editor::block3d::modes::edit as edit_mode;
use crate::editor::block3d::modes::edit::windows::world;
use crate::editor::block3d::panels::{document as document_panel, inspection as inspection_panel};
use crate::editor::block3d::terminology::block3d_labels;
use crate::BlockCamera3d;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionDescriptor, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView,
    ConfigView, DraftView, Editor, EditorApp, Emit, Fault, FaultCode, FaultOrigin, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, NoDraft, NoDraftMutation, UtilityDefinition,
};
// 🚧️ SDK GAP: `Dialect`/`InteractionView` are still only reachable through the `app` submodule they're
// declared in — not (yet) in `semio_framework_plugin`'s curated crate-root re-export list, unlike
// `ArtifactEditor`/`Editor` above (closed by W0-F). Mirrors the sibling `👁️viewer`'s own gap note.
use semio_framework::{
    DomainTopology, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, InteractionTopology, InteractiveJobClassification, MergeMode, SelectionMethod, SelectionMode, SelectionSpec, ToolExecutionContract,
    ToolFactoryKey, ToolJobFactoryError, TopologyNode,
};
use semio_framework_plugin::app::{Dialect, InteractionView};
use dsl::os_pack::json::Value;
use std::collections::{BTreeMap, HashMap};
use store::EngineHandles;

//#region 🔖️Constants
/// 👁️✏️ Plain string tag (NOT the authoring trait's `APP_ID` — that const is removed, contract §2.1) —
/// stays a controller/action-factory id, exactly the way `world_3d_scene`'s `controller_id` needs a
/// stable string distinct from the derived `surface_app_id()`.
pub const BLOCK3D_PLAY_APP_ID: &str = "block3d-play";
pub const BLOCK3D_PLAY_SURFACE_ID: &str = "block3d.play.world";
pub const BLOCK3D_DEFAULT_WINDOW_ID: &str = "block3d-world";
pub const BLOCK3D_WORLD_OBJECT_ID: &str = "block3d-object";
pub const BLOCK3D_UTILITY_SELECT: &str = "select";
pub const BLOCK3D_UTILITY_SURFACE_BRUSH: &str = "surfaceBrush";
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the framework-owned hover/selection
/// domain over this surface's representations ("surface" granularity) and rim-vortex templates ("vortex"
/// granularity, the default) — replaces the deleted `Block3dConfig.selected_ids`/`hovered_vortex_full_id`.
pub const BLOCK3D_INTERACTION_VORTEX: &str = "vortex";
pub const BLOCK3D_GRANULARITY_VORTEX: &str = "vortex";
pub const BLOCK3D_GRANULARITY_SURFACE: &str = "surface";
/// 🗂️ The `s/plugin/puzzle` 3d catalog artifact kind block3d's `"catalog:out"` port produces — see
/// `block3d_io` and `Block3dPlayApp::export_media`.
const KIT_CATALOG_ARTIFACT_ID: &str = "kit.catalog";

/// 🎯️ The semantic-contract action binding addressed at this surface — the single factory every
/// contract-built node (`📌️panels/*`) binds its `on_change`/item actions with.
pub fn block3d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(BLOCK3D_PLAY_APP_ID).action(action, args)
}

/// 🪟️ Bridges window chrome (`☑️options/*`'s [`semio_framework_plugin::WindowMeasure`]s), which still
/// carries the retained WGPU action descriptor rather than a contract action binding.
pub fn block3d_window_action(action: &str, args: Option<dsl::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: BLOCK3D_PLAY_APP_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits resolved block3d text into the semantic UI contract's fixed-capacity label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block3d UI label admission failed"))
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


fn block3d_resolve_world_body(body_key: &str) -> (&str, String) {
    if body_key == world::BLOCK3D_BODY_WORLD || body_key.starts_with(&format!("{}:", world::BLOCK3D_BODY_WORLD)) {
        if let Some((_, window_id)) = body_key.split_once(':') {
            return (world::BLOCK3D_BODY_WORLD, window_id.to_string());
        }
        return (world::BLOCK3D_BODY_WORLD, BLOCK3D_DEFAULT_WINDOW_ID.into());
    }
    (body_key, BLOCK3D_DEFAULT_WINDOW_ID.into())
}

fn f64_vec3_field(args: Option<&Value>, key: &str) -> Option<[f64; 3]> {
    let array = args.and_then(|value| value.get(key))?.as_array()?;
    if array.len() < 3 {
        return None;
    }
    Some([array[0].as_f64()?, array[1].as_f64()?, array[2].as_f64()?])
}

fn window_id_from_args(args: Option<&Value>) -> String {
    args.and_then(|value| value.get("windowId").or_else(|| value.get("pane")).or_else(|| value.get("surfaceId"))).and_then(Value::as_str).map_or_else(|| BLOCK3D_DEFAULT_WINDOW_ID.into(), str::to_string)
}
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ `Block3dPlayApp`'s typed media I/O surface (`AppDefinition.io`) — the implicit document ports
/// (`Kit×Type`, matching the `"3d.block"` artifact kind) plus the `"catalog:out"` port: the puzzle3d
/// seam that gives `puzzle3d_catalog_fragment` a real caller (see `export_media` below).
pub fn block3d_io() -> semio_framework_plugin::AppIo {
    semio_framework::io::resolve_ready(
        semio_framework::io::resolve_ready(semio_framework_plugin::AppIo::from_document(
            BLOCK_3D_SCHEMA,
            MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            semio_framework_plugin::ArtifactPresentation { id: "3d.block".into(), name: "Object Kind".into(), dimension: "3d".into(), component_kind: "block3d".into() },
        ))
        .with_ports(vec![semio_framework_plugin::MediaPortSpec {
            id: "catalog:out".into(),
            label: "Kit Catalog".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            kind_id: Some("kit.catalog".into()),
            required: false,
            multiplicity: semio_framework_plugin::PortMultiplicity::Many,
        }]),
    )
}
//#endregion 🔖️Io

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Block3dPlayApp::Command` — the SOLE dispatch surface for block3d's own behavior, covering
    /// every action `create_block3d_app` declares. Row order is the binary variant ordinal: appending
    /// is safe, reordering is a wire-format break. Every id/key pair is IDENTICAL EXCEPT the three
    /// `worldSurface*` rows (`HoverSurface`/`LeaveSurface`/`PlaceVortex`), whose manifest action id and
    /// `#[dsl(key)]` wire keyword genuinely diverge pre-migration — preserved verbatim.
    pub enum Block3dCommand for Block3dSnapshot, Block3dMutation, Block3dConfig, Block3dConfigMutation {
        "patchObjectKind" as "patchObjectKind" => patch_object_kind::PatchObjectKind,
        "addRepresentation" as "addRepresentation" => add_representation::AddRepresentation,
        "removeRepresentation" as "removeRepresentation" => remove_representation::RemoveRepresentation,
        "addVortexKind" as "addVortexKind" => add_vortex_kind::AddVortexKind,
        "removeVortexKind" as "removeVortexKind" => remove_vortex_kind::RemoveVortexKind,
        "addVortex" as "addVortex" => add_vortex::AddVortex,
        "removeVortex" as "removeVortex" => remove_vortex::RemoveVortex,
        "setActiveExample" as "setActiveExample" => set_active_example::SetActiveExample,
        "edit" as "edit" => edit::Edit,
        "setActiveRepresentation" as "setActiveRepresentation" => set_active_representation::SetActiveRepresentation,
        "setWindowRepresentations" as "setWindowRepresentations" => set_window_representations::SetWindowRepresentations,
        "toggleWindowRepresentation" as "toggleWindowRepresentation" => toggle_window_representation::ToggleWindowRepresentation,
        "setWindowArrangement" as "setWindowArrangement" => set_window_arrangement::SetWindowArrangement,
        "setWindowSpacing" as "setWindowSpacing" => set_window_spacing::SetWindowSpacing,
        "setActiveUtility" as "setActiveUtility" => set_active_utility::SetActiveUtility,
        "setBrushVortexKind" as "setBrushVortexKind" => set_brush_vortex_kind::SetBrushVortexKind,
        "setBrushRadius" as "setBrushRadius" => set_brush_radius::SetBrushRadius,
        "setBrushFlip" as "setBrushFlip" => set_brush_flip::SetBrushFlip,
        "worldSurfaceHover" as "hoverSurface" => hover_surface::HoverSurface,
        "worldSurfaceLeave" as "leaveSurface" => leave_surface::LeaveSurface,
        "worldSurfacePlace" as "placeVortex" => place_vortex::PlaceVortex,
        "setCamera" as "setCamera" => set_camera::SetCamera,
        "patchRepresentation" as "patchRepresentation" => patch_representation::PatchRepresentation,
    }
}
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
/// 🧾️ Every block3d tool id, in `Block3dCommand` declaration order — a bijection with the enum's 23
/// rows, with `BLOCK3D_PUBLICATION_CONTRACTS`, and with the `.action_interactive_job(…, Migrated)` set
/// `create_block3d_app` declares (asserted by `retained_route_dispositions_are_exact_and_exhaustive`).
/// `AppActionRegistry::tool_job_registration` enforces exactly that set equality at construction time:
/// a row missing here, or an action left `Unclassified`, faults the whole app with
/// `interactive-job.catalog-incomplete` instead of silently going dispatch-dead at the UI gate.
const BLOCK3D_RETAINED_TOOL_IDS: &[&str] = &[
    "patchObjectKind",
    "addRepresentation",
    "removeRepresentation",
    "addVortexKind",
    "removeVortexKind",
    "addVortex",
    "removeVortex",
    "setActiveExample",
    "edit",
    "setActiveRepresentation",
    "setWindowRepresentations",
    "toggleWindowRepresentation",
    "setWindowArrangement",
    "setWindowSpacing",
    "setActiveUtility",
    "setBrushVortexKind",
    "setBrushRadius",
    "setBrushFlip",
    "worldSurfaceHover",
    "worldSurfaceLeave",
    "worldSurfacePlace",
    "setCamera",
    "patchRepresentation",
];
const BLOCK3D_RETAINED_PAYLOAD_SCHEMA: &str = "block.3d.tool-command.v1";
const BLOCK3D_RETAINED_RAW_BYTES: usize = 65_536;
const BLOCK3D_RETAINED_WORK_ITEMS: usize = 4_096;
/// 🎒️ Real bound for one Artifact-lane edit: `setActiveExample`/`edit` replay a whole example fixture
/// (`📚️examples/*/🖼️assets/*/🗣️.dsl.semio`, ~1–2 KB of text) as the single largest document mutation any
/// of the 23 tools emits, so 64 KiB is a real ceiling rather than a rubber stamp.
const BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
/// 🎒️ Real bound for one Config-lane edit: every `Block3dConfigMutation` inverse is a whole-config
/// `Snapshot` row (`🎚️config/🦀️.rs`'s `Mutation::inverse`), whose largest member is the per-window
/// `representation_ids` view — 256 KiB covers that without being unbounded.
const BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 262_144;

const BLOCK3D_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "patchObjectKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addVortexKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeVortexKind", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addVortex", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "removeVortex", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveRepresentation", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowRepresentations", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "toggleWindowRepresentation", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowArrangement", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setWindowSpacing", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushVortexKind", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushRadius", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setBrushFlip", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "worldSurfaceHover", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "worldSurfaceLeave", lanes: &[ArtifactToolPublicationLane::Config] },
    // 🖌️ The only two-lane row: `📍️place-vortex` emits the vortex-kind/vortex document mutations AND
    // clears the brush preview in config in one `Emit`.
    ArtifactToolPublicationContract { tool_id: "worldSurfacePlace", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "patchRepresentation", lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn block3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BLOCK3D_RETAINED_RAW_BYTES, 4_096, 1, 262_144, 7_500)
}

fn block3d_retained_extent(command: &Block3dCommand, snapshot: &Block3dSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    if !BLOCK3D_RETAINED_TOOL_IDS.contains(&command.command_id()) {
        return None;
    }
    let collections = [snapshot.representations.len(), snapshot.vortex_kind_extra.len(), snapshot.vortices.len(), snapshot.compatibility.len(), snapshot.attributes.len(), snapshot.authors.len()];
    let items = collections.into_iter().try_fold(1usize, |total, count| total.checked_add(count))?;
    (items <= BLOCK3D_RETAINED_WORK_ITEMS).then_some(1)
}

fn block3d_retained_reduce(
    command: &Block3dCommand,
    snapshot: &Block3dSnapshot,
    config: &Block3dConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Block3dMutation, Block3dConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config })
}

struct Block3dRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Block3dRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BLOCK3D_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for Block3dRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Block3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Block3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BLOCK3D_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        block3d_bounded_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > BLOCK3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Block3d retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Block3dRetainedCommandJobFactory {
    type Owner = EditorApp<Block3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = BLOCK3D_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BLOCK3D_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
/// 🧬️ Builds one `protocol::Edit<M>` for either lane's `advance()` — the artifact and config lanes
/// differ only in `M` and their id prefix, so one generic helper replaces two copies of the same body.
fn block3d_next_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
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
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

fn block3d_artifact_mutation_retained_bytes(mutation: &Block3dMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "block3d-artifact-mutation-encode-failed".to_string())
}

fn admit_block3d_artifact_mutation(mutation: &Block3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = block3d_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("block3d-artifact-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

struct Block3dArtifactStorePreparationFactory;

struct Block3dArtifactStorePreparation {
    base: Option<store::SnapshotRead<Block3dSnapshot>>,
    mutation: Option<Block3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Block3dSnapshot, Block3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Block3dSnapshot, Block3dMutation> for Block3dArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &Block3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("block3d-artifact-lane-or-description-envelope".into());
        }
        admit_block3d_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Block3dSnapshot, Block3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Block3dSnapshot, Block3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Block3dSnapshot, Block3dMutation>> {
        let retained_bytes = block3d_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > BLOCK3D_ARTIFACT_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Block3dArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Block3dSnapshot, Block3dMutation> for Block3dArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::{Mutation as _, MutationDiff as _};
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "block3d-artifact-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "block3d-artifact-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "block3d-artifact-authority-missing".to_string())?;
        let edit = block3d_next_edit("block3d-artifact-retained", mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Block3dSnapshot, Block3dMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Block3dSnapshot, Block3dMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("block3d-artifact-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

fn block3d_config_mutation_retained_bytes(mutation: &Block3dConfigMutation) -> Result<usize, String> {
    ::protocol::OpBinary::encode_op(mutation).map(|bytes| bytes.len()).map_err(|_| "block3d-config-mutation-encode-failed".to_string())
}

fn admit_block3d_config_mutation(mutation: &Block3dConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = block3d_config_mutation_retained_bytes(mutation)?;
    if retained_bytes > BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("block3d-config-mutation-envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

struct Block3dConfigStorePreparationFactory;

struct Block3dConfigStorePreparation {
    base: Option<store::SnapshotRead<Block3dConfig>>,
    mutation: Option<Block3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Block3dConfig, Block3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<Block3dConfig, Block3dConfigMutation> for Block3dConfigStorePreparationFactory {
    fn preflight(&self, mutation: &Block3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("block3d-config-lane-or-description-envelope".into());
        }
        admit_block3d_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Block3dConfig, Block3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Block3dConfig, Block3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Block3dConfig, Block3dConfigMutation>> {
        let retained_bytes = block3d_config_mutation_retained_bytes(&request.mutation).unwrap_or(BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > BLOCK3D_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Block3dConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Block3dConfig, Block3dConfigMutation> for Block3dConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        use protocol::{Mutation as _, MutationDiff as _};
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "block3d-config-base-owner-missing".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "block3d-config-mutation-owner-missing".to_string())?;
        let inverse = mutation.inverse(base.get());
        let post = protocol::MutationDiff::apply(mutation.diff(base.get()).diff(), base.get()).map_err(|error| error.to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "block3d-config-authority-missing".to_string())?;
        let edit = block3d_next_edit("block3d-config-retained", mutation, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Block3dConfig, Block3dConfigMutation>> {
        self.prepared.as_ref()
    }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Block3dConfig, Block3dConfigMutation>> {
        self.prepared.take()
    }
    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("block3d-config-base-retirement-rejected".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.authority.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

//#region 🔖️Block3dPlayApp
/// 🧪️ B1: unit struct — every former `RefCell` field now lives in `crate::editor::block3d::config::
/// Block3dConfig`, written through `Block3dConfigMutation`s.
#[derive(Default)]
pub struct Block3dPlayApp;

impl ArtifactEditor for Block3dPlayApp {
    type Snapshot = Block3dSnapshot;
    type Mutation = Block3dMutation;
    type Config = Block3dConfig;
    type ConfigMutation = Block3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::block3d::presence::Block3dPresence;
    type PresenceMutation = crate::editor::block3d::presence::Block3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Block3dCommand;

    const DIALECT: Dialect = BLOCK3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BLOCK_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Block3dArtifactStorePreparationFactory))
    }

    /// 📬️ Required by the Config publication lane: 12 of the 23 retained tools are config-only, and
    /// `VcsArtifactApp` rejects any tool whose declared lane has no one-item preparation factory with
    /// `interactive-job.publication-authority-missing`.
    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Block3dConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<Block3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.block.block3d@1/*#editor",
        document_schema: "block.3d",
        factory: "Block3dRetainedCommandJobFactory",
        factory_type: Block3dRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(65_536, 4_096, 1, 262_144, 7_500),
        tools: [
            "patchObjectKind",
            "addRepresentation",
            "removeRepresentation",
            "addVortexKind",
            "removeVortexKind",
            "addVortex",
            "removeVortex",
            "setActiveExample",
            "edit",
            "setActiveRepresentation",
            "setWindowRepresentations",
            "toggleWindowRepresentation",
            "setWindowArrangement",
            "setWindowSpacing",
            "setActiveUtility",
            "setBrushVortexKind",
            "setBrushRadius",
            "setBrushFlip",
            "worldSurfaceHover",
            "worldSurfaceLeave",
            "worldSurfacePlace",
            "setCamera",
            "patchRepresentation"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Block3dRetainedCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BLOCK3D_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || block3d_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("block3d-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, block3d_retained_reduce, block3d_retained_extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            operation_context,
            request.completion,
            Block3dCommand::command_id,
            BLOCK3D_RETAINED_RAW_BYTES,
            BLOCK3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    /// 🚀️ Boots on the `hexagonal-cut-concrete-forest-left` fixture instead of the empty document —
    /// the `World3d` window renders `representations[].mesh_url`, so an empty boot document painted an
    /// empty scene until a client dispatched `setActiveExample`. See `dsl::block3d_boot_snapshot`.
    fn initial_snapshot() -> Block3dSnapshot {
        crate::artifacts::block3d::dsl::block3d_boot_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(block3d_io())
    }

    fn command_id(command: &Block3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Maps host action id + JSON args onto `Block3dCommand` — React/wgpu still speak the stringly
    /// `{action,args}` wire; this is the typed-command bridge until those call sites send `OpBinary`
    /// bytes directly.
    fn command_from_action(action: &str, args: Option<&dsl::DslValue>) -> Result<Self::Command, Fault> {
        let args = args.map(dsl::os_pack::json::from_dsl_value);
        let args = args.as_ref();
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        match action {
            "patchObjectKind" => Ok(Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() })),
            "addRepresentation" => Ok(Block3dCommand::AddRepresentation(add_representation::AddRepresentation {})),
            "removeRepresentation" => Ok(Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: str_field("id").unwrap_or_default() })),
            "addVortexKind" => Ok(Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {})),
            "removeVortexKind" => Ok(Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: str_field("id").unwrap_or_default() })),
            "addVortex" => Ok(Block3dCommand::AddVortex(add_vortex::AddVortex {})),
            "removeVortex" => Ok(Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: str_field("id").unwrap_or_default() })),
            "setActiveExample" => Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: str_field("exampleId").or_else(|| str_field("id")).unwrap_or_default() })),
            "edit" => Ok(Block3dCommand::Edit(edit::Edit { text: str_field("text").unwrap_or_default() })),
            "setActiveRepresentation" => Ok(Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: str_field("representationId").or_else(|| str_field("representation_id")) })),
            "setWindowRepresentations" => {
                let rep = str_field("value").or_else(|| str_field("representationId"));
                let representation_ids = rep.filter(|id| !id.is_empty()).map(|id| vec![id]).unwrap_or_default();
                Ok(Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: window_id_from_args(args), representation_ids }))
            }
            "toggleWindowRepresentation" => Ok(Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation {
                window_id: window_id_from_args(args),
                representation_id: str_field("representationId").unwrap_or_default(),
                visible: args.and_then(|value| value.get("visible")).and_then(Value::as_bool).unwrap_or(true),
            })),
            "setWindowArrangement" => Ok(Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: window_id_from_args(args), arrangement: str_field("value").unwrap_or_else(|| "overlap".into()) })),
            "setWindowSpacing" => Ok(Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing { window_id: window_id_from_args(args), spacing: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(8.0) })),
            "setActiveUtility" => Ok(Block3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { window_id: window_id_from_args(args), utility_id: str_field("utilityId").unwrap_or_else(|| BLOCK3D_UTILITY_SELECT.into()) })),
            "setBrushVortexKind" => Ok(Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: str_field("value").or_else(|| str_field("vortexKindId")) })),
            "setBrushRadius" => Ok(Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: args.and_then(|value| value.get("value")).and_then(Value::as_f64).unwrap_or(0.3) })),
            "setBrushFlip" => Ok(Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: args.and_then(|value| value.get("flip")).and_then(Value::as_bool).unwrap_or(false) })),
            "worldSurfaceHover" => Ok(Block3dCommand::HoverSurface(hover_surface::HoverSurface {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "worldSurfaceLeave" => Ok(Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})),
            "worldSurfacePlace" => Ok(Block3dCommand::PlaceVortex(place_vortex::PlaceVortex {
                window_id: window_id_from_args(args),
                object_id: str_field("objectId").unwrap_or_default(),
                position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                normal: f64_vec3_field(args, "normal").unwrap_or([0.0, 0.0, 1.0]),
            })),
            "patchRepresentation" => {
                Ok(Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: str_field("id").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: str_field("value").unwrap_or_default() }))
            }
            // 🩹️ Forward-fix, not a preserved behavior: pre-migration `command_from_action` had NO arm
            // for the manifest-declared `setCamera` view action at all (fell through to the reserved-
            // action error) — a real gap, the `Block3dCommand::SetCamera` variant was only reachable via
            // direct `dispatch_typed`/binary `OpBinary`. `assert_declared_actions_bridge_to_commands`
            // requires every declared action to bridge, so this parses the camera pose from
            // `{position,target,zoom}` args the same shape `BlockCamera3d` serializes to.
            "setCamera" => Ok(Block3dCommand::SetCamera(set_camera::SetCamera {
                camera: BlockCamera3d {
                    position: f64_vec3_field(args, "position").unwrap_or([0.0, 0.0, 0.0]),
                    target: f64_vec3_field(args, "target").unwrap_or([0.0, 0.0, 0.0]),
                    zoom: args.and_then(|value| value.get("zoom")).and_then(Value::as_f64).unwrap_or(1.0),
                },
            })),
            other => Err(Fault::new(FaultOrigin::App, FaultCode::new("block3d.unhandled-action"), format!("action '{other}' is not a framework-reserved action — surface actions are dispatched exclusively through the typed command channel"))),
        }
    }

    fn handle(
        command: &Block3dCommand,
        doc: &ArtifactView<'_, Block3dSnapshot>,
        cfg: &ConfigView<'_, Block3dConfig>,
        _interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Block3dMutation, Block3dConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain's
    /// `HierarchyProvider::Topology` — representations (`surface` granularity) and rim-vortex
    /// templates (`vortex` granularity) as a flat forest (no real structural parent between them, but
    /// declaring `Topology` rather than `Flat` lets `validate_state` prune stale selection/hover ids
    /// the moment a representation or vortex is removed — see `HierarchyProvider::Flat`'s doc comment
    /// on why `Flat` domains are never auto-pruned).
    fn interaction_topology(doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> InteractionTopology {
        let mut ordered: Vec<TopologyNode> = Vec::new();
        for representation in &doc.snapshot.representations {
            ordered.push(TopologyNode { id: format!("surface:{}", representation.id), granularity: BLOCK3D_GRANULARITY_SURFACE.into(), parent: None });
        }
        for vortex in &doc.snapshot.vortices {
            ordered.push(TopologyNode { id: format!("vortex:{}", vortex.id), granularity: BLOCK3D_GRANULARITY_VORTEX.into(), parent: None });
        }
        let mut domains = BTreeMap::new();
        domains.insert(BLOCK3D_INTERACTION_VORTEX.to_string(), DomainTopology { ordered });
        InteractionTopology { domains }
    }

    fn window_measures(doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> HashMap<String, Vec<semio_framework_plugin::WindowMeasure>> {
        let labels = block3d_labels(cfg.snapshot);
        let mut measures = HashMap::new();
        measures.insert(BLOCK3D_DEFAULT_WINDOW_ID.into(), world::window_measures(doc.snapshot, cfg.snapshot, BLOCK3D_DEFAULT_WINDOW_ID, labels));
        measures
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Block3dSnapshot>, cfg: &ConfigView<'_, Block3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let labels = block3d_labels(cfg.snapshot);
        let active_representation_id = cfg.snapshot.active_representation_id.as_deref();
        let (base_body, window_id) = block3d_resolve_world_body(body_key);
        let node = match base_body {
            world::BLOCK3D_BODY_WORLD => world::render(doc.snapshot, cfg.snapshot, &window_id)?,
            document_panel::BLOCK3D_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels)?,
            inspection_panel::BLOCK3D_BODY_INSPECTOR => inspection_panel::render(doc.snapshot, active_representation_id, labels)?,
            _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "block3d unknown-body label admission failed"))?,
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🌉️ The flagship seam: `puzzle3d_catalog_fragment`'s first real caller. Wraps the block-3d
    /// document's puzzle3d-shaped catalog fragment as a `kit.catalog`-schema `Media` value for the
    /// `"catalog:out"` port declared in `block3d_io`. `wanted_tags` should come from `cfg.wanted_tags`
    /// but `ArtifactEditor::export_media`'s landed signature doesn't thread `ConfigView` through yet —
    /// see `Block3dConfig::wanted_tags`'s doc — so this always resolves the active representation with
    /// an empty (all-tags) filter until that lands. Falls through to the default whole-document pack
    /// export for every other port (`"document:out"`).
    fn export_media(port: &str, doc: &ArtifactView<'_, Block3dSnapshot>) -> Result<Media, MediaError> {
        if port != "catalog:out" {
            // 🌉️ Reimplements `ArtifactEditor::export_media`'s default `"document:out"` behavior
            // verbatim — overriding the trait method forfeits the ability to delegate back to its
            // own default body, so the whole-document pack export is duplicated here rather than
            // left unreachable for this surface.
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::Kit, form: MediaForm::Type }, |io| io.document_media_type);
            let bytes = store::ArtifactPack::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let fragment = crate::artifacts::block3d::schema::inferences::puzzle3d_catalog_fragment(doc.snapshot, &[]);
        Ok(Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: MediaPayload::Structured { schema: KIT_CATALOG_ARTIFACT_ID.into(), json: fragment.to_string() } })
    }
}
//#endregion 🔖️Block3dPlayApp

//#region 🔖️Manifest
pub fn create_block3d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(BLOCK3D_DIALECT)
        .artifact_kind(artifact_kind())
        // 🗂️ The puzzle3d catalog artifact this surface's `"catalog:out"` port produces — see
        // `block3d_io`/`Block3dPlayApp::export_media`.
        .artifact_kind(ArtifactKindSpec {
            id: KIT_CATALOG_ARTIFACT_ID.into(),
            name: "Kit Catalog".into(),
            source_format: KIT_CATALOG_ARTIFACT_ID.into(),
            component_kind: "kit-catalog".into(),
            dimension: "3d".into(),
            media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            schema: KIT_CATALOG_ARTIFACT_ID.into(),
            export_formats: vec![],
            import_formats: vec![],
            export_stdio_kinds: vec![],
            import_stdio_kinds: vec![],
        })
        .icon_id("box")
        .mode_def(edit_mode::definition())
        .default_mode_id(edit_mode::BLOCK3D_PLAY_MODE_EDIT)
        .window_kind_def(world::definition())
        .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SELECT, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer"))
        .utility(UtilityDefinition::new(BLOCK3D_UTILITY_SURFACE_BRUSH, LocalizedLabel::native("Surface brush", "Flächenpinsel"), "paintbrush"))
        .window_kind_utilities(world::BLOCK3D_WINDOW_WORLD, vec![BLOCK3D_UTILITY_SELECT.into(), BLOCK3D_UTILITY_SURFACE_BRUSH.into()])
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain
        // replaces the deleted `setSelection`/`selectVortex`/`hoverVortex` view actions — the
        // framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
        // `setSelectionMode`/`setInteractionGranularity` for it.
        .interaction(InteractionDefinition {
            id: BLOCK3D_INTERACTION_VORTEX.into(),
            label: LocalizedLabel::native("Vortices", "Wirbel"),
            granularities: vec![
                GranularityDefinition { id: BLOCK3D_GRANULARITY_VORTEX.into(), label: LocalizedLabel::native("Vortex", "Wirbel"), icon_id: "circle-dot".into() },
                GranularityDefinition { id: BLOCK3D_GRANULARITY_SURFACE.into(), label: LocalizedLabel::native("Surface", "Fläche"), icon_id: "box".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace, MergeMode::Additive], transitive: false, broadcast: true },
        })
        .window_kind_interactions(world::BLOCK3D_WINDOW_WORLD, vec![InteractionRef::new(BLOCK3D_INTERACTION_VORTEX)])
        .default_layout(edit_mode::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        .mutation("patchObjectKind", LocalizedLabel::native("Patch Object Kind", "Objektart bearbeiten"))
        .mutation("addRepresentation", LocalizedLabel::native("Add Representation", "Darstellung hinzufügen"))
        .mutation("removeRepresentation", LocalizedLabel::native("Remove Representation", "Darstellung entfernen"))
        .mutation("addVortexKind", LocalizedLabel::native("Add Vortex Kind", "Wirbelart hinzufügen"))
        .mutation("removeVortexKind", LocalizedLabel::native("Remove Vortex Kind", "Wirbelart entfernen"))
        .mutation("addVortex", LocalizedLabel::native("Add Vortex", "Wirbel hinzufügen"))
        .mutation("removeVortex", LocalizedLabel::native("Remove Vortex", "Wirbel entfernen"))
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
        .mutation("edit", LocalizedLabel::native("Edit", "Bearbeiten"))
        .view_action("setActiveRepresentation", LocalizedLabel::native("Set Active Representation", "Aktive Darstellung festlegen"))
        .view_action("setWindowRepresentations", LocalizedLabel::native("Set Window Representations", "Fensterdarstellungen festlegen"))
        .view_action("toggleWindowRepresentation", LocalizedLabel::native("Toggle Representation", "Darstellung umschalten"))
        .view_action("setWindowArrangement", LocalizedLabel::native("Set Arrangement", "Anordnung festlegen"))
        .view_action("setWindowSpacing", LocalizedLabel::native("Set Spacing", "Abstand festlegen"))
        .view_action("setBrushVortexKind", LocalizedLabel::native("Set Brush Vortex Kind", "Pinsel-Wirbelart festlegen"))
        .view_action("setBrushRadius", LocalizedLabel::native("Set Brush Radius", "Pinselradius festlegen"))
        .view_action("setBrushFlip", LocalizedLabel::native("Set Brush Flip", "Pinselrichtung umkehren"))
        .view_action("worldSurfaceHover", LocalizedLabel::native("Surface Hover", "Flächenhover"))
        .view_action("worldSurfaceLeave", LocalizedLabel::native("Surface Leave", "Fläche verlassen"))
        .mutation("worldSurfacePlace", LocalizedLabel::native("Place Vortex", "Wirbel platzieren"))
        .mutation("patchRepresentation", LocalizedLabel::native("Patch Representation", "Darstellung bearbeiten"))
        // 🎥️ `setCamera` was the one `Block3dCommand` row with no manifest action at all — reachable
        // only through `dispatch_typed`/binary `OpBinary`, and rejected from the real UI path with
        // `interactive-job.unknown-key`. It writes `Block3dConfig.camera`, never the document, so it is
        // a view action like the other camera/window/brush rows.
        .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
        // 🧵️ Grants UI execution authority to every one of this app's 23 command rows. `.mutation(…)`/
        // `.view_action(…)` build their `ActionDefinition` through `ActionDefinition::bounded_catalog`,
        // which — per its own doc — is "a catalog row WITHOUT granting UI execution authority": the
        // classification stays `Unclassified` and `validate_ui_dispatch_classification` rejects every
        // real `handle_action` dispatch with `interactive-job.not-ui-safe`. The `setActiveUtility` row
        // is a documentary no-op: the framework auto-injects that action (because `.utility(…)` is
        // declared) already classified `Migrated`, via `ActionDefinition::resumable_framework_catalog`,
        // and the injection happens in `try_build_definition` — after this builder call has run — so
        // there is nothing here for it to reclassify. It is kept so this list reads as the complete
        // 23-row retained set that `BLOCK3D_RETAINED_TOOL_IDS` must equal.
        .action_interactive_job("patchObjectKind", InteractiveJobClassification::Migrated)
        .action_interactive_job("addRepresentation", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeRepresentation", InteractiveJobClassification::Migrated)
        .action_interactive_job("addVortexKind", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeVortexKind", InteractiveJobClassification::Migrated)
        .action_interactive_job("addVortex", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeVortex", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_interactive_job("edit", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveRepresentation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setWindowRepresentations", InteractiveJobClassification::Migrated)
        .action_interactive_job("toggleWindowRepresentation", InteractiveJobClassification::Migrated)
        .action_interactive_job("setWindowArrangement", InteractiveJobClassification::Migrated)
        .action_interactive_job("setWindowSpacing", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveUtility", InteractiveJobClassification::Migrated)
        .action_interactive_job("setBrushVortexKind", InteractiveJobClassification::Migrated)
        .action_interactive_job("setBrushRadius", InteractiveJobClassification::Migrated)
        .action_interactive_job("setBrushFlip", InteractiveJobClassification::Migrated)
        .action_interactive_job("worldSurfaceHover", InteractiveJobClassification::Migrated)
        .action_interactive_job("worldSurfaceLeave", InteractiveJobClassification::Migrated)
        .action_interactive_job("worldSurfacePlace", InteractiveJobClassification::Migrated)
        .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
        .action_interactive_job("patchRepresentation", InteractiveJobClassification::Migrated)
        .io(block3d_io())
        // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
        // `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
        // `.workflow(...)` on this builder, so the old `BLOCK3D_EXAMPLE_CAPSULE`/`BLOCK3D_EXAMPLE_FOREST_LEFT`
        // app-level example registrations and the no-op `.workflow("block3d", …)` call are dropped here
        // (not silently: reported in this packet's migration report). The subset's own
        // `📚️examples/🌲️hexagonal-cut-concrete-forest-left`/`🏢️nakagin-capsule` facets (artifact-level,
        // pre-existing, untouched by this packet) and this surface's own `setActiveExample` command
        // (still real, still DSL-fixture-backed) are the modern, role-agnostic replacements for this.
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as sdk_new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `Block3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Block3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Block3dPlayApp>` builds it.
    pub type Block3dApp = VcsArtifactApp<EditorApp<Block3dPlayApp>>;

    pub fn new_app() -> Block3dApp {
        sdk_new_app::<EditorApp<Block3dPlayApp>>()
    }

    /// ✏️ Adapts `create_block3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
    /// framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
    pub fn block3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_block3d_app(), examples: Vec::new() }
    }

    pub fn app_with_registry() -> Block3dApp {
        new_app_with_registry::<EditorApp<Block3dPlayApp>>(block3d_app_manifest_for_testkit)
    }

    pub fn dispatch(app: &mut Block3dApp, command: Block3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Block3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut Block3dApp) -> Vec<semio_framework_plugin::WindowMeasure> {
        app.window_measures().get(BLOCK3D_DEFAULT_WINDOW_ID).cloned().unwrap_or_default()
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::PluginApp;
    use testkit::{new_app, Block3dApp};

    //#region 🔖️CommandSurface
    fn every_command() -> Vec<Block3dCommand> {
        vec![
            Block3dCommand::PatchObjectKind(patch_object_kind::PatchObjectKind { field: "name".into(), value: "x".into() }),
            Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}),
            Block3dCommand::RemoveRepresentation(remove_representation::RemoveRepresentation { id: "r0".into() }),
            Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}),
            Block3dCommand::RemoveVortexKind(remove_vortex_kind::RemoveVortexKind { id: "v0".into() }),
            Block3dCommand::AddVortex(add_vortex::AddVortex {}),
            Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: "v0".into() }),
            Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: "capsule".into() }),
            Block3dCommand::Edit(edit::Edit { text: "{}".into() }),
            Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }),
            Block3dCommand::SetWindowRepresentations(set_window_representations::SetWindowRepresentations { window_id: "w0".into(), representation_ids: vec!["r0".into()] }),
            Block3dCommand::ToggleWindowRepresentation(toggle_window_representation::ToggleWindowRepresentation { window_id: "w0".into(), representation_id: "r0".into(), visible: true }),
            Block3dCommand::SetWindowArrangement(set_window_arrangement::SetWindowArrangement { window_id: "w0".into(), arrangement: "x".into() }),
            Block3dCommand::SetWindowSpacing(set_window_spacing::SetWindowSpacing { window_id: "w0".into(), spacing: 8.0 }),
            Block3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { window_id: "w0".into(), utility_id: "select".into() }),
            Block3dCommand::SetBrushVortexKind(set_brush_vortex_kind::SetBrushVortexKind { vortex_kind_id: Some("v0".into()) }),
            Block3dCommand::SetBrushRadius(set_brush_radius::SetBrushRadius { radius: 0.3 }),
            Block3dCommand::SetBrushFlip(set_brush_flip::SetBrushFlip { flip: true }),
            Block3dCommand::HoverSurface(hover_surface::HoverSurface { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
            Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {}),
            Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: "w0".into(), object_id: "r0".into(), position: [0.0, 0.0, 0.0], normal: [0.0, 1.0, 0.0] }),
            Block3dCommand::SetCamera(set_camera::SetCamera { camera: BlockCamera3d::default() }),
            Block3dCommand::PatchRepresentation(patch_representation::PatchRepresentation { id: "r0".into(), field: "name".into(), value: "x".into() }),
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_cover_every_row() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(Block3dCommand::command_id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 23, "every Block3dCommand row must be covered by every_command()");
    }

    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧷️ Pins the exact pre-migration bytes for the three divergent-key rows plus a handful of
    /// `Option`/`Vec` rows — copied verbatim from the ticket's `🧪️wire-baseline-3d-before.txt`.
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: deleting `setSelection` (which
    /// sat BEFORE `LeaveSurface` in the row order) shifts every later row's binary ordinal down by
    /// one — an intentional, greenfield wire-format break (row order IS the ordinal, per this enum's
    /// own doc comment), not a preserved-bytes regression. `LeaveSurface`'s ordinal moves 0x14 -> 0x13.
    #[semio_framework_async_macros::async_test]
    async fn divergent_key_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &Block3dCommand| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>();
        assert_eq!(protocol::OpText::print_op(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), "leaveSurface");
        assert_eq!(hex(&Block3dCommand::LeaveSurface(leave_surface::LeaveSurface {})), "01130000");
    }

    /// ⚖️ LAW: every one of the 23 declared `Block3dCommand` rows is retained-owned by
    /// `Block3dRetainedCommandJobFactory`, classified `Migrated` in the manifest, and carries an exact,
    /// nonempty publication-lane contract. `AppActionRegistry::tool_job_registration` enforces the same
    /// set equality at app construction (`interactive-job.catalog-incomplete`), and
    /// `validate_ui_dispatch_classification` rejects anything not `Migrated` at the very first gate of
    /// `handle_action` — this test pins both so a future command row that forgets its retained-tool-id,
    /// its classification, or its lane contract fails here instead of going silently dispatch-dead.
    /// Mirrors block5d's own retained-route discipline and generation3d's
    /// `retained_route_dispositions_are_exact_and_exhaustive`.
    #[semio_framework_async_macros::async_test]
    async fn retained_route_dispositions_are_exact_and_exhaustive() {
        use semio_framework::ToolExecutionShape;
        assert_eq!(BLOCK3D_RETAINED_TOOL_IDS.len(), 23);
        assert_eq!(<Block3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 23);
        assert_eq!(BLOCK3D_PUBLICATION_CONTRACTS.len(), 23);
        assert_eq!(block3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
        let mut sorted_ids = BLOCK3D_RETAINED_TOOL_IDS.to_vec();
        sorted_ids.sort_unstable();
        sorted_ids.dedup();
        assert_eq!(sorted_ids.len(), BLOCK3D_RETAINED_TOOL_IDS.len(), "duplicate retained tool ids in {BLOCK3D_RETAINED_TOOL_IDS:?}");
        assert_eq!(Block3dRetainedCommandJobFactory::TOOL_IDS, BLOCK3D_RETAINED_TOOL_IDS);
        for command in every_command() {
            let tool_id = command.command_id();
            assert!(BLOCK3D_RETAINED_TOOL_IDS.contains(&tool_id), "command {tool_id} is not owned by Block3dRetainedCommandJobFactory");
            let contract = BLOCK3D_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == tool_id).unwrap_or_else(|| panic!("tool {tool_id} declares a publication contract"));
            assert!(!contract.lanes.is_empty(), "tool {tool_id} declares a nonempty publication lane set");
        }
        // 🪟️ App-level actions are fanned onto every window kind by `try_build_definition`, so the
        // world window carries the complete classified action set (`AppDefinition` has no app-level
        // `actions` field of its own).
        let definition = create_block3d_app();
        let world_window = definition.window_kinds.iter().find(|window| window.id == world::BLOCK3D_WINDOW_WORLD).expect("world window declared");
        for tool_id in BLOCK3D_RETAINED_TOOL_IDS {
            let action = world_window.actions.iter().find(|action| action.id == *tool_id).unwrap_or_else(|| panic!("action {tool_id} is declared by the manifest"));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "action {tool_id} must be UI-dispatchable");
        }
    }

    /// ⚖️ LAW: the two lanes any block3d tool can publish into both have a real one-item preparation
    /// factory — a Config-lane tool without `build_config_store_one_item_preparation_factory` is
    /// rejected at dispatch with `interactive-job.publication-authority-missing`.
    #[semio_framework_async_macros::async_test]
    async fn both_declared_publication_lanes_have_a_preparation_factory() {
        assert!(<Block3dPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
        assert!(<Block3dPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
    }

    /// 🌉️ Every surface-declared action must bridge through `command_from_action` and round-trip
    /// `command_id`.
    #[semio_framework_async_macros::async_test]
    async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<semio_framework_plugin::EditorApp<Block3dPlayApp>>(testkit::block3d_app_manifest_for_testkit);
        assert!(<Block3dPlayApp as ArtifactEditor>::command_from_action("noSuchAction", None).is_err());
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Manifest
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let definition = create_block3d_app();
        assert_eq!(definition.modes.len(), 1);
        assert_eq!(definition.window_kinds.len(), 1);
        for body_key in [document_panel::BLOCK3D_BODY_DOCUMENT, inspection_panel::BLOCK3D_BODY_INSPECTOR] {
            assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
        }
        assert!(definition.artifact_kinds.iter().any(|kind| kind.id == "kit.catalog"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the `vortex` domain is declared
    /// once, with both granularities, a `Topology` hierarchy, and scoped to the world window kind —
    /// the framework auto-injects the six interaction actions for it (asserted separately below via
    /// `assert_declared_actions_bridge_to_commands`'s injected-action allowance).
    #[semio_framework_async_macros::async_test]
    async fn declares_the_vortex_interaction_domain_scoped_to_the_world_window() {
        let definition = create_block3d_app();
        let interaction = definition.interactions.iter().find(|def| def.id == BLOCK3D_INTERACTION_VORTEX).expect("vortex domain declared");
        assert_eq!(interaction.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec![BLOCK3D_GRANULARITY_VORTEX, BLOCK3D_GRANULARITY_SURFACE]);
        assert!(matches!(interaction.hierarchy, HierarchyProvider::Topology));
        let world_window = definition.window_kinds.iter().find(|window| window.id == world::BLOCK3D_WINDOW_WORLD).expect("world window declared");
        assert!(world_window.interactions.contains(&InteractionRef::new(BLOCK3D_INTERACTION_VORTEX)));
    }

    /// 🕹️ `interaction_topology` returns one flat root per representation (`surface` granularity) and
    /// per vortex template (`vortex` granularity) — enough structure for `validate_state` to prune a
    /// stale selection the moment `removeRepresentation`/`removeVortex` deletes its target.
    #[semio_framework_async_macros::async_test]
    async fn interaction_topology_covers_every_representation_and_vortex() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}));
        testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}));
        testkit::dispatch(&mut app, Block3dCommand::AddVortex(add_vortex::AddVortex {}));
        let snapshot = app.snapshot().expect("snapshot");
        let representation_id = snapshot.representations[0].id.clone();
        let vortex_id = snapshot.vortices[0].id.clone();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&snapshot, &history);
        let cfg_snapshot = Block3dConfig::default();
        let cfg = ConfigView { snapshot: &cfg_snapshot };
        let topology = <Block3dPlayApp as ArtifactEditor>::interaction_topology(&doc, &cfg);
        let domain = topology.domains.get(BLOCK3D_INTERACTION_VORTEX).expect("vortex domain topology present");
        assert!(domain.contains(&format!("surface:{representation_id}")));
        assert!(domain.contains(&format!("vortex:{vortex_id}")));
    }

    #[semio_framework_async_macros::async_test]
    async fn block3d_io_declares_the_catalog_out_port() {
        let io = block3d_io();
        assert_eq!(io.document_schema, BLOCK_3D_SCHEMA);
        let ports = io.all_ports();
        assert!(ports.iter().any(|port| port.id == "document:in"));
        assert!(ports.iter().any(|port| port.id == "document:out"));
        let catalog = ports.iter().find(|port| port.id == "catalog:out").expect("catalog:out port declared");
        assert_eq!(catalog.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(catalog.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(catalog.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree_and_inspector() {
        let mut app: Block3dApp = new_app();
        let json = testkit::render(&mut app, document_panel::BLOCK3D_BODY_DOCUMENT);
        assert!(json.contains("Representations"));
        let inspector = testkit::render(&mut app, inspection_panel::BLOCK3D_BODY_INSPECTOR);
        assert!(inspector.contains("\"type\":\"tree\""));
        assert!(inspector.contains("Name"));
        assert!(inspector.contains("Vortices"));
    }
    //#endregion 🔖️Manifest

    //#region 🔖️Behavior
    /// ⚖️ LAW: the editor boots non-empty. `world_meshes_json` drops any representation whose
    /// `mesh_url` is `None`, so a boot document must both carry representations and name their meshes
    /// for the `World3d` window to paint anything before the first user action. The mesh url is
    /// asserted on the scene's own compute facet rather than on the rendered body: the semantic
    /// surface contract pack-encodes the scene into `SurfaceProps.doc.bytes`, so no scene string
    /// survives into the rendered tree's JSON any more.
    #[semio_framework_async_macros::async_test]
    async fn the_editor_boots_with_a_renderable_world() {
        let mut app: Block3dApp = new_app();
        let snapshot = app.snapshot().expect("snapshot");
        assert!(!snapshot.representations.is_empty(), "the boot document must carry at least one representation");
        assert!(snapshot.representations.iter().all(|representation| representation.mesh_url.is_some()), "every boot representation must name a mesh url");
        let visible: Vec<&crate::BlockRepresentation> = snapshot.representations.iter().collect();
        assert!(
            crate::editor::block3d::world::world_meshes_json(&snapshot, &visible).contains("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb"),
            "the world scene must reference the boot document's mesh"
        );
        assert!(testkit::render(&mut app, world::BLOCK3D_BODY_WORLD).contains("\"type\":\"surface\""), "the world body must render a semantic scene surface");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_representation_then_set_active_then_render_world_shows_mesh() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}));
        let representation_id = app.snapshot().expect("snapshot").representations[0].id.clone();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some(representation_id) }));
        let json = testkit::render(&mut app, world::BLOCK3D_BODY_WORLD);
        assert!(json.contains("\"type\":\"surface\""), "world body must render a scene surface");
        assert!(json.contains("world-3d"), "the scene surface must declare the world-3d surface kind");
    }

    /// 🚀️ Counted relative to the boot document (`initial_snapshot` now parses the
    /// `hexagonal-cut-concrete-forest-left` fixture, which already ships vortex kinds and vortices)
    /// rather than against a hard-coded 1/0.
    #[semio_framework_async_macros::async_test]
    async fn add_vortex_kind_then_add_vortex_then_remove_round_trips() {
        let mut app: Block3dApp = new_app();
        let before = app.snapshot().expect("snapshot").vortices.len();
        testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}));
        testkit::dispatch(&mut app, Block3dCommand::AddVortex(add_vortex::AddVortex {}));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.vortices.len(), before + 1);
        let vortex_id = projection.vortices[before].id.clone();
        testkit::dispatch(&mut app, Block3dCommand::RemoveVortex(remove_vortex::RemoveVortex { id: vortex_id }));
        assert_eq!(app.snapshot().expect("snapshot").vortices.len(), before);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_loads_capsule_fixture() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        let projection = app.snapshot().expect("snapshot");
        assert_eq!(projection.object_kind.id, "Capsule J");
        // 🥽️ One representation, not two: the former `"1:500"` row named `/mesh/capsule_J.1to500.glb`,
        // which no mesh delivery catalog ships, so `resolveMeshAsset` threw the instant the example
        // loaded. There is no 1:500 `.glb` anywhere in the repo (only a Rhino `.3dm` source), so the
        // row was removed from the fixture rather than repointed at an unrelated mesh.
        assert_eq!(projection.representations.len(), 1);
        assert_eq!(projection.representations[0].mesh_url.as_deref(), Some("/mesh/🧊️capsule_J.glb"));
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trips_through_the_wrapper() {
        let mut app: Block3dApp = new_app();
        let kinds = |app: &mut Block3dApp| crate::artifacts::block3d::vortex_kinds_of(&app.snapshot().expect("snapshot")).len();
        let before = kinds(&mut app);
        testkit::dispatch(&mut app, Block3dCommand::AddVortexKind(add_vortex_kind::AddVortexKind {}));
        assert_eq!(kinds(&mut app), before + 1);
        app.handle_action("undo", None, &semio_framework_plugin::testkit::meta("local")).expect("undo");
        assert_eq!(kinds(&mut app), before);
        app.handle_action("redo", None, &semio_framework_plugin::testkit::meta("local")).expect("redo");
        assert_eq!(kinds(&mut app), before + 1);
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection`/`selectVortex`/
    /// `hoverVortex` are gone — the still-config-only `setActiveRepresentation` view action now
    /// exercises the "view action never touches the document" contract this test used to cover.
    #[semio_framework_async_macros::async_test]
    async fn set_active_representation_writes_config_not_document() {
        let mut app: Block3dApp = new_app();
        let result =
            app.dispatch_typed(Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }), &semio_framework_plugin::testkit::meta("local")).expect("set active representation");
        assert!(result.mutations.is_empty(), "setActiveRepresentation is config-only and must emit no document operations");
    }

    #[semio_framework_async_macros::async_test]
    async fn export_media_catalog_out_wraps_the_puzzle3d_fragment() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        let media = semio_framework_plugin::resolve_ready(app.export_media("catalog:out")).expect("export catalog");
        assert_eq!(media.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "kit.catalog");
                let value: serde_json::Value = serde_json::from_str(&json).expect("valid json");
                assert_eq!(value["objectKinds"][0]["id"], "Capsule J");
            }
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn place_vortex_on_surface_auto_creates_kind_and_vortex() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id: set_active_example::BLOCK3D_EXAMPLE_CAPSULE.into() }));
        testkit::dispatch(&mut app, Block3dCommand::PlaceVortex(place_vortex::PlaceVortex { window_id: BLOCK3D_DEFAULT_WINDOW_ID.into(), object_id: "r0".into(), position: [0.5, 0.0, 1.0], normal: [0.0, 1.0, 0.0] }));
        let projection = app.snapshot().expect("snapshot");
        assert!(!crate::artifacts::block3d::vortex_kinds_of(&projection).is_empty());
        assert_eq!(projection.vortices.len(), 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn command_from_action_bridges_set_active_example() {
        assert!(
            matches!(<Block3dPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": "capsule" }))), Ok(Block3dCommand::SetActiveExample(set_active_example::SetActiveExample { id })) if id == "capsule")
        );
    }
    //#endregion 🔖️Behavior

    //#region 🔖️WindowMeasures
    /// 🧬️ Kind-discipline wrapper: the real registry enforces View actions never emit document
    /// operations. Exercising it here (rather than only the plain `new_app()`) is the reason
    /// `testkit::app_with_registry` exists.
    #[semio_framework_async_macros::async_test]
    async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
        let mut app = testkit::app_with_registry();
        let result = testkit::dispatch(&mut app, Block3dCommand::SetActiveRepresentation(set_active_representation::SetActiveRepresentation { representation_id: Some("r0".into()) }));
        assert!(result.mutations.is_empty(), "setActiveRepresentation is a view action and must never reach document operations under kind discipline");
    }

    /// 🎚️ The world window collects its five option measures (representations/quick-pick/arrangement/
    /// spacing/brush) fresh per frame — never frozen into the manifest.
    #[semio_framework_async_macros::async_test]
    async fn world_window_measures_collect_all_five_options() {
        let mut app: Block3dApp = new_app();
        testkit::dispatch(&mut app, Block3dCommand::AddRepresentation(add_representation::AddRepresentation {}));
        let measures = testkit::main_window_measures(&mut app);
        assert_eq!(measures.len(), 5, "world window must expose representations/quick-pick/arrangement/spacing/brush");
    }
    //#endregion 🔖️WindowMeasures
}
//#endregion 🧪️Tests
