//! 🔤️ Complete BMP source consumption against an independent hexadecimal decoder.

use super::BmpSnapshot;
use store::ArtifactDsl;

#[test]
fn source_hex_consumption_matches_independent_decoder_without_panicking() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔤️source-hex/🔣️.json")).unwrap();
    let bytes: Vec<u8> = serde_json::from_value(fixture["bytes"].clone()).unwrap();
    let pixels: Vec<u8> = serde_json::from_value(fixture["pixels"].clone()).unwrap();
    let mut failures = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let source = case["source"].as_str().unwrap();
        let valid = case["valid"].as_bool().unwrap();
        let compact: String = source.chars().filter(|ch| !ch.is_whitespace()).collect();
        let oracle = hex::decode(&compact);
        assert_eq!(oracle.is_ok(), valid, "{id}: independent hexadecimal admission");
        let native = std::panic::catch_unwind(|| BmpSnapshot::parse_dsl(source));
        let Ok(native) = native else {
            failures.push(format!("{id}: malformed source panicked"));
            continue;
        };
        if native.is_ok() != valid {
            failures.push(format!("{id}: native admission differs from the independent decoder"));
            continue;
        }
        if valid {
            assert_eq!(oracle.unwrap(), bytes, "{id}: independent byte output");
            let snapshot = native.unwrap();
            let layout = crate::standards::v_v3::subsets::any::io::bmp_layout(&snapshot).unwrap();
            assert_eq!((layout.width, layout.height), (1, 1), "{id}");
            assert_eq!(crate::standards::v_v3::subsets::any::io::bmp_rgba8_preview(&snapshot).unwrap(), pixels, "{id}: decoded pixel output");
            assert_eq!(BmpSnapshot::parse_dsl(&snapshot.print_dsl()).unwrap(), snapshot, "{id}: canonical source roundtrip");
        }
        eprintln!("[DEBUG] bmp source-hex case={id} valid={valid}");
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
