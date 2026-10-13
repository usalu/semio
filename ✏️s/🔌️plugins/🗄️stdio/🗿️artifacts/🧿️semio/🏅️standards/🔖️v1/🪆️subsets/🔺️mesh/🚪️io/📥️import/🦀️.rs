//! 📥️ The `mesh` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `mesh` documents into gltf, las, obj, ply, stl, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod gltf {
                #[path = "."]
                pub mod v2_0 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🎬️gltf/🔖️2.0/✳️any/🦀️.rs"]
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
                        #[path = "🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
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
                        #[path = "🧩️deserializers/🗿️artifacts/🧊️obj/🔖️3.0/✳️any/🦀️.rs"]
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
                        #[path = "🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
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
                        #[path = "🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
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
                        #[path = "🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1024/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

use semio_s_artifact_stdio_semio_mesh::io::SemioMeshFormat;
use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{deserializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf>(), deserializer_entry_of::<deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl>(), deserializer_entry_of::<deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj>(), deserializer_entry_of::<deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly>(), deserializer_entry_of::<deserializers::artifacts::las::v1_0::any::SemioMeshFromLas>()]).as_slice()
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

/// 📥️ `bytes` in `format` read into a mesh through this subset's own import leaves (png has none: a
/// picture of a mesh carries no geometry).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_mesh(bytes: &[u8], format: SemioMeshFormat) -> Result<semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot, String> {
    use {crate::deserializers::artifacts::dwg::v_ac1024::any::SemioMeshFromDwg, crate::deserializers::artifacts::gltf::v2_0::any::SemioMeshFromGltf, crate::deserializers::artifacts::las::v1_0::any::SemioMeshFromLas, crate::deserializers::artifacts::obj::v3_0::any::SemioMeshFromObj, crate::deserializers::artifacts::ply::v1_0::any::SemioMeshFromPly, crate::deserializers::artifacts::stl::v_ascii::any::SemioMeshFromStl};
    use semio_framework_plugin::{ ArtifactDeserializer};
    let text = || std::str::from_utf8(bytes).map_err(|e| e.to_string());
    match format {
        SemioMeshFormat::Stl => ::semio_framework_async::poll::resolve_ready(SemioMeshFromStl::deserialize(&semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::io::decode_stl_auto(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Obj => ::semio_framework_async::poll::resolve_ready(SemioMeshFromObj::deserialize(&semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::io::decode_obj(text()?)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Ply => ::semio_framework_async::poll::resolve_ready(SemioMeshFromPly::deserialize(&semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::io::decode_ply(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Gltf => {
            let gltf = if bytes.starts_with(b"glTF") { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::decode_glb(bytes)? } else { semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::io::import::parse_gltf_document(bytes)? };
            ::semio_framework_async::poll::resolve_ready(SemioMeshFromGltf::deserialize(&gltf)).map_err(|e| e.to_string())
        }
        SemioMeshFormat::Las => ::semio_framework_async::poll::resolve_ready(SemioMeshFromLas::deserialize(&semio_s_artifact_stdio_las::standards::v1_0::subsets::any::io::decode_las(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Dwg => ::semio_framework_async::poll::resolve_ready(SemioMeshFromDwg::deserialize(&semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_dwg(bytes)?)).map_err(|e| e.to_string()),
        SemioMeshFormat::Png => Err("semio/mesh←png: a picture of a mesh carries no geometry".into()),
    }
}
