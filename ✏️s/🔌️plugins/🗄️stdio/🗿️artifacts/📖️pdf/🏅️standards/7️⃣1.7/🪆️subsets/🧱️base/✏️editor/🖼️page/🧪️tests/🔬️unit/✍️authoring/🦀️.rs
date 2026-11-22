//! 🧪 PDF page editor authoring: new annotations, form fill, page replace, merge and extract against a language-agnostic fixture and the independent lopdf reader.

use crate::editor::page::*;
use crate::standards::v1_7::subsets::base::io::export::encode_pdf;
use crate::standards::v1_7::subsets::base::io::mutation_bridge::apply_pdf_mutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfAppearanceEntry, PdfDestination, PdfFont, PdfFormField, PdfFormFieldKind, PdfFormXObject, PdfImage, PdfOp, PdfPage, PdfSnapshot, PdfTextString};
use semio_framework_io_base64::base64_standard_encode;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
use serde_json::{json, Value};

fn fixture() -> Value {
    serde_json::from_str(include_str!("🧪️authoring.json")).unwrap()
}

fn args(entries: Vec<(&str, DslValue)>) -> DslValue {
    DslValue::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

fn text(value: &str) -> DslValue {
    DslValue::String(value.into())
}

fn number(value: f64) -> DslValue {
    DslValue::float(value)
}

fn run(snapshot: &mut PdfSnapshot, action: &str, entries: Vec<(&str, DslValue)>) -> Result<(), String> {
    let edit = edit_from_action(action, Some(&args(entries))).map_err(|fault| format!("{fault:?}"))?;
    for mutation in apply_payload(snapshot, &edit.action, &edit.payload).map_err(|fault| format!("{fault:?}"))? {
        apply_pdf_mutation(snapshot, &mutation);
    }
    Ok(())
}

fn widget(field: &str, rect: [f64; 4]) -> PdfAnnotation {
    PdfAnnotation::new(rect, PdfAnnotationKind::Widget { field: Some(field.into()), highlight: None, characteristics: Vec::new(), action: None, additional_actions: Vec::new() })
}

fn field(name: &str, kind: PdfFormFieldKind, flags: u32, widgets: Vec<[u32; 2]>) -> PdfFormField {
    PdfFormField { name: name.into(), kind, flags, alternate_name: None, mapping_name: None, default_appearance: None, quadding: None, widgets, children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() }
}

fn form_document() -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    let mut page = PdfPage::new(200.0, 300.0);
    page.annotations = vec![widget("name", [10.0, 250.0, 110.0, 270.0]), widget("agree", [10.0, 220.0, 22.0, 232.0]), widget("size", [10.0, 190.0, 22.0, 202.0]), widget("size", [30.0, 190.0, 42.0, 202.0]), widget("color", [10.0, 150.0, 110.0, 170.0])];
    snapshot.pages.push(page);
    snapshot.fonts.push(PdfFont::standard("Helv", "Helvetica"));
    let text_kind = |max| PdfFormFieldKind::Text { value: None, default_value: None, max_length: max, rich_value: None };
    let button = || PdfFormFieldKind::Button { value: None, default_value: None, options: Vec::new() };
    let mut name = field("name", text_kind(Some(10)), 0, vec![[0, 0]]);
    name.default_appearance = Some("/Helv 12 Tf 0 g".into());
    let mut color = field("color", PdfFormFieldKind::Choice { values: Vec::new(), default_values: Vec::new(), options: vec![("r".into(), "Red".into()), ("g".into(), "Green".into())], top_index: None }, 1 << 17, vec![[0, 4]]);
    color.default_appearance = Some("/Helv 0 Tf 0 g".into());
    snapshot.acro_form = Some(PdfAcroForm {
        fields: vec![
            name,
            field("locked", text_kind(None), 1, Vec::new()),
            field("agree", button(), 0, vec![[0, 1]]),
            field("size", PdfFormFieldKind::Button { value: None, default_value: None, options: vec!["S".into(), "L".into()] }, 1 << 15, vec![[0, 2], [0, 3]]),
            color,
            field("send", button(), 1 << 16, Vec::new()),
        ],
        ..Default::default()
    });
    snapshot
}

