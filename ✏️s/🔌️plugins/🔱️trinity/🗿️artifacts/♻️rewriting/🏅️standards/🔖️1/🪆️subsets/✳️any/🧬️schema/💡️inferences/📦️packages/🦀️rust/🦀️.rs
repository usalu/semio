//! 💡️ inferences of `semio-s-artifact-trinity-rewriting` — a path-preserving carve of the artifact root mounts.

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
                    pub use semio_s_artifact_trinity_rewriting_layout::standards::v1::subsets::any::schema::layout;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::errors;
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::model;
                    #[path = "../../../../../../../../🔨️modules/▶️rule-application/🦀️.rs"]
                    pub mod application;
                    #[cfg(any(test, feature = "testkit"))]
                    #[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
                    pub mod test_support;
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../📸️snapshot/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "."]
                        pub mod bounds {
                            #[path = "../../📦bounds/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                    }
                    pub use semio_s_artifact_trinity_rewriting_model::standards::v1::subsets::any::schema::rule;
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
