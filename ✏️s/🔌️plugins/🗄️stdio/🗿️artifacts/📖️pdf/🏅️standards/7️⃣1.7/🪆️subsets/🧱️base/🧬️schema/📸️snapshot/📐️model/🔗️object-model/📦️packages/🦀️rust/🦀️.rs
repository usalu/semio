//! 🔗️ PDF COS objects, navigation and document metadata.

extern crate semio_framework_value_derive as value_derive;

#[path = "../../🦀️.rs"]
mod object_model;
pub use object_model::{ObjRef, PdfCcittParameters, PdfDecimal, PdfDictEntry, PdfIndirectObject, PdfMatrix, PdfObject, PdfPredictor, PdfRect, PdfStreamBytes, PdfStreamFilter, stream_bytes, PDF_IDENTITY_MATRIX, PDF_STREAM_MAXIMUM_BYTES};

#[path = "../../../🎯️navigation/🦀️.rs"]
mod navigation;
pub use navigation::{PdfAction, PdfActionKind, PdfDestination, PdfDestinationFit, PdfFileSpecification, PdfNamedDestination, PdfOutlineItem, PdfPageLabelRange, PdfPageLabelStyle};

#[path = "../../../📄️document-metadata/🦀️.rs"]
mod document_metadata;
pub use document_metadata::{PdfDate, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfInfo, PdfMarkInfo, PdfOpenAction, PdfOptionalContent, PdfOptionalContentGroup, PdfOutputIntent, PdfPageLayout, PdfPageMode, PdfViewerPreferences};

#[path = "../../../../🌱️value/🧵️canonical/🦀️.rs"]
mod canonical;

#[path = "../../../../🌱️value/🧬️octets/🦀️.rs"]
pub mod octets;
