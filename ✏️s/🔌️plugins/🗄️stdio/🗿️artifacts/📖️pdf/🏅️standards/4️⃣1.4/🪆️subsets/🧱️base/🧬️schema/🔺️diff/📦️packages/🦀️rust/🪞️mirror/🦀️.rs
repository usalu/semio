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
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14a {
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../🗄️a/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../🖨️x/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../🖨️x/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🗄️a/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/📐️e/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/📐️e/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/⚕️h/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/♿️ua/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧾️vt/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🖨️x/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
