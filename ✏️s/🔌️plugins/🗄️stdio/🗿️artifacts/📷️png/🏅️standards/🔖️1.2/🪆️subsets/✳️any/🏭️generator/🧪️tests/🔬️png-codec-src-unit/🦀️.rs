
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn every_recipe_round_trips_before_and_after() {
    for id in RECIPE_IDS {
        let (before, after) = recipe(id).unwrap();
        let before_bytes = encode_doc(&before);
        let after_bytes = encode_doc(&after);
        let decoded_before = decode_doc(&before_bytes);
        let decoded_after = decode_doc(&after_bytes);
        assert_eq!(decoded_before.info.width, before.info.width, "{id} before width round-trips");
        assert_eq!(decoded_after.info.width, after.info.width, "{id} after width round-trips");
    }
}

#[test]
fn change_gamma_recipe_actually_differs() {
    let (before, after) = recipe("change-gamma-applied").unwrap();
    let decoded_before = decode_doc(&encode_doc(&before));
    let decoded_after = decode_doc(&encode_doc(&after));
    assert_ne!(decoded_before.info.gama_chunk.map(|g| g.into_scaled()), decoded_after.info.gama_chunk.map(|g| g.into_scaled()));
}
