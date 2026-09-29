mod native_raster_tests {
    use super::*;

    /// 🖼️ The owned PNG encoder behind `rasterize_svg_to_png_base64` writes a stream the third-party `png` crate (the
    /// oracle) decodes to exactly what the renderer drew: a 4×2 opaque red rectangle is 4×2 RGBA8, every pixel
    /// (255, 0, 0, 255).
    #[test]
    fn rasterized_svg_png_decodes_to_the_rendered_pixels_under_the_png_crate() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="2"><rect width="4" height="2" fill="#ff0000"/></svg>"##;
        let bytes = base64_codec::base64_standard_decode(rasterize_svg_to_png_base64(svg, 4, 2).expect("the native host renders SVG")).expect("a base64 payload");
        let mut reader = png::Decoder::new(std::io::Cursor::new(bytes)).read_info().expect("the oracle reads the header");
        let mut pixels = vec![0; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut pixels).expect("the oracle inflates the IDAT stream");
        assert_eq!((frame.width, frame.height, frame.color_type, frame.bit_depth), (4, 2, png::ColorType::Rgba, png::BitDepth::Eight));
        assert!(pixels[..frame.buffer_size()].chunks(4).all(|pixel| pixel == [255, 0, 0, 255]));
    }
}
