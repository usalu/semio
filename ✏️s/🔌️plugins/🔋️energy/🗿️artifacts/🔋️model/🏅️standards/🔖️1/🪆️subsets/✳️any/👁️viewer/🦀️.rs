//! 👁️ Energy model viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `EnergyModelViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<EnergyModelViewer>` (framework
//! SDK) is the sole runtime adapter, so this file can never structurally emit an artifact mutation.
//! Must not import anything from the sibling mutation-capable surface (`policyViewerPurityBreaches`).

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_scene as scene;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
#[cfg(test)]
extern crate semio_s_artifact_energy_model_examples as examples;

use crate::modes::view;
use crate::modes::view::windows::{model as model_window, simulation, structure, zones};
use crate::mutations::EnergyModelMutation;
use crate::snapshot::{EnergyModelSnapshot, ENERGY_MODEL_DOCUMENT_SCHEMA, MODEL_DIALECT};
use semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ArtifactViewer;
use semio_framework_plugin::ComponentTree;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::HistoryView;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::NoTransient;
use semio_framework_plugin::NoTransientMutation;
use semio_framework_plugin::ToolExecutionContract;
use semio_framework_plugin::ToolFactoryKey;
use semio_framework_plugin::ToolJobFactoryError;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_plugin::ViewerApp;

//#region 🏷️ActionIds
/// 🏷️ Every dispatchable verb of this VIEWER, exactly once. One row: the 3d window's orbit pose.
///
/// 🔒️ The whole list must stay config-only. A viewer's publication contracts may never name
/// `ArtifactToolPublicationLane::Artifact`, and `setCamera` names `WindowConfig` — the lane that
/// writes ONE window instance's own retained state and nothing else.
const ENERGY_MODEL_VIEW_TOOL_IDS: &[&str] = &[model_window::SET_CAMERA_ACTION_ID];
const ENERGY_MODEL_VIEW_PAYLOAD_SCHEMA: &str = "energy.model.view-command.v1";
/// 🎒️ Real bound for one viewer gesture's wire payload: a pose is two coordinate triples plus a
/// zoom, so 8 KiB is a real ceiling rather than a rubber stamp.
const ENERGY_MODEL_VIEW_RAW_BYTES: usize = 8_192;
/// 🧮️ One window-config write per gesture.
const ENERGY_MODEL_VIEW_WORK_ITEMS: usize = 1;
//#endregion 🏷️ActionIds

#[path = "🔖️view-command/🦀️.rs"]
mod view_command;
pub use view_command::EnergyModelViewCommand;

#[path = "🎥️camera-gesture/🦀️.rs"]
mod camera_gesture;
use camera_gesture::{camera_emit, command_from_action};

//#region 🧵️RetainedCommands
fn energy_model_view_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(ENERGY_MODEL_VIEW_RAW_BYTES, ENERGY_MODEL_VIEW_WORK_ITEMS, 1, 16_384, 7_500)
}

#[expect(clippy::unnecessary_wraps, reason = "The retained work contract takes an optional extent callback, and one camera is always one work item.")]
fn energy_model_view_extent(_command: &EnergyModelViewCommand, _snapshot: &EnergyModelSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    Some(1)
}

/// 👁️ The one reducer the viewer's single tool runs. `InteractiveJobClassification::Migrated` is the
/// only UI-dispatchable classification and a `Migrated` verb without an exact app-owned reducer is
/// refused with `interactive-job.missing-owned-reducer` — so this function existing is precisely what
/// makes the read-only window's orbit survive.
#[expect(clippy::too_many_arguments, reason = "The retained command reducer implements the framework's eight-argument callback contract.")]
#[allow(clippy::needless_pass_by_value)]
fn energy_model_view_reduce(
    command: &EnergyModelViewCommand,
    _snapshot: &EnergyModelSnapshot,
    _config: &NoConfig,
    _history: &HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<ViewerApp<EnergyModelViewer>>>,
    _operation: &AppOperationContext,
) -> Result<Emit<EnergyModelMutation, NoConfigMutation, NoDraftMutation>, Fault> {
    camera_emit(command, context.and_then(|context| context.view_state.as_ref()))
}

struct EnergyModelViewCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl EnergyModelViewCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: ENERGY_MODEL_VIEW_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework_plugin::ToolJobFactory for EnergyModelViewCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<ViewerApp<EnergyModelViewer>>;
    type Job = ArtifactRetainedCommandJob<ViewerApp<EnergyModelViewer>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        ENERGY_MODEL_VIEW_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        energy_model_view_contract()
    }

    semio_framework_plugin_retained_command::retained_command_factory_admission!(maximum_raw_bytes: ENERGY_MODEL_VIEW_RAW_BYTES, checkpoint_resume: false);

}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for EnergyModelViewCommandJobFactory {
    type Owner = ViewerApp<EnergyModelViewer>;
    const TOOL_IDS: &'static [&'static str] = ENERGY_MODEL_VIEW_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;
    /// 🔒️ WINDOW-CONFIG ONLY. This one row is the runtime half of the read-only guarantee: naming
    /// `ArtifactToolPublicationLane::Artifact` here would be rejected against a viewer's own emit.
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: model_window::SET_CAMERA_ACTION_ID, lanes: &[ArtifactToolPublicationLane::WindowConfig] }];
}
//#endregion 🧵️RetainedCommands

//#region 🔖️Viewer
#[derive(Default, Clone, Copy)]
pub struct EnergyModelViewer;

impl ArtifactViewer for EnergyModelViewer {
    type Snapshot = EnergyModelSnapshot;
    type Mutation = EnergyModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = NoTransient;
    type TransientMutation = NoTransientMutation;
    type Command = EnergyModelViewCommand;
    /// 🧩️ Read-only twin of the editor's roster: `structure`/`zones` are stdio `value`/`table` members.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;

