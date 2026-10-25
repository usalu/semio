//! 👁️ viewer of `semio-s-artifact-stdio-semio-brep` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_brep_snapshot::io::text::snapshot;
        #[path = "../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
        pub use semio_s_artifact_stdio_semio_brep_examples::io::text::diff;
    }
}
#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::*;
        pub use semio_s_artifact_stdio_semio_brep_schema::schema::snapshot::body;
    }
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::diff;
    pub use semio_s_artifact_stdio_semio_brep_examples::schema::inferences;
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️schema/🧬️mutations/🦀️.rs"]
        mod component;
        pub use component::*;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_edge;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::replace_curve;
        #[path = "."]
        pub mod create_vertex {
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-vertex/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🏗️create-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_shell;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_shell;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::move_vertex;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_edge;
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::create_face;
        #[path = "."]
        pub mod delete_solid {
            #[path = "../../../🧬️schema/🧬️mutations/🕳️delete-solid/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🕳️delete-solid/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🕳️delete-solid/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod delete_vertex {
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-vertex/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-vertex/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🗑️delete-vertex/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        #[path = "."]
        pub mod replace_surface {
            #[path = "../../../🧬️schema/🧬️mutations/🗺️replace-surface/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🗺️replace-surface/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🗺️replace-surface/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
        pub use semio_s_artifact_stdio_semio_brep_mut_family::schema::mutations::delete_face;
        #[path = "."]
        pub mod create_solid {
            #[path = "../../../🧬️schema/🧬️mutations/🧊create-solid/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️schema/🧬️mutations/🧊create-solid/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️schema/🧬️mutations/🧊create-solid/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "../../../✏️editor/🧭️edit-rules/🦀️.rs"]
    pub mod edit_rules;
}
#[path = "."]
pub mod viewer {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioBrepSnapshot, schema::mutations::SemioBrepMutation);