fn stored(snapshot: &PdfSnapshot, name: &str) -> String {
    let field = snapshot.acro_form.as_ref().unwrap().fields.iter().find(|field| field.name == name).unwrap();
    match &field.kind {
        PdfFormFieldKind::Text { value, .. } | PdfFormFieldKind::Button { value, .. } => value.clone().unwrap_or_default(),
        PdfFormFieldKind::Choice { values, .. } => values.join(","),
        _ => String::new(),
    }
}

fn widget_states(snapshot: &PdfSnapshot, name: &str) -> Vec<String> {
    let field = snapshot.acro_form.as_ref().unwrap().fields.iter().find(|field| field.name == name).unwrap();
    field.widgets.iter().map(|[page, annotation]| snapshot.pages[*page as usize].annotations[*annotation as usize].appearance_state.clone().unwrap_or_default()).collect()
}

fn shown_texts(snapshot: &PdfSnapshot, name: &str) -> Vec<String> {
    let field = snapshot.acro_form.as_ref().unwrap().fields.iter().find(|field| field.name == name).unwrap();
    field
        .widgets
        .iter()
        .filter_map(|[page, annotation]| match &snapshot.pages[*page as usize].annotations[*annotation as usize].appearance.as_ref()?.normal {
            PdfAppearanceEntry::Single { form } => snapshot.forms.iter().find(|candidate| &candidate.id == form),
            PdfAppearanceEntry::States { .. } => None,
        })
        .flat_map(|form| form.content.iter().filter_map(|op| if let PdfOp::ShowText { text: PdfTextString::Text { text } } = op { Some(text.clone()) } else { None }))
        .collect()
}

#[test]
fn page_selections_follow_the_fixture() {
    for case in fixture()["pageSelections"].as_array().unwrap() {
        let parsed = parse_page_selection(case["selection"].as_str().unwrap(), case["count"].as_u64().unwrap() as usize);
        match case.get("pages") {
            Some(pages) => assert_eq!(serde_json::to_value(parsed.unwrap()).unwrap(), *pages, "{case}"),
            None => assert!(parsed.is_err(), "{case}"),
        }
    }
}

