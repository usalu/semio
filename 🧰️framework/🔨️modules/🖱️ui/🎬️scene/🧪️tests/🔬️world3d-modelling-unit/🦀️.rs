use super::*;
use serde_json::Value;

const FIXTURE: &str = include_str!("../🧫️fixtures/📏️world3d-modelling/🔣️.json");

#[test]
fn original_scalar_field_retirement_keeps_every_native_owner_and_physical_receipt() {
    use semio_framework_value::{retirement::controlled::ControlledRetirement,RetainedCloneGrant};
    let law:Value=serde_json::from_str(include_str!("../🧫️fixtures/♻️scalar-field/🔣️.json")).unwrap();
    let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
    use semio_framework_ui_scene_mesh::testkit::observe_retirement_allocations as observe;
    for row in fixture()["valid"].as_array().unwrap().iter().filter(|row|row["def"]=="scalarField") {
        let (source,heap)=observe(||serde_json::from_value::<World3dScalarField>(row["normalized"].clone()).unwrap());
        assert!(json_eq(&serde_json::to_value(&source).unwrap(),&row["normalized"]));
        let pointers=(source.mesh_id.as_ptr(),source.values.as_ptr(),source.legend.title.en.as_ptr(),source.legend.title.de.as_ptr(),source.legend.unit.as_ref().map(|unit|unit.as_ptr()));
        let (owner,handoff)=observe(||ControlledRetirement::new(source));
        assert_eq!(handoff,(0,0));
        let mut owner=owner.unwrap_or_else(|_|panic!("scalar field must declare original controlled retirement"));
        let original=owner.original().unwrap();
        assert_eq!((original.mesh_id.as_ptr(),original.values.as_ptr(),original.legend.title.en.as_ptr(),original.legend.title.de.as_ptr(),original.legend.unit.as_ref().map(|unit|unit.as_ptr())),pointers);
        let (mut births,mut releases)=heap;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            if owner.terminal_is_empty(){break;}
            let (denied,physical)=observe(||owner.step(RetainedCloneGrant::default()).unwrap());
            assert_eq!(denied.progress(),Default::default());assert_eq!(physical,(0,0));
            let (step,physical)=observe(||owner.step(grant).unwrap());let progress=step.progress();
            assert!(progress.fits(grant));assert_eq!(physical,(progress.retained_capacity_bytes,progress.released_bytes));
            births+=physical.0;releases+=physical.1;
        }
        assert!(owner.terminal_is_empty());assert_eq!(births,releases);
        assert_eq!(observe(||drop(owner)).1,(0,0));
    }
}

#[test]
fn original_projection_record_retirement_uses_exact_scalar_owner_receipts() {
    use semio_framework_value::{retirement::controlled::ControlledRetirement,RetainedCloneGrant};
    use semio_framework_ui_viewport::{Viewport3dProjectionSpec,Viewport3dProjectionFramePolicy};
    use semio_framework_ui_scene_mesh::testkit::observe_retirement_allocations as observe;
    let law:Value=serde_json::from_str(include_str!("../🧫️fixtures/♻️scalar-field/🔣️.json")).unwrap();
    let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();
    for row in law["projections"].as_array().unwrap() {
        let original:(Viewport3dProjectionSpec,Viewport3dProjectionFramePolicy)=serde_json::from_value(row.clone()).unwrap();
        assert!(json_eq(&serde_json::to_value(original).unwrap(),row));
        let (owner,handoff)=observe(||ControlledRetirement::new(original));assert_eq!(handoff,(0,0));
        let mut owner=owner.unwrap_or_else(|_|panic!("original scalar projection leaves declare exact retirement"));
        assert_eq!(owner.original(),Some(&original));let(mut births,mut releases)=(0,0);
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            if owner.terminal_is_empty(){break;}
            let (denied,physical)=observe(||owner.step(RetainedCloneGrant::default()).unwrap());assert_eq!(denied.progress(),Default::default());assert_eq!(physical,(0,0));
            let (step,physical)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(physical,(receipt.retained_capacity_bytes,receipt.released_bytes));births+=physical.0;releases+=physical.1;
        }
        assert!(owner.terminal_is_empty());assert_eq!(births,releases);assert_eq!(observe(||drop(owner)).1,(0,0));
    }
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("modelling fixture parses")
}

