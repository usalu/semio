//! 📐️ The typed PDF 1.7 document model: object graph, content, colour, fonts, XObjects, navigation, annotations, forms and the page model.

#[path="../🌱️value/🧵️canonical/🦀️.rs"]
mod canonical;
#[path="../🌱️value/🧬️octets/🦀️.rs"]
pub mod octets;

//#region 🔖️ObjectModel

#[path = "🔗️object-model/🦀️.rs"]
mod object_model;
pub use object_model::{ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfIndirectObject, PdfMatrix, PdfObject, PdfPredictor, PdfRect, PdfStreamFilter, PDF_IDENTITY_MATRIX};

#[path = "🖋️content-operators/🦀️.rs"]
mod content_operators;
pub use content_operators::{PdfInlineImage, PdfLineCap, PdfLineJoin, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};

#[path = "🌈️colour-and-graphics-state/🦀️.rs"]
mod colour_and_graphics_state;
pub use colour_and_graphics_state::{PdfColorSpace, PdfExtGState, PdfFunction, PdfNamedColorSpace, PdfNamedProperties, PdfPattern, PdfPatternKind, PdfShading, PdfShadingKind, PdfSoftMask};

#[path = "🔤️font-model/🦀️.rs"]
mod font_model;
pub use font_model::{
    PdfBaseEncoding, PdfCMap, PdfCharProc, PdfCidFont, PdfCidMapping, PdfCidSystemInfo, PdfCidToGid, PdfCidVerticalRun, PdfCidWidthRun, PdfCodespaceRange, PdfEmbeddedCMap, PdfEncodingDifference, PdfFont, PdfFontDescriptor, PdfFontKind,
    PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping,
};

#[path = "🖼️xobjects/🦀️.rs"]
mod xobjects;
pub use xobjects::{PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfTransparencyGroup};

#[path = "🎯️navigation/🦀️.rs"]
mod navigation;
pub use navigation::{PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfNamedDestination, PdfOutlineItem, PdfPageLabelRange, PdfPageLabelStyle};

#[path = "📌️annotations-and-forms/🦀️.rs"]
mod annotations_and_forms;
pub use annotations_and_forms::{PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfFormField, PdfFormFieldKind, PdfMarkupAnnotation};

#[path = "📄️document-metadata/🦀️.rs"]
mod document_metadata;
pub use document_metadata::{PdfDate, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfInfo, PdfMarkInfo, PdfOpenAction, PdfOptionalContent, PdfOptionalContentGroup, PdfOutputIntent, PdfPageLayout, PdfPageMode, PdfViewerPreferences};

#[path = "📃️page-model/🦀️.rs"]
mod page_model;
pub use page_model::PdfPage;

//#endregion 🔖️ObjectModel