#[test]
fn created_annotations_follow_the_fixture_and_round_trip_through_lopdf() {
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages.push(PdfPage::new(200.0, 300.0));
    snapshot.pages.push(PdfPage::new(200.0, 300.0));
    let mut subtypes = Vec::new();
    for case in fixture()["annotations"].as_array().unwrap() {
        let rect = case["rect"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        let color = case["color"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap()).collect::<Vec<_>>();
        let before = snapshot.pages[0].annotations.len();
        let outcome = run(
            &mut snapshot,
            "create-annotation",
            vec![("page", number(0.0)), ("object", text(case["kind"].as_str().unwrap())), ("text", text(case["text"].as_str().unwrap_or(""))), ("extra", text(case["extra"].as_str().unwrap_or(""))), ("x", number(rect[0])), ("y", number(rect[1])), ("width", number(rect[2])), ("height", number(rect[3])), ("red", number(color[0])), ("green", number(color[1])), ("blue", number(color[2]))],
        );
        if case.get("error").is_some() {
            assert!(outcome.is_err(), "{case}");
            assert_eq!(snapshot.pages[0].annotations.len(), before);
            continue;
        }
        outcome.unwrap();
        let annotation = snapshot.pages[0].annotations.last().unwrap();
        assert_eq!(serde_json::to_value(annotation.rect).unwrap(), case["annotRect"], "{case}");
        if let Some(quads) = case.get("quads") {
            let (PdfAnnotationKind::Highlight { quad_points } | PdfAnnotationKind::Underline { quad_points } | PdfAnnotationKind::StrikeOut { quad_points }) = &annotation.kind else { panic!("{case}") };
            assert_eq!(serde_json::to_value(quad_points).unwrap(), *quads);
        }
        if let Some(paths) = case.get("paths") {
            let PdfAnnotationKind::Ink { paths: stored } = &annotation.kind else { panic!("{case}") };
            assert_eq!(serde_json::to_value(stored).unwrap(), *paths);
        }
        if let Some(color) = case.get("colorOut") {
            assert_eq!(serde_json::to_value(&annotation.color).unwrap(), *color, "{case}");
        }
        if let Some(flags) = case.get("flags") {
            assert_eq!(json!(annotation.flags), *flags);
        }
        if let Some(page) = case.get("destinationPage") {
            let PdfAnnotationKind::Link { destination: Some(PdfDestination::Page { page: target, .. }), .. } = &annotation.kind else { panic!("{case}") };
            assert_eq!(json!(target), *page);
        }
        subtypes.push(case["subtype"].as_str().unwrap().to_string());
    }
    let bytes = encode_pdf(&snapshot).unwrap();
    let drawn = oracle::reference_widget_appearances(&bytes).unwrap();
    assert_eq!(drawn.len(), 5, "note, highlight, underline, strike-out and ink each carry a generated appearance: {drawn:?}");
    assert!(drawn.iter().any(|(_, content)| content.contains("gs") && content.contains(" f")), "{drawn:?}");
    assert!(snapshot.ext_g_states.iter().any(|state| state.blend_mode.as_deref() == Some(&["Multiply".to_string()][..])));
    assert_eq!(oracle::reference_annotation_subtypes(&bytes).unwrap()[0], subtypes);
    assert!(oracle::reference_annotation_subtypes(&bytes).unwrap()[1].is_empty());
}

#[test]
fn form_fills_follow_the_fixture_and_regenerate_appearances() {
    for case in fixture()["fills"].as_array().unwrap() {
        let mut snapshot = form_document();
        let field = case["field"].as_str().unwrap();
        let outcome = run(&mut snapshot, "fill-form-field", vec![("object", text(field)), ("text", text(case["value"].as_str().unwrap()))]);
        if case.get("error").is_some() {
            assert!(outcome.is_err(), "{case}");
            continue;
        }
        outcome.unwrap();
        assert_eq!(stored(&snapshot, field), case["stored"].as_str().unwrap(), "{case}");
        if let Some(state) = case.get("state") {
            assert!(widget_states(&snapshot, field).contains(&state.as_str().unwrap().to_string()), "{case}");
        }
        if let Some(appearance) = case.get("appearance") {
            let expected = appearance.as_str().unwrap().trim_start_matches('(').trim_end_matches(") Tj");
            assert_eq!(shown_texts(&snapshot, field), vec![expected.to_string()], "{case}");
            let bytes = encode_pdf(&snapshot).unwrap();
            assert!(oracle::reference_widget_appearances(&bytes).unwrap().iter().any(|(_, content)| content.contains(appearance.as_str().unwrap())), "{case}");
            let values = oracle::reference_field_values(&bytes).unwrap();
            assert!(values.contains(&(field.to_string(), case["stored"].as_str().unwrap().to_string())), "{values:?}");
        }
    }
}

#[test]
fn checkbox_and_radio_gain_state_appearances_that_lopdf_reads() {
    let mut snapshot = form_document();
    run(&mut snapshot, "fill-form-field", vec![("object", text("agree")), ("text", text("true"))]).unwrap();
    run(&mut snapshot, "fill-form-field", vec![("object", text("size")), ("text", text("L"))]).unwrap();
    assert_eq!(widget_states(&snapshot, "size"), vec!["Off", "L"]);
    let bytes = encode_pdf(&snapshot).unwrap();
    let appearances = oracle::reference_widget_appearances(&bytes).unwrap();
    assert!(appearances.iter().any(|(state, content)| state.starts_with("Yes=Yes") && content.contains(" l")), "{appearances:?}");
    assert!(appearances.iter().any(|(state, _)| state.starts_with("L=L")), "{appearances:?}");
    assert!(appearances.iter().any(|(state, content)| state.starts_with("Off=Off") && content.trim().is_empty()));
    let values = oracle::reference_field_values(&bytes).unwrap();
    assert!(values.contains(&("agree".into(), "Yes".into())) && values.contains(&("size".into(), "L".into())), "{values:?}");
}

fn source_document() -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    let mut first = PdfPage::new(100.0, 100.0);
    first.content = vec![PdfOp::BeginText, PdfOp::SetFont { name: "F1".into(), size: 10.0 }, PdfOp::SetTextMatrix { matrix: [1.0, 0.0, 0.0, 1.0, 5.0, 50.0] }, PdfOp::ShowText { text: PdfTextString::text("Source") }, PdfOp::EndText, PdfOp::Save, PdfOp::Transform { matrix: [20.0, 0.0, 0.0, 20.0, 10.0, 10.0] }, PdfOp::PaintXObject { name: "Im1".into() }, PdfOp::Restore, PdfOp::PaintXObject { name: "Fm1".into() }];
    first.annotations = vec![PdfAnnotation::link([1.0, 1.0, 20.0, 20.0], "https://example.org/")];
    let mut second = PdfPage::new(120.0, 80.0);
    second.annotations = vec![PdfAnnotation::new([1.0, 1.0, 20.0, 20.0], PdfAnnotationKind::Link { action: None, destination: Some(PdfDestination::Page { page: 0, fit: crate::standards::v1_7::subsets::base::schema::snapshot::PdfDestinationFit::Fit }), highlight: None, quad_points: Vec::new() })];
    snapshot.pages = vec![first, second, PdfPage::new(90.0, 90.0)];
    snapshot.fonts.push(PdfFont::standard("F1", "Helvetica"));
    snapshot.fonts.push(PdfFont::standard("Unused", "Courier"));
    snapshot.images.push(PdfImage::rgb8("Im1", 2, 2, vec![200; 12]));
    snapshot.forms.push(PdfFormXObject::new("Fm1", [0.0, 0.0, 10.0, 10.0], vec![PdfOp::Rectangle { x: 0.0, y: 0.0, width: 5.0, height: 5.0 }, PdfOp::Fill]));
    snapshot
}

