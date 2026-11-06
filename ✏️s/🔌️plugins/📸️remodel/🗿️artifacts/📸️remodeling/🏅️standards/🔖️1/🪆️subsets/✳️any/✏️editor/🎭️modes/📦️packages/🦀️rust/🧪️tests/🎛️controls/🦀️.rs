use crate::editor::remodeling::modes::{analyze::windows::report, capture::windows::frames};
use semio_s_artifact_remodel_remodeling_editor_kit::editor::remodeling::terminology::RemodelingLabels;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::{default_remodeling_scene, FrameRef, GcpObservation, GroundControlPoint, MediaStream};
use semio_ui_locale::{AppLabels, Locale, Terminology};

#[test]
fn controls_publish_localized_values_and_addressed_actions() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️controls/🔣️.json")).unwrap();
    let mut scene = default_remodeling_scene();
    scene.streams.push(MediaStream { id: "stream-a".into(), name: "Sequence A".into(), frames: vec![FrameRef { index: 0, ..Default::default() }, FrameRef { index: 5, ..Default::default() }], ..Default::default() });
    let frame_config = frames::config::RemodelingFramesWindowConfig { frame_cursor: frames::config::RemodelingFrameCursor { stream_id: Some("stream-a".into()), frame_index: 5 } };
    let report_config = report::config::RemodelingReportWindowConfig { report_table: "gcps".into() };
    for case in cases.as_array().unwrap() {
        let locale = Locale::from_language_tag(case["locale"].as_str().unwrap()).unwrap();
        let labels = RemodelingLabels::labels(locale, Terminology::Native);
        let frame = serde_json::to_value(frames::window_measures(&scene, &frame_config, labels)).unwrap();
        let empty = serde_json::to_value(frames::window_measures(&default_remodeling_scene(), &Default::default(), labels)).unwrap();
        assert_eq!(empty[1]["disabled"], true);
        assert_eq!(frame[0]["id"], "remodeling-measure-frame-stream");
        assert_eq!(frame[0]["label"], case["stream"]);
        assert_eq!(frame[0]["value"], "stream-a");
        assert_eq!(frame[0]["items"][0]["value"], "stream-a");
        assert_eq!(frame[0]["onChange"]["action"], "setFrameCursor");
        assert_eq!(frame[0]["onChange"]["args"]["frameIndex"], 5);
        assert_eq!(frame[1]["id"], "remodeling-measure-frame-index");
        assert_eq!(frame[1]["kind"], "slider");
        assert_eq!(frame[1]["label"], case["frame"]);
        assert_eq!(frame[1]["value"], 5.0);
        assert_eq!(frame[1]["max"], 5.0);
        assert_eq!(frame[1]["onChange"]["args"]["streamId"], "stream-a");
        let table = serde_json::to_value(report::window_measures(&report_config, labels)).unwrap();
        assert_eq!(table[0]["id"], "remodeling-measure-report-table");
        assert_eq!(table[0]["label"], case["table"]);
        assert_eq!(table[0]["value"], "gcps");
        assert_eq!(table[0]["onChange"]["action"], "setReportTable");
        for (index, label) in case["tables"].as_array().unwrap().iter().enumerate() { assert_eq!(table[0]["items"][index]["label"], *label); }
    }
}

#[test]
fn different_window_configs_render_distinct_frame_and_table_scenes() {
    let mut scene = default_remodeling_scene();
    scene.streams.push(MediaStream { id: "stream-a".into(), frames: vec![FrameRef { index: 0, ..Default::default() }, FrameRef { index: 5, ..Default::default() }], ..Default::default() });
    scene.gcps.push(GroundControlPoint { id: "point-a".into(), observations: vec![GcpObservation { stream_id: "stream-a".into(), frame_index: 5, pixel: [1.0, 2.0] }], ..Default::default() });
    let first = frames::config::RemodelingFramesWindowConfig { frame_cursor: frames::config::RemodelingFrameCursor { stream_id: Some("stream-a".into()), frame_index: 0 } };
    let second = frames::config::RemodelingFramesWindowConfig { frame_cursor: frames::config::RemodelingFrameCursor { stream_id: Some("stream-a".into()), frame_index: 5 } };
    assert_ne!(format!("{:?}", frames::render(&scene, &first).unwrap()), format!("{:?}", frames::render(&scene, &second).unwrap()));
    let cameras = report::config::RemodelingReportWindowConfig { report_table: "cameras".into() };
    let gcps = report::config::RemodelingReportWindowConfig { report_table: "gcps".into() };
    assert_ne!(format!("{:?}", report::render(&scene, &cameras).unwrap()), format!("{:?}", report::render(&scene, &gcps).unwrap()));
}
