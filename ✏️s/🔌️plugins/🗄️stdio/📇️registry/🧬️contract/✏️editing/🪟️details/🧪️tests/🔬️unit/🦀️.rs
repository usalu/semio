use super::*;
use semio_framework_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_PATH_SEPARATOR};
use semio_s_artifact_stdio_contract_editing::{snapshot_edit_event_from_action, SnapshotEditEvent, REPLACE_SNAPSHOT_SOURCE_ACTION_ID, SET_SNAPSHOT_VALUE_ACTION_ID, SNAPSHOT_EDIT_ACTION_IDS};
use semio_s_artifact_stdio_contract_wire::snapshot_edit_source;

include!("🧱️lazy-and-bounded-laws/🦀️.rs");

include!("🌳️collection-and-table-laws/🦀️.rs");

include!("🧬️schema-template-laws/🦀️.rs");

include!("🔀️union-source-and-window-laws/🦀️.rs");
