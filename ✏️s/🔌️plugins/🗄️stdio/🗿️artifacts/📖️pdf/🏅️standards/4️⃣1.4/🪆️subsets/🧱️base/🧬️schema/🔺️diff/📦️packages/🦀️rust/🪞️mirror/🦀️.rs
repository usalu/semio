//! 🪞️ The mirrored module nest of this crate, kept out of the lib root so the root stays pure wiring.

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_4 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_pdf_snapshot::standards::v1_4::subsets::base::schema::snapshot;
                    #[path = "."]
                    pub mod diff {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
                        pub mod diff;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod v1_7 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod base {
                #[path = "."]
                pub mod schema {
                    #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔗️graph-source/🦀️.rs"]
                    pub mod graph_source;
                    #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🪪️stream-roles/🦀️.rs"]
                    pub mod stream_roles;
                    #[path = "."]
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_pdf_model::standards::v1_7::subsets::base::schema::snapshot::model;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/📸️snapshot/🔢️number/🦀️.rs"]
                            pub mod number;
                        }
                    }
                }
            }
        }
    }
}
