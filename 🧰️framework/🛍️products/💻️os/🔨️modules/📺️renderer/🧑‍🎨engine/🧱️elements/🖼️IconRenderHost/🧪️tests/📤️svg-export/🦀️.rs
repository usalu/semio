use super::*;
use infinite_world::world::{World3dMeshAppearance, World3dPrimitiveMaterial};
use ui_wgpu::wgpu::{
    mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, Mesh3dSchema, SceneAuthoredMaterial3d,
    SceneMaterialAlpha3d,
};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️svg-export/🔣️.json")).expect("SVG export fixture")
}

fn fixture_vec3(value: &serde_json::Value) -> Vec<[f32; 3]> {
    value
        .as_array()
        .expect("SVG fixture vec3 array")
        .iter()
        .map(|value| {
            let value = value.as_array().expect("SVG fixture vec3");
            [0, 1, 2].map(|axis| value[axis].as_f64().expect("SVG fixture scalar") as f32)
        })
        .collect()
}

fn asset(generation: u64, fixture: &serde_json::Value) -> World3dMeshAsset {
    let positions = fixture_vec3(&fixture["geometry"]["positions"]);
    let normals = fixture_vec3(&fixture["geometry"]["normals"]);
    let indices: Vec<u32> = fixture["geometry"]["indices"]
        .as_array()
        .expect("SVG fixture indices")
        .iter()
        .map(|value| value.as_u64().expect("SVG fixture index") as u32)
        .collect();
    let edges: Vec<_> = indices
        .chunks_exact(3)
        .flat_map(|triangle| [[positions[triangle[0] as usize], positions[triangle[1] as usize]], [positions[triangle[1] as usize], positions[triangle[2] as usize]], [positions[triangle[2] as usize], positions[triangle[0] as usize]]])
        .collect();
    let schema = Mesh3dSchema {
        vertices: positions.len() as u32,
        indices: indices.len() as u32,
        face_ids: 0,
        vertex_ids: 0,
        edges: edges.len() as u32,
        edge_ids: 0,
        uvs: 0,
        colors: 0,
    };
    let token = mesh3d_begin(generation, generation, schema).expect("SVG mesh owner");
    while !mesh3d_allocate_step(token).expect("SVG mesh allocation") {}
    for position in positions {
        mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap();
    }
    for normal in normals {
        mesh3d_write_vec3(token, Mesh3dField::Normals, normal).unwrap();
    }
    for index in indices {
        mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap();
    }
    for edge in edges {
        mesh3d_write_edge(token, edge).unwrap();
    }
    let mesh = mesh3d_seal(token).expect("SVG mesh lease");
    let material = SceneAuthoredMaterial3d {
        base_color: [1.0; 4],
        emissive: [0.0; 3],
        metalness: 0.0,
        roughness: 1.0,
        alpha: SceneMaterialAlpha3d::Opaque,
        alpha_cutoff: 0.5,
        double_sided: false,
        preserve_vertex_color: false,
        base_color_texture: None,
        texture_sampler: Default::default(),
    };
    let appearance = World3dMeshAppearance::untextured(vec![World3dPrimitiveMaterial { first_index: 0, index_count: schema.indices, material }]).expect("SVG appearance");
    World3dMeshAsset { mesh, appearance }
}

fn request(value: &serde_json::Value) -> String {
    serde_json::json!({
        "assetUrl": "fixture.glb",
        "format": "svg",
        "camera": {
            "position": value["camera"]["position"],
            "target": value["camera"]["target"],
            "up": value["camera"]["up"],
            "zoom": 1,
            "fov": value["camera"]["fov"],
            "projection": "perspective"
        },
        "lights": {
            "ambientIntensity": value["lighting"]["ambientIntensity"],
            "ambientColor": value["lighting"]["ambientColor"],
            "sunAzimuth": 0,
            "sunElevation": 90,
            "sunIntensity": value["lighting"]["sunIntensity"],
            "sunColor": value["lighting"]["sunColor"]
        },
        "width": value["width"],
        "height": value["height"],
        "shape": value["shape"],
        "background": value["background"],
        "material": {
            "color": value["material"]["color"],
            "metalness": value["material"]["metalness"],
            "roughness": value["material"]["roughness"],
            "emissive": value["material"]["emissive"],
            "emissiveIntensity": 9,
            "stroke": value["material"]["stroke"]
        }
    })
    .to_string()
}

#[test]
fn svg_export_matches_neutral_three_and_sharp_fixture_and_retires_before_delivery() {
    let fixture = fixture();
    let mut export = IconSvgExport::new(&request(&fixture), asset(0x5356_4701, &fixture)).expect("SVG export");
    let initial_progress = export.progress();
    assert_eq!((initial_progress.1, initial_progress.2), (0, 4));
    let mut previous_completed = 0;
    let mut retired_progress = None;
    let mut steps = 0;
    while !export.advance().expect("bounded SVG step") {
        steps += 1;
        let progress = export.progress();
        assert_eq!(progress.2, initial_progress.2, "progress total survives exact asset retirement");
        assert!(progress.1 >= previous_completed, "progress is monotonic");
        previous_completed = progress.1;
        if export.asset.is_none() {
            retired_progress = Some(progress);
        }
        assert!(steps < 256, "SVG export exceeded the bounded fixture ladder");
    }
    assert!(export.asset.is_none(), "decoded mesh and appearance retire before publication");
    assert_eq!(retired_progress.map(|progress| (progress.1, progress.2)), Some((4, 4)));
    let bytes = export.take_svg().expect("one SVG delivery");
    assert!(export.take_svg().is_none(), "SVG delivery is one-shot");
    let markup = String::from_utf8(bytes).unwrap();
    assert!(markup.contains(&format!("fill:{}", fixture["expected"]["fill"].as_str().unwrap())));
    assert!(markup.contains(&format!("stroke:{}", fixture["expected"]["stroke"].as_str().unwrap())));
    assert!(markup.contains("semio-icon-ellipse-clip"));
    assert!(markup.contains("<rect x=\"-48\" y=\"-32\" width=\"96\" height=\"64\" fill=\"rgb(16,32,48)\"/>"));
    assert_eq!(markup.matches("<path ").count(), fixture["expected"]["pathCount"].as_u64().unwrap() as usize);
    println!("[DEBUG] icon SVG neutral receipt steps={steps} bytes={} paths={}", markup.len(), markup.matches("<path ").count());
    export.cancel();
    while !export.close_step() {}
    assert!(export.terminal_is_empty());
}

#[test]
fn svg_export_cancellation_retires_the_exact_asset_without_publication() {
    let fixture = fixture();
    let mut export = IconSvgExport::new(&request(&fixture), asset(0x5356_4702, &fixture)).expect("SVG export");
    export.advance().unwrap();
    export.cancel();
    let mut steps = 0;
    while !export.close_step() {
        steps += 1;
        assert!(steps < 256, "SVG cancellation exceeded the bounded fixture ladder");
    }
    assert!(export.terminal_is_empty());
    assert!(export.take_svg().is_none());
}
