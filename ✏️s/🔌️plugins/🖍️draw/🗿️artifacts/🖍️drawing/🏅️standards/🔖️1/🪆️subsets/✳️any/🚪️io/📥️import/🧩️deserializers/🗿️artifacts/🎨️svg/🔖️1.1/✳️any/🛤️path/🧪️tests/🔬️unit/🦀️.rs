//! 🧪️ SVG normalization shared with the independent TypeScript/Three.js corpus.
use super::*;
#[test]
fn editable_svg_paths_match_neutral_geometry() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let actual=parse_editable_svg_path(row["source"].as_str().unwrap());
        if row["after"].is_null() {assert!(actual.is_err(),"{}",row["name"]);} else {assert_eq!(actual.unwrap(),serde_json::from_value::<Vec<PathSegment>>(row["after"].clone()).unwrap(),"{}",row["name"]);}
    }
    eprintln!("[DEBUG] SVG path normalization agrees with the neutral command corpus");
}
