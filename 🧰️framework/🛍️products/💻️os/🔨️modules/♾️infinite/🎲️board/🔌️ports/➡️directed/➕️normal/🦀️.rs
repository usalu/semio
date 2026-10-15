//! 🧩️ Directed port graph normal leaf: `BoardHost`, board.ports.directed.v1, WASM session paint.

// #region board_host
//! 🕸️ Generic graph board host on infinite canvas.

#![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
#![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure call sites more than it clarifies.")]

use semio_framework_canvas::{Affine, Circle, Color, CubicBez, FillRule, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Point, Rect, Scene, Stroke, Vec2};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

use crate::infinite::board::ports::directed::{
    board_locked_option, board_visible_option, builtin_edge_tips, circle_handle_angle_toward, compute_edge_bezier_outward, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier,
    handle_exterior_cap_fill_path, handle_exterior_cap_peak, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, merge_ids_into_selection, merge_pick_into_selection, normalize_or_zero,
    normalize_selection_mode, pick_merge_mode_for_modifiers, property_bag_from_value, rectangle_handle_angle_toward, region_bounds, region_grip_at, region_grip_drag, rotate_point_about, selection_drag_enclosing, selection_drag_shape,
    snap_region_scalar, snap_transform_angle, transform_pivot_of,
    transform_ring_angle_delta, transform_ring_hit, transform_ring_radius_world, ActiveUtility, BoardElementStyleKind, CachedIconBody, CachedIconPaintLease, CanvasPalette, CompatSpecificity, EdgeData, EdgeDescriptor, EdgeKindDef,
    EdgeStrokePattern, EdgeTipDef, REGION_GRIP_PX, REGION_LABEL_INSET_PX, REGION_MIN_EXTENT_WORLD, TRANSFORM_RING_HIT_TOLERANCE_PX,
    EdgeTipGeometry, BoardSnapshot, GestureStage, GraphPortMode, HandleData, HandleDescriptor, HandleKindDef, IconPaintCache, Interaction, LinkCompatRule, NodeData, NodeDescriptor, NodeKindDef, NodeKindHandleTemplate, NodeShape, RegionData, RegionDescriptor,
    RegionGrip, SceneDescriptor, SelectionOptions, TransformGumballFlags, WireData, WireKindDef,
};
use semio_framework_canvas::camera::Camera;
use semio_framework_canvas::geom_sel::{
    cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
    world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
};
use semio_framework_graph_manifest::GraphManifest;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;

pub use semio_framework_canvas::camera::{CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX, CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN};

use semio_framework_canvas::lod::{Lod, LodScale};

//#region ⚠️ Errors
/// ⚠️ Errors from board host theme/catalog/layout JSON mutators and manifest validation.
#[derive(Debug)]
pub enum NormalPortError {
    Json(serde_json::Error),
    ExternalLinkPreviewJson(serde_json::Error),
    BrushSessionJson(serde_json::Error),
    DropPreviewJson(serde_json::Error),
    Theme(String),
    GridFactorOutOfRange,
    CompatNotArray,
    RowNotObject(&'static str),
    CompatSourceMissing,
    CompatTargetMissing,
    InvalidCompatSpecificity(String),
    LegacyLabelField(&'static str),
    KindCatalogsRootNotObject,
    IdMissing(&'static str),
    HandleKindColorMissing,
    InvalidHandleKindColor(String),
    NodeKindHandleKindMissing,
    NodeKindHandleAngleMissing,
    EdgeTipRowInvalid(String),
    CatalogMissingKind(&'static str, String),
    DropPreviewInvalid,
    InvalidHandleColor(String, String),
    EventCredits,
}

impl std::fmt::Display for NormalPortError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "{error}"),
            Self::ExternalLinkPreviewJson(error) => write!(formatter, "setLinkSessionJson: {error}"),
            Self::BrushSessionJson(error) => write!(formatter, "setBrushSessionJson: {error}"),
            Self::DropPreviewJson(error) => write!(formatter, "setDropPreviewJson: {error}"),
            Self::Theme(message) => formatter.write_str(message),
            Self::GridFactorOutOfRange => formatter.write_str("gridFactor must be finite and in (0, 1e6]"),
            Self::CompatNotArray => formatter.write_str("expected JSON array of compatibility objects"),
            Self::RowNotObject(row) => write!(formatter, "{row} row must be object"),
            Self::CompatSourceMissing => formatter.write_str("compat row missing string source"),
            Self::CompatTargetMissing => formatter.write_str("compat row missing string target"),
            Self::InvalidCompatSpecificity(value) => write!(formatter, "compat specificity must be general|node|edge|handle|wire|vortex, got {value:?}"),
            Self::LegacyLabelField(row) => write!(formatter, "{row} kind row must use name, not legacy label"),
            Self::KindCatalogsRootNotObject => formatter.write_str("kind catalogs root must be object"),
            Self::IdMissing(row) => write!(formatter, "{row} id missing"),
            Self::HandleKindColorMissing => formatter.write_str("handle kind color missing"),
            Self::InvalidHandleKindColor(color) => write!(formatter, "invalid handle kind color {color:?}"),
            Self::NodeKindHandleKindMissing => formatter.write_str("node kind handle handleKind missing"),
            Self::NodeKindHandleAngleMissing => formatter.write_str("node kind handle angle missing"),
            Self::EdgeTipRowInvalid(row) => write!(formatter, "edge tip row {row:?} invalid"),
            Self::CatalogMissingKind(catalog, kind) => write!(formatter, "catalog missing {catalog} kind {kind:?}"),
            Self::DropPreviewInvalid => formatter.write_str("setDropPreviewJson: preview payload missing nodeKind, screen/world point, or size"),
            Self::InvalidHandleColor(handle, color) => write!(formatter, "invalid color on handle {handle}: {color:?}"),
            Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),
        }
    }
}

impl std::error::Error for NormalPortError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => std::error::Error::source(error),
            Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::DropPreviewJson(error) => Some(error),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for NormalPortError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
//#endregion ⚠️ Errors

const GRID_WORLD_LARGE: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;
const GRID_WORLD_MEDIUM: f64 = ui_styling::metrics::board::GRID_WORLD_MEDIUM;
const GRID_WORLD_SMALL: f64 = ui_styling::metrics::board::GRID_WORLD_SMALL;
const GRID_WORLD_MICRO: f64 = ui_styling::metrics::board::GRID_WORLD_MICRO;
const GRID_FACTOR_DEFAULT: f64 = ui_styling::metrics::board::GRID_FACTOR_DEFAULT;
const EDGE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::EDGE_HIT_TOLERANCE_PX;
const HANDLE_HIT_TOLERANCE_PX: f64 = ui_styling::metrics::board::HANDLE_HIT_TOLERANCE_PX;
const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_MARKER_SCALE;
/// 🧱️ Hard ceiling on the handle rows [`BoardHost::handle_positions_json`] publishes into the DOM.
/// Nakagin carries 358 handles; a DOM attribute is not a place to stream a document.
const BOARD_HANDLE_VITALS_CAP: usize = 128;
/// Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = ui_styling::metrics::board::INDIRECT_HANDLE_RING_GAP_SCALE;
const LINK_DRAG_MIN_DISTANCE_PX: f64 = ui_styling::metrics::board::LINK_DRAG_MIN_DISTANCE_PX;
const LINK_HANDLE_SNAP_EXTRA_PX: f64 = ui_styling::metrics::board::LINK_HANDLE_SNAP_EXTRA_PX;
const LINK_COMMIT_SNAP_TIGHT_PX: f64 = ui_styling::metrics::board::LINK_COMMIT_SNAP_TIGHT_PX;
const DEFAULT_SUGGESTION_OFFSET: f64 = ui_styling::metrics::board::SUGGESTION_OFFSET;
const DEFAULT_BRUSH_NODE_SIZE: f64 = ui_styling::metrics::board::BRUSH_NODE_SIZE;
const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = ui_styling::metrics::board::SELECTION_CLICK_MAX_DISTANCE_PX;
const BOUNDED_DRAG_HIT_PAD_PX: f64 = ui_styling::metrics::board::BOUNDED_DRAG_HIT_PAD_PX;
/// 🖍️ World extent an area-brush click paints before the utility's own steppers state one — one
/// large grid cell, so the very first region a user drops is visible without configuring anything.
const REGION_DEFAULT_BRUSH_EXTENT_WORLD: f64 = ui_styling::metrics::board::GRID_WORLD_LARGE;
const DEFAULT_WIRE_KIND_ID: &str = "wire.link";

const PUZZLE_2D_LODS: &[Lod; 6] = &[
    Lod { id: "minimap", name: "Minimap", description: "Whole-board silhouette; group selection and bounded drag only.", max_zoom: 0.15 },
    Lod { id: "overview", name: "Overview", description: "Topology and indirect handle rings; no per-node picks.", max_zoom: 0.35 },
    Lod { id: "compact", name: "Compact", description: "Dense graph layout with simplified chrome.", max_zoom: 0.55 },
    Lod { id: "normal", name: "Normal", description: "Standard editing: nodes, edges, and handle rings.", max_zoom: 1.25 },
    Lod { id: "detail", name: "Detail", description: "Node icons and richer strokes.", max_zoom: 2.5 },
    Lod { id: "micro", name: "Micro", description: "Maximum fidelity including handle icons.", max_zoom: f64::INFINITY },
];

const PUZZLE_2D_LOD_SCALE: LodScale = LodScale { lods: PUZZLE_2D_LODS };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardDrawLod {
    Minimap,
    Overview,
    Compact,
    Normal,
    Detail,
    Micro,
}

impl BoardDrawLod {
    fn label(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::Overview => "overview",
            Self::Compact => "compact",
            Self::Normal => "normal",
            Self::Detail => "detail",
            Self::Micro => "micro",
        }
    }

    fn from_id(id: &str) -> Option<Self> {
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

    fn from_scale_index(index: usize) -> Self {
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

pub fn puzzle_2d_lod_scale_json() -> String {
    let rows: Vec<serde_json::Value> = PUZZLE_2D_LODS
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

/// 🎨️ Whether drawable style resolves committed selection chrome or neutral cached geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StyleChromePass {
    CachedBase,
    InteractionOverlay,
}

/// 🎨️ Which node/handle primitives to paint in a layered draw pass (fills behind icons/text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NodeHandlePaintLayer {
    Full,
    Fill,
    Stroke,
    Icons,
}

#[derive(Clone, Copy, Debug)]
struct BrushCandidateEntry {
    node_kind_id: BoardPointerSpan,
    target_handle_index: u16,
    sort_delta: f64,
}

#[derive(Clone, Copy)]
struct BrushCandidateRef<'a> {
    node_kind_id: &'a str,
    target_handle_index: usize,
}

