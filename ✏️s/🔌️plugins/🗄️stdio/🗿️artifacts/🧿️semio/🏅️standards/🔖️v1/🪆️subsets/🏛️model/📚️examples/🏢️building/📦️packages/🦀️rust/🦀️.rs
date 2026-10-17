//! 🏢️ examples of `semio-s-artifact-stdio-semio-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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
        pub use semio_s_artifact_stdio_semio_model_snapshot::io::text::snapshot;
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
    pub use semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot;
    #[path = "."]
    pub mod diff {
        #[path = "../../../../🧬️schema/🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_model_schema::schema::inferences::*;
        #[path = "."]
        pub mod bounds {
            pub use semio_s_artifact_stdio_semio_model_schema::schema::inferences::bounds::*;
            #[cfg(test)]
            //#region 🧪️Tests
            #[path = "../../../../🧬️schema/💡️inferences/📦bounds/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
        #[cfg(test)]
        //#region 🧪️Tests
        #[path = "../../../../🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod mutations {
        #[path = "../../../../🧬️schema/🧬️mutations/🧰️support/🦀️.rs"]
        pub mod support;
        #[path = "../../../../🧬️schema/🧬️mutations/🎛️set-element/🦀️.rs"]
        pub mod set_element;
    }
}
#[path = "."]
pub mod examples {
    #[path = "../../🦀️.rs"]
    pub mod building;
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
