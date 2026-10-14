//! 📥️ The `brep` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `brep` documents into step, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod step {
                #[path = "."]
                pub mod v_ap214 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{deserializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::step::v_ap214::any::SemioBrepFromStep>()]).as_slice()
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

/// 🖊️ DWG mesh import for the BREP artifact.
pub mod dwg {
    use semio_framework_3d_brep_engine::{BrepError, BrepKernel, GeometryHandle};
    struct DwgImporter;
    impl semio_framework_mesh_engine::io::MeshImporter for DwgImporter {
        fn format_kind(&self) -> &'static str {
            "dwg"
        }
        fn import(&self, bytes: &[u8]) -> Result<semio_framework_mesh_engine::MeshData, String> {
            let drawing = semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::dwg_from_bytes(bytes)?;
            Ok(semio_s_artifact_stdio_dwg_geometry::standards::v_ac1024::subsets::any::schema::geometry::dwg_drawing_to_mesh(&drawing))
        }
    }

    /// 📥️ Imports a DWG mesh into the BREP artifact's kernel.
    pub fn import(kernel: &mut dyn BrepKernel, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        kernel.import_mesh(data, tolerance, &DwgImporter)
    }
}