#[derive(Clone)]
struct BrushCandidatePage {
    bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    byte_len: u16,
    entries: Box<[Option<BrushCandidateEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    len: u16,
}

impl Default for BrushCandidatePage {
    fn default() -> Self {
        Self { bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0 }
    }
}

impl BrushCandidatePage {
    fn push(&mut self, node_kind_id: &str, target_handle_index: usize, sort_delta: f64) -> Result<(), BoardEventFault> {
        let index = usize::from(self.len);
        if index == BOARD_POINTER_ITEM_CAPACITY || target_handle_index > u16::MAX as usize {
            return Err(BoardEventFault::ItemCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(node_kind_id.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(node_kind_id.as_bytes());
        self.byte_len = end as u16;
        self.entries[index] = Some(BrushCandidateEntry { node_kind_id: BoardPointerSpan { start: start as u16, len: node_kind_id.len() as u16 }, target_handle_index: target_handle_index as u16, sort_delta });
        self.len += 1;
        Ok(())
    }

    fn id_from(bytes: &[u8], span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&bytes[start..end]).expect("brush candidate ids originate from UTF-8 strings")
    }

    fn sort(&mut self) {
        let bytes = &self.bytes[..usize::from(self.byte_len)];
        self.entries[..usize::from(self.len)].sort_by(|left, right| {
            let left = left.expect("candidate entry");
            let right = right.expect("candidate entry");
            left.sort_delta
                .partial_cmp(&right.sort_delta)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| Self::id_from(bytes, left.node_kind_id).cmp(Self::id_from(bytes, right.node_kind_id)))
                .then_with(|| left.target_handle_index.cmp(&right.target_handle_index))
        });
    }

    fn get(&self, index: usize) -> Option<BrushCandidateRef<'_>> {
        let entry = self.entries.get(index)?.as_ref()?;
        Some(BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    fn first(&self) -> Option<BrushCandidateRef<'_>> {
        self.get(0)
    }

    fn iter(&self) -> impl Iterator<Item = BrushCandidateRef<'_>> {
        self.entries[..usize::from(self.len)].iter().flatten().map(|entry| BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    fn len(&self) -> usize {
        usize::from(self.len)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn clear(&mut self) {
        self.byte_len = 0;
        self.len = 0;
    }
}

#[derive(Clone, Debug)]
struct BrushPreviewSnapshot {
    source_handle_id: String,
    node_kind_id: String,
    x: f64,
    y: f64,
    shape: NodeShape,
    radius: f64,
    width: f64,
    height: f64,
    handles: Vec<NodeKindHandleTemplate>,
    target_handle_index: usize,
    icon_kind: Option<String>,
}

pub const BOARD_FILL_TEXT_BYTES: usize = 256;
pub const BOARD_FILL_PAGE_ITEMS: usize = 4;
pub const BOARD_FILL_NODE_CAPACITY: usize = 1_024;
pub const BOARD_FILL_HANDLE_CAPACITY: usize = 1_024;
pub const BOARD_FILL_KIND_CAPACITY: usize = 64;
pub const BOARD_FILL_KIND_HANDLE_CAPACITY: usize = 16;
pub const BOARD_FILL_RULE_CAPACITY: usize = 1_024;
pub const BOARD_FILL_PLACEMENT_CAPACITY: usize = 1_000;
const BOARD_FILL_SOURCE_CAPACITY: usize = BOARD_FILL_HANDLE_CAPACITY + BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY;
const BOARD_FILL_CANDIDATE_CAPACITY: usize = BOARD_FILL_KIND_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardFillText {
    bytes: [u8; BOARD_FILL_TEXT_BYTES],
    len: u16,
}

impl BoardFillText {
    pub fn empty() -> Self {
        Self { bytes: [0; BOARD_FILL_TEXT_BYTES], len: 0 }
    }

    pub fn try_from_str(value: &str) -> Result<Self, BoardFillCaptureFault> {
        if value.len() > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        let mut bytes = [0; BOARD_FILL_TEXT_BYTES];
        bytes[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self { bytes, len: value.len() as u16 })
    }

    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.bytes[..usize::from(self.len)]) }
    }

    fn try_indexed(prefix: &str, first: u64, middle: &str, second: Option<usize>) -> Result<Self, BoardFillCaptureFault> {
        let mut result = Self { bytes: [0; BOARD_FILL_TEXT_BYTES], len: 0 };
        result.try_extend(prefix.as_bytes())?;
        result.try_decimal(first)?;
        result.try_extend(middle.as_bytes())?;
        if let Some(second) = second {
            result.try_decimal(second as u64)?;
        }
        Ok(result)
    }

    fn try_extend(&mut self, bytes: &[u8]) -> Result<(), BoardFillCaptureFault> {
        let start = usize::from(self.len);
        let end = start.checked_add(bytes.len()).ok_or(BoardFillCaptureFault::TextCapacity)?;
        if end > BOARD_FILL_TEXT_BYTES {
            return Err(BoardFillCaptureFault::TextCapacity);
        }
        self.bytes[start..end].copy_from_slice(bytes);
        self.len = end as u16;
        Ok(())
    }

    pub fn try_push_byte(&mut self, byte: u8) -> Result<(), BoardFillCaptureFault> {
        self.try_extend(std::slice::from_ref(&byte))
    }

    fn try_decimal(&mut self, mut value: u64) -> Result<(), BoardFillCaptureFault> {
        let mut digits = [0_u8; 20];
        let mut len = 0;
        loop {
            digits[len] = b'0' + (value % 10) as u8;
            len += 1;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        for digit in digits[..len].iter().rev() {
            self.try_extend(std::slice::from_ref(digit))?;
        }
        Ok(())
    }
}

struct BoardFillPage<T> {
    items: Vec<T>,
    backing_bytes: usize,
}

impl<T> BoardFillPage<T> {
    fn try_new() -> Result<Self, ()> {
        let mut items = Vec::new();
        items.try_reserve_exact(BOARD_FILL_PAGE_ITEMS).map_err(|_| ())?;
        let backing_bytes = items.capacity().checked_mul(size_of::<T>()).ok_or(())?;
        Ok(Self { items, backing_bytes })
    }
}

struct BoardFillFixedPages<T, const CAPACITY: usize> {
    pages: Box<[Option<BoardFillPage<T>>]>,
    len: usize,
    page_count: usize,
}

impl<T, const CAPACITY: usize> BoardFillFixedPages<T, CAPACITY> {
    fn new() -> Self {
        Self::try_new().expect("Puzzle2d fill fixed descriptor backing allocation")
    }

    fn try_new() -> Result<Self, ()> {
        let _ = CAPACITY.checked_mul(size_of::<Option<BoardFillPage<T>>>()).ok_or(())?;
        let mut pages = Vec::new();
        pages.try_reserve_exact(CAPACITY).map_err(|_| ())?;
        pages.resize_with(CAPACITY, || None);
        let pages = pages.into_boxed_slice();
        if pages.len() != CAPACITY {
            return Err(());
        }
        Ok(Self { pages, len: 0, page_count: 0 })
    }

    fn item_capacity() -> usize {
        CAPACITY * BOARD_FILL_PAGE_ITEMS
    }

    fn try_push_owned(&mut self, item: T) -> Result<(), T> {
        if self.len == Self::item_capacity() {
            return Err(item);
        }
        let page_index = self.len / BOARD_FILL_PAGE_ITEMS;
        if self.pages[page_index].is_none() {
            let page = match BoardFillPage::try_new() {
                Ok(page) => page,
                Err(()) => return Err(item),
            };
            self.pages[page_index] = Some(page);
            self.page_count += 1;
        }
        let Some(page) = self.pages[page_index].as_mut() else { return Err(item) };
        page.items.push(item);
        self.len += 1;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        self.pages[index / BOARD_FILL_PAGE_ITEMS].as_ref()?.items.get(index % BOARD_FILL_PAGE_ITEMS)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        self.pages[index / BOARD_FILL_PAGE_ITEMS].as_mut()?.items.get_mut(index % BOARD_FILL_PAGE_ITEMS)
    }

    fn pop(&mut self) -> Option<T> {
        let index = self.len.checked_sub(1)?;
        let item = self.pages[index / BOARD_FILL_PAGE_ITEMS].as_mut()?.items.pop()?;
        self.len = index;
        Some(item)
    }

    fn retire_empty_page(&mut self) -> Option<usize> {
        let first_unused = self.len.div_ceil(BOARD_FILL_PAGE_ITEMS);
        let index = self.page_count.checked_sub(1)?;
        if index < first_unused || !self.pages[index].as_ref()?.items.is_empty() {
            return None;
        }
        let bytes = self.pages[index].as_ref()?.backing_bytes;
        self.pages[index] = None;
        self.page_count = index;
        Some(bytes)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.page_count == 0 && self.pages.is_empty()
    }

    fn retire_descriptor(&mut self,grant:semio_framework_value::RetainedCloneGrant)->Option<semio_framework_job::InteractiveJobCloseStep> {
        if self.len!=0||self.page_count!=0||self.pages.is_empty(){return None;}
        let released_bytes=size_of_val(self.pages.as_ref());
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_release_bytes<released_bytes{return Some(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});}
        self.pages=Box::new([]);
        Some(semio_framework_job::InteractiveJobCloseStep::Pending{progress:semio_framework_value::RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}})
    }

    fn retained_page_bytes(&self) -> Option<usize> {
        let index = self.page_count.checked_sub(1)?;
        self.pages[index].as_ref().map(|page| page.backing_bytes)
    }

