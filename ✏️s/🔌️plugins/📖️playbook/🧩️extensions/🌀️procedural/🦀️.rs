//! 🧩️ Playbook procedural block-kind module — flow-backed building component params + live 3D preview.

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use semio_framework_plugin::UiAssemblyResult;

/// 🧯️ Surfaces a retained-ownership refusal as a plugin fault carrying its message.
fn module_value_fault(error: semio_framework_value::ValueError) -> Fault { Fault::new(semio_framework_plugin::FaultOrigin::Plugin, semio_framework_plugin::FaultCode::new("playbook.module.procedural.retained"), error.into_message()) }
use semio_framework_value::{retained_clone::{RetainedCloneGrant, RetainedCloneProgress}, RetirementDemand, ValueError};
use semio_framework_ui_contract::{ActionId as UiActionId, Buildable, HasBase, HasChildren};

use flow::{forms_bridge::flow_host_snapshot_to_form_spec, FlowHost};
use neural_engine::{Registry, RegistryRetirement, SharedRegistry, ValueRetirement};
use semio_s_spatial_kernel_semio_session::{Session, SessionCapture};
use protocol::MutationDiff;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_artifact_playbook_playbook::{visible_blocks, PlaybookBlock};
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_action_bus::RetainedToolWireInput;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec};
use semio_framework_job::Operation;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionHoverState};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactInstanceOperationOwner, ArtifactInstanceOperationOwnerHandle, HistoryView, PluginLifecycleStep};
use semio_framework_ui_locale::app_labels;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::mesh_from_kind;
use semio_framework_plugin::world3d_default_camera;
use semio_framework_plugin::world3d_scene;
use semio_framework_plugin::world3d_selection_json;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::App;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactApp;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::ExecutionMode;
use semio_framework_plugin::ExtensionBundle;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::Plugin;
use semio_framework_plugin::PluginBuilder;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::WorldSunConfig;
// 🌱️ `Value`/`Map` alias `pack::json`'s first-party JSON tree (the `serde_json::Value`
// replacement, `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs`), keeping this file's shape
// unchanged everywhere else. `playbook::visible_blocks` (`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️.rs`)
// takes `&PlaybookValues` (`HashMap<String, DslValue>`) — already first-party, converted through
// `json_to_dsl_value` at its one call site below, no `serde_json` crossing left. `FlowHostSnapshot`
// itself now derives `ToValue`/`FromValue` alongside `Serialize`/`Deserialize`, so its own parse
// goes through `pack::json::from_json_str` below instead. Every other JSON value in this file is
// arbitrary-shaped and goes through `pack::json` instead.
#[cfg(test)]
use semio_framework_pack_json::to_json_string;
use semio_framework_pack_json::{from_dsl_value as json_from_dsl_value, to_dsl_value as json_to_dsl_value, to_string as json_to_string, parse as parse_json, Object as Map, Value};
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants
const MODULE_PLUGIN_ID: &str = "playbook-module-procedural";
/// 🪪️ Canonical surface id (`<artifact_kind>@<standard>/<subset>#<role>`, ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET §1) — `editor` because `.mutation(...)` grants
/// `ExportSolid`/`ImportSolid`; mirrors `s.playbook.playbook@1/*#editor` in the sibling artifact.
const MODULE_APP_ID: &str = "s.playbook.procedural@1/*#editor";
const MODULE_DOCUMENT_SCHEMA: &str = "playbook.module.procedural.payload";
const BODY_PARAMS: &str = "params";
const BODY_PREVIEW: &str = "preview";
const MODULE_WINDOW_PARAMS: &str = "playbook-module-procedural-params";
const MODULE_WINDOW_PREVIEW: &str = "playbook-module-procedural-preview";
const PREVIEW_SURFACE: &str = "playbook.module.procedural.preview";
const PREVIEW_FALLBACK_MESH_KIND: &str = "box";
const ACTION_EXPORT_SOLID: &str = "exportSolidGeometry";
const ACTION_IMPORT_SOLID: &str = "importSolidGeometry";
const SOLID_MEDIA_FORMATS: [&str; 4] = ["step", "obj", "stl", "glb"];
const SOLID_EXPORT_DEFLECTION: f64 = 0.1;
const SOLID_IMPORT_TOLERANCE: f64 = 0.1;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the procedural playbook module surface.
    pub enum ProceduralModuleApps: PluginApp {
        Module(VcsArtifactApp<ModuleApp>),
    }
}
//#endregion 🗃️Apps