fn json_eq(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => (a.as_f64().unwrap() - b.as_f64().unwrap()).abs() <= 1e-12,
        (Value::Array(a), Value::Array(b)) => a.len() == b.len() && a.iter().zip(b).all(|(x, y)| json_eq(x, y)),
        (Value::Object(a), Value::Object(b)) => a.len() == b.len() && a.iter().all(|(key, x)| b.get(key).is_some_and(|y| json_eq(x, y))),
        _ => left == right,
    }
}

fn decode<T: FromValue>(value: &Value) -> Result<T, ValueError> {
    world3d_modelling_from_lane_text(&serde_json::to_string(value).unwrap())
}

fn check_valid<T: FromValue + ToValue + Serialize + PartialEq + std::fmt::Debug>(name: &str, value: &Value, normalized: &Value) {
    let typed: T = decode(value).unwrap_or_else(|error| panic!("{name} must decode: {error}"));
    let shape = serde_json::to_value(&typed).unwrap();
    assert!(json_eq(&shape, normalized), "{name}: serde shape {shape} differs from normalized {normalized}");
    let again: T = world3d_modelling_from_lane_text(&world3d_modelling_lane_text(&typed)).unwrap_or_else(|error| panic!("{name} lane text must decode back: {error}"));
    assert_eq!(again, typed, "{name} lane text round trip");
}

#[test]
fn valid_fixtures_decode_to_their_normalized_shape_and_round_trip_through_lane_text() {
    for entry in fixture()["valid"].as_array().unwrap() {
        let (name, value, normalized) = (entry["name"].as_str().unwrap(), &entry["value"], &entry["normalized"]);
        match entry["def"].as_str().unwrap() {
            "annotationLayer" => check_valid::<World3dAnnotationLayer>(name, value, normalized),
            "scalarField" => check_valid::<World3dScalarField>(name, value, normalized),
            "modellingOptions" => check_valid::<World3dModellingOptions>(name, value, normalized),
            other => panic!("unknown def {other}"),
        }
    }
}

#[test]
fn invalid_fixtures_are_refused_by_the_lane_codec_schema_level_and_semantic() {
    for entry in fixture()["invalid"].as_array().unwrap() {
        let (name, value) = (entry["name"].as_str().unwrap(), &entry["value"]);
        let refused = match entry["def"].as_str().unwrap() {
            "annotationLayer" => decode::<World3dAnnotationLayer>(value).is_err(),
            "scalarField" => decode::<World3dScalarField>(value).is_err(),
            "modellingOptions" => decode::<World3dModellingOptions>(value).is_err(),
            other => panic!("unknown def {other}"),
        };
        assert!(refused, "{name} must be refused: {}", entry["reason"]);
    }
    let marker = World3dAnnotation::marker("m", [0.0; 3], World3dText::new("Point", "Punkt"));
    let layer = |count: usize| World3dAnnotationLayer::new((0..count).map(|index| World3dAnnotation::marker(format!("m{index}"), [0.0; 3], World3dText::new("Point", "Punkt"))).collect());
    assert!(layer(WORLD3D_ANNOTATIONS_MAX).validate().is_ok());
    assert!(layer(WORLD3D_ANNOTATIONS_MAX + 1).validate().is_err());
    assert!(World3dAnnotationLayer::new(vec![marker.clone(), marker]).validate().is_err());
    for hostile in ["", "null", "[]", "4", "{\"items\":\"no\"}", "{\"items\":[null]}", "{\"items\":[],\"items\":[]}"] {
        assert!(world3d_modelling_from_lane_text::<World3dAnnotationLayer>(hostile).is_err(), "{hostile}");
    }
}

fn hex(rgb: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2])
}

fn ramp_named(name: &str) -> World3dColorRamp {
    World3dColorRamp::parse(name).expect("fixture ramp")
}

#[test]
fn colour_ramps_interpolate_exactly_as_the_three_js_oracle_in_the_fixture() {
    let fixture = fixture();
    let ramps = fixture["ramps"].as_object().unwrap();
    assert_eq!(ramps.len(), World3dColorRamp::ALL.len());
    for (name, ramp) in ramps {
        let kind = ramp_named(name);
        let stops: Vec<String> = kind.stops().iter().map(|stop| hex(*stop)).collect();
        let expected: Vec<&str> = ramp["stops"].as_array().unwrap().iter().map(|stop| stop.as_str().unwrap()).collect();
        assert_eq!(stops, expected, "{name} stops");
        for sample in ramp["samples"].as_array().unwrap() {
            let t = sample["t"].as_f64().unwrap();
            assert_eq!(hex(kind.sample(t)), sample["hex"].as_str().unwrap(), "{name} @ {t}");
        }
        assert_eq!(kind.sample(-3.0), kind.sample(0.0));
        assert_eq!(kind.sample(7.0), kind.sample(1.0));
        assert_eq!(kind.sample(f64::NAN), kind.sample(0.0));
    }
}

