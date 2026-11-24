//! 🧪️ PDF 1.4 page canvas against the neutral fixture: MuPDF probe colours, selection, text edit, page structure and the view over the lifted pages.

use super::*;
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_pdf_page_editor::page::{canvas_layers, find_object, objects, page_viewports, reduce_view, search, SearchControl, ViewIntent, ViewState, ZoomMode};

const CURVE_STEPS: usize = 24;

fn fixture() -> Value {
    serde_json::from_str(include_str!("🧪️pdf14-canvas.json")).unwrap()
}

fn snapshot(case: &Value) -> Snapshot14 {
    Snapshot14 { schema: "stdio.pdf".into(), pages: case["pages"].as_array().unwrap().iter().map(|page| PageDoc { width: page["width"].as_f64().unwrap(), height: page["height"].as_f64().unwrap(), text: page["text"].as_str().unwrap().into() }).collect() }
}

fn contours(segments: &Value, transform: [f64; 6]) -> Vec<Vec<(f64, f64)>> {
    let map = |value: &Value| {
        let (x, y) = (value[0].as_f64().unwrap(), value[1].as_f64().unwrap());
        (transform[0] * x + transform[2] * y + transform[4], transform[1] * x + transform[3] * y + transform[5])
    };
    let mut found: Vec<Vec<(f64, f64)>> = Vec::new();
    for segment in segments.as_array().into_iter().flatten() {
        match segment["kind"].as_str().unwrap() {
            "move" => found.push(vec![map(&segment["to"])]),
            "line" => found.last_mut().unwrap().push(map(&segment["to"])),
            "cubic" => {
                let contour = found.last_mut().unwrap();
                let start = *contour.last().unwrap();
                let (a, b, c) = (map(&segment["ctrl1"]), map(&segment["ctrl2"]), map(&segment["to"]));
                for step in 1..=CURVE_STEPS {
                    let t = step as f64 / CURVE_STEPS as f64;
                    let u = 1.0 - t;
                    let blend = |p: f64, q: f64, r: f64, s: f64| u * u * u * p + 3.0 * u * u * t * q + 3.0 * u * t * t * r + t * t * t * s;
                    contour.push((blend(start.0, a.0, b.0, c.0), blend(start.1, a.1, b.1, c.1)));
                }
            }
            _ => {}
        }
    }
    found
}

fn covered(contours: &[Vec<(f64, f64)>], (x, y): (f64, f64)) -> bool {
    let mut inside = false;
    for contour in contours {
        for (index, &(x1, y1)) in contour.iter().enumerate() {
            let (x2, y2) = contour[(index + 1) % contour.len()];
            if (y1 > y) != (y2 > y) && x < x1 + (y - y1) * (x2 - x1) / (y2 - y1) { inside = !inside; }
        }
    }
    inside
}

fn inverse(matrix: [f64; 6]) -> [f64; 6] {
    let determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2];
    let (a, b, c, d) = (matrix[3] / determinant, -matrix[1] / determinant, -matrix[2] / determinant, matrix[0] / determinant);
    [a, b, c, d, -(a * matrix[4] + c * matrix[5]), -(b * matrix[4] + d * matrix[5])]
}