#[path = "🎒️payload-codec-and-diff/🦀️.rs"]
mod payload_codec_and_diff;
use payload_codec_and_diff::{default_payload, params_as_json};
pub use payload_codec_and_diff::{ModulePayloadDiff, ModuleRenderPayload};

#[path = "🧱️learning-program-and-geometry/🦀️.rs"]
mod learning_program_and_geometry;
#[cfg(test)]
use learning_program_and_geometry::HEX_COLUMN_LEARNING_PROGRAM_JSON;
use learning_program_and_geometry::{
    apply_flow_params, bind_control, geometry_handle_for_widget, is_brep_geometry_handle, json_f64_value, json_string_value, learning_program_json, text_node, ui_admit, ui_label, ui_text, value_map, value_text, widget_id, ModuleGeometryOwner,
    ModuleImportedGeometry,
};

#[path = "👁️preview-and-params-body/🦀️.rs"]
mod preview_and_params_body;
#[cfg(test)]
use preview_and_params_body::{evaluated_preview_payload, render_question_control};
use preview_and_params_body::{embedded_payload, evaluated_preview_geometry_handles, render_params_body, render_preview_body};

#[path = "💾️media-export-and-command/🦀️.rs"]
mod media_export_and_command;
use media_export_and_command::{handle_export_solid, handle_import_solid, imported_geometry_handles};
pub use media_export_and_command::Command;

//#endregion 🔖️Constants

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the procedural module; one field per label makes every locale×terminology combination compile-checked.
    struct ModuleLabels {
        no_flow_inputs: native_en "No flow inputs.", native_de "Keine Flow-Eingaben.", reuse_en "No flow inputs.", reuse_de "Keine Flow-Eingaben.";
        no_procedural_parameters: native_en "No procedural parameters.", native_de "Keine prozeduralen Parameter.", reuse_en "No procedural parameters.", reuse_de "Keine prozeduralen Parameter.";
    }
}

//#endregion 🔖️Terminology

//#region 🔖️Payload

//#region 🔖️DocumentMutation
/// ✏️ Whole-payload replace operation for the procedural block-kind slot document. The module's document is a
/// transient render/params payload (not a collaboratively-edited structure), so its single operation
/// swaps the payload wholesale — export/import stash their results on `params` and re-emit it. The VCS
/// store still records the pre-operation payload as a true inverse, so undo works.
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{ModulePayloadMutation, SetPayload};

//#endregion 🔖️DocumentMutation

//#endregion 🔖️Payload

//#region 🧵️RetainedCommands
/// 🧵️ The two declared verbs, which are also the only two routes the shell may dispatch. A verb
/// classified anything but `Migrated` is refused by `validate_ui_dispatch_classification` before it
/// ever reaches the guest, so declaring the classification without the owned tool factory that earns
/// it would be a lie the registry catches at assembly time (`interactive-job.owner-classification`).
const MODULE_RETAINED_TOOL_IDS: &[&str] = &[ACTION_EXPORT_SOLID, ACTION_IMPORT_SOLID];
const MODULE_RETAINED_PAYLOAD_SCHEMA: &str = "playbook.module.procedural.tool-command.v1";
/// 📦️ `importSolidGeometry` carries a whole interchange body (STEP/OBJ/STL/GLB text or base64) as its
/// `data` argument, so the wire ceiling is sized for a model file rather than for a click; the
/// retained OUTPUT is the re-emitted `SetPayload`, whose payload is the same order of magnitude.
const MODULE_RETAINED_RAW_BYTES: usize = 4 * 1024 * 1024;
const MODULE_RETAINED_OUTPUT_BYTES: usize = 4 * 1024 * 1024;
const MODULE_RETAINED_WORK_ITEMS: usize = 1;
/// 🛣️ Both handlers answer `Emit::mutations(vec![ModulePayloadMutation::SetPayload(..)])` and this
/// app declares `NoConfig`, so the artifact lane is the only lane either route may publish into.
const MODULE_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: ACTION_EXPORT_SOLID, lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: ACTION_IMPORT_SOLID, lanes: &[ArtifactToolPublicationLane::Artifact] },
];

