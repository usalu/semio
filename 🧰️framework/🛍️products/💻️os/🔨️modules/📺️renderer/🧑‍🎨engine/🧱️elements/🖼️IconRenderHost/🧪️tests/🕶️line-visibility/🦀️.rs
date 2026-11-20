use super::{camera_view_projection, project, icon_render_camera_pose, transform_clip, validate_request, vec3};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🕶️line-visibility/🔣️.json")).expect("projection and line visibility fixture")
}

fn point(value: &serde_json::Value) -> [f32; 3] {
    std::array::from_fn(|axis| value[axis].as_f64().expect("fixture coordinate") as f32)
}

#[test]
fn project_authored_camera_pose_and_zoom() {
    let fixture = fixture();
    let dimensions = (fixture["width"].as_u64().unwrap() as u32, fixture["height"].as_u64().unwrap() as u32);
    for case in fixture["projections"].as_array().unwrap() {
        let request = serde_json::json!({
            "assetUrl": "fixture.glb", "format": "svg", "width": dimensions.0, "height": dimensions.1,
            "camera": { "projection": case["kind"], "position": case["position"], "target": case["target"], "up": case["up"], "zoom": case["zoom"], "fov": case["fov"] },
        });
        let validated = validate_request(&request.to_string()).expect("projection fixture request");
        let camera = icon_render_camera_pose(&validated.request, Some(([-1.0; 3], [1.0; 3])));
        let matrix = camera_view_projection(&camera, dimensions);
        for (world, expected) in case["points"].as_array().unwrap().iter().zip(case["screen"].as_array().unwrap()) {
            let actual = project(transform_clip(matrix, vec3(point(world))), dimensions);
            for axis in 0..2 {
                assert!((f64::from(actual[axis]) - expected[axis].as_f64().unwrap()).abs() < 1e-4, "{}", case["kind"]);
            }
        }
    }
}

#[test]
fn svg_export_splits_rear_mesh_edges_even_with_fill_disabled() {
    use super::{IconSvgExport, tests::asset};
    let fixture = fixture();
    let geometry = &fixture["meshScene"];
    let asset_fixture = serde_json::json!({
        "geometry": { "positions": geometry["positions"], "indices": geometry["indices"], "normals": vec![[0,0,1]; 6] },
    });
    let request = serde_json::json!({
        "assetUrl": "fixture.glb", "format": "svg", "width": fixture["width"], "height": fixture["height"],
        "camera": geometry["camera"], "drawing": { "fill": false, "hiddenLines": true },
    });
    let mut export = IconSvgExport::new(&request.to_string(), asset(0x5356_4720, &asset_fixture)).expect("SVG export");
    let mut completed = false;
    for _ in 0..256 {
        if export.advance().expect("bounded SVG step") {
            completed = true;
            break;
        }
    }
    assert!(completed);
    let markup = String::from_utf8(export.take_svg().expect("SVG publication")).unwrap();
    assert!(!markup.contains("id=\"fills\""));
    for layer in ["visible", "hidden"] {
        let after_open = markup.split_once(&format!("id=\"{layer}-lines\"")).expect("line layer").1;
        let paths = after_open.split_once("</g>").expect("end of line layer").0;
        let expected = &geometry["expected"][layer];
        for path in expected.as_str().into_iter().chain(expected.as_array().into_iter().flatten().filter_map(serde_json::Value::as_str)) {
            let (from, to) = path[1..].split_once('L').expect("fixture line path");
            let reverse = format!("M{to}L{from}");
            assert!(paths.contains(path) || paths.contains(&reverse), "missing {layer} edge {path} in {paths}");
        }
    }
}
