//! 🧱️ records of `semio-s-artifact-puzzle-2d` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::result_large_err)]

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
                    #[path = "."]
                    pub mod graph_manifest {
                        #[path = "../../../../../../../../🤖️generated/🏢️nakagin/🦀️.rs"]
                        pub mod nakagin;
                        #[path = "../../../../../../../../🤖️generated/◻️puzzle2d-default/🦀️.rs"]
                        pub mod puzzle2d_default;
                    }
                    #[path = "../../🦀️.rs"]
                    pub mod records;
                    #[cfg(any(test, feature = "testkit"))]
                    #[path = "../../../../../../../../🧪️tests/🔗️source-custody/🦀️.rs"]
                    pub mod testkit;
                    #[path = "."]
                    pub mod preparation {
                        #[path = "../../../🧬️mutations/🎮️prepare/🧰️child/🦀️.rs"]
                        pub mod native_preparation_child;
                    }
                    #[path = "."]
                    pub mod mutations {
                        #[path="../../../🧬️mutations/🧵️canonical/🦀️.rs"]
                        pub mod canonical_fields;
                    }
                }
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod puzzle2d {
        #[path = "."]
        pub mod window {
            #[path = "../../../../✏️editor/🪟️window/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../../../✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "../../../../✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "../../../../✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
    }
}
