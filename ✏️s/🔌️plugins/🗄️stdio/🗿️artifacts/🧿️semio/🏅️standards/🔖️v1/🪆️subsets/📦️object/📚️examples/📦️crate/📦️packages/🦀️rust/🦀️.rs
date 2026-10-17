//! 📦️ examples of `semio-s-artifact-stdio-semio-object` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        pub use semio_s_artifact_stdio_semio_object_snapshot::io::binary::snapshot;
        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
        pub mod diff;
        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
        pub mod inferences;
    }
    #[path = "."]
    pub mod text {
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
    pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot;
    #[path = "."]
    pub mod diff {
        #[path = "../../../../🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "../../../../🧬️schema/💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod composition {
            #[path = "../../../../🧬️schema/💡️inferences/🧩composition/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod crate_ {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
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
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
            #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
            mod component;
            pub use component::*;
            pub use semio_s_artifact_stdio_semio_object_schema::viewer::modes::view::windows;
        }
    }
}