fn module_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(MODULE_RETAINED_RAW_BYTES, 64, MODULE_RETAINED_WORK_ITEMS as u64, MODULE_RETAINED_OUTPUT_BYTES, 7_500)
}

fn module_retained_command_id(command: &Command) -> &'static str {
    match command {
        Command::ExportSolid { .. } => ACTION_EXPORT_SOLID,
        Command::ImportSolid { .. } => ACTION_IMPORT_SOLID,
    }
}

fn module_retained_extent(command: &Command, _snapshot: &ModuleRenderPayload, _interaction: &InteractionState) -> Option<usize> {
    MODULE_RETAINED_TOOL_IDS.contains(&module_retained_command_id(command)).then_some(MODULE_RETAINED_WORK_ITEMS)
}

struct ModuleGeometryCommandWork {
    tool_id: &'static str,
    instance_owner: ArtifactInstanceOperationOwnerHandle,
    consumed: bool,
}

impl ArtifactCommandWork<ModuleApp> for ModuleGeometryCommandWork {
    fn tool_id(&self) -> &'static str { self.tool_id }

    fn extent(&self, command: &Command, snapshot: &ModuleRenderPayload, interaction: &InteractionState, _context: Option<&ArtifactOwnedToolJobContext<ModuleApp>>) -> Option<usize> {
        module_retained_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, ModuleApp>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<ModuleApp>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed { return Err(Fault::from("playbook.module.procedural.geometry-work-repeated")); }
        self.consumed = true;
        self.instance_owner.with_mut::<ModuleGeometryOwner, _>(|owner| {
            let mut payload = input.snapshot.clone();
            match input.command {
                Command::ExportSolid { format } => handle_export_solid(owner, &mut payload, format),
                Command::ImportSolid { format, data } => handle_import_solid(owner, &mut payload, format, data),
            }
            Ok(ArtifactCommandWorkStep::Complete(Emit::mutations(vec![ModulePayloadMutation::SetPayload(SetPayload { payload })])))
        })

        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub struct ModuleRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ModuleRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: MODULE_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for ModuleRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ModuleApp>;
    type Job = ArtifactRetainedCommandJob<ModuleApp>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        MODULE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        module_retained_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: MODULE_RETAINED_RAW_BYTES, checkpoint_resume: false);

}

impl ArtifactOwnedToolJobFactory for ModuleRetainedCommandJobFactory {
    type Owner = ModuleApp;
    const TOOL_IDS: &'static [&'static str] = MODULE_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = MODULE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = MODULE_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 🔖️App
#[derive(Default)]
pub struct ModuleApp;

impl ArtifactApp for ModuleApp {
    const DIALECT: Dialect = Dialect { artifact_kind: "s.playbook.procedural", standard: StandardId("1"), subset: SubsetId::ANY };
    type Snapshot = ModuleRenderPayload;
    type Mutation = ModulePayloadMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;

    type Command = Command;

