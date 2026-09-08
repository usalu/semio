
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn applied_recipes_have_an_after_state_rejected_recipes_do_not() {
    for id in RECIPE_IDS {
        let (_, after) = recipe(id).unwrap();
        assert_eq!(after.is_some(), id.contains("-applied"), "recipe {id} outcome must match its own id");
    }
}

#[test]
fn encode_decode_round_trips_the_base_document() {
    let doc = base_doc();
    let bytes = encode_avi(&doc);
    let back = decode_avi(&bytes);
    assert_eq!(back.main_header.width, doc.main_header.width);
    assert_eq!(back.streams.len(), doc.streams.len());
    assert_eq!(back.streams[0].chunks.len(), doc.streams[0].chunks.len());
    assert_eq!(back.streams[0].chunks[0].keyframe, true);
    assert_eq!(back.streams[0].chunks[1].keyframe, false);
    assert_eq!(back.unknown_chunks.len(), 1);
    assert_eq!(back.unknown_chunks[0].fourcc, "JUNK");
}

#[test]
fn no_idx1_omits_the_idx1_chunk_entirely() {
    let mut doc = base_doc();
    doc.idx1_present = false;
    let bytes = encode_avi(&doc);
    let back = decode_avi(&bytes);
    assert_eq!(back.idx1_present, false);
}

#[test]
fn json_round_trips_via_hex_are_lossless() {
    let data = vec![0u8, 1, 254, 255, 16, 17];
    assert_eq!(from_hex(&to_hex(&data)), data);
}
