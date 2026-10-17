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
                    #[path = "."]
                    pub mod snapshot {
                        #[path = "../../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/💾️binary/🧬️mutations/🧰️wire/🦀️.rs"]
                            pub mod wire;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                        #[path = "../../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../../🚪️io/📝️text/🧬️mutations/🧰️wire/🦀️.rs"]
                            pub mod wire;
                        }
                    }
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "../../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
                        pub mod snapshot;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod pdf17a {
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🗄️a/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17 {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧱️base/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17e {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/📐️e/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/📐️e/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17h {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/⚕️h/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/⚕️h/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17ua {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/♿️ua/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/♿️ua/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17vt {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🧾️vt/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
    #[path = "."]
    pub mod pdf17x {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🖨️x/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../../../7️⃣1.7/🪆️subsets/🖨️x/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod pdf14a {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../🗄️a/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