    const APP_ID: &'static str = MODULE_APP_ID;
    const DOCUMENT_SCHEMA: &'static str = MODULE_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ModuleApp,
        owner_file: "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs",
        controller: "s.playbook.procedural@1/*#editor",
        artifact_schema: "playbook.module.procedural.payload",
        factory: "ModuleRetainedCommandJobFactory",
        factory_type: ModuleRetainedCommandJobFactory,
        contract: module_retained_contract(),
        tools: ["exportSolidGeometry", "importSolidGeometry"]
    }

    /// 📬️ Without a publication authority on the artifact lane every `Artifact`-lane tool fails
    /// closed with `interactive-job.publication-authority-missing`, so the `Migrated` classification
    /// alone would move the refusal one stage later instead of curing it.
    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(store::cold_mutation_apply_preparation_factory::<Self::Snapshot, Self::Mutation>())
    }

    /// 🗃️ The artifact store needs its own retirement catalog before it may fold a batched item:
    /// `ArtifactStore`'s batch stage copies `mutation_retirement`/`snapshot_retirement` out of the
    /// store and refuses with `batched fold lacks exact snapshot or mutation retirement authority`
    /// (`🧰️framework/…/🏪️store/🦀️.rs:17288`) when either is `None`, which the trait default is.
    /// Measured inside `s` on 2026-09-22 as the refusal that replaced the presence one below, on the
    /// same `importSolidGeometry`. This module's snapshot and mutation are both bounded values, so
    /// the framework's own one-page catalog is exactly right — 🖨️raster, 🌊️flow, 🔱️trinity and
    /// 📕️norm each declare this hook for the same reason.
    /// 🧍 A presence DISPOSER is not a presence retirement OWNER. `PresenceStore::local_read` fails
    /// closed with `presence local read requires a live exact local retirement owner` while
    /// `local_retirement_factory` is `None`, so the first command whose ephemeral leg reads local
    /// presence is refused however correct the command is — measured inside `s` on 2026-09-22 as
    /// `importSolidGeometry refused: dispatch-failed … presence local read requires a live exact
    /// local retirement owner`, the refusal that replaced the `interactive-job` one this module's
    /// tool factory cured. `Presence = NoPresence` here, so the framework's own `NoPresence`-typed
    /// owners fit exactly, as 📖️playbook's own editor already declares them.
    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(ModuleRetainedCommandJobFactory::new(&controller))
    }

    async fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<Self>>, output: &mut Option<semio_framework_plugin::ErasedToolJob>, cx: &mut semio_framework_plugin::__job::StepContext<'_>) -> Result<bool, semio_framework_plugin::__job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "original retained producer request is absent"))?;
        if !MODULE_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if module_retained_command_id(&original.command) != original.tool_id {
            return Err(semio_framework_plugin::__value::ValueError::literal(semio_framework_plugin::__value::ValueRefusalKind::InvariantViolated, "the procedural module command does not match its exact registered tool").into());
        }
        let tool_id = module_retained_command_id(&original.command);
        semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, module_retained_command_id, MODULE_RETAINED_RAW_BYTES, MODULE_RETAINED_WORK_ITEMS, false, |request| ModuleGeometryCommandWork { tool_id, instance_owner: request.instance_operation_owner.clone(), consumed: false })
    }

    async fn initial_snapshot() -> ModuleRenderPayload {
        default_payload()
    }

    /// 🏷️ Maps each `Command` variant back to the action id it was declared under in
    /// `create_module_app` — command-log labeling and the registry's kind-discipline check.
    async fn command_id(command: &Command) -> &'static str {
        match command {
            Command::ExportSolid { .. } => ACTION_EXPORT_SOLID,
            Command::ImportSolid { .. } => ACTION_IMPORT_SOLID,
        }
    }

    /// 🎯️ The bridge the React/wgpu shells still speak (`{action,args}`) — parses the two solid
    /// media actions this module dispatches into `Command`; `format` defaults to `"obj"` (matching
    /// the handlers' pre-B1 defaults) and `data` (import's file-callback payload) defaults to empty.
    async fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let format = args.and_then(|value| value.get("format")).and_then(DslValue::as_str).unwrap_or("obj").to_string();
        match action {
            ACTION_EXPORT_SOLID => Ok(Command::ExportSolid { format }),
            ACTION_IMPORT_SOLID => {
                let data = args.and_then(|value| value.get("data")).and_then(DslValue::as_str).unwrap_or("").to_string();
                Ok(Command::ImportSolid { format, data })
            }
            other => Err(Fault::from(format!("action '{other}' is not supported by {MODULE_APP_ID}"))),
        }
    }

    async fn handle(
        _command: &Command,
        _doc: &ArtifactView<'_, ModuleRenderPayload>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ModulePayloadMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        Err(Fault::from("playbook.module.procedural.geometry-instance-owner-required"))
    }

    fn build_instance_operation_owner() -> Box<dyn ArtifactInstanceOperationOwner> {
        Box::new(ModuleGeometryOwner::new())
    }

    async fn render_with_instance_operation_owner(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, ModuleRenderPayload>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
    ) -> UiAssemblyResult<ComponentTree> {
        if body_key != BODY_PREVIEW { return Self::render(body_key, doc, cfg, view_state).await; }
        let embedded = view_state.extension_input_json.as_deref().map(|input| embedded_payload(input, body_key)).transpose()?;
        let payload = embedded.as_ref().map(|(payload, _)| payload).unwrap_or(doc.snapshot);
        owner.with_mut::<ModuleGeometryOwner, _>(|owner| render_preview_body(owner, payload).map_err(|_| Fault::from("playbook.module.procedural.preview-assembly")))
            .map(built_to_component_tree)
            .map_err(|_| PluginAssemblyError::new("playbook.module.procedural.preview-instance-owner", "procedural preview requires its live geometry instance owner"))
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, ModuleRenderPayload>, _cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
        let labels = resolve_labels::<ModuleLabels>(view_state);
        let embedded = view_state.extension_input_json.as_deref().map(|input| embedded_payload(input, body_key)).transpose()?;
        let payload = embedded.as_ref().map(|(payload, _)| payload).unwrap_or(doc.snapshot);
        let parent_window = embedded.as_ref().and_then(|(_, window)| window.as_ref()).map(|(id, kind)| (id.as_str(), kind.as_str()));
        match body_key {
            BODY_PARAMS => render_params_body(payload, labels, parent_window, embedded.is_some()),
            BODY_PREVIEW => Err(PluginAssemblyError::new("playbook.module.procedural.preview-instance-owner", "procedural preview requires its live geometry instance owner")),
            _ => text_node(format!("Unknown body: {body_key}")),
        }
        .map(built_to_component_tree)
    }
}

