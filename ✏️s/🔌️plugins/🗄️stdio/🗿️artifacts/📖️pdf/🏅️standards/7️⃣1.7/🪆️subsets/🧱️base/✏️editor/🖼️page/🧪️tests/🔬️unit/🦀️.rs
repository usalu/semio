use crate::standards::v1_7::subsets::base::schema::mutations::set_font::SetFont;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotation, PdfAppearanceEntry, PdfBaseEncoding, PdfColorSpace, PdfDestination, PdfEncryptionAlgorithm, PdfFont, PdfLineCap, PdfFontKind, PdfFontProgram, PdfFormFieldKind, PdfFunction, PdfFormXObject, PdfImageBody, PdfImageMask, PdfObject, PdfOpenAction, PdfPatternKind, PdfSimpleEncoding, PdfPageLabelStyle, PdfShadingKind};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use semio_ui_locale::Locale;
use serde_json::Value;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::diff::PdfPageBox;
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::mutations::{insert_object::InsertObject, insert_page::InsertPage, move_page::MovePage, remove_catalog_entry::RemoveCatalogEntry, remove_object::RemoveObject, remove_embedded_file::RemoveEmbeddedFile, remove_named_destination::RemoveNamedDestination, remove_page::RemovePage, replace_page::ReplacePage, remove_trailer_entry::RemoveTrailerEntry, set_annotation::SetAnnotation, set_embedded_file::SetEmbeddedFile, set_acro_form::SetAcroForm, set_document_id::SetDocumentId, set_color_space::SetColorSpace, set_catalog_entry::SetCatalogEntry, set_encryption::SetEncryption, set_ext_g_state::SetExtGState, set_form::SetForm, set_trailer_entry::SetTrailerEntry, set_open_action::SetOpenAction, set_output_intents::SetOutputIntents, set_pattern::SetPattern, set_properties::SetProperties, set_image::SetImage, set_info::SetInfo, set_language::SetLanguage, set_mark_info::SetMarkInfo, set_metadata::SetMetadata, set_named_destination::SetNamedDestination, set_object_value::SetObjectValue, set_optional_content::SetOptionalContent, set_outlines::SetOutlines, set_page_box::SetPageBox, set_page_content::SetPageContent, set_page_labels::SetPageLabels, set_page_layout::SetPageLayout, set_page_media_box::SetPageMediaBox, set_page_mode::SetPageMode, set_page_rotation::SetPageRotation, set_page_user_unit::SetPageUserUnit, set_shading::SetShading, set_viewer_preferences::SetViewerPreferences, PdfMutation};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfAction, PdfAppearance, PdfBorderStyle, PdfCharProc, PdfDate, PdfDestinationFit, PdfDictEntry, PdfEmbeddedFile, PdfEncryption, PdfExtGState, PdfFileSpecification, PdfLineJoin, PdfFontDescriptor, PdfFormField, PdfMarkInfo, PdfMarkupAnnotation, PdfMatrix, PdfNamedColorSpace, PdfNamedDestination, PdfNamedProperties, PdfOutputIntent, PdfPattern, PdfOptionalContentGroup, PdfOutlineItem, PdfPageLabelRange, PdfTextArrayItem, PdfTransparencyGroup, PdfViewerPreferences, PDF_IDENTITY_MATRIX};
#[allow(unused_imports)]
use semio_plugin::plugin_app_close_prelude as ui;
#[allow(unused_imports)]
use semio_plugin::ActionArgDef;
#[allow(unused_imports)]
use semio_plugin::ActionDefinition;
#[allow(unused_imports)]
use semio_plugin::ActionId;
#[allow(unused_imports)]
use semio_plugin::ActionKind;
#[allow(unused_imports)]
use semio_plugin::Buildable;
#[allow(unused_imports)]
use semio_plugin::Canvas2dScene;
#[allow(unused_imports)]
use semio_plugin::Fault;
#[allow(unused_imports)]
use semio_plugin::FaultCode;
#[allow(unused_imports)]
use semio_plugin::FaultOrigin;
#[allow(unused_imports)]
use semio_plugin::HasBase;
#[allow(unused_imports)]
use semio_plugin::HasChildren;
#[allow(unused_imports)]
use semio_ui_locale::LocalizedLabel;
#[allow(unused_imports)]
use semio_plugin::PluginAssemblyError;
#[allow(unused_imports)]
use semio_plugin::SurfaceKind;
#[allow(unused_imports)]
use semio_plugin::UiAssemblyResult;
#[allow(unused_imports)]
use semio_plugin::UiMapBuilder;
#[allow(unused_imports)]
use semio_plugin::UiText;
#[allow(unused_imports)]
use semio_plugin::UiValue;
#[allow(unused_imports)]
use semio_plugin::WindowKindDefinition;
#[allow(unused_imports)]
use semio_plugin::WindowLayout;
#[allow(unused_imports)]
#[allow(unused_imports)]
#[allow(unused_imports)]
#[allow(unused_imports)]
#[allow(unused_imports)]
#[allow(unused_imports)]
use semio_plugin::WindowOptions;
#[allow(unused_imports)]
use serde_json::json;
use crate::editor::page::{inspector};
use crate::editor::page::*;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotationKind, PdfImage, PdfOp, PdfPage, PdfPageLayout, PdfPageMode, PdfTextString};

fn apply(snapshot: &mut PdfSnapshot, edit: PdfPageEdit) {
    let mutations = apply_payload(snapshot, &edit.action, &edit.payload).expect("page edit");
    for mutation in &mutations {
        apply_pdf_mutation(snapshot, mutation);
    }
}

fn args(entries: Vec<(&str, semio_framework_value::DslValue)>) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn sample() -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    let mut page = PdfPage::new(200.0, 300.0);
    page.content = vec![
        PdfOp::SetFillRgb { r: 0.2, g: 0.3, b: 0.4 },
        PdfOp::Rectangle { x: 10.0, y: 20.0, width: 30.0, height: 40.0 },
        PdfOp::Fill,
        PdfOp::Save,
        PdfOp::Transform { matrix: [50.0, 0.0, 0.0, 40.0, 60.0, 70.0] },
        PdfOp::PaintXObject { name: "Im1".into() },
        PdfOp::Restore,
        PdfOp::BeginText,
        PdfOp::SetFont { name: "F1".into(), size: 12.0 },
        PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 15.0, 250.0] },
        PdfOp::ShowText { text: PdfTextString::text("Hello") },
        PdfOp::EndText,
    ];
    snapshot.pages.push(page);
    snapshot.images.push(PdfImage::rgb8("Im1", 2, 2, vec![255; 12]));
    snapshot.fonts.push(crate::standards::v1_7::subsets::base::schema::snapshot::PdfFont::standard("F1", "Helvetica"));
    snapshot
}

#[test]
fn page_lists_text_vector_and_image() {
    let snapshot = sample();
    let found = objects(&snapshot);
    let text = found.iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let vector = found.iter().find(|object| object.kind == ObjectKind::Vector).expect("vector");
    let image = found.iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    assert_eq!(text.text, "Hello");
    assert!((text.x - 15.0).abs() < 0.01 && (text.y - 250.0).abs() < 0.01);
    assert!((vector.x - 10.0).abs() < 0.01 && (vector.y - 20.0).abs() < 0.01);
    assert!((vector.width - 30.0).abs() < 0.01 && (vector.height - 40.0).abs() < 0.01);
    assert!((image.x - 60.0).abs() < 0.01 && (image.y - 70.0).abs() < 0.01);
    assert!((image.width - 50.0).abs() < 0.01 && (image.height - 40.0).abs() < 0.01);
    let layers = canvas_layers(&snapshot, Some(&text.id));
    assert!(layers.contains(&format!("\"id\":\"{}\"", text.id)));
    assert!(layers.contains(&format!("{}.selection", text.id)));
    assert!(layers.contains(&image.id));
    assert!(layers.contains(&vector.id));
    let hit = hit_test(&snapshot, text.x + 2.0, 300.0 - (text.y + text.font_size) + 2.0).expect("text hit");
    assert_eq!(hit, text.id);
}

#[test]
fn set_text_replaces_the_run_and_keeps_the_rectangle() {
    let mut snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let edit = edit_from_action("set-text", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(text.id)), ("text", semio_framework_value::DslValue::String("World".into()))]))).expect("edit");
    apply(&mut snapshot, edit);
    assert_eq!(snapshot.pages[0].text(), "World");
    assert!(snapshot.pages[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { .. })));
}

#[test]
fn move_resize_and_delete_address_text_vectors_and_images() {
    let mut snapshot = sample();
    let found = objects(&snapshot);
    let text = found.iter().find(|object| object.kind == ObjectKind::Text).unwrap().clone();
    let vector = found.iter().find(|object| object.kind == ObjectKind::Vector).unwrap().clone();
    let image = found.iter().find(|object| object.kind == ObjectKind::Image).unwrap().clone();
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(text.id)), ("x", semio_framework_value::DslValue::float(20.0)), ("y", semio_framework_value::DslValue::float(240.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(vector.id.clone())), ("x", semio_framework_value::DslValue::float(12.0)), ("y", semio_framework_value::DslValue::float(22.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(image.id)), ("x", semio_framework_value::DslValue::float(62.0)), ("y", semio_framework_value::DslValue::float(72.0))]))).unwrap());
    let moved = objects(&snapshot);
    let text = moved.iter().find(|object| object.kind == ObjectKind::Text).unwrap();
    let vector = moved.iter().find(|object| object.kind == ObjectKind::Vector).unwrap();
    let image = moved.iter().find(|object| object.kind == ObjectKind::Image).unwrap();
    assert!((text.x - 20.0).abs() < 0.01 && (text.y - 240.0).abs() < 0.01);
    assert!((vector.x - 12.0).abs() < 0.01 && (vector.y - 22.0).abs() < 0.01);
    assert!((image.x - 62.0).abs() < 0.01 && (image.y - 72.0).abs() < 0.01);
    let vector_id = vector.id.clone();
    apply(&mut snapshot, edit_from_action("delete", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(vector_id))]))).unwrap());
    assert!(objects(&snapshot).iter().all(|object| object.kind != ObjectKind::Vector));
    assert!(snapshot.pages[0].content.iter().any(|op| matches!(op, PdfOp::ShowText { .. })));
    assert!(snapshot.pages[0].content.iter().any(|op| matches!(op, PdfOp::PaintXObject { .. })));
}

#[test]
fn insert_text_rectangle_and_image_round_trip_onto_the_page() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("insert-text", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(4.0)), ("y", semio_framework_value::DslValue::float(8.0)), ("text", semio_framework_value::DslValue::String("Added".into())), ("height", semio_framework_value::DslValue::float(14.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("insert-rectangle", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(1.0)), ("y", semio_framework_value::DslValue::float(2.0)), ("width", semio_framework_value::DslValue::float(3.0)), ("height", semio_framework_value::DslValue::float(4.0)), ("red", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("insert-image", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(5.0)), ("y", semio_framework_value::DslValue::float(6.0)), ("width", semio_framework_value::DslValue::float(7.0)), ("height", semio_framework_value::DslValue::float(8.0))]))).unwrap());
    let found = objects(&snapshot);
    assert!(found.iter().any(|object| object.text == "Added"));
    assert!(found.iter().any(|object| object.kind == ObjectKind::Vector && (object.width - 3.0).abs() < 0.01));
    assert_eq!(snapshot.images.len(), 2);
    assert!(found.iter().any(|object| object.kind == ObjectKind::Image && (object.x - 5.0).abs() < 0.01));
}

#[test]
fn document_info_pages_and_annotations_are_editable() {
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(crate::standards::v1_7::subsets::base::schema::snapshot::PdfAnnotation::new([1.0, 2.0, 11.0, 12.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    apply(&mut snapshot, edit_from_action("set-info", Some(&args(vec![("text", semio_framework_value::DslValue::String("Title".into())), ("extra", semio_framework_value::DslValue::String("Author".into())), ("object", semio_framework_value::DslValue::String("Subject".into()))]))).unwrap());
    assert_eq!(snapshot.info.title.as_deref(), Some("Title"));
    assert_eq!(snapshot.info.author.as_deref(), Some("Author"));
    assert_eq!(snapshot.info.subject.as_deref(), Some("Subject"));
    apply(&mut snapshot, edit_from_action("insert-page", Some(&args(vec![("page", semio_framework_value::DslValue::float(1.0)), ("width", semio_framework_value::DslValue::float(100.0)), ("height", semio_framework_value::DslValue::float(120.0))]))).unwrap());
    assert_eq!(snapshot.pages.len(), 2);
    assert!((snapshot.pages[1].width() - 100.0).abs() < 0.01);
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation");
    apply(&mut snapshot, edit_from_action("set-annotation", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(annotation.id)), ("text", semio_framework_value::DslValue::String("Note".into())), ("x", semio_framework_value::DslValue::float(3.0)), ("y", semio_framework_value::DslValue::float(4.0)), ("width", semio_framework_value::DslValue::float(5.0)), ("height", semio_framework_value::DslValue::float(6.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].annotations[0].contents.as_deref(), Some("Note"));
    assert_eq!(snapshot.pages[0].annotations[0].rect, [3.0, 4.0, 8.0, 10.0]);
    apply(&mut snapshot, edit_from_action("set-page-size", Some(&args(vec![("page", semio_framework_value::DslValue::float(1.0)), ("width", semio_framework_value::DslValue::float(90.0)), ("height", semio_framework_value::DslValue::float(80.0))]))).unwrap());
    assert!((snapshot.pages[1].width() - 90.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("move-page", Some(&args(vec![("page", semio_framework_value::DslValue::float(1.0)), ("x", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    assert!((snapshot.pages[0].width() - 90.0).abs() < 0.01);
}

#[test]
fn set_image_replaces_samples_and_keeps_the_placement() {
    let mut snapshot = sample();
    let image = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    let edit = edit_from_action(
        "set-image",
        Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(image.id)), ("width", semio_framework_value::DslValue::float(2.0)), ("height", semio_framework_value::DslValue::float(2.0)), ("text", semio_framework_value::DslValue::String("255,0,0,0,255,0,0,0,255,255,0,0".into()))])),
    )
    .expect("edit");
    apply(&mut snapshot, edit);
    assert_eq!(snapshot.images[0].body,crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody::Samples {values:vec![255,0,0,0,255,0,0,0,255,255,0,0]});
    let image = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    assert!((image.x - 60.0).abs() < 0.01 && (image.y - 70.0).abs() < 0.01);
}

#[test]
fn form_move_and_shading_move_change_their_geometry() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfFormXObject, PdfFunction, PdfShading, PdfShadingKind};
    let mut snapshot = sample();
    snapshot.forms.push(PdfFormXObject::new("Fm1", [0.0, 0.0, 1.0, 1.0], Vec::new()));
    snapshot.pages[0].content.extend([
        PdfOp::Save,
        PdfOp::Transform { matrix: [20.0, 0.0, 0.0, 10.0, 8.0, 9.0] },
        PdfOp::PaintXObject { name: "Fm1".into() },
        PdfOp::Restore,
        PdfOp::PaintShading { name: "Sh1".into() },
    ]);
    snapshot.shadings.push(PdfShading {
        id: "Sh1".into(),
        color_space: PdfColorSpace::DeviceRgb,
        kind: PdfShadingKind::Axial { coords: [10.0, 20.0, 40.0, 20.0], domain: None, function: PdfFunction::Exponential { domain: vec![0.0, 1.0], range: None, c0: vec![0.0, 0.0, 0.0], c1: vec![1.0, 1.0, 1.0], n: 1.0 }, extend: [false, false] },
        background: None,
        bbox: Some([10.0, 10.0, 40.0, 30.0]),
        anti_alias: false,
        extra: Vec::new(),
    });
    let form = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Form).expect("form");
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(form.id)), ("x", semio_framework_value::DslValue::float(10.0)), ("y", semio_framework_value::DslValue::float(11.0))]))).unwrap());
    let form = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Form).expect("form");
    assert!((form.x - 10.0).abs() < 0.01 && (form.y - 11.0).abs() < 0.01);
    let shading = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Shading).expect("shading");
    assert!((shading.x - 10.0).abs() < 0.01 && (shading.y - 10.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(shading.id)), ("x", semio_framework_value::DslValue::float(12.0)), ("y", semio_framework_value::DslValue::float(14.0))]))).unwrap());
    let PdfShadingKind::Axial { coords, .. } = &snapshot.shadings[0].kind else { panic!("axial shading") };
    assert!((coords[0] - 12.0).abs() < 0.01 && (coords[1] - 24.0).abs() < 0.01);
    assert_eq!(snapshot.shadings[0].bbox, Some([12.0, 14.0, 42.0, 34.0]));
}

