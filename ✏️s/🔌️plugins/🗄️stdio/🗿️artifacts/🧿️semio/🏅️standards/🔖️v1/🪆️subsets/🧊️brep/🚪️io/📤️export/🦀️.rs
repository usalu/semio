//! 📤️ The `brep` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `brep` documents out of step, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod step {
                #[path = "."]
                pub mod v_ap214 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/📐️step/🔖️ap214/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }

use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{serializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::step::v_ap214::any::SemioBrepToStep>()]).as_slice()
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

/// 🖊️ DWG mesh export for the BREP artifact.
pub mod dwg {
    use semio_framework_3d_brep_engine::{BrepError, BrepKernel, GeometryHandle};
    struct DwgExporter;
    impl semio_framework_mesh_engine::io::MeshExporter for DwgExporter {
        fn format_kind(&self) -> &'static str {
            "dwg"
        }
        fn export(&self, mesh: &semio_framework_mesh_engine::MeshData) -> Result<Vec<u8>, String> {
            let drawing = semio_s_artifact_stdio_dwg_model::standards::v_ac1024::subsets::any::schema::geometry::mesh_to_dwg_drawing(mesh);
            semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::dwg_to_bytes(&drawing)
        }
    }

    /// 📤️ Encodes tessellated BREP geometry through the DWG artifact codec.
    pub fn export(kernel: &dyn BrepKernel, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        kernel.export_mesh(shapes, deflection, &DwgExporter)
    }
}
