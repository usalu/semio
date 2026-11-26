//! 📃️ PDF pages, annotations and interactive forms.

extern crate semio_framework_value_derive as value_derive;

#[path = "../../🦀️.rs"]
mod page_model;
pub use page_model::PdfPage;

#[path = "../../../📌️annotations-and-forms/🦀️.rs"]
mod annotations_and_forms;
pub use annotations_and_forms::{PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfAppearanceState, PdfBorderStyle, PdfFormField, PdfFormFieldKind, PdfMarkupAnnotation};
