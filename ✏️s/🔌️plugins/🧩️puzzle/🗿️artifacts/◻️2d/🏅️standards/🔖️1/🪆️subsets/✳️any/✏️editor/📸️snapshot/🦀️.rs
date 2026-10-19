//! 📸️ Native editor root with typed authority and a derived intrinsic host view.
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
use semio_framework_pack_json::Value as HostValue;

//#region 🔖️PlaySnapshot

#[path = "🌱️play-snapshot-authority/🦀️.rs"]
mod play_snapshot_authority;
pub use play_snapshot_authority::Puzzle2dPlaySnapshot;

#[path = "🔖️constants-examples-and-scene/🦀️.rs"]
mod constants_examples_and_scene;
pub use constants_examples_and_scene::{
    board_snapshot_edges, board_snapshot_nodes, concrete_forest_example_json, empty_board_snapshot,
    kind_catalog_entries, nakagin_example_json, puzzle2d_action, puzzle2d_active_utility,
    puzzle2d_clear_selection_write, puzzle2d_interaction_definition, puzzle2d_interaction_select,
    puzzle2d_load_document_effect, puzzle2d_selection_targets, puzzle2d_selection_write,
    runtime_camera, set_member, Puzzle2dInteractionSnapshot, Puzzle2dScene,
    PUZZLE2D_ADD_NODE_DIALOG_ID, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA, PUZZLE2D_GRANULARITY_EDGE,
    PUZZLE2D_GRANULARITY_HANDLE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_GRANULARITY_TARGET_REGION,
    PUZZLE2D_HOVER_CHANNEL, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_NODE_KIND_OPTIONS_MAX,
    PUZZLE2D_PANES, PUZZLE2D_PLAY_CONTROLLER_ID, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
    PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID, PUZZLE2D_PLAY_SURFACE_ID, PUZZLE2D_PROXIMITY_CONNECT_MAX,
    PUZZLE2D_PROXIMITY_GESTURE_MAX, PUZZLE2D_PROXIMITY_RADIUS_MAX,
};
use constants_examples_and_scene::{BOARD_DEFAULT_HEIGHT, BOARD_DEFAULT_WIDTH};

#[path = "📇️kind-catalogs-and-shipped-kinds/🦀️.rs"]
mod kind_catalogs_and_shipped_kinds;
pub use kind_catalogs_and_shipped_kinds::{
    board_kind_catalogs_json, board_kind_catalogs_json_or_inferred, inferred_kind_entries,
    inferred_node_kind_rows, manifest_board_kind_catalogs_json, puzzle2d_node_kind_arg,
    puzzle2d_node_kind_options, puzzle2d_node_kind_rows, PUZZLE2D_SHIPPED_NODE_KINDS,
};

#[path = "🏷️node-labels-ids-and-placement-geometry/🦀️.rs"]
mod node_labels_ids_and_placement_geometry;
pub use node_labels_ids_and_placement_geometry::{
    new_edge_id, new_node_id, patched_field_value, puzzle2d_absolute_or_delta,
    puzzle2d_addresses_locked_entity, puzzle2d_area_brush_extent_world,
    puzzle2d_clipboard_media_type, puzzle2d_entity_hidden, puzzle2d_entity_label,
    puzzle2d_handle_kind, puzzle2d_handle_world_position, puzzle2d_kind_catalog_label,
    puzzle2d_kind_ids, puzzle2d_kinds_compatible, puzzle2d_next_node_label,
    puzzle2d_node_display_label, puzzle2d_node_reach, puzzle2d_occupied_handles,
    puzzle2d_placed_handles, puzzle2d_push_edge, puzzle2d_region_bounds, puzzle2d_selected_node_ids,
    puzzle2d_selected_target_region_ids, puzzle2d_selection_is_locked,
    puzzle2d_snapshot_regions_admit, puzzle_extension_id, select_same_kind_ids, set_runtime_camera,
    snapshot_target_regions, unique_edge_id, unique_node_id, PUZZLE2D_CLIPBOARD_SCHEMA,
    PUZZLE2D_PASTE_OFFSET, PUZZLE2D_PLACEMENT_MEASURE_MAX,
};

#[path = "🔁️host-sync-and-ui-scopes/🦀️.rs"]
mod host_sync_and_ui_scopes;
pub use host_sync_and_ui_scopes::{
    puzzle2d_select_scope, puzzle2d_window_and_engagements_scope,
    puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, sync_host_fixture_content,
    sync_host_runtime_state,
};

#[path = "🧵️command-variants/🦀️.rs"]
mod command_variants;
pub use command_variants::Puzzle2dCommand;

#[path = "🧰️retained-tool-ids-and-scene-builders/🦀️.rs"]
mod retained_tool_ids_and_scene_builders;
pub use retained_tool_ids_and_scene_builders::{
    puzzle2d_scene_for, puzzle2d_scene_with, PUZZLE2D_GENERIC_TOOL_IDS,
    PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS, PUZZLE2D_RETAINED_TOOL_IDS, PUZZLE2D_TOOL_JOB_IDS,
};

