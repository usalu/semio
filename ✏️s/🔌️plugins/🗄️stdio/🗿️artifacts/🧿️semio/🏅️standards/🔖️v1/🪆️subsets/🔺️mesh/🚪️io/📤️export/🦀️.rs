//! 📤️ The `mesh` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `mesh` documents out of gltf, las, obj, ply, png, stl, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod v2_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🎬️gltf/🔖️2.0/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod stl {
                #[path = "."]
                pub mod v_ascii {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod png {
                #[path = "."]
                pub mod v1_2 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod obj {
                #[path = "."]
                pub mod v3_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod ply {
                #[path = "."]
                pub mod v1_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod las {
                #[path = "."]
                pub mod v1_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod dwg {
                #[path = "."]
                pub mod v_ac1024 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

use semio_s_artifact_stdio_semio_mesh::io::SemioMeshFormat;
use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{serializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf>(), serializer_entry_of::<serializers::artifacts::stl::v_ascii::any::SemioMeshToStl>(), serializer_entry_of::<serializers::artifacts::png::v1_2::any::SemioMeshToPng>(), serializer_entry_of::<serializers::artifacts::obj::v3_0::any::SemioMeshToObj>(), serializer_entry_of::<serializers::artifacts::ply::v1_0::any::SemioMeshToPly>(), serializer_entry_of::<serializers::artifacts::las::v1_0::any::SemioMeshToLas>()]).as_slice()
}

/// 🔌️ Registers and declares this direction's composer rows; consumers name it in `register` / `declaration`.
pub const CONVERSION: SemioConversion = SemioConversion { declare, register };

fn declare(builder: ArtifactDeclarationBuilder<DeclarationReady>) -> ArtifactDeclarationBuilder<DeclarationReady> {
    static COMPOSERS: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    builder.composers(semio_s_artifact_stdio_semio_base::semio_written(entries(), &COMPOSERS))
}

fn register() {
    register_composer_entries(entries()).expect("static Stdio registration must be available and conflict-free");
}

/// 🧾️ The file bytes of `mesh` in `format` — the one call every domain artifact that projects into a
/// mesh makes, so each standard format has exactly one writer in the repository.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_mesh(mesh: &semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot, format: SemioMeshFormat) -> Result<Vec<u8>, String> {
    use {crate::serializers::artifacts::dwg::v_ac1024::any::SemioMeshToDwg, crate::serializers::artifacts::gltf::v2_0::any::SemioMeshToGltf, crate::serializers::artifacts::las::v1_0::any::SemioMeshToLas, crate::serializers::artifacts::obj::v3_0::any::SemioMeshToObj, crate::serializers::artifacts::ply::v1_0::any::SemioMeshToPly, crate::serializers::artifacts::png::v1_2::any::SemioMeshToPng, crate::serializers::artifacts::stl::v_ascii::any::SemioMeshToStl};
    use semio_framework_plugin::{ ArtifactSerializer};
    match format {
        SemioMeshFormat::Stl => Ok(semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::io::encode_stl_ascii(&::semio_framework_async::poll::resolve_ready(SemioMeshToStl::serialize(mesh)).map_err(|e| e.to_string())?).into_bytes()),
        SemioMeshFormat::Obj => Ok(semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::io::encode_obj(&::semio_framework_async::poll::resolve_ready(SemioMeshToObj::serialize(mesh)).map_err(|e| e.to_string())?).into_bytes()),
        SemioMeshFormat::Ply => semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::io::encode_ply(&::semio_framework_async::poll::resolve_ready(SemioMeshToPly::serialize(mesh)).map_err(|e| e.to_string())?),
        SemioMeshFormat::Gltf => Ok(semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::export::serialize_gltf_document(&::semio_framework_async::poll::resolve_ready(SemioMeshToGltf::serialize(mesh)).map_err(|e| e.to_string())?)),
        SemioMeshFormat::Las => semio_s_artifact_stdio_las::standards::v1_0::subsets::any::io::encode_las(&::semio_framework_async::poll::resolve_ready(SemioMeshToLas::serialize(mesh)).map_err(|e| e.to_string())?),
        SemioMeshFormat::Dwg => semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::dwg_to_bytes(&::semio_framework_async::poll::resolve_ready(SemioMeshToDwg::serialize(mesh)).map_err(|e| e.to_string())?.drawing.to_native()?),
        SemioMeshFormat::Png => semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&::semio_framework_async::poll::resolve_ready(SemioMeshToPng::serialize(mesh)).map_err(|e| e.to_string())?),
    }
}
