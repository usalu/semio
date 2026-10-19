//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use {semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot, semio_s_artifact_puzzle_3d_mutations::Puzzle3dMutation};
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_value::DslValue;

//#region 🔖️PlaySnapshot

#[path = "🌱️play-snapshot-authority-and-codecs/🦀️.rs"]
mod play_snapshot_authority_and_codecs;
pub use play_snapshot_authority_and_codecs::Puzzle3dPlaySnapshot;

#[path = "🔖️constants-examples-and-scene-types/🦀️.rs"]
mod constants_examples_and_scene_types;
pub use constants_examples_and_scene_types::{
    catalog_entry_label, empty_scene_snapshot, puzzle3d_action, puzzle3d_interaction_select,
    Puzzle3dAttraction, Puzzle3dObject, Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dScene,
    Puzzle3dSceneMetadata, Puzzle3dSceneSnapshot, Puzzle3dTargetVolume, Puzzle3dVortex,
    CONCRETE_FOREST_EXAMPLE_JSON, CONCRETE_FOREST_EXAMPLE_SNAPSHOT, EMPTY_EXAMPLE_SNAPSHOT,
    NAKAGIN_EXAMPLE_JSON, NAKAGIN_EXAMPLE_SNAPSHOT, PUZZLE3D_DEFAULT_UTILITY,
    PUZZLE3D_ENGAGEMENT_VERBS, PUZZLE3D_FALLBACK_MESH_KIND, PUZZLE3D_FILL_TOOL_ID,
    PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_KIND, PUZZLE3D_GRANULARITY_OBJECT,
    PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX,
    PUZZLE3D_HOVER_CHANNEL, PUZZLE3D_ID_COUNTER, PUZZLE3D_INTERACTION_DOMAIN,
    PUZZLE3D_LOCALIZATION_UNSUPPORTED, PUZZLE3D_PLAY_APP_ID, PUZZLE3D_PLAY_CONTROLLER_ID,
    PUZZLE3D_SCENE_SNAPSHOT_SCHEMA, VORTEX_MARKER_MESH_KIND,
};

#[path = "🎬️scene-snapshot-conversion/🦀️.rs"]
mod scene_snapshot_conversion;
pub use scene_snapshot_conversion::{
    default_scene_snapshot, nakagin_scene_snapshot, puzzle3d_copy_fragment,
    puzzle3d_copy_fragment_from, puzzle3d_paste_operations, puzzle3d_projection_value,
    puzzle3d_scene_snapshot_from_document, puzzle3d_scene_snapshot_from_play_snapshot,
    puzzle3d_selected_objects_from, puzzle3d_snapshot_from_host_snapshot, scene_from_snapshot,
    scene_object_from_snapshot,
};

#[path = "🧱️catalog-rows-mesh-index-and-labels/🦀️.rs"]
mod catalog_rows_mesh_index_and_labels;
pub use catalog_rows_mesh_index_and_labels::{
    collect_mesh_urls, mesh_selection_ids, next_object_id, object_scale_json,
    puzzle3d_catalog_entries, puzzle3d_entity_label, puzzle3d_fault_notices,
    puzzle3d_flag_value_action, puzzle3d_flag_value_argument, puzzle3d_kind_catalog_label,
    puzzle3d_kind_ids, puzzle3d_next_object_label, puzzle3d_object_display_label,
    puzzle3d_upsert_catalog_rows, puzzle3d_vortices_from_kind_template, resolve_object_mesh_url,
    resolve_vortex_world_position, target_volume_scale_json, world_vortex_position,
    Puzzle3dKindMeshIndex,
};

#[path = "🪟️windows-utilities-and-interaction/🦀️.rs"]
mod windows_utilities_and_interaction;
pub use windows_utilities_and_interaction::{
    apply_puzzle3d_focus_selection, puzzle3d_absolute_or_delta, puzzle3d_addressed_window_id,
    puzzle3d_axis_index, puzzle3d_brush_target_vortex, puzzle3d_fill_tool_active,
    puzzle3d_inspector_origin_nudge, puzzle3d_resolve_number_edit, puzzle3d_scene_active_utility,
    puzzle3d_window_id_is_kind, value_as_vec3, Puzzle3dInteractionSnapshot,
};

