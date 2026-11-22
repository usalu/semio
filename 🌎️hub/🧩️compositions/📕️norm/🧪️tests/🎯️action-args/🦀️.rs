//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`. Ticket 26/09/23 slice S19.

include!("../🧰️laws/🎯️action-args/🦀️.rs");

value_tree_case!(din4108_declares_its_value_tree_arguments, semio_s_artifact_norm_din4108_editor::create_din4108_app);
value_tree_case!(din16798_declares_its_value_tree_arguments, semio_s_artifact_norm_din16798_editor::create_din16798_app);
value_tree_case!(iso16757_declares_its_value_tree_arguments, semio_s_artifact_norm_iso16757_editor::create_iso16757_app);
value_tree_case!(vdi3805_declares_its_value_tree_arguments, semio_s_artifact_norm_vdi3805_editor::create_vdi3805_app);
