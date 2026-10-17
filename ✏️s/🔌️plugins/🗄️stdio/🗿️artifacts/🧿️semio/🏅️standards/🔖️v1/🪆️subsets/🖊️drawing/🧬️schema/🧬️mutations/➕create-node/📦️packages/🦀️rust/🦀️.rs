//! ➕ mut-family of `semio-s-artifact-stdio-semio-drawing` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot::*;
        #[path = "."]
        pub mod native_binding {
            #[path="../../../../📸️snapshot/🧩️component/🌳️nodes/🦀️.rs"]
            pub mod nodes;
        }
    }
    pub use semio_s_artifact_stdio_semio_drawing_inferences::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod create_node {
            #[path = "../../🦀️.rs"]
            mod component;
            #[path = "../../🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_node {
            #[path = "../../../➖delete-node/🦀️.rs"]
            mod component;
            #[path = "../../../➖delete-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../➖delete-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_layer {
            #[path = "../../../🌱create-layer/🦀️.rs"]
            mod component;
            #[path = "../../../🌱create-layer/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🌱create-layer/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod ungroup_node {
            #[path = "../../../💫ungroup-node/🦀️.rs"]
            mod component;
            #[path = "../../../💫ungroup-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../💫ungroup-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_node {
            #[path = "../../../📍move-node/🦀️.rs"]
            mod component;
            #[path = "../../../📍move-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../📍move-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod scale_node {
            #[path = "../../../📏scale-node/🦀️.rs"]
            mod component;
            #[path = "../../../📏scale-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../📏scale-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_layer {
            #[path = "../../../🗑️delete-layer/🦀️.rs"]
            mod component;
            #[path = "../../../🗑️delete-layer/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🗑️delete-layer/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod group_nodes {
            #[path = "../../../🧷group-nodes/🦀️.rs"]
            mod component;
            #[path = "../../../🧷group-nodes/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧷group-nodes/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