fn composite(scene: &[Value], point: (f64, f64)) -> [f64; 3] {
    let mut rgb = [1.0; 3];
    for layer in scene {
        let Some(fill) = layer.get("fill").filter(|fill| fill.is_object() && fill["color"].is_array()) else { continue };
        let transform: [f64; 6] = layer.get("transform").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([1.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        let back = inverse(transform);
        let local = (back[0] * point.0 + back[2] * point.1 + back[4], back[1] * point.0 + back[3] * point.1 + back[5]);
        if covered(&contours(&layer["segments"], [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]), local) {
            let alpha = fill["color"][3].as_f64().unwrap_or(1.0) * layer["opacity"].as_f64().unwrap_or(1.0);
            for channel in 0..3 { rgb[channel] = rgb[channel] * (1.0 - alpha) + fill["color"][channel].as_f64().unwrap() * alpha; }
        }
    }
    rgb
}

fn canvas_point(lifted: &Snapshot17, probe: &Value) -> (f64, f64) {
    let top = page_viewports(lifted).nth(probe["page"].as_u64().unwrap() as usize).unwrap().clip_rect()[1];
    (probe["at"][0].as_f64().unwrap(), top + probe["at"][1].as_f64().unwrap())
}

fn args(entries: &[(&str, f64)]) -> DslValue {
    DslValue::Object(entries.iter().map(|(key, value)| ((*key).into(), DslValue::float(*value))).collect())
}

fn payload(action: &str, entries: &[(&str, f64)]) -> String {
    edit_from_action(action, Some(&args(entries))).unwrap()
}

fn ink(case: &Value) -> Value {
    case["probes"].as_array().unwrap().iter().find(|probe| probe["rgb"] == serde_json::json!([0, 0, 0])).unwrap().clone()
}

#[test]
fn canvas_paint_of_1_4_pages_matches_mupdf_probes() {
    let fixture = fixture();
    let tolerance = fixture["tolerance"].as_f64().unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let lifted = lift(&snapshot(case)).unwrap_or_else(|error| panic!("{name}: {}", error.message));
        let scene: Vec<Value> = serde_json::from_str(&canvas_layers(&lifted, None)).unwrap();
        assert!(!case["probes"].as_array().unwrap().is_empty(), "{name}: the fixture holds no probe");
        for probe in case["probes"].as_array().unwrap() {
            let actual = composite(&scene, canvas_point(&lifted, probe)).map(|value| (value * 255.0).round());
            for channel in 0..3 {
                let expected = probe["rgb"][channel].as_f64().unwrap();
                assert!((actual[channel] - expected).abs() <= tolerance, "{name} at {:?}: {actual:?} vs {:?}", probe["at"], probe["rgb"]);
            }
        }
    }
}

#[test]
fn lifted_pages_keep_size_text_and_one_text_object_per_inked_page() {
    for case in fixture()["cases"].as_array().unwrap() {
        let source = snapshot(case);
        let lifted = lift(&source).unwrap();
        assert_eq!(lifted.pages.len(), source.pages.len());
        let texts: Vec<_> = objects(&lifted).into_iter().filter(|object| object.kind == page::ObjectKind::Text).collect();
        assert_eq!(texts.len(), source.pages.iter().filter(|page| !page.text.is_empty()).count());
        for (object, page) in texts.iter().zip(source.pages.iter().filter(|page| !page.text.is_empty())) {
            assert_eq!(object.text, page.text);
        }
        for (viewport, page) in page_viewports(&lifted).zip(&source.pages) {
            let [_, _, width, height] = viewport.clip_rect();
            assert_eq!((width, height), (page.width, page.height));
        }
    }
}

#[test]
fn canvas_window_offers_exactly_the_fixture_actions() {
    let offered: Vec<String> = window_definition().actions.iter().map(|action| format!("{:?}", action.id)).collect();
    for expected in fixture()["actions"].as_array().unwrap() {
        let expected = expected.as_str().unwrap();
        assert!(offered.iter().any(|id| id.contains(expected)), "{expected} missing from {offered:?}");
    }
    assert_eq!(offered.len(), fixture()["actions"].as_array().unwrap().len());
    assert!(window_kinds().iter().any(|kind| kind.id == page::THUMBNAILS_WINDOW_KIND_ID));
    assert!(window_kinds().iter().any(|kind| kind.id == page::TEXT_LAYER_WINDOW_KIND_ID));
}

#[test]
fn pointer_down_on_ink_selects_the_text_object_and_a_miss_selects_the_paper() {
    let case = fixture()["cases"][0].clone();
    let source = snapshot(&case);
    let lifted = lift(&source).unwrap();
    let text = objects(&lifted).into_iter().find(|object| object.kind == page::ObjectKind::Text).unwrap();
    let probe = ink(&case);
    let (x, y) = canvas_point(&lifted, &probe);
    let hit = emit_page_edit(&source, "canvasPointerDown", &payload("canvasPointerDown", &[("x", x), ("y", y)])).unwrap();
    assert!(hit.artifact_mutations.is_empty());
    assert!(format!("{:?}", hit.effects).contains(&text.id), "{:?}", hit.effects);
    let miss = emit_page_edit(&source, "canvasPointerDown", &payload("canvasPointerDown", &[("x", 5.0), ("y", 5.0)])).unwrap();
    assert!(format!("{:?}", miss.effects).contains("p0:paper"), "{:?}", miss.effects);
    assert!(emit_page_edit(&source, "canvasPointerMove", &payload("canvasPointerMove", &[("x", x), ("y", y)])).unwrap().effects.is_empty());
}

#[test]
fn set_text_on_the_selected_object_replaces_the_page_text_and_inverts() {
    use semio_os_kernel::{apply_diff_cold, ColdMutation};
    let case = fixture()["cases"][0].clone();
    let source = snapshot(&case);
    let id = objects(&lift(&source).unwrap()).into_iter().find(|object| object.kind == page::ObjectKind::Text).unwrap().id;
    let set = |text: &str| edit_from_action("set-text", Some(&DslValue::Object(vec![("page".into(), DslValue::float(1.0)), ("object".into(), DslValue::String(id.clone())), ("text".into(), DslValue::String(text.into()))]))).unwrap();
    let emit = emit_page_edit(&source, "set-text", &set("Edited")).unwrap();
    let [mutation] = emit.artifact_mutations.as_slice() else { panic!("one mutation") };
    assert!(matches!(mutation, PdfMutation::ReplacePageText(ReplacePageText { index: 0, text }) if text == "Edited"));
    let next = apply_diff_cold(mutation.diff(&source).diff(), &source).unwrap();
    assert_eq!(next.pages[0].text, "Edited");
    assert_eq!(objects(&lift(&next).unwrap()).into_iter().find(|object| object.kind == page::ObjectKind::Text).unwrap().text, "Edited");
    let mut restored = next;
    for inverse in mutation.inverse(&source).unwrap() { restored = apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, source);
    assert!(emit_page_edit(&source, "set-text", &set(&source.pages[0].text)).unwrap().artifact_mutations.is_empty());
    let missing = edit_from_action("set-text", Some(&DslValue::Object(vec![("page".into(), DslValue::float(1.0)), ("object".into(), DslValue::String("p0:nothing".into())), ("text".into(), DslValue::String("x".into()))]))).unwrap();
    assert!(emit_page_edit(&source, "set-text", &missing).is_err());
    assert!(find_object(&lift(&source).unwrap(), &id).is_some());
}

#[test]
fn page_structure_actions_emit_only_their_own_mutation_kind() {
    let source = snapshot(&fixture()["cases"][1]);
    let emit = |action: &str, entries: &[(&str, f64)]| emit_page_edit(&source, action, &payload(action, entries)).unwrap().artifact_mutations;
    let resized = emit("set-page-size", &[("page", 2.0), ("width", 8.0), ("height", 9.0)]);
    assert_eq!(resized.len(), 1);
    assert!(matches!(resized.as_slice(), [PdfMutation::ResizePage(ResizePage { index: 1, width: 8.0, height: 9.0 })]));
    let op = &resized[0];
    let next = semio_os_kernel::apply_diff_cold(op.diff(&source).diff(), &source).unwrap();
    assert_eq!(next.pages[0], source.pages[0]);
    assert_eq!(next.pages[1].text, source.pages[1].text);
    assert_eq!(next.pages[1].width.to_bits(), 8.0f64.to_bits());
    assert_eq!(next.pages[1].height.to_bits(), 9.0f64.to_bits());
    let mut restored = next;
    for inverse in op.inverse(&source).unwrap() { restored = semio_os_kernel::apply_diff_cold(inverse.diff(&restored).diff(), &restored).unwrap(); }
    assert_eq!(restored, source);
    let out = PdfMutation::ResizePage(ResizePage { index: 2, width: 8.0, height: 9.0 });
    assert!(emit_page_edit(&source, "set-page-size", &semio_os_kernel::OpText::print_op(&out)).is_err());
    assert!(emit_page_edit(&source, "set-page-size", &payload("set-page-size", &[("page", 3.0), ("width", 8.0), ("height", 9.0)])).is_err());
    assert!(matches!(emit("insert-page", &[("page", 2.0), ("width", 50.0), ("height", 60.0)]).as_slice(), [PdfMutation::InsertPage(InsertPage { index: 1, .. })]));
    assert!(matches!(emit("remove-page", &[("page", 1.0)]).as_slice(), [PdfMutation::RemovePage(RemovePage { index: 0 })]));
    assert!(matches!(emit("move-page", &[("page", 1.0), ("x", 2.0)]).as_slice(), [PdfMutation::MovePage(MovePage { from: 0, to: 1 })]));
    assert!(emit_page_edit(&source, "remove-page", &payload("remove-page", &[("page", 9.0)])).is_err());
    assert!(emit_page_edit(&source, "insert-image", &payload("set-page-size", &[("page", 1.0), ("width", 5.0), ("height", 5.0)])).is_err());
    assert!(edit_from_action("insert-image", Some(&args(&[("page", 0.0)]))).is_err());
    assert!(edit_from_action("set-page-size", None).is_err());
    assert!(edit_from_action("set-page-size", Some(&args(&[("page", 2.0), ("width", f64::NAN), ("height", 9.0)]))).is_err());
    assert!(emit_page_edit(&source, "set-page-size", "not json").is_err());
}

#[test]
fn view_lane_reduces_and_searches_over_the_lifted_pages() {
    let _ui_arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let lifted = lift(&snapshot(case)).unwrap();
        for query in case["search"].as_array().unwrap() {
            let outcome = search(&lifted, query["query"].as_str().unwrap(), query["matchCase"].as_bool().unwrap(), &mut SearchControl { cancelled: &|| false, progress: &mut |_, _| {} });
            assert_eq!(outcome.hits.len() as u64, query["hits"].as_u64().unwrap(), "{} {query}", case["name"]);
        }
    }
    let lifted = lift(&snapshot(&fixture["cases"][1])).unwrap();
    let fit = reduce_view(&lifted, &ViewState::default(), &ViewIntent::FitPage, (800.0, 600.0), &[]);
    assert_eq!(fit.mode, ZoomMode::FitPage);
    assert_eq!(reduce_view(&lifted, &ViewState::default(), &ViewIntent::GoToPage(1), (800.0, 600.0), &[]).page, 1);
    assert_eq!(page::thumbnails(&lifted, 96.0).len(), 2);
    for locale in [Locale::En, Locale::De] {
        let strip = page::render_thumbnail_strip(&lifted, "s.stdio.pdf@1.4/*#editor", page::OBJECT_DOMAIN, locale).unwrap();
        semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(strip), |_| ());
        let view = page::render_view(&lifted, None, &ViewState::default(), (800.0, 600.0), Vec::new(), locale).unwrap();
        semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(view), |_| ());
    }
}

#[test]
fn layout_places_the_canvas_beside_thumbnails_text_and_details() {
    let WindowLayoutNode::Split { children, .. } = document_layout("text-window").root else { panic!("split layout") };
    assert_eq!(children.len(), 5);
    assert_eq!(static_action("zoom-in"), "zoom-in");
    assert_eq!(static_action("insert-image"), "page-edit");
    assert!(is_page_action("canvasPointerDown") && !is_page_action("move"));
}
