use super::*;
use crate::standards::v1_7::subsets::base::schema::mutations::apply_pdf_mutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotationKind, PdfImage, PdfOp, PdfPage, PdfPageLayout, PdfPageMode, PdfTextString};

fn apply(snapshot: &mut PdfSnapshot, edit: PdfPageEdit) {
    let mutations = apply_payload(snapshot, &edit.action, &edit.payload).expect("page edit");
    for mutation in &mutations {
        apply_pdf_mutation(snapshot, mutation);
    }
}

fn args(entries: Vec<(&str, dsl::DslValue)>) -> dsl::DslValue {
    dsl::DslValue::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
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
    assert!(layers.contains("Hello"));
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
    let edit = edit_from_action("set-text", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(text.id)), ("text", dsl::DslValue::String("World".into()))]))).expect("edit");
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
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(text.id)), ("x", dsl::DslValue::float(20.0)), ("y", dsl::DslValue::float(240.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(vector.id.clone())), ("x", dsl::DslValue::float(12.0)), ("y", dsl::DslValue::float(22.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(image.id)), ("x", dsl::DslValue::float(62.0)), ("y", dsl::DslValue::float(72.0))]))).unwrap());
    let moved = objects(&snapshot);
    let text = moved.iter().find(|object| object.kind == ObjectKind::Text).unwrap();
    let vector = moved.iter().find(|object| object.kind == ObjectKind::Vector).unwrap();
    let image = moved.iter().find(|object| object.kind == ObjectKind::Image).unwrap();
    assert!((text.x - 20.0).abs() < 0.01 && (text.y - 240.0).abs() < 0.01);
    assert!((vector.x - 12.0).abs() < 0.01 && (vector.y - 22.0).abs() < 0.01);
    assert!((image.x - 62.0).abs() < 0.01 && (image.y - 72.0).abs() < 0.01);
    let vector_id = vector.id.clone();
    apply(&mut snapshot, edit_from_action("delete", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(vector_id))]))).unwrap());
    assert!(objects(&snapshot).iter().all(|object| object.kind != ObjectKind::Vector));
    assert!(snapshot.pages[0].content.iter().any(|op| matches!(op, PdfOp::ShowText { .. })));
    assert!(snapshot.pages[0].content.iter().any(|op| matches!(op, PdfOp::PaintXObject { .. })));
}

#[test]
fn insert_text_rectangle_and_image_round_trip_onto_the_page() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("insert-text", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(4.0)), ("y", dsl::DslValue::float(8.0)), ("text", dsl::DslValue::String("Added".into())), ("height", dsl::DslValue::float(14.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("insert-rectangle", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(1.0)), ("y", dsl::DslValue::float(2.0)), ("width", dsl::DslValue::float(3.0)), ("height", dsl::DslValue::float(4.0)), ("red", dsl::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("insert-image", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(5.0)), ("y", dsl::DslValue::float(6.0)), ("width", dsl::DslValue::float(7.0)), ("height", dsl::DslValue::float(8.0))]))).unwrap());
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
    apply(&mut snapshot, edit_from_action("set-info", Some(&args(vec![("text", dsl::DslValue::String("Title".into())), ("extra", dsl::DslValue::String("Author".into())), ("object", dsl::DslValue::String("Subject".into()))]))).unwrap());
    assert_eq!(snapshot.info.title.as_deref(), Some("Title"));
    assert_eq!(snapshot.info.author.as_deref(), Some("Author"));
    assert_eq!(snapshot.info.subject.as_deref(), Some("Subject"));
    apply(&mut snapshot, edit_from_action("insert-page", Some(&args(vec![("page", dsl::DslValue::float(1.0)), ("width", dsl::DslValue::float(100.0)), ("height", dsl::DslValue::float(120.0))]))).unwrap());
    assert_eq!(snapshot.pages.len(), 2);
    assert!((snapshot.pages[1].width() - 100.0).abs() < 0.01);
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation");
    apply(&mut snapshot, edit_from_action("set-annotation", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(annotation.id)), ("text", dsl::DslValue::String("Note".into())), ("x", dsl::DslValue::float(3.0)), ("y", dsl::DslValue::float(4.0)), ("width", dsl::DslValue::float(5.0)), ("height", dsl::DslValue::float(6.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].annotations[0].contents.as_deref(), Some("Note"));
    assert_eq!(snapshot.pages[0].annotations[0].rect, [3.0, 4.0, 8.0, 10.0]);
    apply(&mut snapshot, edit_from_action("set-page-size", Some(&args(vec![("page", dsl::DslValue::float(1.0)), ("width", dsl::DslValue::float(90.0)), ("height", dsl::DslValue::float(80.0))]))).unwrap());
    assert!((snapshot.pages[1].width() - 90.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("move-page", Some(&args(vec![("page", dsl::DslValue::float(1.0)), ("x", dsl::DslValue::float(0.0))]))).unwrap());
    assert!((snapshot.pages[0].width() - 90.0).abs() < 0.01);
}

#[test]
fn set_image_replaces_samples_and_keeps_the_placement() {
    let mut snapshot = sample();
    let image = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    let edit = edit_from_action(
        "set-image",
        Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(image.id)), ("width", dsl::DslValue::float(2.0)), ("height", dsl::DslValue::float(2.0)), ("text", dsl::DslValue::String("ff000000ff000000ffff0000".into()))])),
    )
    .expect("edit");
    apply(&mut snapshot, edit);
    assert_eq!(snapshot.images[0].data, vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 0, 0]);
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
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(form.id)), ("x", dsl::DslValue::float(10.0)), ("y", dsl::DslValue::float(11.0))]))).unwrap());
    let form = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Form).expect("form");
    assert!((form.x - 10.0).abs() < 0.01 && (form.y - 11.0).abs() < 0.01);
    let shading = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Shading).expect("shading");
    assert!((shading.x - 10.0).abs() < 0.01 && (shading.y - 10.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(shading.id)), ("x", dsl::DslValue::float(12.0)), ("y", dsl::DslValue::float(14.0))]))).unwrap());
    let PdfShadingKind::Axial { coords, .. } = &snapshot.shadings[0].kind else { panic!("axial shading") };
    assert!((coords[0] - 12.0).abs() < 0.01 && (coords[1] - 24.0).abs() < 0.01);
    assert_eq!(snapshot.shadings[0].bbox, Some([12.0, 14.0, 42.0, 34.0]));
}

