//! 📻 schema-3 of `semio-s-artifact-stdio-semio-animation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_animation_schema::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_animation_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        pub use semio_s_artifact_stdio_semio_animation_examples::schema::mutations::support;
        #[path = "../../🦀️.rs"]
        pub mod insert_channel;
        #[path = "../../../🔑insert-keyframe/🦀️.rs"]
        pub mod insert_keyframe;
        #[path = "../../../🎬insert-timeline/🦀️.rs"]
        pub mod insert_timeline;
        #[path = "../../../🗑️remove-channel/🦀️.rs"]
        pub mod remove_channel;
        #[path = "../../../🔓remove-keyframe/🦀️.rs"]
        pub mod remove_keyframe;
        #[path = "../../../🧹remove-timeline/🦀️.rs"]
        pub mod remove_timeline;
        #[path = "../../../📈set-channel-interpolation/🦀️.rs"]
        pub mod set_channel_interpolation;
        #[path = "../../../🎯set-channel-target/🦀️.rs"]
        pub mod set_channel_target;
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
                pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::windows::*;
                #[path = "."]
                pub mod main {
                    pub use semio_s_artifact_stdio_semio_animation_examples::viewer::modes::view::windows::main::*;
                    #[cfg(test)]
                    #[path = "../../../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🧪️tests/🔬️unit/🦀️.rs"]
                    mod tests;
                }
            }
        }
    }
}
