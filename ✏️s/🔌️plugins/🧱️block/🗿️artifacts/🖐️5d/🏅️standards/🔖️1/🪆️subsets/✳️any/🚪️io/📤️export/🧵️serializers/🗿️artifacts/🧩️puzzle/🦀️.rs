//! 🧩️ Puzzle catalog physical projection.
use crate::Block5dSnapshot;
use semio_framework_pack_json::{array,Value};
/// 🌉️ Maps this `PartKind` definition into the `s/plugin/puzzle` 5d catalog shape
/// (`Puzzle5dKindCatalogs`: `parts`/`grips`/`fasteners`/`ropes`), the seam puzzle imports through its
/// `Kit×Type` media port. Block owns no fastener/rope-kind rows, so those arrays stay empty here.
pub fn puzzle5d_catalog_fragment(definition: &Block5dSnapshot) -> Value {
    let vec3 = |v: [f64; 3]| array(v.iter().map(|c| Value::from(*c)));
    let grips: Vec<Value> = definition
        .grips
        .iter()
        .map(|grip| {
            semio_framework_pack_json::json!({
                "gripKind": grip.grip_kind.as_str(),
                "2d": { "angle": grip.angle, "gripKind": grip.grip_kind.as_str(), "radius": grip.radius_2d },
                "3d": { "position": vec3(grip.position), "direction": vec3(grip.direction), "radius": grip.radius_3d },
            })
        })
        .collect();
    let mesh_url = definition.representations.first().and_then(|representation| representation.mesh_url.clone());
    let part = semio_framework_pack_json::json!({
        "id": definition.part_kind.id.as_str(),
        "name": definition.part_kind.name.as_str(),
        "label": definition.part_kind.label.as_str(),
        "meshUrl": mesh_url,
        "grips": grips,
    });
    let grip_kinds: Vec<Value> = definition.grip_kinds.iter().map(|kind| semio_framework_pack_json::json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "label": kind.label.as_str(), "color": kind.color.as_str(), "defaultRopeKind": kind.default_rope_kind.as_str() })).collect();
    semio_framework_pack_json::json!({
        "schema": "manifest",
        "parts": [part],
        "grips": grip_kinds,
        "fasteners": Vec::<Value>::new(),
        "ropes": Vec::<Value>::new(),
        "kindCompatibility": definition.compatibility.iter().map(|rule| semio_framework_pack_json::json!({ "source": rule.source.as_str(), "target": rule.target.as_str(), "bidirectional": rule.bidirectional })).collect::<Vec<Value>>(),
    })
}
