mod color_stroke_raster_tests {
    use super::*;

    const FIXTURES: [[f32; 4]; 12] = [
        [0.0, 0.0, 0.0, 1.0],
        [1.0, 1.0, 1.0, 1.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.5, 0.5, 0.5, 0.5],
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 0.75],
        [0.0, 0.0, 1.0, 0.25],
        [0.019_607_844, 0.0, 0.0, 1.0],
        [0.996, 0.004, 0.5, 0.9999],
        [0.313_725_5, 0.627_451, 0.941_176_5, 1.0],
        [1.5, -0.2, 0.5, 1.0],
        [0.001, 0.999, 0.333, 0.667],
    ];

    #[test]
    fn to_rgba8_agrees_with_peniko_color_across_fixtures() {
        for rgba in FIXTURES {
            let ours = Color::new(rgba).to_rgba8();
            let oracle = peniko::Color::new(rgba).to_rgba8();
            assert_eq!((ours.r, ours.g, ours.b, ours.a), (oracle.r, oracle.g, oracle.b, oracle.a), "to_rgba8 mismatch for {rgba:?}");
        }
    }

    #[test]
    fn from_rgba8_agrees_with_peniko_color_across_every_channel_byte() {
        for byte in 0..=255u8 {
            let ours = Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components();
            let oracle = peniko::Color::from_rgba8(byte, 255 - byte, byte / 2, byte).components;
            assert_eq!(ours, oracle, "from_rgba8 mismatch for byte {byte}");
        }
    }

    #[test]
    fn multiply_alpha_agrees_with_peniko_color_across_fixtures() {
        for rgba in FIXTURES {
            for factor in [0.0_f32, 0.25, 0.5, 1.0, 1.5] {
                let ours = Color::new(rgba).multiply_alpha(factor).components();
                let oracle = peniko::Color::new(rgba).multiply_alpha(factor).components;
                assert_eq!(ours, oracle, "multiply_alpha mismatch for {rgba:?} * {factor}");
            }
        }
    }

    #[test]
    fn stroke_to_kurbo_matches_kurbo_stroke_new_defaults() {
        let ours = Stroke::new(3.5).to_kurbo();
        let oracle = kurbo::Stroke::new(3.5);
        assert_eq!(ours.width, oracle.width);
        assert_eq!(ours.join, oracle.join);
        assert_eq!(ours.miter_limit, oracle.miter_limit);
        assert_eq!(ours.start_cap, oracle.start_cap);
        assert_eq!(ours.end_cap, oracle.end_cap);
        assert_eq!(ours.dash_pattern.as_slice(), oracle.dash_pattern.as_slice());
        assert_eq!(ours.dash_offset, oracle.dash_offset);
    }

    #[test]
    fn stroke_to_kurbo_reflects_dash_pattern_and_cap_setters() {
        let mut stroke = Stroke::new(2.0);
        stroke.set_dash_pattern(vec![4.0, 2.0]);
        stroke.set_start_cap(Cap::Butt);
        stroke.set_end_cap(Cap::Square);
        let built = stroke.to_kurbo();
        assert_eq!(built.dash_pattern.as_slice(), &[4.0, 2.0]);
        assert_eq!(built.start_cap, kurbo::Cap::Butt);
        assert_eq!(built.end_cap, kurbo::Cap::Square);
    }

    #[test]
    fn raster_image_to_peniko_preserves_dimensions_and_bytes() {
        let data = SharedArc::new(vec![1u8, 2, 3, 4, 5, 6, 7, 8]);
        let image = RasterImage::rgba8(1, 2, SharedArc::clone(&data));
        let built = image.to_peniko();
        assert_eq!(built.width, 1);
        assert_eq!(built.height, 2);
        assert_eq!(built.format, peniko::ImageFormat::Rgba8);
        assert_eq!(built.alpha_type, peniko::ImageAlphaType::Alpha);
        assert_eq!(built.data.data(), data.as_slice());
    }

    #[test]
    fn raster_image_clone_data_shares_the_same_backing_allocation() {
        let data = SharedArc::new(vec![9u8, 9, 9]);
        let image = RasterImage::rgba8(4, 4, SharedArc::clone(&data));
        let cloned = image.clone_data();
        assert!(SharedArc::ptr_eq(&image.data, &cloned.data));
        assert_eq!(cloned.width(), 4);
        assert_eq!(cloned.height(), 4);
    }
}
