//! 🔁️ Cycle, layout and extension of the DAG model: cycle detection, the DAG layout, the DAG extension and the level of detail constants and indices. See [`super`].

use super::*;

// #endregion 🔖️IoNode

// #region 🔖️Acyclicity
/// 🚫️ Returns true when adding `source -> target` would create a cycle.
pub fn would_create_cycle(existing: &[(String, String)], source: &str, target: &str) -> bool {
    ::graph::algorithms::would_create_cycle_ids(existing, source, target)
}

// #endregion 🔖️Acyclicity

/// 🌳️ Applies original layered geometry directly to admitted DAG records.
pub fn apply_dag_layout(snapshot:&mut DagHostSnapshot,opts:&DagLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),DagError>{
 if snapshot.schema!="dag.hostDocument"{return Err(DagError::SchemaMismatch);}
 let ids:HashSet<String>=snapshot.nodes.iter().map(|n|n.id.clone()).collect();
 let mut handle_owners=HashMap::new();for node in &snapshot.nodes{for port in node.inputs().iter().chain(node.outputs().iter()){handle_owners.insert(format!("{}@{}",node.id,port.id),node.id.clone());}}
 let resolve=|key:&str|handle_owners.get(key).cloned().or_else(||ids.contains(key).then(||key.to_owned())).or_else(||key.split('@').next().filter(|base|ids.contains(*base)).map(str::to_owned)).unwrap_or_else(||key.to_owned());
 let directed:Vec<(String,String)>=snapshot.edges.iter().filter_map(|e|{let a=resolve(&e.source);let b=resolve(&e.target);(a!=b&&ids.contains(&a)&&ids.contains(&b)).then_some((a,b))}).collect();
 let positions=layered_positions(&ids,&directed,opts,control).map_err(|e|DagError::Json(e.to_string()))?;
 for node in &mut snapshot.nodes{if let Some(&(x,y))=positions.get(&node.id){node.x=x;node.y=y;}}Ok(())
}

// #endregion 🔖️Layout

// #region 🔖️GraphExtension
/// 🧩️ DAG-specific graph extension marker.
pub struct DagExtension;

impl canvas::CanvasExtension for DagExtension {
    fn extension_id(&self) -> &str {
        "dag"
    }
}

impl GraphExtension for DagExtension {}

// #endregion 🔖️GraphExtension

pub(super) const DAG_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-graph silhouette; fill only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Node icons only.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Horizontal abbreviations.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Vertical names with sections; channel abbreviations on ports.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Channel names on ports, port handles, and control text.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Full channel names on ports and maximum node fidelity.", max_zoom: f64::INFINITY },
];

const DAG_LOD_SCALE: LodScale = LodScale { lods: DAG_LODS };

/// 📶️ Uniform delay before each DAG LOD band activates (requires slightly more zoom for detail tiers).
pub(super) const DAG_LOD_ZOOM_SHIFT: f64 = ui_styling::metrics::dag::LOD_ZOOM_SHIFT;

const DAG_LOD_BAND_FLOOR_ZOOM: &[f64] = ui_styling::metrics::dag::LOD_BAND_FLOOR_ZOOM;

fn dag_lod_resolve_zoom(zoom: f64) -> f64 {
    (zoom - DAG_LOD_ZOOM_SHIFT).max(0.05)
}

pub fn dag_lod_index(zoom: f64) -> usize {
    DAG_LOD_SCALE.resolve_index(dag_lod_resolve_zoom(zoom))
}

pub fn dag_lod_band_floor_zoom(lod_index: usize) -> f64 {
    let floor = canvas::lod::band_floor_zoom(DAG_LOD_BAND_FLOOR_ZOOM, lod_index, 0.05);
    if lod_index == 0 {
        floor
    } else {
        (floor + DAG_LOD_ZOOM_SHIFT).max(0.05)
    }
}
