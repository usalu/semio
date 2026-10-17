//! 🔺️ diff of `semio-s-artifact-stdio-semio-image` — a path-preserving carve of the artifact root mounts.

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
        pub use semio_s_artifact_stdio_semio_image_snapshot::io::text::snapshot;
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
    pub use semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot;
    #[path = "."]
    pub mod diff {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_image_schema::schema::inferences::*;
        #[path = "."]
        pub mod dimensions {
            pub use semio_s_artifact_stdio_semio_image_schema::schema::inferences::dimensions::*;
            #[cfg(test)]
            //#region 🧪️Tests
            #[path = "../../../💡️inferences/📐dimensions/🧪️tests/🔬️unit/🦀️.rs"]
            mod tests;
        }
        #[cfg(test)]
        //#region 🧪️Tests
        #[path = "../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        mod tests;
    }
    #[path = "."]
    pub mod mutations {
        #[path = "../../../🧬️mutations/➕️insert-frame/🦀️.rs"]
        pub mod insert_frame;
        #[path = "../../../🧬️mutations/🚫️remove-frame/🦀️.rs"]
        pub mod remove_frame;
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
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
