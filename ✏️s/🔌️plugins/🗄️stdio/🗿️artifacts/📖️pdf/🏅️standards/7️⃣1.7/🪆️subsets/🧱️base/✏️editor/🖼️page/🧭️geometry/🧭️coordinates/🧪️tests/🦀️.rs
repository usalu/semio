use super::*;

#[test]
fn page_corners_and_transform_inversion_match_shared_cases() {
    let cases: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in cases.as_array().unwrap() {
        let media = serde_json::from_value(case["media"].clone()).unwrap();
        let crop = serde_json::from_value(case["crop"].clone()).unwrap();
        let viewport = PageViewport::new(media, crop, case["angle"].as_i64().unwrap() as i32, case["unit"].as_f64(), case["top"].as_f64().unwrap());
        let point: [f64; 2] = serde_json::from_value(case["point"].clone()).unwrap();
        let expected: (f64, f64) = serde_json::from_value(case["display"].clone()).unwrap();
        let clip: [f64; 4] = serde_json::from_value(case["clip"].clone()).unwrap();
        assert_eq!(viewport.point(point[0], point[1]), expected);
        assert_eq!(viewport.clip_rect(), clip);
        let matrix = [0.0, 2.0, -3.0, 0.0, 20.0, 30.0];
        let (x, y) = apply(matrix, point[0], point[1]);
        assert_eq!(unapply(matrix, x, y), (point[0], point[1]));
    }
}
