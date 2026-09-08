
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
    }
}

#[test]
fn uncarried_table_and_restart_recipes_are_byte_identical() {
    for id in ["replace-quant-table-applied", "remove-quant-table-applied", "replace-huffman-table-applied", "remove-huffman-table-applied", "change-restart-interval-applied"] {
        let (before, after) = recipe(id).unwrap();
        assert_eq!(before, after, "recipe {id} must be byte-identical — production discards this field");
    }
}

#[test]
fn change_jfif_header_recipe_differs_in_bytes_despite_being_reader_uncarried() {
    let (before, after) = recipe("change-jfif-header-applied").unwrap();
    assert_ne!(before, after);
}

#[test]
fn xmp_round_trips_through_the_splice() {
    let (before, after) = recipe("insert-other-segment-applied").unwrap();
    assert_ne!(before, after);
    let (before2, after2) = recipe("remove-other-segment-applied").unwrap();
    assert_ne!(before2, after2);
}

#[test]
fn replace_pixels_and_re_encode_quality_recipes_change_the_decoded_raster() {
    for id in ["replace-pixels-applied", "change-re-encode-quality-applied"] {
        let (before, after) = recipe(id).unwrap();
        let before_rgb = image::load_from_memory(&before).unwrap().to_rgb8();
        let after_rgb = image::load_from_memory(&after).unwrap().to_rgb8();
        assert_ne!(before_rgb.into_raw(), after_rgb.into_raw(), "recipe {id} must change the decoded raster");
    }
}
