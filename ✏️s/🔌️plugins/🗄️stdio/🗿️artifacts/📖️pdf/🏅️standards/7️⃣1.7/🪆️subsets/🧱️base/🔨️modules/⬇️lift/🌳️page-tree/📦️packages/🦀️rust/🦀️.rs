//! 🌳️ Resumable PDF page-tree traversal.

#![allow(async_fn_in_trait)]

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
                    pub use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::lexer;
                }
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_diff::standards::v1_7::subsets::base::schema::graph_source;
                    pub use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot;
                }
            }
        }
    }
}

#[path = "../../🦀️.rs"]
mod page_tree;
pub use page_tree::*;