    fn next_release_bytes(&self)->usize {
        if self.len!=0{return 0;}
        self.retained_page_bytes().unwrap_or_else(||size_of_val(self.pages.as_ref()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillShape {
    Circle,
    Rectangle,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillNodeSnapshot {
    id: BoardFillText,
    bounds: [f64; 4],
}

struct BoardFillNodeCapture {
    id: BoardFillText,
    bounds: Option<[f64; 4]>,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillHandleSnapshot {
    id: BoardFillText,
    node_kind: BoardFillText,
    handle_kind: BoardFillText,
    wire_kind: BoardFillText,
    edge_kind: BoardFillText,
    slot: [f64; 2],
    visible: bool,
    connected: bool,
    weight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillHandleCaptureStage {
    NodeKind,
    HandleKind,
    Slot,
    WireKind,
    EdgeKind,
    Visible,
    Connected,
    Weight,
    Publish,
}

struct BoardFillHandleCapture {
    handle: BoardFillHandleSnapshot,
    edge_key: Option<BoardFillText>,
    stage: BoardFillHandleCaptureStage,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillTemplateSnapshot {
    handle_kind: BoardFillText,
    wire_kind: BoardFillText,
    edge_kind: BoardFillText,
    angle: f64,
    radius: Option<f64>,
    weight: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillTemplateCaptureStage {
    HandleKind,
    WireKind,
    EdgeKind,
    Angle,
    Radius,
    Weight,
    Publish,
}

struct BoardFillTemplateCapture {
    value: BoardFillTemplateSnapshot,
    stage: BoardFillTemplateCaptureStage,
}

struct BoardFillKindSnapshot {
    id: BoardFillText,
    shape: BoardFillShape,
    radius: f64,
    width: f64,
    height: f64,
    icon: Option<BoardFillText>,
    weight: f64,
    handles: BoardFillFixedPages<BoardFillTemplateSnapshot, { BOARD_FILL_KIND_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillKindCaptureStage {
    Icon,
    Shape,
    Radius,
    Width,
    Height,
    Weight,
    Templates,
    Publish,
}

struct BoardFillKindCapture {
    value: BoardFillKindSnapshot,
    stage: BoardFillKindCaptureStage,
    template: Option<BoardFillTemplateCapture>,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillRuleSnapshot {
    source: BoardFillText,
    target: BoardFillText,
    bidirectional: bool,
    specificity: CompatSpecificity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillRuleCaptureStage {
    Target,
    Bidirectional,
    Specificity,
    Publish,
}

struct BoardFillRuleCapture {
    value: BoardFillRuleSnapshot,
    stage: BoardFillRuleCaptureStage,
}

/// 📸️ Fixed-page immutable input transferred after staged board capture.
pub struct BoardFillSnapshot {
    nodes: BoardFillFixedPages<BoardFillNodeSnapshot, { BOARD_FILL_NODE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    handles: BoardFillFixedPages<BoardFillHandleSnapshot, { BOARD_FILL_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    kinds: BoardFillFixedPages<BoardFillKindSnapshot, { BOARD_FILL_KIND_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    rules: BoardFillFixedPages<BoardFillRuleSnapshot, { BOARD_FILL_RULE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    suggestion_offset: f64,
}

impl Drop for BoardFillSnapshot {
    fn drop(&mut self) {
        assert!((self.nodes.terminal_is_empty() && self.handles.terminal_is_empty() && self.kinds.terminal_is_empty() && self.rules.terminal_is_empty()) || std::thread::panicking(), "Puzzle2d fill snapshot must transfer or close every exact page before Drop");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCaptureFault {
    TextCapacity,
    NodeCapacity,
    HandleCapacity,
    KindCapacity,
    KindHandleCapacity,
    RuleCapacity,
    StaleNode,
    StaleHandle,
    StaleKind,
    StaleRule,
    GenerationExhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillCapturePhase {
    Nodes,
    Handles,
    Kinds,
    Rules,
    Complete,
    Closing,
}

/// 📷️ Retained UI capture cursor; one host node, handle, kind scalar, template, or rule per call.
pub struct BoardFillSnapshotCapture {
    snapshot: Option<BoardFillSnapshot>,
    phase: BoardFillCapturePhase,
    last_key: Option<BoardFillText>,
    node: Option<BoardFillNodeCapture>,
    handle: Option<BoardFillHandleCapture>,
    kind: Option<BoardFillKindCapture>,
    kind_handle_cursor: usize,
    rule: Option<BoardFillRuleCapture>,
    rule_cursor: usize,
    closing: bool,
}

/// 📥️ Worker-owned fixed-page ingress for immutable artifact snapshot leases.
pub struct BoardFillSnapshotIngress {
    snapshot: Option<BoardFillSnapshot>,
    node: Option<BoardFillNodeSnapshot>,
    handle: Option<BoardFillHandleSnapshot>,
    kind: Option<BoardFillKindSnapshot>,
    template: Option<BoardFillTemplateSnapshot>,
    rule: Option<BoardFillRuleSnapshot>,
    closing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressHandleText {
    Id,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressKindText {
    Id,
    Icon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressTemplateText {
    HandleKind,
    WireKind,
    EdgeKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillIngressRuleText {
    Source,
    Target,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCaptureStep {
    Pending,
    Complete,
    Fault(BoardFillCaptureFault),
}

#[derive(Clone, Copy, Debug)]
struct BoardFillSource {
    id: BoardFillText,
    node_kind: BoardFillText,
    handle_kind: BoardFillText,
    wire_kind: BoardFillText,
    edge_kind: BoardFillText,
    slot: [f64; 2],
    weight: f64,
    owner: BoardFillSourceOwner,
    rejected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillSourceCaptureStage {
    Id,
    NodeKind,
    HandleKind,
    WireKind,
    EdgeKind,
    Slot,
    Weight,
    Publish,
}

struct BoardFillSourceCapture {
    value: BoardFillSource,
    stage: BoardFillSourceCaptureStage,
}

#[derive(Clone, Copy, Debug)]
enum BoardFillSourceOwner {
    Host(usize),
    Virtual(usize),
}

#[derive(Clone, Copy, Debug)]
struct BoardFillVirtualNode {
    id: BoardFillText,
    bounds: [f64; 4],
}

#[derive(Clone, Copy, Debug)]
struct BoardFillVirtualHandle {
    id: BoardFillText,
    node_kind: BoardFillText,
    handle_kind: BoardFillText,
    wire_kind: BoardFillText,
    edge_kind: BoardFillText,
    slot: [f64; 2],
    weight: f64,
    connected: bool,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillCandidate {
    kind_index: usize,
    target_handle_index: usize,
    weight: f64,
    rejected: bool,
}

#[derive(Clone, Copy, Debug)]
struct BoardFillCandidatePreview {
    source_id: BoardFillText,
    kind_index: usize,
    target_handle_index: usize,
    x: f64,
    y: f64,
    bounds: [f64; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillStage {
    ResetSources,
    PrepareSources,
    SelectTarget,
    ResetCandidates,
    PrepareCandidates,
    ScanCompatibility,
    SelectCandidate,
    ConstructPreview,
    ScanHostCollision,
    ScanVirtualCollision,
    AcceptCandidate,
    AcceptNodeId,
    AcceptEdgeId,
    AcceptEdgeKind,
    AcceptNodeKind,
    AcceptSourceHandle,
    AcceptTargetHandle,
    AcceptIcon,
    AcceptVirtualNode,
    AcceptSourceConnection,
    AcceptHandles,
    AcceptHandleId,
    AcceptHandleVirtual,
    AcceptHandlePublish,
    PublishPlanPrefix,
    Complete,
}

impl BoardFillStage {
    pub fn id(self) -> &'static str {
        match self {
            Self::ResetSources => "reset-sources",
            Self::PrepareSources => "prepare-sources",
            Self::SelectTarget => "select-target",
            Self::ResetCandidates => "reset-candidates",
            Self::PrepareCandidates => "prepare-candidates",
            Self::ScanCompatibility => "scan-compatibility",
            Self::SelectCandidate => "select-candidate",
            Self::ConstructPreview => "construct-preview",
            Self::ScanHostCollision => "scan-host-collision",
            Self::ScanVirtualCollision => "scan-virtual-collision",
            Self::AcceptCandidate => "accept-candidate",
            Self::AcceptNodeId => "accept-node-id",
            Self::AcceptEdgeId => "accept-edge-id",
            Self::AcceptEdgeKind => "accept-edge-kind",
            Self::AcceptNodeKind => "accept-node-kind",
            Self::AcceptSourceHandle => "accept-source-handle",
            Self::AcceptTargetHandle => "accept-target-handle",
            Self::AcceptIcon => "accept-icon",
            Self::AcceptVirtualNode => "accept-virtual-node",
            Self::AcceptSourceConnection => "accept-source-connection",
            Self::AcceptHandles => "accept-handles",
            Self::AcceptHandleId => "accept-handle-id",
            Self::AcceptHandleVirtual => "accept-handle-virtual",
            Self::AcceptHandlePublish => "accept-handle-publish",
            Self::PublishPlanPrefix => "publish-plan-prefix",
            Self::Complete => "complete",
        }
    }
}

struct BoardFillJobState {
    snapshot: BoardFillSnapshot,
    stage: BoardFillStage,
    max_count: u32,
    rng_state: u64,
    sources: BoardFillFixedPages<BoardFillSource, { BOARD_FILL_SOURCE_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    source_scan_cursor: usize,
    source_capture: Option<BoardFillSourceCapture>,
    target_selection_cursor: usize,
    target_best: Option<(f64, usize)>,
    current_target: Option<usize>,
    candidates: BoardFillFixedPages<BoardFillCandidate, { BOARD_FILL_CANDIDATE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
    kind_cursor: usize,
    template_cursor: usize,
    compatibility_candidate: Option<BoardFillCandidate>,
    compatibility_cursor: usize,
    compatibility_matched: bool,
    candidate_selection_cursor: usize,
    candidate_best: Option<(f64, usize)>,
    current_candidate: Option<usize>,
    current_preview: Option<BoardFillCandidatePreview>,
    host_collision_cursor: usize,
    virtual_collision_cursor: usize,
    virtual_nodes: BoardFillFixedPages<BoardFillVirtualNode, { BOARD_FILL_PLACEMENT_CAPACITY.div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    virtual_handles: BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY).div_ceil(BOARD_FILL_PAGE_ITEMS) }>,
    pending_placement: Option<BoardFillPlacement>,
    accept_pending_virtual_node: Option<BoardFillVirtualNode>,
    accept_handle_template: Option<BoardFillTemplateSnapshot>,
    accept_handle_id: Option<BoardFillText>,
    accept_handle_pending_virtual: Option<BoardFillVirtualHandle>,
    accept_handle_pending_placement: Option<BoardFillPlacementHandle>,
    accept_handle_cursor: usize,
    accepted_count: u32,
    next_serial: u64,
    stalled: bool,
    rejection: Option<BoardFillText>,
    search_count: u64,
    preview_sequence: u64,
    candidate_event: Option<BoardFillCandidateEvent>,
}

/// 📡️ Latest bounded fill search projection, separate from authoritative placements.
#[derive(Clone, Copy, Debug)]
pub struct BoardFillPreview {
    pub sequence: u64,
    pub generation: u64,
    pub stage: BoardFillStage,
    pub accepted_count: u32,
    pub target_handle_id: Option<BoardFillText>,
    pub candidate_node_kind_id: Option<BoardFillText>,
    pub host_collision_cursor: usize,
    pub virtual_collision_cursor: usize,
    pub tested_collision_id: Option<BoardFillText>,
    pub rejection: Option<BoardFillText>,
    pub search_count: u64,
}

/// ⚖️ What one fill search transition decided about one candidate placement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCandidateVerdict {
    Testing,
    HostCollision,
    VirtualCollision,
    PortIncompatible,
    RuleIncompatible,
    Fits,
}

/// 🔎️ One candidate verdict with the candidate's kind index (capture order), slot center and axis-aligned
/// collision footprint `[min_x, min_y, max_x, max_y]`, taken once per transition by
/// [`BoardFillJob::take_candidate_event`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCandidateEvent {
    pub verdict: BoardFillCandidateVerdict,
    pub kind_index: usize,
    pub position: [f64; 2],
    pub bounds: [f64; 4],
}

#[derive(Clone, Copy, Debug)]
struct BoardFillPlacementHandle {
    id: BoardFillText,
    template: BoardFillTemplateSnapshot,
}

/// 🧩️ Typed placement transferred to the Puzzle2d document applier without a JSON callback.
pub struct BoardFillPlacement {
    pub node_id: BoardFillText,
    pub edge_id: BoardFillText,
    pub edge_kind: BoardFillText,
    pub node_kind: BoardFillText,
    pub source_handle_id: BoardFillText,
    pub target_handle_id: BoardFillText,
    pub target_handle_index: usize,
    pub x: f64,
    pub y: f64,
    pub shape: &'static str,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<BoardFillText>,
    handles: BoardFillFixedPages<BoardFillPlacementHandle, { BOARD_FILL_KIND_HANDLE_CAPACITY / BOARD_FILL_PAGE_ITEMS }>,
}

impl BoardFillPlacement {
    pub fn handle_count(&self) -> usize {
        self.handles.len
    }

    pub fn handle(&self, index: usize) -> Option<(&str, &str, f64, Option<f64>)> {
        let handle = self.handles.get(index)?;
        Some((handle.id.as_str(), handle.template.handle_kind.as_str(), handle.template.angle, handle.template.radius))
    }

    pub fn fixed_handle_id(&self, index: usize) -> Option<&BoardFillText> {
        self.handles.get(index).map(|handle| &handle.id)
    }

    pub fn fixed_handle_kind(&self, index: usize) -> Option<&BoardFillText> {
        self.handles.get(index).map(|handle| &handle.template.handle_kind)
    }

    pub fn fixed_handle_angle(&self, index: usize) -> Option<f64> {
        self.handles.get(index).map(|handle| handle.template.angle)
    }

    pub fn fixed_handle_radius(&self, index: usize) -> Option<Option<f64>> {
        self.handles.get(index).map(|handle| handle.template.radius)
    }

    pub fn next_close_release_byte_demand(&self)->usize {self.handles.next_release_bytes()}

    pub fn close_step(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep;
        use semio_framework_value::RetainedCloneProgress;
        if self.terminal_is_empty(){return InteractiveJobCloseStep::Complete{progress:Default::default()};}
        if grant.maximum_items==0||grant.maximum_depth==0{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
        if self.handles.pop().is_some(){return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}};}
        if let Some(bytes)=self.handles.retained_page_bytes(){
            if bytes>grant.maximum_release_bytes{return InteractiveJobCloseStep::Pending{progress:Default::default()};}
            let Some(released_bytes)=self.handles.retire_empty_page()else{return InteractiveJobCloseStep::Pending{progress:Default::default()};};
            return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}};
        }
        self.handles.retire_descriptor(grant).unwrap_or(InteractiveJobCloseStep::Complete{progress:Default::default()})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.handles.terminal_is_empty()
    }
}

impl Drop for BoardFillPlacement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "Puzzle2d fill placement must reach exact terminal-empty before Drop");
    }
}

/// 💾 Exact typed checkpoint root moved out of the job until the mounted caller adopts it.
pub struct BoardFillCheckpoint {
    operation: semio_framework_job::Operation,
    state: Option<BoardFillJobState>,
}

impl BoardFillCheckpoint {
    pub fn operation(&self) -> semio_framework_job::Operation {
        self.operation
    }

    pub fn take_pending_placement(&mut self) -> Option<BoardFillPlacement> {
        self.state.as_mut()?.pending_placement.take()
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn put_pending_placement(&mut self, placement: BoardFillPlacement) -> Result<(), BoardFillPlacement> {
        let Some(state) = self.state.as_mut() else { return Err(placement) };
        if state.pending_placement.is_some() {
            return Err(placement);
        }
        state.pending_placement = Some(placement);
        Ok(())
    }

    pub fn pending_placement(&self) -> Option<&BoardFillPlacement> {
        self.state.as_ref()?.pending_placement.as_ref()
    }

    pub fn accepted_count(&self) -> u32 {
        self.state.as_ref().map_or(0, |state| state.accepted_count)
    }

    pub fn into_closing_job(mut self) -> BoardFillJob {
        BoardFillJob { operation: self.operation, state: self.state.take(), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, lent: None, commit_output: None, offer: Cell::new(None), admitted: Cell::new(false), fault: None, closing: true }
    }
}

impl Drop for BoardFillCheckpoint {
    fn drop(&mut self) {
        assert!(self.state.is_none() || std::thread::panicking(), "Puzzle2d fill checkpoint must transfer or close its exact state before Drop");
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillResult {
    pub accepted_count: u32,
    pub stalled: bool,
    pub search_count: u64,
}

/// 🧩️ Fixed handle carried by the exact terminal fill candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitHandle {
    pub id: BoardFillText,
    pub handle_kind: BoardFillText,
    pub angle: f64,
    pub radius: Option<f64>,
}

/// 🔷️ Fixed shape carried by the exact terminal fill candidate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardFillCommitShape {
    Circle,
    Rectangle,
}

/// 📦️ Full bounded typed placement carried by the exact terminal candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitPlacement {
    pub node_id: BoardFillText,
    pub edge_id: BoardFillText,
    pub edge_kind: BoardFillText,
    pub node_kind: BoardFillText,
    pub source_handle_id: BoardFillText,
    pub target_handle_id: BoardFillText,
    pub target_handle_index: usize,
    pub x: f64,
    pub y: f64,
    pub shape: BoardFillCommitShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub icon_kind: Option<BoardFillText>,
    pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY],
    pub handle_count: usize,
}

/// 🎯️ Typed terminal projection decoded while the retained payload remains authoritative.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoardFillCommitCandidate {
    pub result: BoardFillResult,
    pub placement: Option<BoardFillCommitPlacement>,
}

const BOARD_FILL_COMMIT_MAGIC: [u8; 4] = *b"P7BF";
const BOARD_FILL_COMMIT_VERSION: u8 = 1;
const BOARD_FILL_COMMIT_HEADER_BYTES: usize = 20;
const BOARD_FILL_COMMIT_TEXT_SLOT_BYTES: usize = 2 + BOARD_FILL_TEXT_BYTES;
const BOARD_FILL_COMMIT_NODE_ID_OFFSET: usize = BOARD_FILL_COMMIT_HEADER_BYTES;
const BOARD_FILL_COMMIT_EDGE_ID_OFFSET: usize = BOARD_FILL_COMMIT_NODE_ID_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_EDGE_KIND_OFFSET: usize = BOARD_FILL_COMMIT_EDGE_ID_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_NODE_KIND_OFFSET: usize = BOARD_FILL_COMMIT_EDGE_KIND_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_SOURCE_OFFSET: usize = BOARD_FILL_COMMIT_NODE_KIND_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_TARGET_OFFSET: usize = BOARD_FILL_COMMIT_SOURCE_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET: usize = BOARD_FILL_COMMIT_TARGET_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_X_OFFSET: usize = BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET + 8;
const BOARD_FILL_COMMIT_Y_OFFSET: usize = BOARD_FILL_COMMIT_X_OFFSET + 8;
const BOARD_FILL_COMMIT_SHAPE_OFFSET: usize = BOARD_FILL_COMMIT_Y_OFFSET + 8;
const BOARD_FILL_COMMIT_RADIUS_OFFSET: usize = BOARD_FILL_COMMIT_SHAPE_OFFSET + 1;
const BOARD_FILL_COMMIT_WIDTH_OFFSET: usize = BOARD_FILL_COMMIT_RADIUS_OFFSET + 8;
const BOARD_FILL_COMMIT_HEIGHT_OFFSET: usize = BOARD_FILL_COMMIT_WIDTH_OFFSET + 8;
const BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET: usize = BOARD_FILL_COMMIT_HEIGHT_OFFSET + 8;
const BOARD_FILL_COMMIT_ICON_OFFSET: usize = BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET + 1;
const BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET: usize = BOARD_FILL_COMMIT_ICON_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
const BOARD_FILL_COMMIT_HANDLE_OFFSET: usize = BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET + 2;
const BOARD_FILL_COMMIT_HANDLE_BYTES: usize = BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8 + 1 + 8;
const BOARD_FILL_COMMIT_BYTES: usize = BOARD_FILL_COMMIT_HANDLE_OFFSET + BOARD_FILL_KIND_HANDLE_CAPACITY * BOARD_FILL_COMMIT_HANDLE_BYTES;
const BOARD_FILL_COMMIT_PAGE_COUNT: usize = BOARD_FILL_COMMIT_BYTES.div_ceil(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillCommitEncodeStage {
    Header,
    Version,
    Presence,
    Stalled,
    Accepted,
    Search,
    NodeId,
    EdgeId,
    EdgeKind,
    NodeKind,
    Source,
    Target,
    TargetIndex,
    X,
    Y,
    Shape,
    Radius,
    Width,
    Height,
    Icon,
    IconText,
    HandleCount,
    HandleId,
    HandleKind,
    HandleAngle,
    HandleRadius,
    HandleRadiusValue,
    Complete,
}

struct BoardFillCommitEncoder {
    bytes: [u8; BOARD_FILL_COMMIT_BYTES],
    stage: BoardFillCommitEncodeStage,
    handle: usize,
    text_byte: usize,
    text_length_written: bool,
    output_cursor: usize,
}

impl BoardFillCommitEncoder {
    fn new() -> Self {
        Self { bytes: [0; BOARD_FILL_COMMIT_BYTES], stage: BoardFillCommitEncodeStage::Header, handle: 0, text_byte: 0, text_length_written: false, output_cursor: 0 }
    }

    fn step_text(&mut self, offset: usize, text: &BoardFillText) -> bool {
        if !self.text_length_written {
            self.bytes[offset..offset + 2].copy_from_slice(&text.len.to_le_bytes());
            self.text_length_written = true;
            return false;
        }
        if self.text_byte < usize::from(text.len) {
            self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte];
            self.text_byte += 1;
            if self.text_byte < usize::from(text.len) {
                return false;
            }
        }
        self.text_byte = 0;
        self.text_length_written = false;
        true
    }

    fn step(&mut self, state: &BoardFillJobState) -> Result<bool, &'static str> {
        let placement = state.pending_placement.as_ref();
        match self.stage {
            BoardFillCommitEncodeStage::Header => {
                self.bytes[..4].copy_from_slice(&BOARD_FILL_COMMIT_MAGIC);
                self.stage = BoardFillCommitEncodeStage::Version;
            }
            BoardFillCommitEncodeStage::Version => {
                self.bytes[4] = BOARD_FILL_COMMIT_VERSION;
                self.stage = BoardFillCommitEncodeStage::Presence;
            }
            BoardFillCommitEncodeStage::Presence => {
                self.bytes[5] = u8::from(placement.is_some());
                self.stage = BoardFillCommitEncodeStage::Stalled;
            }
            BoardFillCommitEncodeStage::Stalled => {
                self.bytes[6] = u8::from(state.stalled);
                self.stage = BoardFillCommitEncodeStage::Accepted;
            }
            BoardFillCommitEncodeStage::Accepted => {
                self.bytes[8..12].copy_from_slice(&state.accepted_count.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Search;
            }
            BoardFillCommitEncodeStage::Search => {
                self.bytes[12..20].copy_from_slice(&state.search_count.to_le_bytes());
                self.stage = if placement.is_some() { BoardFillCommitEncodeStage::NodeId } else { BoardFillCommitEncodeStage::Complete };
            }
            BoardFillCommitEncodeStage::NodeId => {
                if self.step_text(BOARD_FILL_COMMIT_NODE_ID_OFFSET, &placement.ok_or("fill-commit-placement")?.node_id) {
                    self.stage = BoardFillCommitEncodeStage::EdgeId;
                }
            }
            BoardFillCommitEncodeStage::EdgeId => {
                if self.step_text(BOARD_FILL_COMMIT_EDGE_ID_OFFSET, &placement.ok_or("fill-commit-placement")?.edge_id) {
                    self.stage = BoardFillCommitEncodeStage::EdgeKind;
                }
            }
            BoardFillCommitEncodeStage::EdgeKind => {
                if self.step_text(BOARD_FILL_COMMIT_EDGE_KIND_OFFSET, &placement.ok_or("fill-commit-placement")?.edge_kind) {
                    self.stage = BoardFillCommitEncodeStage::NodeKind;
                }
            }
            BoardFillCommitEncodeStage::NodeKind => {
                if self.step_text(BOARD_FILL_COMMIT_NODE_KIND_OFFSET, &placement.ok_or("fill-commit-placement")?.node_kind) {
                    self.stage = BoardFillCommitEncodeStage::Source;
                }
            }
            BoardFillCommitEncodeStage::Source => {
                if self.step_text(BOARD_FILL_COMMIT_SOURCE_OFFSET, &placement.ok_or("fill-commit-placement")?.source_handle_id) {
                    self.stage = BoardFillCommitEncodeStage::Target;
                }
            }
            BoardFillCommitEncodeStage::Target => {
                if self.step_text(BOARD_FILL_COMMIT_TARGET_OFFSET, &placement.ok_or("fill-commit-placement")?.target_handle_id) {
                    self.stage = BoardFillCommitEncodeStage::TargetIndex;
                }
            }
            BoardFillCommitEncodeStage::TargetIndex => {
                let index = u64::try_from(placement.ok_or("fill-commit-placement")?.target_handle_index).map_err(|_| "fill-commit-target-index")?;
                self.bytes[BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET..BOARD_FILL_COMMIT_X_OFFSET].copy_from_slice(&index.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::X;
            }
            BoardFillCommitEncodeStage::X => {
                self.bytes[BOARD_FILL_COMMIT_X_OFFSET..BOARD_FILL_COMMIT_Y_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.x.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Y;
            }
            BoardFillCommitEncodeStage::Y => {
                self.bytes[BOARD_FILL_COMMIT_Y_OFFSET..BOARD_FILL_COMMIT_SHAPE_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.y.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Shape;
            }
            BoardFillCommitEncodeStage::Shape => {
                self.bytes[BOARD_FILL_COMMIT_SHAPE_OFFSET] = match placement.ok_or("fill-commit-placement")?.shape {
                    "circle" => 0,
                    "rectangle" => 1,
                    _ => return Err("fill-commit-shape"),
                };
                self.stage = BoardFillCommitEncodeStage::Radius;
            }
            BoardFillCommitEncodeStage::Radius => {
                self.bytes[BOARD_FILL_COMMIT_RADIUS_OFFSET..BOARD_FILL_COMMIT_WIDTH_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.radius.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Width;
            }
            BoardFillCommitEncodeStage::Width => {
                self.bytes[BOARD_FILL_COMMIT_WIDTH_OFFSET..BOARD_FILL_COMMIT_HEIGHT_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.width.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Height;
            }
            BoardFillCommitEncodeStage::Height => {
                self.bytes[BOARD_FILL_COMMIT_HEIGHT_OFFSET..BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.height.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Icon;
            }
            BoardFillCommitEncodeStage::Icon => {
                if placement.ok_or("fill-commit-placement")?.icon_kind.as_ref().is_some() {
                    self.bytes[BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET] = 1;
                    self.stage = BoardFillCommitEncodeStage::IconText;
                } else {
                    self.stage = BoardFillCommitEncodeStage::HandleCount;
                }
            }
            BoardFillCommitEncodeStage::IconText => {
                if self.step_text(BOARD_FILL_COMMIT_ICON_OFFSET, placement.ok_or("fill-commit-placement")?.icon_kind.as_ref().ok_or("fill-commit-icon")?) {
                    self.stage = BoardFillCommitEncodeStage::HandleCount;
                }
            }
            BoardFillCommitEncodeStage::HandleCount => {
                let count = placement.ok_or("fill-commit-placement")?.handle_count();
                let count = u16::try_from(count).map_err(|_| "fill-commit-handle-count")?;
                self.bytes[BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET..BOARD_FILL_COMMIT_HANDLE_OFFSET].copy_from_slice(&count.to_le_bytes());
                self.handle = 0;
                self.stage = BoardFillCommitEncodeStage::HandleId;
            }
            BoardFillCommitEncodeStage::HandleId => {
                let placement = placement.ok_or("fill-commit-placement")?;
                let Some(handle) = placement.handles.get(self.handle) else {
                    self.stage = BoardFillCommitEncodeStage::Complete;
                    return Ok(false);
                };
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES;
                if self.step_text(offset, &handle.id) {
                    self.stage = BoardFillCommitEncodeStage::HandleKind;
                }
            }
            BoardFillCommitEncodeStage::HandleKind => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
                if self.step_text(offset, &handle.template.handle_kind) {
                    self.stage = BoardFillCommitEncodeStage::HandleAngle;
                }
            }
            BoardFillCommitEncodeStage::HandleAngle => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2;
                self.bytes[offset..offset + 8].copy_from_slice(&handle.template.angle.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::HandleRadius;
            }
            BoardFillCommitEncodeStage::HandleRadius => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8;
                if let Some(radius) = handle.template.radius {
                    self.bytes[offset] = 1;
                    let _ = radius;
                    self.stage = BoardFillCommitEncodeStage::HandleRadiusValue;
                } else {
                    self.handle += 1;
                    self.stage = BoardFillCommitEncodeStage::HandleId;
                }
            }
            BoardFillCommitEncodeStage::HandleRadiusValue => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 9;
                self.bytes[offset..offset + 8].copy_from_slice(&handle.template.radius.ok_or("fill-commit-handle-radius")?.to_le_bytes());
                self.handle += 1;
                self.stage = BoardFillCommitEncodeStage::HandleId;
            }
            BoardFillCommitEncodeStage::Complete => return Ok(true),
        }
        Ok(false)
    }
}

struct BoardFillCommitPayload<'a> {
    payload: &'a semio_framework_job::RetainedJobPayload,
}

impl<'a> BoardFillCommitPayload<'a> {
    fn new(payload: &'a semio_framework_job::RetainedJobPayload) -> Option<Self> {
        (payload.len() == BOARD_FILL_COMMIT_BYTES && payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT && payload.page(0)?.len() == BOARD_FILL_COMMIT_BYTES).then_some(Self { payload })
    }

    fn read<const N: usize>(&self, offset: usize) -> Option<[u8; N]> {
        let end = offset.checked_add(N)?;
        if end > BOARD_FILL_COMMIT_BYTES {
            return None;
        }
        let mut output = [0; N];
        let mut copied = 0usize;
        while copied < N {
            let absolute = offset + copied;
            let page_index = absolute / semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
            let page_offset = absolute % semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
            let page = self.payload.page(page_index)?;
            let count = (N - copied).min(page.len().checked_sub(page_offset)?);
            if count == 0 {
                return None;
            }
            output[copied..copied + count].copy_from_slice(&page[page_offset..page_offset + count]);
            copied += count;
        }
        Some(output)
    }

    fn byte(&self, offset: usize) -> Option<u8> {
        Some(self.read::<1>(offset)?[0])
    }
}

impl BoardFillCommitCandidate {
    fn read_text(payload: &BoardFillCommitPayload<'_>, offset: usize) -> Option<BoardFillText> {
        let length = u16::from_le_bytes(payload.read::<2>(offset)?);
        if usize::from(length) > BOARD_FILL_TEXT_BYTES {
            return None;
        }
        let mut text = BoardFillText::empty();
        text.bytes = payload.read::<BOARD_FILL_TEXT_BYTES>(offset + 2)?;
        text.len = length;
        std::str::from_utf8(&text.bytes[..usize::from(length)]).ok()?;
        Some(text)
    }

    fn read_f64(payload: &BoardFillCommitPayload<'_>, offset: usize) -> Option<f64> {
        let value = f64::from_le_bytes(payload.read::<8>(offset)?);
        value.is_finite().then_some(value)
    }

    pub fn from_complete(state: Option<&semio_framework_job::RetainedJobPayload>, output: Option<&semio_framework_job::RetainedJobPayload>) -> Option<Self> {
        let payload = BoardFillCommitPayload::new(output?)?;
        if state.is_some_and(|state| !state.is_empty()) || payload.read::<4>(0)? != BOARD_FILL_COMMIT_MAGIC || payload.byte(4)? != BOARD_FILL_COMMIT_VERSION {
            return None;
        }
        let result = BoardFillResult {
            accepted_count: u32::from_le_bytes(payload.read::<4>(8)?),
            stalled: match payload.byte(6)? {
                0 => false,
                1 => true,
                _ => return None,
            },
            search_count: u64::from_le_bytes(payload.read::<8>(12)?),
        };
        let placement = match payload.byte(5)? {
            0 => None,
            1 => {
                let handle_count = usize::from(u16::from_le_bytes(payload.read::<2>(BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET)?));
                if handle_count > BOARD_FILL_KIND_HANDLE_CAPACITY {
                    return None;
                }
                let handles = std::array::from_fn(|index| {
                    if index >= handle_count {
                        return None;
                    }
                    let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + index * BOARD_FILL_COMMIT_HANDLE_BYTES;
                    let radius_offset = offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8;
                    let radius = match payload.byte(radius_offset)? {
                        0 => None,
                        1 => Some(Self::read_f64(&payload, radius_offset + 1)?),
                        _ => return None,
                    };
                    Some(BoardFillCommitHandle {
                        id: Self::read_text(&payload, offset)?,
                        handle_kind: Self::read_text(&payload, offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES)?,
                        angle: Self::read_f64(&payload, offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2)?,
                        radius,
                    })
                });
                if handles[..handle_count].iter().any(Option::is_none) {
                    return None;
                }
                let target_handle_index = usize::try_from(u64::from_le_bytes(payload.read::<8>(BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET)?)).ok()?;
                if target_handle_index >= handle_count {
                    return None;
                }
                let shape = match payload.byte(BOARD_FILL_COMMIT_SHAPE_OFFSET)? {
                    0 => BoardFillCommitShape::Circle,
                    1 => BoardFillCommitShape::Rectangle,
                    _ => return None,
                };
                let icon_kind = match payload.byte(BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET)? {
                    0 => None,
                    1 => Some(Self::read_text(&payload, BOARD_FILL_COMMIT_ICON_OFFSET)?),
                    _ => return None,
                };
                Some(BoardFillCommitPlacement {
                    node_id: Self::read_text(&payload, BOARD_FILL_COMMIT_NODE_ID_OFFSET)?,
                    edge_id: Self::read_text(&payload, BOARD_FILL_COMMIT_EDGE_ID_OFFSET)?,
                    edge_kind: Self::read_text(&payload, BOARD_FILL_COMMIT_EDGE_KIND_OFFSET)?,
                    node_kind: Self::read_text(&payload, BOARD_FILL_COMMIT_NODE_KIND_OFFSET)?,
                    source_handle_id: Self::read_text(&payload, BOARD_FILL_COMMIT_SOURCE_OFFSET)?,
                    target_handle_id: Self::read_text(&payload, BOARD_FILL_COMMIT_TARGET_OFFSET)?,
                    target_handle_index,
                    x: Self::read_f64(&payload, BOARD_FILL_COMMIT_X_OFFSET)?,
                    y: Self::read_f64(&payload, BOARD_FILL_COMMIT_Y_OFFSET)?,
                    shape,
                    radius: Self::read_f64(&payload, BOARD_FILL_COMMIT_RADIUS_OFFSET)?,
                    width: Self::read_f64(&payload, BOARD_FILL_COMMIT_WIDTH_OFFSET)?,
                    height: Self::read_f64(&payload, BOARD_FILL_COMMIT_HEIGHT_OFFSET)?,
                    icon_kind,
                    handles,
                    handle_count,
                })
            }
            _ => return None,
        };
        Some(Self { result, placement })
    }
}

/// 🧵️ Persistent worker-owned fill search over a Send board snapshot.
pub struct BoardFillJob {
    operation: semio_framework_job::Operation,
    state: Option<BoardFillJobState>,
    checkpoint: Option<BoardFillCheckpoint>,
    preview: Option<BoardFillPreview>,
    commit_encoder: Option<BoardFillCommitEncoder>,
    commit_writer: Option<semio_framework_job::RetainedJobPayloadWriter>,
    lent: Option<semio_framework_job::RetainedJobPayload>,
    commit_output: Option<semio_framework_job::RetainedJobPayload>,
    offer: Cell<Option<BoardFillOffer>>,
    admitted: Cell<bool>,
    fault: Option<&'static str>,
    closing: bool,
}

/// 🎟️ The one outcome a fill turn owes its driver; it stays offered until admission succeeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoardFillOffer { Yield, Preview, Checkpoint(u64), Complete, Fault, Cancelled }

/// 🔄️ One live rotate-ring gesture. The node centres and handle angles captured at grab time are
/// the base every frame re-derives from, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture geometry.
#[derive(Clone, Debug)]
struct BoardTransformDrag {
    gesture: GestureStage,
    pivot: Point,
    grab: Point,
    radius_world: f64,
    radians: f64,
    start_positions: BTreeMap<String, (f64, f64)>,
    start_handle_angles: BTreeMap<String, f64>,
}

/// 🚚️ A live target-region gesture: which region, which grip, and the bounds it was grabbed at.
/// Every frame re-derives from `start_bounds`, so the preview never accumulates float drift and a
/// cancel restores the exact pre-gesture rectangle.
#[derive(Clone, Debug)]
struct BoardRegionDrag {
    gesture: GestureStage,
    id: String,
    grip: RegionGrip,
    grab: Point,
    start_bounds: [f64; 4],
    bounds: [f64; 4],
    delta: Vec2,
}

/// 🖍️ A live area-brush paint: the grid-snapped anchor and the corner the pointer is at. A release
/// that never left the anchor commits the configured brush extent instead of a zero-area sliver.
#[derive(Clone, Debug)]
struct BoardRegionPaint {
    anchor: Point,
    corner: Point,
    dragged: bool,
}

#[derive(Clone, Debug)]
struct DropPreviewSnapshot {
    node_kind_id: String,
    x: f64,
    y: f64,
    shape: NodeShape,
    radius: f64,
    width: f64,
    height: f64,
    icon_kind: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct BoardPickTargetJson {
    domain: String,
    id: String,
    generality: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

pub const BOARD_EVENT_ITEM_CAPACITY: usize = 256;
pub const BOARD_EVENT_BYTE_CAPACITY: usize = 256 * 1024;
pub const BOARD_EVENT_PAYLOAD_BYTE_CAPACITY: usize = 16 * 1024;
pub const BOARD_EVENT_KEY_BYTE_CAPACITY: usize = 256;

/// 🏷️ Every row kind the board engine publishes. `NodeMove` and `TransformPreview` are live-preview
/// frames hosts mirror into sibling panes and never dispatch; a finished pointer gesture is ONE `Gesture`
/// record (drag, rotate), riding one batch with the `select` row its press staged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardEventKind {
    Camera,
    NodeMove,
    TransformPreview,
    Gesture,
    Select,
    Preselect,
    PreselectCancel,
    Hover,
    BrushPreview,
    BrushCandidates,
    BrushPlace,
    LinkCompatibleNodes,
    LinkTargetRing,
    EdgeCreate,
    EdgeDelete,
    NodeDelete,
    IndirectConnect,
    ProximityConnect,
    RegionCreate,
    RegionResize,
}

impl BoardEventKind {
    pub const ALL: [Self; 20] = [
        Self::Camera,
        Self::NodeMove,
        Self::TransformPreview,
        Self::Gesture,
        Self::Select,
        Self::Preselect,
        Self::PreselectCancel,
        Self::Hover,
        Self::BrushPreview,
        Self::BrushCandidates,
        Self::BrushPlace,
        Self::LinkCompatibleNodes,
        Self::LinkTargetRing,
        Self::EdgeCreate,
        Self::EdgeDelete,
        Self::NodeDelete,
        Self::IndirectConnect,
        Self::ProximityConnect,
        Self::RegionCreate,
        Self::RegionResize,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::NodeMove => "nodeMove",
            Self::TransformPreview => "transformPreview",
            Self::Gesture => "gesture",
            Self::Select => "select",
            Self::Preselect => "preselect",
            Self::PreselectCancel => "preselectCancel",
            Self::Hover => "hover",
            Self::BrushPreview => "brushPreview",
            Self::BrushCandidates => "brushCandidates",
            Self::BrushPlace => "brushPlace",
            Self::LinkCompatibleNodes => "linkCompatibleNodes",
            Self::LinkTargetRing => "linkTargetRing",
            Self::EdgeCreate => "edgeCreate",
            Self::EdgeDelete => "edgeDelete",
            Self::NodeDelete => "nodeDelete",
            Self::IndirectConnect => "indirectConnect",
            Self::ProximityConnect => "proximityConnect",
            Self::RegionCreate => "regionCreate",
            Self::RegionResize => "regionResize",
        }
    }

    /// 🔎️ The kind a row `name` spells, `None` for a name the engine never publishes.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == name)
    }
}

/// ✂️ A drag offset measured from pointer input, as the shortest decimal of its f32 value. Pointer coordinates
/// arrive with f32 precision, so every f64 digit below it is representation noise — an 80 px drag reading
/// `80.00003051757813` — that must never become a stored mutation input. A rotation keeps its f64 angle: a snapped
/// angle is an exact multiple whose narrowing would move every orbit off its exact pose.
pub fn board_pointer_offset(value: f64) -> f64 {
    let narrowed = value as f32;
    if !narrowed.is_finite() {
        return value;
    }
    narrowed.to_string().parse().unwrap_or(value)
}

/// 🎬️ What one released pointer gesture did, in its record's own terms: a drag by one offset, or a
/// rotation by `angle` radians (counter-clockwise) about `pivot`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardGestureMotion {
    Drag { dx: f64, dy: f64 },
    Rotate { pivot: Point, angle: f64 },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardEventFault {
    ItemCredits,
    ByteCredits,
    KeyCredits,
    Schema,
}

#[derive(Debug)]
pub struct BoardOwnedEvent {
    kind: BoardEventKind,
    payload: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    payload_len: u16,
    key: [u8; BOARD_EVENT_KEY_BYTE_CAPACITY],
    key_len: u16,
}

struct BoardPayloadBuilder {
    bytes: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    len: u16,
}

impl BoardPayloadBuilder {
    fn new() -> Self {
        Self { bytes: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), len: 0 }
    }

    fn raw(&mut self, value: &str) -> Result<(), BoardEventFault> {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }

    fn string(&mut self, value: &str) -> Result<(), BoardEventFault> {
        self.raw("\"")?;
        for character in value.chars() {
            match character {
                '"' => self.raw("\\\"")?,
                '\\' => self.raw("\\\\")?,
                '\n' => self.raw("\\n")?,
                '\r' => self.raw("\\r")?,
                '\t' => self.raw("\\t")?,
                character if character <= '\u{1f}' => {
                    let escaped = format!("\\u{:04x}", character as u32);
                    self.raw(&escaped)?;
                }
                character => {
                    let mut encoded = [0; 4];
                    self.raw(character.encode_utf8(&mut encoded))?;
                }
            }
        }
        self.raw("\"")
    }

    fn string_array<'a>(&mut self, values: impl IntoIterator<Item = &'a str>) -> Result<(), BoardEventFault> {
        self.raw("[")?;
        let mut first = true;
        for value in values {
            if !first {
                self.raw(",")?;
            }
            first = false;
            self.string(value)?;
        }
        self.raw("]")
    }

    fn number(&mut self, value: f64) -> Result<(), BoardEventFault> {
        if !value.is_finite() {
            return Err(BoardEventFault::Schema);
        }
        self.raw(&value.to_string())
    }

    fn usize(&mut self, value: usize) -> Result<(), BoardEventFault> {
        self.raw(&value.to_string())
    }

    fn boolean(&mut self, value: bool) -> Result<(), BoardEventFault> {
        self.raw(if value { "true" } else { "false" })
    }

    fn finish(self, kind: BoardEventKind, key: Option<&str>) -> Result<BoardOwnedEvent, BoardEventFault> {
        let payload = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).map_err(|_| BoardEventFault::Schema)?;
        BoardOwnedEvent::from_payload(kind, payload, key)
    }

    fn finish_owned(self, kind: BoardEventKind) -> BoardOwnedEvent {
        BoardOwnedEvent { kind, payload: self.bytes, payload_len: self.len, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: 0 }
    }
}

pub struct BoardEventReservation {
    expected_len: u16,
    expected_bytes: usize,
    event: Option<BoardOwnedEvent>,
}

const BOARD_EVENT_BATCH_CAPACITY: usize = 4;

pub struct BoardEventBatchReservation {
    expected_len: u16,
    expected_bytes: usize,
    events: [Option<BoardOwnedEvent>; BOARD_EVENT_BATCH_CAPACITY],
    len: u8,
    cursor: u8,
}

impl BoardEventBatchReservation {
    fn one(event: BoardOwnedEvent) -> Self {
        let mut events = std::array::from_fn(|_| None);
        events[0] = Some(event);
        Self { expected_len: 0, expected_bytes: 0, events, len: 1, cursor: 0 }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let index = usize::from(self.len);
        if index == BOARD_EVENT_BATCH_CAPACITY {
            return Err(event);
        }
        self.events[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    fn owned_bytes(&self) -> Option<usize> {
        self.events[usize::from(self.cursor)..usize::from(self.len)].iter().try_fold(0usize, |bytes, event| bytes.checked_add(event.as_ref()?.owned_bytes()))
    }

    fn peek(&self) -> Option<&BoardOwnedEvent> {
        self.events.get(usize::from(self.cursor))?.as_ref()
    }

    fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.cursor == self.len {
            return None;
        }
        let event = self.events[usize::from(self.cursor)].take();
        self.cursor += 1;
        event
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.len
    }
}

impl BoardOwnedEvent {
    pub fn from_payload(kind: BoardEventKind, payload: &str, key: Option<&str>) -> Result<Self, BoardEventFault> {
        if payload.len() > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        let key = key.unwrap_or_default();
        if key.len() > BOARD_EVENT_KEY_BYTE_CAPACITY {
            return Err(BoardEventFault::KeyCredits);
        }
        let mut owned = Self { kind, payload: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), payload_len: payload.len() as u16, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: key.len() as u16 };
        owned.payload[..payload.len()].copy_from_slice(payload.as_bytes());
        owned.key[..key.len()].copy_from_slice(key.as_bytes());
        Ok(owned)
    }

    pub fn kind(&self) -> BoardEventKind {
        self.kind
    }

    pub fn payload_json(&self) -> &str {
        std::str::from_utf8(&self.payload[..usize::from(self.payload_len)]).expect("board event payload is UTF-8")
    }

    pub fn key(&self) -> Option<&str> {
        (self.key_len > 0).then(|| std::str::from_utf8(&self.key[..usize::from(self.key_len)]).expect("board event key is UTF-8"))
    }

    pub fn owned_bytes(&self) -> usize {
        usize::from(self.payload_len) + usize::from(self.key_len)
    }

    pub fn write_json(&self, output: &mut String) {
        output.push_str("{\"name\":\"");
        output.push_str(self.kind.name());
        output.push_str("\",\"payload\":");
        output.push_str(self.payload_json());
        output.push('}');
    }

    fn selection(kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        if let Some(removed_ids) = removed_ids {
            payload.raw(",\"removedIds\":")?;
            payload.string_array(removed_ids.iter().map(String::as_str))?;
        }
        if kind == BoardEventKind::Select {
            payload.raw(",\"exitHighlightIds\":[]")?;
        }
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    fn camera(x: f64, y: f64, zoom: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw(",\"zoom\":")?;
        payload.number(zoom)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::Camera, None)
    }

    fn id(kind: BoardEventKind, id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    fn id_list<'a>(kind: BoardEventKind, field: &str, ids: impl IntoIterator<Item = &'a str>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{")?;
        payload.string(field)?;
        payload.raw(":[")?;
        for (count, id) in ids.into_iter().enumerate() {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    fn preselect_sets(ids: &BTreeSet<String>, removed_ids: &BTreeSet<String>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || removed_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"removedIds\":")?;
        payload.string_array(removed_ids.iter().map(String::as_str))?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Preselect, None)
    }

    fn select_set(ids: &BTreeSet<String>, anchor_ids: Option<&BTreeSet<String>>, gesture: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY || anchor_ids.is_some_and(|anchor_ids| anchor_ids.len() > BOARD_POINTER_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        if let Some(anchor_ids) = anchor_ids {
            payload.raw(",\"anchorIds\":")?;
            payload.string_array(anchor_ids.iter().map(String::as_str))?;
        }
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(gesture) = gesture {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(gesture)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    fn edge(kind: BoardEventKind, id: &str, source: &str, target: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"target\":")?;
        payload.string(target)?;
        payload.raw("}")?;
        payload.finish(kind, None)
    }

    fn node_move(id: &str, x: f64, y: f64) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(x)?;
        payload.raw(",\"y\":")?;
        payload.number(y)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::NodeMove, Some(id))
    }

    /// 🔄️ The live rotate-ring frame as resolved positions — a TRANSIENT row hosts mirror into
    /// sibling panes and drop before dispatch, so a drag previews everywhere and still commits once.
    fn transform_preview<'a>(moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        Self::moves_row(BoardEventKind::TransformPreview, moves)
    }

    /// 🕹️ The selection a gesture's press staged, published at its release. `gesture_id` tags it when the
    /// release also carries the gesture record, so both leave as one dispatch.
    fn staged_select(ids: &[String], merge: Option<&str>, gesture_id: Option<&str>) -> Result<Self, BoardEventFault> {
        if ids.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"ids\":")?;
        payload.string_array(ids.iter().map(String::as_str))?;
        payload.raw(",\"exitHighlightIds\":[]")?;
        if let Some(merge) = merge {
            payload.raw(",\"gestureMergeMode\":")?;
            payload.string(merge)?;
        }
        if let Some(gesture_id) = gesture_id {
            payload.raw(",\"gestureId\":")?;
            payload.string(gesture_id)?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Select, None)
    }

    /// 🎬️ ONE finished pointer gesture: its id, kind, the ids it acted on, its motion and the proximity
    /// handle pairs it recorded — never positions, so the record is a parametric input a guest can yield
    /// as one editable mutation.
    fn gesture<'a>(gesture_id: &str, targets: impl IntoIterator<Item = &'a str>, motion: BoardGestureMotion, proximity: &[(String, String)]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"gestureId\":")?;
        payload.string(gesture_id)?;
        payload.raw(",\"kind\":")?;
        payload.string(match motion {
            BoardGestureMotion::Drag { .. } => "drag",
            BoardGestureMotion::Rotate { .. } => "rotate",
        })?;
        payload.raw(",\"targets\":[")?;
        let mut count = 0usize;
        for id in targets {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.string(id)?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]")?;
        match motion {
            BoardGestureMotion::Drag { dx, dy } => {
                payload.raw(",\"dx\":")?;
                payload.number(board_pointer_offset(dx))?;
                payload.raw(",\"dy\":")?;
                payload.number(board_pointer_offset(dy))?;
            }
            BoardGestureMotion::Rotate { pivot, angle } => {
                payload.raw(",\"pivotX\":")?;
                payload.number(pivot.x)?;
                payload.raw(",\"pivotY\":")?;
                payload.number(pivot.y)?;
                payload.raw(",\"angle\":")?;
                payload.number(angle)?;
            }
        }
        payload.raw(",\"proximity\":[")?;
        for (index, (source, target)) in proximity.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"source\":")?;
            payload.string(source)?;
            payload.raw(",\"target\":")?;
            payload.string(target)?;
            payload.raw("}")?;
        }
        payload.raw("]}")?;
        payload.finish(BoardEventKind::Gesture, None)
    }

    fn moves_row<'a>(kind: BoardEventKind, moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"moves\":[")?;
        let mut count = 0usize;
        for (id, x, y) in moves {
            if count == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if count > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"id\":")?;
            payload.string(id)?;
            payload.raw(",\"x\":")?;
            payload.number(x)?;
            payload.raw(",\"y\":")?;
            payload.number(y)?;
            payload.raw("}")?;
            count += 1;
        }
        if count == 0 {
            return Err(BoardEventFault::Schema);
        }
        payload.raw("]}")?;
        payload.finish(kind, None)
    }

    /// 🎯️ One painted target region, as the normalized rectangle the brush released on. The id is
    /// minted by the document, never here — the engine paints geometry, the guest owns identity.
    fn region_create(bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionCreate, None)
    }

    /// 📐️ A region grip drag committed on release: corner AND extent, because a west/north grip
    /// moves the minimum corner as well as the size.
    fn region_resize(id: &str, bounds: [f64; 4]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        payload.string(id)?;
        payload.raw(",\"x\":")?;
        payload.number(bounds[0])?;
        payload.raw(",\"y\":")?;
        payload.number(bounds[1])?;
        payload.raw(",\"width\":")?;
        payload.number(bounds[2] - bounds[0])?;
        payload.raw(",\"height\":")?;
        payload.number(bounds[3] - bounds[1])?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::RegionResize, Some(id))
    }

    fn link_compatible(source: &str, node_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeIds\":")?;
        payload.string_array(node_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkCompatibleNodes, None)
    }

    fn link_ring(source: &str, node_id: Option<&str>, handle_ids: &[String]) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"source\":")?;
        payload.string(source)?;
        payload.raw(",\"nodeId\":")?;
        if let Some(node_id) = node_id {
            payload.string(node_id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"handleIds\":")?;
        payload.string_array(handle_ids.iter().map(String::as_str))?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::LinkTargetRing, None)
    }

    fn hover(id: Option<&str>, kind: Option<(&str, &str)>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"id\":")?;
        if let Some(id) = id {
            payload.string(id)?;
        } else {
            payload.raw("null")?;
        }
        payload.raw(",\"kind\":")?;
        if let Some((domain, kind_id)) = kind {
            payload.raw("{\"domain\":")?;
            payload.string(domain)?;
            payload.raw(",\"kindId\":")?;
            payload.string(kind_id)?;
            payload.raw("}")?;
        } else {
            payload.raw("null")?;
        }
        payload.raw("}")?;
        payload.finish(BoardEventKind::Hover, None)
    }

    fn brush_handles(payload: &mut BoardPayloadBuilder, handles: &[NodeKindHandleTemplate]) -> Result<(), BoardEventFault> {
        if handles.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        payload.raw("[")?;
        for (index, handle) in handles.iter().enumerate() {
            if index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"angle\":")?;
            payload.number(handle.angle)?;
            payload.raw(",\"handleKind\":")?;
            payload.string(&handle.handle_kind)?;
            if let Some(radius) = handle.radius {
                payload.raw(",\"radius\":")?;
                payload.number(radius)?;
            }
            payload.raw("}")?;
        }
        payload.raw("]")
    }

    fn brush_preview(preview: Option<&BrushPreviewSnapshot>) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        let Some(preview) = preview else {
            payload.raw("{\"node\":null,\"edge\":null}")?;
            return payload.finish(BoardEventKind::BrushPreview, None);
        };
        payload.raw("{\"node\":{\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("},\"edge\":{\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw("}}")?;
        payload.finish(BoardEventKind::BrushPreview, None)
    }

