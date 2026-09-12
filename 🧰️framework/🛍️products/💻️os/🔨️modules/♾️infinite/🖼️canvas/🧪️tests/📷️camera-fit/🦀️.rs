//! 📷️ Laws over the node-graph opening-camera fixture (`🔣️.json`): a stored camera is adopted only
//! when it already frames the graph it was stored for, and otherwise the graph is fitted.
//!
//! The defect this closes: the generation3d Flow window opened on the camera its document carried
//! (`x=94.75 y=-97.5 zoom=1.78`) into a 483x814 pane, which showed one node of seven while the
//! minimap showed the whole column — the stored camera won over any fit, forever.

use super::camera;
use serde_json::Value;

const CAMERA_FIT_FIXTURE: &str = include_str!("🔣️.json");

fn fixture() -> Value {
    serde_json::from_str(CAMERA_FIT_FIXTURE).expect("camera fit fixture")
}

fn bounds(value: &Value) -> Option<camera::ContentBounds> {
    value.as_object().map(|content| camera::ContentBounds {
        min_x: content["minX"].as_f64().expect("minX"),
        min_y: content["minY"].as_f64().expect("minY"),
        max_x: content["maxX"].as_f64().expect("maxX"),
        max_y: content["maxY"].as_f64().expect("maxY"),
    })
}

fn stored(value: &Value) -> Option<camera::Camera> {
    value.as_object().map(|cam| camera::Camera { x: cam["x"].as_f64().expect("x"), y: cam["y"].as_f64().expect("y"), zoom: cam["zoom"].as_f64().expect("zoom") })
}

fn viewport(value: &Value) -> camera::Viewport {
    camera::Viewport { width: value["width"].as_u64().expect("width") as u32, height: value["height"].as_u64().expect("height") as u32, dpr: 1.0 }
}

fn close(left: f64, right: f64) -> bool {
    (left - right).abs() <= 1e-9
}

#[test]
fn the_shared_constants_are_the_ones_the_fixture_was_pinned_against() {
    let document = fixture();
    let constants = &document["provenance"]["constants"];
    assert!(close(camera::CONTENT_FIT_PADDING_PX, constants["paddingPx"].as_f64().expect("paddingPx")));
    assert!(close(camera::CONTENT_FRAMED_MIN_COVERAGE, constants["minCoverage"].as_f64().expect("minCoverage")));
    assert!(close(camera::CONTENT_REFIT_MAX_COVERAGE, constants["refitMaxCoverage"].as_f64().expect("refitMaxCoverage")));
    assert!(close(camera::CANVAS_CAMERA_ZOOM_MIN, constants["zoomMin"].as_f64().expect("zoomMin")));
    assert!(close(camera::CANVAS_CAMERA_ZOOM_MAX, constants["zoomMax"].as_f64().expect("zoomMax")));
}

#[test]
fn every_fixture_row_opens_on_the_camera_the_law_names() {
    let document = fixture();
    let rows = document["rows"].as_array().expect("rows");
    assert!(rows.len() >= 8, "the opening-camera law needs more than a happy path");
    for row in rows {
        let name = row["name"].as_str().expect("row name");
        let content = bounds(&row["content"]);
        let view = viewport(&row["viewport"]);
        let stored_camera = stored(&row["stored"]);
        let (opened, fitted) =
            camera::startup_camera(stored_camera.as_ref(), content.as_ref(), &view, camera::CONTENT_FIT_PADDING_PX, camera::CONTENT_FRAMED_MIN_COVERAGE);
        let expect = &row["expect"];
        let expect_camera = &expect["camera"];
        assert!(close(opened.x, expect_camera["x"].as_f64().expect("x")), "{name}: camera x {} != {}", opened.x, expect_camera["x"]);
        assert!(close(opened.y, expect_camera["y"].as_f64().expect("y")), "{name}: camera y {} != {}", opened.y, expect_camera["y"]);
        assert!(close(opened.zoom, expect_camera["zoom"].as_f64().expect("zoom")), "{name}: camera zoom {} != {}", opened.zoom, expect_camera["zoom"]);
        assert_eq!(fitted, expect["fitted"].as_bool().expect("fitted"), "{name}: the wrong camera won");
        if let (Some(content), Some(stored_camera)) = (content.as_ref(), stored_camera.as_ref()) {
            let coverage = camera::content_coverage(content, stored_camera, &view);
            assert!(close(coverage, expect["coverage"].as_f64().expect("coverage")), "{name}: coverage {coverage} != {}", expect["coverage"]);
            let refits = content_left_the_view(content, stored_camera, &view);
            assert_eq!(refits, expect["refits"].as_bool().expect("refits"), "{name}: the refit gate disagrees");
        }
    }
}

/// 🔀️ The second, far stricter gate: a camera the viewer already set is only re-fitted when the
/// graph that changed under it left the view entirely.
fn content_left_the_view(content: &camera::ContentBounds, at: &camera::Camera, view: &camera::Viewport) -> bool {
    camera::content_coverage(content, at, view) <= camera::CONTENT_REFIT_MAX_COVERAGE
}

#[test]
fn a_fitted_camera_frames_the_whole_graph_it_was_fitted_to() {
    let document = fixture();
    for row in document["rows"].as_array().expect("rows") {
        let Some(content) = bounds(&row["content"]) else { continue };
        let view = viewport(&row["viewport"]);
        let fitted = camera::fit_camera(&content, &view, camera::CONTENT_FIT_PADDING_PX);
        let coverage = camera::content_coverage(&content, &fitted, &view);
        // 📐️ A graph past the zoom-out floor cannot be fully framed by construction; every other row
        // must come out whole, which is the only property a "fit" actually promises.
        let clamped = fitted.zoom <= camera::CANVAS_CAMERA_ZOOM_MIN;
        assert!(clamped || coverage >= 0.999, "{}: a fit that frames {coverage} of its own graph is not a fit", row["name"]);
    }
}

#[test]
fn the_fit_leaves_the_padding_it_promises() {
    let content = camera::ContentBounds { min_x: -200.0, min_y: -160.0, max_x: 240.0, max_y: -80.0 };
    let view = camera::Viewport { width: 483, height: 814, dpr: 1.0 };
    let fitted = camera::fit_camera(&content, &view, camera::CONTENT_FIT_PADDING_PX);
    let left = (content.min_x - fitted.x) * fitted.zoom + f64::from(view.width) * 0.5;
    let right = (content.max_x - fitted.x) * fitted.zoom + f64::from(view.width) * 0.5;
    assert!(left >= camera::CONTENT_FIT_PADDING_PX - 1e-6, "the fit pushed the graph into the left edge ({left}px)");
    assert!(f64::from(view.width) - right >= camera::CONTENT_FIT_PADDING_PX - 1e-6, "the fit pushed the graph into the right edge");
}

#[test]
fn a_camera_that_frames_nothing_is_never_reported_as_framing_something() {
    let content = camera::ContentBounds { min_x: 5000.0, min_y: 5000.0, max_x: 5200.0, max_y: 5100.0 };
    let view = camera::Viewport { width: 400, height: 400, dpr: 1.0 };
    let away = camera::Camera { x: 0.0, y: 0.0, zoom: 1.0 };
    assert!(close(camera::content_coverage(&content, &away, &view), 0.0));
    let (opened, fitted) = camera::startup_camera(Some(&away), Some(&content), &view, camera::CONTENT_FIT_PADDING_PX, camera::CONTENT_FRAMED_MIN_COVERAGE);
    assert!(fitted, "a camera that shows none of the graph must lose to the fit");
    assert!(close(camera::content_coverage(&content, &opened, &view), 1.0));
}
