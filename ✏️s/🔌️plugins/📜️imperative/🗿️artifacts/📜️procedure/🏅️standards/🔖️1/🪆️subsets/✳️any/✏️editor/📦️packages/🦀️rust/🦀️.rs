//! ✏️ editor of `semio-s-artifact-imperative-procedure` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use crate::schema::snapshot::ProcedureSnapshot;
pub use imperative_engine::Step;
pub use imperative_engine::Path;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_imperative_procedure_snapshot::{PROCEDURE_DIALECT, PROCEDURE_DOCUMENT_SCHEMA, artifact_kind, path_from_flow_content_snapshot, seed_from_text_content_snapshot};
pub use semio_s_artifact_imperative_procedure_schema::{ProcedureScene, genesis_procedure_child_pack, procedure_derivable_scene, procedure_scene};
pub use semio_s_artifact_imperative_procedure_inferences::{procedure_child_restore_projection};
pub use crate::editor::procedure::artifact_items_4::{declaration, definition, pilot_languages};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::schema::*;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_imperative_procedure_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_imperative_procedure_inferences::standards::v1::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::text::snapshot;
                        pub use semio_s_artifact_imperative_procedure_schema::standards::v1::subsets::any::io::text::diff;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::binary::snapshot;
                    }
                }
            }
        }
    }
}
pub use semio_s_artifact_imperative_procedure_schema::examples;
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::engine;
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::config::*;
            pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::config::schema;
        }
        pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::terminology;
        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::add_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::add_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::move_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::move_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::remove_step;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::remove_step_at;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::run;
            #[path = "../../🎮️commands/🧬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_contributions;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_step_params;
            pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::commands::set_step_params_at;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::modes::edit::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::modes::edit::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::modes::edit::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/📋️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod script {
                        pub use semio_s_artifact_imperative_procedure_viewer_2::editor::procedure::modes::edit::windows::script::*;
                        #[cfg(test)]
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/📝️script/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
        #[path = "."]
        pub mod panels {
            #[path = "../../📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "../../📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "../../📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;

        #[cfg(test)]
        #[path = "../../📌️panels/🧪️tests/🔬️unit/🦀️.rs"]
        mod leaf_tests;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::script;
                }
            }
        }
    }
}
