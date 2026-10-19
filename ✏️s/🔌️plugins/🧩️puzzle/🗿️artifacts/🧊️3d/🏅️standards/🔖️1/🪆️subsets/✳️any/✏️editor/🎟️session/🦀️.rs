//! lane), the process-global session registry that keeps it across calls, the action context every command arm reads,
//! 🧵️ Puzzle 3d editing session — one document instance's resident state (precompute engine, instance residency, mesh
//! the engine bridge from the transient scene to the precompute session, and the context menu.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FALLBACK_MESH_KIND, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_LOCALIZATION_UNSUPPORTED, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dAttraction, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dInteractionSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dScene, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dTargetVolume, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::collect_mesh_urls, semio_s_artifact_puzzle_3d_editor_snapshot::mesh_selection_ids};
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime};
use {semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsLink, semio_s_artifact_puzzle_3d_precompute_brush::BrushSuggestionsOwner};
use {semio_s_artifact_puzzle_3d_precompute::Puzzle3dCollisionSession, semio_s_artifact_puzzle_3d_precompute::Puzzle3dPrecomputeSession};
use {semio_s_artifact_puzzle_3d_editor_terminology::puzzle3d_labels, semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels};
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use semio_s_artifact_puzzle_3d_model::Puzzle3dEngineCommand;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::mesh_from_kind;
use semio_framework_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionTarget;
use semio_framework_plugin::InteractionWrite;
use semio_framework_ui_locale::Label;
use semio_framework_plugin::MergeMode;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use std::collections::HashMap;
use {std::sync::Mutex, std::sync::OnceLock};

#[path = "🌉️engine-bridge/🦀️.rs"]
mod engine_bridge;
pub use self::engine_bridge::*;

#[path = "🎬️action-context/🦀️.rs"]
mod action_context;
pub use self::action_context::*;

#[path = "🖱️context-menu/🦀️.rs"]
mod context_menu;
pub use self::context_menu::*;

// 🧩️ Puzzle-3d play app. Owns the precompute engine; the persisted document (bare `Puzzle3dSceneSnapshot`
// json) lives in the wrapping `VcsArtifactApp`'s operation store and the view state in
// `Puzzle3dConfig`. Each action rehydrates the engine from the projection, mutates a transient
// [`Puzzle3dScene`], then emits the granular operation delta.
//
#[path = "🗃️session-registry/🦀️.rs"]
mod session_registry;
pub use self::session_registry::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

use semio_framework_plugin::world3d_mesh_id_from_url;
use std::hash::Hasher;

#[path = "🏘️instance-residency/🦀️.rs"]
mod instance_residency;
pub use self::instance_residency::*;

//#region 🧩️EditorContract
/// 🧩️ The puzzle3d editor contract: every editor whose lanes are puzzle3d's play snapshot, mutation union, config,
/// presence and command vocabulary. The action dispatch, the retained works and the tool run builders are written
/// against it, so they sit below the one concrete editor that implements it.
pub trait Puzzle3dEditor: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle3dPlaySnapshot, Mutation = Puzzle3dMutation, Config = Puzzle3dConfig, ConfigMutation = semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresence, PresenceMutation = semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand> {}

impl<A> Puzzle3dEditor for A where A: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle3dPlaySnapshot, Mutation = Puzzle3dMutation, Config = Puzzle3dConfig, ConfigMutation = semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresence, PresenceMutation = semio_s_artifact_puzzle_3d_editor_presence::Puzzle3dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dCommand> {}
//#endregion 🧩️EditorContract