#[test]
fn scalar_fields_colour_values_clamp_to_the_range_and_mark_missing_data() {
    let fixture = fixture();
    assert_eq!(hex(WORLD3D_SCALAR_NO_DATA_RGB), fixture["noDataHex"].as_str().unwrap());
    for entry in fixture["scalarColors"].as_array().unwrap() {
        let field: World3dScalarField = decode(&entry["field"]).unwrap();
        let expected: Vec<&str> = entry["colors"].as_array().unwrap().iter().map(|color| color.as_str().unwrap()).collect();
        let colors: Vec<String> = field.values.iter().map(|value| hex(field.color_of(*value))).collect();
        assert_eq!(colors, expected);
        let bytes = field.color_bytes();
        assert_eq!(bytes.len(), field.values.len() * 3);
        for (index, color) in expected.iter().enumerate() {
            assert_eq!(hex([bytes[index * 3], bytes[index * 3 + 1], bytes[index * 3 + 2]]), *color);
        }
    }
}

#[test]
fn scalar_field_legends_sample_the_ramp_at_evenly_spaced_ticks() {
    for entry in fixture()["legends"].as_array().unwrap() {
        let field: World3dScalarField = decode(&entry["field"]).unwrap();
        let ticks = field.legend_ticks();
        let expected = entry["ticks"].as_array().unwrap();
        assert_eq!(ticks.len(), expected.len());
        assert_eq!(ticks.len(), usize::from(field.legend.ticks));
        for (tick, want) in ticks.iter().zip(expected) {
            assert!((tick.value - want["value"].as_f64().unwrap()).abs() <= 1e-12);
            assert_eq!(hex(tick.rgb), want["hex"].as_str().unwrap());
        }
    }
}

#[test]
fn the_pick_filter_admits_exactly_one_granularity() {
    for row in fixture()["pickTargets"].as_array().unwrap() {
        let filter = World3dPickGranularity::parse(row["filter"].as_str().unwrap()).unwrap();
        let targets = World3dPickGranularity::targets(Some(filter));
        let want = &row["targets"];
        assert_eq!((targets.mesh, targets.face, targets.edge, targets.vertex), (want["mesh"].as_bool().unwrap(), want["face"].as_bool().unwrap(), want["edge"].as_bool().unwrap(), want["vertex"].as_bool().unwrap()), "{filter:?}");
    }
    assert_eq!(World3dPickGranularity::targets(None), World3dPickTargets { mesh: true, face: true, edge: true, vertex: true });
}

#[test]
fn the_section_plane_matches_the_three_js_clipping_convention() {
    for row in fixture()["sectionPlanes"].as_array().unwrap() {
        let options: World3dModellingOptions = decode(&serde_json::json!({ "section": row["section"] })).unwrap();
        let plane = options.section.unwrap().clip_plane();
        for (component, want) in plane.iter().zip(row["plane"].as_array().unwrap()) {
            assert!((component - want.as_f64().unwrap()).abs() <= 1e-12, "{plane:?} vs {}", row["plane"]);
        }
    }
}

#[test]
fn text_resolves_for_the_active_locale_and_never_invents_a_language() {
    let text = World3dText::new("Width", "Breite");
    assert_eq!(text.resolve("de"), "Breite");
    assert_eq!(text.resolve("de-CH"), "Breite");
    assert_eq!(text.resolve("en-GB"), "Width");
    assert_eq!(text.resolve("fr"), "Width");
}

//#region 🔖️Overlay
fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or_else(|| panic!("{value} is a number"))
}