#[test]
fn set_font_and_set_outline_publish_onto_the_document() {
    let mut snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    apply(&mut snapshot, edit_from_action("set-font", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(text.id)), ("text", semio_framework_value::DslValue::String("Times-Roman".into()))]))).unwrap());
    assert!(snapshot.fonts.iter().any(|font| font.base_font() == "Times-Roman"));
    let show = snapshot.pages[0].content.iter().position(|op| matches!(op, PdfOp::ShowText { .. })).expect("show");
    let PdfOp::SetFont { name, size } = &snapshot.pages[0].content[show - 1] else { panic!("font before the run") };
    assert_eq!(snapshot.fonts.iter().find(|font| font.id == *name).expect("font").base_font(), "Times-Roman");
    assert!((*size - 12.0).abs() < 0.01);
    assert_eq!(snapshot.pages[0].text(), "Hello");
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("Start".into()))]))).unwrap());
    assert_eq!(snapshot.outlines[0].title, "Start");
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("Begin".into()))]))).unwrap());
    assert_eq!(snapshot.outlines.len(), 1);
    assert_eq!(snapshot.outlines[0].title, "Begin");
}

#[test]
fn page_viewing_settings_publish_onto_the_document() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-page-rotation", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(90.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].rotate, 90);
    apply(&mut snapshot, edit_from_action("set-page-box", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("crop".into())), ("x", semio_framework_value::DslValue::float(1.0)), ("y", semio_framework_value::DslValue::float(2.0)), ("width", semio_framework_value::DslValue::float(30.0)), ("height", semio_framework_value::DslValue::float(40.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].crop_box, Some([1.0, 2.0, 31.0, 42.0]));
    apply(&mut snapshot, edit_from_action("set-page-user-unit", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(2.5))]))).unwrap());
    assert_eq!(snapshot.pages[0].user_unit, Some(2.5));
    apply(&mut snapshot, edit_from_action("set-language", Some(&args(vec![("text", semio_framework_value::DslValue::String("de".into()))]))).unwrap());
    assert_eq!(snapshot.language.as_deref(), Some("de"));
    apply(&mut snapshot, edit_from_action("set-page-layout", Some(&args(vec![("text", semio_framework_value::DslValue::String("twoColumnLeft".into()))]))).unwrap());
    assert_eq!(snapshot.page_layout, Some(PdfPageLayout::TwoColumnLeft));
    apply(&mut snapshot, edit_from_action("set-page-mode", Some(&args(vec![("text", semio_framework_value::DslValue::String("useOutlines".into()))]))).unwrap());
    assert_eq!(snapshot.page_mode, Some(PdfPageMode::UseOutlines));
    let layers = canvas_layers(&snapshot, None);
    assert!(layers.contains("p0:paper"));
}

#[test]
fn catalog_collections_publish_onto_the_document() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDestination, PdfEncryptionAlgorithm, PdfPageLabelStyle};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-optional-content", Some(&args(vec![("object", semio_framework_value::DslValue::String("OC1".into())), ("text", semio_framework_value::DslValue::String("Layer".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    let content = snapshot.optional_content.as_ref().expect("layers");
    assert_eq!(content.groups[0].name, "Layer");
    assert_eq!(content.on, vec!["OC1".to_string()]);
    apply(&mut snapshot, edit_from_action("set-embedded-file", Some(&args(vec![("object", semio_framework_value::DslValue::String("note".into())), ("text", semio_framework_value::DslValue::String("note.txt".into())), ("extra", semio_framework_value::DslValue::String("hello".into()))]))).unwrap());
    assert_eq!(snapshot.embedded_files[0].data, b"hello");
    apply(&mut snapshot, edit_from_action("remove-embedded-file", Some(&args(vec![("object", semio_framework_value::DslValue::String("note".into()))]))).unwrap());
    assert!(snapshot.embedded_files.is_empty());
    apply(&mut snapshot, edit_from_action("set-named-destination", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("Start".into()))]))).unwrap());
    assert!(matches!(snapshot.named_destinations[0].destination, PdfDestination::Page { page: 0, .. }));
    apply(&mut snapshot, edit_from_action("set-page-label", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("decimal".into())), ("extra", semio_framework_value::DslValue::String("A-".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    assert_eq!(snapshot.page_labels[0].style, Some(PdfPageLabelStyle::Decimal));
    assert_eq!(snapshot.page_labels[0].prefix.as_deref(), Some("A-"));
    apply(&mut snapshot, edit_from_action("set-mark-info", Some(&args(vec![("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    assert!(snapshot.mark_info.as_ref().expect("marks").marked);
    apply(&mut snapshot, edit_from_action("set-metadata", Some(&args(vec![("text", semio_framework_value::DslValue::String("<xmp/>".into()))]))).unwrap());
    assert_eq!(snapshot.metadata.as_deref(), Some("<xmp/>"));
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", semio_framework_value::DslValue::String("hideToolbar".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    assert!(snapshot.viewer_preferences.as_ref().expect("viewer").hide_toolbar);
    apply(&mut snapshot, edit_from_action("set-encryption", Some(&args(vec![("text", semio_framework_value::DslValue::String("aes256".into())), ("extra", semio_framework_value::DslValue::String("user".into())), ("object", semio_framework_value::DslValue::String("owner".into()))]))).unwrap());
    let encryption = snapshot.encryption.as_ref().expect("encryption");
    assert_eq!(encryption.algorithm, PdfEncryptionAlgorithm::Aes256);
    assert_eq!(encryption.user_password, "user");
    assert_eq!(encryption.owner_password.as_deref(), Some("owner"));
    apply(&mut snapshot, edit_from_action("remove-named-destination", Some(&args(vec![("object", semio_framework_value::DslValue::String("Start".into()))]))).unwrap());
    assert!(snapshot.named_destinations.is_empty());
}

#[test]
fn mesh_move_shifts_the_decode_range() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfShading, PdfShadingKind};
    let mut snapshot = sample();
    snapshot.pages[0].content.push(PdfOp::PaintShading { name: "ShM".into() });
    snapshot.shadings.push(PdfShading { id: "ShM".into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Mesh { shading_type: 4, bits_per_coordinate: 8, bits_per_component: 8, bits_per_flag: None, vertices_per_row: None, decode: vec![0.0, 10.0, 0.0, 10.0], function: None, reference: semio_framework_artifact_reference::ArtifactRef { artifact_id: "fixture:mesh".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.binary".into(), standard: "raw".into(), subset: "*".into() } } }, background: None, bbox: Some([0.0, 0.0, 10.0, 10.0]), anti_alias: false, extra: Vec::new() });
    let shading = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Shading).expect("mesh");
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(shading.id)), ("x", semio_framework_value::DslValue::float(2.0)), ("y", semio_framework_value::DslValue::float(3.0))]))).unwrap());
    let PdfShadingKind::Mesh { decode, reference, .. } = &snapshot.shadings[0].kind else { panic!("mesh") };
    assert_eq!(decode, &vec![2.0, 12.0, 3.0, 13.0]);
    assert_eq!(reference.artifact_id, "fixture:mesh");
    assert_eq!(snapshot.shadings[0].bbox, Some([2.0, 3.0, 12.0, 13.0]));
}

#[test]
fn conformance_form_identity_and_font_program_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfFontKind, PdfFontProgram, PdfFormFieldKind, PdfOpenAction};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-output-intent", Some(&args(vec![("object", semio_framework_value::DslValue::String("GTS_PDFA1".into())), ("text", semio_framework_value::DslValue::String("sRGB".into())), ("extra", semio_framework_value::DslValue::String("sRGB IEC61966".into()))]))).unwrap());
    assert_eq!(snapshot.output_intents[0].condition_identifier, "sRGB");
    assert_eq!(snapshot.output_intents[0].info.as_deref(), Some("sRGB IEC61966"));
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", semio_framework_value::DslValue::String("Name".into())), ("text", semio_framework_value::DslValue::String("Ada".into())), ("extra", semio_framework_value::DslValue::String("text".into()))]))).unwrap());
    let form = snapshot.acro_form.as_ref().expect("form");
    assert!(form.need_appearances);
    let PdfFormFieldKind::Text { value, .. } = &form.fields[0].kind else { panic!("text field") };
    assert_eq!(value.as_deref(), Some("Ada"));
    apply(&mut snapshot, edit_from_action("set-open-action", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("uri".into())), ("text", semio_framework_value::DslValue::String("https://semio.example".into()))]))).unwrap());
    let PdfOpenAction::Action { action } = snapshot.open_action.as_ref().expect("open") else { panic!("uri action") };
    assert!(matches!(action.kind, crate::standards::v1_7::subsets::base::schema::snapshot::PdfActionKind::Uri { ref uri, .. } if uri == "https://semio.example"));
    apply(&mut snapshot, edit_from_action("set-document-id", Some(&args(vec![("text", semio_framework_value::DslValue::String("abc".into())), ("extra", semio_framework_value::DslValue::String("def".into()))]))).unwrap());
    assert_eq!(snapshot.document_id.as_ref().expect("id"), &[b"abc".to_vec(), b"def".to_vec()]);
    apply(&mut snapshot, edit_from_action("set-font-reference", Some(&args(vec![("object", semio_framework_value::DslValue::String("F1".into())), ("text", semio_framework_value::DslValue::String("truetype".into())), ("extra", semio_framework_value::DslValue::String("0001".into()))]))).unwrap());
    let PdfFontKind::TrueType { program, .. } = &snapshot.fonts[0].kind else { panic!("truetype font") };
    let Some(PdfFontProgram::TrueType { reference }) = program else { panic!("program") };
    assert_eq!(reference.artifact_id,"0001");
}

#[test]
fn resources_and_font_metrics_publish_onto_the_document() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfColorSpace, PdfFontKind, PdfObject, PdfPatternKind};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-graphics-state", Some(&args(vec![("object", semio_framework_value::DslValue::String("GS1".into())), ("text", semio_framework_value::DslValue::String("Multiply".into())), ("x", semio_framework_value::DslValue::float(0.5)), ("y", semio_framework_value::DslValue::float(0.25))]))).unwrap());
    assert_eq!(snapshot.ext_g_states[0].fill_alpha, Some(0.5));
    assert_eq!(snapshot.ext_g_states[0].stroke_alpha, Some(0.25));
    assert_eq!(snapshot.ext_g_states[0].blend_mode.as_deref(), Some(["Multiply".to_string()].as_slice()));
    apply(&mut snapshot, edit_from_action("set-pattern", Some(&args(vec![("object", semio_framework_value::DslValue::String("P1".into())), ("x", semio_framework_value::DslValue::float(0.0)), ("y", semio_framework_value::DslValue::float(0.0)), ("width", semio_framework_value::DslValue::float(8.0)), ("height", semio_framework_value::DslValue::float(8.0))]))).unwrap());
    let PdfPatternKind::Tiling { x_step, y_step, .. } = &snapshot.patterns[0].kind else { panic!("tiling") };
    assert!((*x_step - 8.0).abs() < 0.01 && (*y_step - 8.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("set-color-space", Some(&args(vec![("object", semio_framework_value::DslValue::String("CS1".into())), ("text", semio_framework_value::DslValue::String("deviceCmyk".into()))]))).unwrap());
    assert!(matches!(snapshot.color_spaces[0].color_space, PdfColorSpace::DeviceCmyk));
    apply(&mut snapshot, edit_from_action("set-properties", Some(&args(vec![("object", semio_framework_value::DslValue::String("MC1".into())), ("text", semio_framework_value::DslValue::String("OC".into())), ("extra", semio_framework_value::DslValue::String("OC1".into()))]))).unwrap());
    assert!(matches!(&snapshot.properties[0].entries[0].value, PdfObject::Name(name) if name == "OC1"));
    apply(&mut snapshot, edit_from_action("set-font-metrics", Some(&args(vec![("object", semio_framework_value::DslValue::String("F1".into())), ("text", semio_framework_value::DslValue::String("macRoman".into())), ("extra", semio_framework_value::DslValue::String("500,600".into())), ("x", semio_framework_value::DslValue::float(32.0)), ("y", semio_framework_value::DslValue::float(720.0))]))).unwrap());
    let PdfFontKind::Type1 { encoding, first_char, widths, descriptor, .. } = &snapshot.fonts[0].kind else { panic!("simple font") };
    assert_eq!(encoding.base, Some(PdfBaseEncoding::MacRoman));
    assert_eq!(*first_char, 32);
    assert_eq!(widths, &vec![500.0, 600.0]);
    assert!((descriptor.as_ref().expect("descriptor").ascent - 720.0).abs() < 0.01);
}

