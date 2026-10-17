//! 🧬️ schema of `semio-s-artifact-stdio-png-mutations` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

pub use schema::snapshot::PngSnapshot;
pub mod schema {
    pub use super::standards::v1_2::subsets::any::schema::*;
}

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1_2 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_png_schema::standards::v1_2::subsets::any::schema::*;
                    pub use semio_s_artifact_stdio_png_snapshot::standards::v1_2::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        #[path = "../../../../../🧬️schema/🧬️mutations/🦀️.rs"]
                        mod top_level;
                        pub use top_level::*;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🌗️change-gamma/🦀️.rs"]
                        pub mod change_gamma;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs"]
                        pub mod patch_pixels;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🎨️paint-native-samples/🦀️.rs"]
                        pub mod paint_native_samples;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🔄️replace-image/🦀️.rs"]
                        pub mod replace_image;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🎚️set-gamma/🦀️.rs"]
                        pub mod set_gamma;
                        #[path = "../../../../../🧬️schema/🧬️mutations/🧩️replace-samples/🦀️.rs"]
                        pub mod replace_samples;
                    }
                    pub use semio_s_artifact_stdio_png_diff::standards::v1_2::subsets::any::schema::operations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::*;
                    #[path = "."]
                    pub mod text {
                        pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::text::*;
                        #[path = "../../../../📝️text/🧬️mutations/🦀️.rs"]
                        pub mod mutations;
                    }
                    #[path = "."]
                    pub mod binary {
                        pub use semio_s_artifact_stdio_png_mutations_io::standards::v1_2::subsets::any::io::binary::*;
                        #[path = "../../🦀️.rs"]
                        pub mod mutations;
                    }
                }
            }
        }
    }
}