fn data_url(snapshot: &PdfSnapshot) -> String {
    format!("data:application/pdf;base64,{}", base64_standard_encode(encode_pdf(snapshot).unwrap()))
}

fn destination_document() -> PdfSnapshot {
    let mut snapshot = PdfSnapshot::default();
    snapshot.pages = vec![PdfPage::new(50.0, 50.0), PdfPage::new(60.0, 60.0)];
    snapshot.fonts.push(PdfFont::standard("F1", "Times-Roman"));
    snapshot
}

#[test]
fn merging_inserts_picked_pages_with_renamed_used_resources_only() {
    let source = source_document();
    let mut snapshot = destination_document();
    run(&mut snapshot, "merge-document", vec![("page", number(1.0)), ("payload", text(&data_url(&source))), ("extra", text("1-2"))]).unwrap();
    assert_eq!(snapshot.pages.iter().map(|page| (page.width(), page.height())).collect::<Vec<_>>(), vec![(50.0, 50.0), (100.0, 100.0), (120.0, 80.0), (60.0, 60.0)]);
    assert_eq!(snapshot.fonts.iter().map(|font| font.id.as_str()).collect::<Vec<_>>(), vec!["F1", "m1_F1"]);
    assert_eq!(snapshot.images.iter().map(|image| image.id.as_str()).collect::<Vec<_>>(), vec!["m1_Im1"]);
    assert_eq!(snapshot.forms.iter().map(|form| form.id.as_str()).collect::<Vec<_>>(), vec!["m1_Fm1"]);
    let names: Vec<String> = snapshot.pages[1].content.iter().filter_map(|op| match op { PdfOp::SetFont { name, .. } | PdfOp::PaintXObject { name } => Some(name.clone()), _ => None }).collect();
    assert_eq!(names, vec!["m1_F1", "m1_Im1", "m1_Fm1"]);
    let PdfAnnotationKind::Link { destination: Some(PdfDestination::Page { page, .. }), .. } = &snapshot.pages[2].annotations[0].kind else { panic!() };
    assert_eq!(*page, 1);
    let bytes = encode_pdf(&snapshot).unwrap();
    assert_eq!(oracle::reference_page_count(&bytes).unwrap(), 4);
    assert_eq!(oracle::reference_page_operator_names(&bytes).unwrap()[1].first().map(String::as_str), Some("BT"));
    run(&mut snapshot, "merge-document", vec![("page", number(0.0)), ("payload", text(&data_url(&source))), ("extra", text("1"))]).unwrap();
    assert!(snapshot.fonts.iter().any(|font| font.id == "m2_F1") && snapshot.pages.len() == 5);
}

