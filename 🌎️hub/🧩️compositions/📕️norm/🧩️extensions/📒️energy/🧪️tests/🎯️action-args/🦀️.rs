//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`. Ticket 26/09/23 slice S19.

include!("../../../../🧪️tests/🧰️laws/🎯️action-args/🦀️.rs");

value_tree_case!(din18599_declares_its_value_tree_arguments, semio_s_artifact_norm_din18599_editor::create_din18599_app);