#[path = "⚖️joint-weights-and-distribution/🦀️.rs"]
mod joint_weights_and_distribution;
pub use joint_weights_and_distribution::{
    puzzle3d_apply_distribution_weight, puzzle3d_distribution_children, puzzle3d_distribution_group,
    puzzle3d_ensure_catalog_kind_weights, puzzle3d_ensure_joint_distribution,
    puzzle3d_global_joint_total, puzzle3d_joint_vortex_measures, puzzle3d_joint_vortex_weight,
    puzzle3d_joint_weight_key, puzzle3d_joint_weights_use_composite_keys,
    puzzle3d_normalize_kind_weight_group, puzzle3d_object_joint_total, puzzle3d_read_joint_weight,
    puzzle3d_set_object_kind_distribution_weight, puzzle3d_set_vortex_joint_weight,
    puzzle3d_sync_object_kind_weights_from_joints, puzzle3d_uniform_kind_weights,
    PUZZLE3D_JOINT_WEIGHT_SEP,
};

#[path = "🧵️command-variants/🦀️.rs"]
mod command_variants;
pub use command_variants::Puzzle3dCommand;

//#endregion 🔖️PlaySnapshot

use semio_s_artifact_puzzle_3d_editor_config::Puzzle3dRuntime;
use semio_s_artifact_puzzle_3d_editor_terminology::Puzzle3dLabels;
use {semio_framework_plugin::kernel::ClipboardError, semio_framework_plugin::kernel::ClipboardFragment, semio_framework_plugin::kernel::PastePlacement};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::InteractionTarget;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::SET_ACTIVE_TOOL_ACTION_ID;
use semio_framework_value::FromValue;
use {semio_framework_pack_json::from_json_str, semio_framework_pack_json::json, semio_framework_pack_json::object, semio_framework_pack_json::parse, semio_framework_pack_json::to_json_string, semio_framework_pack_json::to_string, semio_framework_pack_json::Object, semio_framework_pack_json::Value};
use semio_framework_plugin::app::InteractionView;
use {std::collections::HashMap, std::collections::HashSet};
use {std::sync::atomic::AtomicU32, std::sync::atomic::Ordering};
use std::sync::LazyLock;
use semio_s_artifact_puzzle_3d_model::puzzle3d_vortex_full_id;
use crate as snapshot;

//#region 🚪️Codecs
/// 🚪️ The play snapshot shares the typed authority's text, pack, store and SQLite representations: every codec reads and
/// writes through `Puzzle3dSnapshot`.
use store::sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteSnapshotControl};
use store::os_store::serialization::ArtifactSqliteSnapshotHost;
use semio_framework_value::ValueError;
use pack::snapshot::ArtifactSqliteSnapshot as _;

//#endregion 🚪️Codecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️play-pack/🦀️.rs"]
mod play_pack_tests;
//#endregion 🧪️Tests

impl ColdMutationDiff<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn absorb(&mut self, other: Self) {
        ColdMutationDiff::<Puzzle3dSnapshot>::absorb(self, other);
    }

    fn apply(&self, projection: &mut Puzzle3dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        ColdMutationDiff::<Puzzle3dSnapshot>::apply(self, std::sync::Arc::make_mut(&mut projection.typed), capability).map_err(|error| error.under(["document"]))?;
        projection.value = std::sync::OnceLock::new();
        Ok(())
    }
}

impl ColdDiffAlgebra<Puzzle3dPlaySnapshot> for Puzzle3dDiff {
    fn inverse(&self, base: &Puzzle3dPlaySnapshot) -> Self {
        ColdDiffAlgebra::<Puzzle3dSnapshot>::inverse(self, base.typed())
    }
}

impl protocol::ColdMutation<Puzzle3dPlaySnapshot> for Puzzle3dMutation {
    type ReferenceDiff = Puzzle3dDiff;
    fn diff(&self, projection: &Puzzle3dPlaySnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
        ColdMutation::<Puzzle3dSnapshot>::diff(self, projection.typed.as_ref())
    }
    fn inverse(&self, projection: &Puzzle3dPlaySnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
        ColdMutation::<Puzzle3dSnapshot>::inverse(self, projection.typed.as_ref())?
    
    })
}
}

use protocol::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
