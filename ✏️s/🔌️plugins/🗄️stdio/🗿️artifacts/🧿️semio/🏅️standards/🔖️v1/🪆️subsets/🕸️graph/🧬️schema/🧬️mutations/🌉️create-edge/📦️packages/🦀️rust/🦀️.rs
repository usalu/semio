//! 🌉️ mut-family of `semio-s-artifact-stdio-semio-graph` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_graph_snapshot::schema::diff;
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_graph_inferences::schema::inferences::*;
        #[path = "."]
        pub mod connectivity {
            pub use semio_s_artifact_stdio_semio_graph_inferences::schema::inferences::connectivity::*;
            #[cfg(test)]
            //#region 🧪️Tests
            #[path = "../../../../💡️inferences/🕸️connectivity/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod create_edge {
            #[path = "../../🦀️.rs"]
            mod component;
            #[path = "../../🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_edge {
            #[path = "../../../✂️delete-edge/🦀️.rs"]
            mod component;
            #[path = "../../../✂️delete-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../✂️delete-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_node_property {
            #[path = "../../../🎛️set-node-property/🦀️.rs"]
            mod component;
            #[path = "../../../🎛️set-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🎛️set-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod resize_node {
            #[path = "../../../📐resize-node/🦀️.rs"]
            mod component;
            #[path = "../../../📐resize-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../📐resize-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_node {
            #[path = "../../../🏷️rename-node/🦀️.rs"]
            mod component;
            #[path = "../../../🏷️rename-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🏷️rename-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_edge_property {
            #[path = "../../../🎚️set-edge-property/🦀️.rs"]
            mod component;
            #[path = "../../../🎚️set-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🎚️set-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_edge_property {
            #[path = "../../../➕add-edge-property/🦀️.rs"]
            mod component;
            #[path = "../../../➕add-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../➕add-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_edge_property {
            #[path = "../../../➖remove-edge-property/🦀️.rs"]
            mod component;
            #[path = "../../../➖remove-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../➖remove-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
