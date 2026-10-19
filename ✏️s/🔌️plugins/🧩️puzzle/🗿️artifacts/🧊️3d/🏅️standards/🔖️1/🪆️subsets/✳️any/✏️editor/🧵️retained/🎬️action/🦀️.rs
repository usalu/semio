//! staged placement works (transform, brush placement, suggestion acceptance, focus, precompute commands) that each spend
//! 🎬️ Puzzle 3d retained action works — the one-action-per-window route through the shared action prologue, and the
//! bounded turns. Every work is written against the [`Puzzle3dEditor`] contract, not the concrete editor.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_handle_action, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dActionPrologue, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_shell_only_emit};
use {semio_s_artifact_puzzle_3d_editor_session::Puzzle3dInstanceOperationOwner, semio_s_artifact_puzzle_3d_editor_session::with_puzzle3d_session_for};
use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_LOCALIZATION_UNSUPPORTED, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_addressed_window_id, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_inspector_origin_nudge, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::value_as_vec3};
use {semio_s_artifact_puzzle_3d_commands_document::add_target_volume, semio_s_artifact_puzzle_3d_commands_document::export_snapshot, semio_s_artifact_puzzle_3d_commands_runtime::set_fill_count};
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetFillCount};
use semio_s_artifact_puzzle_3d_editor_panels::artifact;
use {semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_labels, semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_window as window_ownership;
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionWrite;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use {semio_s_artifact_puzzle_3d_retained_document::PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT, semio_s_artifact_puzzle_3d_editor_session as session};
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand;

/// 🧯️ ONE bounded, localized notice as a terminal `Emit` — how a retained placement work whose gesture
/// produced nothing tells the user WHY, instead of the bare `Emit::default()` every refusal path used
/// to complete with (`📓️2026-09-09-user-feature-checklist.md` §9/§13: "a real placement error produces
/// no visible feedback even though the menu correctly closes"). The message is resolved against the
/// host's declared locale×terminology axes; an axis this app never authored carries
/// [`PUZZLE3D_LOCALIZATION_UNSUPPORTED`] rather than an English sentence, because this UI has no
/// default language and a silent drop would restore the very defect being fixed.
///
/// 🐢️ The scope is [`UiDirtyScope::None`], not `Emit::effect`'s `Default` (which is `Full`): a work
/// that refused published nothing, so there is nothing to repaint — the notice itself travels on the
/// effect lane, which the host applies before it ever consults the scope.
fn puzzle3d_notice_emit(view_state: Option<&semio_framework_plugin::ViewModel>, message: impl Fn(&Puzzle3dLabels) -> &'static str) -> Emit<Puzzle3dMutation, Puzzle3dConfigMutation> {
    let text = view_state.and_then(puzzle3d_labels).map_or_else(|| PUZZLE3D_LOCALIZATION_UNSUPPORTED.to_string(), |labels| message(labels).to_string());
    Emit { effects: vec![Effect::Notify { message: text }], ui_scope: UiDirtyScope::None, ..Default::default() }
}

pub const PUZZLE3D_RETAINED_PAYLOAD_SCHEMA: &str = "puzzle.3d.tool-command.v1";

pub fn puzzle3d_retained_extent(command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    if matches!(command.action_id(), "addTargetVolume" | "openAddObjectDialog" | "worldPointerDown") {
        return Some(1);
    }
    let selection = interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let artifact = snapshot.typed();
    let artifact_items = match command.action_id() {
        "focusSelection" | "patchInspector" | "translateSelection" | "rotateSelection" | "scaleSelection" => artifact.objects.len().checked_add(artifact.target_volumes.len())?,
        "createAttraction" | "worldRelocate" => artifact.objects.len().checked_add(artifact.attractions.len())?,
        "addObjectKind" | "setObjectKindWeight" | "setVortexKindWeight" => artifact.meta.kind_catalogs.as_ref().map_or(0, |catalogs| catalogs.objects.len().saturating_add(catalogs.vortices.len())),
        _ => 1,
    };
    selection.checked_add(artifact_items).filter(|items| *items <= semio_s_plugin_puzzle_retained::PUZZLE_COMMAND_WORK_ITEMS)
}

/// 🧵️ Session-less entry point — the shape `PuzzleCommandReducer` demands of a plain fn pointer, for
/// the retained actions that only rewrite the document and gain nothing from a warm cache.
#[expect(clippy::unnecessary_wraps, reason = "Implements the fallible PuzzleCommandReducer callback signature.")]
pub fn puzzle3d_retained_reduce<A: Puzzle3dEditor>(
    command: &Puzzle3dCommand,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dConfig,
    interaction: &protocol::InteractionState,
    hover: &semio_framework_plugin::app::InteractionHoverState,
    view_state: Option<&semio_framework_plugin::ViewModel>,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
    let runtime = window_ownership::runtime(config, &window_ownership::Puzzle3dWindowConfig::default(), &window_ownership::Puzzle3dWindowTransient::default(), view_state);
    if command.action_id() == "openAddObjectDialog" {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }));
    }
    if command.action_id() == "worldPointerDown" {
        return Ok(Emit::default());
    }
    if command.action_id() == "addTargetVolume" {
        let Some(origin) = command.args().and_then(|args| args.get("origin")).and_then(value_as_vec3) else { return Ok(Emit::default()) };
        let mutation = add_target_volume::add_target_volume_mutation(origin, runtime.grid_spacing, runtime.voxel_dims);
        return Ok(Emit { artifact_mutations: vec![mutation], ui_scope: puzzle3d_scope(puzzle3d_command_scope_class("addTargetVolume")), ..Default::default() });
    }
    let snapshot_interaction = Puzzle3dInteractionSnapshot::from_state(interaction, hover);
    let window_id = puzzle3d_addressed_window_id(view_state, None, command.window_id(), &runtime.window_ids);
    Ok(with_puzzle3d_session_for(None, &runtime, |session| puzzle3d_handle_action::<A>(session, "", command, Some(window_id), snapshot, &runtime, view_state, &snapshot_interaction).0))
}

#[path = "🪟️window-command-work/🦀️.rs"]
mod window_command_work;
pub use self::window_command_work::*;

#[path = "🔄️transform-work/🦀️.rs"]
mod transform_work;
#[derive(Clone)]
pub use self::transform_work::*;

#[path = "🖌️brush-object-work/🦀️.rs"]
mod brush_object_work;
pub use self::brush_object_work::*;

#[path = "🎯️focus-selection-work/🦀️.rs"]
mod focus_selection_work;
pub use self::focus_selection_work::*;

#[path = "💡️accept-suggestion-work/🦀️.rs"]
mod accept_suggestion_work;
pub use  pubself::accept_suggestion_work::*;

#[path = "🧠️precompute-command-work/🦀️.rs"]
mod precompute_command_work;
pub use self::precompute_command_work::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod original_construction_tests;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod original_construction_tests;
