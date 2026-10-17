//! 🧩 mut-family of `semio-s-artifact-stdio-semio-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_presentation_snapshot::schema::diff;
    #[path = "."]
    pub mod inferences {
        pub use semio_s_artifact_stdio_semio_presentation_examples::schema::inferences::*;
        #[cfg(test)]
        //#region 🧪️Tests
        #[path = "../../../../💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
        #[path = "."]
        pub mod outline {
            pub use semio_s_artifact_stdio_semio_presentation_examples::schema::inferences::outline::*;
            #[cfg(test)]
            //#region 🧪️Tests
            #[path = "../../../../💡️inferences/🧾outline/🧪️tests/🔬️unit/🦀️.rs"]
            pub mod tests;
        }
    }
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_presentation_schema::schema::mutations::support;
        #[path = "../../🦀️.rs"]
        pub mod insert_layout;
        #[path = "../../../🎓insert-master/🦀️.rs"]
        pub mod insert_master;
        #[path = "../../../🔷insert-shape/🦀️.rs"]
        pub mod insert_shape;
        #[path = "../../../🧹️remove-layout/🦀️.rs"]
        pub mod remove_layout;
        #[path = "../../../🚫️remove-master/🦀️.rs"]
        pub mod remove_master;
        #[path = "../../../🔶remove-shape/🦀️.rs"]
        pub mod remove_shape;
        #[path = "../../../🔧set-layout-master/🦀️.rs"]
        pub mod set_layout_master;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod modes {
        #[path = "."]
        pub mod edit {
            #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod windows {
                #[path = "."]
                pub mod main {
                    #[path = "../../../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
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
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                    mod component;
                    pub use component::*;
                }
            }
        }
    }
}
