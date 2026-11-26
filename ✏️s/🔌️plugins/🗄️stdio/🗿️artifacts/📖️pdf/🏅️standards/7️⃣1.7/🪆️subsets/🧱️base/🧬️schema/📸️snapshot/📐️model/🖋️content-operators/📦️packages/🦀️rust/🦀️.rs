//! 🖋️ PDF content operators, graphics state and XObjects.

extern crate semio_framework_value_derive as value_derive;

#[path = "../../🦀️.rs"]
mod content_operators;
pub use content_operators::{PdfInlineImage, PdfLineCap, PdfLineJoin, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};

#[path = "../../../🌈️colour-and-graphics-state/🦀️.rs"]
mod colour_and_graphics_state;
pub use colour_and_graphics_state::{PdfColorSpace, PdfExtGState, PdfFunction, PdfNamedColorSpace, PdfNamedProperties, PdfPattern, PdfPatternKind, PdfShading, PdfShadingKind, PdfSoftMask};

#[path = "../../../🖼️xobjects/🦀️.rs"]
mod xobjects;
pub use xobjects::{pack_samples, packed_sample_geometry, PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfPackedSamples, PdfTransparencyGroup};
