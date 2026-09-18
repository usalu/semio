
use super::*;

#[test]
fn neutral_mesh_catalog_agrees_with_independent_serde_projection() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
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

/// 🌐️ The transport boundary law both renderers apply before handing a mesh id to a loader: a
/// `/mesh/…` PUBLIC id is rewritten to its catalog delivery path, everything else is passed through
/// untouched, and an id the catalog does not name is returned unchanged so the loader reports the
/// miss instead of this resolver inventing a filename.
///
/// React's `World3dHost` calls `meshAssetTransportUrl` inside every `useLoader(GLTFLoader, …)`; the
/// wgpu renderer applies the same rewrite on both of its fetch lanes — the browser frame Worker
/// (`🎞️frame-worker/🟦️.ts`, `fetch(meshAssetTransportUrl(request.url))`) and the native reader
/// (`🧊️renderer/🦀️.rs`, `native_renderer_asset_path` → `resolve_mesh_asset(url).source`). Without it
/// a url-declared world mesh 404s and the whole scene stays empty (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3d-world3d-glb-url-lane.md`).
#[test]
fn public_mesh_ids_reach_their_loader_through_the_catalog_transport_path() {
    let left = mesh_asset_transport_url("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb");
    assert_eq!(left, format!("/mesh/{}", resolve_mesh_asset("/mesh/🧊️hexagonal-cut-concrete-forest-left.glb").unwrap().path));
    assert_ne!(left, "/mesh/🧊️hexagonal-cut-concrete-forest-left.glb", "a public id is not its own delivery path");
    assert_eq!(mesh_asset_transport_url("https://external.test/model.glb"), "https://external.test/model.glb", "other asset domains keep ownership of their urls");
    assert_eq!(mesh_asset_transport_url("/infinite-assets/🖼️.jpg"), "/infinite-assets/🖼️.jpg");
    assert_eq!(mesh_asset_transport_url("/mesh/🧊️ellipsoid-🧊️capsule_J.glb"), "/mesh/🧊️ellipsoid-🧊️capsule_J.glb", "an unknown public id is never resolved to an invented filename");
}