#[test]
fn merging_refuses_garbage_and_missing_pages() {
    let mut snapshot = destination_document();
    assert!(run(&mut snapshot, "merge-document", vec![("page", number(0.0)), ("payload", text("data:application/pdf;base64,AAAA"))]).is_err());
    assert!(run(&mut snapshot, "merge-document", vec![("page", number(0.0)), ("payload", text("***"))]).is_err());
    assert!(run(&mut snapshot, "merge-document", vec![("page", number(0.0)), ("payload", text(&data_url(&source_document()))), ("extra", text("9"))]).is_err());
    assert_eq!(snapshot.pages.len(), 2);
}

#[test]
fn merging_rewires_form_widgets_and_keeps_names_unique() {
    let mut source = form_document();
    run(&mut source, "fill-form-field", vec![("object", text("name")), ("text", text("Ada"))]).unwrap();
    let mut snapshot = form_document();
    snapshot.pages.insert(0, PdfPage::new(200.0, 300.0));
    snapshot.acro_form.as_mut().unwrap().fields.iter_mut().for_each(|field| field.widgets.iter_mut().for_each(|widget| widget[0] += 1));
    run(&mut snapshot, "merge-document", vec![("page", number(0.0)), ("payload", text(&data_url(&source)))]).unwrap();
    let form = snapshot.acro_form.as_ref().unwrap();
    let merged = form.fields.iter().find(|field| field.name == "name_2").expect("renamed merged field");
    assert_eq!(merged.widgets, vec![[0, 0]]);
    assert!(matches!(&snapshot.pages[0].annotations[0].kind, PdfAnnotationKind::Widget { field: Some(name), .. } if name == "name_2"));
    assert_eq!(form.fields.iter().find(|field| field.name == "name").unwrap().widgets, vec![[2, 0]]);
    assert_eq!(snapshot.pages.len(), 3);
}

#[test]
fn replacing_a_page_keeps_the_others_and_drops_stale_widgets() {
    let mut snapshot = form_document();
    snapshot.pages.push(PdfPage::new(10.0, 10.0));
    run(&mut snapshot, "replace-page", vec![("page", number(1.0)), ("width", number(30.0)), ("height", number(40.0))]).unwrap();
    assert_eq!((snapshot.pages[1].width(), snapshot.pages[1].height()), (30.0, 40.0));
    run(&mut snapshot, "replace-page", vec![("page", number(0.0)), ("payload", text(&data_url(&source_document()))), ("extra", text("2"))]).unwrap();
    assert_eq!((snapshot.pages[0].width(), snapshot.pages[0].height()), (120.0, 80.0));
    assert!(snapshot.acro_form.as_ref().unwrap().fields.iter().all(|field| field.widgets.is_empty()));
    assert_eq!(snapshot.pages.len(), 2);
    assert!(run(&mut snapshot, "replace-page", vec![("page", number(5.0))]).is_err());
    assert!(run(&mut snapshot, "replace-page", vec![("page", number(0.0)), ("payload", text(&data_url(&source_document()))), ("extra", text("1-2"))]).is_err());
}

#[test]
fn extraction_writes_standalone_pdfs_that_lopdf_counts() {
    let source = source_document();
    let bytes = extract_pdf(&source, &[2, 0]).unwrap();
    assert_eq!(oracle::reference_page_count(&bytes).unwrap(), 2);
    let reread = crate::standards::v1_7::subsets::base::io::import::decode_pdf(&bytes).unwrap();
    assert_eq!(reread.pages.iter().map(|page| (page.width(), page.height())).collect::<Vec<_>>(), vec![(90.0, 90.0), (100.0, 100.0)]);
    assert_eq!(reread.fonts.iter().map(|font| font.id.as_str()).collect::<Vec<_>>(), vec!["F1"]);
    assert_eq!(reread.images.len(), 1);
    assert!(extract_pdf(&source, &[7]).is_err());
}

