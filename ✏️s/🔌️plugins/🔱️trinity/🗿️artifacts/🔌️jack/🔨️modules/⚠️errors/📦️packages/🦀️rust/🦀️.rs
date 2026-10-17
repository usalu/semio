//! ⚠️ model of `semio-s-artifact-trinity-jack` — a path-preserving carve of the artifact root mounts.

#![allow(clippy::unnecessary_wraps)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as vcs;
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
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🔎️set-query/🦀️.rs"]
                            pub mod set_query;
                        }
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                        #[path = "."]
                        pub mod mutations {
                            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🔎️set-query/🦀️.rs"]
                            pub mod set_query;
                        }
                    }
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod graph_manifest {
                        #[path = "../../../../🤖️generated/🏢️nakagin/🦀️.rs"]
                        pub mod nakagin;
                    }
                    #[path = "../../🦀️.rs"]
                    pub mod errors;
                    #[path = "../../../../🛂️manifest/🦀️.rs"]
                    pub mod manifest;
                    #[path = "../../../🔺️graph-model/🦀️.rs"]
                    pub mod graph;
                    #[path = "../../../../🪆️content/👁️read/🦀️.rs"]
                    pub mod content_read;
                    #[path = "../../../📡️json-transport/🦀️.rs"]
                    pub mod json_transport;
                    #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔤️lexer/🦀️.rs"]
                    pub mod lexer;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod jack {
        #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🔍️lod/🦀️.rs"]
        pub mod lod;
        #[path = "."]
        pub mod presence {
            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo_session {
                #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🦀️.rs"]
                mod component;
                pub use component::*;
                #[cfg(test)]
                #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
                pub mod tests;
            }
        }
        #[path = "."]
        pub mod terminology {
            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
            mod component;
            pub use component::*;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "."]
                pub mod tools {
                    #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🦀️.rs"]
                    pub mod reorganize;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod jack {
        #[path = "."]
        pub mod presence {
            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
    }
}
