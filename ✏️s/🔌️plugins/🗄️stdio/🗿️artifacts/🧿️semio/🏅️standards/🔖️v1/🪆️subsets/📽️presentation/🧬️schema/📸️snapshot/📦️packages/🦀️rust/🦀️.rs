//! 📸️ snapshot of `semio-s-artifact-stdio-semio-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![recursion_limit = "256"]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod io {
    #[path = "."]
    pub mod binary {
        #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
        #[path = "../../../../🚪️io/💾️binary/🔺️diff/🦀️.rs"]
        pub mod diff;
    }
    #[path = "."]
    pub mod text {
        #[path = "../../../../🚪️io/📝️text/📸️snapshot/🦀️.rs"]
        pub mod snapshot;
        #[path = "../../../../🚪️io/📝️text/🔺️diff/🦀️.rs"]
        pub mod diff;
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
    #[path = "."]
    pub mod diff {
        #[path = "../../../🔺️diff/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
semio_framework_value::artifact_retire_leaf!(schema::snapshot::SlideFrame);
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlidePictureImage { asset_id, mime, bytes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideTableCell { blocks });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideTableRow { cells });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideMaster { id, shapes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SlideLayout { id, master_id, shapes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::Slide { id, layout_id, shapes, notes });
semio_framework_value::artifact_retire_struct!(schema::snapshot::SemioPresentationSnapshot { schema, masters, layouts, slides });
semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!(schema::snapshot::SemioPresentationSnapshot);
