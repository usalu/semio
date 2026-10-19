//! 🏪️ Puzzle3d document-store laws — command envelopes, granular replay and the selection leaves' time travel, each over a real store built by the host authority.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_mutations::*;
use semio_s_artifact_puzzle_3d_diff::Puzzle3dDiff;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;
use protocol::DiffAlgebra;
use protocol::Mutation;
use semio_s_artifact_puzzle_3d_mutations_vortex::add_object_vortex::mutation::AddObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_catalog::change_domain::mutation::ChangeDomain;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_anchor::mutation::ChangeObjectAnchor;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_hidden::mutation::ChangeObjectHidden;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_kind::mutation::ChangeObjectKind;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_locked::mutation::ChangeObjectLocked;
use semio_s_artifact_puzzle_3d_mutations_object::change_object_mesh::mutation::ChangeObjectMesh;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_hidden::mutation::ChangeReferenceHidden;
use semio_s_artifact_puzzle_3d_mutations_reference::change_reference_locked::mutation::ChangeReferenceLocked;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_hidden::mutation::ChangeTargetVolumeHidden;
use semio_s_artifact_puzzle_3d_mutations_volume::change_target_volume_locked::mutation::ChangeTargetVolumeLocked;
use semio_s_artifact_puzzle_3d_mutations_catalog::connect_kind_compatibility::mutation::ConnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::connect_vortices::mutation::ConnectVortices;
use semio_s_artifact_puzzle_3d_mutations_object::create_object::mutation::CreateObject;
use semio_s_artifact_puzzle_3d_mutations_reference::create_reference::mutation::CreateReference;
use semio_s_artifact_puzzle_3d_mutations_volume::create_target_volume::mutation::CreateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_object::delete_object::mutation::DeleteObject;
use semio_s_artifact_puzzle_3d_mutations_reference::delete_reference::mutation::DeleteReference;
use semio_s_artifact_puzzle_3d_mutations_volume::delete_target_volume::mutation::DeleteTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_catalog::disconnect_kind_compatibility::mutation::DisconnectKindCompatibility;
use semio_s_artifact_puzzle_3d_mutations_vortex::disconnect_vortices::mutation::DisconnectVortices;
use semio_s_artifact_puzzle_3d_mutations_selection::drag_selection::mutation::DragSelection;
use semio_s_artifact_puzzle_3d_mutations_object::edit_object_label::mutation::EditObjectLabel;
use semio_s_artifact_puzzle_3d_mutations_pose::move_object::mutation::MoveObject;
use semio_s_artifact_puzzle_3d_mutations_reference::move_reference::mutation::MoveReference;
use semio_s_artifact_puzzle_3d_mutations_volume::move_target_volume::mutation::MoveTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_vortex::remove_object_vortex::mutation::RemoveObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_attraction_geometry::mutation::ReplaceAttractionGeometry;
use semio_s_artifact_puzzle_3d_mutations_catalog::replace_kind_catalogs::mutation::ReplaceKindCatalogs;
use semio_s_artifact_puzzle_3d_mutations_vortex::replace_object_vortex::mutation::ReplaceObjectVortex;
use semio_s_artifact_puzzle_3d_mutations_reference::replace_reference_source::mutation::ReplaceReferenceSource;
use semio_s_artifact_puzzle_3d_mutations_reference::resize_reference::mutation::ResizeReference;
use semio_s_artifact_puzzle_3d_mutations_pose::rotate_object::mutation::RotateObject;
use semio_s_artifact_puzzle_3d_mutations_selection::rotate_selection::mutation::RotateSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::rotate_target_volume::mutation::RotateTargetVolume;
use semio_s_artifact_puzzle_3d_mutations_pose::scale_object::mutation::ScaleObject;
use semio_s_artifact_puzzle_3d_mutations_selection::scale_selection::mutation::ScaleSelection;
use semio_s_artifact_puzzle_3d_mutations_volume::scale_target_volume::mutation::ScaleTargetVolume;

#[path = "🔬️envelope/🦀️.rs"]
mod envelope;
#[path = "🔬️replay/🦀️.rs"]
mod replay;
#[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️selection-time-travel/🦀️.rs"]
mod selection_time_travel;
