//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`/`setSnapshot`. Ticket 26/09/23 slice S19.

include!("../../../../🧪️tests/🧰️laws/🎯️action-args/🦀️.rs");

value_tree_case!(en1995_declares_its_value_tree_arguments, semio_s_artifact_norm_en1995_editor::create_en1995_app);
value_tree_case!(en1996_declares_its_value_tree_arguments, semio_s_artifact_norm_en1996_editor::create_en1996_app);
value_tree_case!(en1999_declares_its_value_tree_arguments, semio_s_artifact_norm_en1999_editor::create_en1999_app);