#[test]
fn extract_and_pick_actions_emit_download_and_picker_effects() {
    let source = source_document();
    let edit = edit_from_action("extract-pages", Some(&args(vec![("extra", text("1;2-3"))]))).unwrap();
    let emitted = emit_page_edit(&source, &edit.action, &edit.payload).unwrap();
    assert!(emitted.artifact_mutations.is_empty());
    let filenames: Vec<&str> = emitted.effects.iter().map(|effect| match effect { semio_plugin::Effect::DownloadMediaExport { filename, mime_type, .. } => { assert_eq!(mime_type, "application/pdf"); filename.as_str() } other => panic!("{other:?}") }).collect();
    assert_eq!(filenames, vec!["pages-1-1.pdf", "pages-2-2_3.pdf"]);
    for (pick, import) in [("merge-pick", "merge-document"), ("replace-page-pick", "replace-page")] {
        let edit = edit_from_action(pick, Some(&args(vec![("page", number(2.0)), ("extra", text("1-2"))]))).unwrap();
        let emitted = emit_page_edit(&source, &edit.action, &edit.payload).unwrap();
        let [semio_plugin::Effect::RequestFileOpen { import_action, accept, max_file_bytes, args, multiple, .. }] = emitted.effects.as_slice() else { panic!() };
        assert_eq!((import_action.as_str(), accept.contains("pdf"), *max_file_bytes, *multiple), (import, true, Some(MAX_SOURCE_BYTES), false));
        assert_eq!(Value::from(args.clone().unwrap()), json!({ "page": 2, "extra": "1-2" }));
    }
    assert!(emit_page_edit(&source, "extract-pages", &json!({ "extra": "9" }).to_string()).is_err());
}

#[test]
fn authoring_actions_are_declared_with_english_and_german_labels() {
    let definition = window_definition();
    for id in ["create-annotation", "fill-form-field", "replace-page", "replace-page-pick", "merge-document", "merge-pick", "extract-pages"] {
        assert!(is_page_action(id), "{id}");
        assert_eq!(static_action(id), id);
        assert!(definition.actions.iter().any(|action| action.id == id), "{id}");
    }
}


fn structure_document() -> PdfSnapshot {
    let mut snapshot = form_document();
    let mut linked = PdfPage::new(10.0, 10.0);
    linked.annotations = vec![PdfAnnotation::new([1.0, 1.0, 5.0, 5.0], PdfAnnotationKind::Link { action: None, destination: Some(PdfDestination::Page { page: 0, fit: crate::standards::v1_7::subsets::base::schema::snapshot::PdfDestinationFit::Fit }), highlight: None, quad_points: Vec::new() })];
    snapshot.pages.push(linked);
    snapshot.pages.push(PdfPage::new(20.0, 20.0));
    snapshot
}

#[test]
fn page_structure_actions_renumber_widgets_and_links_per_the_fixture() {
    for case in fixture()["pageStructure"].as_array().unwrap() {
        let mut snapshot = structure_document();
        let entries: Vec<(&str, DslValue)> = case["args"].as_object().unwrap().iter().map(|(key, value)| (key.as_str(), number(value.as_f64().unwrap()))).collect();
        run(&mut snapshot, case["action"].as_str().unwrap(), entries).unwrap();
        assert_eq!(snapshot.pages.len() as u64, case["pages"].as_u64().unwrap(), "{case}");
        let widgets = &snapshot.acro_form.as_ref().unwrap().fields.iter().find(|field| field.name == "name").unwrap().widgets;
        match case["widgetPage"].as_u64() {
            Some(page) => assert_eq!(widgets, &vec![[page as u32, 0]], "{case}"),
            None => assert!(widgets.is_empty(), "{case}"),
        }
        let link = |page: &PdfPage| page.annotations.iter().find_map(|annotation| match &annotation.kind { PdfAnnotationKind::Link { destination, .. } => Some(destination.clone()), _ => None });
        match case["linkAt"].as_u64() {
            Some(at) => {
                let target = link(&snapshot.pages[at as usize]).unwrap_or_else(|| panic!("{case}"));
                let expected = case["linkTarget"].as_u64().map(|page| PdfDestination::Page { page: page as u32, fit: crate::standards::v1_7::subsets::base::schema::snapshot::PdfDestinationFit::Fit });
                assert_eq!(target, expected, "{case}");
            }
            None => assert!(snapshot.pages.iter().all(|page| link(page).is_none()), "{case}"),
        }
    }
}