fn camera_view_proj(fixture: &Value) -> ([f32; 16], f32, f32) {
    use semio_framework_geometry::Vec3;
    use semio_framework_ui_scene_algebra::Mat4Math;
    use semio_framework_ui_scene_camera::{Camera3d, CameraProjection3d};
    let camera = &fixture["camera"];
    let point = |key: &str| Vec3 { x: number(&camera[key][0]) as f32, y: number(&camera[key][1]) as f32, z: number(&camera[key][2]) as f32 };
    let (width, height) = (number(&fixture["size"]["width"]) as f32, number(&fixture["size"]["height"]) as f32);
    let camera = Camera3d { position: point("position"), target: point("target"), up: point("up"), fov_y: (number(&camera["fovYDegrees"]) as f32).to_radians(), near: number(&camera["near"]) as f32, far: number(&camera["far"]) as f32, projection: CameraProjection3d::Perspective, zoom: 1.0 };
    (camera.view_proj(width, height).to_cols_array_m(), width, height)
}

fn near(left: f64, right: f64, tolerance: f64, what: &str) {
    assert!((left - right).abs() <= tolerance, "{what}: {left} differs from {right} by more than {tolerance}");
}

#[test]
fn the_annotation_projection_reproduces_the_three_js_screen_geometry() {
    let fixture = &fixture()["annotationProjection"];
    let (view_proj, width, height) = camera_view_proj(fixture);
    let layer: World3dAnnotationLayer = decode(&fixture["layer"]).expect("fixture layer decodes");
    let projected = project_world3d_annotations(&layer, &view_proj, width, height);
    let expected = fixture["expected"].as_array().unwrap();
    assert_eq!(projected.len(), expected.len());
    for (got, want) in projected.iter().zip(expected) {
        let id = want["id"].as_str().unwrap();
        assert_eq!((got.id.as_str(), got.kind, got.tone.as_str(), got.visible), (id, want["kind"].as_str().unwrap(), want["tone"].as_str().unwrap(), want["visible"].as_bool().unwrap()), "{id}");
        let lines = want["lines"].as_array().unwrap();
        assert_eq!(got.lines.len(), lines.len(), "{id} lines");
        for (line, row) in got.lines.iter().zip(lines) {
            for axis in 0..4 {
                near(line[axis], number(&row[axis]), 0.05, &format!("{id} line"));
            }
        }
        let arrows = want["arrows"].as_array().unwrap();
        assert_eq!(got.arrows.len(), arrows.len(), "{id} arrows");
        for (arrow, row) in got.arrows.iter().zip(arrows) {
            near(arrow.x, number(&row["x"]), 0.05, &format!("{id} arrow x"));
            near(arrow.y, number(&row["y"]), 0.05, &format!("{id} arrow y"));
            near(arrow.angle, number(&row["angle"]), 1e-3, &format!("{id} arrow angle"));
        }
        assert_eq!(got.arc.is_some(), want.get("arc").is_some(), "{id} arc");
        if let (Some(arc), Some(row)) = (got.arc, want.get("arc")) {
            for (value, key) in [(arc.cx, "cx"), (arc.cy, "cy"), (arc.radius, "radius")] {
                near(value, number(&row[key]), 0.05, &format!("{id} arc {key}"));
            }
            near(arc.start, number(&row["start"]), 1e-3, &format!("{id} arc start"));
            near(arc.sweep, number(&row["sweep"]), 1e-3, &format!("{id} arc sweep"));
        }
        assert_eq!(got.marker.is_some(), want.get("marker").is_some(), "{id} marker");
        if let (Some(marker), Some(row)) = (got.marker, want.get("marker")) {
            near(marker.x, number(&row["x"]), 0.05, &format!("{id} marker x"));
            near(marker.y, number(&row["y"]), 0.05, &format!("{id} marker y"));
            assert_eq!(marker.shape.as_str(), row["shape"].as_str().unwrap(), "{id} marker shape");
        }
        assert_eq!(got.label.is_some(), want.get("label").is_some(), "{id} label");
        if let (Some(label), Some(row)) = (got.label, want.get("label")) {
            near(label.x, number(&row["x"]), 0.05, &format!("{id} label x"));
            near(label.y, number(&row["y"]), 0.05, &format!("{id} label y"));
            let align = match label.align {
                World3dLabelAlign::Middle => "middle",
                World3dLabelAlign::Start => "start",
                World3dLabelAlign::End => "end",
            };
            assert_eq!(align, row["align"].as_str().unwrap(), "{id} label align");
        }
    }
    assert!(expected.iter().any(|row| row["visible"] == false) && expected.iter().any(|row| row["visible"] == true), "the fixture exercises visible and clipped annotations");
}

