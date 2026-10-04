use super::*;
use serde_json::{json, Value};

fn region(value: &Value) -> RasterRegion {
    RasterRegion {
        x: value["x"].as_u64().unwrap() as u32,
        y: value["y"].as_u64().unwrap() as u32,
        width: value["width"].as_u64().unwrap() as u32,
        height: value["height"].as_u64().unwrap() as u32,
        color: std::array::from_fn(|index| value["color"][index].as_u64().unwrap() as u8),
    }
}

#[test]
fn raster_region_fixtures_match_independent_patch_and_inverse() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let pixels: Vec<u8> = serde_json::from_value(row["pixels"].clone()).unwrap();
        let limits = RasterRegionLimits { maximum_raster_bytes: 1024, maximum_patch_bytes: row["patchBytes"].as_u64().unwrap() as usize, maximum_patches: 8 };
        let plan = RasterRegionPlan::new(row["width"].as_u64().unwrap() as u32, row["height"].as_u64().unwrap() as u32, pixels.len(), region(&row["region"]), limits).unwrap();
        assert_eq!(plan.patch_count(), row["patches"].as_u64().unwrap() as usize);
        let mut actual = pixels.clone();
        let mut changes = Vec::new();
        let mut inverse = Vec::new();
        for ordinal in 0..plan.patch_count() {
            if let Some(patch) = plan.patch(&pixels, ordinal).unwrap() {
                assert!(patch.pixels.len() <= limits.maximum_patch_bytes);
                actual[patch.index..patch.index + patch.pixels.len()].copy_from_slice(&patch.pixels);
                for (offset, value) in patch.pixels.iter().enumerate() {
                    let index = patch.index + offset;
                    changes.push(json!({"op":"replace", "path":format!("/{index}"), "value":value}));
                    inverse.push(json!({"op":"replace", "path":format!("/{index}"), "value":pixels[index]}));
                }
            }
        }
        let mut oracle = row["pixels"].clone();
        json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(json!(changes)).unwrap()).unwrap();
        assert_eq!(json!(actual), row["expected"], "{}", row["id"]);
        assert_eq!(oracle, row["expected"]);
        json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(json!(inverse)).unwrap()).unwrap();
        assert_eq!(oracle, row["pixels"]);
        assert_eq!(plan.patch(&pixels, plan.patch_count()).unwrap_err(), RasterRegionError::InvalidOrdinal);
        assert_eq!(plan.patch(&[], 0).unwrap_err(), RasterRegionError::NoncanonicalRaster);
        if row["id"] == "unchanged" {
            assert!(changes.is_empty());
        }
        eprintln!("[DEBUG] raster {}: {} bounded spans; independent patch and inverse agree", row["id"], plan.patch_count());
    }
}

#[test]
fn raster_region_refusal_fixtures_are_atomic() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["rejected"].as_array().unwrap() {
        let result = RasterRegionPlan::new(
            row["width"].as_u64().unwrap() as u32,
            row["height"].as_u64().unwrap() as u32,
            row["byteLength"].as_u64().unwrap() as usize,
            region(&row["region"]),
            RasterRegionLimits { maximum_raster_bytes: 1024, maximum_patch_bytes: row["patchBytes"].as_u64().unwrap() as usize, maximum_patches: row["maxPatches"].as_u64().unwrap() as usize },
        );
        assert_eq!(result.unwrap_err().code(), row["error"].as_str().unwrap(), "{}", row["id"]);
    }
}

#[test]
fn raster_region_plan_owns_checked_geometry_after_caller_edits() {
    let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let contract = &fixture["planOwnership"];
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["id"] == contract["case"]).unwrap();
    let pixels: Vec<u8> = serde_json::from_value(row["pixels"].clone()).unwrap();
    let mut input = region(&row["region"]);
    let plan = RasterRegionPlan::new(
        row["width"].as_u64().unwrap() as u32,
        row["height"].as_u64().unwrap() as u32,
        pixels.len(),
        input,
        RasterRegionLimits { maximum_raster_bytes: 1024, maximum_patch_bytes: row["patchBytes"].as_u64().unwrap() as usize, maximum_patches: 8 },
    )
    .unwrap();
    input.color = serde_json::from_value(contract["replacementColor"].clone()).unwrap();
    assert_ne!(input.color, region(&row["region"]).color);
    let patch = plan.patch(&pixels, 0).unwrap().unwrap();
    let mut actual = pixels.clone();
    actual[patch.index..patch.index + patch.pixels.len()].copy_from_slice(&patch.pixels);
    let changes: Vec<Value> = patch.pixels.iter().enumerate().map(|(offset, value)| json!({ "op": "replace", "path": format!("/{}", patch.index + offset), "value": value })).collect();
    let mut oracle = row["pixels"].clone();
    json_patch::patch(&mut oracle, &serde_json::from_value::<json_patch::Patch>(json!(changes)).unwrap()).unwrap();
    assert_eq!(json!(actual), row["expected"]);
    assert_eq!(oracle, row["expected"]);
    eprintln!("[DEBUG] raster plan ownership: checked geometry remains independent of the caller's later input");
}
