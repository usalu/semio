//! 📐️ The typed PDF 1.7 document model: object graph, content, colour, fonts, XObjects, navigation, annotations, forms and the page model.

pub use semio_s_artifact_stdio_pdf_model_object::octets;
pub use semio_s_artifact_stdio_pdf_model_object::{ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfIndirectObject, PdfMatrix, PdfObject, PdfPredictor, PdfRect, PdfStreamBytes, PdfStreamFilter, stream_bytes, PDF_IDENTITY_MATRIX, PDF_STREAM_MAXIMUM_BYTES};
pub use semio_s_artifact_stdio_pdf_model_object::{PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfNamedDestination, PdfOutlineItem, PdfPageLabelRange, PdfPageLabelStyle};
pub use semio_s_artifact_stdio_pdf_model_object::{PdfDate, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfInfo, PdfMarkInfo, PdfOpenAction, PdfOptionalContent, PdfOptionalContentGroup, PdfOutputIntent, PdfPageLayout, PdfPageMode, PdfViewerPreferences};
pub use semio_s_artifact_stdio_pdf_model_graphics::{PdfInlineImage, PdfLineCap, PdfLineJoin, PdfOp, PdfPropertyList, PdfTextArrayItem, PdfTextString};
pub use semio_s_artifact_stdio_pdf_model_graphics::{PdfColorSpace, PdfExtGState, PdfFunction, PdfNamedColorSpace, PdfNamedProperties, PdfPattern, PdfPatternKind, PdfShading, PdfShadingKind, PdfSoftMask};
pub use semio_s_artifact_stdio_pdf_model_graphics::{pack_samples, packed_sample_geometry, PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfPackedSamples, PdfTransparencyGroup};
pub use semio_s_artifact_stdio_pdf_model_font::{
    PdfBaseEncoding, PdfCMap, PdfCharProc, PdfCidFont, PdfCidMapping, PdfCidSystemInfo, PdfCidToGid, PdfCidVerticalRun, PdfCidWidthRun, PdfCodespaceRange, PdfEmbeddedCMap, PdfEncodingDifference, PdfFont, PdfFontDescriptor, PdfFontKind,
    PdfFontProgram, PdfSimpleEncoding, PdfToUnicode, PdfToUnicodeMapping,
};
pub use semio_s_artifact_stdio_pdf_model_page::PdfPage;
pub use semio_s_artifact_stdio_pdf_model_page::{PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfFormField, PdfFormFieldKind, PdfMarkupAnnotation};
