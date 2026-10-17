//! 🎬️ examples of `semio-s-artifact-space-home` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

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
                pub mod io {
                    #[path = "../../../../🚪️io/💾️binary/🦀️.rs"]
                    pub mod binary;
                    #[path = "../../../../🚪️io/📝️text/🦀️.rs"]
                    pub mod text;
                    #[path = "../../../../🚪️io/🪶️sqlite/🦀️.rs"]
                    pub mod sqlite;
                }
            }
        }
    }
}
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod home {
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
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod explore {
                #[path = "../../../../✏️editor/🎭️modes/🔎️explore/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod home {
        #[path = "."]
        pub mod config {
            #[path = "../../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod presence {
            #[path = "../../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