#[test]
fn masks_forms_transitions_and_dictionary_entries_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfImageMask, PdfObject, PdfOp};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-image-mask", Some(&args(vec![("object", semio_framework_value::DslValue::String("Im1".into())), ("text", semio_framework_value::DslValue::String("colorKey".into())), ("extra", semio_framework_value::DslValue::String("0,255".into()))]))).unwrap());
    let PdfImageMask::ColorKey { ranges } = snapshot.images[0].mask.as_ref().expect("mask") else { panic!("color key") };
    assert_eq!(ranges, &vec![0, 255]);
    apply(&mut snapshot, edit_from_action("set-form-content", Some(&args(vec![("object", semio_framework_value::DslValue::String("Fm1".into())), ("text", semio_framework_value::DslValue::String("Inside".into())), ("x", semio_framework_value::DslValue::float(4.0)), ("y", semio_framework_value::DslValue::float(6.0)), ("width", semio_framework_value::DslValue::float(20.0)), ("height", semio_framework_value::DslValue::float(12.0))]))).unwrap());
    assert!(snapshot.forms[0].content.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "Inside")));
    apply(&mut snapshot, edit_from_action("set-page-transition", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("Wipe".into())), ("extra", semio_framework_value::DslValue::String("None".into())), ("x", semio_framework_value::DslValue::float(2.0))]))).unwrap());
    let transition = snapshot.pages[0].transition.as_ref().expect("transition");
    assert!(matches!(&transition[0].value, PdfObject::Name(name) if name == "Wipe"));
    assert_eq!(snapshot.pages[0].duration, Some(2.0));
    apply(&mut snapshot, edit_from_action("set-catalog-entry", Some(&args(vec![("object", semio_framework_value::DslValue::String("Marker".into())), ("text", semio_framework_value::DslValue::String("Yes".into()))]))).unwrap());
    assert!(matches!(&snapshot.catalog_extra[0].value, PdfObject::Name(name) if name == "Yes"));
    apply(&mut snapshot, edit_from_action("set-trailer-entry", Some(&args(vec![("object", semio_framework_value::DslValue::String("Info".into())), ("text", semio_framework_value::DslValue::String("Kept".into()))]))).unwrap());
    assert!(matches!(&snapshot.trailer[0].value, PdfObject::Name(name) if name == "Kept"));
    apply(&mut snapshot, edit_from_action("set-catalog-entry", Some(&args(vec![("object", semio_framework_value::DslValue::String("Marker".into())), ("text", semio_framework_value::DslValue::String(String::new()))]))).unwrap());
    assert!(snapshot.catalog_extra.is_empty());
}

#[test]
fn appearances_glyphs_objects_and_mesh_bytes_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotation, PdfAnnotationKind, PdfAppearanceEntry, PdfColorSpace, PdfFontKind, PdfObject, PdfOp, PdfShading, PdfShadingKind, PdfSimpleEncoding};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation");
    apply(&mut snapshot, edit_from_action("set-annotation-appearance", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(annotation.id)), ("text", semio_framework_value::DslValue::String("Fm1".into()))]))).unwrap());
    let PdfAppearanceEntry::Single { form } = &snapshot.pages[0].annotations[0].appearance.as_ref().expect("appearance").normal else { panic!("single appearance") };
    assert_eq!(form, "Fm1");
    snapshot.fonts[0].kind = PdfFontKind::Type3 { font_matrix: [0.001, 0.0, 0.0, 0.001, 0.0, 0.0], font_bbox: [0.0, 0.0, 1000.0, 1000.0], encoding: PdfSimpleEncoding::default(), first_char: 0, widths: Vec::new(), char_procs: Vec::new(), descriptor: None };
    apply(&mut snapshot, edit_from_action("set-glyph", Some(&args(vec![("object", semio_framework_value::DslValue::String("F1".into())), ("text", semio_framework_value::DslValue::String("A".into())), ("x", semio_framework_value::DslValue::float(0.0)), ("y", semio_framework_value::DslValue::float(0.0)), ("width", semio_framework_value::DslValue::float(10.0)), ("height", semio_framework_value::DslValue::float(12.0))]))).unwrap());
    let PdfFontKind::Type3 { char_procs, .. } = &snapshot.fonts[0].kind else { panic!("type 3") };
    assert_eq!(char_procs[0].name, "A");
    assert!(char_procs[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { width, height, .. } if (*width - 10.0).abs() < 0.01 && (*height - 12.0).abs() < 0.01)));
    apply(&mut snapshot, edit_from_action("set-indirect-object", Some(&args(vec![("x", semio_framework_value::DslValue::float(7.0)), ("y", semio_framework_value::DslValue::float(0.0)), ("text", semio_framework_value::DslValue::String("Hello".into()))]))).unwrap());
    assert!(matches!(&snapshot.objects[0].value, PdfObject::Name(name) if name == "Hello"));
    assert_eq!(snapshot.objects[0].id.num, 7);
    snapshot.shadings.push(PdfShading { id: "ShM".into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Mesh { shading_type: 4, bits_per_coordinate: 8, bits_per_component: 8, bits_per_flag: None, vertices_per_row: None, decode: vec![0.0, 1.0, 0.0, 1.0], function: None, reference: semio_framework_artifact_reference::ArtifactRef { artifact_id: "fixture:mesh".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.binary".into(), standard: "raw".into(), subset: "*".into() } } }, background: None, bbox: None, anti_alias: false, extra: Vec::new() });
    apply(&mut snapshot, edit_from_action("set-mesh-reference", Some(&args(vec![("object", semio_framework_value::DslValue::String("ShM".into())), ("text", semio_framework_value::DslValue::String("0,10,0,10".into())), ("extra", semio_framework_value::DslValue::String("fixture:mesh:replacement".into()))]))).unwrap());
    let PdfShadingKind::Mesh { reference, decode, .. } = &snapshot.shadings[0].kind else { panic!("mesh") };
    assert_eq!(reference.artifact_id, "fixture:mesh:replacement");
    assert_eq!(decode, &vec![0.0, 10.0, 0.0, 10.0]);
}

#[test]
fn pointer_down_selects_the_hit_object_and_clears_a_miss() {
    let snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let x = text.x + 2.0;
    let y = 300.0 - (text.y + text.font_size) + 2.0;
    let edit = edit_from_action("canvasPointerDown", Some(&args(vec![("x", semio_framework_value::DslValue::float(x)), ("y", semio_framework_value::DslValue::float(y))]))).expect("pointer");
    let emit = emit_page_edit(&snapshot, &edit.action, &edit.payload).expect("select");
    assert!(emit.artifact_mutations.is_empty());
    let semio_plugin::Effect::DispatchAction { action, args: effect_args, .. } = &emit.effects[0] else { panic!("select effect") };
    assert_eq!(action, "interactionSelect");
    let value = serde_json::Value::from(effect_args.clone().expect("args"));
    assert_eq!(value["domainId"], "objects");
    assert_eq!(value["merge"], "replace");
    assert!(value["targets"].as_str().expect("targets").contains(&text.id));
    let miss = edit_from_action("canvasPointerDown", Some(&args(vec![("x", semio_framework_value::DslValue::float(-1.0)), ("y", semio_framework_value::DslValue::float(1.0))]))).expect("miss");
    let cleared = emit_page_edit(&snapshot, &miss.action, &miss.payload).expect("clear");
    let semio_plugin::Effect::DispatchAction { action, args: cleared_args, .. } = &cleared.effects[0] else { panic!("clear effect") };
    assert_eq!(action, "clearSelection");
    assert!(cleared_args.is_none());
    let shifted = edit_from_action("canvasPointerDown", Some(&args(vec![("x", semio_framework_value::DslValue::float(x)), ("y", semio_framework_value::DslValue::float(y)), ("shift", semio_framework_value::DslValue::Bool(true))]))).expect("shift");
    let toggled = emit_page_edit(&snapshot, &shifted.action, &shifted.payload).expect("invert");
    let semio_plugin::Effect::DispatchAction { args: effect_args, .. } = &toggled.effects[0] else { panic!("invert effect") };
    let value = serde_json::Value::from(effect_args.clone().expect("args"));
    assert_eq!(value["merge"], "invertive");
    let moved = emit_page_edit(&snapshot, "canvasPointerMove", &edit.payload).expect("move");
    assert!(moved.effects.is_empty() && moved.artifact_mutations.is_empty());
}

#[test]
fn info_page_extras_annotation_style_and_viewer_details_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotation, PdfColorSpace, PdfPageMode};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation");
    apply(&mut snapshot, edit_from_action("set-info-field", Some(&args(vec![("object", semio_framework_value::DslValue::String("keywords".into())), ("text", semio_framework_value::DslValue::String("paper".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-info-field", Some(&args(vec![("object", semio_framework_value::DslValue::String("creationDate".into())), ("text", semio_framework_value::DslValue::String("D:20200102".into()))]))).unwrap());
    assert_eq!(snapshot.info.keywords.as_deref(), Some("paper"));
    assert_eq!(snapshot.info.creation_date.as_ref().expect("date").year, 2020);
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("thumbnail".into())), ("text", semio_framework_value::DslValue::String("Im1".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("metadata".into())), ("text", semio_framework_value::DslValue::String("<xmp/>".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("group".into())), ("text", semio_framework_value::DslValue::String("deviceRgb".into())), ("x", semio_framework_value::DslValue::float(1.0)), ("y", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("structParents".into())), ("x", semio_framework_value::DslValue::float(4.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String("action".into())), ("extra", semio_framework_value::DslValue::String("O".into())), ("text", semio_framework_value::DslValue::String("Next".into()))]))).unwrap());
    assert_eq!(snapshot.pages[0].thumbnail.as_deref(), Some("Im1"));
    assert_eq!(snapshot.pages[0].metadata.as_deref(), Some("<xmp/>"));
    let group = snapshot.pages[0].group.as_ref().expect("group");
    assert_eq!(group.color_space, Some(PdfColorSpace::DeviceRgb));
    assert!(group.isolated && !group.knockout);
    assert_eq!(snapshot.pages[0].struct_parents, Some(4));
    assert_eq!(snapshot.pages[0].additional_actions[0].key, "O");
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(annotation.id.clone())), ("text", semio_framework_value::DslValue::String("color".into())), ("red", semio_framework_value::DslValue::float(1.0)), ("green", semio_framework_value::DslValue::float(0.0)), ("blue", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(annotation.id.clone())), ("text", semio_framework_value::DslValue::String("name".into())), ("extra", semio_framework_value::DslValue::String("Note".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(annotation.id)), ("text", semio_framework_value::DslValue::String("flags".into())), ("x", semio_framework_value::DslValue::float(4.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].annotations[0].color, vec![1.0, 0.0, 0.0]);
    assert_eq!(snapshot.pages[0].annotations[0].name.as_deref(), Some("Note"));
    assert_eq!(snapshot.pages[0].annotations[0].flags, 4);
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", semio_framework_value::DslValue::String("direction".into())), ("text", semio_framework_value::DslValue::String("R2L".into())), ("x", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", semio_framework_value::DslValue::String("nonFullScreenPageMode".into())), ("text", semio_framework_value::DslValue::String("useThumbs".into())), ("x", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", semio_framework_value::DslValue::String("printPageRange".into())), ("text", semio_framework_value::DslValue::String("1,3".into())), ("x", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    let preferences = snapshot.viewer_preferences.as_ref().expect("viewer");
    assert_eq!(preferences.direction.as_deref(), Some("R2L"));
    assert_eq!(preferences.non_full_screen_page_mode, Some(PdfPageMode::UseThumbs));
    assert_eq!(preferences.print_page_range, vec![1, 3]);
}

#[test]
fn annotation_markup_form_settings_and_extra_entries_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotation, PdfObject};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation").id;
    let page = semio_framework_value::DslValue::float(0.0);
    let object = semio_framework_value::DslValue::String(annotation);
    apply(&mut snapshot, edit_from_action("set-annotation-border", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("D".into())), ("extra", semio_framework_value::DslValue::String("3,1".into())), ("x", semio_framework_value::DslValue::float(2.0)), ("y", semio_framework_value::DslValue::float(1.0)), ("width", semio_framework_value::DslValue::float(2.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("title".into())), ("extra", semio_framework_value::DslValue::String("Reviewer".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("opacity".into())), ("x", semio_framework_value::DslValue::float(0.4))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("layer".into())), ("extra", semio_framework_value::DslValue::String("OC1".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page), ("object", object), ("text", semio_framework_value::DslValue::String("entry".into())), ("extra", semio_framework_value::DslValue::String("OC=On".into()))]))).unwrap());
    let note = &snapshot.pages[0].annotations[0];
    let border = note.border.as_ref().expect("border");
    assert!((border.width - 2.0).abs() < 0.01 && border.style.as_deref() == Some("D") && border.dash.as_deref() == Some(&[3.0, 1.0][..]) && border.radii == Some([1.0, 2.0]));
    let markup = note.markup.as_ref().expect("markup");
    assert_eq!(markup.title.as_deref(), Some("Reviewer"));
    assert_eq!(markup.opacity, Some(0.4));
    assert_eq!(note.optional_content.as_deref(), Some("OC1"));
    assert!(matches!(&note.extra[0].value, PdfObject::Name(name) if name == "On"));
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", semio_framework_value::DslValue::String("Name".into())), ("text", semio_framework_value::DslValue::String("Ada".into())), ("extra", semio_framework_value::DslValue::String("text".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", semio_framework_value::DslValue::String("signatureFlags".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", semio_framework_value::DslValue::String("defaultAppearance".into())), ("text", semio_framework_value::DslValue::String("/Helv 0 Tf".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", semio_framework_value::DslValue::String("quadding".into())), ("x", semio_framework_value::DslValue::float(2.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", semio_framework_value::DslValue::String("defaultFonts".into())), ("text", semio_framework_value::DslValue::String("F1,F2".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", semio_framework_value::DslValue::String("Name".into())), ("text", semio_framework_value::DslValue::String("appearance".into())), ("extra", semio_framework_value::DslValue::String("/Helv 12 Tf".into()))]))).unwrap());
    let form = snapshot.acro_form.as_ref().expect("form");
    assert_eq!(form.signature_flags, 1);
    assert_eq!(form.default_appearance.as_deref(), Some("/Helv 0 Tf"));
    assert_eq!(form.quadding, Some(2));
    assert_eq!(form.default_fonts, vec!["F1".to_string(), "F2".to_string()]);
    assert_eq!(form.fields[0].default_appearance.as_deref(), Some("/Helv 12 Tf"));
    apply(&mut snapshot, edit_from_action("set-extra-entry", Some(&args(vec![("object", semio_framework_value::DslValue::String("info".into())), ("extra", semio_framework_value::DslValue::String("Custom".into())), ("text", semio_framework_value::DslValue::String("Kept".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-extra-entry", Some(&args(vec![("object", semio_framework_value::DslValue::String("viewer".into())), ("extra", semio_framework_value::DslValue::String("PrintArea".into())), ("text", semio_framework_value::DslValue::String("CropBox".into()))]))).unwrap());
    assert!(matches!(&snapshot.info.extra[0].value, PdfObject::Name(name) if name == "Kept"));
    assert!(matches!(&snapshot.viewer_preferences.as_ref().expect("viewer").extra[0].value, PdfObject::Name(name) if name == "CropBox"));
}

#[test]
fn annotation_kinds_and_choice_options_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfActionKind, PdfAnnotation, PdfAnnotationKind, PdfFormFieldKind};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation").id;
    let page = semio_framework_value::DslValue::float(0.0);
    let object = semio_framework_value::DslValue::String(annotation);
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("kind".into())), ("extra", semio_framework_value::DslValue::String("link".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("uri".into())), ("extra", semio_framework_value::DslValue::String("https://example.test".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("quads".into())), ("extra", semio_framework_value::DslValue::String("0,0,10,0,10,10,0,10".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Link { action, quad_points, .. } => {
            let PdfActionKind::Uri { uri, .. } = &action.as_ref().expect("uri").kind else { panic!("uri action") };
            assert_eq!(uri, "https://example.test");
            assert_eq!(quad_points, &vec![0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 0.0, 10.0]);
        }
        _ => panic!("link"),
    }
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("kind".into())), ("extra", semio_framework_value::DslValue::String("ink".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("paths".into())), ("extra", semio_framework_value::DslValue::String("0,0,8,0;8,0,8,8".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Ink { paths } => assert_eq!(paths, &vec![vec![0.0, 0.0, 8.0, 0.0], vec![8.0, 0.0, 8.0, 8.0]]),
        _ => panic!("ink"),
    }
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", semio_framework_value::DslValue::String("kind".into())), ("extra", semio_framework_value::DslValue::String("line".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page), ("object", object), ("text", semio_framework_value::DslValue::String("points".into())), ("extra", semio_framework_value::DslValue::String("1,2,3,4".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Line { points, .. } => assert_eq!(points, &[1.0, 2.0, 3.0, 4.0]),
        _ => panic!("line"),
    }
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", semio_framework_value::DslValue::String("City".into())), ("text", semio_framework_value::DslValue::String(String::new())), ("extra", semio_framework_value::DslValue::String("choice".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", semio_framework_value::DslValue::String("City".into())), ("text", semio_framework_value::DslValue::String("options".into())), ("extra", semio_framework_value::DslValue::String("Bern=Bern,Zug=Zug".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", semio_framework_value::DslValue::String("City".into())), ("text", semio_framework_value::DslValue::String("values".into())), ("extra", semio_framework_value::DslValue::String("Bern".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", semio_framework_value::DslValue::String("City".into())), ("text", semio_framework_value::DslValue::String("top".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    let PdfFormFieldKind::Choice { options, values, top_index, .. } = &snapshot.acro_form.as_ref().expect("form").fields[0].kind else { panic!("choice") };
    assert_eq!(options, &vec![("Bern".to_string(), "Bern".to_string()), ("Zug".to_string(), "Zug".to_string())]);
    assert_eq!(values, &vec!["Bern".to_string()]);
    assert_eq!(top_index, &Some(1));
}

#[test]
fn resource_details_publish_onto_the_document() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("x", semio_framework_value::DslValue::float(-1.0)), ("text", semio_framework_value::DslValue::String("Chapter".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("0".into())), ("text", semio_framework_value::DslValue::String("outline.bold".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("0".into())), ("text", semio_framework_value::DslValue::String("outline.color".into())), ("red", semio_framework_value::DslValue::float(1.0)), ("green", semio_framework_value::DslValue::float(0.0)), ("blue", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    assert!(snapshot.outlines[0].bold);
    assert_eq!(snapshot.outlines[0].color, Some([1.0, 0.0, 0.0]));
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("Im1".into())), ("text", semio_framework_value::DslValue::String("image.interpolate".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("Im1".into())), ("text", semio_framework_value::DslValue::String("image.decode".into())), ("extra", semio_framework_value::DslValue::String("0,1".into()))]))).unwrap());
    assert!(snapshot.images[0].interpolate);
    assert_eq!(snapshot.images[0].decode, vec![0.0, 1.0]);
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("GS1".into())), ("text", semio_framework_value::DslValue::String("graphics.cap".into())), ("x", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("GS1".into())), ("text", semio_framework_value::DslValue::String("graphics.dash".into())), ("extra", semio_framework_value::DslValue::String("3,1".into())), ("x", semio_framework_value::DslValue::float(0.0))]))).unwrap());
    assert!(matches!(snapshot.ext_g_states[0].line_cap, Some(crate::standards::v1_7::subsets::base::schema::snapshot::PdfLineCap::Round)));
    assert_eq!(snapshot.ext_g_states[0].dash.as_ref().map(|(dash, phase)| (dash.clone(), *phase)), Some((vec![3.0, 1.0], 0.0)));
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String("F1".into())), ("text", semio_framework_value::DslValue::String("font.italicAngle".into())), ("x", semio_framework_value::DslValue::float(-12.0))]))).unwrap());
    let italic = match &snapshot.fonts[0].kind {
        crate::standards::v1_7::subsets::base::schema::snapshot::PdfFontKind::Type1 { descriptor, .. } => descriptor.as_ref().expect("descriptor").italic_angle,
        _ => panic!("simple font"),
    };
    assert!((italic + 12.0).abs() < 0.01);
}

