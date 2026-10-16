//! 📄 PDF page editor — paints every page and addresses text, vectors, images, forms, shadings,
//! and annotations as objects the end user can insert, move, resize, restyle, and delete.
//! Document info and page structure use the same command channel. A canvas click hit-tests a page
//! object and redispatches `interactionSelect`; it does not mutate the PDF. The object window then
//! shows that selection's own fields and commits them through the same page actions. Raw fields that
//! have no page geometry stay on the snapshot details window.

use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
use crate::standards::v1_7::subsets::base::schema::mutations::{
    insert_object::InsertObject, insert_page::InsertPage, move_page::MovePage, remove_catalog_entry::RemoveCatalogEntry, remove_object::RemoveObject, remove_embedded_file::RemoveEmbeddedFile, remove_named_destination::RemoveNamedDestination, remove_page::RemovePage, replace_page::ReplacePage, remove_trailer_entry::RemoveTrailerEntry, set_annotation::SetAnnotation, set_embedded_file::SetEmbeddedFile, set_acro_form::SetAcroForm, set_document_id::SetDocumentId, set_color_space::SetColorSpace, set_catalog_entry::SetCatalogEntry, set_encryption::SetEncryption, set_ext_g_state::SetExtGState, set_font::SetFont, set_form::SetForm, set_trailer_entry::SetTrailerEntry, set_open_action::SetOpenAction, set_output_intents::SetOutputIntents, set_pattern::SetPattern, set_properties::SetProperties, set_image::SetImage, set_info::SetInfo, set_language::SetLanguage, set_mark_info::SetMarkInfo, set_metadata::SetMetadata, set_named_destination::SetNamedDestination, set_object_value::SetObjectValue, set_optional_content::SetOptionalContent, set_outlines::SetOutlines, set_page_box::SetPageBox, set_page_content::SetPageContent, set_page_labels::SetPageLabels, set_page_layout::SetPageLayout, set_page_media_box::SetPageMediaBox, set_page_mode::SetPageMode, set_page_rotation::SetPageRotation, set_page_user_unit::SetPageUserUnit, set_shading::SetShading, set_viewer_preferences::SetViewerPreferences, PdfMutation,
};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfAction, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfBaseEncoding, PdfBorderStyle, PdfCharProc, PdfColorSpace, PdfDate, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfEmbeddedFile, PdfEncryption, PdfEncryptionAlgorithm, PdfExtGState, PdfFileSpecification, PdfFont, PdfLineCap, PdfLineJoin, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfFormField, PdfFormFieldKind, PdfFunction, PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfMarkInfo, PdfMarkupAnnotation, PdfMatrix, PdfNamedColorSpace, PdfNamedDestination, PdfNamedProperties, PdfObject, PdfOpenAction, PdfOutputIntent, PdfPattern, PdfPatternKind, PdfSimpleEncoding, PdfOp, PdfOptionalContentGroup, PdfOutlineItem, PdfPage, PdfPageLabelRange, PdfPageLabelStyle, PdfPageLayout, PdfPageMode, PdfShadingKind, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup, PdfViewerPreferences, PDF_IDENTITY_MATRIX};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_framework_plugin::plugin_app_close_prelude as ui;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionId;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::Buildable;
use semio_framework_plugin::Canvas2dScene;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::HasBase;
use semio_framework_plugin::HasChildren;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::SurfaceKind;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::WindowKindDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutAxisNode;
use semio_framework_plugin::WindowLayoutChild;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;
use semio_framework_plugin::WindowOptions;
use serde_json::{json, Value};

#[path = "🧱️model/🦀️.rs"]
mod model;
pub use self::model::*;

#[path = "🪟️windows/🦀️.rs"]
mod windows;
pub use self::windows::*;

#[path = "🎬️actions/🦀️.rs"]
mod actions;
pub use self::actions::*;

#[path = "🗺️scene/🦀️.rs"]
mod scene;
pub use self::scene::*;

#[path = "🖱️object-edits/🦀️.rs"]
mod object_edits;
pub use self::object_edits::*;

#[path = "📄️document-edits/🦀️.rs"]
mod document_edits;
pub use self::document_edits::*;

#[path = "🔖️annotation-edits/🦀️.rs"]
mod annotation_edits;
pub use self::annotation_edits::*;

#[path = "🎨️resource-edits/🦀️.rs"]
mod resource_edits;
pub use self::resource_edits::*;

#[path = "🧮️content-ops/🦀️.rs"]
mod content_ops;
pub use self::content_ops::*;

#[path = "🧭️geometry/🦀️.rs"]
mod geometry;
pub use self::geometry::*;

#[path = "🧰️arguments/🦀️.rs"]
mod arguments;
pub use self::arguments::*;

#[path = "🔎️inspector/🦀️.rs"]
mod inspector;
pub use self::inspector::*;


#[path = "🖌️graphics/🦀️.rs"]
mod graphics;
pub use self::graphics::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[path = "📝️page-text/🦀️.rs"]
mod page_text;
pub use self::page_text::*;
