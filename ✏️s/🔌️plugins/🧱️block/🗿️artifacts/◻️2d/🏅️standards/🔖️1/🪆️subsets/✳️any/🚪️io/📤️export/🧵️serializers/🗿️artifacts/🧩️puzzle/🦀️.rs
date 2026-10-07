//! 🧩️ Puzzle catalog physical projection.
use crate::Block2dSnapshot;
use semio_framework_pack_json::{array,Value};
/// 🌉️ Maps this `NodeKind` definition into the `s/plugin/puzzle` 2d manifest shape (`portKinds`/
/// `wireKinds`/`edgeKinds`/`nodeKinds`/`kindCompatibility` — see
/// `s/plugin/puzzle/app/2d/manifest/🔣️.json`), the seam puzzle imports through
/// its `Kit×Type` media port. Block owns no wire/edge-kind rows (`AGENTS.md`: referenced by
/// `default_wire_kind` only), so those arrays stay empty here — a merge keeps the puzzle manifest's
/// existing rows.
pub fn puzzle2d_manifest_fragment(definition: &Block2dSnapshot) -> Value {
    let port_kinds: Vec<Value> = definition.handle_kinds.iter().map(|kind| semio_framework_pack_json::json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "presentation": { "color": kind.color.as_str(), "defaultWireKind": kind.default_wire_kind.as_str() } })).collect();
    let handles: Vec<Value> = definition.handles.iter().map(|handle| semio_framework_pack_json::json!({ "handleKind": handle.handle_kind.as_str(), "angle": handle.angle, "radius": handle.radius })).collect();
    let node_kind = semio_framework_pack_json::json!({
        "id": definition.node_kind.id.as_str(),
        "name": definition.node_kind.name.as_str(),
        "presentation": {
            "meshUrl": null,
            "handles": handles,
        },
    });
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| semio_framework_pack_json::json!({ "bidirectional": rule.bidirectional, "specificity": "handle", "source": rule.source.as_str(), "target": rule.target.as_str() })).collect();
    semio_framework_pack_json::json!({
        "schema": "manifest",
        "id": definition.node_kind.id.as_str(),
        "name": definition.node_kind.name.as_str(),
        "axes": { "portModel": "ported", "directedness": "directed" },
        "portKinds": port_kinds,
        "wireKinds": Vec::<Value>::new(),
        "edgeKinds": Vec::<Value>::new(),
        "nodeKinds": [node_kind],
        "kindCompatibility": kind_compatibility,
    })
}
