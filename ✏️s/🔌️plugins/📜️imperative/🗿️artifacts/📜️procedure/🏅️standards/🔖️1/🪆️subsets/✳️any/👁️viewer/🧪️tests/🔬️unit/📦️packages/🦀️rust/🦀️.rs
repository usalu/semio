//! 🔬️ viewer-2 of `semio-s-artifact-imperative-procedure` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use neural_engine::Dictionary;
pub use crate::schema::snapshot::ProcedureSnapshot;
pub use imperative_engine::Step;
pub use neural_engine::Registry;
pub use neural_engine::Value;
pub use imperative_engine::Path;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use semio_s_artifact_imperative_procedure_snapshot::{PROCEDURE_DIALECT, PathRef};
pub use semio_s_artifact_imperative_procedure_schema::{ProcedureScene, procedure_edit_emit, procedure_scene};

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
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::operations;
                    pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_imperative_procedure_snapshot::standards::v1::subsets::any::io::text::snapshot;
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
        #[path = "."]
        pub mod engine {
            pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::engine::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/⚙️engine/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
        #[path = "."]
        pub mod config {
            pub use semio_s_artifact_imperative_procedure_inferences::editor::procedure::config::*;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
            #[cfg(test)]
            #[path = "../../../../../✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs"]
            mod contract_vectors;
        }
        #[path = "."]
        pub mod commands {
            #[path = "../../../../../✏️editor/🎮️commands/➕️add-step/🦀️.rs"]
            pub mod add_step;
            #[path = "../../../../../✏️editor/🎮️commands/📍️add-step-at/🦀️.rs"]
            pub mod add_step_at;
            #[path = "../../../../../✏️editor/🎮️commands/🚚️move-step/🦀️.rs"]
            pub mod move_step;
            #[path = "../../../../../✏️editor/🎮️commands/🧭️move-step-at/🦀️.rs"]
            pub mod move_step_at;
            #[path = "../../../../../✏️editor/🎮️commands/➖️remove-step/🦀️.rs"]
            pub mod remove_step;
            #[path = "../../../../../✏️editor/🎮️commands/✂️remove-step-at/🦀️.rs"]
            pub mod remove_step_at;
            #[path = "../../../../../✏️editor/🎮️commands/🏃️run/🦀️.rs"]
            pub mod run;
            #[path = "../../../../../✏️editor/🎮️commands/🧩️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "../../../../../✏️editor/🎮️commands/🎚️set-step-params/🦀️.rs"]
            pub mod set_step_params;
            #[path = "../../../../../✏️editor/🎮️commands/🎯️set-step-params-at/🦀️.rs"]
            pub mod set_step_params_at;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_viewer::editor::procedure::modes::edit::windows::main;
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/📝️script/🦀️.rs"]
                    pub mod script;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod procedure {
        pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::*;
        #[cfg(test)]
        #[path = "../../🦀️.rs"]
        mod tests;
        #[path = "."]
        pub mod modes {
            pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::*;
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::*;
                #[path = "."]
                pub mod windows {
                    pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::*;
                    #[path = "."]
                    pub mod main {
                        pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::main::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/👁️view/🪟️windows/📋️main/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                    #[path = "."]
                    pub mod script {
                        pub use semio_s_artifact_imperative_procedure_inferences::viewer::procedure::modes::view::windows::script::*;
                        #[cfg(test)]
                        #[path = "../../../../🎭️modes/👁️view/🪟️windows/📝️script/🧪️tests/🔬️unit/🦀️.rs"]
                        mod tests;
                    }
                }
            }
        }
    }
}
