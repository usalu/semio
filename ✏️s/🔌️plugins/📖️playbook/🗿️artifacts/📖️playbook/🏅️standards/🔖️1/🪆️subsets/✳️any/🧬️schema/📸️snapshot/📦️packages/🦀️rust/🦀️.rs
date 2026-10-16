//! 📸️ snapshot of `semio-s-artifact-playbook-playbook` — a path-preserving carve of the artifact root mounts.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_framework_artifact_playbook_playbook as playbook;
pub use crate::schema::diff::PlaybookDiff;
pub use crate::schema::snapshot::PlaybookSnapshot;
pub use crate::schema::diff::PlaybookStringList;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::mutations::central_apply;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub use crate::standards::v1::subsets::any::schema::mutations::artifact_items::{PLAYBOOK_ARTIFACT_SCHEMA_ID, PLAYBOOK_BUILTIN_KINDS, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA, PlaybookBlock, PlaybookBlockOption, PlaybookExpr, PlaybookMutation, PlaybookSpec, PlaybookStep, PlaybookVectorField, artifact_kind, empty_playbook_snapshot};
pub use crate::standards::v1::subsets::any::schema::mutations::artifact_items_2::{PLAYBOOK_BLOCK_PARAM_PREFIX, PLAYBOOK_DESCRIPTION_PARAM, PLAYBOOK_FLOW_SLOT, PLAYBOOK_GENESIS_FLOW_ID, PLAYBOOK_NEXT_PORT, PLAYBOOK_PREV_PORT, PLAYBOOK_SEQUENCE_EDGE_KIND, PLAYBOOK_STEP_NODE_KIND, PLAYBOOK_STEP_SPACING, PlaybookFlowChild, flow_content_snapshot_from_steps, playbook_block_param_key, playbook_chain, playbook_flow_child, playbook_sequence_edge, playbook_step_from_node, playbook_step_node, playbook_step_order, steps_from_flow_content};

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
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../../../../../../../🌉️apply/🦀️.rs"]
                        pub mod central_apply;
                        #[path = "."]
                        pub mod change_title {
                            #[path = "../../../🧬️mutations/✏️change-title/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🧬️mutations/✏️change-title/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🧬️mutations/✏️change-title/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            #[cfg(test)]
                            #[path = "../../../🧬️mutations/✏️change-title/🧪️tests/🧪️changes/🦀️.rs"]
                            mod tests_changes_the_playbook_title;
                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
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
pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::{PlaybookMutation};

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
    pub mod playbook {
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🚪️io/🦀️.rs"]
            pub mod io;
        }
    }
}
