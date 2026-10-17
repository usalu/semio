//! 🧬️ schema of `semio-s-artifact-stdio-semio-object` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot::*;
        pub use semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod view {
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
#[path = "."]
pub mod io {
    #[path = "../../../🚪️io/💾️binary/🦀️.rs"]
    pub mod binary;
    #[path = "../../../🚪️io/📝️text/🦀️.rs"]
    pub mod text;
}
