//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-object` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_object_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod move_object {
            #[path = "../../🚚move-object/🦀️.rs"]
            mod component;
            #[path = "../../🚚move-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🚚move-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod rotate_object {
            #[path = "../../🔄rotate-object/🦀️.rs"]
            mod component;
            #[path = "../../🔄rotate-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔄rotate-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod scale_object {
            #[path = "../../📏scale-object/🦀️.rs"]
            mod component;
            #[path = "../../📏scale-object/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📏scale-object/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_brep {
            #[path = "../../🧱create-brep/🦀️.rs"]
            mod component;
            #[path = "../../🧱create-brep/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🧱create-brep/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_brep {
            #[path = "../../💥delete-brep/🦀️.rs"]
            mod component;
            #[path = "../../💥delete-brep/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../💥delete-brep/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
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
            #[path = "../../🧨delete-mesh/🦀️.rs"]
            mod component;
            #[path = "../../🧨delete-mesh/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🧨delete-mesh/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_properties {
            #[path = "../../🏷️create-properties/🦀️.rs"]
            mod component;
            #[path = "../../🏷️create-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🏷️create-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_properties {
            #[path = "../../🚫delete-properties/🦀️.rs"]
            mod component;
            #[path = "../../🚫delete-properties/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🚫delete-properties/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