//#endregion 🔖️PlaySnapshot

//#region 🚪️Codecs
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::SqliteDatabase, sqlite_snapshot::SqliteSnapshotControl};
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v1::subsets::any::io::sqlite::snapshot::{semantic_cells, restore_authority};

//#endregion 🚪️Codecs

use crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC;
use crate::editor::puzzle2d::config::Puzzle2dPlayRuntime;
use {crate::editor::puzzle2d::engine::handle_position_on_circle, crate::editor::puzzle2d::engine::handle_position_on_rectangle, crate::editor::puzzle2d::engine::BoardHost, crate::editor::puzzle2d::engine::Point, crate::editor::puzzle2d::engine::Puzzle2dExtension};
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::Effect;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionTarget;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_plugin::app::InteractionView;
use {semio_framework_pack_json::json, semio_framework_pack_json::Value};
#[cfg(test)] use {std::collections::BTreeSet, std::collections::HashSet};
use crate::editor::puzzle2d::snapshot;
use crate::editor::puzzle2d::engine::board_set_grid_visible;

//#region 🧩️EditorContract
/// 🧩️ The puzzle2d editor contract: every editor whose lanes are puzzle2d's play snapshot, mutation union, config,
/// presence and command vocabulary. The action dispatch and the retained works are written against it, so they sit
/// below the one concrete editor that implements it.
pub trait Puzzle2dEditor: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle2dPlaySnapshot, Mutation = Puzzle2dMutation, Config = crate::editor::puzzle2d::config::Puzzle2dConfig, ConfigMutation = crate::editor::puzzle2d::config::Puzzle2dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle2d::presence::Puzzle2dPresence, PresenceMutation = crate::editor::puzzle2d::presence::Puzzle2dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = Puzzle2dCommand> {}

impl<A> Puzzle2dEditor for A where A: semio_framework_plugin::ArtifactEditor<Snapshot = Puzzle2dPlaySnapshot, Mutation = Puzzle2dMutation, Config = crate::editor::puzzle2d::config::Puzzle2dConfig, ConfigMutation = crate::editor::puzzle2d::config::Puzzle2dConfigMutation, Draft = semio_framework_plugin::NoDraft, DraftMutation = semio_framework_plugin::NoDraftMutation, Presence = crate::editor::puzzle2d::presence::Puzzle2dPresence, PresenceMutation = crate::editor::puzzle2d::presence::Puzzle2dPresenceMutation, Transient = semio_framework_plugin::NoTransient, TransientMutation = semio_framework_plugin::NoTransientMutation, Command = Puzzle2dCommand> {}
//#endregion 🧩️EditorContract

impl ColdMutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn absorb(&mut self, other: Self) {
        ColdMutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }

    fn apply(&self, projection: &mut Puzzle2dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        ColdMutationDiff::<Puzzle2dSnapshot>::apply(self, std::sync::Arc::make_mut(&mut projection.typed), capability).map_err(|error| error.under(["document"]))?;
        projection.value = std::sync::OnceLock::new();
        Ok(())
    }
}

impl ColdDiffAlgebra<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dPlaySnapshot) -> Self {
        ColdDiffAlgebra::<Puzzle2dSnapshot>::inverse(self, base.typed())
    }
}

impl protocol::ColdMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type ReferenceDiff = Puzzle2dDiff;
    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        ColdMutation::<Puzzle2dSnapshot>::diff(self, projection.typed())
    }
    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        ColdMutation::<Puzzle2dSnapshot>::inverse(self, projection.typed())?
    
    })
}
}

use protocol::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};

impl ColdMutationDiff<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn absorb(&mut self, other: Self) {
        ColdMutationDiff::<Puzzle2dSnapshot>::absorb(self, other);
    }

    fn apply(&self, projection: &mut Puzzle2dPlaySnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        ColdMutationDiff::<Puzzle2dSnapshot>::apply(self, std::sync::Arc::make_mut(&mut projection.typed), capability).map_err(|error| error.under(["document"]))?;
        projection.value = std::sync::OnceLock::new();
        Ok(())
    }
}

impl ColdDiffAlgebra<Puzzle2dPlaySnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dPlaySnapshot) -> Self {
        ColdDiffAlgebra::<Puzzle2dSnapshot>::inverse(self, base.typed())
    }
}

impl protocol::ColdMutation<Puzzle2dPlaySnapshot> for Puzzle2dMutation {
    type ReferenceDiff = Puzzle2dDiff;
    fn diff(&self, projection: &Puzzle2dPlaySnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        ColdMutation::<Puzzle2dSnapshot>::diff(self, projection.typed())
    }
    fn inverse(&self, projection: &Puzzle2dPlaySnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        ColdMutation::<Puzzle2dSnapshot>::inverse(self, projection.typed())?
    
    })
}
}


