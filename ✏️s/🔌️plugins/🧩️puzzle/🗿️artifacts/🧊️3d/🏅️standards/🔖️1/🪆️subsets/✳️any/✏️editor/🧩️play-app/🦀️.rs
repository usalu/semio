//! 🧩️ Play app: `Puzzle3dPlayApp`, the `ArtifactEditor` the framework mounts, and its context menu, render and window measure bodies.

use super::*;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;
use semio_s_artifact_puzzle_3d_editor_session::{puzzle3d_view_session_key, sync_precompute_session, with_puzzle3d_session_for};
use semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main::utilities;
use semio_s_artifact_puzzle_3d_editor_modes::windows::main;
use semio_s_artifact_puzzle_3d_editor_panels::artifact;
use semio_s_artifact_puzzle_3d_editor_panels::catalogue;
use semio_s_artifact_puzzle_3d_editor_panels::inspection;
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_presence as presence;
use semio_s_artifact_puzzle_3d_editor_panels::settings as settings_panel;

/// 🧩️ The puzzle-3d play editor: the `ArtifactEditor` the framework mounts as `EditorApp<Puzzle3dPlayApp>`. It holds no
/// state of its own — every call checks the document instance's [`Puzzle3dSession`] out of the session registry, and
/// the persisted document lives in the wrapping `VcsArtifactApp`'s operation store.
#[derive(Clone, Copy, Debug, Default)]
pub struct Puzzle3dPlayApp;

