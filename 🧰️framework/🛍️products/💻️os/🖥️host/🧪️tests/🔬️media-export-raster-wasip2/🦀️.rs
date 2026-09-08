mod wasip2_tests {
    use super::{rasterize_svg_to_png_base64, svg_to_polylines};

    #[test]
    fn native_svg_engines_report_unavailable() {
        assert_eq!(rasterize_svg_to_png_base64("<svg/>", 1, 1), Err("SVG rasterization requires the native semio-framework-os host".into()));
        assert_eq!(svg_to_polylines("<svg/>"), Err("SVG path extraction requires the native semio-framework-os host".into()));
    }
}
