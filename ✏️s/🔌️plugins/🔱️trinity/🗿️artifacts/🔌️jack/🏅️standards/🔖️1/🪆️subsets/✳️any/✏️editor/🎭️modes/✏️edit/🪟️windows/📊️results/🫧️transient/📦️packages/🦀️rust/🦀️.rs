//! 🫧️ results-window of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::ast;
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "."]
        pub mod transient {
            #[path = "../../🦀️.rs"]
            pub mod results_window;
        }
    }
}
