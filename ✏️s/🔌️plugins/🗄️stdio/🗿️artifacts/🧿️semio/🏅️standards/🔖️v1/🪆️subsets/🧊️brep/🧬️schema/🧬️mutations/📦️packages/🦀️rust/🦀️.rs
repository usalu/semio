//! 🧬️ mut-family of `semio-s-artifact-stdio-semio-brep` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::*;
    }
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod delete_edge {
            #[path = "../../✂️delete-edge/🦀️.rs"]
            mod component;
            #[path = "../../✂️delete-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../✂️delete-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_curve {
            #[path = "../../➰replace-curve/🦀️.rs"]
            mod component;
            #[path = "../../➰replace-curve/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../➰replace-curve/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_shell {
            #[path = "../../🐚create-shell/🦀️.rs"]
            mod component;
            #[path = "../../🐚create-shell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🐚create-shell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_shell {
            #[path = "../../💥delete-shell/🦀️.rs"]
            mod component;
            #[path = "../../💥delete-shell/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../💥delete-shell/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod move_vertex {
            #[path = "../../📍move-vertex/🦀️.rs"]
            mod component;
            #[path = "../../📍move-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../📍move-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_edge {
            #[path = "../../🖇️create-edge/🦀️.rs"]
            mod component;
            #[path = "../../🖇️create-edge/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🖇️create-edge/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod create_face {
            #[path = "../../🔷create-face/🦀️.rs"]
            mod component;
            #[path = "../../🔷create-face/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🔷create-face/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_face {
            #[path = "../../🚮delete-face/🦀️.rs"]
            mod component;
            #[path = "../../🚮delete-face/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../🚮delete-face/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