#[test]
fn set_font_and_set_outline_publish_onto_the_document() {
    let mut snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    apply(&mut snapshot, edit_from_action("set-font", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(text.id)), ("text", dsl::DslValue::String("Times-Roman".into()))]))).unwrap());
    assert!(snapshot.fonts.iter().any(|font| font.base_font() == "Times-Roman"));
    let show = snapshot.pages[0].content.iter().position(|op| matches!(op, PdfOp::ShowText { .. })).expect("show");
    let PdfOp::SetFont { name, size } = &snapshot.pages[0].content[show - 1] else { panic!("font before the run") };
    assert_eq!(snapshot.fonts.iter().find(|font| font.id == *name).expect("font").base_font(), "Times-Roman");
    assert!((*size - 12.0).abs() < 0.01);
    assert_eq!(snapshot.pages[0].text(), "Hello");
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("Start".into()))]))).unwrap());
    assert_eq!(snapshot.outlines[0].title, "Start");
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("Begin".into()))]))).unwrap());
    assert_eq!(snapshot.outlines.len(), 1);
    assert_eq!(snapshot.outlines[0].title, "Begin");
}

#[test]
fn page_viewing_settings_publish_onto_the_document() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-page-rotation", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(90.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].rotate, 90);
    apply(&mut snapshot, edit_from_action("set-page-box", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("crop".into())), ("x", dsl::DslValue::float(1.0)), ("y", dsl::DslValue::float(2.0)), ("width", dsl::DslValue::float(30.0)), ("height", dsl::DslValue::float(40.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].crop_box, Some([1.0, 2.0, 31.0, 42.0]));
    apply(&mut snapshot, edit_from_action("set-page-user-unit", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(2.5))]))).unwrap());
    assert_eq!(snapshot.pages[0].user_unit, Some(2.5));
    apply(&mut snapshot, edit_from_action("set-language", Some(&args(vec![("text", dsl::DslValue::String("de".into()))]))).unwrap());
    assert_eq!(snapshot.language.as_deref(), Some("de"));
    apply(&mut snapshot, edit_from_action("set-page-layout", Some(&args(vec![("text", dsl::DslValue::String("twoColumnLeft".into()))]))).unwrap());
    assert_eq!(snapshot.page_layout, Some(PdfPageLayout::TwoColumnLeft));
    apply(&mut snapshot, edit_from_action("set-page-mode", Some(&args(vec![("text", dsl::DslValue::String("useOutlines".into()))]))).unwrap());
    assert_eq!(snapshot.page_mode, Some(PdfPageMode::UseOutlines));
    let layers = canvas_layers(&snapshot, None);
    assert!(layers.contains("p0:paper"));
}

