//! 📥️ The `drawing` subset's import conversions: the deserializer leaves that carry `s.stdio.semio` `drawing` documents into dxf, pdf, svg, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod deserializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
            #[path = "."]
            pub mod pdf {
                #[path = "."]
                pub mod v1_7 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
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

use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat;
use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{deserializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![deserializer_entry_of::<deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg>(), deserializer_entry_of::<deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf>(), deserializer_entry_of::<deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf>()]).as_slice()
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

/// 📥️ `bytes` in `format` read into a drawing through this subset's own import leaves (png has none:
/// a raster carries no vector geometry).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_drawing(bytes: &[u8], format: SemioDrawingFormat) -> Result<semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot, String> {
    use {crate::deserializers::artifacts::dwg::v_ac1024::any::SemioDrawingFromDwg, crate::deserializers::artifacts::dxf::v_r12::any::SemioDrawingFromDxf, crate::deserializers::artifacts::pdf::v1_7::any::SemioDrawingFromPdf, crate::deserializers::artifacts::svg::v1_1::any::SemioDrawingFromSvg};
    use semio_framework_plugin::{ ArtifactDeserializer};
    let text = || std::str::from_utf8(bytes).map_err(|e| e.to_string());
    match format {
        SemioDrawingFormat::Svg => ::semio_framework_async::poll::resolve_ready(SemioDrawingFromSvg::deserialize(&semio_s_artifact_stdio_svg::SvgSnapshot::import_utf8(bytes)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Dxf => ::semio_framework_async::poll::resolve_ready(SemioDrawingFromDxf::deserialize(&semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::io::text::snapshot::parse_dxf_document(text()?)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Dwg => ::semio_framework_async::poll::resolve_ready(SemioDrawingFromDwg::deserialize(&semio_s_artifact_stdio_dwg_read::standards::v_ac1024::subsets::any::io::import::decode_dwg(bytes)?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Pdf { .. } => ::semio_framework_async::poll::resolve_ready(SemioDrawingFromPdf::deserialize(&semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::import::decode_pdf(bytes).map_err(|e| e.to_string())?)).map_err(|e| e.to_string()),
        SemioDrawingFormat::Png => Err("semio/drawing←png: a raster carries no vector geometry".into()),
    }
}
