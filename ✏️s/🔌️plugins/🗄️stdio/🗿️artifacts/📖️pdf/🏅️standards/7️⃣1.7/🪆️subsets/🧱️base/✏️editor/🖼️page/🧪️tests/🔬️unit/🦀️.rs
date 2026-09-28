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