impl ArtifactEditor for Puzzle3dPlayApp {
    const DIALECT: Dialect = semio_s_artifact_puzzle_3d_model::PUZZLE3D_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_puzzle_3d_model::PUZZLE_3D_SCHEMA;
    type Snapshot = Puzzle3dPlaySnapshot;
    type Mutation = Puzzle3dMutation;
    type Config = Puzzle3dConfig;
    type ConfigMutation = Puzzle3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Puzzle3dPresence;
    type PresenceMutation = Puzzle3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = Puzzle3dCommand;

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_config(registry)
    }

    /// 🎮️ The six framework interaction verbs, answered out of this app's OWN scope table
    /// ([`Puzzle3dScopeClass::Interaction`]) instead of the framework's blanket `UiDirtyScope::Full`: a
    /// hover repaints the world body alone, a pick adds the inspector + outliner rows + history panel,
    /// a mode/granularity switch only the window's Select chrome. A verb that touched any domain this
    /// app does not declare — or none at all — answers `None`, and the framework keeps its widest scope.
    fn interaction_scope(verb: InteractionVerb, domains: &[&str]) -> Option<UiDirtyScope> {
        let declared = !domains.is_empty() && domains.iter().all(|domain| *domain == PUZZLE3D_INTERACTION_DOMAIN);
        declared.then(|| puzzle3d_scope(Puzzle3dScopeClass::Interaction(verb)))
    }

    /// 🪧️ A history-edit reference chip names its world entity as the outliner does ([`puzzle3d_entity_label`]).
    fn entity_label(snapshot: &Puzzle3dPlaySnapshot, kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        puzzle3d_entity_label(snapshot.typed(), kinds, id)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        window_ownership::register_transient(registry)
    }



    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Puzzle3dConfigStorePreparationFactory))
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Puzzle3dArtifactStorePreparationFactory))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }


    /// 🧽️ puzzle3d owns no draft lane, but `VcsArtifactApp::close_step` still walks it: every one of
    /// its seven owned lanes is mandatory, and the `interaction-store` lane behind this one never runs
    /// until the draft/presence/transient disposers exist.
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(presence::Puzzle3dPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(presence::puzzle3d_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    /// 🧭️ The two framework-injected host-configuration verbs, resolved to one event-sourced Config
    /// mutation each instead of a retained tool operation. The shell owns their session state and then
    /// forwards the resolved value here so the app can clear/prepare its scratch
    /// (`🏛️ShellHost/🟦️.tsx`'s `SET_ACTIVE_TOOL_ACTION_ID`/`SET_ACTIVE_UTILITY_ACTION_ID` branches);
    /// without this hook both verbs fell through to `admit_command_json` and failed closed with
    /// `interactive-job.missing-factory`, so the fill tool could not even be selected.
    ///
    /// 🧰️ `setActiveUtility` needs no variant of its own: puzzle3d's active utility IS host view state
    /// (`puzzle3d_scene_active_utility` reads `ViewModel.active_utility_by_window_id`), and the only
    /// app-owned state a utility switch invalidates is that window's engagement input.
    fn host_configuration_mutation(_action: &str, _args: Option<&semio_framework_value::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok(None)
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        let mut proofs = Puzzle3dRetainedCommandProofs::proofs();
        proofs.extend(Puzzle3dHostConfigurationProofs::proofs());
        proofs
    }

    /// ⏯️ The fill tool's run and finalize revalidation jobs and the brush's read-only suggestions run
    /// (`📋️tool-run-contract.md` §3.7).
    fn build_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<semio_framework_plugin::ToolRunJob>, Fault> {
        match request.tool_id {
            utilities::brush::UTILITY_ID => utilities::brush::build_run_job(request),
            _ => Ok(None),
        }
    }

    /// 🎯️ The fill tool's run and finalize revalidation jobs, retargeted in place across base and count changes.
    fn build_retargetable_tool_run_job(request: semio_framework_plugin::ToolRunJobRequest<'_, EditorApp<Self>>) -> Result<Option<Box<dyn semio_framework_plugin::ToolRunRetargetableJob<Self::Config>>>, Fault> {
        fill_tool::build_run_job(request)
    }

    /// 📢️ The localized notices of this guest's refusal codes ([`puzzle3d_fault_notices`]).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        puzzle3d_fault_notices()
    }

    /// 🧠️ The instance-retained owner the brush suggestions link lives in.
    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::new(Puzzle3dInstanceOperationOwner::default())
    }

    /// 🚦️ Starts, wakes or aborts the brush suggestions run for the vortex the link points at
    /// ([`utilities::brush::run_effects`]).
    fn pending_effects(owner: &ArtifactInstanceOperationOwnerHandle, doc: &semio_framework_plugin::ArtifactPendingView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, _view: Option<&semio_framework_plugin::ViewModel>) -> Vec<Effect> {
        owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(utilities::brush::run_effects(&mut owner.brush_suggestions, doc.tool_run()))).unwrap_or_default()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Puzzle3dRetainedCommandJobFactory::new(&controller))?;
        Ok(())
    }

    fn build_tool_job(request: &mut Option<semio_framework_plugin::ArtifactOwnedToolJobRequest<EditorApp<Self>>>, output: &mut Option<semio_framework::ErasedToolJob>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool,semio_framework_job::OriginalTurnFailure> {
        let original=request.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original puzzle3d producer request is absent"))?;
        if !PUZZLE3D_RETAINED_TOOL_IDS.contains(&original.tool_id.as_str()) {return Ok(false);}
        if original.command.action_id()!=original.tool_id {return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"puzzle3d-command-tool-mismatch").into());}
        let tool_id=original.command.action_id();
        macro_rules! work {
            (|$request:ident| $make:expr) => {
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer(request,output,cx,Puzzle3dCommand::action_id,PUZZLE3D_IMPORT_RAW_BYTES,semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS,true,|$request|{let _=&$request;$make})
            };
        }
        macro_rules! seed {
            (|$request:ident,$seed:ident| $make:expr) => {{
                let bytes=original.authoring_seed.len();
                let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured(request,output,cx,Puzzle3dCommand::action_id,PUZZLE3D_IMPORT_RAW_BYTES,semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS,true,demand,|$request,_|{
                    let $seed=$request.authoring_seed.clone();let capacity=$seed.capacity();
                    Ok(($make,semio_framework_job::RetainedCloneProgress{copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))
                })
            }};
        }
        macro_rules! bound {
            (|$request:ident,$instance:ident| $make:expr) => {{
                let bytes=original.parent_document_id.len();
                let demand=semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()};
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured(request,output,cx,Puzzle3dCommand::action_id,PUZZLE3D_IMPORT_RAW_BYTES,semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS,true,demand,|$request,_|{
                    let parent_document_id=$request.parent_document_id.clone();let capacity=parent_document_id.capacity();
                    let $instance=Puzzle3dInstanceBinding{app_instance_id:$request.app_instance_id,parent_document_id,owner:$request.instance_operation_owner.clone()};
                    Ok(($make,semio_framework_job::RetainedCloneProgress{copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()}))
                })
            }};
        }
        macro_rules! owned {
            ($owner:ty, |$request:ident, $scope:ident| $make:expr) => {{
                let demand = <$owner>::original_birth_demands()?;
                semio_framework_plugin::retained_command::producer::admit_retained_tool_producer_measured_scoped(
                    request, output, cx, Puzzle3dCommand::action_id, PUZZLE3D_IMPORT_RAW_BYTES,
                    semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS, true, demand,
                    |$request, $scope| { let _ = &$request; $make },
                )
            }};
        }
        match tool_id {
            "translateSelection" | "rotateSelection" | "scaleSelection" | "worldRelocate" | "relocateTargetVolume" => seed!(|request, seed| Puzzle3dTransformWork::new(tool_id, seed)),
            "patchInspector" if puzzle3d_inspector_origin_nudge(original.command.args()).is_some() => seed!(|request, seed| Puzzle3dTransformWork::new(tool_id, seed)),
            "patchInspector" => owned!(Puzzle3dPatchInspectorWork, |request, scope| Puzzle3dPatchInspectorWork::from_original_construction(scope)),
            "createAttraction" => work!(|request| Puzzle3dCreateAttractionWork::default()),
            "setActiveExample" => owned!(Puzzle3dSetActiveExampleWork, |request, scope| Puzzle3dSetActiveExampleWork::from_original_construction(scope)),
            "addBrushObject" => owned!(Puzzle3dAddBrushObjectWork, |request, scope| Puzzle3dAddBrushObjectWork::from_original_construction(scope)),
            "addObjectKind" => owned!(Puzzle3dAddObjectKindWork, |request, scope| Puzzle3dAddObjectKindWork::from_original_construction(scope)),
            "engagementAbort" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "engagementRepeatLast" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "engagementSubmit" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "setObjectKindWeight" | "setVortexKindWeight" => owned!(Puzzle3dKindWeightWork, |request, scope| Puzzle3dKindWeightWork::from_original_construction(tool_id, scope)),
            "acceptSuggestion" => owned!(Puzzle3dAcceptSuggestionWork, |request, scope| Puzzle3dAcceptSuggestionWork::from_original_construction(scope).map(|(work, progress)| (work.bound(Puzzle3dInstanceBinding { app_instance_id: request.app_instance_id, parent_document_id: String::new(), owner: request.instance_operation_owner.clone() }), progress))),
            "cycleBrushCandidate" | "cycleBrushCandidateBack" | "registerBrushMesh" | "setFillCount" => bound!(|request, instance| Puzzle3dPrecomputeCommandWork::new(tool_id).bound(instance)),
            "focusSelection" => work!(|request| Puzzle3dFocusSelectionWork::default()),
            "setCamera"
            | "setProjection"
            | "setProjectionParam"
            | "toggleSun"
            | "setSunAzimuth"
            | "setSunElevation"
            | "setSunIntensity"
            | "setLodAutomatic"
            | "setLodDepthVariable"
            | "setLodManual"
            | "setGridVisible"
            | "setGridSnapEnabled"
            | "setGridSpacing"
            | "setSelectableKind"
            | "setProximityRadius"
            | "setChunkSize"
            | "setVoxelDims"
            | "setTransformGumballFlag"
            | "setVortexShow"
            | "setVortexDirection"
            | "setBrushPlacementContactTolerance"
            | "openVortexSuggestions"
            | "closeVortexSuggestions"
            | "targetBrushSuggestions"
            | "hoverSuggestion"
            | "engagementControlSelect"
            | "engagementInput" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "exportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "openImportSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "importSnapshot" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "addTargetVolume" => bound!(|request, instance| Puzzle3dWindowCommandWork::new(tool_id).bound(instance)),
            "worldPointerDown" => work!(|request| semio_s_plugin_puzzle_retained::NoopPuzzleCommandWork::new(tool_id)),
            _ => work!(|request| semio_s_plugin_puzzle_retained::BoundedFirstStepCommandWork::new(tool_id, semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_reduce::<Self>, semio_s_artifact_puzzle_3d_retained_action::puzzle3d_retained_extent)),
        }
    }

    /// 📎 Ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d: replaces the old
    /// `crate::editor::puzzle3d::config::schema::register_app_schema()` self-registering call, which
    /// puzzle's plugin root used to reach `.setup()` for — `register_document_app`/`document_app`
    /// now call this automatically the moment `Puzzle3dPlayApp` is bound to a plugin, exactly like
    /// `🗒️note`'s own `app_schema` override.
    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_puzzle_3d_editor_config::schema::app_schema_descriptor())
    }

    /// 🚀️ The `default_scene_snapshot()` (Concrete Forest) document, nothing else.
    ///
    /// 🐛️ This used to pre-warm `NAKAGIN_EXAMPLE_SNAPSHOT` first — 128 755 B of DSL parsed into a typed
    /// scene_snapshot that the boot document never reads, paid by every mount before its first frame. The
    /// static is a `LazyLock`: the example that needs it forces it when it is actually switched to.
    /// It then built a scene and synced a whole precompute session (mesh fallbacks, collision scene) into a
    /// throwaway `Puzzle3dPlayApp` dropped on return: every rendering app syncs its own session before its
    /// first body (`with_puzzle3d_app_for`), so that work reached no one — while every `codec` call of a hub
    /// creating or validating a document paid for it in the interpreter.
    fn reference_birth()->Option<fn()->Puzzle3dPlaySnapshot>{Some(||{
        Puzzle3dPlaySnapshot::new(puzzle3d_snapshot_from_host_snapshot(&default_scene_snapshot()).expect("authored initial snapshot admits"))
    })}

    fn clipboard_media_type() -> Option<MediaType> {
        Some(MediaType { class: MediaClass::ThreeD, form: MediaForm::Design })
    }

    fn copy_fragment(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Result<ClipboardFragment, ClipboardError> {
        puzzle3d_copy_fragment(doc, interaction)
    }

    fn cut_operations(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>, interaction: &InteractionView<'_>) -> Vec<Puzzle3dMutation> {
        puzzle3d_cut_operations(doc, interaction)
    }

    fn paste_operations(_doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle3dMutation>, ClipboardError> {
        puzzle3d_paste_operations(fragment, placement)
    }

    fn build_reserved_tool_job(request: ArtifactReservedToolJobRequest<EditorApp<Self>>) -> Result<Option<ArtifactReservedToolJob>, Fault> {
        if !matches!(request.tool_id.as_str(), "copy" | "cut" | "paste") {
            return Ok(None);
        }
        Ok(Some(ArtifactReservedToolJob::new(Puzzle3dClipboardJob::new(request))))
    }

    /// 🏷️ Maps each `Puzzle3dCommand` variant back to the action id it was declared under.
    fn command_id(command: &Puzzle3dCommand) -> &'static str {
        command.action_id()
    }

    /// 🎯️ Maps the host's transitional `{action,args}` wire onto Puzzle 3D's closed command
    /// enum until React and wgpu send `OpBinary` command bytes directly.
    fn command_from_action(action: &str, args: &mut Option<semio_framework_value::DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() {
            return Err(fault);
        }
        let args = args.as_ref();
        let window_id = args.and_then(|value| value.get("windowId").or_else(|| value.get("window_id"))).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        if let Some(flag) = puzzle3d_flag_value_argument(action) {
            args.and_then(|value| value.get(flag)).and_then(semio_framework_value::DslValue::as_bool).ok_or_else(|| Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("puzzle3d.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets")))?;
        }
        let args = args.map(semio_framework_pack_json::from_dsl_value);
        Puzzle3dCommand::from_action(action, args, window_id).ok_or_else(|| Fault::from(format!("unknown Puzzle 3D action '{action}'")))
    }

    /// 🧩️ Thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple `handle_action_impl` expects from the typed `Puzzle3dCommand`.
    fn handle(
        command: &Puzzle3dCommand,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, Fault> {
        let interaction = Puzzle3dInteractionSnapshot::from_interaction(interaction);
        let window = window_ownership::config_from_view(cfg);
        let runtime = window_ownership::runtime(cfg.snapshot, &window, &window_ownership::Puzzle3dWindowTransient::default(), view_state);
        let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
        let authoring_seed = doc.operation_optional().map_or("", |operation| operation.authoring_seed.as_str());
        Ok(with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |session| puzzle3d_handle_action::<Self>(session, authoring_seed, command, Some(window_id), doc.snapshot, &runtime, view_state, &interaction).0))
    }

    /// 🕹️ `vortex` domain topology (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
    /// objects and every root-level entity kind (vortex marker, attraction, target volume, reference,
    /// catalogue kind) as one flat forest, except object-owned vortex markers whose parent is the
    /// object they mark — the one real nesting relationship this app's document carries, replacing
    /// what `hoveredVortexFullId`'s ad hoc highlighting used to do by hand.
    /// 🔭️ Read off the SAME projection every render publishes (`puzzle3d_scene_snapshot_from_projection`),
    /// never a second decoding of the document: `protocol::validate_state` prunes every selected id
    /// this topology does not contain, so a universe narrower than what the viewport paints makes a
    /// pick land on the leftover and then vanish — with no error anywhere. One authority is what
    /// makes "the user can see it" and "the user can pick it" the same statement
    /// (26/09/02/PUZZLE-3D-END-TO-END wave B9 lane 2).
    fn interaction_topology(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, _cfg: &ConfigView<'_, Puzzle3dConfig>) -> Result<semio_framework_plugin::InteractionTopology, semio_framework_value::ValueError> {
 Ok((||{
        #[cfg(test)]
        PUZZLE3D_INTERACTION_TOPOLOGY_BUILDS.with(|counter| counter.set(counter.get().saturating_add(1)));
        let snapshot = puzzle3d_scene_snapshot_from_document(doc.snapshot.typed());
        let mut ordered = Vec::new();
        for object in &snapshot.objects {
            ordered.push(semio_framework_plugin::TopologyNode { id: object.id.clone(), granularity: PUZZLE3D_GRANULARITY_OBJECT.into(), parent: None });
            for vortex in &object.vortices {
                ordered.push(semio_framework_plugin::TopologyNode { id: puzzle3d_vortex_full_id(&object.id, &vortex.id), granularity: PUZZLE3D_GRANULARITY_VORTEX.into(), parent: Some(object.id.clone()) });
            }
        }
        for attraction in &snapshot.attractions {
            ordered.push(semio_framework_plugin::TopologyNode { id: attraction.id.clone(), granularity: PUZZLE3D_GRANULARITY_ATTRACTION.into(), parent: None });
        }
        for volume in &snapshot.target_volumes {
            ordered.push(semio_framework_plugin::TopologyNode { id: volume.id.clone(), granularity: PUZZLE3D_GRANULARITY_TARGET_VOLUME.into(), parent: None });
        }
        for reference in &snapshot.references {
            ordered.push(semio_framework_plugin::TopologyNode { id: reference.id.clone(), granularity: PUZZLE3D_GRANULARITY_REFERENCE.into(), parent: None });
        }
        for kind in puzzle3d_kind_ids(&snapshot, "objects") {
            ordered.push(semio_framework_plugin::TopologyNode { id: kind, granularity: PUZZLE3D_GRANULARITY_KIND.into(), parent: None });
        }
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PUZZLE3D_INTERACTION_DOMAIN.to_string(), semio_framework_plugin::DomainTopology { ordered });
        semio_framework_plugin::InteractionTopology { domains }
    
})())
}

    /// 🔌️ Declares puzzle3d's typed media I/O surface — the implicit document ports plus the flagship
    /// `kit:in` seam: an input port accepting `Kit×Type` media tagged `kit.catalog`, fanning IN from
    /// potentially many producers (`multiplicity: Many`).
    fn io() -> Option<AppIo> {
        Some(puzzle3d_io())
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (`objectKinds`/`vortexKinds`/
    /// `cableKinds`/`attractionKinds`/`kindCompatibility`) into puzzle3d's own `meta.kind_catalogs`
    /// vocabulary (`objects`/`vortices`/`cables`/`attractions`) and upserts it (keyed by row `id`,
    /// deterministic/order-independent — safe for `multiplicity: Many` fan-in) via the same
    /// concrete `replace-kind-catalogs` and `connect-`/`disconnect-kind-compatibility` kinds, so this never mutates
    /// anything directly — only real, undoable operations.
    fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = parse(json.as_str(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let before = doc.snapshot.typed();
        let scene_snapshot = puzzle3d_scene_snapshot_from_document(before);

        let mut catalogs: semio_framework_value::DslValue = scene_snapshot.meta.kind_catalogs.clone().unwrap_or_else(|| {
            semio_framework_value::DslValue::object([
                ("objects".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("vortices".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("cables".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
                ("attractions".to_string(), semio_framework_value::DslValue::Array(Vec::new())),
            ])
        });
        puzzle3d_upsert_catalog_rows(&mut catalogs, "objects", fragment.get("objectKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "vortices", fragment.get("vortexKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "cables", fragment.get("cableKinds"));
        puzzle3d_upsert_catalog_rows(&mut catalogs, "attractions", fragment.get("attractionKinds"));
        let kind_catalogs: Option<semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs> = Some(<semio_s_artifact_puzzle_3d_model::Puzzle3dKindCatalogs as FromValue>::from_value(catalogs).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);
        let mut operations = Vec::new();
        if kind_catalogs != before.meta.kind_catalogs {
            operations.push(semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::replace_kind_catalogs(kind_catalogs));
        }

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut rows = before.meta.kind_compatibility.clone();
            for incoming in incoming_compat {
                let row = <semio_s_artifact_puzzle_3d_model::Puzzle3dKindCompatibility as FromValue>::from_value(semio_framework_pack_json::to_dsl_value(incoming)).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let connect = semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::connect_kind_compatibility(row.source.clone(), row.target.clone(), row.bidirectional, row.important, row.specificity, None);
                match rows.iter().position(|entry| entry.source == row.source && entry.target == row.target) {
                    None => {
                        operations.push(connect);
                        rows.push(row);
                    }
                    Some(index) if rows[index] != row => {
                        operations.push(semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::disconnect_kind_compatibility(row.source.clone(), row.target.clone()));
                        operations.push(connect);
                        rows[index] = row;
                    }
                    Some(_) => {}
                }
            }
        }

        Ok(Emit::mutations(operations))
    }

    /// 🕹️ Plain `render` is the no-interaction entry point the framework only reaches through
    /// `render_with_request_context`'s default body — this app overrides that, so every real render
    /// carries a live selection. Kept as a thin delegate against an empty snapshot rather than a
    /// second body, so there is exactly one render implementation
    /// ([`Puzzle3dPlayApp::render_body`]).
    fn render(body_key: &str, doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        Self::render_body(None, body_key, doc, cfg, view_state, &window_ownership::Puzzle3dWindowTransient::default(), &Puzzle3dInteractionSnapshot::default())
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `vortex` domain
    /// (selection + `"pointer"` hover) once per render and threads it into the whole body — the world
    /// scene's selection/gumball/vortex-marker paint, the suggestion popup and the inspection panel's
    /// per-entity field groups all read it instead of ever storing selection in this app again.
    fn render_with_request_context(
        owner: &ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> semio_framework_plugin::plugin_app_close_prelude::semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let window_transient = window_ownership::transient_from_view(transient);
        Self::render_body(Some(owner), body_key, doc, cfg, view_state, &window_transient, &Puzzle3dInteractionSnapshot::from_interaction(interaction)).map(Some).map_err(Into::into)
    }

    /// 🕹️ The context menu reads the AUTHORITATIVE framework-owned selection, not only the
    /// client-supplied `request.surface.selection`: `World3dHost` only ever puts its painted object
    /// ids (and component ids) into that field, so a selected vortex / target volume / reference would
    /// otherwise never reach a menu row. The surface's own hits/selection still win when present, so a
    /// right-click on an unselected entity still targets what was clicked.
    fn context_menu_with_request_context(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &InteractionView<'_>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::from_interaction(interaction), registry)
    }

    fn window_engagements(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |_session| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels, doc.tool_run()))])
        })
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, Self::Transient>,
        _interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::transient_from_view(transient), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |_session| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::engagement(&envelope, labels, doc.tool_run()))])
        })
    }

    fn window_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        Self::window_measures_body(doc, cfg, view_state)
    }

    fn tool_measures(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |_session| {
            let wid = view_state.window_id.as_deref().unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &runtime, Some(view_state), wid);
            HashMap::from([(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID.to_string(), fill_tool::measures(&envelope, labels, doc.tool_run()))])
        })
    }

    fn context_menu(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        Self::context_menu_body(request, doc, cfg, view_state, &Puzzle3dInteractionSnapshot::default(), registry)
    }
}

