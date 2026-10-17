//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-mesh` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_mesh_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod create_mesh {
            #[path = "../../🕸️create-mesh/🦀️.rs"]
            mod component;
            #[path = "../../🕸️create-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🕸️create-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_mesh {
            #[path = "../../🗑️delete-mesh/🦀️.rs"]
            mod component;
            #[path = "../../🗑️delete-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🗑️delete-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_primitive {
            #[path = "../../🔺create-primitive/🦀️.rs"]
            mod component;
            #[path = "../../🔺create-primitive/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔺create-primitive/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_primitive {
            #[path = "../../✂️delete-primitive/🦀️.rs"]
            mod component;
            #[path = "../../✂️delete-primitive/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../✂️delete-primitive/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_primitive_topology {
            #[path = "../../🔀set-primitive-topology/🦀️.rs"]
            mod component;
            #[path = "../../🔀set-primitive-topology/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔀set-primitive-topology/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_primitive_geometry {
            #[path = "../../📐replace-primitive-geometry/🦀️.rs"]
            mod component;
            #[path = "../../📐replace-primitive-geometry/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📐replace-primitive-geometry/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod set_primitive_material {
            #[path = "../../🧲️set-primitive-material/🦀️.rs"]
            mod component;
            #[path = "../../🧲️set-primitive-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🧲️set-primitive-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_material {
            #[path = "../../🎨create-material/🦀️.rs"]
            mod component;
            #[path = "../../🎨create-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🎨create-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_material {
            #[path = "../../🚮delete-material/🦀️.rs"]
            mod component;
            #[path = "../../🚮delete-material/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🚮delete-material/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
