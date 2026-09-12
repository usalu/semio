
use super::*;

#[test]
fn every_applied_recipe_id_resolves_except_the_refused_one() {
    for id in RECIPE_IDS {
        if *id == "change-byte-order-applied" {
            assert!(recipe(id).is_none(), "the refused recipe must not resolve a buildable pair");
            continue;
        }
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn encode_decode_round_trips_a_two_ifd_document() {
    let ifds = vec![IfdSpec { width: 4, height: 3, pixels: fill(4, 3, 0), description: Some("hi") }, IfdSpec { width: 2, height: 2, pixels: fill(2, 2, 9), description: None }];
    let bytes = write_doc(&ifds);
    fs::write("/tmp/tiff-ifd-codec-test.tiff", &bytes).unwrap();
    let json = project("/tmp/tiff-ifd-codec-test.tiff").expect("project the just-written file");
    assert!(json.contains("\"ifdCount\":2"));
    assert!(json.contains("little-endian"));
    assert!(json.contains("\"kind\":\"ascii\",\"value\":\"hi\""));
}
