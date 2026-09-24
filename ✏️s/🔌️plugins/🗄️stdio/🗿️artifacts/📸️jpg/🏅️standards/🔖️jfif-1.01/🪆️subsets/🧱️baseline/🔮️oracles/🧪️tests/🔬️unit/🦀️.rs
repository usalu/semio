use super::*;

fn scan() -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("semio-jpg-baseline-oracle");
    std::fs::create_dir_all(&dir).expect("temp dir");
    let path = dir.join("scan.jpg");
    std::fs::write(&path, include_bytes!("../../../🧫️fixtures/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg")).expect("write scan");
    path
}

/// 🖼️ libjpeg-turbo reads the committed scan as a baseline JPEG: SOF0, 8-bit, Huffman-coded, two DC
/// and two AC tables, three 1x1 components.
#[test]
fn libjpeg_reads_the_real_scan_inside_the_baseline_class() {
    let axes = read_axes(&scan()).expect("libjpeg-turbo's djpeg and rdjpgcom read the scan");
    assert_eq!((axes.sof_marker, axes.precision, axes.arithmetic), (0xC0, 8, false));
    assert_eq!(axes.huffman_tables, vec![("dc".to_string(), 0), ("ac".to_string(), 0), ("dc".to_string(), 1), ("ac".to_string(), 1)]);
    assert_eq!(axes.components, vec![(1, 1, 1), (2, 1, 1), (3, 1, 1)]);
    assert_eq!(verdict(&axes), Vec::<&str>::new());
}

/// 🛡️ Every kind moves its own axis, and the codes are the ones T.81's tables name.
#[test]
fn every_kind_moves_its_axis_and_the_verdict_the_tables_name() {
    let axes = Axes { sof_marker: 0xC0, precision: 8, arithmetic: false, huffman_tables: vec![("dc".to_string(), 0), ("ac".to_string(), 0), ("dc".to_string(), 1), ("ac".to_string(), 1)], components: vec![(1, 1, 1), (2, 1, 1), (3, 1, 1)] };
    let json = |text: &str| semio_repo_test_host::parse_json(text).expect("params");
    for (kind, params, code) in [
        ("set-sof-marker", r#"{"marker": 194}"#, Some("stdio.jpg.baseline.sof-marker")),
        ("set-sample-precision", r#"{"precision": 12}"#, Some("stdio.jpg.baseline.precision")),
        ("set-arithmetic", r#"{"arithmetic": true}"#, Some("stdio.jpg.baseline.arithmetic-conditioning-present")),
        ("insert-huffman-table", r#"{"index": 4, "class": "dc", "id": 2}"#, Some("stdio.jpg.baseline.huffman-table-count")),
        ("remove-huffman-table", r#"{"class": "dc", "id": 0}"#, None),
        ("insert-frame-component", r#"{"index": 3, "id": 4, "hSampling": 1, "vSampling": 1}"#, None),
        ("remove-frame-component", r#"{"id": 3}"#, None),
        ("set-component-sampling", r#"{"id": 1, "hSampling": 5, "vSampling": 1}"#, Some("stdio.jpg.baseline.component-sampling")),
    ] {
        let next = apply(&axes, kind, &json(params)).expect("applies");
        assert_ne!(project(&next), project(&axes), "{kind} must move the projection");
        assert_eq!(verdict(&next).first().copied(), code, "{kind}");
    }
}