#[test]
fn catalog_collections_publish_onto_the_document() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDestination, PdfEncryptionAlgorithm, PdfPageLabelStyle};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-optional-content", Some(&args(vec![("object", dsl::DslValue::String("OC1".into())), ("text", dsl::DslValue::String("Layer".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    let content = snapshot.optional_content.as_ref().expect("layers");
    assert_eq!(content.groups[0].name, "Layer");
    assert_eq!(content.on, vec!["OC1".to_string()]);
    apply(&mut snapshot, edit_from_action("set-embedded-file", Some(&args(vec![("object", dsl::DslValue::String("note".into())), ("text", dsl::DslValue::String("note.txt".into())), ("extra", dsl::DslValue::String("hello".into()))]))).unwrap());
    assert_eq!(snapshot.embedded_files[0].data, b"hello");
    apply(&mut snapshot, edit_from_action("remove-embedded-file", Some(&args(vec![("object", dsl::DslValue::String("note".into()))]))).unwrap());
    assert!(snapshot.embedded_files.is_empty());
    apply(&mut snapshot, edit_from_action("set-named-destination", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("Start".into()))]))).unwrap());
    assert!(matches!(snapshot.named_destinations[0].destination, PdfDestination::Page { page: 0, .. }));
    apply(&mut snapshot, edit_from_action("set-page-label", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("decimal".into())), ("extra", dsl::DslValue::String("A-".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    assert_eq!(snapshot.page_labels[0].style, Some(PdfPageLabelStyle::Decimal));
    assert_eq!(snapshot.page_labels[0].prefix.as_deref(), Some("A-"));
    apply(&mut snapshot, edit_from_action("set-mark-info", Some(&args(vec![("x", dsl::DslValue::float(1.0))]))).unwrap());
    assert!(snapshot.mark_info.as_ref().expect("marks").marked);
    apply(&mut snapshot, edit_from_action("set-metadata", Some(&args(vec![("text", dsl::DslValue::String("<xmp/>".into()))]))).unwrap());
    assert_eq!(snapshot.metadata.as_deref(), Some("<xmp/>"));
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", dsl::DslValue::String("hideToolbar".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    assert!(snapshot.viewer_preferences.as_ref().expect("viewer").hide_toolbar);
    apply(&mut snapshot, edit_from_action("set-encryption", Some(&args(vec![("text", dsl::DslValue::String("aes256".into())), ("extra", dsl::DslValue::String("user".into())), ("object", dsl::DslValue::String("owner".into()))]))).unwrap());
    let encryption = snapshot.encryption.as_ref().expect("encryption");
    assert_eq!(encryption.algorithm, PdfEncryptionAlgorithm::Aes256);
    assert_eq!(encryption.user_password, "user");
    assert_eq!(encryption.owner_password.as_deref(), Some("owner"));
    apply(&mut snapshot, edit_from_action("remove-named-destination", Some(&args(vec![("object", dsl::DslValue::String("Start".into()))]))).unwrap());
    assert!(snapshot.named_destinations.is_empty());
}

#[test]
fn mesh_move_shifts_the_decode_range() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfColorSpace, PdfShading, PdfShadingKind};
    let mut snapshot = sample();
    snapshot.pages[0].content.push(PdfOp::PaintShading { name: "ShM".into() });
    snapshot.shadings.push(PdfShading { id: "ShM".into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Mesh { shading_type: 4, bits_per_coordinate: 8, bits_per_component: 8, bits_per_flag: None, vertices_per_row: None, decode: vec![0.0, 10.0, 0.0, 10.0], function: None, data: vec![0] }, background: None, bbox: Some([0.0, 0.0, 10.0, 10.0]), anti_alias: false, extra: Vec::new() });
    let shading = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Shading).expect("mesh");
    apply(&mut snapshot, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(shading.id)), ("x", dsl::DslValue::float(2.0)), ("y", dsl::DslValue::float(3.0))]))).unwrap());
    let PdfShadingKind::Mesh { decode, data, .. } = &snapshot.shadings[0].kind else { panic!("mesh") };
    assert_eq!(decode, &vec![2.0, 12.0, 3.0, 13.0]);
    assert_eq!(data, &vec![0]);
    assert_eq!(snapshot.shadings[0].bbox, Some([2.0, 3.0, 12.0, 13.0]));
}

#[test]
fn conformance_form_identity_and_font_program_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfFontKind, PdfFontProgram, PdfFormFieldKind, PdfOpenAction};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-output-intent", Some(&args(vec![("object", dsl::DslValue::String("GTS_PDFA1".into())), ("text", dsl::DslValue::String("sRGB".into())), ("extra", dsl::DslValue::String("sRGB IEC61966".into()))]))).unwrap());
    assert_eq!(snapshot.output_intents[0].condition_identifier, "sRGB");
    assert_eq!(snapshot.output_intents[0].info.as_deref(), Some("sRGB IEC61966"));
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", dsl::DslValue::String("Name".into())), ("text", dsl::DslValue::String("Ada".into())), ("extra", dsl::DslValue::String("text".into()))]))).unwrap());
    let form = snapshot.acro_form.as_ref().expect("form");
    assert!(form.need_appearances);
    let PdfFormFieldKind::Text { value, .. } = &form.fields[0].kind else { panic!("text field") };
    assert_eq!(value.as_deref(), Some("Ada"));
    apply(&mut snapshot, edit_from_action("set-open-action", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("uri".into())), ("text", dsl::DslValue::String("https://semio.example".into()))]))).unwrap());
    let PdfOpenAction::Action { action } = snapshot.open_action.as_ref().expect("open") else { panic!("uri action") };
    assert!(matches!(action.kind, crate::standards::v1_7::subsets::base::schema::snapshot::PdfActionKind::Uri { ref uri, .. } if uri == "https://semio.example"));
    apply(&mut snapshot, edit_from_action("set-document-id", Some(&args(vec![("text", dsl::DslValue::String("abc".into())), ("extra", dsl::DslValue::String("def".into()))]))).unwrap());
    assert_eq!(snapshot.document_id.as_ref().expect("id"), &[b"abc".to_vec(), b"def".to_vec()]);
    apply(&mut snapshot, edit_from_action("set-font-program", Some(&args(vec![("object", dsl::DslValue::String("F1".into())), ("text", dsl::DslValue::String("truetype".into())), ("extra", dsl::DslValue::String("0001".into()))]))).unwrap());
    let PdfFontKind::TrueType { program, .. } = &snapshot.fonts[0].kind else { panic!("truetype font") };
    let Some(PdfFontProgram::TrueType { data }) = program else { panic!("program") };
    assert_eq!(data, &vec![0, 1]);
}

