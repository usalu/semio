
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn indexed_round_trip_preserves_indices_and_palette() {
    let doc = base_indexed_doc();
    let bytes = encode(&doc);
    let projected = project(&bytes).expect("project the encoded indexed BMP");
    assert_eq!(projected.storage, "indexed");
    assert_eq!(projected.width, WIDTH);
    assert_eq!(projected.height, HEIGHT);
    let palette = projected.palette.expect("indexed file reports a palette");
    assert_eq!(&palette[..7], &base_palette()[..]);
    assert_eq!(projected.indices_hex.expect("indices"), to_hex(&base_indices()));
}

#[test]
fn direct_round_trip_preserves_solid_fill() {
    let doc = solid_direct_doc([10, 20, 30]);
    let bytes = encode(&doc);
    let projected = project(&bytes).expect("project the encoded direct-colour BMP");
    assert_eq!(projected.storage, "direct");
    assert!(projected.palette.is_none());
    let expected: Vec<u8> = [10u8, 20, 30].iter().copied().cycle().take(WIDTH as usize * HEIGHT as usize * 3).collect();
    assert_eq!(projected.pixels_hex.expect("pixels"), to_hex(&expected));
}

#[test]
fn insert_palette_entry_recipe_leaves_indices_untouched() {
    let (before, after) = recipe("insert-palette-entry-applied").unwrap();
    let (Content::Indexed { indices: bi, .. }, Content::Indexed { indices: ai, palette: ap, .. }) = (&before.content, &after.content) else {
        panic!("expected indexed content");
    };
    assert_eq!(bi, ai, "a palette-only mutation must leave the index buffer untouched");
    assert_eq!(ap.len(), 8, "one entry was inserted into the 7-entry base palette");
}
