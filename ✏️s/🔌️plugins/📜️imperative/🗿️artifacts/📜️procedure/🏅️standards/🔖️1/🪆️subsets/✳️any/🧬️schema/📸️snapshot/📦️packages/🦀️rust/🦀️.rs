//! 📸️ snapshot of `semio-s-artifact-imperative-procedure` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

pub use crate::schema::snapshot::ProcedureSnapshot;
pub use neural_engine::Dictionary;
pub use imperative_engine::Path;
pub use imperative_engine::Step;
pub use neural_engine::Registry;
pub use neural_engine::Value;
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::editor::procedure::config::io::binary::artifact_items::{PROCEDURE_DIALECT, PROCEDURE_DOCUMENT_SCHEMA, PathRef, ProcedureDiff, ProcedureMutation, artifact_kind, seed_dictionary};
pub use crate::editor::procedure::config::io::binary::artifact_items_2::{PROCEDURE_BODY_EDGE, PROCEDURE_SEQUENCE_EDGE, ProcedureFlowChild, ProcedureTextChild, SemioFlowMutation, flow_content_snapshot_from_path, path_from_flow_content_snapshot, procedure_body_edge_id, procedure_flow_child_handle, procedure_sequence_edge_id, procedure_snapshot_naming, procedure_text_child_handle, seed_from_text_content_snapshot, text_content_snapshot_from_seed};

#[path = "."]
pub mod extensions {
    #[path = "../../../../../../../../../../🧩️extensions/🎮️control/🦀️.rs"]
    pub mod control;
    #[path = "../../../../../../../../../../🧩️extensions/📣️effect/🦀️.rs"]
    pub mod effect;
    #[path = "../../../../../../../../../../🧩️extensions/🧠️logic/🦀️.rs"]
    pub mod logic;
    #[path = "../../../../../../../../../../🧩️extensions/🧮️math/🦀️.rs"]
    pub mod math;
    #[path = "../../../../../../../../../../🧩️extensions/📝️text/🦀️.rs"]
    pub mod text;

#[cfg(test)]
#[path = "../../../../../../../../../../🧩️extensions/🧪️tests/🔬️unit/🦀️.rs"]
mod leaf_tests;
}
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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🔺️diff/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "../../../⚙️operations/🦀️.rs"]
                    pub mod operations;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::diff::*;
    }

}
pub mod snapshot {
    pub mod schema {
        pub use crate::standards::v1::subsets::any::schema::snapshot::*;
    }

}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod procedure {
        #[path = "."]
        pub mod config {
            #[path = "."]
            pub mod io {
                #[path = "../../../../✏️editor/🎚️config/🚪️io/💾️binary/🦀️.rs"]
                pub mod binary;
                #[path = "../../../../✏️editor/🎚️config/🚪️io/📝️text/🦀️.rs"]
                pub mod text;
            }
        }
    }
}
