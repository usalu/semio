
use super::*;

#[test]
fn artifact_polyline_io_round_trips_layers_vertices_and_closure() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/📏️polyline-io/🔣️.json")).unwrap();
    let paths: Vec<(String, Vec<[f64; 2]>, bool)> =
        fixture["polylines"].as_array().unwrap().iter().map(|path| (path["layer"].as_str().unwrap().into(), serde_json::from_value(path["vertices"].clone()).unwrap(), path["closed"].as_bool().unwrap())).collect();
    let bytes = polylines_to_dwg_bytes(paths.iter().map(|(layer, vertices, closed)| (layer.as_str(), vertices.as_slice(), *closed))).unwrap();
    let drawing = dwg_from_bytes(&bytes).unwrap();
    assert_eq!(drawing.entities.len(), paths.len());
    for (entity, (layer, vertices, closed)) in drawing.entities.iter().zip(&paths) {
        assert_eq!(&drawing.layers[entity.layer].name, layer);
        assert!(matches!(&entity.geometry, DwgGeometry::LwPolyline { vertices: actual, closed: closure, .. } if actual == vertices && closure == closed));
    }
    let (svg, width, height) = dwg_drawing_to_svg(&drawing).unwrap();
    assert!(svg.contains("<path"));
    assert!(width > 0 && height > 0);
    println!("[DEBUG] Artifact polyline I/O: {} bytes, {} layers, {} entities", bytes.len(), drawing.layers.len(), drawing.entities.len());
}