#[test]
fn resources_and_font_metrics_publish_onto_the_document() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfColorSpace, PdfFontKind, PdfObject, PdfPatternKind};
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-graphics-state", Some(&args(vec![("object", dsl::DslValue::String("GS1".into())), ("text", dsl::DslValue::String("Multiply".into())), ("x", dsl::DslValue::float(0.5)), ("y", dsl::DslValue::float(0.25))]))).unwrap());
    assert_eq!(snapshot.ext_g_states[0].fill_alpha, Some(0.5));
    assert_eq!(snapshot.ext_g_states[0].stroke_alpha, Some(0.25));
    assert_eq!(snapshot.ext_g_states[0].blend_mode.as_deref(), Some(["Multiply".to_string()].as_slice()));
    apply(&mut snapshot, edit_from_action("set-pattern", Some(&args(vec![("object", dsl::DslValue::String("P1".into())), ("x", dsl::DslValue::float(0.0)), ("y", dsl::DslValue::float(0.0)), ("width", dsl::DslValue::float(8.0)), ("height", dsl::DslValue::float(8.0))]))).unwrap());
    let PdfPatternKind::Tiling { x_step, y_step, .. } = &snapshot.patterns[0].kind else { panic!("tiling") };
    assert!((*x_step - 8.0).abs() < 0.01 && (*y_step - 8.0).abs() < 0.01);
    apply(&mut snapshot, edit_from_action("set-color-space", Some(&args(vec![("object", dsl::DslValue::String("CS1".into())), ("text", dsl::DslValue::String("deviceCmyk".into()))]))).unwrap());
    assert!(matches!(snapshot.color_spaces[0].color_space, PdfColorSpace::DeviceCmyk));
    apply(&mut snapshot, edit_from_action("set-properties", Some(&args(vec![("object", dsl::DslValue::String("MC1".into())), ("text", dsl::DslValue::String("OC".into())), ("extra", dsl::DslValue::String("OC1".into()))]))).unwrap());
    assert!(matches!(&snapshot.properties[0].entries[0].value, PdfObject::Name(name) if name == "OC1"));
    apply(&mut snapshot, edit_from_action("set-font-metrics", Some(&args(vec![("object", dsl::DslValue::String("F1".into())), ("text", dsl::DslValue::String("macRoman".into())), ("extra", dsl::DslValue::String("500,600".into())), ("x", dsl::DslValue::float(32.0)), ("y", dsl::DslValue::float(720.0))]))).unwrap());
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
    apply(&mut snapshot, edit_from_action("set-image-mask", Some(&args(vec![("object", dsl::DslValue::String("Im1".into())), ("text", dsl::DslValue::String("colorKey".into())), ("extra", dsl::DslValue::String("0,255".into()))]))).unwrap());
    let PdfImageMask::ColorKey { ranges } = snapshot.images[0].mask.as_ref().expect("mask") else { panic!("color key") };
    assert_eq!(ranges, &vec![0, 255]);
    apply(&mut snapshot, edit_from_action("set-form-content", Some(&args(vec![("object", dsl::DslValue::String("Fm1".into())), ("text", dsl::DslValue::String("Inside".into())), ("x", dsl::DslValue::float(4.0)), ("y", dsl::DslValue::float(6.0)), ("width", dsl::DslValue::float(20.0)), ("height", dsl::DslValue::float(12.0))]))).unwrap());
    assert!(snapshot.forms[0].content.iter().any(|op| matches!(op, PdfOp::ShowText { text: PdfTextString::Text { text } } if text == "Inside")));
    apply(&mut snapshot, edit_from_action("set-page-transition", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("Wipe".into())), ("extra", dsl::DslValue::String("None".into())), ("x", dsl::DslValue::float(2.0))]))).unwrap());
    let transition = snapshot.pages[0].transition.as_ref().expect("transition");
    assert!(matches!(&transition[0].value, PdfObject::Name(name) if name == "Wipe"));
    assert_eq!(snapshot.pages[0].duration, Some(2.0));
    apply(&mut snapshot, edit_from_action("set-catalog-entry", Some(&args(vec![("object", dsl::DslValue::String("Marker".into())), ("text", dsl::DslValue::String("Yes".into()))]))).unwrap());
    assert!(matches!(&snapshot.catalog_extra[0].value, PdfObject::Name(name) if name == "Yes"));
    apply(&mut snapshot, edit_from_action("set-trailer-entry", Some(&args(vec![("object", dsl::DslValue::String("Info".into())), ("text", dsl::DslValue::String("Kept".into()))]))).unwrap());
    assert!(matches!(&snapshot.trailer[0].value, PdfObject::Name(name) if name == "Kept"));
    apply(&mut snapshot, edit_from_action("set-catalog-entry", Some(&args(vec![("object", dsl::DslValue::String("Marker".into())), ("text", dsl::DslValue::String(String::new()))]))).unwrap());
    assert!(snapshot.catalog_extra.is_empty());
}

