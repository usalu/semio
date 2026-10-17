//! 🧬️ schema of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::io::binary::mutations;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::manifest;
                    pub use semio_s_artifact_trinity_jack_model::standards::v1::subsets::any::schema::graph;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::content;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::composition;
                    #[path = "../../../../../../../🔨️modules/🏠️host/🦀️.rs"]
                    pub mod host;
                    pub use semio_s_artifact_trinity_jack_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::*;
                        #[cfg(test)]
                        #[path = "../../🧬️mutations/🧪️tests/🔬️structural-correspondence/🦀️.rs"]
                        pub mod structural_correspondence_tests;
                        #[path = "."]
                        pub mod set_query {
                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::*;
                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::diff;
                            pub use semio_s_artifact_trinity_jack_language_service::standards::v1::subsets::any::schema::mutations::set_query::inverse;
                        }
                    }
                }
            }
        }
    }
}
