//! 🧬️ `stdio.gltf` model: the glTF 2.0 snapshot, diff and schema, the accessor and GLB vocabulary, the reader, the writer and the native codecs.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::*;

                    /// 🧳️ The element types of the snapshot, plus the snapshot root and its schema id at the path the artifact has always published them under.
                    pub mod snapshot {
                        pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::schema::snapshot::*;
                        pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root::{GltfSnapshot, STDIO_GLTF_DOCUMENT_SCHEMA};
                    }
                    pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::schema::snapshot_root;
                    /// 🔺️ The element diffs, plus the aggregate diff at the path the artifact has always published it under.
                    pub mod diff {
                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::schema::diff::*;
                        pub use semio_s_artifact_stdio_gltf_model_diff_aggregate::standards::v2_0::subsets::any::schema::diff_aggregate::{demo_diff_cases, GltfDiff};
                    }
                    pub use semio_s_artifact_stdio_gltf_model_diff_aggregate::standards::v2_0::subsets::any::schema::diff_aggregate;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_stdio_gltf_model_snapshot::standards::v2_0::subsets::any::io::binary::*;
    pub use semio_s_artifact_stdio_gltf_model_diff_aggregate::standards::v2_0::subsets::any::io::binary::diff;
}
pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::deserializers_artifacts_json;
pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::export_serializers_artifacts;
pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::import_deserializers_artifacts;
pub use semio_s_artifact_stdio_gltf_model_v2_0::standards::v2_0::subsets::any::io::serializers_artifacts_json;
pub mod text {
    pub use semio_s_artifact_stdio_gltf_model_diff_aggregate::standards::v2_0::subsets::any::io::text::*;
}
pub mod sqlite {
    pub use semio_s_artifact_stdio_gltf_model_diff_aggregate::standards::v2_0::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod snapshot_codecs {
                        pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::io::snapshot_codecs::pack_codec;
                        pub use semio_s_artifact_stdio_gltf_model_snapshot_root::standards::v2_0::subsets::any::io::snapshot_codecs::dsl_codec;
                    }
                    #[path = "."]
                    pub mod import {
                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::io::import::*;

                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::io::import::deserializers;
                    }
                    #[path = "."]
                    pub mod export {
                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::io::export::*;

                        pub use semio_s_artifact_stdio_gltf_model_schema::standards::v2_0::subsets::any::io::export::serializers;
                    }
                }
            }
        }
    }
}