    fn brush_candidates(source: &str, candidates: &BrushCandidatePage, index: usize, suggestions_active: bool) -> Result<Self, BoardEventFault> {
        if candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardEventFault::ItemCredits);
        }
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"sourceHandleId\":")?;
        payload.string(source)?;
        payload.raw(",\"candidates\":[")?;
        for (candidate_index, candidate) in candidates.iter().enumerate() {
            if candidate_index > 0 {
                payload.raw(",")?;
            }
            payload.raw("{\"nodeKind\":")?;
            payload.string(candidate.node_kind_id)?;
            payload.raw(",\"targetHandleIndex\":")?;
            payload.usize(candidate.target_handle_index)?;
            payload.raw("}")?;
        }
        payload.raw("],\"index\":")?;
        payload.usize(index)?;
        payload.raw(",\"suggestionsActive\":")?;
        payload.boolean(suggestions_active)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushCandidates, None)
    }

    fn brush_place(preview: &BrushPreviewSnapshot, node_id: &str, edge_id: &str) -> Result<Self, BoardEventFault> {
        let mut payload = BoardPayloadBuilder::new();
        payload.raw("{\"nodeId\":")?;
        payload.string(node_id)?;
        payload.raw(",\"edgeId\":")?;
        payload.string(edge_id)?;
        payload.raw(",\"nodeKind\":")?;
        payload.string(&preview.node_kind_id)?;
        payload.raw(",\"sourceHandleId\":")?;
        payload.string(&preview.source_handle_id)?;
        payload.raw(",\"targetHandleIndex\":")?;
        payload.usize(preview.target_handle_index)?;
        payload.raw(",\"x\":")?;
        payload.number(preview.x)?;
        payload.raw(",\"y\":")?;
        payload.number(preview.y)?;
        payload.raw(",\"shape\":")?;
        payload.string(if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" })?;
        if preview.shape == NodeShape::Rectangle {
            payload.raw(",\"width\":")?;
            payload.number(preview.width)?;
            payload.raw(",\"height\":")?;
            payload.number(preview.height)?;
        } else {
            payload.raw(",\"radius\":")?;
            payload.number(preview.radius)?;
        }
        if let Some(icon) = &preview.icon_kind {
            payload.raw(",\"iconKind\":")?;
            payload.string(icon)?;
        }
        payload.raw(",\"handles\":")?;
        Self::brush_handles(&mut payload, &preview.handles)?;
        payload.raw("}")?;
        payload.finish(BoardEventKind::BrushPlace, None)
    }
}

