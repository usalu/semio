//! 🎚️ Constants, level of detail and naming of the board model: the hit tolerance and link constants, the draw level of detail, its scale JSON and the board and placement naming. See [`super`].

use super::*;

pub const EDGE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::EDGE_HIT_TOLERANCE_PX;

pub const HANDLE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::HANDLE_HIT_TOLERANCE_PX;

pub const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_MARKER_SCALE;

/// 🧱️ Hard ceiling on the handle rows [`BoardHost::handle_positions_json`] publishes into the DOM.
/// Nakagin carries 358 handles; a DOM attribute is not a place to stream a document.
pub const BOARD_HANDLE_VITALS_CAP: usize = 128;

/// 🤚️ Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
pub const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_RING_GAP_SCALE;

pub const LINK_DRAG_MIN_DISTANCE_PX: f64 = ui_styling::metrics::board::LINK_DRAG_MIN_DISTANCE_PX;

pub const LINK_HANDLE_SNAP_EXTRA_PX: f64 = ui_styling::metrics::board::LINK_HANDLE_SNAP_EXTRA_PX;

pub const LINK_COMMIT_SNAP_TIGHT_PX: f64 = ui_styling::metrics::board::LINK_COMMIT_SNAP_TIGHT_PX;

pub const DEFAULT_SUGGESTION_OFFSET: f64 = ui_styling::metrics::board::SUGGESTION_OFFSET;

pub const DEFAULT_BRUSH_NODE_SIZE: f64 = ui_styling::metrics::board::BRUSH_NODE_SIZE;

pub const BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;

/// 🖍️ World extent an area-brush click paints before the utility's own steppers state one — one
/// large grid cell, so the very first region a user drops is visible without configuring anything.
pub const REGION_DEFAULT_BRUSH_EXTENT_WORLD: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;

pub const DEFAULT_WIRE_KIND_ID: &str = "wire.link";

const BOARD_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-board silhouette; group selection and bounded drag only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology and indirect handle rings; no per-node picks.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Dense graph layout with simplified chrome.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Standard editing: nodes, edges, and handle rings.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node icons and richer strokes.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum fidelity including handle icons.", max_zoom: f64::INFINITY },
];

pub const BOARD_LOD_SCALE: LodScale = LodScale { lods: BOARD_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl BoardDrawLod {
    pub fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Some(match id {
            "minimap" => Self::Minimap,
            "overview" => Self::Overview,
            "compact" => Self::Compact,
            "normal" => Self::Normal,
            "detail" => Self::Detail,
            "micro" => Self::Micro,
            _ => return None,
        })
    }

    pub fn from_scale_index(index: usize) -> Self {
        match index {
            0 => Self::Minimap,
            1 => Self::Overview,
            2 => Self::Compact,
            3 => Self::Normal,
            4 => Self::Detail,
            _ => Self::Micro,
        }
    }
}

/// 🔭️ The board's six draw LOD bands ([`BoardDrawLod`]) as `[{id, name, description, maxZoom}]` JSON for LOD pickers.
pub fn board_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = BOARD_LODS
        .iter()
        .map(|lod| {
            serde_json::json!({
                "id": lod.id,
                "name": lod.name,
                "description": lod.description,
                "maxZoom": lod.max_zoom,
            })
        })
        .collect();
    serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
}

/// 🏷️ Domain naming a board consumer installs through [`BoardHost::set_naming`] and [`BoardFillSnapshotIngress::new`]:
/// the id prefixes of brush and fill placements, and the fill job's progress stage and stale-operation labels.
#[derive(Clone, Copy, Debug)]
pub struct BoardNaming {
    pub brush_node_prefix: &'static str,
    pub brush_edge_prefix: &'static str,
    pub fill_node_prefix: &'static str,
    pub fill_edge_prefix: &'static str,
    pub fill_stage_label: fn(BoardFillStage) -> &'static str,
    pub fill_stale_operation_fault: &'static str,
}

/// 📮️ The wire form of a domain's brush placement prefixes, JSON `{"brushNodePrefix":…,"brushEdgePrefix":…}`: what a scene
/// carries (`Board2dScene::placement_naming`) to hosts no domain can hand a `&'static` [`BoardNaming`], such as the os renderer's
/// wgpu `Board2d` surface. Installed with [`BoardHost::set_placement_naming_json`], it overrides the installed naming's brush prefixes.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BoardPlacementNaming {
    pub brush_node_prefix: String,
    pub brush_edge_prefix: String,
}

impl BoardPlacementNaming {
    /// 🧾️ Parses the scene wire form; both prefixes must be non-empty.
    pub fn from_json(json: &str) -> Result<Self, NormalPortError> {
        let naming: Self = serde_json::from_str(json).map_err(NormalPortError::PlacementNamingJson)?;
        if naming.brush_node_prefix.is_empty() || naming.brush_edge_prefix.is_empty() {
            return Err(NormalPortError::PlacementNamingEmpty);
        }
        Ok(naming)
    }

    /// 🖨️ The scene wire form of these prefixes.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl BoardNaming {
    /// 📤️ This naming's brush prefixes in wire form, for a domain to publish on its scene.
    pub fn placement_naming(&self) -> BoardPlacementNaming {
        BoardPlacementNaming { brush_node_prefix: self.brush_node_prefix.into(), brush_edge_prefix: self.brush_edge_prefix.into() }
    }
}