impl Puzzle3dPlayApp {
    /// 🖱️ The ONE context-menu implementation — `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    fn context_menu_body(
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &Puzzle3dInteractionSnapshot,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        let Some(labels) = puzzle3d_labels(view_state) else { return Vec::new() };
        let wid = view_state.window_id.as_deref().unwrap_or(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID);
        let active_utility = puzzle3d_scene_active_utility(&config, Some(view_state), Some(wid));
        let envelope = scene_from_snapshot(doc.snapshot.typed(),config,&active_utility);
        let mut selection = Puzzle3dContextSelection::from_surface(request.surface.as_ref());
        selection.fill_from_interaction(interaction);
        puzzle3d_context_menu_items(&envelope, &selection, labels, view_state, registry)
    }

    /// 🖼️ The ONE render implementation — both `ArtifactEditor::render` and
    /// `render_with_request_context` funnel here, differing only in whether `interaction` carries a
    /// live `vortex`-domain read or the empty default.
    pub fn render_body(
        owner: Option<&ArtifactInstanceOperationOwnerHandle>,
        body_key: &str,
        doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>,
        cfg: &ConfigView<'_, Puzzle3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &window_ownership::Puzzle3dWindowTransient,
        interaction: &Puzzle3dInteractionSnapshot,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let runtime = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), transient, Some(view_state));
        let node = with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &runtime, |app| -> semio_framework_plugin::UiAssemblyResult<_> {
            let (base_body_key, window_id_from_key) = body_key.split_once(':').map_or((body_key, None), |(b, w)| (b, Some(w)));
            let config = &runtime;
            let wid = puzzle3d_addressed_window_id(Some(view_state), window_id_from_key, None, &config.window_ids);
            let envelope = main::scene_for(doc.snapshot, config, Some(view_state), wid);
            {
                let mut precompute = app.precompute.borrow_mut();
                sync_precompute_session(&mut precompute, &envelope);
            }
            let precompute = app.precompute.borrow();
            let labels = puzzle3d_labels(view_state).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.localization.unsupported", "puzzle3d has no authored label set for the host's locale/terminology axes"))?;
            match base_body_key {
                main::BODY_KEY => {
                    let provisional = doc.tool_run().map(|run| std::sync::Arc::clone(&run.provisional_entities)).unwrap_or_default();
                    let (instances_json, meshes_json, instances_delta_json) = app.geometry_jsons(&envelope.scene_snapshot, &provisional, &interaction.referenced);
                    let menu_target = envelope.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.as_str());
                    let suggestions = menu_target.and_then(|target| owner?.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(owner.brush_suggestions.found(target).cloned())).ok().flatten());
                    main::render(&envelope, &precompute, instances_json, meshes_json, instances_delta_json, interaction, suggestions.as_ref())
                }
                artifact::BODY_KEY => artifact::render(&envelope.scene_snapshot, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, artifact::BODY_KEY)),
                catalogue::BODY_KEY => catalogue::render(&envelope, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, catalogue::BODY_KEY)),
                inspection::BODY_KEY => inspection::render(&envelope, interaction, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, inspection::BODY_KEY)),
                settings_panel::BODY_KEY => settings_panel::render(&envelope, labels, wid),
                _ => semio_framework_plugin::built_text_node(Label::data(format!("Unknown body: {body_key}"))).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "puzzle3d unknown-body label admission failed")),
            }
        })?;
        Ok(semio_framework_plugin::built_to_component_tree(node))
    }

    /// 🪟️ The ONE per-window-instance chrome implementation `ArtifactEditor::window_measures` funnels into.
    fn window_measures_body(doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>, cfg: &ConfigView<'_, Puzzle3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let Some(window_id) = view_state.window_id.as_deref() else { return HashMap::new() };
        let config = window_ownership::runtime(cfg.snapshot, &window_ownership::config_from_view(cfg), &window_ownership::Puzzle3dWindowTransient::default(), Some(view_state));
        with_puzzle3d_session_for(puzzle3d_view_session_key(doc), &config, |app| {
            let Some(labels) = puzzle3d_labels(view_state) else { return HashMap::new() };
            let envelope = main::scene_for(doc.snapshot, &config, Some(view_state), window_id);
            HashMap::from([(window_id.to_string(), main::window_measures(&envelope, labels))])
        })
    }
}