#[test]
fn page_structure_round_trips_through_the_independent_reader() {
    let mut snapshot = structure_document();
    run(&mut snapshot, "insert-page", vec![("page", number(1.0)), ("width", number(30.0)), ("height", number(40.0))]).unwrap();
    run(&mut snapshot, "move-page", vec![("page", number(0.0)), ("x", number(3.0))]).unwrap();
    run(&mut snapshot, "remove-page", vec![("page", number(0.0))]).unwrap();
    let bytes = encode_pdf(&snapshot).unwrap();
    assert_eq!(oracle::reference_page_count(&bytes).unwrap(), snapshot.pages.len());
    assert_eq!(snapshot.pages.len(), 3);
}

fn collect(node: &semio_ui_contract::BuiltNode, names: &mut Vec<String>, shown: &mut Vec<String>) {
    names.extend(node.bindings.iter().map(|binding| binding.action.name.as_str().to_string()));
    if let semio_ui_contract::Component::Input(input) = &node.component {
        shown.push(input.value.as_str().to_string());
    }
    node.children.iter().for_each(|child| collect(child, names, shown));
}

fn panel(snapshot: &PdfSnapshot, selected: Option<&str>, locale: semio_ui_locale::Locale) -> (Vec<String>, Vec<String>) {
    let node = render_inspector(snapshot, selected, "s.stdio.pdf@1.7/*#editor", semio_plugin::UiPublicationRevision(17), locale).expect("inspector");
    let (mut names, mut shown) = (Vec::new(), Vec::new());
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node), |node| collect(node, &mut names, &mut shown));
    (names, shown)
}

#[test]
fn the_page_inspector_offers_every_authoring_control_in_both_languages() {
    let snapshot = structure_document();
    let expected = &fixture_controls()["pageInspectorBindings"];
    for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
        let (names, _) = panel(&snapshot, Some("p0:paper"), locale);
        for (name, count) in expected.as_object().unwrap() {
            assert_eq!(names.iter().filter(|candidate| *candidate == name).count() as u64, count.as_u64().unwrap(), "{name} {locale:?}");
        }
    }
}

#[test]
fn a_selected_widget_shows_its_value_and_fills_through_the_inspector_action() {
    let mut snapshot = form_document();
    run(&mut snapshot, "fill-form-field", vec![("object", text("name")), ("text", text("Ada"))]).unwrap();
    let (names, shown) = panel(&snapshot, Some("p0:a0"), semio_ui_locale::Locale::En);
    assert!(names.iter().any(|name| name == "fill-form-field"), "{names:?}");
    assert!(shown.iter().any(|value| value == "Ada"), "{shown:?}");
    let commit = args(vec![("page", number(0.0)), ("object", text("name")), ("field", text("text")), ("value", text("Bob"))]);
    let edit = edit_from_action("fill-form-field", Some(&commit)).unwrap();
    for mutation in apply_payload(&snapshot, &edit.action, &edit.payload).unwrap() {
        apply_pdf_mutation(&mut snapshot, &mutation);
    }
    assert_eq!(stored(&snapshot, "name"), "Bob");
}

fn fixture_controls() -> Value {
    serde_json::from_str(include_str!("🧪️field-controls.json")).unwrap()
}

fn camel(name: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for character in name.chars() {
        if character == '_' {
            upper = true;
        } else if upper {
            out.extend(character.to_uppercase());
            upper = false;
        } else {
            out.push(character);
        }
    }
    out
}

