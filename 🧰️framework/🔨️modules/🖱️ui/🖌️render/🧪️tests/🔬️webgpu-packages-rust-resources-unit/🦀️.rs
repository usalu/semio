
use super::*;

#[test]
fn single_channel_pixels_classify_as_glyph() {
    assert_eq!(classify_atlas_upload(4, 4, 16), Some(AtlasSlotKind::Glyph));
}

#[test]
fn four_channel_pixels_classify_as_icon() {
    assert_eq!(classify_atlas_upload(4, 4, 64), Some(AtlasSlotKind::Icon));
}

#[test]
fn mismatched_length_classifies_as_none() {
    assert_eq!(classify_atlas_upload(4, 4, 10), None);
}

#[test]
fn zero_area_classifies_as_none() {
    assert_eq!(classify_atlas_upload(0, 4, 0), None);
}
