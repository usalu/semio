//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`. Ticket 26/09/23 slice S19.

include!("../../../../🧪️tests/🧰️laws/🎯️action-args/🦀️.rs");

value_tree_case!(en1992_declares_its_value_tree_arguments, semio_s_artifact_norm_en1992_editor::create_en1992_app);
value_tree_case!(en1993_declares_its_value_tree_arguments, semio_s_artifact_norm_en1993_editor::create_en1993_app);
value_tree_case!(en1994_declares_its_value_tree_arguments, semio_s_artifact_norm_en1994_editor::create_en1994_app);
