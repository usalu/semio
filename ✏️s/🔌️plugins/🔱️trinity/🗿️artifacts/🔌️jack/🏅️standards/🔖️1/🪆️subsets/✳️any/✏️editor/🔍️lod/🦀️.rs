//! 🔍️ Shared Jack graph detail scale for Jack and Rewriting app surfaces.

use semio_framework_os_infinite::canvas::lod::{Lod, LodScale};

const TRINITY_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; edges and node fills only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology without labels or port handles.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Abbreviated node names.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Full node names.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node names and port handles.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum port-graph fidelity.", max_zoom: f64::INFINITY },
];

/// 📏️ Ordered zoom boundaries used by graph rendering.
pub const TRINITY_LOD_SCALE: LodScale = LodScale { lods: TRINITY_LODS };

/// 📋️ Serializes the shared detail tiers for graph controls.
pub fn trinity_lod_scale_json() -> String {
    let rows: Vec<pack::JsonValue> = TRINITY_LODS
        .iter()
        .map(|lod| {
            pack::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    pack::json_to_string(&pack::json_array(rows))
}
