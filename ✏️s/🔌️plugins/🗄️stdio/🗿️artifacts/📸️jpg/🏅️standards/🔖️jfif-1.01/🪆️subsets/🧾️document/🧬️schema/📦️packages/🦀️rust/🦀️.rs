//! 🧬️ schema of `semio-s-artifact-stdio-jpg` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

pub mod schema {
    pub use super::standards::v_jfif_1_01::subsets::document::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v_jfif_1_01 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    #[path = "../../🦀️.rs"]
                    mod component;
                    pub use component::*;
                    pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::document::schema::snapshot;
                }
            }
            #[path = "."]
            pub mod baseline {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod conformance {
                        pub use semio_s_artifact_stdio_jpg_inferences::standards::v_jfif_1_01::subsets::baseline::schema::conformance::*;
                        #[cfg(test)]
                        #[path="../../../../🧱️baseline/🧬️schema/🏅️conformance/🧪️tests/🦀️.rs"]
                        pub mod tests;
                    }
                }
            }
        }
    }
}
