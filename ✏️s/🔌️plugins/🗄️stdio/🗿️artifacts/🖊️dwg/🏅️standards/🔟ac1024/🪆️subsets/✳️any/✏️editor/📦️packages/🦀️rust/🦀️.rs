//! ✏️ `stdio.dwg` editors, viewers and examples of both standards.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod architectural {
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/📚️examples/🏛️architectural/🧪️tests/🧩️example/🦀️.rs"]
        mod architectural_tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod dwg_ac1018 {
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "../../🧭️edit-rules/🦀️.rs"]
        pub mod edit_rules;
        #[path = "."]
        pub mod presence {
            #[path = "../../👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod dwg_ac1018 {
        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../../../4️⃣ac1018/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
    #[path = "."]
    pub mod dwg_ac1024 {
        #[path = "../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
