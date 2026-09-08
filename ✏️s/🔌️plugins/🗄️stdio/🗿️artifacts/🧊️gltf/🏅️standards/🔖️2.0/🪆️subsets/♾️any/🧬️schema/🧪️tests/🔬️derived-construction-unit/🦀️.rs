mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn typed_constructors_build_a_decodable_triangle() {
        let mut b = GltfBuilderConstruction::empty();
        b.set_asset_version("2.0");
        let mut bytes = Vec::new();
        let verts: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        for v in verts {
            for c in v {
                bytes.extend_from_slice(&c.to_le_bytes());
            }
        }
        let buf = b.add_buffer(bytes);
        let bv = b.add_buffer_view(buf, 0, 36, None, Some(34962));
        let acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Vec3, 3).with_buffer_view(bv, 0).with_min_max(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 0.0]));
        let mat = b.add_material(GltfMaterial { pbr_metallic_roughness: Some(crate::schema::snapshot::GltfPbrMetallicRoughness { base_color_factor: [1.0, 0.0, 0.0, 1.0], ..Default::default() }), ..Default::default() });
        let mesh = b.add_mesh();
        b.add_mesh_primitive(mesh, &[("POSITION", acc)], None, Some(mat), None);
        let node = b.add_node(Some(mesh));
        let scene = b.add_scene(vec![node], None);
        b.set_default_scene(scene);
        let snapshot = b.build().expect("build");

        assert_eq!(snapshot.document.asset.version, "2.0");
        let decoded = crate::engine::decode_accessor(&snapshot.document, &snapshot.buffers, acc).expect("decode");
        assert_eq!(decoded.components, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    }
}
