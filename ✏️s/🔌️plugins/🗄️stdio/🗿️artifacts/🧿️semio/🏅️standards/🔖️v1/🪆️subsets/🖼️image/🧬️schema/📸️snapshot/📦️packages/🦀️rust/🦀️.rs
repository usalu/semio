//! 📸️ snapshot of `semio-s-artifact-stdio-semio-image` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
    #[path = "."]
    pub mod sqlite {
        #[path = "../../../../🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
    }
}
#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod snapshot {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
semio_framework_value::artifact_retire_leaf!(schema::snapshot::SemioColorspace);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioImageFrame { delay_ms, rgba8 });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioImageMetadataEntry { key, value });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioImageSnapshot { schema, width, height, colorspace, bit_depth, frames, icc, metadata });
semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioImageSnapshot);
