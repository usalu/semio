//! 🩹️ object kinds, the inspector patch, attraction creation and the active example switch. Every work is written against
//! 🗂️ Puzzle 3d retained document works — the staged edits that read the document before they emit: kind weights,
//! the [`Puzzle3dEditor`] contract, not the concrete editor.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_editor_snapshot::CONCRETE_FOREST_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::EMPTY_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::NAKAGIN_EXAMPLE_SNAPSHOT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_ATTRACTION, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_REFERENCE, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_TARGET_VOLUME, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_VORTEX, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_ID_COUNTER, semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_INTERACTION_DOMAIN, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dAttraction, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dObject, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dReference, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dSceneSnapshot, semio_s_artifact_puzzle_3d_editor_dispatch::Puzzle3dScopeClass, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dTargetVolume, semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dVortex, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_apply_distribution_weight, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_axis_index, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_command_scope_class, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_next_object_label, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_object_joint_total, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_resolve_number_edit, semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_scene_snapshot_from_document, semio_s_artifact_puzzle_3d_editor_dispatch::puzzle3d_scope, semio_s_artifact_puzzle_3d_editor_snapshot::value_as_vec3};
use {semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_EXAMPLE_CONCRETE_FOREST, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_EXAMPLE_NAKAGIN};
use semio_s_artifact_puzzle_3d_commands_document::delete_target_volume;
use {semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfig, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigMutation, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetObjectKindWeights, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetVortexKindWeights, semio_s_artifact_puzzle_3d_editor_config::Puzzle3dConfigSetActiveExampleId};
use semio_s_artifact_puzzle_3d_editor_snapshot::Puzzle3dPlaySnapshot;
use semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation;
use {semio_framework_job::InteractiveJobCloseStep, semio_framework_job::StepContext};
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionWrite;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::object, semio_framework_pack_json::to_string, semio_framework_pack_json::Value};
use std::collections::HashMap;
use std::sync::atomic::Ordering;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use semio_s_artifact_puzzle_3d_model::derive_attraction_params;
use semio_s_artifact_puzzle_3d_model::quat_normalize;
use semio_s_artifact_puzzle_3d_editor_snapshot as snapshot;

#[path = "⚖️kind-weight-work/🦀️.rs"]
mod kind_weight_work;
pub use self::kind_weight_work::*;

#[path = "➕️add-object-kind-work/🦀️.rs"]
mod add_object_kind_work;
pub use self::add_object_kind_work::*;

#[path = "🔍️patch-inspector-work/🦀️.rs"]
mod patch_inspector_work;
pub use self::patch_inspector_work::*;

#[path = "💞️create-attraction-work/🦀️.rs"]
mod create_attraction_work;
pub use self::create_attraction_work::*;

#[path = "📚️set-active-example-work/🦀️.rs"]
mod set_active_example_work;
pub use self::set_active_example_work::*;
