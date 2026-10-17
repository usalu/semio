//! 💡️ inferences of `semio-s-artifact-stdio-semio-drawing` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
        pub mod diff;
        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_drawing_snapshot::io::text::snapshot;
        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
        pub mod diff;
        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
    pub mod sqlite;
}
#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot::*;
        #[path="../../../📸️snapshot/🧩️component/🦀️.rs"]
        pub mod native_binding;
    }
    #[path = "."]
    pub mod diff {
        #[path = "../../../🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod flattened_scene {
            #[path = "../../🎛️flattened-scene/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
    #[path = "."]
    pub mod mutations {
        #[path = "."]
        pub mod unflatten_node {
            #[path = "../../../🧬️mutations/🎈unflatten-node/🦀️.rs"]
            mod component;
            #[path = "../../../🧬️mutations/🎈unflatten-node/🔺️diff/🦀️.rs"]
            pub mod diff;
            #[path = "../../../🧬️mutations/🎈unflatten-node/↩️inverse/🦀️.rs"]
            pub mod inverse;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "../../../../✏️editor/🧭️edit-rules/🦀️.rs"]
    pub mod edit_rules;
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod presence {
        #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
    #[path = "."]
    pub mod config {
        #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
        pub mod schema;
    }
}
