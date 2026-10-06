
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn no_op_recipes_have_byte_identical_before_and_after() {
    for id in ["no-mutation-no-op", "set-snapshot-no-op"] {
        let (before, after) = recipe(id).unwrap();
        assert_eq!(encode_gif(&before), encode_gif(&after), "recipe {id} must be byte-identical");
    }
}

#[test]
fn applied_recipes_change_the_bytes() {
    for id in RECIPE_IDS {
        if id.ends_with("-no-op") {
            continue;
        }
        let (before, after) = recipe(id).unwrap();
        assert_ne!(encode_gif(&before), encode_gif(&after), "recipe {id} must change the bytes");
    }
}

#[test]
fn project_round_trips_the_base_document() {
    let bytes = encode_gif(&base_doc());
    let json = project_gif(&bytes).expect("project the base document");
    assert!(json.contains("\"width\":8"));
    assert!(json.contains("\"frameCount\":3"));
    assert!(json.contains("\"backgroundColorIndex\":2"));
    assert!(json.contains("\"loopCount\":3"));
}

#[test]
fn interlace_flag_is_readable_via_next_frame_info_before_pixel_decode() {
    let mut doc = base_doc();
    doc.frames[0].interlaced = true;
    let bytes = encode_gif(&doc);
    let json = project_gif(&bytes).expect("project an interlaced document");
    let first_frame = json.split("\"frames\":[").nth(1).unwrap();
    assert!(first_frame.starts_with("{\"left\":0,\"top\":0,\"width\":4,\"height\":3,\"interlaced\":true"), "got: {first_frame}");
}

#[test]
fn interlaced_and_natural_encodings_project_the_same_pixel_bytes() {
    let mut interlaced_doc = base_doc();
    interlaced_doc.frames[0].interlaced = true;
    let natural_bytes = encode_gif(&base_doc());
    let interlaced_bytes = encode_gif(&interlaced_doc);
    assert_ne!(natural_bytes, interlaced_bytes, "the stored row order must actually differ");
    let natural_json = project_gif(&natural_bytes).unwrap();
    let interlaced_json = project_gif(&interlaced_bytes).unwrap();
    let extract_indices = |json: &str| json.split("\"indicesHex\":\"").nth(1).unwrap().split('"').next().unwrap().to_string();
    assert_eq!(extract_indices(&natural_json), extract_indices(&interlaced_json), "de-interlaced pixel bytes must be identical regardless of storage order");
}
