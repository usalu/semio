//! 🪆️ The `drawing` subset of the semio artifact: its schema, io codecs, editor and viewer, and the retirement and member-owner
//! implementations of its own types.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "🚪️io/🦀️.rs"]
    mod component;
    pub use component::*;
}
#[path = "."]
pub mod schema {
    #[path = "🧬️schema/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod geometry {
        #[path = "🧬️schema/📐️geometry/🦀️.rs"]
        mod component;
        pub use component::*;
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
        pub mod flattened_scene {
            #[path = "🧬️schema/💡️inferences/🎛️flattened-scene/🦀️.rs"]
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
            #[path = "🧬️schema/🧬️mutations/➕create-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➕create-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➕create-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_node {
            #[path = "🧬️schema/🧬️mutations/➖delete-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/➖delete-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/➖delete-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_layer {
            #[path = "🧬️schema/🧬️mutations/🌱create-layer/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🌱create-layer/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🌱create-layer/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod unflatten_node {
            #[path = "🧬️schema/🧬️mutations/🎈unflatten-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🎈unflatten-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🎈unflatten-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod ungroup_node {
            #[path = "🧬️schema/🧬️mutations/💫ungroup-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/💫ungroup-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/💫ungroup-node/↩️inverse/🦀️.rs"]
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
        pub mod scale_node {
            #[path = "🧬️schema/🧬️mutations/📏scale-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📏scale-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📏scale-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_stroke_width {
            #[path = "🧬️schema/🧬️mutations/📐change-stroke-width/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/📐change-stroke-width/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/📐change-stroke-width/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod reorder_nodes {
            #[path = "🧬️schema/🧬️mutations/🔀reorder-nodes/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔀reorder-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rotate_node {
            #[path = "🧬️schema/🧬️mutations/🔄rotate-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🔄rotate-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🔄rotate-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod change_stroke_color {
            #[path = "🧬️schema/🧬️mutations/🖌️change-stroke/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖌️change-stroke/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖌️change-stroke/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod drag_nodes {
            #[path = "🧬️schema/🧬️mutations/🖐️drag-nodes/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🖐️drag-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🖐️drag-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_layer {
            #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🗑️delete-layer/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_path {
            #[path = "🧬️schema/🧬️mutations/🛤️replace-path/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🛤️replace-path/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🛤️replace-path/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod group_nodes {
            #[path = "🧬️schema/🧬️mutations/🧷group-nodes/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🧷group-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🧷group-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_fill {
            #[path = "🧬️schema/🧬️mutations/🪣replace-fill/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🪣replace-fill/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🪣replace-fill/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod flatten_node {
            #[path = "🧬️schema/🧬️mutations/🫓flatten-node/🦀️.rs"]
            mod component;
            #[path = "🧬️schema/🧬️mutations/🫓flatten-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "🧬️schema/🧬️mutations/🫓flatten-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
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

semio_framework_value::artifact_retire_leaf!(schema::snapshot::DrawCanvas);
semio_framework_value::artifact_retire_struct!(schema::snapshot::DrawStyle { name, fill, stroke, stroke_width, opacity });
semio_framework_value::artifact_retire_struct!(schema::snapshot::DrawLayer { id, name, visible, root });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioDrawingSnapshot { schema, canvas, styles, layers });
semio_framework_value::artifact_retire_struct!(schema::diff::NodePath { layer, path });

semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioDrawingSnapshot);
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioDrawingSnapshot, schema::mutations::SemioDrawingMutation);
