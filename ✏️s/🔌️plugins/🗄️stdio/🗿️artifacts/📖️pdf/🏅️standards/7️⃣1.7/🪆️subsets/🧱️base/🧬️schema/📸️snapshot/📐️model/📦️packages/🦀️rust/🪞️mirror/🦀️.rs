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
                pub mod io {
                    #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
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
                pub mod modules {
                    #[path = "."]
                    pub mod fonts {
                        #[path = "../../../../../../🔨️modules/🔤️fonts/📏️metrics/🦀️.rs"]
                        pub mod metrics;
                        #[path = "../../../../../../🔨️modules/🔤️fonts/🔠️truetype/🦀️.rs"]
                        pub mod truetype;
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️mutations/🧰️support/🦀️.rs"]
                        pub mod support;
                    }
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../🦀️.rs"]
                        pub mod model;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../../../🚪️io/🚨️error/🦀️.rs"]
                    pub mod error;
                    #[path = "../../../../../../🚪️io/🔤️text-string/🦀️.rs"]
                    pub mod text_string;
                }
            }
            #[path = "."]
            pub mod e {
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../📐️e/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../📐️e/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
            #[path = "."]
            pub mod h {
                #[path = "."]
                pub mod io {
                    #[path = "../../../../../../../⚕️h/🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../../../../⚕️h/🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod pdf14a {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🗄️a/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf14 {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf14x {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../../4️⃣1.4/🪆️subsets/🖨️x/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17a {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../🗄️a/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