#[test]
fn page_window_declares_object_selection() {
    let definition = crate::editor::pdf17::create_pdf17_editor();
    assert!(definition.interactions.iter().any(|item| item.id == OBJECT_DOMAIN));
    let window = definition.window_kinds.iter().find(|window| window.id == WINDOW_KIND_ID).expect("page window");
    assert_eq!(window.interactions[0].as_str(), OBJECT_DOMAIN);
    assert!(window.actions.iter().any(|action| action.id == "canvasPointerDown"));
    let inspector = definition.window_kinds.iter().find(|window| window.id == INSPECTOR_WINDOW_KIND_ID).expect("object window");
    assert_eq!(inspector.body_key, INSPECTOR_BODY_KEY);
}

#[test]
fn selected_object_fields_commit_through_the_value_argument() {
    let _ui_arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let vector = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Vector).expect("vector");
    let mut edited = snapshot.clone();
    apply(&mut edited, edit_from_action("set-text", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(text.id.clone())), ("field", semio_framework_value::DslValue::String("text".into())), ("value", semio_framework_value::DslValue::String("World".into()))]))).unwrap());
    assert_eq!(edited.pages[0].text(), "World");
    apply(&mut edited, edit_from_action("move", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(vector.id.clone())), ("field", semio_framework_value::DslValue::String("x".into())), ("y", semio_framework_value::DslValue::float(20.0)), ("value", semio_framework_value::DslValue::float(44.0))]))).unwrap());
    let moved = objects(&edited).into_iter().find(|object| object.id == vector.id).expect("vector");
    assert!((moved.x - 44.0).abs() < 0.01);
    let node = render_inspector(&snapshot, Some(&text.id), "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), semio_ui_locale::Locale::En).expect("inspector");
    let mut names = Vec::new();
    let mut shown = Vec::new();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node), |node| collect_inspector(node, &mut names, &mut shown));
    assert!(names.iter().any(|name| name == "set-text"));
    assert!(names.iter().any(|name| name == "set-font"));
    assert!(names.iter().any(|name| name == "delete"));
    assert!(shown.iter().any(|value| value == "Hello"));
    assert!(shown.iter().any(|value| value == "Helvetica"));
    apply(&mut edited, edit_from_action("set-font", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(text.id.clone())), ("field", semio_framework_value::DslValue::String("text".into())), ("value", semio_framework_value::DslValue::String("Times-Roman".into()))]))).unwrap());
    assert!(edited.fonts.iter().any(|font| font.base_font() == "Times-Roman"));
    let image = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    let image_panel = render_inspector(&snapshot, Some(&image.id), "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), semio_ui_locale::Locale::En).expect("image inspector");
    let mut image_names = Vec::new();
    let mut image_shown = Vec::new();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(image_panel), |node| collect_inspector(node, &mut image_names, &mut image_shown));
    assert!(image_names.iter().any(|name| name == "set-resource-detail"));
    assert!(image_shown.iter().any(|value| value == "0"));
    apply(&mut edited, edit_from_action("set-resource-detail", Some(&args(vec![("object", semio_framework_value::DslValue::String(image.text.clone())), ("text", semio_framework_value::DslValue::String("image.interpolate".into())), ("field", semio_framework_value::DslValue::String("x".into())), ("value", semio_framework_value::DslValue::float(1.0))]))).unwrap());
    assert!(edited.images[0].interpolate);
    let empty = render_inspector(&snapshot, None, "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), semio_ui_locale::Locale::De).expect("empty inspector");
    let mut empty_names = Vec::new();
    let mut empty_shown = Vec::new();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(empty), |node| collect_inspector(node, &mut empty_names, &mut empty_shown));
    assert!(empty_names.iter().any(|name| name == "set-page-size"));
    assert!(empty_names.iter().any(|name| name == "set-info-field"));
    assert!(empty_shown.iter().any(|value| value == "200"));
    apply(&mut edited, edit_from_action("set-page-size", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("field", semio_framework_value::DslValue::String("width".into())), ("height", semio_framework_value::DslValue::float(300.0)), ("value", semio_framework_value::DslValue::float(180.0))]))).unwrap());
    assert!((edited.pages[0].width() - 180.0).abs() < 0.01);
    apply(&mut edited, edit_from_action("set-info", Some(&args(vec![("field", semio_framework_value::DslValue::String("text".into())), ("value", semio_framework_value::DslValue::String("Title".into()))]))).unwrap());
    assert_eq!(edited.info.title.as_deref(), Some("Title"));
}

fn collect_inspector(node: &semio_ui_contract::BuiltNode, names: &mut Vec<String>, shown: &mut Vec<String>) {
    for binding in node.bindings.iter() {
        names.push(binding.action.name.as_str().to_string());
    }
    if let semio_ui_contract::Component::Input(input) = &node.component {
        assert!(input.draft_target.is_some(), "every PDF inspector input has an explicit semantic draft target");
        assert_eq!(input.publication_revision, Some(semio_plugin::UiPublicationRevision(17)), "every PDF inspector input carries the exact render publication");
        shown.push(input.value.as_str().to_string());
    }
    for child in node.children.iter() {
        collect_inspector(child, names, shown);
    }
}

#[test]
fn canvas_vector_paint_matches_document_paths() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️vector-paint.json")).unwrap();
    let stream = fixture["content"].as_array().unwrap().iter().map(|line| line.as_str().unwrap()).collect::<Vec<_>>().join("\n");
    let pdf = pdf_with_content(fixture["width"].as_u64().unwrap(), fixture["height"].as_u64().unwrap(), &stream);
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let layers = &scene.as_array().unwrap()[1..];
    let paths = fixture["paths"].as_array().unwrap();
    assert_eq!(layers.len(), paths.len(), "an unpainted path is not an editor object");
    for (layer, path) in layers.iter().zip(paths) {
        let mode = path["type"].as_str().unwrap();
        assert_eq!(!layer["fill"].is_null(), mode.contains('f'));
        assert_eq!(!layer["stroke"].is_null(), mode.contains('s'));
        assert_eq!(layer["fillRule"], path["rule"]);
        let segments = layer["segments"].as_array().unwrap();
        if let Some(expected) = path["curves"].as_array() {
            let curves = segments.iter().filter(|segment| segment["kind"] == "cubic").collect::<Vec<_>>();
            assert_eq!(curves.len(), expected.len());
            for (segment, curve) in curves.into_iter().zip(expected) {
                for key in ["ctrl1", "ctrl2", "to"] {
                    assert_layer_point(layer, &segment[key], &curve[key]);
                }
            }
        } else {
            assert_layer_point(layer, &segments[0]["to"], &path["start"]);
            assert_layer_point(layer, &segments[2]["to"], &path["end"]);
        }
    }
    let scale = layers[0]["transform"][0].as_f64().unwrap();
    let width = layers[0]["stroke"]["width"].as_f64().unwrap();
    assert_eq!(Some(width * scale), fixture["scaledStrokeWidth"].as_f64());
    for probe in fixture["pixels"].as_array().unwrap() {
        let x = probe["at"][0].as_f64().unwrap() + 0.5;
        let y = probe["at"][1].as_f64().unwrap() + 0.5;
        assert_eq!(hit_test(&snapshot, x, y).is_some(), probe["rgb"] != serde_json::json!([255, 255, 255]));
    }
    assert_eq!(hit_test(&snapshot, 40.0, 120.0).as_deref(), layers[0]["id"].as_str());
    assert_eq!(hit_test(&snapshot, 210.0, 115.0), None);
}