    const DIALECT: Dialect = MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = ENERGY_MODEL_DOCUMENT_SCHEMA;

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        crate::snapshot::energy_child_restore_projection(snapshot)
    }

    /// 🌱️ Both composed children derive from the model — see `crate::energy_genesis_child_pack`.
    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        crate::snapshot::energy_genesis_child_pack(snapshot, slot, child_id)
    
})())
}

    fn initial_snapshot() -> EnergyModelSnapshot {
        EnergyModelSnapshot::default()
    }

    /// 🛂️ A viewer declares every nontrivial store owner explicitly (the trait defaults fail closed);
    /// the document store is the bounded pair the editor also uses, and every other lane is `No*`.
    /// Without these the two genesis members can never be retired (`interactive-job.close-owned-
    /// disposer-missing`).

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }


    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::no_config_store_disposer())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_framework_plugin::no_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_peer_retirement_factory())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::no_presence_local_root_retirement_factory())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn command_id(command: &Self::Command) -> &'static str {
        command.action_id()
    }

    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        command_from_action(action, args)
    }

    /// 👁️ Structurally read-only. `ViewEmit` has no window-config lane at all, so the ONE verb this
    /// viewer owns cannot be served here — `setCamera` is `InteractiveJobClassification::Migrated`
    /// and therefore travels the RETAINED route ([`energy_model_view_reduce`]), whose `Emit` does
    /// carry `window_config_mutations`. This arm exists so that route's absence is a loud refusal
    /// rather than a silently dropped orbit.
    fn handle(
        _command: &Self::Command,
        _doc: &ArtifactView<'_, Self::Snapshot>,
        _cfg: &ConfigView<'_, Self::Config>,
        _interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _engines: &semio_framework_2d_compute::EngineHandles,
    ) -> Result<ViewEmit<Self::ConfigMutation>, Fault> {
        Err(Fault::new(
            FaultOrigin::App,
            FaultCode::new("energy.model.3d.viewer.retained-route-required"),
            "a viewer camera is a window-config write, which ViewEmit cannot carry — it must reach the retained reducer",
        ))
    }

    /// 🎥️ The 3d viewer window's own retained orbit pose — one bounded window-config store, keyed by
    /// window INSTANCE, so two open read-only panes never share a camera. Its state and its verb are
    /// authored under `👁️viewer`, never imported from the editor's twin.
    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<model_window::config::EnergyModelViewerWindowConfigOwner>()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, ViewerApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(EnergyModelViewCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: &mut Option<ArtifactOwnedToolJobRequest<ViewerApp<Self>>>, output: &mut Option<semio_framework::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        let original = request.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original tool producer request is absent"))?;
        if !ENERGY_MODEL_VIEW_TOOL_IDS.contains(&original.tool_id.as_str()) {
            return Ok(false);
        }
        if original.command.action_id() != original.tool_id {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "the energy model view command does not match its exact registered tool").into());
        }
        let tool_id = original.command.action_id();
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, EnergyModelViewCommand::action_id, ENERGY_MODEL_VIEW_RAW_BYTES, 1, false, |_request| BoundedArtifactCommandWork::new(tool_id, energy_model_view_reduce, energy_model_view_extent))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: ViewerApp<EnergyModelViewer>,
        owner_file: "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs",
        controller: "s.energy.model@1/*#viewer",
        artifact_schema: "energy.model",
        factory: "EnergyModelViewCommandJobFactory",
        factory_type: EnergyModelViewCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 1, 1, 16_384, 7_500),
        tools: ["setCamera"]
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_framework_plugin::ViewModel) -> UiAssemblyResult<ComponentTree> {
        let node = match body_key {
            structure::BODY_KEY => structure::render(doc.snapshot)?,
            zones::BODY_KEY => zones::render(doc.snapshot)?,
            simulation::BODY_KEY => simulation::render(&doc.snapshot.model),
            model_window::BODY_KEY => model_window::render_with_camera(&doc.snapshot.model, model_window::config::current(cfg))?,
            _ => {
                semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("energy.model.viewer.render", "the unknown-body label could not be assembled"))?
            }
        };
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }
}
//#endregion 🔖️Viewer

//#region 🔖️Manifest
pub fn create_energy_model_viewer() -> semio_framework_plugin::AppDefinition {
    Viewer::builder(MODEL_DIALECT)
        .document(["semio", "energy", "model"])
        .icon_id("battery")
        .mode_def(view::definition())
        .default_mode_id(view::ENERGY_MODEL_VIEW_MODE_ID)
        .window_kind_def(structure::definition())
        .window_kind_def(zones::definition())
        .window_kind_def(simulation::definition())
        .window_kind_def(model_window::definition())
        .default_layout(view::layout())
        .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧩️Facets
#[path = "👥️presence/🧬️schema/🦀️.rs"]
pub mod presence;
#[path = "🎚️config"] pub mod config {
    #[path = "🧬️schema/🦀️.rs"]
    pub mod schema;
}
#[path = "🎭️modes"] pub mod modes {
    #[path = "👁️view"] pub mod view {
        #[path = "🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "🪟️windows"] pub mod windows {
            #[path = "🧊️model/🦀️.rs"]
            pub mod model;
            #[path = "⚡️simulation/🦀️.rs"]
            pub mod simulation;
            #[path = "🌳️structure/🦀️.rs"]
            pub mod structure;
            #[path = "📊️zones/🦀️.rs"]
            pub mod zones;
        }
    }
}
//#endregion 🧩️Facets
