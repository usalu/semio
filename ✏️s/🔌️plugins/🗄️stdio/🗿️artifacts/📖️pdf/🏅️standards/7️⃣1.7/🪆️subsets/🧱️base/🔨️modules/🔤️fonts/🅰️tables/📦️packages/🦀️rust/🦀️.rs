//! 🅰️ tables of `semio-s-artifact-stdio-pdf` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod modules {
                    #[path = "."]
                    pub mod fonts {
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::modules::fonts::metrics;
                        #[path = "../../🦀️.rs"]
                        pub mod tables;
                    }
                }
            }
        }
    }
}
