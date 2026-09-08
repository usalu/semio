
use super::*;

#[test]
fn neutral_mesh_catalog_agrees_with_independent_serde_projection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    let catalog = parse_mesh_delivery_catalog(&fixture["delivery"].to_string(), |path| Ok(fixture["catalogs"][path].to_string())).unwrap();
    let actual: Vec<_> = catalog.iter().map(|entry| serde_json::json!({ "url": entry.url, "source": entry.source, "path": entry.path })).collect();
    assert_eq!(serde_json::Value::Array(actual), fixture["expected"]);
    for url in fixture["unknown"].as_array().unwrap() {
        assert!(!catalog.iter().any(|entry| Some(entry.url.as_str()) == url.as_str()));
    }
    assert_eq!(resolve_mesh_asset("/mesh/🧊️capsule_J.glb").unwrap().path, "🌱️metabolism/💊️capsules/🪝️j/🧊️capsule_J.glb");
    assert!(resolve_mesh_asset("/mesh/🧊️ellipsoid-🧊️capsule_J.glb").is_err());
}

#[test]
fn hostile_mesh_catalog_is_rejected_without_alias_or_path_fallback() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    for key in ["url", "source", "path"] {
        let mut input = fixture["delivery"].clone();
        let mut extra = serde_json::json!({ "url": "/mesh/🛖️hut.glb", "source": "🛖️hut/🧊️shape.glb", "path": "🛖️hut/🧊️shape.glb" });
        extra[key] = input["entries"][0][key].clone();
        input["entries"].as_array_mut().unwrap().push(extra);
        assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
    }
    for path in ["../🧊️shape.glb", "/🧊️shape.glb", "🏠️house//🧊️shape.glb", "🏠️house/%2e%2e/🧊️shape.glb", "🏠️house\\🧊️shape.glb", "🏠️house/./🧊️shape.glb"] {
        let mut input = fixture["delivery"].clone();
        input["entries"][0]["path"] = path.into();
        assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
    }
    let mut input = fixture["delivery"].clone();
    input["entries"][0]["alias"] = "/mesh/old.glb".into();
    assert!(parse_mesh_delivery_catalog(&input.to_string(), |path| Ok(fixture["catalogs"][path].to_string())).is_err());
    assert!(parse_mesh_delivery_catalog(&fixture["delivery"].to_string(), |_| Err("Unknown source".into())).is_err());
}
