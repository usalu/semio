//! 🔤️ Typed PDF fonts and encodings.

extern crate semio_framework_value_derive as value_derive;

#[path = "../../🦀️.rs"]
mod font_model;
pub use font_model::{
    PdfBaseEncoding, PdfCMap, PdfCharProc, PdfCidFont, PdfCidMapping, PdfCidSystemInfo, PdfCidToGid, PdfCidVerticalRun, PdfCidWidthRun, PdfCodespaceRange, PdfEmbeddedCMap, PdfEncodingDifference, PdfFont, PdfFontDescriptor, PdfFontKind,
    PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping,
};
