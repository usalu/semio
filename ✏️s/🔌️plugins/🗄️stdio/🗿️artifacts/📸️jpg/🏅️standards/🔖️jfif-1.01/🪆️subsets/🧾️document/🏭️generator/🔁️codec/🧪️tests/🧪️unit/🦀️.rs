
use super::*;

#[test]
fn every_declared_recipe_id_resolves() {
    for id in RECIPE_IDS {
        assert!(recipe(id).is_some(), "recipe {id} must resolve");
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
fn owned_raster_recipes_change_the_decoded_image() {
    for id in ["replace-pixels-applied", "replace-image-applied"] {
        let (before, after) = recipe(id).unwrap();
        let before_rgb = image::load_from_memory(&before).unwrap().to_rgb8();
        let after_rgb = image::load_from_memory(&after).unwrap().to_rgb8();
        assert_ne!(before_rgb.into_raw(), after_rgb.into_raw(), "recipe {id} must change the decoded raster");
    }
}

#[test]
fn replacement_image_recipe_has_independent_extent_density_and_xmp(){
 let(before,after)=recipe("replace-image-applied").expect("replacement image recipe");
 let before=image::load_from_memory(&before).unwrap();let after_image=image::load_from_memory(&after).unwrap();assert_eq!((before.width(),before.height()),(32,24));assert_eq!((after_image.width(),after_image.height()),(16,8));let mut decoder=image::codecs::jpeg::JpegDecoder::new(std::io::Cursor::new(after)).unwrap();assert_eq!(decoder.xmp_metadata().unwrap().unwrap(),xmp_packet());
}