fn assert_layer_point(layer: &Value, actual: &Value, expected: &Value) {
    if layer["transform"].is_null() {
        assert_point(actual, expected);
    } else {
        let [a, b, c, d, e, f]: [f64; 6] = serde_json::from_value(layer["transform"].clone()).unwrap();
        let x = actual[0].as_f64().unwrap();
        let y = actual[1].as_f64().unwrap();
        assert_point(&serde_json::json!([a * x + c * y + e, b * x + d * y + f]), expected);
    }
}
fn assert_point(actual: &Value, expected: &Value) {
    for axis in 0..2 {
        assert_eq!(actual[axis].as_f64(), expected[axis].as_f64());
    }
}

#[test]
fn inspector_can_clear_title_without_removing_author() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    let fixture: Value = serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let case = &fixture["metadataEdit"];
    let mut snapshot = sample();
    snapshot.info.title = Some(case["before"]["title"].as_str().unwrap().into());
    snapshot.info.author = Some(case["before"]["author"].as_str().unwrap().into());
    let commit = args(vec![
        ("object", semio_framework_value::DslValue::String("title".into())),
        ("field", semio_framework_value::DslValue::String("text".into())),
        ("value", semio_framework_value::DslValue::String(case["value"].as_str().unwrap().into())),
    ]);
    apply(&mut snapshot, edit_from_action("set-info-field", Some(&commit)).unwrap());
    let bytes = crate::standards::v1_7::subsets::base::io::export::encode_pdf(&snapshot).unwrap();
    let parsed: Value = serde_json::from_str(&oracle::project_pdf_1_7(&bytes).unwrap().to_string()).unwrap();
    assert_eq!(parsed["metadata"], case["after"]);
}

#[test]
fn canvas_repaints_image_samples_after_replacement() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️image-paint.json")).unwrap();
    let bytes = |state: &str| serde_json::from_value::<Vec<u8>>(fixture[state].clone()).unwrap();
    let width_points = fixture["page"][0].as_f64().unwrap();
    let height_points = fixture["page"][1].as_f64().unwrap();
    let mut page = PdfPage::new(width_points, height_points);
    page.content = vec![
        PdfOp::Save,
        PdfOp::Transform { matrix: serde_json::from_value(fixture["matrix"].clone()).unwrap() },
        PdfOp::PaintXObject { name: "Im1".into() },
        PdfOp::Restore,
    ];
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(page);
    let width = fixture["width"].as_u64().unwrap() as u32;
    let height = fixture["height"].as_u64().unwrap() as u32;
    snapshot.images.push(PdfImage::rgb8("Im1", width, height, bytes("before")));
    let object = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).unwrap();
    let verify = |snapshot: &PdfSnapshot, expected: Vec<u8>| {
        let canvas: Value = serde_json::from_str(&canvas_layers(snapshot, Some(&object.id))).unwrap();
        let image = canvas.as_array().unwrap().iter().find(|layer| layer["id"] == object.id.as_str()).unwrap();
        assert_eq!(image["kind"], "image");
        for (index, key) in ["x", "y", "width", "height"].into_iter().enumerate() {
            assert_eq!(image[key].as_f64(), fixture["bounds"][index].as_f64());
        }
        let data = image["image"]["src"].as_str().unwrap().strip_prefix("data:image/png;base64,").unwrap();
        let png = semio_framework_io_base64::base64_standard_decode(data).unwrap();
        let raster = semio_framework_pixels::decode_png(&png).unwrap();
        assert_eq!((raster.width, raster.height), (width, height));
        assert_eq!(raster.pixels, expected);
        render_window(snapshot).expect("image scene admission");
    };
    let opaque_rgba = |state| bytes(state).chunks_exact(3).flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255]).collect();
    verify(&snapshot, opaque_rgba("before"));
    let set_mask = |snapshot: &mut PdfSnapshot, kind: &str, value: String| {
        apply(snapshot, edit_from_action("set-image-mask", Some(&args(vec![
            ("object", semio_framework_value::DslValue::String("Im1".into())),
            ("text", semio_framework_value::DslValue::String(kind.into())),
            ("extra", semio_framework_value::DslValue::String(value)),
        ]))).unwrap());
    };
    let mut masked = snapshot.clone();
    masked.images[0].decode = serde_json::from_value(fixture["decode"].clone()).unwrap();
    let key = fixture["mask"].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(",");
    set_mask(&mut masked, "colorKey", key);
    verify(&masked, bytes("maskedRgba"));
    for case in fixture["referencedMasks"].as_array().unwrap() {
        let mut masked = snapshot.clone();
        if let Some(source) = case.get("source") {
            masked.images[0] = PdfImage::rgb8("Im1", width, height, serde_json::from_value(source.clone()).unwrap());
        }
        let kind = case["kind"].as_str().unwrap();
        let number = |key: &str| case[key].as_u64().unwrap().try_into().unwrap();
        let mut mask = PdfImage::samples("Mask1", number("width"), number("height"), PdfColorSpace::DeviceGray,
            number("bits"), serde_json::from_value(case["samples"].clone()).unwrap());
        mask.image_mask = kind == "stencil";
        if mask.image_mask {
            mask.color_space = None;
        }
        mask.decode = serde_json::from_value(case["decode"].clone()).unwrap();
        mask.matte = serde_json::from_value(case["matte"].clone()).unwrap();
        masked.images.push(mask);
        if case["overridesColorKey"] == true {
            set_mask(&mut masked, "colorKey", "0,255,0,255,0,255".into());
        }
        set_mask(&mut masked, kind, "Mask1".into());
        verify(&masked, serde_json::from_value(case["expectedRgba"].clone()).unwrap());
    }
    let samples = bytes("after").iter().map(u8::to_string).collect::<Vec<_>>().join(",");
    apply(&mut snapshot, edit_from_action("set-image", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(object.id.clone())),
        ("width", semio_framework_value::DslValue::float(f64::from(width))),
        ("height", semio_framework_value::DslValue::float(f64::from(height))),
        ("text", semio_framework_value::DslValue::String(samples)),
    ]))).unwrap());
    verify(&snapshot, opaque_rgba("after"));
}

#[test]
fn canvas_paints_base14_glyph_outlines_at_the_pdf_baseline() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️text-paint.json")).unwrap();
    let mut page = PdfPage::new(fixture["page"][0].as_f64().unwrap(), fixture["page"][1].as_f64().unwrap());
    let expected = &fixture["expected"];
    page.content = vec![
        PdfOp::Transform { matrix: serde_json::from_value(fixture["ctm"].clone()).unwrap() },
        PdfOp::BeginText,
        PdfOp::SetFont { name: "F1".into(), size: expected["text"]["size"].as_f64().unwrap() },
        PdfOp::SetTextMatrix { matrix: serde_json::from_value(fixture["textMatrix"].clone()).unwrap() },
        PdfOp::SetTextRise { rise: fixture["textRise"].as_f64().unwrap() },
        PdfOp::ShowText { text: PdfTextString::text(expected["text"]["content"].as_str().unwrap()) },
        PdfOp::EndText,
    ];
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(page);
    snapshot.fonts.push(PdfFont::standard("F1", fixture["baseFont"].as_str().unwrap()));
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let layer = &scene[1];
    assert_eq!(layer["fillRule"], expected["fillRule"]);
    let segments = layer["segments"].as_array().expect("Base 14 outline paths");
    assert!(segments.iter().any(|segment| segment["kind"] == "cubic"));
    assert_eq!(segments.iter().filter(|segment| segment["kind"] == "close").count(), expected["contours"].as_u64().unwrap() as usize);
    for axis in 0..6 {
        assert_eq!(layer["transform"][axis].as_f64(), expected["transform"][axis].as_f64());
    }
    render_window(&snapshot).expect("Base 14 outline scene admission");
}

#[test]
fn moving_resizing_and_deleting_an_annotation_preserves_its_note_until_removal() {
    use semio_framework_test_fixtures::fixture_bytes;
    use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf};
    let before = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/⬅️before.pdf");
    let after = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/📍️remove-annotation/➡️after.pdf");
    let mut doc = decode_pdf(before).unwrap();
    let original = doc.pages[0].annotations[0].clone();
    for (action, rect) in [("move", [5.0, 8.0, 25.0, 28.0]), ("resize", [5.0, 8.0, 65.0, 78.0])] {
        let edit = edit_from_action(action, Some(&args(vec![
            ("page", semio_framework_value::DslValue::float(0.0)),
            ("object", semio_framework_value::DslValue::String("p0:a0".into())),
            ("x", semio_framework_value::DslValue::float(5.0)), ("y", semio_framework_value::DslValue::float(8.0)),
            ("width", semio_framework_value::DslValue::float(60.0)), ("height", semio_framework_value::DslValue::float(70.0)),
        ]))).unwrap();
        apply(&mut doc, edit);
        assert_eq!(doc.pages[0].annotations[0].rect, rect);
        assert_eq!(doc.pages[0].annotations[0].contents, original.contents);
    }
    let removal = edit_from_action("delete", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String("p0:a0".into())),
    ]))).unwrap();
    apply(&mut doc, removal);
    let reopened = decode_pdf(&encode_pdf(&doc).unwrap()).unwrap();
    let independent = decode_pdf(after).unwrap();
    assert!(independent.pages[0].annotations.is_empty());
    assert_eq!(reopened.pages[0].annotations, independent.pages[0].annotations);
    assert_eq!(reopened.pages[1].text(), independent.pages[1].text());
}

#[test]
fn canvas_paints_nested_form_content_at_its_pdf_placement() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️form-paint.json")).unwrap();
    let forms = fixture["forms"].as_array().unwrap();
    let stream = |dictionary: &str, data: &str| format!("<< {dictionary} /Length {} >>\nstream\n{data}\nendstream", data.len());
    let coordinates = |values: &Value| values.as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ");
    let mut pdf_objects = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {}] /Contents 4 0 R /Resources << /XObject << /Outer 5 0 R >> >> >>", coordinates(&fixture["page"])),
        stream("", fixture["pageContent"].as_str().unwrap()),
    ];
    for form in forms {
        let resources = if form["use"].is_null() { "" } else { "/Resources << /XObject << /Inner 6 0 R >> >>" };
        let dictionary = format!("/Type /XObject /Subtype /Form /BBox [{}] /Matrix [{}] {resources}", coordinates(&form["bbox"]), coordinates(&form["matrix"]));
        pdf_objects.push(stream(&dictionary, form["content"].as_str().unwrap()));
    }
    let bytes = pdf_objects_bytes(&pdf_objects);
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(bytes.as_bytes()).unwrap();
    let selectable = objects(&snapshot);
    assert_eq!(selectable.len(), 1);
    assert_eq!(selectable[0].kind, ObjectKind::Form);
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    assert_eq!(scene.as_array().unwrap().len(), 3);
    for (layer, form) in scene.as_array().unwrap()[1..].iter().zip(forms) {
        assert!(layer["id"].as_str().unwrap().starts_with(&selectable[0].id));
        for axis in 0..4 {
            assert_eq!(layer["clipRect"][axis].as_f64(), form["clip"][axis].as_f64());
        }
        assert_point(&layer["segments"][0]["to"], &form["start"]);
        assert_point(&layer["segments"][2]["to"], &form["end"]);
    }
    assert_eq!(hit_test(&snapshot, 42.0, 60.0).as_deref(), Some(selectable[0].id.as_str()));
    assert_eq!(hit_test(&snapshot, 65.0, 60.0), None);
}

fn pdf_objects_bytes(objects: &[String]) -> String {
    let mut pdf = "%PDF-1.7\n".to_string();
    let mut xref = String::new();
    for (index, object) in objects.iter().enumerate() {
        xref.push_str(&format!("{:010} 00000 n \n", pdf.len()));
        pdf.push_str(&format!("{} 0 obj\n{object}\nendobj\n", index + 1));
    }
    let size = objects.len() + 1;
    pdf.push_str(&format!("xref\n0 {size}\n0000000000 65535 f \n{xref}trailer\n<< /Root 1 0 R /Size {size} >>\nstartxref\n{}\n%%EOF\n", pdf.len()));
    pdf
}

#[test]
fn canvas_viewport_crops_page_paint_and_hit_testing() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️page-viewport.json")).unwrap();
    let content = fixture["rectangles"].as_array().unwrap().iter().flat_map(|rectangle| {
        let [r, g, b]: [f64; 3] = serde_json::from_value(rectangle["rgb"].clone()).unwrap();
        let [x, y, width, height]: [f64; 4] = serde_json::from_value(rectangle["bounds"].clone()).unwrap();
        [PdfOp::SetFillRgb { r, g, b }, PdfOp::Rectangle { x, y, width, height }, PdfOp::Fill]
    }).collect::<Vec<_>>();
    let point = |value: &Value| -> [f64; 2] { serde_json::from_value(value.clone()).unwrap() };

    for case in fixture["cases"].as_array().unwrap() {
        for crop in ["cropBox", "overrunCropBox"] {
            let mut snapshot = PdfSnapshot::default();
            snapshot.pages.push(PdfPage {
                media_box: serde_json::from_value(fixture["mediaBox"].clone()).unwrap(),
                crop_box: Some(serde_json::from_value(fixture[crop].clone()).unwrap()),
                user_unit: fixture["userUnit"].as_f64(),
                rotate: case["rotate"].as_i64().unwrap() as i32,
                content: content.clone(),
                ..Default::default()
            });
            snapshot.pages.push(PdfPage::new(100.0, 100.0));

            let id = objects(&snapshot)[0].id.clone();
            let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, Some(&id))).unwrap();
            let [width, height] = point(&case["size"]);
            assert_point(&scene[0]["segments"][2]["to"], &case["size"]);
            assert_layer_point(&scene[2], &scene[2]["segments"][0]["to"], &case["firstPoint"]);
            for layer in &scene.as_array().unwrap()[2..] {
                let clip: [f64; 4] = serde_json::from_value(layer["clipRect"].clone()).unwrap();
                assert_eq!(clip, [0.0, 0.0, width, height]);
            }
            let [x, y] = point(&case["ink"]);
            assert_eq!(hit_test(&snapshot, x, y).as_deref(), Some(id.as_str()));
            let [x, y] = point(&case["blank"]);
            assert_eq!(hit_test(&snapshot, x, y), None);
            if case["rotate"] == 90 {
                assert_eq!(scene[1]["segments"][0]["to"][1].as_f64(), fixture["secondPageTop"].as_f64());
                for probe in ["outsideFirstPage", "onSecondPage"] {
                    let [x, y] = point(&fixture[probe]);
                    assert_eq!(hit_test(&snapshot, x, y), None);
                }
                let [x, y, width, height]: [f64; 4] = serde_json::from_value(fixture["secondPageInk"].clone()).unwrap();
                snapshot.pages[1].content = vec![PdfOp::Rectangle { x, y, width, height }, PdfOp::Fill];
                let [x, y] = point(&fixture["onSecondPage"]);
                assert_eq!(hit_test(&snapshot, x, y).as_deref(), Some("p1:v0"));
                let [x, y] = point(&fixture["pageGap"]);
                assert_eq!(hit_test(&snapshot, x, y), None);
            }
        }
    }
}

