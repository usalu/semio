//! 📄 PDF page editor — paints every page and addresses text, vectors, images, forms, shadings,
//! and annotations as objects the end user can insert, move, resize, restyle, and delete.
//! Document info and page structure use the same command channel. A canvas click hit-tests a page
//! object and redispatches `interactionSelect`; it does not mutate the PDF. The object window then
//! shows that selection's own fields and commits them through the same page actions. Raw fields that
//! have no page geometry stay on the snapshot details window.

use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::{
    insert_object::InsertObject, insert_page::InsertPage, move_page::MovePage, remove_catalog_entry::RemoveCatalogEntry, remove_object::RemoveObject, remove_embedded_file::RemoveEmbeddedFile, remove_named_destination::RemoveNamedDestination, remove_page::RemovePage, replace_page::ReplacePage, remove_trailer_entry::RemoveTrailerEntry, set_annotation::SetAnnotation, set_acro_form::SetAcroForm, set_color_space::SetColorSpace, set_catalog_entry::SetCatalogEntry, set_ext_g_state::SetExtGState, set_font::SetFont, set_form::SetForm, set_trailer_entry::SetTrailerEntry, set_pattern::SetPattern, set_properties::SetProperties, set_image::SetImage, set_info::SetInfo, set_language::SetLanguage, set_metadata::SetMetadata, set_object_value::SetObjectValue, set_optional_content::SetOptionalContent, set_outlines::SetOutlines, set_page_content::SetPageContent, set_page_layout::SetPageLayout, set_page_media_box::SetPageMediaBox, set_page_mode::SetPageMode, set_shading::SetShading, set_viewer_preferences::SetViewerPreferences, PdfMutation,
};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{PdfContent, PdfProgram, pdf_content, ObjRef, PdfAction, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfBaseEncoding, PdfBorderStyle, PdfCharProc, PdfColorSpace, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfExtGState, PdfFileSpecification, PdfFont, PdfLineCap, PdfLineJoin, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfFormField, PdfFormFieldKind, PdfFunction, PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfMarkupAnnotation, PdfMatrix, PdfNamedColorSpace, PdfNamedProperties, PdfObject, PdfPattern, PdfPatternKind, PdfSimpleEncoding, PdfOp, PdfPage, PdfShadingKind, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup, PdfViewerPreferences, PDF_IDENTITY_MATRIX};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_plugin::plugin_app_close_prelude as ui;
use semio_plugin::ActionArgDef;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionId;
use semio_plugin::ActionKind;
use semio_plugin::Buildable;
use semio_plugin::Canvas2dScene;
use semio_plugin::Fault;
use semio_plugin::FaultCode;
use semio_plugin::HasBase;
use semio_plugin::HasChildren;
use semio_ui_locale::Locale;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PluginAssemblyError;
use semio_plugin::SurfaceKind;
use semio_plugin::UiAssemblyResult;
use semio_plugin::UiMapBuilder;
use semio_plugin::UiText;
use semio_plugin::UiValue;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowLayout;
use semio_plugin::WindowOptions;
pub use semio_s_artifact_stdio_pdf_page_editor_base::*;
use serde_json::{json, Value};

#[path = "🪟️windows/🦀️.rs"]
mod windows;
pub use self::windows::*;

#[path = "🎬️actions/🦀️.rs"]
mod actions;
pub use self::actions::*;

#[path = "🖱️object-edits/🦀️.rs"]
mod object_edits;
pub use self::object_edits::*;

#[path = "📄️document-edits/🦀️.rs"]
mod document_edits;
use self::document_edits::*;
use semio_s_artifact_stdio_pdf_editor_document::*;

pub use semio_s_artifact_stdio_pdf_page_editor_authoring::*;

#[path = "🔖️annotation-edits/🦀️.rs"]
mod annotation_edits;
pub use self::annotation_edits::*;

#[path = "🎨️resource-edits/🦀️.rs"]
mod resource_edits;
pub use self::resource_edits::*;

#[path = "🔎️inspector/🦀️.rs"]
pub mod inspector;
pub use self::inspector::*;

pub use semio_s_artifact_stdio_pdf_page_editor_view::*;


pub use semio_s_artifact_stdio_pdf_editor_document::{cancelled, page_fault, page_of, pdf_fault_notices, refuse, resolve_page, PdfFault, SaveCause};

#[path = "♿️text-layer/🦀️.rs"]
mod text_layer;
pub use self::text_layer::*;

#[path = "📝️page-text/🦀️.rs"]
mod page_text;
pub use self::page_text::*;