#[test]
fn appearances_glyphs_objects_and_mesh_bytes_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAnnotation, PdfAnnotationKind, PdfAppearanceEntry, PdfColorSpace, PdfFontKind, PdfObject, PdfOp, PdfShading, PdfShadingKind, PdfSimpleEncoding};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation");
    apply(&mut snapshot, edit_from_action("set-annotation-appearance", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(annotation.id)), ("text", dsl::DslValue::String("Fm1".into()))]))).unwrap());
    let PdfAppearanceEntry::Single { form } = &snapshot.pages[0].annotations[0].appearance.as_ref().expect("appearance").normal else { panic!("single appearance") };
    assert_eq!(form, "Fm1");
    snapshot.fonts[0].kind = PdfFontKind::Type3 { font_matrix: [0.001, 0.0, 0.0, 0.001, 0.0, 0.0], font_bbox: [0.0, 0.0, 1000.0, 1000.0], encoding: PdfSimpleEncoding::default(), first_char: 0, widths: Vec::new(), char_procs: Vec::new(), descriptor: None };
    apply(&mut snapshot, edit_from_action("set-glyph", Some(&args(vec![("object", dsl::DslValue::String("F1".into())), ("text", dsl::DslValue::String("A".into())), ("x", dsl::DslValue::float(0.0)), ("y", dsl::DslValue::float(0.0)), ("width", dsl::DslValue::float(10.0)), ("height", dsl::DslValue::float(12.0))]))).unwrap());
    let PdfFontKind::Type3 { char_procs, .. } = &snapshot.fonts[0].kind else { panic!("type 3") };
    assert_eq!(char_procs[0].name, "A");
    assert!(char_procs[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { width, height, .. } if (*width - 10.0).abs() < 0.01 && (*height - 12.0).abs() < 0.01)));
    apply(&mut snapshot, edit_from_action("set-indirect-object", Some(&args(vec![("x", dsl::DslValue::float(7.0)), ("y", dsl::DslValue::float(0.0)), ("text", dsl::DslValue::String("Hello".into()))]))).unwrap());
    assert!(matches!(&snapshot.objects[0].value, PdfObject::Name(name) if name == "Hello"));
    assert_eq!(snapshot.objects[0].id.num, 7);
    snapshot.shadings.push(PdfShading { id: "ShM".into(), color_space: PdfColorSpace::DeviceRgb, kind: PdfShadingKind::Mesh { shading_type: 4, bits_per_coordinate: 8, bits_per_component: 8, bits_per_flag: None, vertices_per_row: None, decode: vec![0.0, 1.0, 0.0, 1.0], function: None, data: vec![0] }, background: None, bbox: None, anti_alias: false, extra: Vec::new() });
    apply(&mut snapshot, edit_from_action("set-mesh-data", Some(&args(vec![("object", dsl::DslValue::String("ShM".into())), ("text", dsl::DslValue::String("0,10,0,10".into())), ("extra", dsl::DslValue::String("00ff".into()))]))).unwrap());
    let PdfShadingKind::Mesh { data, decode, .. } = &snapshot.shadings[0].kind else { panic!("mesh") };
    assert_eq!(data, &vec![0, 255]);
    assert_eq!(decode, &vec![0.0, 10.0, 0.0, 10.0]);
}