#[test]
fn deleting_a_path_retains_the_graphics_state_used_by_the_next_path() {
    let case: Value = serde_json::from_str(include_str!("🧪️delete-path.json")).unwrap();
    let pdf = pdf_with_content(50, 50, case["before"].as_str().unwrap());
    let mut snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf.as_bytes()).unwrap();
    let removal = edit_from_action("delete", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String("p0:v0".into())),
    ]))).unwrap();
    apply(&mut snapshot, removal);
    let remaining = objects(&snapshot);
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].fill, [1.0, 0.0, 0.0]);
    assert_eq!(remaining[0].x, 20.0);
    let bytes = crate::standards::v1_7::subsets::base::io::export::encode_pdf(&snapshot).unwrap();
    let expected = pdf_with_content(50, 50, case["after"].as_str().unwrap());
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;
    let projection = |bytes: &[u8]| -> Value { serde_json::from_str(&project_pdf_1_7(bytes).unwrap().to_string()).unwrap() };
    assert_eq!(projection(&bytes)["pages"], projection(expected.as_bytes())["pages"]);

}

fn pdf_with_content(width: u64, height: u64, stream: &str) -> String {
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Contents 4 0 R >>", width, height),
        format!("<< /Length {} >>\nstream\n{stream}\nendstream", stream.len()),
    ];
    let pdf = pdf_objects_bytes(&objects);
    pdf
}

#[test]
fn canvas_projects_axial_shading_with_unextended_ends() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️shading-paint.json")).unwrap();
    let words = |key: &str| fixture[key].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ");
    let bbox = words("bbox");
    let coords = words("coords");
    let extend = words("extend");
    let colors = fixture["colors"].as_array().unwrap().iter().map(|color| color.as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ")).collect::<Vec<_>>();
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {}] /Contents 4 0 R /Resources << /Shading << /Sh1 5 0 R >> >> >>", words("page")),
        "<< /Length 7 >>\nstream\n/Sh1 sh\nendstream".into(),
        format!("<< /ShadingType 2 /ColorSpace /DeviceRGB /BBox [{bbox}] /Coords [{coords}] /Extend [{extend}] /Function << /FunctionType 2 /Domain [0 1] /C0 [{}] /C1 [{}] /N 1 >> >>", colors[0], colors[1]),
    ];
    let bytes = pdf_objects_bytes(&objects);
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(bytes.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let layer = &scene[1];
    let fill = &layer["fill"];
    assert_eq!(fill["kind"], "linearGradient");
    for (index, axis) in ["x1", "y1", "x2", "y2"].into_iter().enumerate() {
        assert_eq!(fill[axis].as_f64(), fixture["coords"][index].as_f64());
    }
    let stops = fill["stops"].as_array().unwrap();
    assert_eq!(stops.len(), 2);
    for channel in 0..3 {
        assert_eq!(stops[0]["color"][channel].as_f64(), fixture["colors"][0][channel].as_f64());
        assert_eq!(stops[1]["color"][channel].as_f64(), fixture["colors"][1][channel].as_f64());
    }
    for axis in 0..6 {
        assert_eq!(layer["transform"][axis].as_f64(), fixture["transform"][axis].as_f64());
    }
    let vertices = layer["segments"].as_array().unwrap().iter().filter_map(|segment| {
        Some([segment["to"][0].as_f64()?, segment["to"][1].as_f64()?])
    }).collect::<Vec<_>>();
    let expected = &fixture["expectedRect"];
    assert_eq!(vertices.len(), 4);
    assert!(vertices.iter().all(|point| (0..2).all(|axis| {
        [axis, axis + 2].into_iter().any(|bound| (point[axis] - expected[bound].as_f64().unwrap()).abs() < 1e-8)
    })));
}

#[test]
fn restyling_one_path_preserves_its_neighbors() {
    let case: Value = serde_json::from_str(include_str!("🧪️edit-paint.json")).unwrap();
    let pdf = pdf_with_content(50, 50, case["before"].as_str().unwrap());
    let mut doc = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf.as_bytes()).unwrap();
    let original = objects(&doc);
    let target = &original[1].id;
    let edit = edit_from_action("set-fill", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(target.clone())),
        ("red", semio_framework_value::DslValue::float(0.0)), ("green", semio_framework_value::DslValue::float(1.0)),
        ("blue", semio_framework_value::DslValue::float(0.0)),
    ]))).unwrap();
    apply(&mut doc, edit);
    let edited = objects(&doc);
    assert_eq!(edited[0].fill, original[0].fill);
    assert_eq!(edited[1].fill, [0.0, 1.0, 0.0]);
    assert_eq!(edited[2].fill, original[2].fill);
    let bytes = crate::standards::v1_7::subsets::base::io::export::encode_pdf(&doc).unwrap();
    let expected = pdf_with_content(50, 50, case["after"].as_str().unwrap());
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;
    let projection = |bytes: &[u8]| -> Value { serde_json::from_str(&project_pdf_1_7(bytes).unwrap().to_string()).unwrap() };
    assert_eq!(projection(&bytes)["pages"], projection(expected.as_bytes())["pages"]);
    assert_eq!(&edited[1].id, target, "editing paint preserves the inspector selection");
    let selected = target.clone();
    let stroke = edit_from_action("set-stroke", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(selected)),
        ("red", semio_framework_value::DslValue::float(0.0)), ("green", semio_framework_value::DslValue::float(0.0)),
        ("blue", semio_framework_value::DslValue::float(1.0)), ("width", semio_framework_value::DslValue::float(3.0)),
    ]))).unwrap();
    apply(&mut doc, stroke);
    let painted = objects(&doc);
    assert_eq!(painted[1].stroke, [0.0, 0.0, 1.0]);
    assert_eq!(painted[1].line_width, 3.0);
    assert_eq!(painted[2].stroke, original[2].stroke);
    assert_eq!(painted[2].line_width, original[2].line_width);

}

#[test]
fn canvas_clips_a_skewed_pdf_form_to_its_transformed_corners() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️oriented-form-clip.json")).unwrap();
    let words = |key: &str| fixture[key].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ");
    let content = fixture["formContent"].as_str().unwrap();
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {}] /Contents 4 0 R /Resources << /XObject << /Form 5 0 R >> >> >>", words("page")),
        "<< /Length 8 >>\nstream\n/Form Do\nendstream".into(),
        format!("<< /Type /XObject /Subtype /Form /BBox [{}] /Matrix [{}] /Length {} >>\nstream\n{content}\nendstream", words("bbox"), words("matrix"), content.len()),
    ];
    let bytes = pdf_objects_bytes(&objects);
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(bytes.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    assert_eq!(scene[1]["clipPolygons"].as_array().unwrap().len(), 1);
    for point in 0..4 {
        assert_point(&scene[1]["clipPolygons"][0][point], &fixture["polygon"][point]);
    }
    for probe in fixture["probes"].as_array().unwrap() {
        let x = probe["at"][0].as_f64().unwrap() + 0.5;
        let y = probe["at"][1].as_f64().unwrap() + 0.5;
        assert_eq!(hit_test(&snapshot, x, y).is_some(), probe["rgb"][1] == 0);
    }
}

#[test]
fn resizing_a_transformed_curve_moves_its_control_points() {
    let case: Value = serde_json::from_str(include_str!("🧪️resize-curve.json")).unwrap();
    let file = |stream: &Value| pdf_with_content(240, 240, stream.as_str().unwrap());
    let open = |stream: &Value| crate::standards::v1_7::subsets::base::io::import::decode_pdf(file(stream).as_bytes()).unwrap();
    let mut doc = open(&case["before"]);
    let original = objects(&doc);
    let edit = edit_from_action("resize", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(original[0].id.clone())),
        ("x", semio_framework_value::DslValue::float(30.0)), ("y", semio_framework_value::DslValue::float(40.0)),
        ("width", semio_framework_value::DslValue::float(80.0)), ("height", semio_framework_value::DslValue::float(20.0)),
    ]))).unwrap();
    apply(&mut doc, edit);
    assert_eq!(canvas_layers(&doc, None), canvas_layers(&open(&case["after"]), None));
}

#[test]
fn moving_one_image_preserves_another_painted_in_the_same_transform() {
    let mut doc = sample();
    let case: Value = serde_json::from_str(include_str!("🧪️image-move.json")).unwrap();
    let matrix: [f64; 6] = serde_json::from_value(case["ctm"].clone()).unwrap();
    doc.pages[0].content = vec![
        PdfOp::Transform { matrix },
        PdfOp::PaintXObject { name: "Im1".into() },
        PdfOp::PaintXObject { name: "Im1".into() },
    ];
    let images = objects(&doc);
    let edit = edit_from_action("move", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(images[0].id.clone())),
        ("x", semio_framework_value::DslValue::float(case["target"][0].as_f64().unwrap())),
        ("y", semio_framework_value::DslValue::float(case["target"][1].as_f64().unwrap())),
    ]))).unwrap();
    apply(&mut doc, edit);
    let result = objects(&doc);
    assert_eq!([result[0].x, result[0].y], [70.0, 90.0]);
    assert_eq!(result[1].ctm, matrix);
    assert_eq!(result[0].id, images[0].id);
}

#[test]
fn font_edits_preserve_following_text_and_existing_resources() {
    let case: Value = serde_json::from_str(include_str!("🧪️font-edit.json")).unwrap();
    let mut doc = sample();
    let occupied = case["occupied"].as_str().unwrap();
    doc.fonts.push(PdfFont::standard(occupied, case["occupiedFont"].as_str().unwrap()));
    let content = &mut doc.pages[0].content;
    content.insert(content.len() - 1, PdfOp::ShowText { text: PdfTextString::text("Following") });
    let following = objects(&doc).into_iter().find(|object| object.text == "Following").unwrap();
    let selected = objects(&doc).into_iter().find(|object| object.kind == ObjectKind::Text).unwrap();
    let edit = edit_from_action("set-font", Some(&args(vec![
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("object", semio_framework_value::DslValue::String(selected.id)),
        ("text", semio_framework_value::DslValue::String(case["replacement"].as_str().unwrap().into())),
    ]))).unwrap();
    apply(&mut doc, edit);
    let after = objects(&doc).into_iter().find(|object| object.text == "Following").unwrap();
    assert!((following.x - after.x).abs() < 0.001 && (following.y - after.y).abs() < 0.001);
    let fonts: Vec<_> = objects(&doc).into_iter().filter(|o| o.kind == ObjectKind::Text)
        .map(|o| doc.fonts.iter().find(|f| f.id == o.font).unwrap().base_font()).collect();
    assert_eq!(serde_json::json!(fonts), case["expected"]);
    assert_eq!(doc.fonts.iter().find(|f| f.id == occupied).unwrap().base_font(), case["occupiedFont"].as_str().unwrap());
}

#[test]
fn moving_a_show_operator_preserves_the_following_text_position() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️move-text-run.json")).unwrap();
    let mut doc = sample();
    doc.pages[0].content = vec![
        PdfOp::BeginText,
        PdfOp::SetFont { name: "F1".into(), size: 10.0 },
        PdfOp::MoveText { tx: 20.0, ty: 100.0 },
        PdfOp::ShowText { text: PdfTextString::text("A") },
        PdfOp::ShowText { text: PdfTextString::text("B") },
        PdfOp::EndText,
    ];
    let original = objects(&doc);
    assert!((original[1].x - fixture["followingX"].as_f64().unwrap()).abs() < 1e-5);
    let mut deleted = doc.clone();
    apply(&mut deleted, edit_from_action("delete", Some(&args(vec![
        ("object", semio_framework_value::DslValue::String(original[0].id.clone())),
        ("page", semio_framework_value::DslValue::float(0.0)),
    ]))).unwrap());
    let remaining = objects(&deleted);
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].text, "B");
    assert!((remaining[0].x - original[1].x).abs() < 1e-5);
    let mut resized = doc.clone();
    let target = &fixture["resize"];
    apply(&mut resized, edit_from_action("resize", Some(&args(vec![
        ("object", semio_framework_value::DslValue::String(original[0].id.clone())),
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("x", semio_framework_value::DslValue::float(target["x"].as_f64().unwrap())),
        ("y", semio_framework_value::DslValue::float(target["y"].as_f64().unwrap())),
        ("width", semio_framework_value::DslValue::float(target["width"].as_f64().unwrap())),
        ("height", semio_framework_value::DslValue::float(target["height"].as_f64().unwrap())),
    ]))).unwrap());
    let result = objects(&resized);
    for (actual, key) in [(result[0].x, "x"), (result[0].y, "y"), (result[0].width, "width"), (result[0].height, "height")] {
        assert!((actual - target[key].as_f64().unwrap()).abs() < 1e-5);
    }
    assert!((result[1].x - original[1].x).abs() < 1e-5);
    assert_eq!(result[1].y, original[1].y);
    let x = original[0].x + fixture["offset"][0].as_f64().unwrap();
    let y = original[0].y + fixture["offset"][1].as_f64().unwrap();
    apply(&mut doc, edit_from_action("move", Some(&args(vec![
        ("object", semio_framework_value::DslValue::String(original[0].id.clone())),
        ("page", semio_framework_value::DslValue::float(0.0)),
        ("x", semio_framework_value::DslValue::float(x)), ("y", semio_framework_value::DslValue::float(y)),
    ]))).unwrap());
    let moved = objects(&doc);
    assert_eq!(moved.len(), 2);
    assert_eq!((moved[0].x, moved[0].y), (x, y));
    assert!((moved[1].x - original[1].x).abs() < 1e-7);
    assert_eq!(moved[1].y, original[1].y);
    for snapshot in [&doc, &resized, &deleted] {
        use crate::standards::v1_7::subsets::base::io::{export::encode_pdf, import::decode_pdf};
        use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;
        let bytes = encode_pdf(snapshot).unwrap();
        project_pdf_1_7(&bytes).expect("lopdf accepts the edited text document");
        let saved = objects(&decode_pdf(&bytes).unwrap());
        let expected = objects(snapshot);
        assert_eq!(saved.len(), expected.len());
        for (actual, expected) in saved.iter().zip(expected) {
            assert_eq!(actual.text, expected.text);
            for (a, b) in [(actual.x, expected.x), (actual.y, expected.y), (actual.width, expected.width), (actual.height, expected.height)] {
                assert!((a - b).abs() < 1e-4);
            }
        }
    }
}

