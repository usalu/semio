#[cfg(not(target_arch = "wasm32"))]
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

mod registrar_tests {
    use super::*;
    use semio_framework_os_host_workflow::{export_handlers, import_handlers, os_media_handler_key};

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    #[test]
    fn svg_path_extraction_preserves_transformed_geometry() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect x="1" y="1" width="4" height="4"/></svg>"#;
        let paths = svg_to_polylines(svg).expect("SVG paths");
        assert_eq!(paths.len(), 1);
        assert!(paths[0].closed);
        assert_eq!(paths[0].vertices[0], [1.0, 9.0]);
    }

    #[test]
    fn mesh_exporter_registrar_round_trips_a_box_through_glb() {
        register_mesh_exporter("3d.__mesh_exporter_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework::mesh_io::GlbExporter));
        let result = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_handler_key("3d.__mesh_exporter_test", "glb")).expect("glb handler registered")(&serde_json::json!({})).expect("export glb");
        let bytes = base64_codec::base64_standard_decode(result.data).expect("decode base64");
        let mesh = semio_framework::mesh_io::binary::mesh_from_glb(&bytes).expect("glb decodes back to a mesh");
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn mesh_importer_registrar_round_trips_a_box_through_obj() {
        register_mesh_importer("3d.__mesh_importer_test", |mesh| Ok(serde_json::json!({ "vertexCount": mesh.vertex_count() })), Box::new(semio_framework::mesh_io::ObjImporter));
        let obj_bytes = semio_framework::mesh_io::text::mesh_to_obj(&semio_framework_plugin::mesh_from_kind("box"), "box").into_bytes();
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_handler_key("3d.__mesh_importer_test", "obj")).expect("obj handler registered");
        let document = handler(&obj_bytes).expect("import obj");
        assert!(document["vertexCount"].as_u64().expect("vertex count") > 0);
    }
}