#[test]
fn pointer_down_selects_the_hit_object_and_clears_a_miss() {
    let snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let x = text.x + 2.0;
    let y = 300.0 - (text.y + text.font_size) + 2.0;
    let edit = edit_from_action("canvasPointerDown", Some(&args(vec![("x", dsl::DslValue::float(x)), ("y", dsl::DslValue::float(y))]))).expect("pointer");
    let emit = emit_page_edit(&snapshot, &edit.action, &edit.payload).expect("select");
    assert!(emit.artifact_mutations.is_empty());
    let semio_framework_plugin::Effect::DispatchAction { action, args: effect_args, .. } = &emit.effects[0] else { panic!("select effect") };
    assert_eq!(action, "interactionSelect");
    let value = serde_json::Value::from(effect_args.clone().expect("args"));
    assert_eq!(value["domainId"], "objects");
    assert_eq!(value["merge"], "replace");
    assert!(value["targets"].as_str().expect("targets").contains(&text.id));
    let miss = edit_from_action("canvasPointerDown", Some(&args(vec![("x", dsl::DslValue::float(1.0)), ("y", dsl::DslValue::float(1.0))]))).expect("miss");
    let cleared = emit_page_edit(&snapshot, &miss.action, &miss.payload).expect("clear");
    let semio_framework_plugin::Effect::DispatchAction { action, args: cleared_args, .. } = &cleared.effects[0] else { panic!("clear effect") };
    assert_eq!(action, "clearSelection");
    assert!(cleared_args.is_none());
    let shifted = edit_from_action("canvasPointerDown", Some(&args(vec![("x", dsl::DslValue::float(x)), ("y", dsl::DslValue::float(y)), ("shift", dsl::DslValue::Bool(true))]))).expect("shift");
    let toggled = emit_page_edit(&snapshot, &shifted.action, &shifted.payload).expect("invert");
    let semio_framework_plugin::Effect::DispatchAction { args: effect_args, .. } = &toggled.effects[0] else { panic!("invert effect") };
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
    apply(&mut snapshot, edit_from_action("set-info-field", Some(&args(vec![("object", dsl::DslValue::String("keywords".into())), ("text", dsl::DslValue::String("paper".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-info-field", Some(&args(vec![("object", dsl::DslValue::String("creationDate".into())), ("text", dsl::DslValue::String("D:20200102".into()))]))).unwrap());
    assert_eq!(snapshot.info.keywords.as_deref(), Some("paper"));
    assert_eq!(snapshot.info.creation_date.as_ref().expect("date").year, 2020);
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("thumbnail".into())), ("text", dsl::DslValue::String("Im1".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("metadata".into())), ("text", dsl::DslValue::String("<xmp/>".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("group".into())), ("text", dsl::DslValue::String("deviceRgb".into())), ("x", dsl::DslValue::float(1.0)), ("y", dsl::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("structParents".into())), ("x", dsl::DslValue::float(4.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-page-extra", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String("action".into())), ("extra", dsl::DslValue::String("O".into())), ("text", dsl::DslValue::String("Next".into()))]))).unwrap());
    assert_eq!(snapshot.pages[0].thumbnail.as_deref(), Some("Im1"));
    assert_eq!(snapshot.pages[0].metadata.as_deref(), Some("<xmp/>"));
    let group = snapshot.pages[0].group.as_ref().expect("group");
    assert_eq!(group.color_space, Some(PdfColorSpace::DeviceRgb));
    assert!(group.isolated && !group.knockout);
    assert_eq!(snapshot.pages[0].struct_parents, Some(4));
    assert_eq!(snapshot.pages[0].additional_actions[0].key, "O");
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(annotation.id.clone())), ("text", dsl::DslValue::String("color".into())), ("red", dsl::DslValue::float(1.0)), ("green", dsl::DslValue::float(0.0)), ("blue", dsl::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(annotation.id.clone())), ("text", dsl::DslValue::String("name".into())), ("extra", dsl::DslValue::String("Note".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-style", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(annotation.id)), ("text", dsl::DslValue::String("flags".into())), ("x", dsl::DslValue::float(4.0))]))).unwrap());
    assert_eq!(snapshot.pages[0].annotations[0].color, vec![1.0, 0.0, 0.0]);
    assert_eq!(snapshot.pages[0].annotations[0].name.as_deref(), Some("Note"));
    assert_eq!(snapshot.pages[0].annotations[0].flags, 4);
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", dsl::DslValue::String("direction".into())), ("text", dsl::DslValue::String("R2L".into())), ("x", dsl::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", dsl::DslValue::String("nonFullScreenPageMode".into())), ("text", dsl::DslValue::String("useThumbs".into())), ("x", dsl::DslValue::float(0.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-viewer-preferences", Some(&args(vec![("object", dsl::DslValue::String("printPageRange".into())), ("text", dsl::DslValue::String("1,3".into())), ("x", dsl::DslValue::float(0.0))]))).unwrap());
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
    let page = dsl::DslValue::float(0.0);
    let object = dsl::DslValue::String(annotation);
    apply(&mut snapshot, edit_from_action("set-annotation-border", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("D".into())), ("extra", dsl::DslValue::String("3,1".into())), ("x", dsl::DslValue::float(2.0)), ("y", dsl::DslValue::float(1.0)), ("width", dsl::DslValue::float(2.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("title".into())), ("extra", dsl::DslValue::String("Reviewer".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("opacity".into())), ("x", dsl::DslValue::float(0.4))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("layer".into())), ("extra", dsl::DslValue::String("OC1".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-markup", Some(&args(vec![("page", page), ("object", object), ("text", dsl::DslValue::String("entry".into())), ("extra", dsl::DslValue::String("OC=On".into()))]))).unwrap());
    let note = &snapshot.pages[0].annotations[0];
    let border = note.border.as_ref().expect("border");
    assert!((border.width - 2.0).abs() < 0.01 && border.style.as_deref() == Some("D") && border.dash.as_deref() == Some(&[3.0, 1.0][..]) && border.radii == Some([1.0, 2.0]));
    let markup = note.markup.as_ref().expect("markup");
    assert_eq!(markup.title.as_deref(), Some("Reviewer"));
    assert_eq!(markup.opacity, Some(0.4));
    assert_eq!(note.optional_content.as_deref(), Some("OC1"));
    assert!(matches!(&note.extra[0].value, PdfObject::Name(name) if name == "On"));
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", dsl::DslValue::String("Name".into())), ("text", dsl::DslValue::String("Ada".into())), ("extra", dsl::DslValue::String("text".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", dsl::DslValue::String("signatureFlags".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", dsl::DslValue::String("defaultAppearance".into())), ("text", dsl::DslValue::String("/Helv 0 Tf".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", dsl::DslValue::String("quadding".into())), ("x", dsl::DslValue::float(2.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", dsl::DslValue::String("defaultFonts".into())), ("text", dsl::DslValue::String("F1,F2".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-form-settings", Some(&args(vec![("object", dsl::DslValue::String("Name".into())), ("text", dsl::DslValue::String("appearance".into())), ("extra", dsl::DslValue::String("/Helv 12 Tf".into()))]))).unwrap());
    let form = snapshot.acro_form.as_ref().expect("form");
    assert_eq!(form.signature_flags, 1);
    assert_eq!(form.default_appearance.as_deref(), Some("/Helv 0 Tf"));
    assert_eq!(form.quadding, Some(2));
    assert_eq!(form.default_fonts, vec!["F1".to_string(), "F2".to_string()]);
    assert_eq!(form.fields[0].default_appearance.as_deref(), Some("/Helv 12 Tf"));
    apply(&mut snapshot, edit_from_action("set-extra-entry", Some(&args(vec![("object", dsl::DslValue::String("info".into())), ("extra", dsl::DslValue::String("Custom".into())), ("text", dsl::DslValue::String("Kept".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-extra-entry", Some(&args(vec![("object", dsl::DslValue::String("viewer".into())), ("extra", dsl::DslValue::String("PrintArea".into())), ("text", dsl::DslValue::String("CropBox".into()))]))).unwrap());
    assert!(matches!(&snapshot.info.extra[0].value, PdfObject::Name(name) if name == "Kept"));
    assert!(matches!(&snapshot.viewer_preferences.as_ref().expect("viewer").extra[0].value, PdfObject::Name(name) if name == "CropBox"));
}

#[test]
fn annotation_kinds_and_choice_options_publish() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfActionKind, PdfAnnotation, PdfAnnotationKind, PdfFormFieldKind};
    let mut snapshot = sample();
    snapshot.pages[0].annotations.push(PdfAnnotation::new([0.0, 0.0, 10.0, 10.0], PdfAnnotationKind::Text { open: false, icon: None, state: None, state_model: None }));
    let annotation = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Annotation).expect("annotation").id;
    let page = dsl::DslValue::float(0.0);
    let object = dsl::DslValue::String(annotation);
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("kind".into())), ("extra", dsl::DslValue::String("link".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("uri".into())), ("extra", dsl::DslValue::String("https://example.test".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("quads".into())), ("extra", dsl::DslValue::String("0,0,10,0,10,10,0,10".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Link { action, quad_points, .. } => {
            let PdfActionKind::Uri { uri, .. } = &action.as_ref().expect("uri").kind else { panic!("uri action") };
            assert_eq!(uri, "https://example.test");
            assert_eq!(quad_points, &vec![0.0, 0.0, 10.0, 0.0, 10.0, 10.0, 0.0, 10.0]);
        }
        _ => panic!("link"),
    }
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("kind".into())), ("extra", dsl::DslValue::String("ink".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("paths".into())), ("extra", dsl::DslValue::String("0,0,8,0;8,0,8,8".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Ink { paths } => assert_eq!(paths, &vec![vec![0.0, 0.0, 8.0, 0.0], vec![8.0, 0.0, 8.0, 8.0]]),
        _ => panic!("ink"),
    }
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page.clone()), ("object", object.clone()), ("text", dsl::DslValue::String("kind".into())), ("extra", dsl::DslValue::String("line".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-annotation-kind", Some(&args(vec![("page", page), ("object", object), ("text", dsl::DslValue::String("points".into())), ("extra", dsl::DslValue::String("1,2,3,4".into()))]))).unwrap());
    match &snapshot.pages[0].annotations[0].kind {
        PdfAnnotationKind::Line { points, .. } => assert_eq!(points, &[1.0, 2.0, 3.0, 4.0]),
        _ => panic!("line"),
    }
    apply(&mut snapshot, edit_from_action("set-form-field", Some(&args(vec![("object", dsl::DslValue::String("City".into())), ("text", dsl::DslValue::String(String::new())), ("extra", dsl::DslValue::String("choice".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", dsl::DslValue::String("City".into())), ("text", dsl::DslValue::String("options".into())), ("extra", dsl::DslValue::String("Bern=Bern,Zug=Zug".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", dsl::DslValue::String("City".into())), ("text", dsl::DslValue::String("values".into())), ("extra", dsl::DslValue::String("Bern".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-field-data", Some(&args(vec![("object", dsl::DslValue::String("City".into())), ("text", dsl::DslValue::String("top".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    let PdfFormFieldKind::Choice { options, values, top_index, .. } = &snapshot.acro_form.as_ref().expect("form").fields[0].kind else { panic!("choice") };
    assert_eq!(options, &vec![("Bern".to_string(), "Bern".to_string()), ("Zug".to_string(), "Zug".to_string())]);
    assert_eq!(values, &vec!["Bern".to_string()]);
    assert_eq!(top_index, &Some(1));
}

#[test]
fn resource_details_publish_onto_the_document() {
    let mut snapshot = sample();
    apply(&mut snapshot, edit_from_action("set-outline", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("x", dsl::DslValue::float(-1.0)), ("text", dsl::DslValue::String("Chapter".into()))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("0".into())), ("text", dsl::DslValue::String("outline.bold".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("0".into())), ("text", dsl::DslValue::String("outline.color".into())), ("red", dsl::DslValue::float(1.0)), ("green", dsl::DslValue::float(0.0)), ("blue", dsl::DslValue::float(0.0))]))).unwrap());
    assert!(snapshot.outlines[0].bold);
    assert_eq!(snapshot.outlines[0].color, Some([1.0, 0.0, 0.0]));
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("Im1".into())), ("text", dsl::DslValue::String("image.interpolate".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("Im1".into())), ("text", dsl::DslValue::String("image.decode".into())), ("extra", dsl::DslValue::String("0,1".into()))]))).unwrap());
    assert!(snapshot.images[0].interpolate);
    assert_eq!(snapshot.images[0].decode, vec![0.0, 1.0]);
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("GS1".into())), ("text", dsl::DslValue::String("graphics.cap".into())), ("x", dsl::DslValue::float(1.0))]))).unwrap());
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("GS1".into())), ("text", dsl::DslValue::String("graphics.dash".into())), ("extra", dsl::DslValue::String("3,1".into())), ("x", dsl::DslValue::float(0.0))]))).unwrap());
    assert!(matches!(snapshot.ext_g_states[0].line_cap, Some(crate::standards::v1_7::subsets::base::schema::snapshot::PdfLineCap::Round)));
    assert_eq!(snapshot.ext_g_states[0].dash.as_ref().map(|(dash, phase)| (dash.clone(), *phase)), Some((vec![3.0, 1.0], 0.0)));
    apply(&mut snapshot, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String("F1".into())), ("text", dsl::DslValue::String("font.italicAngle".into())), ("x", dsl::DslValue::float(-12.0))]))).unwrap());
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
    let snapshot = sample();
    let text = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Text).expect("text");
    let vector = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Vector).expect("vector");
    let mut edited = snapshot.clone();
    apply(&mut edited, edit_from_action("set-text", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(text.id.clone())), ("field", dsl::DslValue::String("text".into())), ("value", dsl::DslValue::String("World".into()))]))).unwrap());
    assert_eq!(edited.pages[0].text(), "World");
    apply(&mut edited, edit_from_action("move", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(vector.id.clone())), ("field", dsl::DslValue::String("x".into())), ("y", dsl::DslValue::float(20.0)), ("value", dsl::DslValue::float(44.0))]))).unwrap());
    let moved = objects(&edited).into_iter().find(|object| object.id == vector.id).expect("vector");
    assert!((moved.x - 44.0).abs() < 0.01);
    let node = render_inspector(&snapshot, Some(&text.id), "s.stdio.pdf@1.7/*#editor", semio_framework_plugin::Locale::En).expect("inspector");
    let mut names = Vec::new();
    let mut shown = Vec::new();
    collect_inspector(&node, &mut names, &mut shown);
    assert!(names.iter().any(|name| name == "set-text"));
    assert!(names.iter().any(|name| name == "set-font"));
    assert!(names.iter().any(|name| name == "delete"));
    assert!(shown.iter().any(|value| value == "Hello"));
    assert!(shown.iter().any(|value| value == "Helvetica"));
    apply(&mut edited, edit_from_action("set-font", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("object", dsl::DslValue::String(text.id.clone())), ("field", dsl::DslValue::String("text".into())), ("value", dsl::DslValue::String("Times-Roman".into()))]))).unwrap());
    assert!(edited.fonts.iter().any(|font| font.base_font() == "Times-Roman"));
    let image = objects(&snapshot).into_iter().find(|object| object.kind == ObjectKind::Image).expect("image");
    let image_panel = render_inspector(&snapshot, Some(&image.id), "s.stdio.pdf@1.7/*#editor", semio_framework_plugin::Locale::En).expect("image inspector");
    let mut image_names = Vec::new();
    let mut image_shown = Vec::new();
    collect_inspector(&image_panel, &mut image_names, &mut image_shown);
    assert!(image_names.iter().any(|name| name == "set-resource-detail"));
    assert!(image_shown.iter().any(|value| value == "0"));
    apply(&mut edited, edit_from_action("set-resource-detail", Some(&args(vec![("object", dsl::DslValue::String(image.text.clone())), ("text", dsl::DslValue::String("image.interpolate".into())), ("field", dsl::DslValue::String("x".into())), ("value", dsl::DslValue::float(1.0))]))).unwrap());
    assert!(edited.images[0].interpolate);
    let empty = render_inspector(&snapshot, None, "s.stdio.pdf@1.7/*#editor", semio_framework_plugin::Locale::De).expect("empty inspector");
    let mut empty_names = Vec::new();
    let mut empty_shown = Vec::new();
    collect_inspector(&empty, &mut empty_names, &mut empty_shown);
    assert!(empty_names.iter().any(|name| name == "set-page-size"));
    assert!(empty_names.iter().any(|name| name == "set-info"));
    assert!(empty_shown.iter().any(|value| value == "200"));
    apply(&mut edited, edit_from_action("set-page-size", Some(&args(vec![("page", dsl::DslValue::float(0.0)), ("field", dsl::DslValue::String("width".into())), ("height", dsl::DslValue::float(300.0)), ("value", dsl::DslValue::float(180.0))]))).unwrap());
    assert!((edited.pages[0].width() - 180.0).abs() < 0.01);
    apply(&mut edited, edit_from_action("set-info", Some(&args(vec![("field", dsl::DslValue::String("text".into())), ("value", dsl::DslValue::String("Title".into()))]))).unwrap());
    assert_eq!(edited.info.title.as_deref(), Some("Title"));
}

fn collect_inspector(node: &semio_framework_ui_contract::BuiltNode, names: &mut Vec<String>, shown: &mut Vec<String>) {
    for binding in node.bindings.iter() {
        names.push(binding.action.name.as_str().to_string());
    }
    if let semio_framework_ui_contract::Component::Input(input) = &node.component {
        shown.push(input.value.as_str().to_string());
    }
    for child in node.children.iter() {
        collect_inspector(child, names, shown);
    }
}