#[test]
fn text_render_modes_select_fill_stroke_and_invisible_text() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️text-rendering.json")).unwrap();
    let mut doc = sample();
    let mut ops = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 12.0 }];
    for mode in fixture["modes"].as_array().unwrap() {
        let value = mode["value"].as_u64().unwrap() as u32;
        ops.extend([PdfOp::SetTextRenderingMode { mode: value }, PdfOp::ShowText { text: PdfTextString::text(value.to_string()) }]);
    }
    ops.push(PdfOp::EndText);
    doc.pages[0].content = ops;
    let scene: Value = serde_json::from_str(&canvas_layers(&doc, None)).unwrap();
    assert_eq!(objects(&doc).len(), 8);
    for mode in fixture["modes"].as_array().unwrap() {
        let id = objects(&doc).into_iter().find(|object| object.text == mode["label"].as_str().unwrap()).map(|object| object.id).unwrap();
        let layer = scene.as_array().unwrap().iter().find(|layer| layer["id"].as_str() == Some(id.as_str()));
        if mode["fill"] == false && mode["stroke"] == false {
            assert!(layer.is_none());
        } else {
            let layer = layer.unwrap();
            assert_eq!(layer.get("fill").is_some(), mode["fill"].as_bool().unwrap());
            assert_eq!(layer.get("stroke").is_some(), mode["stroke"].as_bool().unwrap());
        }
    }
}

#[test]
fn skewed_image_pointer_hits_follow_its_painted_quadrilateral() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️oriented-form-clip.json")).unwrap();
    let mut page = PdfPage::new(100.0, 100.0);
    page.content = vec![PdfOp::Transform { matrix: [20.0, 10.0, 8.0, 16.0, 20.0, 20.0] }, PdfOp::PaintXObject { name: "Im1".into() }];
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(page);
    snapshot.images.push(PdfImage::rgb8("Im1", 1, 1, vec![255, 0, 0]));
    for probe in fixture["probes"].as_array().unwrap() {
        let x = probe["at"][0].as_f64().unwrap() + 0.5;
        let y = probe["at"][1].as_f64().unwrap() + 0.5;
        assert_eq!(hit_test(&snapshot, x, y).is_some(), probe["rgb"][1] == 0);
    }
}

#[test]
fn blank_second_page_selects_its_own_inspector_and_rotation() {
    let _ui_arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let mut snapshot = sample();
    snapshot.info.subject = Some("Subject of the PDF".into());
    snapshot.pages.truncate(1);
    snapshot.pages.push(PdfPage::new(140.0, 190.0));
    let edit = edit_from_action("canvasPointerDown", Some(&args(vec![("x", semio_framework_value::DslValue::float(10.0)), ("y", semio_framework_value::DslValue::float(336.0))]))).unwrap();
    let result = emit_page_edit(&snapshot, &edit.action, &edit.payload).unwrap();
    let semio_plugin::Effect::DispatchAction { args: Some(selection), .. } = &result.effects[0] else { panic!("page selection") };
    let value = serde_json::Value::from(selection.clone());
    assert!(value["targets"].as_str().unwrap().contains("p1:paper"));
    let inspector = render_inspector(&snapshot, Some("p1:paper"), "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), Locale::En).unwrap();
    let mut shown = Vec::new();
    let mut bindings = Vec::new();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(inspector), |node| collect_inspector(node, &mut bindings, &mut shown));
    assert!(shown.iter().any(|value| value == "140"));
    assert!(shown.iter().any(|value| value == "Subject of the PDF"));
    for name in ["insert-page", "move-page", "remove-page"] { assert!(bindings.iter().any(|value| value == name)); }
    let edit = edit_from_action("set-page-rotation", Some(&args(vec![("page", semio_framework_value::DslValue::float(1.0)), ("x", semio_framework_value::DslValue::float(90.0))]))).unwrap();
    apply(&mut snapshot, edit);
    assert_eq!(snapshot.pages[1].rotate, 90);
    assert_eq!(snapshot.pages[0].rotate, 0);
}

#[test]
fn replacing_a_text_run_keeps_the_following_run_at_its_original_baseline() {
    let mut doc = sample();
    doc.pages[0].content = vec![
        PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 12.0 },
        PdfOp::MoveText { tx: 20.0, ty: 100.0 },
        PdfOp::ShowText { text: PdfTextString::text("A") }, PdfOp::ShowText { text: PdfTextString::text("B") }, PdfOp::EndText,
    ];
    let original = objects(&doc);
    let edit = edit_from_action("set-text", Some(&args(vec![("page", semio_framework_value::DslValue::float(0.0)), ("object", semio_framework_value::DslValue::String(original[0].id.clone())), ("text", semio_framework_value::DslValue::String("Longer replacement".into()))]))).unwrap();
    apply(&mut doc, edit);
    let edited = objects(&doc);
    assert_eq!(edited[0].text, "Longer replacement");
    assert!((edited[1].x - original[1].x).abs() < 0.001);
    assert!((edited[1].y - original[1].y).abs() < 0.001);
}

#[test]
fn compound_pdf_clips_preserve_fill_rule_order_and_hit_testing() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️path-clips.json")).unwrap();
    let mut page = PdfPage::new(fixture["width"].as_f64().unwrap(), fixture["height"].as_f64().unwrap());
    page.content = fixture["ops"].as_array().unwrap().iter().map(|op| {
        let num = |index: usize| op[index].as_f64().unwrap();
        match op[0].as_str().unwrap() {
            "q" => PdfOp::Save,
            "Q" => PdfOp::Restore,
            "n" => PdfOp::EndPath,
            "W" => PdfOp::Clip,
            "W*" => PdfOp::ClipEvenOdd,
            "f" => PdfOp::Fill,
            "h" => PdfOp::ClosePath,
            "m" => PdfOp::MoveTo { x: num(1), y: num(2) },
            "l" => PdfOp::LineTo { x: num(1), y: num(2) },
            "c" => PdfOp::CurveTo { x1: num(1), y1: num(2), x2: num(3), y2: num(4), x3: num(5), y3: num(6) },
            "re" => PdfOp::Rectangle { x: num(1), y: num(2), width: num(3), height: num(4) },
            "rg" => PdfOp::SetFillRgb { r: num(1), g: num(2), b: num(3) },
            unexpected => panic!("unknown clip fixture operator {unexpected}"),
        }
    }).collect();
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(page);
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let expected = fixture["layers"].as_array().unwrap();
    let scene = &scene.as_array().unwrap()[1..];
    assert_eq!(scene.len(), expected.len());
    for (actual, expected) in scene.iter().zip(expected) {
        for key in ["segments", "fill", "fillRule", "clipPaths", "clipPolygons"] {
            assert_eq!(actual[key], expected[key], "clip fixture {key}");
        }
    }
    for probe in fixture["probes"].as_array().unwrap() {
        let id = probe["object"].as_u64().map(|ordinal| format!("p0:v{ordinal}"));
        assert_eq!(hit_test(&snapshot, probe["at"][0].as_f64().unwrap() + 0.5, probe["at"][1].as_f64().unwrap() + 0.5), id);
    }
}

#[path = "📝️native-page-edit/🦀️.rs"]
mod native_page_edit;

#[test]
fn authored_strokes_round_trip_from_pdf_to_scene_and_selection() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️stroke-style.json")).unwrap();
    let pdf = pdf_with_content(fixture["width"].as_u64().unwrap(), fixture["height"].as_u64().unwrap(), fixture["ops"].as_str().unwrap());
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let layers = &scene.as_array().unwrap()[1..];
    let reference = fixture["layers"].as_array().unwrap();
    assert_eq!(layers.len(), reference.len());
    for (actual, expected) in layers.iter().zip(reference) {
        assert_eq!(actual["stroke"]["cap"], expected["stroke"]["cap"]);
        assert_eq!(actual["stroke"]["join"], expected["stroke"]["join"]);
        let numbers = |values: &Value| values.as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        for key in ["width", "dashOffset", "miterLimit"] {
            assert_eq!(actual["stroke"][key].as_f64(), expected["stroke"][key].as_f64());
        }
        for key in ["dash", "color"] {
            assert_eq!(numbers(&actual["stroke"][key]), numbers(&expected["stroke"][key]));
        }
        assert_eq!(numbers(&actual["transform"]), numbers(&expected["transform"]));
        for (actual, expected) in actual["segments"].as_array().unwrap().iter().zip(expected["segments"].as_array().unwrap()) {
            assert_eq!(actual["kind"], expected["kind"]);
            if expected["kind"] != "close" { assert_point(&actual["to"], &expected["to"]); }
        }
    }
    for probe in fixture["probes"].as_array().unwrap() {
        let expected = probe["object"].as_u64().map(|ordinal| format!("p0:v{ordinal}"));
        let x = probe["at"][0].as_f64().unwrap() + 0.5;
        let y = probe["at"][1].as_f64().unwrap() + 0.5;
        assert_eq!(hit_test(&snapshot, x, y), expected, "{probe}");
    }
}

#[test]
fn canvas_paints_selected_annotation_appearance_at_the_authored_rect() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️annotation-appearance.json")).unwrap();
    let words = |key: &str| fixture[key].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ");
    let stream = |key: &str| {
        let content = fixture[key].as_str().unwrap();
        format!("<< /Type /XObject /Subtype /Form /BBox [{}] /Matrix [{}] /Length {} >>\nstream\n{content}\nendstream", words("bbox"), words("matrix"), content.len())
    };
    let appearance = format!("<< /Type /Annot /Subtype /Stamp /Rect [{}] /AP << /N << /On 5 0 R /Off 6 0 R >> >> /AS /{} >>", words("rect"), fixture["state"].as_str().unwrap());
    let pdf = pdf_objects_bytes(&[
        "<< /Type /Catalog /Pages 2 0 R >>".into(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {}] /Annots [4 0 R] >>", words("page")),
        appearance,
        stream("selected"),
        stream("unselected"),
    ]);
    let mut snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    assert_eq!(scene.as_array().unwrap().len(), 2);
    assert!(scene[1]["id"].as_str().unwrap().starts_with("p0:a0/"));
    assert_eq!(scene[1]["fill"]["color"], serde_json::json!([1.0, 0.0, 0.0, 1.0]));
    assert_point(&scene[1]["segments"][0]["to"], &fixture["start"]);
    assert_point(&scene[1]["segments"][2]["to"], &fixture["end"]);
    for axis in 0..4 { assert_eq!(scene[1]["clipRect"][axis].as_f64(), fixture["clip"][axis].as_f64()); }
    assert_eq!(hit_test(&snapshot, 100.0, 80.0).as_deref(), Some("p0:a0"));
    snapshot.pages[0].annotations[0].flags |= 32;
    let hidden: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    assert_eq!(hidden.as_array().unwrap().len(), 1);
    assert_eq!(hit_test(&snapshot, 100.0, 80.0), None);
}

#[test]
fn canvas_paints_authored_type3_glyphs_and_pdf_text_adjustments() {
    let fixture: Value = serde_json::from_str(include_str!("🧪️type3-paint.json")).unwrap();
    let words = |entry: &Value| entry.as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(" ");
    let stream = |body: &str| format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len());
    let font = &fixture["type3"];
    let mut rows = vec![
        "<< /Type /Catalog /Pages 2 0 R >>".into(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {}] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>", words(&fixture["page"])),
        stream(fixture["pageContent"].as_str().unwrap()),
        format!("<< /Type /Font /Subtype /Type3 /FontBBox [{}] /FontMatrix [{}] /FirstChar 65 /LastChar 66 /Widths [{}] /Encoding << /Differences [65 /A /B] >> /CharProcs << /A 6 0 R /B 7 0 R >> /Resources <<>> >>", words(&font["bbox"]), words(&font["matrix"]), words(&font["widths"])),
    ];
    for (glyph, width) in font["glyphs"].as_array().unwrap().iter().zip(font["widths"].as_array().unwrap()) {
        rows.push(stream(&format!("{width} 0 d0 {}", glyph.as_str().unwrap())));
    }
    let input = pdf_objects_bytes(&rows);
    let snapshot = crate::standards::v1_7::subsets::base::io::import::decode_pdf(input.as_bytes()).unwrap();
    let scene: Value = serde_json::from_str(&canvas_layers(&snapshot, None)).unwrap();
    let scene = scene.as_array().unwrap();
    assert_eq!(scene.len(), 3);
    for glyph in 0..2 {
        let layer = &scene[glyph + 1];
        assert!(layer["id"].as_str().unwrap().starts_with("p0:t0/"));
        assert_point(&layer["segments"][0]["to"], &fixture["starts"][glyph]);
        assert_eq!(layer["fill"]["color"][if glyph == 0 { 0 } else { 2 }].as_f64(), Some(1.0));
    }
    for rendering in fixture["renderModes"].as_array().unwrap() {
        let mode = rendering["mode"].as_u64().unwrap();
        let content = fixture["pageContent"].as_str().unwrap().replacen(" Tf ", &format!(" Tf {mode} Tr "), 1);
        rows[3] = stream(&content);
        let input = pdf_objects_bytes(&rows);
        let decoded = crate::standards::v1_7::subsets::base::io::import::decode_pdf(input.as_bytes()).unwrap();
        let actual: Vec<Value> = serde_json::from_str(&canvas_layers(&decoded, None)).unwrap();
        if rendering["visible"].as_bool().unwrap() {
            assert_eq!(&actual, scene);
        } else {
            assert_eq!(actual.len(), 1);
        }
    }
}

