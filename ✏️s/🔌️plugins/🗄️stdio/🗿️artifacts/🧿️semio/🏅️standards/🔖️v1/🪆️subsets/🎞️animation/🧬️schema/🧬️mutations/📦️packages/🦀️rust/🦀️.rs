//! 🧬️ schema-4 of `semio-s-artifact-stdio-semio-animation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
    #[path = "."]
    pub mod text {
        pub use semio_s_artifact_stdio_semio_animation_schema::io::text::snapshot;
        #[path = "../../../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
        pub mod mutations;
    }
}
#[path = "."]
pub mod schema {
    pub use semio_s_artifact_stdio_semio_animation_schema::schema::snapshot;
    pub use semio_s_artifact_stdio_semio_animation_examples::schema::diff;
    #[path = "."]
    pub mod mutations {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
semio_s_artifact_stdio_semio_base::semio_member_owner!(schema::snapshot::SemioAnimationSnapshot, schema::mutations::SemioAnimationMutation);
