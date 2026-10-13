//! 📤️ The `drawing` subset's export conversions: the serializer leaves that carry `s.stdio.semio` `drawing` documents out of dxf, pdf, png, svg, and the
//! `CONVERSION` that registers and declares their composer rows.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as store;

    #[path = "."]
    pub mod serializers {
        #[path = "."]
        pub mod artifacts {
            #[path = "."]
            pub mod svg {
                #[path = "."]
                pub mod v1_1 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
            pub mod dxf {
                #[path = "."]
                pub mod v_r12 {
                    #[path = "."]
                    pub mod any {
                        #[path = "🧵️serializers/🗿️artifacts/🔄️dxf/🔖️r12/✳️any/🦀️.rs"]
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
                        #[path = "🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🦀️.rs"]
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

use semio_s_artifact_stdio_semio_drawing::io::SemioDrawingFormat;
use semio_s_artifact_stdio_semio_base::SemioConversion;
use semio_framework_plugin::{serializer_entry_of, io::{register_composer_entries, ComposerEntry}, app::{ArtifactDeclarationBuilder, DeclarationReady}};

/// 🧾️ This direction's composer rows, built once.
pub fn entries() -> &'static [ComposerEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ComposerEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| vec![serializer_entry_of::<serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg>(), serializer_entry_of::<serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf>(), serializer_entry_of::<serializers::artifacts::pdf::v1_7::any::SemioDrawingToPdf>(), serializer_entry_of::<serializers::artifacts::png::v1_2::any::SemioDrawingToPng>()]).as_slice()
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

/// 🧾️ The file bytes of `drawing` in `format` — the one call every domain artifact that projects
/// into a drawing makes, so each standard format has exactly one writer in the repository.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_drawing(drawing: &semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot, format: SemioDrawingFormat) -> Result<Vec<u8>, String> {
    use {crate::serializers::artifacts::dwg::v_ac1024::any::SemioDrawingToDwg, crate::serializers::artifacts::dxf::v_r12::any::SemioDrawingToDxf, crate::serializers::artifacts::pdf::v1_7::any::drawing_to_pdf, crate::serializers::artifacts::png::v1_2::any::SemioDrawingToPng, crate::serializers::artifacts::svg::v1_1::any::SemioDrawingToSvg};
    use semio_framework_plugin::{ ArtifactSerializer};
    match format {
        SemioDrawingFormat::Svg => ::semio_framework_async::poll::resolve_ready(SemioDrawingToSvg::serialize(drawing)).map_err(|e| e.to_string())?.export_utf8(),
        SemioDrawingFormat::Dxf => Ok(semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::io::text::snapshot::print_dxf_document(&::semio_framework_async::poll::resolve_ready(SemioDrawingToDxf::serialize(drawing)).map_err(|e| e.to_string())?).into_bytes()),
        SemioDrawingFormat::Dwg => semio_s_artifact_stdio_dwg_write::standards::v_ac1024::subsets::any::io::export::dwg_to_bytes(&::semio_framework_async::poll::resolve_ready(SemioDrawingToDwg::serialize(drawing)).map_err(|e| e.to_string())?.drawing.to_native()?),
        SemioDrawingFormat::Pdf { version } => {
            let mut pdf = drawing_to_pdf(drawing)?;
            pdf.declared_version = version.to_string();
            semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::io::export::encode_pdf(&pdf).map_err(|e| e.to_string())
        }
        SemioDrawingFormat::Png => semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&::semio_framework_async::poll::resolve_ready(SemioDrawingToPng::serialize(drawing)).map_err(|e| e.to_string())?),
    }
}