pub struct BoardEventQueue {
    slots: Box<[Option<BoardOwnedEvent>; BOARD_EVENT_ITEM_CAPACITY]>,
    head: u16,
    len: u16,
    bytes: usize,
    claimed_items: u16,
    claimed_bytes: usize,
    closing: bool,
}

impl Default for BoardEventQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }
    }
}

impl BoardEventQueue {
    pub fn reserve(&self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        if self.closing || usize::from(self.len).checked_add(usize::from(self.claimed_items)).and_then(|value| value.checked_add(items)).is_none_or(|value| value > BOARD_EVENT_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|value| value.checked_add(bytes)).is_none_or(|value| value > BOARD_EVENT_BYTE_CAPACITY) {
            return Err(BoardEventFault::ByteCredits);
        }
        Ok(())
    }

    fn claim(&mut self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        self.reserve(items, bytes)?;
        let items = u16::try_from(items).map_err(|_| BoardEventFault::ItemCredits)?;
        self.claimed_items += items;
        self.claimed_bytes += bytes;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    fn push_claimed(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let bytes = event.owned_bytes();
        if self.closing || self.claimed_items == 0 || self.claimed_bytes < bytes || usize::from(self.len) == BOARD_EVENT_ITEM_CAPACITY {
            return Err(event);
        }
        self.claimed_items -= 1;
        self.claimed_bytes -= bytes;
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += bytes;
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    fn release_claim(&mut self, items: usize, bytes: usize) -> bool {
        let Ok(items) = u16::try_from(items) else {
            return false;
        };
        if self.claimed_items < items || self.claimed_bytes < bytes {
            return false;
        }
        self.claimed_items -= items;
        self.claimed_bytes -= bytes;
        true
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_event(&self, event: BoardOwnedEvent) -> Result<BoardEventReservation, BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        Ok(BoardEventReservation { expected_len: self.len, expected_bytes: self.bytes, event: Some(event) })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_reserved(&mut self, mut reservation: BoardEventReservation) -> Result<(), BoardEventReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        let event = reservation.event.take().expect("board reservation owns one event");
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_batch(&self, mut reservation: BoardEventBatchReservation) -> Result<BoardEventBatchReservation, BoardEventBatchReservation> {
        let Some(bytes) = reservation.owned_bytes() else {
            return Err(reservation);
        };
        if self.reserve(usize::from(reservation.len), bytes).is_err() {
            return Err(reservation);
        }
        reservation.expected_len = self.len;
        reservation.expected_bytes = self.bytes;
        Ok(reservation)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_batch(&mut self, mut reservation: BoardEventBatchReservation) -> Result<(), BoardEventBatchReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        for index in 0..usize::from(reservation.len) {
            let event = reservation.events[index].take().expect("board batch slot is occupied");
            let slot = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
            self.bytes += event.owned_bytes();
            self.slots[slot] = Some(event);
            self.len += 1;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.len == 0 {
            return None;
        }
        let index = usize::from(self.head);
        let event = self.slots[index].take()?;
        self.bytes -= event.owned_bytes();
        self.head = ((index + 1) % BOARD_EVENT_ITEM_CAPACITY) as u16;
        self.len -= 1;
        Some(event)
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &BoardOwnedEvent> {
        (0..usize::from(self.len)).filter_map(|offset| self.slots[(usize::from(self.head) + offset) % BOARD_EVENT_ITEM_CAPACITY].as_ref())
    }

    pub fn owned_bytes(&self) -> usize {
        self.bytes
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.pop().is_some() {
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.bytes == 0 && self.claimed_items == 0 && self.claimed_bytes == 0
    }
}

/// 🎨️ One node's icon commands inside [`WorldContentCache::scene`], baked at `(x, y)`.
struct NodeIconSpan {
    id: String,
    start: usize,
    end: usize,
    x: f64,
    y: f64,
}

/// 🎨️ World-space node icons reused across frames. Spans stay valid under translation: a drag
/// draws each span at `current - baked` until retirement frees a slot for a fresh bake.
struct WorldContentCache {
    generation: u64,
    lod: BoardDrawLod,
    scene: Scene,
    spans: Vec<NodeIconSpan>,
}

pub struct BoardHost {
    pub camera: Camera,
    pub nodes: BTreeMap<String, NodeData>,
    pub handles: BTreeMap<String, HandleData>,
    pub edges: BTreeMap<String, EdgeData>,
    pub wires: BTreeMap<String, WireData>,
    /// 🎯️ Fill-constraining rectangles, keyed by document id. Painted beneath every entity,
    /// hit-tested after all of them, and counted against the descriptor census like any other entity.
    pub regions: BTreeMap<String, RegionData>,
    /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
    pub handle_kinds: BTreeMap<String, HandleKindDef>,
    pub wire_kinds: BTreeMap<String, WireKindDef>,
    pub node_kinds: BTreeMap<String, NodeKindDef>,
    pub edge_kinds: BTreeMap<String, EdgeKindDef>,
    /// 🔺️ Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
    pub edge_tips: BTreeMap<String, EdgeTipDef>,
    /// 🔗️ Kind-compatibility rules for link gestures; empty = unrestricted.
    pub link_compat_rules: Vec<LinkCompatRule>,
    pub selection: BTreeSet<String>,
    /// 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
    pub preselect: BTreeSet<String>,
    /// 💠️ During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
    pub preselect_removed: BTreeSet<String>,
    /// 💠️ After commit: ids dropped in the last `select` transition only.
    pub selection_exit_highlight: BTreeSet<String>,
    pub selection_options: SelectionOptions,
    pub hovered_id: Option<String>,
    /// 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
    pub hovered_kind: Option<(String, String)>,
    /// 💠️ Externally driven highlight ids (e.g. cross-panel variable binding); below hover, above neutral.
    pub highlighted_ids: BTreeSet<String>,
    pub interaction: Interaction,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
    pub world_raster_tiling: String,
    pub events: BoardEventQueue,
    event_overflow: Option<BoardOwnedEvent>,
    event_batch_overflow: Option<BoardEventBatchReservation>,
    event_schema_fault: bool,
    /// Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
    pub selection_screen_preview: Option<Vec<Point>>,
    /// ↔ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
    pub selection_preview_crossing: bool,
    /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
    pub link_screen_preview: Option<Vec<Point>>,
    pub canvas_theme: CanvasPalette,
    /// 👁️ When false, the LOD world grid is not stroked at all (snapping is unaffected — that is `grid_snap_enabled`).
    pub grid_visible: bool,
    /// 📐️ Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
    pub grid_factor: f64,
    /// 🧲️ When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
    pub grid_snap_enabled: bool,
    pub preserve_original_element_style: bool,
    /// 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
    pub automatic_lod: bool,
    forced_draw_lod: Option<BoardDrawLod>,
    pub icon_paint_cache: IconPaintCache,
    /// 📡️ Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
    link_compat_nodes_emit_key: Option<String>,
    /// 📡️ Dedupes {@code linkTargetRing} emissions while a link wire is active.
    link_target_ring_emit_key: Option<String>,
    /// 📡️ Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
    last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
    /// 📡️ Dedupes `preselect` emissions during area-select drag.
    last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
    /// 🧿️ Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
    content_scene_generation: u64,
    /// 🎨️ World-space node icons reused across pan/zoom. Keyed by generation and LOD; a pending
    /// retirement keeps the previous bake and the draw shifts each span onto the live node center.
    world_content_cache: RefCell<Option<WorldContentCache>>,
    opaque_scene_retirement: Cell<Option<OpaqueSceneRetirementToken>>,
    opaque_scene_fault: Cell<bool>,
    /// 🔍️ True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
    wheel_zoom_active: bool,
    /// 📶️ LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
    wheel_zoom_render_lod: Option<BoardDrawLod>,
    /// 🖌️ Active viewport utility (`select` suppresses brush slot logic).
    active_utility: ActiveUtility,
    suggestion_offset: f64,
    brush_node_size: f64,
    brush_slot_source_id: Option<String>,
    brush_candidates: BrushCandidatePage,
    brush_candidate_index: usize,
    brush_preview: Option<BrushPreviewSnapshot>,
    drop_preview: Option<DropPreviewSnapshot>,
    brush_candidates_emit_key: Option<String>,
    brush_preview_emit_key: Option<String>,
    brush_placement_serial: u64,
    brush_node_kind_weights: HashMap<String, f64>,
    brush_handle_kind_weights: HashMap<String, f64>,
    /// ⌥️ Alt held while brushing — enables suggestion offset and commit-on-leave.
    brush_alt_pressed: bool,
    /// ✨️ Suggestions menu opened a slot outside brush utility — use suggestion offset and highlight source handle.
    brush_slot_suggestions_active: bool,
    /// 🕹️ Which selection-gumball handles the select utility offers (`setTransformGumballFlag`).
    transform_flags: TransformGumballFlags,
    /// 🔄️ The live rotate-ring gesture, `None` whenever the ring is merely drawn.
    transform_drag: Option<BoardTransformDrag>,
    /// 🚚️ The live region move/resize gesture (`None` when no region is being dragged).
    region_drag: Option<BoardRegionDrag>,
    /// 🖍️ The live area-brush rectangle (`None` unless the areaBrush utility is painting).
    region_paint: Option<BoardRegionPaint>,
    /// 🖍️ World width/height an area-brush CLICK (no drag) paints, from the utility's own steppers.
    area_brush_extent: (f64, f64),
    pub port_mode: GraphPortMode,
    interaction_revision: u64,
    /// 🪪️ Serial of the last pointer gesture opened ([`GestureStage::id`] is `gesture-<serial>`).
    gesture_serial: u64,
    pending_delete_planning: Option<BoardDeletePlanningOperation>,
    pending_delete_operation: Option<BoardDeleteOperation>,
    pending_pointer_commit: Option<BoardPointerCommitOperation>,
    queued_pointer_commit: Option<BoardPointerPlan>,
    cancelled_pointer_plan_retirement: Option<BoardPointerPlanRetirement>,
    pointer_publication: Option<BoardPointerPublication>,
    close_phase: BoardHostClosePhase,
    close_entity_retirement: Option<BoardEntityRetirement>,
    close_strings: [Option<String>; 16],
    close_string_len: u8,
    close_node_handles: Option<Vec<NodeKindHandleTemplate>>,
}

#[path = "🧭️host/🦀️.rs"]
mod host;
pub use host::*;
#[path = "🎨️style/🦀️.rs"]
mod style;
#[path = "🪣️fill-job/🦀️.rs"]
mod fill_job;
#[path = "🎛️selection-events/🦀️.rs"]
mod selection_events;
#[path = "🧹️scene-delete-links/🦀️.rs"]
mod scene_delete_links;
#[path = "🖱️pointer-gestures/🦀️.rs"]
mod pointer_gestures;
// #endregion board_host


pub use crate::infinite::board::ports::directed::*;
pub use semio_framework_canvas as canvas;