#[test]
fn canvas_paints_the_embedded_true_type_program_as_curved_outlines() {
    use std::fmt::Write;
    let case: Value = serde_json::from_str(include_str!("🧪️truetype-outline.json")).unwrap();
    let font = include_bytes!("../../../../../../../../../../../../../🧰️framework/🛍️products/📓️print/🖼️assets/🔤️font/🖥️share-tech-mono/💻️ShareTechMono-Regular.ttf");
    let widths = &case["widths"];
    let (first, last, advance) = (widths["first"].as_u64().unwrap(), widths["last"].as_u64().unwrap(), widths["advance"].as_u64().unwrap());
    let advances = (first..=last).map(|_| advance.to_string()).collect::<Vec<_>>().join(" ");
    let mut hex = String::with_capacity(font.len() * 2 + 1);
    for byte in font { write!(hex, "{byte:02x}").unwrap(); }
    hex.push('>');
    let content = format!("BT /F1 32 Tf 1 0 0 1 20 40 Tm <{}> Tj ET", case["codes"].as_str().unwrap());
    let pdf = |content: &str| pdf_objects_bytes(&[
        "<< /Type /Catalog /Pages 2 0 R >>".into(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".into(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {} {}] /Resources << /Font << /F1 5 0 R >> >> /Contents 4 0 R >>", case["page"][0], case["page"][1]),
        format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len()),
        format!("<< /Type /Font /Subtype /TrueType /BaseFont /ShareTechMono-Regular /Encoding /WinAnsiEncoding /FontDescriptor 6 0 R /FirstChar {first} /LastChar {last} /Widths [{advances}] >>"),
        "<< /Type /FontDescriptor /FontName /ShareTechMono-Regular /Flags 32 /FontBBox [0 -250 700 850] /ItalicAngle 0 /Ascent 800 /Descent -200 /CapHeight 675 /StemV 80 /FontFile2 7 0 R >>".into(),
        format!("<< /Filter /ASCIIHexDecode /Length {} /Length1 {} >>\nstream\n{hex}\nendstream", hex.len(), font.len()),
    ]);
    let decode = |content: &str| crate::standards::v1_7::subsets::base::io::import::decode_pdf(pdf(content).as_bytes()).unwrap();
    let decoded = decode(&content);
    assert_eq!(decoded.pages[0].text(), case["paintText"].as_str().unwrap());
    let scene: Value = serde_json::from_str(&canvas_layers(&decoded, None)).unwrap();
    assert_eq!(scene[1]["id"].as_str(), Some("p0:t0"));
    assert!(scene[1].get("text").is_none());
    let matrix = |value: &Value| serde_json::from_value::<[f64; 6]>(value.clone()).unwrap();
    assert_eq!(matrix(&scene[1]["transform"]), matrix(&case["matrix"]));
    let segments = scene[1]["segments"].as_array().unwrap();
    assert!(segments.len() >= 32);
    assert!(segments.iter().any(|segment| segment["kind"] == "cubic"));
    let clip = decode(case["clip"]["content"].as_str().unwrap());
    let scene: Value = serde_json::from_str(&canvas_layers(&clip, None)).unwrap();
    let layers = scene.as_array().unwrap();
    assert_eq!(layers.len(), 3, "clip-only glyphs themselves do not paint");
    let paths = layers[1]["clipPaths"].as_array().unwrap();
    assert_eq!(paths.len(), 1, "a text object contributes one combined clip");
    assert_eq!(paths[0]["fillRule"], "nonzero");
    let glyphs = paths[0]["segments"].as_array().unwrap();
    for region in case["clip"]["regions"].as_array().unwrap() {
        let [left, top, right, bottom] = serde_json::from_value::<[f64; 4]>(region.clone()).unwrap();
        assert!(glyphs.iter().filter_map(|segment| segment.get("to")).any(|point| {
            let x = point[0].as_f64().unwrap();
            let y = point[1].as_f64().unwrap();
            (left..=right).contains(&x) && (top..=bottom).contains(&y)
        }));
    }
    assert!(layers[2].get("clipPaths").is_none(), "Q restores the preceding clip");
    let mut raw = decoded;
    let shown = raw.pages[0].content.iter_mut().find_map(|op| match op { PdfOp::ShowText { text } => Some(text), _ => None }).unwrap();
    *shown = PdfTextString::Codes { codes: vec![65, 103, 233, 128] };
    let objects = objects(&raw);
    assert_eq!(objects[0].text, case["paintText"].as_str().unwrap());
    assert!((objects[0].width - 64.0).abs() < 0.001);
}

#[path = "🔆️coded-image/🦀️.rs"]
mod coded_image;

#[path = "🌈️paint-fidelity/🦀️.rs"]
mod paint_fidelity;

fn navigation_fixture() -> (Value, PdfSnapshot) {
    let fixture: Value = serde_json::from_str(include_str!("🧪️viewer-navigation.json")).unwrap();
    let mut snapshot = PdfSnapshot::default();
    for spec in fixture["pages"].as_array().unwrap() {
        let mut page = PdfPage::new(spec["width"].as_f64().unwrap(), spec["height"].as_f64().unwrap());
        page.content.push(PdfOp::BeginText);
        page.content.push(PdfOp::SetFont { name: "F1".into(), size: 12.0 });
        for (line, text) in spec["texts"].as_array().unwrap().iter().enumerate() {
            page.content.push(PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 15.0, 250.0 - 30.0 * line as f64] });
            page.content.push(PdfOp::ShowText { text: PdfTextString::text(text.as_str().unwrap()) });
        }
        page.content.push(PdfOp::EndText);
        snapshot.pages.push(page);
    }
    (fixture, snapshot)
}

#[test]
fn zoom_modes_and_steps_match_the_fixture() {
    let (fixture, snapshot) = navigation_fixture();
    let viewport = (fixture["viewport"][0].as_f64().unwrap(), fixture["viewport"][1].as_f64().unwrap());
    for case in fixture["fit"].as_array().unwrap() {
        let mode = match case["mode"].as_str().unwrap() {
            "fit-width" => ZoomMode::FitWidth,
            "fit-page" => ZoomMode::FitPage,
            "actual-size" => ZoomMode::ActualSize,
            _ => ZoomMode::Factor(case["factor"].as_f64().unwrap()),
        };
        let view = ViewState { mode, page: case["page"].as_u64().unwrap() as usize, ..ViewState::default() };
        assert!((resolve_zoom(&snapshot, &view, viewport) - case["zoom"].as_f64().unwrap()).abs() < 1e-9, "{case}");
    }
    for pair in fixture["zoom_in"].as_array().unwrap() {
        assert_eq!(zoom_in(pair[0].as_f64().unwrap()), pair[1].as_f64().unwrap());
    }
    for pair in fixture["zoom_out"].as_array().unwrap() {
        assert_eq!(zoom_out(pair[0].as_f64().unwrap()), pair[1].as_f64().unwrap());
    }
    let view = ViewState { mode: ZoomMode::Factor(2.0), page: 1, ..ViewState::default() };
    let scene = scene_for_view(&snapshot, &canvas_layers(&snapshot, None), &view, viewport, Vec::new());
    assert_eq!((scene.zoom, scene.camera_x, scene.camera_y), (2.0, 300.0, page_top(&snapshot, 1) + 200.0));
    assert!(scene.framing.is_none());
    let fit = scene_for_view(&snapshot, &canvas_layers(&snapshot, None), &ViewState { mode: ZoomMode::FitPage, page: 1, revision: 4, ..ViewState::default() }, viewport, Vec::new()).framing.unwrap();
    assert_eq!((fit.revision, fit.bounds[2], fit.bounds[3] - fit.bounds[1]), (4, 600.0, 400.0));
    let width = scene_for_view(&snapshot, &canvas_layers(&snapshot, None), &ViewState { mode: ZoomMode::FitWidth, ..ViewState::default() }, viewport, Vec::new()).framing.unwrap();
    assert_eq!(width.bounds[3], width.bounds[1]);
    assert!(page_top(&snapshot, 1) > 300.0);
}

#[test]
fn search_finds_every_hit_and_matches_the_oracle_fixture() {
    let (fixture, snapshot) = navigation_fixture();
    for case in fixture["search"].as_array().unwrap() {
        let mut progress = Vec::new();
        let outcome = search(&snapshot, case["query"].as_str().unwrap(), case["match_case"].as_bool().unwrap(), &mut SearchControl { cancelled: &|| false, progress: &mut |done, total| progress.push((done, total)) });
        let hits: Vec<Value> = outcome.hits.iter().map(|hit| json!([hit.page, hit.offset])).collect();
        assert_eq!(Value::Array(hits), case["hits"], "{case}");
        assert!(outcome.completed);
        assert_eq!(progress.last(), Some(&(3, 3)));
    }
}

#[test]
fn search_cancellation_stops_before_the_next_page_and_keeps_the_hits_so_far() {
    let (_, snapshot) = navigation_fixture();
    let scanned = std::cell::Cell::new(0);
    let outcome = search(&snapshot, "o", false, &mut SearchControl { cancelled: &|| scanned.get() >= 1, progress: &mut |done, _| scanned.set(done) });
    assert!(!outcome.completed);
    assert!(!outcome.hits.is_empty() && outcome.hits.iter().all(|hit| hit.page == 0));
}

#[test]
fn hit_navigation_wraps_and_highlights_layer_per_hit() {
    let (fixture, snapshot) = navigation_fixture();
    for step in fixture["steps"].as_array().unwrap() {
        assert_eq!(step_hit(step[0].as_u64().unwrap() as usize, step[1].as_u64().map(|at| at as usize), step[2].as_bool().unwrap()), step[3].as_u64().map(|at| at as usize));
    }
    let outcome = search(&snapshot, "world", false, &mut SearchControl { cancelled: &|| false, progress: &mut |_, _| {} });
    let layers = hit_layers(&snapshot, &outcome.hits, Some(1));
    assert_eq!(layers.len(), 2);
    assert_ne!(layers[0]["fill"]["color"], layers[1]["fill"]["color"]);
}

#[test]
fn thumbnails_scale_each_page_and_reorder_through_move_page() {
    let (fixture, snapshot) = navigation_fixture();
    for (thumbnail, expected) in thumbnails(&snapshot, THUMBNAIL_EDGE).iter().zip(fixture["thumbnails"].as_array().unwrap()) {
        assert_eq!((thumbnail.width, thumbnail.height), (expected[0].as_f64().unwrap(), expected[1].as_f64().unwrap()));
        assert_eq!(thumbnail.target, format!("p{}:paper", thumbnail.page));
    }
    for case in fixture["reorder"].as_array().unwrap() {
        let expected = case[3].as_array().map(|pair| (pair[0].as_u64().unwrap() as usize, pair[1].as_u64().unwrap() as usize));
        assert_eq!(reorder_args(case[0].as_u64().unwrap() as usize, case[1].as_u64().unwrap() as usize, case[2].as_u64().unwrap() as usize), expected);
    }
    let _ui_arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    for locale in [Locale::En, Locale::De] {
        assert!(render_thumbnail_strip(&snapshot, "s.stdio.pdf@1.7/*#editor", OBJECT_DOMAIN, locale).is_ok());
    }
}

#[test]
fn view_intents_reduce_to_the_fixture_script() {
    let (fixture, snapshot) = navigation_fixture();
    let viewport = (fixture["viewport"][0].as_f64().unwrap(), fixture["viewport"][1].as_f64().unwrap());
    let mut view = ViewState::default();
    for step in fixture["script"].as_array().unwrap() {
        let hits = search(&snapshot, &view.query, view.match_case, &mut SearchControl { cancelled: &|| false, progress: &mut |_, _| {} }).hits;
        let intent = match step["intent"].as_str().unwrap() {
            "zoom-in" => ViewIntent::ZoomIn,
            "zoom-out" => ViewIntent::ZoomOut,
            "fit-page" => ViewIntent::FitPage,
            "fit-width" => ViewIntent::FitWidth,
            "actual-size" => ViewIntent::ActualSize,
            "go-to-page" => ViewIntent::GoToPage(step["arg"].as_u64().unwrap() as usize),
            "set-query" => ViewIntent::SetQuery(step["arg"].as_str().unwrap().into()),
            "toggle-match-case" => ViewIntent::ToggleMatchCase,
            "next-hit" => ViewIntent::NextHit,
            _ => ViewIntent::PreviousHit,
        };
        view = reduce_view(&snapshot, &view, &intent, viewport, &hits);
        let expect = &step["expect"];
        let mode = match view.mode { ZoomMode::Factor(factor) => { assert_eq!(Some(factor), expect["factor"].as_f64(), "{step}"); "factor" } ZoomMode::FitPage => "fit-page", ZoomMode::FitWidth => "fit-width", ZoomMode::ActualSize => "actual-size" };
        assert_eq!(mode, expect["mode"], "{step}");
        assert_eq!((view.page as u64, view.revision as u64, view.query.as_str(), view.match_case, view.active.map(|at| at as u64)), (expect["page"].as_u64().unwrap(), expect["revision"].as_u64().unwrap(), expect["query"].as_str().unwrap(), expect["match_case"].as_bool().unwrap(), expect["active"].as_u64()), "{step}");
    }
}
