//! 🎯️ Every norm editor publishes the arguments its value-tree verbs read, typed as the handlers decode them
//! (`app_surface::{path_arg, index_arg, value_arg_json, check_id_arg}`): an agent that follows the published input schema
//! must be able to call `setField`/`insertItem`/`removeItem`/`applyRemedy`. Ticket 26/09/23 slice S19.

include!("../../../../🧪️tests/🧰️laws/🎯️action-args/🦀️.rs");

value_tree_case!(en1990_declares_its_value_tree_arguments, semio_s_artifact_norm_en1990_editor::create_en1990_app);
value_tree_case!(en1991_declares_its_value_tree_arguments, semio_s_artifact_norm_en1991_editor::create_en1991_app);
value_tree_case!(en1997_declares_its_value_tree_arguments, semio_s_artifact_norm_en1997_editor::create_en1997_app);
value_tree_case!(en1998_declares_its_value_tree_arguments, semio_s_artifact_norm_en1998_editor::create_en1998_app);
