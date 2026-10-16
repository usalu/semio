//! 💡️ inferences of `semio-s-artifact-playbook-playbook` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework as semio_framework;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::playbook::PlaybookSpec;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::playbook::PlaybookExpr;
pub use crate::playbook::PlaybookStep;
pub use crate::playbook::PLAYBOOK_DOCUMENT_SCHEMA;
pub use crate::schema::diff::PlaybookDiff;
pub use crate::schema::mutations::PlaybookMutation;
pub use crate::playbook::PLAYBOOK_BUILTIN_KINDS;
use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::playbook::PlaybookBlock;
pub use crate::playbook::PlaybookBlockOption;
pub use crate::playbook::PlaybookVectorField;
pub use crate::schema::diff::PlaybookStringList;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_playbook_playbook_snapshot::{PLAYBOOK_BLOCK_PARAM_PREFIX, PLAYBOOK_DIALECT, PLAYBOOK_FLOW_SLOT, PLAYBOOK_GENESIS_FLOW_ID, PLAYBOOK_STEP_NODE_KIND, artifact_kind, empty_playbook_snapshot, flow_content_snapshot_from_steps, playbook_block_param_key, playbook_chain, playbook_sequence_edge, playbook_step_from_node, playbook_step_node, playbook_step_order, steps_from_flow_content};
pub use crate::editor::playbook::artifact_items_3::{PLAYBOOK_BLOCK_DUPLICATE, PLAYBOOK_BLOCK_MISSING, PLAYBOOK_FLOW_CONTENT, PLAYBOOK_FLOW_DIALECT, PLAYBOOK_FLOW_PROJECTION, PLAYBOOK_FLOW_UNAVAILABLE, PLAYBOOK_STEP_DUPLICATE, PLAYBOOK_STEP_MISSING, genesis_playbook_child_pack, playbook_child_restore_projection, playbook_composed_spec, playbook_fault, playbook_fault_notices, playbook_flow_content, playbook_genesis_flow};
pub use crate::editor::playbook::artifact_items_4::{playbook_add_block_leaves, playbook_add_step_leaves, playbook_block_rows, playbook_chain_leaves, playbook_flow_emit, playbook_insert_block_leaf, playbook_move_block_leaves, playbook_move_step_leaves, playbook_remove_block_leaf, playbook_remove_block_leaves, playbook_remove_step_leaves, playbook_step_node_of, playbook_step_of};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