#[test]
fn every_typed_snapshot_field_has_a_declared_control_action() {
    let source = include_str!("../../../../../🧬️schema/📸️snapshot/🦀️.rs");
    let body = source.split("pub struct PdfSnapshot {").nth(1).unwrap().split("\n}").next().unwrap();
    let fields: Vec<String> = body.lines().filter_map(|line| line.trim().strip_prefix("pub ")?.split(':').next().map(camel)).collect();
    assert!(fields.len() >= 30, "{fields:?}");
    let controls = fixture_controls();
    let (derived, mapped) = (controls["derived"].as_array().unwrap(), controls["controls"].as_object().unwrap());
    let definition = window_definition();
    for field in &fields {
        let covered = derived.iter().any(|name| name == field) || mapped.get(field).is_some_and(|actions| !actions.as_array().unwrap().is_empty());
        assert!(covered, "typed field '{field}' has no control action");
    }
    let named: Vec<&str> = mapped.keys().map(String::as_str).chain(derived.iter().filter_map(Value::as_str)).collect();
    for name in named {
        assert!(fields.iter().any(|field| field == name), "fixture names unknown snapshot field '{name}'");
    }
    for actions in mapped.values() {
        for action in actions.as_array().unwrap() {
            let action = action.as_str().unwrap();
            assert!(is_page_action(action) && definition.actions.iter().any(|declared| declared.id == action), "'{action}' is not a declared window action");
        }
    }
}

#[test]
fn merge_and_extract_stop_when_cancelled_and_report_progress() {
    let source = source_document();
    let destination = destination_document();
    let mut seen = Vec::new();
    let planned = import_pages_controlled(&destination, &source, &[0, 1], ImportTarget::Insert(0), None, &mut |progress| { seen.push(progress.phase); true }).unwrap();
    assert!(!planned.is_empty() && seen.contains(&"scan") && seen.contains(&"resources") && seen.contains(&"pages"), "{seen:?}");
    for allowed in 0..seen.len() {
        let mut calls = 0;
        let outcome = import_pages_controlled(&destination, &source, &[0, 1], ImportTarget::Insert(0), None, &mut |_| { calls += 1; calls <= allowed });
        assert!(outcome.is_err(), "cancelled after {allowed} checkpoints");
    }
    let mut calls = 0;
    assert!(extract_pdf_controlled(&source, &[0], &mut |_| { calls += 1; calls < 3 }).is_err());
    assert!(extract_pdf_controlled(&source, &[0], &mut |_| true).is_ok());
}

#[test]
fn page_edits_renumber_outline_entries_and_named_destinations() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDestinationFit, PdfNamedDestination, PdfOutlineItem};
    let go = |page: u32| PdfDestination::Page { page, fit: PdfDestinationFit::Fit };
    let mut snapshot = structure_document();
    let item = |title: &str, page: u32, children: Vec<PdfOutlineItem>| PdfOutlineItem { title: title.into(), destination: Some(go(page)), action: None, color: None, italic: false, bold: false, open: false, children };
    snapshot.outlines = vec![item("one", 0, vec![item("child", 2, Vec::new())]), item("two", 1, Vec::new())];
    snapshot.named_destinations = vec![PdfNamedDestination { name: "last".into(), destination: go(2) }, PdfNamedDestination { name: "first".into(), destination: go(0) }];
    let mut inserted = snapshot.clone();
    run(&mut inserted, "insert-page", vec![("page", number(0.0)), ("width", number(9.0)), ("height", number(9.0))]).unwrap();
    assert_eq!(inserted.outlines[0].destination, Some(go(1)));
    assert_eq!(inserted.outlines[0].children[0].destination, Some(go(3)));
    assert_eq!(inserted.named_destinations.iter().find(|named| named.name == "last").unwrap().destination, go(3));
    let mut moved = snapshot.clone();
    run(&mut moved, "move-page", vec![("page", number(0.0)), ("x", number(2.0))]).unwrap();
    assert_eq!((moved.outlines[0].destination.clone(), moved.outlines[1].destination.clone()), (Some(go(2)), Some(go(0))));
    let mut removed = snapshot;
    run(&mut removed, "remove-page", vec![("page", number(1.0))]).unwrap();
    assert_eq!(removed.outlines[1].destination, None);
    assert_eq!(removed.outlines[0].children[0].destination, Some(go(1)));
    assert!(removed.named_destinations.iter().any(|named| named.name == "first" && named.destination == go(0)));
}