/// 🧷️ Fallible — `App::try_from_builder` surfaces a malformed `MODULE_APP_ID` (or any other
/// definition-time mistake) as a `PluginAssemblyError` this crate's `plugin()` entry point can
/// propagate, instead of a guest panic that would trap the wasm instance for good (a trapped
/// `wasm32-wasip2` instance cannot unwind — see `AppBuilder::try_build_definition`'s docs).
async fn create_module_app() -> Result<App, PluginAssemblyError> {
    App::try_from_builder(
        App::builder(MODULE_APP_ID, LocalizedLabel::native("Playbook Module Procedural", "Playbook-Modul Prozedural")).await
            .document(["semio", "forms"])
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil").await
            .window_kind(MODULE_WINDOW_PARAMS, LocalizedLabel::native("Params", "Parameter"), BODY_PARAMS, SurfaceKind::NodeGraph, "clipboard-list").await
            .window_kind(MODULE_WINDOW_PREVIEW, LocalizedLabel::native("Preview", "Vorschau"), BODY_PREVIEW, SurfaceKind::World3d, "preview").await
            .default_layout(create_default_layout(
                &[MODULE_WINDOW_PARAMS.into(), MODULE_WINDOW_PREVIEW.into()],
                "row",
                Some(&[50.0, 50.0]),
                Some(&["Params".into(), "Preview".into()]),
            )).await
            .mutation(ACTION_EXPORT_SOLID, LocalizedLabel::native("Export Solid", "Volumenkörper exportieren")).await
            .mutation(ACTION_IMPORT_SOLID, LocalizedLabel::native("Import Solid", "Volumenkörper importieren")).await
            .action_args(ACTION_EXPORT_SOLID, vec![solid_format_arg()]).await
            .action_args(ACTION_IMPORT_SOLID, vec![solid_format_arg()]).await
            .action_describe("exportSolidGeometry", LocalizedLabel::native("Exports the block's preview geometry as STEP, OBJ, STL or GLB and stores the result in the block's parameters for the shell to read back; nothing is written to disk.", "Exportiert die Vorschaugeometrie des Bausteins als STEP, OBJ, STL oder GLB und legt das Ergebnis in den Parametern des Bausteins ab, damit die Shell es ausliest; auf die Festplatte wird nichts geschrieben.")).await
            .action_describe("importSolidGeometry", LocalizedLabel::native("Imports solid geometry in the given format (STEP or OBJ text, STL or GLB as base64) into the block and stores the replayable geometry source in its parameters.", "Importiert Volumengeometrie im angegebenen Format (STEP- oder OBJ-Text, STL oder GLB als Base64) in den Baustein und legt die wiederherstellbare Geometriequelle in seinen Parametern ab.")).await
            .action_interactive_job(ACTION_EXPORT_SOLID, InteractiveJobClassification::Migrated).await
            .action_interactive_job(ACTION_IMPORT_SOLID, InteractiveJobClassification::Migrated).await,
    )
    .await
}

