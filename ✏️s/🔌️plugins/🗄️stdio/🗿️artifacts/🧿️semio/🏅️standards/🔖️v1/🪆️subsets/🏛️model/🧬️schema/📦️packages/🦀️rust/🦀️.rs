//! 🧬️ schema of `semio-s-artifact-stdio-semio-model` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod schema {
    #[path = "../../🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "."]
    pub mod snapshot {
        pub use semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot::*;
        #[cfg(test)]
        #[path = "../../📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
        pub mod tests;
    }
    #[path = "."]
    pub mod inferences {
        #[path = "../../💡️inferences/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod bounds {
            #[path = "../../💡️inferences/📦bounds/🦀️.rs"]
            mod component;
            pub use component::*;
        }
    }
}