#[test]
fn legend_values_match_the_intl_number_format_fixture() {
    for row in fixture()["legendValues"]["rows"].as_array().unwrap() {
        let (locale, value, text) = (row["locale"].as_str().unwrap(), number(&row["value"]), row["text"].as_str().unwrap());
        assert_eq!(format_world3d_legend_value(value, locale), text, "{value} in {locale}");
    }
}

#[test]
fn legend_lines_match_the_react_legend_text_in_both_languages() {
    for row in fixture()["legendLines"]["rows"].as_array().unwrap() {
        let field: World3dScalarField = decode(&row["field"]).expect("legend fixture field");
        let locale = row["locale"].as_str().unwrap();
        let lines = field.legend_lines(locale);
        let want = &row["lines"];
        assert_eq!(lines.title, want["title"].as_str().unwrap(), "title in {locale}");
        let ticks = want["ticks"].as_array().unwrap();
        assert_eq!(lines.ticks.len(), ticks.len());
        for (tick, row) in lines.ticks.iter().zip(ticks) {
            assert_eq!(tick.text, row["text"].as_str().unwrap(), "tick in {locale}");
            assert_eq!(format!("#{:02x}{:02x}{:02x}", tick.rgb[0], tick.rgb[1], tick.rgb[2]), row["hex"].as_str().unwrap(), "tick colour in {locale}");
        }
        assert_eq!(lines.no_data, want["noData"].as_str(), "no-data caption in {locale}");
    }
}

#[test]
fn annotation_accessible_names_follow_the_active_language() {
    let layer: World3dAnnotationLayer = decode(&fixture()["annotationProjection"]["layer"]).unwrap();
    let names = |locale: &str| layer.items.iter().map(|item| item.accessible_name(locale)).collect::<Vec<_>>();
    assert_eq!(names("en")[0], "Dimension: Edge 4 mm");
    assert_eq!(names("de")[0], "Bemaßung: Kante 4 mm");
    assert_eq!(names("de-CH")[1], "Winkel: Rechter Winkel");
    assert_eq!(names("fr")[3], "Marker: Centre");
}

#[test]
fn sub_element_paint_lets_a_token_override_exactly_its_granularity() {
    let leak = |text: &str| -> &'static str { Box::leak(text.to_string().into_boxed_str()) };
    let rows = fixture()["subElementPaint"]["rows"].clone();
    for row in rows.as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let highlight: Option<World3dHighlight> = (!row["highlight"].is_null()).then(|| serde_json::from_value(row["highlight"].clone()).expect("fixture highlight"));
        let defaults = &row["defaults"];
        let defaults = World3dSubElementDefaults { select: leak(defaults["select"].as_str().unwrap()), hover: leak(defaults["hover"].as_str().unwrap()), edge_hover: leak(defaults["edgeHover"].as_str().unwrap()), edge_width: number(&defaults["edgeWidth"]), vertex_mark_px: number(&defaults["vertexMarkPx"]) };
        let paint = world3d_sub_element_paint(highlight.as_ref(), |tone| leak(&format!("tone:{}", tone.as_str())), defaults);
        let want = &row["expected"];
        let text = |key: &str| want[key].as_str().unwrap();
        assert_eq!((paint.face_select, paint.face_hover, paint.edge_select, paint.edge_hover, paint.vertex_select, paint.vertex_hover), (text("faceSelect"), text("faceHover"), text("edgeSelect"), text("edgeHover"), text("vertexSelect"), text("vertexHover")), "{name}");
        assert_eq!((paint.edge_width, paint.vertex_mark_px), (number(&want["edgeWidth"]), number(&want["vertexMarkPx"])), "{name}");
    }
}

#[test]
fn a_section_unit_normal_points_to_the_removed_side() {
    let section = World3dSection::new([1.0, 2.0, 3.0], [0.0, 0.0, 2.0]);
    assert_eq!(section.unit_normal(), [0.0, 0.0, 1.0]);
    assert_eq!(section.clip_plane(), [-0.0, -0.0, -1.0, 3.0]);
}
//#endregion 🔖️Overlay

#[global_allocator]
static OBSERVED_ALLOCATOR: semio_framework_ui_scene_mesh::testkit::ObservedAllocator = semio_framework_ui_scene_mesh::testkit::ObservedAllocator;
