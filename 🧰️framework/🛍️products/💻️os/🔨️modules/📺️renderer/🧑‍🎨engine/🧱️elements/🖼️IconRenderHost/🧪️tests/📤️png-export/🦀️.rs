use super::*;

fn contract() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️png-export/🔣️.json")).unwrap()
}

fn fixture_bytes(fixture: &serde_json::Value, name: &str) -> Vec<u8> {
    fixture["cases"].as_array().unwrap().iter().flat_map(|row| row[name].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8)).collect()
}

#[test]
fn icon_png_export_matches_canvas_straight_alpha_pixels() {
    let fixture = contract();
    let width = fixture["cases"].as_array().unwrap().len() as u32;
    let mut job = IconPngExport::new(width, 1, fixture_bytes(&fixture, "premultiplied")).unwrap();
    assert_eq!(job.progress(), (0, 0, width as usize));
    assert!(job.take_png().is_none());
    assert!(!job.advance().unwrap());
    assert_eq!(job.progress().0, 1);
    assert!(job.take_png().is_none());
    while !job.advance().unwrap() {}
    let bytes = job.take_png().unwrap();
    let decoded = semio_framework_pixels::decode_png(&bytes).unwrap();
    assert_eq!((decoded.width, decoded.height), (width, 1));
    assert_eq!(decoded.pixels, fixture_bytes(&fixture, "straight"));
    assert!(job.take_png().is_none());
}

#[test]
fn icon_png_export_bounds_each_phase_and_discards_cancelled_candidates() {
    let fixture = contract();
    assert_eq!(fixture["maximumPixelsPerStep"].as_u64().unwrap(), PIXELS_PER_STEP as u64);
    let width = PIXELS_PER_STEP as u32 * 4 + 1;
    let mut job = IconPngExport::new(width, 1, [64, 32, 16, 128].repeat(width as usize)).unwrap();
    assert!(!job.advance().unwrap());
    assert_eq!(job.progress(), (0, PIXELS_PER_STEP, width as usize));
    job.cancel();
    assert!(job.advance().is_err());
    assert!(job.take_png().is_none());
    for turns in [0, 5, 6] {
        let mut job = IconPngExport::new(width, 1, [64, 32, 16, 128].repeat(width as usize)).unwrap();
        for _ in 0..turns {
            let before = job.progress();
            assert!(!job.advance().unwrap());
            let after = job.progress();
            if before.0 == after.0 {
                assert!(after.1 > before.1);
                assert!(after.1 - before.1 <= if after.0 == 0 { PIXELS_PER_STEP } else { 4096 });
            }
        }
        job.cancel();
        assert!(job.take_png().is_none());
        assert!(job.advance().is_err());
    }
    assert!(IconPngExport::new(0, 1, Vec::new()).is_err());
    assert!(IconPngExport::new(1, 1, vec![0; 3]).is_err());
    assert!(IconPngExport::new(16385, 1, Vec::new()).is_err());
}
