
use super::*;

fn oracle_triangle_indices(mode: gltf::mesh::Mode, source: Vec<u32>) -> Vec<u32> {
    match mode {
        gltf::mesh::Mode::Triangles => source,
        gltf::mesh::Mode::TriangleStrip => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        gltf::mesh::Mode::TriangleFan => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn oracle_append_primitive(mesh: &mut MeshData, primitive: &gltf::Primitive<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    if !matches!(primitive.mode(), gltf::mesh::Mode::Triangles | gltf::mesh::Mode::TriangleStrip | gltf::mesh::Mode::TriangleFan) {
        return Ok(());
    }
    let reader = primitive.reader(|buffer| (buffer.index() == 0).then_some(bin));
    let positions: Vec<[f32; 3]> = reader.read_positions().ok_or_else(|| "glb triangle primitive missing POSITION".to_string())?.collect();
    let source_indices: Vec<u32> = reader.read_indices().map_or_else(|| (0..positions.len() as u32).collect(), |indices| indices.into_u32().collect());
    let indices = oracle_triangle_indices(primitive.mode(), source_indices);
    let normals: Vec<[f32; 3]> = if let Some(normals) = reader.read_normals() {
        normals.collect()
    } else {
        let mut local = MeshData { positions: positions.iter().flatten().copied().collect(), indices: indices.clone(), ..Default::default() };
        local.compute_normals();
        local.normals.as_chunks::<3>().0.to_vec()
    };
    let vertex_offset = mesh.vertex_count() as u32;
    for position in positions {
        mesh.positions.extend(glb_transform_point(matrix, position));
    }
    for normal in normals {
        mesh.normals.extend(glb_transform_normal(matrix, normal));
    }
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

fn oracle_append_mesh(mesh: &mut MeshData, source: &gltf::Mesh<'_>, matrix: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    for primitive in source.primitives() {
        oracle_append_primitive(mesh, &primitive, matrix, bin)?;
    }
    Ok(())
}

fn oracle_append_node(mesh: &mut MeshData, node: &gltf::Node<'_>, parent: GlbMatrix, bin: &[u8]) -> Result<(), String> {
    let matrix = glb_matrix_mul(parent, node.transform().matrix());
    if let Some(source) = node.mesh() {
        oracle_append_mesh(mesh, &source, matrix, bin)?;
    }
    for child in node.children() {
        oracle_append_node(mesh, &child, matrix, bin)?;
    }
    Ok(())
}

fn oracle_mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let document = gltf::Gltf::from_slice(bytes).map_err(|error| error.to_string())?;
    let bin = document.blob.as_deref().unwrap_or(&[]);
    let mut mesh = MeshData::default();
    if let Some(scene) = document.default_scene().or_else(|| document.scenes().next()) {
        for node in scene.nodes() {
            oracle_append_node(&mut mesh, &node, glb_identity(), bin)?;
        }
    } else {
        for source in document.meshes() {
            oracle_append_mesh(&mut mesh, &source, glb_identity(), bin)?;
        }
    }
    Ok(mesh)
}

fn assert_structurally_equal(ours: &MeshData, oracle: &MeshData) {
    assert_eq!(ours.indices, oracle.indices, "indices differ");
    assert_eq!(ours.positions.len(), oracle.positions.len(), "position count differs");
    for (a, b) in ours.positions.iter().zip(oracle.positions.iter()) {
        assert!((a - b).abs() < 1e-4, "position component differs: {a} vs {b}");
    }
    assert_eq!(ours.normals.len(), oracle.normals.len(), "normal count differs");
    for (a, b) in ours.normals.iter().zip(oracle.normals.iter()) {
        assert!((a - b).abs() < 1e-3, "normal component differs: {a} vs {b}");
    }
}

#[test]
fn differential_embedded_bin_chunk_matches_gltf_crate_oracle() {
    let bytes = include_bytes!("../../🧫️fixtures/🧊️gltf-codec/🧊️single-triangle-embedded.glb");
    let ours = mesh_from_glb(bytes).expect("first-party decode");
    let oracle = oracle_mesh_from_glb(bytes).expect("oracle decode");
    assert_structurally_equal(&ours, &oracle);
}

#[test]
fn differential_generated_uv_sphere_glb_matches_gltf_crate_oracle() {
    let mesh = mesh_uv_sphere(1.0, 8, 6);
    let bytes = mesh_to_glb(&mesh);
    let ours = mesh_from_glb(&bytes).expect("first-party decode");
    let oracle = oracle_mesh_from_glb(&bytes).expect("oracle decode");
    assert_structurally_equal(&ours, &oracle);
}

#[test]
fn differential_puzzle_fixture_glb_matches_gltf_crate_oracle() {
    let bytes = include_bytes!("../../../🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/🪝️j/🧊️capsule_J.glb");
    let ours = mesh_from_glb(bytes).expect("first-party decode");
    let oracle = oracle_mesh_from_glb(bytes).expect("oracle decode");
    assert_structurally_equal(&ours, &oracle);
}