/// 🎛️ The shared `format` Select over the solid interchange formats, defaulting to OBJ (the handlers' default).
fn solid_format_arg() -> ActionArgDef {
    ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), SOLID_MEDIA_FORMATS.iter().map(|format| ActionArgOption::new(*format, LocalizedLabel::data(format.to_uppercase()))).collect()).default_value(&"obj")
}

pub fn module_plugin_bundle() -> Result<Plugin<ProceduralModuleApps>, PluginAssemblyError> {
    PluginBuilder::<_, ProceduralModuleApps>::new(MODULE_PLUGIN_ID)
        .label("Playbook Module Procedural")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:playbook-module-procedural")
        .depends_on("playbook", semio_framework::tree_pin!())
        .foreign_document_codec::<ModuleApp>(MODULE_DOCUMENT_SCHEMA)
        .document_app::<ModuleApp>(::semio_framework_async::poll::resolve_ready(create_module_app())?)
        .try_build()
}

pub fn module_extension_bundle() -> ExtensionBundle {
    let mut bundle = ExtensionBundle::new(MODULE_PLUGIN_ID, "Playbook Module Procedural", env!("CARGO_PKG_VERSION"))
        .extends("playbook").depends_on("playbook", semio_framework::tree_pin!()).mode(ExecutionMode::Isolated);
    for (topic, kind_key) in [("playbook.blockKind", "blockKind"), ("forms.questionKind", "questionKind")] {
        bundle = bundle.contributes_topic(topic, DslValue::object([
            ("appId".to_string(), DslValue::String(MODULE_APP_ID.to_string())),
            (kind_key.to_string(), DslValue::String("buildingComponent".to_string())),
            ("label".to_string(), if topic == "forms.questionKind" { semio_framework_value::ToValue::to_value(&LocalizedLabel::native("Building Component", "Bauteil")) } else { DslValue::String("Building Component".to_string()) }),
            ("iconId".to_string(), DslValue::String("building".to_string())),
            ("defaultValueJson".to_string(), DslValue::String(r#"{"height":6,"radius":0.5,"sides":6}"#.to_string())),
            ("paramsBodyKey".to_string(), DslValue::String(BODY_PARAMS.to_string())),
            ("previewBodyKey".to_string(), DslValue::String(BODY_PREVIEW.to_string())),
        ]));
    }
    bundle
}

//#endregion 🔖️App

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🚪️io/🦀️.rs"]
pub mod io;
