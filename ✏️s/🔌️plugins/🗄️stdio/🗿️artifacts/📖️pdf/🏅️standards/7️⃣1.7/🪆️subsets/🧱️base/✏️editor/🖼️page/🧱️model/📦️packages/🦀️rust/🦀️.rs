//! 🧱 PDF page editor base layer: model, geometry, scene painting, content operators and shared measurements.

use semio_s_artifact_stdio_pdf_mut_family_8::standards::v1_7::subsets::base::schema::mutations::{
    insert_object::InsertObject, insert_page::InsertPage, move_page::MovePage, remove_catalog_entry::RemoveCatalogEntry, remove_object::RemoveObject, remove_embedded_file::RemoveEmbeddedFile, remove_named_destination::RemoveNamedDestination, remove_page::RemovePage, replace_page::ReplacePage, remove_trailer_entry::RemoveTrailerEntry, set_annotation::SetAnnotation, set_acro_form::SetAcroForm, set_color_space::SetColorSpace, set_catalog_entry::SetCatalogEntry, set_ext_g_state::SetExtGState, set_font::SetFont, set_form::SetForm, set_trailer_entry::SetTrailerEntry, set_pattern::SetPattern, set_properties::SetProperties, set_image::SetImage, set_info::SetInfo, set_language::SetLanguage, set_metadata::SetMetadata, set_object_value::SetObjectValue, set_optional_content::SetOptionalContent, set_outlines::SetOutlines, set_page_content::SetPageContent, set_page_layout::SetPageLayout, set_page_media_box::SetPageMediaBox, set_page_mode::SetPageMode, set_shading::SetShading, set_viewer_preferences::SetViewerPreferences, PdfMutation,
};
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfAction, PdfAnnotation, PdfAnnotationKind, PdfAppearance, PdfAppearanceEntry, PdfBaseEncoding, PdfBorderStyle, PdfCharProc, PdfColorSpace, PdfDestination, PdfDestinationFit, PdfDictEntry, PdfExtGState, PdfFileSpecification, PdfFont, PdfLineCap, PdfLineJoin, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfFormFieldKind, PdfFunction, PdfFormXObject, PdfImage, PdfImageBody, PdfImageMask, PdfMarkupAnnotation, PdfMatrix, PdfNamedColorSpace, PdfNamedProperties, PdfObject, PdfPattern, PdfPatternKind, PdfSimpleEncoding, PdfOp, PdfPage, PdfShadingKind, PdfTextArrayItem, PdfTextString, PdfTransparencyGroup, PdfViewerPreferences, PDF_IDENTITY_MATRIX};
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
use semio_plugin::FaultOrigin;
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
use semio_s_artifact_stdio_pdf_editor_document::*;
use semio_s_artifact_stdio_pdf_page_editor_view::*;
use serde_json::{json, Value};


#[path = "../../../🧱️model/🦀️.rs"]
mod model;
pub use self::model::*;

#[path = "../../../🗣️labels/🦀️.rs"]
mod labels;
pub use self::labels::*;

#[path = "../../../🗺️scene/🦀️.rs"]
mod scene;
pub use self::scene::*;

#[path = "../../../🧮️content-ops/🦀️.rs"]
mod content_ops;
pub use self::content_ops::*;

#[path = "../../../🔤️metrics/🦀️.rs"]
mod metrics;
pub use self::metrics::*;

#[path = "../../../🧭️geometry/🦀️.rs"]
mod geometry;
pub use self::geometry::*;

#[path = "../../../🧰️arguments/🦀️.rs"]
mod arguments;
pub use self::arguments::*;

#[path = "../../../🖌️graphics/🦀️.rs"]
mod graphics;
pub use self::graphics::*;

#[path = "../../../🧮️function-colour/🦀️.rs"]
mod function_colour;
pub use self::function_colour::*;

#[path = "../../../🌄️shading-preview/🦀️.rs"]
mod shading_preview;
pub use self::shading_preview::*;

#[path = "../../../🔆️image-preview/🦀️.rs"]
mod image_preview;
pub use self::image_preview::*;
