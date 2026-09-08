
use super::*;
use std::mem::{align_of, offset_of, size_of};

#[test]
fn inferred_drawable_size_matches_recorded_typed_oracle() {
    let layer = MetalLayer::new();
    set_drawable_size(&layer, 4096, 2160);
    let actual = layer.drawable_size();
    assert_eq!((actual.width, actual.height), (4096.0, 2160.0));
}

#[test]
fn owned_size_abi_and_boundary_cases_match_the_language_neutral_fixture() {
    assert_eq!(size_of::<CoreGraphicsSize>(), 16);
    assert_eq!(align_of::<CoreGraphicsSize>(), 8);
    assert_eq!(offset_of!(CoreGraphicsSize, width), 0);
    assert_eq!(offset_of!(CoreGraphicsSize, height), 8);
    assert_eq!(CoreGraphicsSize::OBJECTIVE_C_ENCODING, "{CGSize=dd}");

    let layer = MetalLayer::new();
    for (width, height) in [(0, 0), (1, 1), (4096, 2160), (16384, 16384)] {
        set_drawable_size(&layer, width, height);
        let actual = layer.drawable_size();
        assert_eq!((actual.width, actual.height), (width as f64, height as f64));
    }
    for hostile in [16385, u32::MAX] {
        set_drawable_size(&layer, hostile, hostile);
        let actual = layer.drawable_size();
        assert_eq!((actual.width, actual.height), (16384.0, 16384.0));
    }
}

#[test]
fn owned_layer_retains_exactly_one_owner_and_accepts_nullable_device() {
    let layer = MetalLayer::new();
    layer.set_pixel_format(SURFACE_FORMAT);
    layer.set_framebuffer_only(true);
    let before = retain_count(&*layer);
    let retained = layer.clone();
    let during = retain_count(&*layer);
    drop(retained);
    let after = retain_count(&*layer);
    assert_eq!(during, before + 1);
    assert_eq!(after, before);
    layer.set_device(None);
    assert!(layer.next_drawable().is_none());
    println!("abi=16/8/{{CGSize=dd}} dimensions=6 maxPlusOne=clamped retainDelta=1 restored=true nullDevice=true nextDrawable=nil");
}
