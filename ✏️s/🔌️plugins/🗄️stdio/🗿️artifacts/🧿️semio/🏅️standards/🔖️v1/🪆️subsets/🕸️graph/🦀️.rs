//! 🪆️ The `graph` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_graph as graph_core;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧬️schema/🔧️operators-internals/🦀️.rs"]
    pub mod operators_internals;
    #[path = "🧬️schema/🚶️traversal-internals/🦀️.rs"]
    pub mod traversal_internals;
    #[path = "."]
    pub mod normal_internals {
        #[path = "🧬️schema/➕️normal-internals/➡️directed/🦀️.rs"]
        pub mod directed;
        #[path = "🧬️schema/➕️normal-internals/↔️undirected/🦀️.rs"]
        pub mod undirected;
    }
    #[path = "."]
    pub mod ports_internals {
        #[path = "🧬️schema/🔌️ports-internals/↔️undirected/🦀️.rs"]
        pub mod undirected;
        #[path = "."]
        pub mod directed {
            #[path = "🧬️schema/🔌️ports-internals/➡️directed/➕️normal/🦀️.rs"]
            pub mod normal;
        }
    }
    #[path = "."]
    pub mod snapshot {
        #[path = "🧬️schema/📸️snapshot/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod diff {
        #[path = "🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod topology {
            #[path = "🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod connectivity {
            #[path = "🧬️schema/💡️inferences/🕸️connectivity/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod create_node {
            #[path = "🧬️schema/🧬️mutations/🏗️create-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏗️create-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏗️create-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_node {
            #[path = "🧬️schema/🧬️mutations/🗑️delete-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_node_kind {
            #[path = "🧬️schema/🧬️mutations/🔧change-node-kind/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔧change-node-kind/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔧change-node-kind/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_node_label {
            #[path = "🧬️schema/🧬️mutations/🖍️change-node-label/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖍️change-node-label/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖍️change-node-label/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_node {
            #[path = "🧬️schema/🧬️mutations/📍move-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📍move-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📍move-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_node_port {
            #[path = "🧬️schema/🧬️mutations/🔌add-node-port/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔌add-node-port/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔌add-node-port/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_node_port {
            #[path = "🧬️schema/🧬️mutations/🔚remove-node-port/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔚remove-node-port/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔚remove-node-port/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_node_property {
            #[path = "🧬️schema/🧬️mutations/➕add-node-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➕add-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➕add-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_node_property {
            #[path = "🧬️schema/🧬️mutations/➖remove-node-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖remove-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖remove-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_edge {
            #[path = "🧬️schema/🧬️mutations/🌉️create-edge/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🌉️create-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🌉️create-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_edge {
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✂️delete-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod drag_nodes {
            #[path = "🧬️schema/🧬️mutations/✋️drag-nodes/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/✋️drag-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/✋️drag-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_node_property {
            #[path = "🧬️schema/🧬️mutations/🎛️set-node-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🎛️set-node-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🎛️set-node-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod resize_node {
            #[path = "🧬️schema/🧬️mutations/📐resize-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📐resize-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📐resize-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rename_node {
            #[path = "🧬️schema/🧬️mutations/🏷️rename-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🏷️rename-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🏷️rename-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_edge_property {
            #[path = "🧬️schema/🧬️mutations/🎚️set-edge-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🎚️set-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🎚️set-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod add_edge_property {
            #[path = "🧬️schema/🧬️mutations/➕add-edge-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➕add-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➕add-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod remove_edge_property {
            #[path = "🧬️schema/🧬️mutations/➖remove-edge-property/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖remove-edge-property/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖remove-edge-property/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod wires {
        #[path = "📚️examples/🕸️wires/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "✏️editor/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "👁️viewer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod presence {
        #[path = "👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}

semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioGraphPortKind);
semio_framework_value::artifact_retire_struct!(schema::snapshot::GraphNodeId { value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::GraphEdgeId { value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphPort { name, kind, category, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphNode { id, kind, label, position, width, height, ports, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphEdge { id, source, target, kind, label, source_port, target_port, properties });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioGraphSnapshot { schema, nodes, edges });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioGraphSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioGraphSnapshot, schema::mutations::SemioGraphMutation);
