//! 🧩️ Puzzle catalog physical projection.
use crate::Block3dSnapshot;
use semio_framework_pack_json::{array,Value};
use crate::standards::v1::subsets::any::schema::inferences::resolve_active_mesh_url;
/// 🌉️ Maps this `ObjectKind` definition into the `s/plugin/puzzle` 3d catalog shape (`objectKinds`/
/// `vortexKinds`/`cableKinds`/`attractionKinds` — see `Puzzle3dKindCatalogs`), the seam puzzle imports
/// through its `Kit×Type` media port. The active representation's mesh (first row, or the first
/// matching `wanted_tags`) becomes the catalog row's `meshUrl`.
pub fn puzzle3d_catalog_fragment(definition: &Block3dSnapshot, wanted_tags: &[&str]) -> Value {
    let vec3 = |v: [f64; 3]| array(v.iter().map(|c| Value::from(*c)));
    let vortices: Vec<Value> = definition.vortices.iter().map(|vortex| semio_framework_pack_json::json!({ "id": vortex.id.as_str(), "vortexKind": vortex.vortex_kind.as_str(), "position": vec3(vortex.position), "direction": vec3(vortex.direction), "radius": vortex.radius })).collect();
    let object_kind = semio_framework_pack_json::json!({
        "id": definition.object_kind.id.as_str(),
        "name": definition.object_kind.name.as_str(),
        "label": definition.object_kind.label.as_str(),
        "meshUrl": resolve_active_mesh_url(definition, wanted_tags),
        "vortices": vortices,
    });
    let vortex_kinds: Vec<Value> = crate::vortex_kinds_of(definition).iter().map(|kind| semio_framework_pack_json::json!({ "id": kind.id.as_str(), "name": kind.name.as_str(), "label": kind.label.as_str(), "color": kind.color.as_str(), "defaultCableKind": kind.default_cable_kind.as_str() })).collect();
    let kind_compatibility: Vec<Value> = definition.compatibility.iter().map(|rule| semio_framework_pack_json::json!({ "source": rule.source.as_str(), "target": rule.target.as_str(), "bidirectional": rule.bidirectional })).collect();
    semio_framework_pack_json::json!({
        "schema": "manifest",
        "objectKinds": [object_kind],
        "vortexKinds": vortex_kinds,
        "cableKinds": Vec::<Value>::new(),
        "attractionKinds": Vec::<Value>::new(),
        "kindCompatibility": kind_compatibility,
    })
}
