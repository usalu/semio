//! 🧩️ Directed port graph normal leaf: `BoardHost`, puzzle.2d.fixture, WASM session paint.

pub mod board_host {
    // #region board_host
    //! 🕸️ Generic graph board host on infinite canvas.

    #![allow(clippy::missing_errors_doc, reason = "Graph board host is internal to directed port normal.")]
    #![allow(clippy::too_many_arguments, reason = "Immediate-mode paint helpers take one positional arg per geometry/style input; grouping them into structs would obscure call sites more than it clarifies.")]

    use crate::infinite::canvas::{Affine, Circle, Color, CubicBez, FillRule, Point, Rect, Scene, Stroke, Vec2};
    use serde::{Deserialize, Serialize};
    use std::collections::{BTreeMap, BTreeSet};

    use super::{
        board_json_locked_option, board_json_visible_option, builtin_edge_tips, circle_handle_angle_toward, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, fixture_edge_handle_ids_from_object,
        handle_exterior_cap_fill_path, handle_exterior_cap_stroke_path, handle_outward_at_node_rim, handle_position_on_circle, handle_position_on_rectangle, merge_ids_into_selection, merge_pick_into_selection, normalize_or_zero,
        normalize_selection_mode, pick_merge_mode_for_modifiers, property_bag_from_json, rectangle_handle_angle_toward, selection_drag_enclosing, selection_drag_shape, ActiveUtility, BoardElementStyleKind, CachedIconBody, CachedIconPaintLease,
        CanvasPalette, CompatSpecificity, EdgeData, EdgeDescJson, EdgeKindDef, EdgeStrokePattern, EdgeTipDef, EdgeTipGeometry, FixtureJson, GraphPortMode, HandleData, HandleDescJson, HandleKindDef, IconPaintCache, Interaction, LinkCompatRule,
        NodeData, NodeDescJson, NodeKindDef, NodeKindHandleTemplate, NodeShape, SceneDescriptorJson, SelectionOptions, WireData, WireKindDef,
    };
    use crate::infinite::canvas::camera::Camera;
    use crate::infinite::canvas::geom_sel::{
        cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
        world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
    };
    use graph::manifest::manifest_by_id;

    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;

    pub use crate::infinite::canvas::camera::{CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX, CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN};

    use crate::infinite::canvas::lod::{Lod, LodScale};

    //#region ⚠️ Errors
    /// ⚠️ Errors from board host theme/catalog/layout JSON mutators and manifest validation.
    #[derive(Debug)]
    pub enum NormalPortError {
        Json(serde_json::Error),
        ExternalLinkPreviewJson(serde_json::Error),
        BrushSessionJson(serde_json::Error),
        FixtureDropPreviewJson(serde_json::Error),
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
        UnknownManifestId(String),
        CatalogMissingKind(&'static str, String),
        FixtureDropPreviewInvalid,
        InvalidHandleColor(String, String),
        EventCredits,
    }

    impl std::fmt::Display for NormalPortError {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Json(error) => write!(formatter, "{error}"),
                Self::ExternalLinkPreviewJson(error) => write!(formatter, "setLinkSessionJson: {error}"),
                Self::BrushSessionJson(error) => write!(formatter, "setBrushSessionJson: {error}"),
                Self::FixtureDropPreviewJson(error) => write!(formatter, "setFixtureDropPreviewJson: {error}"),
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
                Self::UnknownManifestId(id) => write!(formatter, "unknown manifest id {id}"),
                Self::CatalogMissingKind(catalog, kind) => write!(formatter, "catalog missing {catalog} kind {kind:?}"),
                Self::FixtureDropPreviewInvalid => formatter.write_str("setFixtureDropPreviewJson: preview payload missing nodeKind, screen/world point, or size"),
                Self::InvalidHandleColor(handle, color) => write!(formatter, "invalid color on handle {handle}: {color:?}"),
                Self::EventCredits => formatter.write_str("board event credits exhausted before descriptor publication"),
            }
        }
    }

    impl std::error::Error for NormalPortError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::Json(error) => std::error::Error::source(error),
                Self::ExternalLinkPreviewJson(error) | Self::BrushSessionJson(error) | Self::FixtureDropPreviewJson(error) => Some(error),
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

    /// @emoji 🎨️ Whether drawable style resolves committed selection chrome or neutral cached geometry.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum StyleChromePass {
        CachedBase,
        InteractionOverlay,
    }

    /// @emoji 🎨️ Which node/handle primitives to paint in a layered draw pass (fills behind icons/text).
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
            let backing_bytes = items.capacity().checked_mul(std::mem::size_of::<T>()).ok_or(())?;
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
            let _ = CAPACITY.checked_mul(std::mem::size_of::<Option<BoardFillPage<T>>>()).ok_or(())?;
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
            let first_unused = (self.len + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS;
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
            self.len == 0 && self.page_count == 0
        }

        fn retained_page_bytes(&self) -> Option<usize> {
            let index = self.page_count.checked_sub(1)?;
            self.pages[index].as_ref().map(|page| page.backing_bytes)
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
            assert!(self.nodes.terminal_is_empty() && self.handles.terminal_is_empty() && self.kinds.terminal_is_empty() && self.rules.terminal_is_empty(), "Puzzle2d fill snapshot must transfer or close every exact page before Drop");
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
        sources: BoardFillFixedPages<BoardFillSource, { (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>,
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
        virtual_nodes: BoardFillFixedPages<BoardFillVirtualNode, { (BOARD_FILL_PLACEMENT_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>,
        virtual_handles: BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>,
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

        pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> bool {
            if maximum_items == 0 {
                return false;
            }
            if self.handles.pop().is_some() {
                return false;
            }
            if let Some(bytes) = self.handles.retained_page_bytes() {
                if bytes > maximum_bytes {
                    return false;
                }
                if self.handles.retire_empty_page().is_none() {
                    return false;
                }
                return false;
            }
            true
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.handles.terminal_is_empty()
        }
    }

    impl Drop for BoardFillPlacement {
        fn drop(&mut self) {
            assert!(self.terminal_is_empty(), "Puzzle2d fill placement must reach exact terminal-empty before Drop");
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
            BoardFillJob { operation: self.operation, state: self.state.take(), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, fault: None, closing: true }
        }
    }

    impl Drop for BoardFillCheckpoint {
        fn drop(&mut self) {
            assert!(self.state.is_none(), "Puzzle2d fill checkpoint must transfer or close its exact state before Drop");
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
    const BOARD_FILL_COMMIT_PAGE_COUNT: usize = (BOARD_FILL_COMMIT_BYTES + semio_framework_job::JOB_PAYLOAD_PAGE_BYTES - 1) / semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

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

        pub fn from_commit_candidate(candidate: &semio_framework_job::CommitCandidate) -> Option<Self> {
            let payload = BoardFillCommitPayload::new(&candidate.output)?;
            if !candidate.state.is_empty() || payload.read::<4>(0)? != BOARD_FILL_COMMIT_MAGIC || payload.byte(4)? != BOARD_FILL_COMMIT_VERSION {
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
        fault: Option<&'static str>,
        closing: bool,
    }

    #[derive(Clone, Debug)]
    struct FixtureDropPreviewSnapshot {
        node_kind_id: String,
        x: f64,
        y: f64,
        shape: NodeShape,
        radius: f64,
        width: f64,
        height: f64,
        icon_kind: Option<String>,
    }

    #[derive(Clone, Debug, serde::Serialize)]
    struct BoardPickTargetJson {
        domain: String,
        id: String,
        generality: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    pub const BOARD_EVENT_ITEM_CAPACITY: usize = 256;
    pub const BOARD_EVENT_BYTE_CAPACITY: usize = 256 * 1024;
    pub const BOARD_EVENT_PAYLOAD_BYTE_CAPACITY: usize = 16 * 1024;
    pub const BOARD_EVENT_KEY_BYTE_CAPACITY: usize = 256;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BoardEventKind {
        Camera,
        NodeMove,
        NodeDragEnd,
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
    }

    impl BoardEventKind {
        pub fn name(self) -> &'static str {
            match self {
                Self::Camera => "camera",
                Self::NodeMove => "nodeMove",
                Self::NodeDragEnd => "nodeDragEnd",
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
            }
        }
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
            let mut count = 0usize;
            for id in ids {
                if count == BOARD_POINTER_ITEM_CAPACITY {
                    return Err(BoardEventFault::ItemCredits);
                }
                if count > 0 {
                    payload.raw(",")?;
                }
                payload.string(id)?;
                count += 1;
            }
            payload.raw("]}")?;
            payload.finish(kind, None)
        }

        fn select_ids<'a>(ids: impl IntoIterator<Item = &'a str>) -> Result<Self, BoardEventFault> {
            let mut payload = BoardPayloadBuilder::new();
            payload.raw("{\"ids\":[")?;
            let mut count = 0usize;
            for id in ids {
                if count == BOARD_POINTER_ITEM_CAPACITY {
                    return Err(BoardEventFault::ItemCredits);
                }
                if count > 0 {
                    payload.raw(",")?;
                }
                payload.string(id)?;
                count += 1;
            }
            payload.raw("],\"exitHighlightIds\":[]}")?;
            payload.finish(BoardEventKind::Select, None)
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

        fn node_drag_end<'a>(moves: impl IntoIterator<Item = (&'a str, f64, f64)>) -> Result<Self, BoardEventFault> {
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
            payload.finish(BoardEventKind::NodeDragEnd, None)
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
            Self { slots: Box::new(std::array::from_fn(|_| None)), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }
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

        pub fn reserve_event(&self, event: BoardOwnedEvent) -> Result<BoardEventReservation, BoardOwnedEvent> {
            if self.reserve(1, event.owned_bytes()).is_err() {
                return Err(event);
            }
            Ok(BoardEventReservation { expected_len: self.len, expected_bytes: self.bytes, event: Some(event) })
        }

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

    pub struct BoardHost {
        pub camera: Camera,
        pub nodes: BTreeMap<String, NodeData>,
        pub handles: BTreeMap<String, HandleData>,
        pub edges: BTreeMap<String, EdgeData>,
        pub wires: BTreeMap<String, WireData>,
        /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
        pub handle_kinds: BTreeMap<String, HandleKindDef>,
        pub wire_kinds: BTreeMap<String, WireKindDef>,
        pub node_kinds: BTreeMap<String, NodeKindDef>,
        pub edge_kinds: BTreeMap<String, EdgeKindDef>,
        /// @emoji 🔺️ Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
        pub edge_tips: BTreeMap<String, EdgeTipDef>,
        /// @emoji 🔗️ Kind-compatibility rules for link gestures; empty = unrestricted.
        pub link_compat_rules: Vec<LinkCompatRule>,
        pub selection: BTreeSet<String>,
        /// @emoji 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
        pub preselect: BTreeSet<String>,
        /// @emoji 💠️ During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
        pub preselect_removed: BTreeSet<String>,
        /// @emoji 💠️ After commit: ids dropped in the last `select` transition only.
        pub selection_exit_highlight: BTreeSet<String>,
        pub selection_options: SelectionOptions,
        pub hovered_id: Option<String>,
        /// @emoji 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
        pub hovered_kind: Option<(String, String)>,
        /// @emoji 💠️ Externally driven highlight ids (e.g. cross-panel variable binding); below hover, above neutral.
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
        /// @emoji ↔ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
        pub selection_preview_crossing: bool,
        /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
        pub link_screen_preview: Option<Vec<Point>>,
        pub canvas_theme: CanvasPalette,
        /// @emoji 📐️ Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
        pub grid_factor: f64,
        /// @emoji 🧲️ When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
        pub grid_snap_enabled: bool,
        pub preserve_original_element_style: bool,
        /// @emoji 📶️ When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
        pub automatic_lod: bool,
        forced_draw_lod: Option<BoardDrawLod>,
        pub icon_paint_cache: IconPaintCache,
        /// @emoji 📡️ Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
        link_compat_nodes_emit_key: Option<String>,
        /// @emoji 📡️ Dedupes {@code linkTargetRing} emissions while a link wire is active.
        link_target_ring_emit_key: Option<String>,
        /// @emoji 📡️ Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
        last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
        /// @emoji 📡️ Dedupes `preselect` emissions during area-select drag.
        last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
        /// @emoji 🧿️ Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
        content_scene_generation: u64,
        /// @emoji 🎨️ World-space Vello content reused across pan/zoom when generation and LOD match.
        world_content_cache: RefCell<Option<(u64, BoardDrawLod, Scene)>>,
        opaque_scene_fault: Cell<bool>,
        /// @emoji 🔍️ True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
        wheel_zoom_active: bool,
        /// @emoji 📶️ LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
        wheel_zoom_render_lod: Option<BoardDrawLod>,
        /// @emoji 🖌️ Active viewport utility (`select` suppresses brush slot logic).
        active_utility: ActiveUtility,
        suggestion_offset: f64,
        brush_node_size: f64,
        brush_slot_source_id: Option<String>,
        brush_candidates: BrushCandidatePage,
        brush_candidate_index: usize,
        brush_preview: Option<BrushPreviewSnapshot>,
        fixture_drop_preview: Option<FixtureDropPreviewSnapshot>,
        brush_candidates_emit_key: Option<String>,
        brush_preview_emit_key: Option<String>,
        brush_placement_serial: u64,
        brush_node_kind_weights: HashMap<String, f64>,
        brush_handle_kind_weights: HashMap<String, f64>,
        /// @emoji ⌥️ Alt held while brushing — enables suggestion offset and commit-on-leave.
        brush_alt_pressed: bool,
        /// @emoji ✨️ Suggestions menu opened a slot outside brush utility — use suggestion offset and highlight source handle.
        brush_slot_suggestions_active: bool,
        pub port_mode: GraphPortMode,
        interaction_revision: u64,
        pending_delete_planning: Option<BoardDeletePlanningOperation>,
        pending_delete_operation: Option<BoardDeleteOperation>,
        pending_pointer_commit: Option<BoardPointerCommitOperation>,
        queued_pointer_commit: Option<BoardPointerPlan>,
        pointer_publication: Option<BoardPointerPublication>,
        close_phase: BoardHostClosePhase,
        close_entity_retirement: Option<BoardEntityRetirement>,
        close_strings: [Option<String>; 16],
        close_string_len: u8,
        close_node_handles: Option<Vec<NodeKindHandleTemplate>>,
    }

    pub struct BoardHostRetirement {
        host: std::mem::ManuallyDrop<BoardHost>,
        released: bool,
    }

    impl BoardHostRetirement {
        pub fn new(host: BoardHost) -> Self {
            Self { host: std::mem::ManuallyDrop::new(host), released: false }
        }

        pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
            if self.released {
                return true;
            }
            if !self.host.close_nonopaque_step(context) {
                return false;
            }
            assert!(self.host.nonopaque_terminal_is_empty(), "BoardHost nonopaque terminal witness precedes shallow release");
            unsafe { std::mem::ManuallyDrop::drop(&mut self.host) };
            self.released = true;
            true
        }

        pub fn terminal_nonopaque_is_empty(&self) -> bool {
            self.released
        }
    }

    impl Drop for BoardHostRetirement {
        fn drop(&mut self) {
            debug_assert!(self.released, "BoardHostRetirement must reach terminal-empty before release");
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum BoardHostClosePhase {
        Events,
        Pointer,
        WorldScene,
        Icons,
        Nodes,
        Handles,
        Edges,
        Wires,
        Selection,
        Preselect,
        PreselectRemoved,
        SelectionExit,
        Highlighted,
        Interaction,
        HandleKinds,
        WireKinds,
        NodeKinds,
        EdgeKinds,
        EdgeTips,
        LinkRules,
        Previews,
        Strings,
        Weights,
        Done,
    }

    #[derive(Clone, Debug)]
    pub struct BoardWheelPlan {
        revision: u64,
        expected: Camera,
        next: Camera,
    }

    pub const BOARD_POINTER_ITEM_CAPACITY: usize = 256;
    pub const BOARD_POINTER_BYTE_CAPACITY: usize = 16 * 1024;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BoardPointerPhase {
        Move,
        Up,
        Leave,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct BoardPointerIntent {
        pub phase: BoardPointerPhase,
        pub x: f64,
        pub y: f64,
        pub shift: bool,
        pub ctrl_or_meta: bool,
        pub alt: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BoardPointerPlanFault {
        ItemCredits,
        ByteCredits,
        Unsupported,
    }

    #[derive(Clone, Copy, Debug)]
    struct BoardPointerSpan {
        start: u16,
        len: u16,
    }

    #[derive(Clone, Copy, Debug)]
    struct BoardPointerDelta {
        id: BoardPointerSpan,
        x: f64,
        y: f64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum BoardDeleteKind {
        Edge,
        Node,
        Handle,
        Wire,
    }

    #[derive(Clone, Copy, Debug)]
    struct BoardDeleteEntry {
        kind: BoardDeleteKind,
        id: BoardPointerSpan,
    }

    struct BoardDeletePlan {
        revision: u64,
        bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
        byte_len: u16,
        entries: Box<[Option<BoardDeleteEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
        len: u16,
        property_nodes: u16,
        property_bytes: u16,
    }

    struct BoardDeleteOperation {
        plan: BoardDeletePlan,
        select_event: Option<BoardOwnedEvent>,
        mutation_cursor: u16,
        publication_cursor: u16,
        remaining_claimed_items: u16,
        remaining_claimed_bytes: usize,
        claimed: bool,
        retiring_entity: Option<BoardEntityRetirement>,
        cancelling: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum BoardDeletePlanningPhase {
        SelectedEdges,
        Nodes,
        DiscoverNode,
        Handles,
        Wires,
        Edges,
        Handle,
        Node,
        SelectionEvent,
        Finish,
    }

    struct BoardDeletePlanningOperation {
        plan: BoardDeletePlan,
        phase: BoardDeletePlanningPhase,
        scan_after: Option<String>,
        node_after: Option<String>,
        handle_after: Option<String>,
        relation_after: Option<String>,
        node_id: Option<String>,
        handle_id: Option<String>,
        node_relevant: bool,
        property_audit: Option<BoardPropertyAudit>,
        select_builder: BoardPayloadBuilder,
        select_first: bool,
        fault: Option<BoardEventFault>,
        cancelling: bool,
    }

    enum BoardPropertyAuditFrame {
        Array { values: Vec<graph::manifest::PropertyValue>, index: usize, pending: Option<usize> },
        Object { values: graph::PropertyBag, after: Option<String>, pending: Option<String> },
    }

    struct BoardPropertyAudit {
        kind: BoardDeleteKind,
        id: String,
        current: Option<graph::manifest::PropertyValue>,
        completed: Option<graph::manifest::PropertyValue>,
        stack: Box<[Option<BoardPropertyAuditFrame>; BOARD_POINTER_ITEM_CAPACITY]>,
        depth: u16,
        nodes: usize,
        bytes: usize,
        fault: Option<BoardEventFault>,
    }

    enum BoardRemovedEntity {
        Node(NodeData),
        Edge(EdgeData),
        Handle(HandleData),
        Wire(WireData),
    }

    enum BoardPropertyContainer {
        Array(Vec<graph::manifest::PropertyValue>),
        Object(graph::PropertyBag),
    }

    struct BoardPropertyRetirement {
        current: Option<graph::manifest::PropertyValue>,
        stack: Box<[Option<BoardPropertyContainer>; BOARD_POINTER_ITEM_CAPACITY]>,
        depth: u16,
        faulted: bool,
    }

    struct BoardEntityRetirement {
        entity: Option<BoardRemovedEntity>,
        strings: Box<[Option<String>; 16]>,
        string_len: u8,
        properties: Option<BoardPropertyRetirement>,
    }

    pub struct BoardPointerPublication {
        bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
        len: u16,
    }

    struct BoardPointerCommitOperation {
        plan: BoardPointerPlan,
        phase: u8,
        cursor: u16,
        changed: bool,
        cancelling: bool,
        faulted: bool,
        scan_after: Option<String>,
        points: Vec<Point>,
        screen_points: Vec<Point>,
        overlay_points: Vec<Point>,
        brush_candidates: BrushCandidatePage,
        retiring_ids: BTreeSet<String>,
        retiring_points: Vec<Point>,
        retiring_screen_points: Vec<Point>,
        retiring_overlay_points: Vec<Point>,
        retiring_signature_a: Vec<String>,
        retiring_signature_b: Vec<String>,
        retiring_signature_c: Vec<String>,
        retiring_gestures: [Option<String>; 2],
    }

    impl BoardPointerCommitOperation {
        fn new(plan: BoardPointerPlan) -> Self {
            Self {
                plan,
                phase: 0,
                cursor: 0,
                changed: false,
                cancelling: false,
                faulted: false,
                scan_after: None,
                points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
                screen_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
                overlay_points: Vec::with_capacity(BOARD_POINTER_ITEM_CAPACITY),
                brush_candidates: BrushCandidatePage::default(),
                retiring_ids: BTreeSet::new(),
                retiring_points: Vec::new(),
                retiring_screen_points: Vec::new(),
                retiring_overlay_points: Vec::new(),
                retiring_signature_a: Vec::new(),
                retiring_signature_b: Vec::new(),
                retiring_signature_c: Vec::new(),
                retiring_gestures: [None, None],
            }
        }

        fn retire_one(&mut self) -> bool {
            if let Some(id) = self.retiring_ids.pop_first() {
                drop(id);
                return true;
            }
            if self.retiring_points.pop().is_some() || self.retiring_screen_points.pop().is_some() || self.retiring_overlay_points.pop().is_some() {
                return true;
            }
            for values in [&mut self.retiring_signature_a, &mut self.retiring_signature_b, &mut self.retiring_signature_c] {
                if let Some(value) = values.pop() {
                    drop(value);
                    return true;
                }
            }
            for gesture in &mut self.retiring_gestures {
                if let Some(value) = gesture.take() {
                    drop(value);
                    return true;
                }
            }
            false
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BoardAuthorityStep {
        Pending,
        Complete,
        Cancelled,
        Fault,
    }

    impl BoardDeletePlan {
        fn new(revision: u64) -> Self {
            Self { revision, bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0, property_nodes: 0, property_bytes: 0 }
        }

        fn id(&self, span: BoardPointerSpan) -> &str {
            let start = usize::from(span.start);
            let end = start + usize::from(span.len);
            std::str::from_utf8(&self.bytes[start..end]).expect("board deletion ids originate from UTF-8 strings")
        }

        fn contains(&self, kind: BoardDeleteKind, id: &str) -> bool {
            self.entries[..usize::from(self.len)].iter().flatten().any(|entry| entry.kind == kind && self.id(entry.id) == id)
        }

        fn removes_selection_id(&self, id: &str) -> bool {
            self.entries[..usize::from(self.len)].iter().flatten().any(|entry| self.id(entry.id) == id)
        }

        fn push(&mut self, kind: BoardDeleteKind, id: &str) -> Result<(), BoardEventFault> {
            if self.contains(kind, id) {
                return Ok(());
            }
            if matches!(kind, BoardDeleteKind::Edge | BoardDeleteKind::Node) && 7usize.checked_add(board_json_string_bytes(id)).is_none_or(|bytes| bytes > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY) {
                return Err(BoardEventFault::ByteCredits);
            }
            let index = usize::from(self.len);
            if index == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if id.len() > u16::MAX as usize {
                return Err(BoardEventFault::ByteCredits);
            }
            let start = usize::from(self.byte_len);
            let end = start.checked_add(id.len()).ok_or(BoardEventFault::ByteCredits)?;
            if end > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
            self.bytes[start..end].copy_from_slice(id.as_bytes());
            self.byte_len = end as u16;
            self.entries[index] = Some(BoardDeleteEntry { kind, id: BoardPointerSpan { start: start as u16, len: id.len() as u16 } });
            self.len += 1;
            Ok(())
        }

        #[cfg(test)]
        fn push_entity(&mut self, kind: BoardDeleteKind, id: &str, properties: &graph::PropertyBag) -> Result<(), BoardEventFault> {
            if self.contains(kind, id) {
                return Ok(());
            }
            let (nodes, bytes) = board_property_retirement_credits(properties)?;
            let next_nodes = usize::from(self.property_nodes).checked_add(nodes).ok_or(BoardEventFault::ItemCredits)?;
            let next_bytes = usize::from(self.property_bytes).checked_add(bytes).ok_or(BoardEventFault::ByteCredits)?;
            if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
            self.push(kind, id)?;
            self.property_nodes = next_nodes as u16;
            self.property_bytes = next_bytes as u16;
            Ok(())
        }

        fn push_admitted_entity(&mut self, kind: BoardDeleteKind, id: &str, property_nodes: usize, property_bytes: usize) -> Result<(), BoardEventFault> {
            if self.contains(kind, id) {
                return Ok(());
            }
            let next_nodes = usize::from(self.property_nodes).checked_add(property_nodes).ok_or(BoardEventFault::ItemCredits)?;
            let next_bytes = usize::from(self.property_bytes).checked_add(property_bytes).ok_or(BoardEventFault::ByteCredits)?;
            if next_nodes > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            if next_bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
            self.push(kind, id)?;
            self.property_nodes = next_nodes as u16;
            self.property_bytes = next_bytes as u16;
            Ok(())
        }

        fn emitted_event_count(&self) -> usize {
            self.entries[..usize::from(self.len)].iter().flatten().filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node)).count()
        }

        fn emitted_event_bytes(&self) -> Option<usize> {
            self.entries[..usize::from(self.len)]
                .iter()
                .flatten()
                .filter(|entry| matches!(entry.kind, BoardDeleteKind::Edge | BoardDeleteKind::Node))
                .try_fold(0usize, |bytes, entry| bytes.checked_add(7)?.checked_add(board_json_string_bytes(self.id(entry.id))))
        }
    }

    #[cfg(test)]
    fn board_property_retirement_credits(properties: &graph::PropertyBag) -> Result<(usize, usize), BoardEventFault> {
        let mut stack: Box<[Option<(&graph::manifest::PropertyValue, u16)>; BOARD_POINTER_ITEM_CAPACITY]> = Box::new(std::array::from_fn(|_| None));
        let mut stack_len = 0usize;
        let mut nodes = 0usize;
        let mut bytes = 0usize;
        for (key, value) in properties {
            bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
            if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            stack[stack_len] = Some((value, 1));
            stack_len += 1;
        }
        while stack_len > 0 {
            stack_len -= 1;
            let (value, depth) = stack[stack_len].take().expect("property pre-admission stack owner");
            nodes += 1;
            if nodes > BOARD_POINTER_ITEM_CAPACITY || usize::from(depth) > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            match value {
                graph::manifest::PropertyValue::String(value) => {
                    bytes = bytes.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
                }
                graph::manifest::PropertyValue::Array(values) => {
                    for value in values {
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                graph::manifest::PropertyValue::Object(values) => {
                    for (key, value) in values {
                        bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                graph::manifest::PropertyValue::Null | graph::manifest::PropertyValue::Bool(_) | graph::manifest::PropertyValue::Number(_) => {}
            }
            if bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
        }
        Ok((nodes, bytes))
    }

    impl BoardPropertyAudit {
        fn new(kind: BoardDeleteKind, id: String, properties: graph::PropertyBag) -> Self {
            Self { kind, id, current: Some(graph::manifest::PropertyValue::Object(properties)), completed: None, stack: Box::new(std::array::from_fn(|_| None)), depth: 0, nodes: 0, bytes: 0, fault: None }
        }

        fn add_bytes(&mut self, bytes: usize) {
            self.bytes = self.bytes.checked_add(bytes).unwrap_or(usize::MAX);
            if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
                self.fault.get_or_insert(BoardEventFault::ByteCredits);
            }
        }

        fn push_frame(&mut self, frame: BoardPropertyAuditFrame) -> Result<(), BoardPropertyAuditFrame> {
            let index = usize::from(self.depth);
            if index == BOARD_POINTER_ITEM_CAPACITY {
                return Err(frame);
            }
            self.stack[index] = Some(frame);
            self.depth += 1;
            Ok(())
        }

        fn step(&mut self) -> bool {
            if let Some(value) = self.completed.take() {
                if self.depth == 0 {
                    self.completed = Some(value);
                    return true;
                }
                let frame = self.stack[usize::from(self.depth - 1)].as_mut().expect("property audit frame");
                match frame {
                    BoardPropertyAuditFrame::Array { values, pending, .. } => {
                        let index = pending.take().expect("array audit child index");
                        values[index] = value;
                    }
                    BoardPropertyAuditFrame::Object { values, pending, .. } => {
                        let key = pending.take().expect("object audit child key");
                        *values.get_mut(&key).expect("object audit key remains reserved") = value;
                    }
                }
                return false;
            }
            if let Some(value) = self.current.take() {
                self.nodes = self.nodes.saturating_add(1);
                if self.nodes > BOARD_POINTER_ITEM_CAPACITY {
                    self.fault.get_or_insert(BoardEventFault::ItemCredits);
                }
                if self.fault.is_some() {
                    self.completed = Some(value);
                    return false;
                }
                match value {
                    graph::manifest::PropertyValue::Array(values) => {
                        let frame = BoardPropertyAuditFrame::Array { values, index: 0, pending: None };
                        if let Err(frame) = self.push_frame(frame) {
                            self.fault = Some(BoardEventFault::ItemCredits);
                            self.completed = Some(match frame {
                                BoardPropertyAuditFrame::Array { values, .. } => graph::manifest::PropertyValue::Array(values),
                                BoardPropertyAuditFrame::Object { .. } => unreachable!(),
                            });
                        }
                    }
                    graph::manifest::PropertyValue::Object(values) => {
                        let frame = BoardPropertyAuditFrame::Object { values, after: None, pending: None };
                        if let Err(frame) = self.push_frame(frame) {
                            self.fault = Some(BoardEventFault::ItemCredits);
                            self.completed = Some(match frame {
                                BoardPropertyAuditFrame::Object { values, .. } => graph::manifest::PropertyValue::Object(values),
                                BoardPropertyAuditFrame::Array { .. } => unreachable!(),
                            });
                        }
                    }
                    graph::manifest::PropertyValue::String(value) => {
                        self.add_bytes(value.len());
                        self.completed = Some(graph::manifest::PropertyValue::String(value));
                    }
                    scalar => self.completed = Some(scalar),
                }
                return false;
            }
            if self.depth == 0 {
                return false;
            }
            let index = usize::from(self.depth - 1);
            if self.fault.is_some() {
                let frame = self.stack[index].take().expect("faulted property audit frame");
                self.depth -= 1;
                self.completed = Some(match frame {
                    BoardPropertyAuditFrame::Array { values, .. } => graph::manifest::PropertyValue::Array(values),
                    BoardPropertyAuditFrame::Object { values, .. } => graph::manifest::PropertyValue::Object(values),
                });
                return false;
            }
            let frame = self.stack[index].take().expect("property audit frame");
            match frame {
                BoardPropertyAuditFrame::Array { mut values, mut index, mut pending } => {
                    if index == values.len() {
                        self.depth -= 1;
                        self.completed = Some(graph::manifest::PropertyValue::Array(values));
                    } else {
                        let child_index = index;
                        index += 1;
                        pending = Some(child_index);
                        self.current = Some(std::mem::replace(&mut values[child_index], graph::manifest::PropertyValue::Null));
                        self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Array { values, index, pending });
                    }
                }
                BoardPropertyAuditFrame::Object { mut values, mut after, mut pending } => {
                    let next = match after.as_ref() {
                        Some(after) => values.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => values.first_key_value(),
                    }
                    .map(|(key, _)| admitted_board_pointer_id(key));
                    match next {
                        Some(Ok(key)) => {
                            self.bytes = self.bytes.checked_add(key.len()).unwrap_or(usize::MAX);
                            if self.bytes > BOARD_POINTER_BYTE_CAPACITY {
                                self.fault.get_or_insert(BoardEventFault::ByteCredits);
                            }
                            after = Some(key.clone());
                            pending = Some(key.clone());
                            self.current = Some(std::mem::replace(values.get_mut(&key).expect("property audit key"), graph::manifest::PropertyValue::Null));
                            self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                        }
                        Some(Err(fault)) => {
                            self.fault = Some(fault);
                            self.stack[usize::from(self.depth - 1)] = Some(BoardPropertyAuditFrame::Object { values, after, pending });
                        }
                        None => {
                            self.depth -= 1;
                            self.completed = Some(graph::manifest::PropertyValue::Object(values));
                        }
                    }
                }
            }
            false
        }

        fn take_result(&mut self) -> Option<(BoardDeleteKind, String, graph::PropertyBag, usize, usize, Option<BoardEventFault>)> {
            if self.depth != 0 || self.current.is_some() {
                return None;
            }
            let graph::manifest::PropertyValue::Object(properties) = self.completed.take()? else {
                self.fault = Some(BoardEventFault::Schema);
                return None;
            };
            Some((self.kind, std::mem::take(&mut self.id), properties, self.nodes, self.bytes, self.fault.take()))
        }
    }

    impl BoardPropertyRetirement {
        fn new(properties: graph::PropertyBag) -> Self {
            Self { current: Some(graph::manifest::PropertyValue::Object(properties)), stack: Box::new(std::array::from_fn(|_| None)), depth: 0, faulted: false }
        }

        fn push_container(&mut self, container: BoardPropertyContainer) -> Result<(), BoardPropertyContainer> {
            let index = usize::from(self.depth);
            if index == BOARD_POINTER_ITEM_CAPACITY {
                return Err(container);
            }
            self.stack[index] = Some(container);
            self.depth += 1;
            Ok(())
        }

        fn pop_container(&mut self) -> Option<BoardPropertyContainer> {
            if self.depth == 0 {
                return None;
            }
            self.depth -= 1;
            self.stack[usize::from(self.depth)].take()
        }

        fn step(&mut self) -> Result<bool, ()> {
            if self.faulted {
                return Err(());
            }
            if self.current.is_none() {
                self.current = self.pop_container().map(|container| match container {
                    BoardPropertyContainer::Array(values) => graph::manifest::PropertyValue::Array(values),
                    BoardPropertyContainer::Object(values) => graph::manifest::PropertyValue::Object(values),
                });
                return Ok(self.current.is_none());
            }
            let current = self.current.take().expect("property retirement current owner");
            match current {
                graph::manifest::PropertyValue::Array(mut values) => {
                    let child = values.pop();
                    if !values.is_empty() {
                        if let Err(values) = self.push_container(BoardPropertyContainer::Array(values)) {
                            self.current = Some(match values {
                                BoardPropertyContainer::Array(values) => graph::manifest::PropertyValue::Array(values),
                                BoardPropertyContainer::Object(_) => unreachable!(),
                            });
                            self.faulted = true;
                            return Err(());
                        }
                    }
                    self.current = child;
                }
                graph::manifest::PropertyValue::Object(mut values) => {
                    let child = values.pop_first().map(|(key, value)| {
                        drop(key);
                        value
                    });
                    if !values.is_empty() {
                        if let Err(values) = self.push_container(BoardPropertyContainer::Object(values)) {
                            self.current = Some(match values {
                                BoardPropertyContainer::Object(values) => graph::manifest::PropertyValue::Object(values),
                                BoardPropertyContainer::Array(_) => unreachable!(),
                            });
                            self.faulted = true;
                            return Err(());
                        }
                    }
                    self.current = child;
                }
                graph::manifest::PropertyValue::String(value) => drop(value),
                graph::manifest::PropertyValue::Null | graph::manifest::PropertyValue::Bool(_) | graph::manifest::PropertyValue::Number(_) => {}
            }
            Ok(false)
        }

        fn terminal_is_empty(&self) -> bool {
            self.current.is_none() && self.depth == 0 && !self.faulted
        }
    }

    impl BoardEntityRetirement {
        fn new(entity: BoardRemovedEntity) -> Self {
            Self { entity: Some(entity), strings: Box::new(std::array::from_fn(|_| None)), string_len: 0, properties: None }
        }

        fn push_string(&mut self, value: String) {
            let index = usize::from(self.string_len);
            self.strings[index] = Some(value);
            self.string_len += 1;
        }

        fn push_optional_string(&mut self, value: Option<String>) {
            if let Some(value) = value {
                self.push_string(value);
            }
        }

        fn split_entity(&mut self, entity: BoardRemovedEntity) {
            let properties = match entity {
                BoardRemovedEntity::Node(NodeData { id, style, text, icon_kind, node_kind, properties, .. }) => {
                    self.push_string(id);
                    self.push_optional_string(style);
                    self.push_optional_string(text);
                    self.push_optional_string(icon_kind);
                    self.push_string(node_kind);
                    properties
                }
                BoardRemovedEntity::Edge(EdgeData { id, source, target, style, edge_kind, source_tip, target_tip, properties, .. }) => {
                    self.push_string(id);
                    self.push_string(source);
                    self.push_string(target);
                    self.push_optional_string(style);
                    self.push_string(edge_kind);
                    self.push_optional_string(source_tip);
                    self.push_optional_string(target_tip);
                    properties
                }
                BoardRemovedEntity::Handle(HandleData { id, node_id, style, handle_kind, icon_kind, properties, .. }) => {
                    self.push_string(id);
                    self.push_string(node_id);
                    self.push_optional_string(style);
                    self.push_string(handle_kind);
                    self.push_optional_string(icon_kind);
                    properties
                }
                BoardRemovedEntity::Wire(WireData { id, source, target, style, wire_kind, properties, .. }) => {
                    self.push_string(id);
                    self.push_string(source);
                    self.push_optional_string(target);
                    self.push_optional_string(style);
                    self.push_string(wire_kind);
                    properties
                }
            };
            self.properties = Some(BoardPropertyRetirement::new(properties));
        }

        fn step(&mut self) -> Result<bool, ()> {
            if let Some(entity) = self.entity.take() {
                self.split_entity(entity);
                return Ok(false);
            }
            if let Some(properties) = self.properties.as_mut() {
                if !properties.step()? {
                    return Ok(false);
                }
                debug_assert!(properties.terminal_is_empty());
                self.properties = None;
                return Ok(false);
            }
            if self.string_len > 0 {
                self.string_len -= 1;
                drop(self.strings[usize::from(self.string_len)].take());
                return Ok(false);
            }
            Ok(true)
        }

        fn terminal_is_empty(&self) -> bool {
            self.entity.is_none() && self.string_len == 0 && self.properties.is_none()
        }
    }

    impl BoardPointerPublication {
        pub fn events_json(&self) -> &str {
            std::str::from_utf8(&self.bytes[..usize::from(self.len)]).expect("pointer publication is encoded from UTF-8 schema tokens")
        }

        pub fn close_step(&mut self) -> bool {
            self.len = 0;
            true
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.len == 0
        }
    }

    #[derive(Clone, Copy, Debug)]
    enum BoardPointerPlanKind {
        Idle,
        Pan {
            camera: [f64; 3],
        },
        DragMove,
        FinishPan {
            camera: [f64; 3],
        },
        FinishDrag,
        SelectionPreview {
            start: Point,
            start_screen: Point,
        },
        SelectionCommit,
        LinkMove {
            source: BoardPointerSpan,
            target: Option<BoardPointerSpan>,
            hover: Option<BoardPointerSpan>,
            compat_key: BoardPointerSpan,
            ring_key: BoardPointerSpan,
            end_world: Point,
            activated: bool,
            start_screen: Point,
        },
        LinkFinish {
            source: BoardPointerSpan,
            target: Option<BoardPointerSpan>,
            edge: Option<BoardPointerSpan>,
            target_node: Option<BoardPointerSpan>,
            hover: Option<BoardPointerSpan>,
            compat_key: Option<BoardPointerSpan>,
            ring_key: Option<BoardPointerSpan>,
        },
        LinkRetain {
            hover: Option<BoardPointerSpan>,
        },
        Hover {
            hover: Option<BoardPointerSpan>,
        },
        Brush {
            source: Option<BoardPointerSpan>,
            hover: Option<BoardPointerSpan>,
            alt: bool,
            commit_old: bool,
        },
        LeaveIdle,
    }

    #[derive(Debug)]
    pub struct BoardPointerPlan {
        revision: u64,
        kind: BoardPointerPlanKind,
        bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
        byte_len: u16,
        deltas: Box<[Option<BoardPointerDelta>; BOARD_POINTER_ITEM_CAPACITY]>,
        delta_len: u16,
        points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
        screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
        point_len: u16,
        output: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
        output_len: u16,
    }

    pub struct BoardPointerPlanRetirement {
        plan: Option<BoardPointerPlan>,
        cursor: u16,
    }

    impl BoardPointerPlanRetirement {
        pub fn new(plan: BoardPointerPlan) -> Self {
            Self { plan: Some(plan), cursor: 0 }
        }

        pub fn close_step(&mut self) -> bool {
            let Some(plan) = self.plan.as_mut() else {
                return true;
            };
            if self.cursor < plan.delta_len {
                plan.deltas[usize::from(self.cursor)] = None;
                self.cursor += 1;
                return false;
            }
            plan.byte_len = 0;
            plan.output_len = 0;
            self.plan = None;
            true
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.plan.is_none()
        }
    }

    impl BoardPointerPlan {
        fn empty(revision: u64, kind: BoardPointerPlanKind) -> Self {
            Self {
                revision,
                kind,
                bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
                byte_len: 0,
                deltas: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]),
                delta_len: 0,
                points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
                screen_points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
                point_len: 0,
                output: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
                output_len: 0,
            }
        }

        fn push_id(&mut self, id: &str) -> Result<BoardPointerSpan, BoardPointerPlanFault> {
            if id.len() > u16::MAX as usize {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            let start = usize::from(self.byte_len);
            let end = start.checked_add(id.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
            if end > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            self.bytes[start..end].copy_from_slice(id.as_bytes());
            self.byte_len = end as u16;
            Ok(BoardPointerSpan { start: start as u16, len: id.len() as u16 })
        }

        fn push_delta(&mut self, id: &str, x: f64, y: f64) -> Result<(), BoardPointerPlanFault> {
            let index = usize::from(self.delta_len);
            if index == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            let id = self.push_id(id)?;
            self.deltas[index] = Some(BoardPointerDelta { id, x, y });
            self.delta_len += 1;
            Ok(())
        }

        fn id(&self, span: BoardPointerSpan) -> &str {
            let start = usize::from(span.start);
            let end = start + usize::from(span.len);
            std::str::from_utf8(&self.bytes[start..end]).expect("board pointer ids originate from UTF-8 strings")
        }

        fn push_point(&mut self, world: Point, screen: Point) -> Result<(), BoardPointerPlanFault> {
            let index = usize::from(self.point_len);
            if index == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            self.points[index] = world;
            self.screen_points[index] = screen;
            self.point_len += 1;
            Ok(())
        }

        fn selection_ids(&self) -> impl Iterator<Item = &str> {
            self.deltas[..usize::from(self.delta_len)].iter().flatten().map(|delta| self.id(delta.id))
        }

        pub fn event_count(&self) -> usize {
            match self.kind {
                BoardPointerPlanKind::FinishPan { .. } => 1,
                BoardPointerPlanKind::FinishDrag => usize::from(self.delta_len).min(1),
                BoardPointerPlanKind::SelectionPreview { .. } | BoardPointerPlanKind::SelectionCommit => 1,
                BoardPointerPlanKind::LinkMove { .. } => usize::from(self.output_len > 2),
                BoardPointerPlanKind::LinkFinish { .. } => usize::from(self.output_len > 2),
                BoardPointerPlanKind::LinkRetain { .. } => usize::from(self.output_len > 2),
                BoardPointerPlanKind::Hover { .. } => usize::from(self.output_len > 2),
                BoardPointerPlanKind::Brush { .. } => usize::from(self.output_len > 2),
                _ => 0,
            }
        }

        pub fn requires_retained_commit(&self) -> bool {
            !matches!(self.kind, BoardPointerPlanKind::Idle)
        }

        fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
            let mut output = String::with_capacity(BOARD_POINTER_BYTE_CAPACITY);
            self.write_events_json(&mut output)?;
            self.output[..output.len()].copy_from_slice(output.as_bytes());
            self.output_len = output.len() as u16;
            Ok(())
        }

        fn output_raw(&mut self, value: &str) -> Result<(), BoardPointerPlanFault> {
            let start = usize::from(self.output_len);
            let end = start.checked_add(value.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
            if end > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            self.output[start..end].copy_from_slice(value.as_bytes());
            self.output_len = end as u16;
            Ok(())
        }

        fn seal_owned_event(&mut self, event: &BoardOwnedEvent) -> Result<(), BoardPointerPlanFault> {
            self.output_len = 0;
            self.output_raw("[{\"name\":\"")?;
            self.output_raw(event.kind().name())?;
            self.output_raw("\",\"payload\":")?;
            self.output_raw(event.payload_json())?;
            self.output_raw("}]")
        }

        fn seal_owned_events(&mut self, events: &[BoardOwnedEvent]) -> Result<(), BoardPointerPlanFault> {
            self.output_len = 0;
            self.output_raw("[")?;
            for (index, event) in events.iter().enumerate() {
                if index > 0 {
                    self.output_raw(",")?;
                }
                self.output_raw("{\"name\":\"")?;
                self.output_raw(event.kind().name())?;
                self.output_raw("\",\"payload\":")?;
                self.output_raw(event.payload_json())?;
                self.output_raw("}")?;
            }
            self.output_raw("]")
        }

        fn seal_optional_events<const N: usize>(&mut self, events: &[Option<BoardOwnedEvent>; N]) -> Result<(), BoardPointerPlanFault> {
            self.output_len = 0;
            self.output_raw("[")?;
            let mut emitted = 0usize;
            for event in events.iter().flatten() {
                if emitted > 0 {
                    self.output_raw(",")?;
                }
                emitted += 1;
                self.output_raw("{\"name\":\"")?;
                self.output_raw(event.kind().name())?;
                self.output_raw("\",\"payload\":")?;
                self.output_raw(event.payload_json())?;
                self.output_raw("}")?;
            }
            self.output_raw("]")
        }

        pub fn events_json(&self) -> &str {
            std::str::from_utf8(&self.output[..usize::from(self.output_len)]).expect("board event page is encoded from UTF-8 schema tokens")
        }

        pub fn write_events_json(&self, output: &mut String) -> Result<(), BoardPointerPlanFault> {
            output.clear();
            match self.kind {
                BoardPointerPlanKind::FinishPan { camera } => {
                    output.push_str("[{\"name\":\"camera\",\"payload\":{");
                    output.push_str("\"x\":");
                    output.push_str(&camera[0].to_string());
                    output.push_str(",\"y\":");
                    output.push_str(&camera[1].to_string());
                    output.push_str(",\"zoom\":");
                    output.push_str(&camera[2].to_string());
                    output.push_str("}}]");
                }
                BoardPointerPlanKind::FinishDrag if self.delta_len > 0 => {
                    let mut admitted_bytes = 64usize;
                    for index in 0..usize::from(self.delta_len) {
                        let delta = self.deltas[index].expect("bounded board delta");
                        admitted_bytes = admitted_bytes.checked_add(board_json_string_bytes(self.id(delta.id))).and_then(|bytes| bytes.checked_add(96)).ok_or(BoardPointerPlanFault::ByteCredits)?;
                    }
                    if admitted_bytes > BOARD_POINTER_BYTE_CAPACITY {
                        return Err(BoardPointerPlanFault::ByteCredits);
                    }
                    output.push_str("[{\"name\":\"nodeDragEnd\",\"payload\":{\"moves\":[");
                    for index in 0..usize::from(self.delta_len) {
                        let delta = self.deltas[index].expect("bounded board delta");
                        if index > 0 {
                            output.push(',');
                        }
                        output.push_str("{\"id\":");
                        write_json_string(output, self.id(delta.id));
                        output.push_str(",\"x\":");
                        output.push_str(&delta.x.to_string());
                        output.push_str(",\"y\":");
                        output.push_str(&delta.y.to_string());
                        output.push('}');
                        if output.len() > BOARD_POINTER_BYTE_CAPACITY {
                            return Err(BoardPointerPlanFault::ByteCredits);
                        }
                    }
                    output.push_str("]}}]");
                }
                _ => output.push_str("[]"),
            }
            if output.len() > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            Ok(())
        }

        fn camera(&self) -> [f64; 3] {
            match self.kind {
                BoardPointerPlanKind::Pan { camera } | BoardPointerPlanKind::FinishPan { camera } => camera,
                _ => [0.0, 0.0, 1.0],
            }
        }
    }

    fn write_json_string(output: &mut String, value: &str) {
        output.push('"');
        for character in value.chars() {
            match character {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                character if character <= '\u{1f}' => {
                    use std::fmt::Write;
                    let _ = write!(output, "\\u{:04x}", character as u32);
                }
                character => output.push(character),
            }
        }
        output.push('"');
    }

    fn board_json_string_bytes(value: &str) -> usize {
        value
            .chars()
            .map(|character| match character {
                '"' | '\\' | '\n' | '\r' | '\t' => 2,
                character if character <= '\u{1f}' => 6,
                character => character.len_utf8(),
            })
            .try_fold(2usize, usize::checked_add)
            .unwrap_or(usize::MAX)
    }

    fn admitted_board_pointer_id(value: &str) -> Result<String, BoardEventFault> {
        if value.len() > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        Ok(value.to_owned())
    }

    fn board_node_move_owned_bytes(id: &str, x: f64, y: f64) -> Option<usize> {
        if id.len() > BOARD_EVENT_KEY_BYTE_CAPACITY || !x.is_finite() || !y.is_finite() {
            return None;
        }
        17usize.checked_add(board_json_string_bytes(id))?.checked_add(x.to_string().len())?.checked_add(y.to_string().len())?.checked_add(id.len())
    }

    fn board_edge_event_owned_bytes(id: &str, source: &str, target: &str) -> Option<usize> {
        let bytes = 27usize.checked_add(board_json_string_bytes(id))?.checked_add(board_json_string_bytes(source))?.checked_add(board_json_string_bytes(target))?;
        (bytes <= BOARD_EVENT_PAYLOAD_BYTE_CAPACITY).then_some(bytes)
    }

    impl BoardWheelPlan {
        pub fn camera(&self) -> [f64; 3] {
            [self.next.x, self.next.y, self.next.zoom]
        }
    }

    impl Default for BoardHost {
        fn default() -> Self {
            Self {
                camera: Camera::default(),
                nodes: BTreeMap::new(),
                handles: BTreeMap::new(),
                edges: BTreeMap::new(),
                wires: BTreeMap::new(),
                handle_kinds: BTreeMap::new(),
                wire_kinds: BTreeMap::new(),
                node_kinds: BTreeMap::new(),
                edge_kinds: BTreeMap::new(),
                edge_tips: builtin_edge_tips(),
                link_compat_rules: Vec::new(),
                selection: BTreeSet::new(),
                preselect: BTreeSet::new(),
                preselect_removed: BTreeSet::new(),
                selection_exit_highlight: BTreeSet::new(),
                selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
                hovered_id: None,
                hovered_kind: None,
                highlighted_ids: BTreeSet::new(),
                interaction: Interaction::None,
                width: 1,
                height: 1,
                dpr: 1.0,
                world_raster_tiling: "world-clip".into(),
                events: BoardEventQueue::default(),
                event_overflow: None,
                event_batch_overflow: None,
                event_schema_fault: false,
                selection_screen_preview: None,
                selection_preview_crossing: false,
                link_screen_preview: None,
                canvas_theme: CanvasPalette::default(),
                grid_factor: GRID_FACTOR_DEFAULT,
                grid_snap_enabled: false,
                preserve_original_element_style: false,
                automatic_lod: true,
                forced_draw_lod: None,
                icon_paint_cache: IconPaintCache::new(),
                link_compat_nodes_emit_key: None,
                link_target_ring_emit_key: None,
                last_select_emit_sig: None,
                last_preselect_emit_sig: None,
                content_scene_generation: 0,
                world_content_cache: RefCell::new(None),
                opaque_scene_fault: Cell::new(false),
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                active_utility: ActiveUtility::Select,
                suggestion_offset: DEFAULT_SUGGESTION_OFFSET,
                brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
                brush_slot_source_id: None,
                brush_candidates: BrushCandidatePage::default(),
                brush_candidate_index: 0,
                brush_preview: None,
                fixture_drop_preview: None,
                brush_candidates_emit_key: None,
                brush_preview_emit_key: None,
                brush_placement_serial: 0,
                brush_node_kind_weights: HashMap::new(),
                brush_handle_kind_weights: HashMap::new(),
                brush_alt_pressed: false,
                brush_slot_suggestions_active: false,
                port_mode: GraphPortMode::Ported,
                interaction_revision: 0,
                pending_delete_planning: None,
                pending_delete_operation: None,
                pending_pointer_commit: None,
                queued_pointer_commit: None,
                pointer_publication: None,
                close_phase: BoardHostClosePhase::Events,
                close_entity_retirement: None,
                close_strings: std::array::from_fn(|_| None),
                close_string_len: 0,
                close_node_handles: None,
            }
        }
    }

    impl BoardHost {
        /// @emoji 📶️ Draw LOD used while building the vector scene (pins during wheel zoom).
        fn draw_lod_for_frame(&self) -> BoardDrawLod {
            if self.wheel_zoom_active {
                if let Some(pinned) = self.wheel_zoom_render_lod {
                    return pinned;
                }
            }
            self.current_draw_lod()
        }

        fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
            lod.label()
        }

        /// @emoji 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
        pub fn overlay_paint_state_json(&self) -> String {
            let nodes: Vec<serde_json::Value> = self.nodes.values().filter(|n| n.visible).map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y })).collect();
            serde_json::json!({
                "camera": { "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom },
                "lod": Self::board_draw_lod_label(self.draw_lod_for_frame()),
                "nodes": nodes,
            })
            .to_string()
        }

        fn bump_content_scene_generation(&mut self) {
            self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
        }

        pub fn quarantine_world_content_step(&mut self) -> bool {
            let mut cache = self.world_content_cache.borrow_mut();
            if cache.is_none() {
                return true;
            }
            let Some(token) = crate::infinite::canvas::reserve_opaque_scene_retirement() else {
                self.opaque_scene_fault.set(true);
                return false;
            };
            let (_, _, scene) = cache.take().expect("world content cache was witnessed occupied");
            crate::infinite::canvas::publish_opaque_scene_retirement(token, scene);
            true
        }

        pub fn opaque_scene_faulted(&self) -> bool {
            self.opaque_scene_fault.get()
        }

        fn push_close_string(&mut self, value: String) {
            let index = usize::from(self.close_string_len);
            assert!(index < self.close_strings.len(), "board close string scratch is schema bounded");
            self.close_strings[index] = Some(value);
            self.close_string_len += 1;
        }

        fn push_close_optional_string(&mut self, value: Option<String>) {
            if let Some(value) = value {
                self.push_close_string(value);
            }
        }

        fn close_scratch_step(&mut self) -> bool {
            if self.close_node_handles.is_some() {
                let handle = self.close_node_handles.as_mut().and_then(Vec::pop);
                if let Some(handle) = handle {
                    self.push_close_string(handle.handle_kind);
                    return true;
                }
                self.close_node_handles = None;
                return true;
            }
            if self.close_string_len > 0 {
                self.close_string_len -= 1;
                drop(self.close_strings[usize::from(self.close_string_len)].take());
                return true;
            }
            false
        }

        fn close_interaction_step(&mut self) -> bool {
            match &mut self.interaction {
                Interaction::None | Interaction::Pan { .. } => {
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::DragNodes { primary_id, start_positions, proximity_pair, .. } => {
                    if let Some((id, _)) = start_positions.pop_first() {
                        drop(id);
                        return false;
                    }
                    if let Some((left, right)) = proximity_pair.take() {
                        if right.is_empty() {
                            drop(left);
                        } else {
                            drop(right);
                            *proximity_pair = Some((left, String::new()));
                        }
                        return false;
                    }
                    drop(std::mem::take(primary_id));
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::SelectionPending { initial_ids, .. } => {
                    if let Some(id) = initial_ids.pop_first() {
                        drop(id);
                        return false;
                    }
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::Selection { initial_ids, points, screen_points, .. } => {
                    if initial_ids.pop_first().is_some() || points.pop().is_some() || screen_points.pop().is_some() {
                        return false;
                    }
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::LinkAtSourceHandle { source_id, .. } => {
                    drop(std::mem::take(source_id));
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::LinkDragSnap { source_id, target_id, .. } => {
                    if let Some(target) = target_id.take() {
                        drop(target);
                        return false;
                    }
                    drop(std::mem::take(source_id));
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::LinkTargetNode { source_id, target_node_id } => {
                    if !target_node_id.is_empty() {
                        drop(std::mem::take(target_node_id));
                        return false;
                    }
                    drop(std::mem::take(source_id));
                    self.interaction = Interaction::None;
                    true
                }
                Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } => {
                    if let Some(id) = compatible_node_ids.pop().or_else(|| ring_handle_ids.pop()).or_else(|| ring_node_id.take()) {
                        drop(id);
                        return false;
                    }
                    drop(std::mem::take(source_id));
                    self.interaction = Interaction::None;
                    true
                }
            }
        }

        pub fn close_nonopaque_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
            if context.should_yield() {
                return false;
            }
            if self.close_scratch_step() {
                context.consume_fuel(1);
                return false;
            }
            if let Some(retirement) = self.close_entity_retirement.as_mut() {
                match retirement.step() {
                    Ok(true) => self.close_entity_retirement = None,
                    Ok(false) => {}
                    Err(()) => {
                        self.event_schema_fault = true;
                        return false;
                    }
                }
                context.consume_fuel(1);
                return false;
            }
            match self.close_phase {
                BoardHostClosePhase::Events => {
                    if self.close_event_authority_step(context) {
                        self.close_phase = BoardHostClosePhase::Pointer;
                    }
                    return false;
                }
                BoardHostClosePhase::Pointer => {
                    if self.close_pointer_authority_step(context) {
                        self.close_phase = BoardHostClosePhase::WorldScene;
                    }
                    return false;
                }
                BoardHostClosePhase::WorldScene => {
                    if self.quarantine_world_content_step() {
                        self.close_phase = BoardHostClosePhase::Icons;
                    }
                }
                BoardHostClosePhase::Icons => {
                    if self.icon_paint_cache.close_step() {
                        self.close_phase = BoardHostClosePhase::Nodes;
                    }
                }
                BoardHostClosePhase::Nodes => {
                    if let Some((key, value)) = self.nodes.pop_first() {
                        drop(key);
                        self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Node(value)));
                    } else {
                        self.close_phase = BoardHostClosePhase::Handles;
                    }
                }
                BoardHostClosePhase::Handles => {
                    if let Some((key, value)) = self.handles.pop_first() {
                        drop(key);
                        self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Handle(value)));
                    } else {
                        self.close_phase = BoardHostClosePhase::Edges;
                    }
                }
                BoardHostClosePhase::Edges => {
                    if let Some((key, value)) = self.edges.pop_first() {
                        drop(key);
                        self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Edge(value)));
                    } else {
                        self.close_phase = BoardHostClosePhase::Wires;
                    }
                }
                BoardHostClosePhase::Wires => {
                    if let Some((key, value)) = self.wires.pop_first() {
                        drop(key);
                        self.close_entity_retirement = Some(BoardEntityRetirement::new(BoardRemovedEntity::Wire(value)));
                    } else {
                        self.close_phase = BoardHostClosePhase::Selection;
                    }
                }
                BoardHostClosePhase::Selection => {
                    if self.selection.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::Preselect;
                    }
                }
                BoardHostClosePhase::Preselect => {
                    if self.preselect.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::PreselectRemoved;
                    }
                }
                BoardHostClosePhase::PreselectRemoved => {
                    if self.preselect_removed.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::SelectionExit;
                    }
                }
                BoardHostClosePhase::SelectionExit => {
                    if self.selection_exit_highlight.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::Highlighted;
                    }
                }
                BoardHostClosePhase::Highlighted => {
                    if self.highlighted_ids.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::Interaction;
                    }
                }
                BoardHostClosePhase::Interaction => {
                    if self.close_interaction_step() {
                        self.close_phase = BoardHostClosePhase::HandleKinds;
                    }
                }
                BoardHostClosePhase::HandleKinds => {
                    if let Some((key, value)) = self.handle_kinds.pop_first() {
                        self.push_close_string(key);
                        self.push_close_string(value.name);
                        self.push_close_optional_string(value.default_wire_kind);
                    } else {
                        self.close_phase = BoardHostClosePhase::WireKinds;
                    }
                }
                BoardHostClosePhase::WireKinds => {
                    if let Some((key, value)) = self.wire_kinds.pop_first() {
                        self.push_close_string(key);
                        self.push_close_string(value.name);
                        self.push_close_optional_string(value.default_edge_kind);
                    } else {
                        self.close_phase = BoardHostClosePhase::NodeKinds;
                    }
                }
                BoardHostClosePhase::NodeKinds => {
                    if let Some((key, value)) = self.node_kinds.pop_first() {
                        self.push_close_string(key);
                        self.push_close_string(value.name);
                        self.push_close_optional_string(value.icon);
                        self.close_node_handles = Some(value.handles);
                    } else {
                        self.close_phase = BoardHostClosePhase::EdgeKinds;
                    }
                }
                BoardHostClosePhase::EdgeKinds => {
                    if let Some((key, value)) = self.edge_kinds.pop_first() {
                        self.push_close_string(key);
                        self.push_close_string(value.name);
                        self.push_close_optional_string(value.source_tip);
                        self.push_close_optional_string(value.target_tip);
                    } else {
                        self.close_phase = BoardHostClosePhase::EdgeTips;
                    }
                }
                BoardHostClosePhase::EdgeTips => {
                    if self.edge_tips.pop_first().is_none() {
                        self.close_phase = BoardHostClosePhase::LinkRules;
                    }
                }
                BoardHostClosePhase::LinkRules => {
                    if let Some(rule) = self.link_compat_rules.pop() {
                        self.push_close_string(rule.source);
                        self.push_close_string(rule.target);
                    } else {
                        self.close_phase = BoardHostClosePhase::Previews;
                    }
                }
                BoardHostClosePhase::Previews => {
                    if self.selection_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) || self.link_screen_preview.as_mut().is_some_and(|values| values.pop().is_some()) {
                    } else if let Some(preview) = self.brush_preview.take() {
                        self.push_close_string(preview.source_handle_id);
                        self.push_close_string(preview.node_kind_id);
                        self.push_close_optional_string(preview.icon_kind);
                        self.close_node_handles = Some(preview.handles);
                    } else if let Some(preview) = self.fixture_drop_preview.take() {
                        self.push_close_string(preview.node_kind_id);
                        self.push_close_optional_string(preview.icon_kind);
                    } else {
                        self.selection_screen_preview = None;
                        self.link_screen_preview = None;
                        self.close_phase = BoardHostClosePhase::Strings;
                    }
                }
                BoardHostClosePhase::Strings => {
                    if let Some(value) = self
                        .hovered_id
                        .take()
                        .or_else(|| self.link_compat_nodes_emit_key.take())
                        .or_else(|| self.link_target_ring_emit_key.take())
                        .or_else(|| self.brush_slot_source_id.take())
                        .or_else(|| self.brush_candidates_emit_key.take())
                        .or_else(|| self.brush_preview_emit_key.take())
                    {
                        drop(value);
                    } else if let Some((left, right)) = self.hovered_kind.take() {
                        self.push_close_string(left);
                        self.push_close_string(right);
                    } else if let Some((values, mode)) = self.last_select_emit_sig.as_mut() {
                        if let Some(value) = values.pop().or_else(|| mode.take()) {
                            drop(value);
                        } else {
                            self.last_select_emit_sig = None;
                        }
                    } else if let Some((left, right, mode)) = self.last_preselect_emit_sig.as_mut() {
                        if let Some(value) = left.pop().or_else(|| right.pop()).or_else(|| mode.take()) {
                            drop(value);
                        } else {
                            self.last_preselect_emit_sig = None;
                        }
                    } else {
                        drop(std::mem::take(&mut self.world_raster_tiling));
                        let method = std::mem::take(&mut self.selection_options.method);
                        let mode = std::mem::take(&mut self.selection_options.mode);
                        self.push_close_string(method);
                        self.push_close_string(mode);
                        self.close_phase = BoardHostClosePhase::Weights;
                    }
                }
                BoardHostClosePhase::Weights => {
                    if self.brush_node_kind_weights.keys().next().cloned().and_then(|key| self.brush_node_kind_weights.remove_entry(&key)).is_some() {
                    } else if self.brush_handle_kind_weights.keys().next().cloned().and_then(|key| self.brush_handle_kind_weights.remove_entry(&key)).is_some() {
                    } else {
                        self.close_phase = BoardHostClosePhase::Done;
                    }
                }
                BoardHostClosePhase::Done => return self.nonopaque_terminal_is_empty(),
            }
            context.consume_fuel(1);
            false
        }

        pub fn nonopaque_terminal_is_empty(&self) -> bool {
            self.close_phase == BoardHostClosePhase::Done
                && self.event_authority_terminal_is_empty()
                && self.pointer_authority_terminal_is_empty()
                && self.close_entity_retirement.is_none()
                && self.close_string_len == 0
                && self.close_node_handles.is_none()
                && self.nodes.is_empty()
                && self.handles.is_empty()
                && self.edges.is_empty()
                && self.wires.is_empty()
                && self.handle_kinds.is_empty()
                && self.wire_kinds.is_empty()
                && self.node_kinds.is_empty()
                && self.edge_kinds.is_empty()
                && self.edge_tips.is_empty()
                && self.link_compat_rules.is_empty()
                && self.selection.is_empty()
                && self.preselect.is_empty()
                && self.preselect_removed.is_empty()
                && self.selection_exit_highlight.is_empty()
                && self.highlighted_ids.is_empty()
                && matches!(self.interaction, Interaction::None)
                && self.icon_paint_cache.terminal_is_empty()
                && self.world_content_cache.borrow().is_none()
        }

        #[doc(hidden)]
        pub fn test_content_scene_generation(&self) -> u64 {
            self.content_scene_generation
        }

        fn viewport(&self) -> infinite::canvas::camera::Viewport {
            infinite::canvas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
        }

        fn camera_content_affine(&self) -> Affine {
            infinite::canvas::camera::camera_content_affine(&self.camera, &self.viewport())
        }
    }

    impl BoardHost {
        fn grid_step_large_world(&self) -> f64 {
            GRID_WORLD_LARGE * self.grid_factor
        }
        fn grid_step_medium_world(&self) -> f64 {
            GRID_WORLD_MEDIUM * self.grid_factor
        }
        fn grid_step_small_world(&self) -> f64 {
            GRID_WORLD_SMALL * self.grid_factor
        }
        fn grid_step_micro_world(&self) -> f64 {
            GRID_WORLD_MICRO * self.grid_factor
        }

        pub fn new() -> Self {
            Self::default()
        }

        /// 🧠️ Normal directed graph host: no handles, edges reference node ids.
        pub fn new_normal() -> Self {
            let mut host = Self { port_mode: GraphPortMode::Normal, ..Self::default() };
            host.selection_options.select_handles = false;
            host
        }

        fn has_ports(&self) -> bool {
            self.port_mode.has_ports()
        }

        fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Option<Point> {
            let center = Point::new(node.x, node.y);
            match node.shape {
                NodeShape::Circle => {
                    let radius = self.scaled_node_radius(node);
                    let angle = circle_handle_angle_toward(center, toward);
                    Some(handle_position_on_circle(center, radius, angle))
                }
                NodeShape::Rectangle => {
                    let width = self.scaled_node_width(node);
                    let height = self.scaled_node_height(node);
                    let angle = rectangle_handle_angle_toward(center, width, height, toward);
                    Some(handle_position_on_rectangle(center, width, height, angle))
                }
            }
        }

        fn current_draw_lod(&self) -> BoardDrawLod {
            if !self.automatic_lod {
                if let Some(lod) = self.forced_draw_lod {
                    return lod;
                }
            }
            BoardDrawLod::from_scale_index(PUZZLE_2D_LOD_SCALE.resolve_index(self.camera.zoom))
        }

        fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
            match self.current_draw_lod() {
                BoardDrawLod::Minimap => None,
                BoardDrawLod::Overview | BoardDrawLod::Compact => Some(self.grid_step_large_world()),
                BoardDrawLod::Normal => Some(self.grid_step_medium_world()),
                BoardDrawLod::Detail => Some(self.grid_step_small_world()),
                BoardDrawLod::Micro => Some(self.grid_step_micro_world()),
            }
        }

        fn snap_world_scalar(&self, v: f64) -> f64 {
            if !self.grid_snap_enabled {
                return v;
            }
            let Some(step) = self.lod_visible_grid_snap_step_world() else {
                return v;
            };
            (v / step).round() * step
        }

        fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
            (self.snap_world_scalar(x), self.snap_world_scalar(y))
        }

        pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
            self.grid_snap_enabled = enabled;
        }

        pub fn set_automatic_lod(&mut self, enabled: bool) {
            self.automatic_lod = enabled;
            if enabled {
                self.forced_draw_lod = None;
            }
        }

        pub fn set_forced_draw_lod_label(&mut self, label: &str) {
            let t = label.trim();
            if t.is_empty() {
                self.forced_draw_lod = None;
                return;
            }
            self.forced_draw_lod = match BoardDrawLod::from_id(t) {
                Some(lod) => Some(lod),
                None => {
                    self.forced_draw_lod = None;
                    return;
                }
            };
        }

        pub fn set_grid_factor(&mut self, v: f64) -> Result<(), NormalPortError> {
            if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
                return Err(NormalPortError::GridFactorOutOfRange);
            }
            self.grid_factor = v;
            Ok(())
        }

        /// @emoji 🔗️ Applies or clears a host-driven link preview session (cross-surface mirror).
        pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::ExternalLinkPreviewJson)?;
            let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
            if source.is_empty() {
                if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                }
                return Ok(());
            }
            let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
            let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
            let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            self.interaction = Interaction::ExternalLinkPreview { source_id: source.clone(), end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
            self.sync_link_gesture_events();
            Ok(())
        }

        /// @emoji 🔗️ Clears host-driven link preview without touching local link drags.
        pub fn clear_external_link_preview(&mut self) {
            if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
        }

        fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<CachedIconPaintLease<'_>> {
            self.icon_paint_cache.get_or_build(encoded, fg, bg, preserve_original_style)
        }

        pub fn clear_icon_vector_cache(&mut self) {
            self.icon_paint_cache.clear();
        }

        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
        }

        pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
            self.set_camera_internal(x, y, zoom, true);
        }

        /// @emoji 🔇️ Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
        pub fn set_camera_silent(&mut self, x: f64, y: f64, zoom: f64) {
            self.set_camera_internal(x, y, zoom, false);
        }

        fn set_camera_internal(&mut self, x: f64, y: f64, zoom: f64, emit_event: bool) {
            let zoom = infinite::canvas::camera::clamp_zoom(zoom);
            if (self.camera.x - x).abs() < 1e-9 && (self.camera.y - y).abs() < 1e-9 && (self.camera.zoom - zoom).abs() < 1e-9 {
                return;
            }
            let reservation = if emit_event {
                let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::camera(x, y, zoom)) else {
                    return;
                };
                Some(reservation)
            } else {
                None
            };
            self.camera.x = x;
            self.camera.y = y;
            self.camera.zoom = zoom;
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
            if let Some(reservation) = reservation {
                self.publish_event_reservation(reservation);
            }
        }

        pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
            self.selection_options.method = method.into();
            self.selection_options.mode = normalize_selection_mode(mode);
            self.selection_options.select_nodes = select_nodes;
            self.selection_options.select_edges = select_edges;
            self.selection_options.select_handles = select_handles;
        }

        /// @emoji 🔗️ JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
        pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            let v: serde_json::Value = serde_json::from_str(json)?;
            let arr = v.as_array().ok_or(NormalPortError::CompatNotArray)?;
            let mut next = Vec::new();
            for row in arr {
                let o = row.as_object().ok_or(NormalPortError::RowNotObject("compat"))?;
                let source = o.get("source").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatSourceMissing)?.trim().to_string();
                let target = o.get("target").and_then(|x| x.as_str()).ok_or(NormalPortError::CompatTargetMissing)?.trim().to_string();
                let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
                let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
                let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
                let specificity = Self::parse_compat_specificity(spec_s)?;
                next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
            }
            self.link_compat_rules = next;
            Ok(())
        }

        fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, NormalPortError> {
            match raw.trim().to_ascii_lowercase().as_str() {
                "general" => Ok(CompatSpecificity::General),
                "node" => Ok(CompatSpecificity::Node),
                "edge" => Ok(CompatSpecificity::Edge),
                "handle" | "vortex" => Ok(CompatSpecificity::Handle),
                "wire" => Ok(CompatSpecificity::Wire),
                _ => Err(NormalPortError::InvalidCompatSpecificity(raw.to_string())),
            }
        }

        fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &'static str) -> Result<(), NormalPortError> {
            if row.contains_key("label") {
                return Err(NormalPortError::LegacyLabelField(slice));
            }
            Ok(())
        }

        /// @emoji 🧩️ JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
        pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.len() > BOARD_EVENT_BYTE_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            let v: serde_json::Value = serde_json::from_str(json)?;
            let o = v.as_object().ok_or(NormalPortError::KindCatalogsRootNotObject)?;
            for key in ["handleKinds", "wireKinds", "nodeKinds", "edgeTips", "edgeKinds"] {
                if o.get(key).and_then(serde_json::Value::as_array).is_some_and(|rows| rows.len() > BOARD_POINTER_ITEM_CAPACITY) {
                    return Err(NormalPortError::EventCredits);
                }
            }
            let template_count = o
                .get("nodeKinds")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_object)
                .filter_map(|row| row.get("handles"))
                .filter_map(serde_json::Value::as_array)
                .try_fold(0usize, |count, handles| count.checked_add(handles.len()))
                .ok_or(NormalPortError::EventCredits)?;
            if template_count > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let ho = row.as_object().ok_or(NormalPortError::RowNotObject("handle kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(ho, "handle")?;
                    let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("handle kind"))?;
                    let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::HandleKindColorMissing)?;
                    let color = Self::parse_css_color(color_s).ok_or_else(|| NormalPortError::InvalidHandleKindColor(color_s.to_string()))?;
                    let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
                }
                self.handle_kinds = next;
            }
            if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let wo = row.as_object().ok_or(NormalPortError::RowNotObject("wire kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(wo, "wire")?;
                    let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("wire kind"))?;
                    let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
                }
                self.wire_kinds = next;
            }
            if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let no = row.as_object().ok_or(NormalPortError::RowNotObject("node kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(no, "node")?;
                    let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("node kind"))?;
                    let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                        Some("rectangle") => NodeShape::Rectangle,
                        _ => NodeShape::Circle,
                    };
                    let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                    if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                        for row in arr {
                            let ho = row.as_object().ok_or(NormalPortError::RowNotObject("node kind handle"))?;
                            let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::NodeKindHandleKindMissing)?;
                            let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or(NormalPortError::NodeKindHandleAngleMissing)?;
                            let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                            handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                        }
                    }
                    let icon = no.get("icon").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let color_fill = no.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                    next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
                }
                self.node_kinds = next;
            }
            if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
                let mut tips = builtin_edge_tips();
                for row in arr {
                    let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge tip"))?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge tip"))?;
                    let def = EdgeTipDef::from_catalog_row(eo).ok_or_else(|| NormalPortError::EdgeTipRowInvalid(id.to_string()))?;
                    tips.insert(id.to_string(), def);
                }
                self.edge_tips = tips;
            }
            if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let eo = row.as_object().ok_or(NormalPortError::RowNotObject("edge kind"))?;
                    Self::reject_kind_catalog_row_legacy_label(eo, "edge")?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or(NormalPortError::IdMissing("edge kind"))?;
                    let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color = eo.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).and_then(Self::parse_css_hex_color);
                    let stroke_width = eo
                        .get("stroke")
                        .and_then(|x| x.as_f64())
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                        .unwrap_or(2.0);
                    let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                        Some("dashed") => EdgeStrokePattern::Dashed,
                        Some("dotted") => EdgeStrokePattern::Dotted,
                        _ => EdgeStrokePattern::Solid,
                    };
                    let source_tip = Self::parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                    let target_tip = Self::parse_catalog_tip_slot(eo.get("targetTip").or_else(|| eo.get("target_tip")).and_then(|x| x.as_str()).or_else(|| eo.get("marker").and_then(|x| x.as_str())));
                    let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                    next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
                }
                self.edge_kinds = next;
            }
            Ok(())
        }

        /// @emoji 🛡️ Ensures runtime catalogs declare every kind from a compile-time manifest.
        pub fn validate_against_manifest_id(&self, manifest_id: &str) -> Result<(), NormalPortError> {
            let gm = manifest_by_id(manifest_id).ok_or_else(|| NormalPortError::UnknownManifestId(manifest_id.to_string()))?;
            for row in &gm.port_kinds {
                let visual = row.presentation.as_ref().is_some_and(|p| p.get("color").is_some());
                if visual && !self.handle_kinds.contains_key(&row.id) {
                    return Err(NormalPortError::CatalogMissingKind("handle", row.id.clone()));
                }
            }
            for row in &gm.wire_kinds {
                if !self.wire_kinds.contains_key(&row.id) {
                    return Err(NormalPortError::CatalogMissingKind("wire", row.id.clone()));
                }
            }
            for row in &gm.edge_kinds {
                if row.presentation.is_some() && !self.edge_kinds.contains_key(&row.id) {
                    return Err(NormalPortError::CatalogMissingKind("edge", row.id.clone()));
                }
            }
            for row in &gm.node_kinds {
                if row.id == "Piece" {
                    continue;
                }
                if !self.node_kinds.contains_key(&row.id) {
                    return Err(NormalPortError::CatalogMissingKind("node", row.id.clone()));
                }
            }
            Ok(())
        }

        fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
            let s = value?.trim();
            if s.is_empty() || s.eq_ignore_ascii_case("none") {
                Some(String::new())
            } else {
                Some(s.to_string())
            }
        }

        fn lookup_edge_tip<'a>(&'a self, id: &str) -> Option<&'a EdgeTipDef> {
            if id.is_empty() {
                return None;
            }
            self.edge_tips.get(id)
        }

        fn resolve_tip_slot<'a>(&'a self, slot: Option<&str>) -> Option<&'a EdgeTipDef> {
            match slot {
                Some("") => None,
                Some(id) => self.lookup_edge_tip(id),
                None => None,
            }
        }

        fn parse_css_hex_color(s: &str) -> Option<Color> {
            let s = s.trim();
            let hex = s.strip_prefix('#')?;
            match hex.len() {
                3 => {
                    let mut full = String::new();
                    for ch in hex.chars() {
                        full.push(ch);
                        full.push(ch);
                    }
                    let v = u32::from_str_radix(&full, 16).ok()?;
                    let r = ((v >> 16) & 0xff) as u8;
                    let g = ((v >> 8) & 0xff) as u8;
                    let b = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, 255))
                }
                6 => {
                    let v = u32::from_str_radix(hex, 16).ok()?;
                    let r = ((v >> 16) & 0xff) as u8;
                    let g = ((v >> 8) & 0xff) as u8;
                    let b = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, 255))
                }
                8 => {
                    let v = u32::from_str_radix(hex, 16).ok()?;
                    let r = ((v >> 24) & 0xff) as u8;
                    let g = ((v >> 16) & 0xff) as u8;
                    let b = ((v >> 8) & 0xff) as u8;
                    let a = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, a))
                }
                _ => None,
            }
        }

        /// @emoji 🎨️ Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
        fn parse_css_color(s: &str) -> Option<Color> {
            if let Some(c) = Self::parse_css_hex_color(s) {
                return Some(c);
            }
            Self::parse_css_hsl_color(s)
        }

        fn parse_css_hsl_color(s: &str) -> Option<Color> {
            let low = s.trim().to_ascii_lowercase();
            let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
                (true, inner)
            } else {
                let inner = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')'))?;
                (false, inner)
            };
            let inner = inner.trim();
            let (main, alpha_slash) = inner.split_once('/').map(|(a, b)| (a.trim(), Some(b.trim()))).unwrap_or((inner, None));
            let normalized = main.replace(',', " ");
            let parts: Vec<&str> = normalized.split_whitespace().collect();
            if parts.len() < 3 {
                return None;
            }
            let h = Self::parse_css_hsl_hue(parts[0])?;
            let sat = Self::parse_css_hsl_sl(parts[1])?;
            let light = Self::parse_css_hsl_sl(parts[2])?;
            let alpha = if let Some(a) = alpha_slash {
                Self::parse_css_alpha_channel(a)?
            } else if legacy_alpha_form && parts.len() >= 4 {
                Self::parse_css_alpha_channel(parts[3])?
            } else {
                1.0
            };
            let (r, g, b) = Self::hsl_to_rgb_bytes(h, sat, light);
            let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
            Some(Color::from_rgba8(r, g, b, a))
        }

        fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
            let t = tok.trim();
            let n = t.strip_suffix("deg").map(str::trim).unwrap_or(t);
            let v: f64 = n.parse().ok()?;
            v.is_finite().then_some(v)
        }

        fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
            let t = tok.trim();
            if let Some(p) = t.strip_suffix('%') {
                let v: f64 = p.trim().parse().ok()?;
                Some((v / 100.0).clamp(0.0, 1.0))
            } else {
                let v: f64 = t.parse().ok()?;
                Some(v.clamp(0.0, 1.0))
            }
        }

        fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
            Self::parse_css_hsl_sl(tok)
        }

        fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 0.5 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
            let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
            let s = s.clamp(0.0, 1.0);
            let l = l.clamp(0.0, 1.0);
            if s <= f64::EPSILON {
                let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
                return (v, v, v);
            }
            let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
            let p = 2.0 * l - q;
            let r = Self::board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
            let g = Self::board_hsl_hue_to_rgb_component(p, q, h_norm);
            let b = Self::board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
            ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
        }

        fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
            match style.map(str::trim).filter(|s| !s.is_empty()) {
                Some("original") => Some(BoardElementStyleKind::Original),
                Some("neutral") => Some(BoardElementStyleKind::Neutral),
                Some("hovered") => Some(BoardElementStyleKind::Hovered),
                Some("selected") => Some(BoardElementStyleKind::Selected),
                Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
                Some("disabled") => Some(BoardElementStyleKind::Disabled),
                _ => None,
            }
        }

        fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
            if self.is_preselect_active() {
                return None;
            }
            if self.selection.contains(id) {
                return None;
            }
            if self.hovered_id.as_deref() == Some(id) {
                return Some(BoardElementStyleKind::Hovered);
            }
            if self.hovered_id.is_none() {
                if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                    if hover_domain == domain && hover_kind == element_kind {
                        return Some(BoardElementStyleKind::Hovered);
                    }
                }
            }
            None
        }

        fn highlighted_style_kind(&self, id: &str) -> Option<BoardElementStyleKind> {
            if self.is_preselect_active() || self.selection.contains(id) {
                return None;
            }
            if self.hovered_id.as_deref() == Some(id) {
                return None;
            }
            if self.highlighted_ids.contains(id) {
                return Some(BoardElementStyleKind::Highlighted);
            }
            None
        }

        fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
            if let Some(node) = self.nodes.get(id) {
                return Some(("node".to_string(), node.node_kind.clone()));
            }
            if let Some(handle) = self.handles.get(id) {
                return Some(("handle".to_string(), handle.handle_kind.clone()));
            }
            if let Some(edge) = self.edges.get(id) {
                return Some(("edge".to_string(), edge.edge_kind.clone()));
            }
            if let Some(wire) = self.wires.get(id) {
                return Some(("wire".to_string(), wire.wire_kind.clone()));
            }
            None
        }

        fn ids_matching_kind_hover(&self) -> Vec<String> {
            let Some((domain, kind_id)) = self.hovered_kind.as_ref() else {
                return Vec::new();
            };
            let mut ids = Vec::new();
            match domain.as_str() {
                "node" => {
                    for node in self.nodes.values() {
                        if &node.node_kind == kind_id && !self.selection.contains(&node.id) {
                            ids.push(node.id.clone());
                        }
                    }
                }
                "handle" => {
                    for handle in self.handles.values() {
                        if &handle.handle_kind == kind_id && !self.selection.contains(&handle.id) {
                            ids.push(handle.id.clone());
                        }
                    }
                }
                "edge" => {
                    for edge in self.edges.values() {
                        if &edge.edge_kind == kind_id && !self.selection.contains(&edge.id) {
                            ids.push(edge.id.clone());
                        }
                    }
                }
                "wire" => {
                    for wire in self.wires.values() {
                        if &wire.wire_kind == kind_id && !self.selection.contains(&wire.id) {
                            ids.push(wire.id.clone());
                        }
                    }
                }
                _ => {}
            }
            ids
        }

        fn is_preselect_active(&self) -> bool {
            self.is_preselecting() || !self.preselect.is_empty()
        }

        /// @emoji 🎨️ During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
        fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
            if self.is_preselect_active() {
                if self.preselect.contains(id) {
                    return BoardElementStyleKind::Selected;
                }
                if self.selection.contains(id) {
                    return BoardElementStyleKind::Highlighted;
                }
                return BoardElementStyleKind::Neutral;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            if self.highlighted_ids.contains(id) {
                return BoardElementStyleKind::Highlighted;
            }
            BoardElementStyleKind::Neutral
        }

        fn locked_style_dim(kind: BoardElementStyleKind, locked: bool) -> BoardElementStyleKind {
            if locked && !matches!(kind, BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Hovered) {
                BoardElementStyleKind::Disabled
            } else {
                kind
            }
        }

        fn resolve_node_style_kind(&self, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(n.style.as_deref()) {
                return Self::locked_style_dim(kind, n.locked);
            }
            let kind = match pass {
                StyleChromePass::CachedBase => {
                    if self.preserve_original_element_style {
                        BoardElementStyleKind::Original
                    } else {
                        BoardElementStyleKind::Neutral
                    }
                }
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                        return Self::locked_style_dim(kind, n.locked);
                    }
                    if let Some(kind) = self.highlighted_style_kind(n.id.as_str()) {
                        return Self::locked_style_dim(kind, n.locked);
                    }
                    self.resolve_interaction_style_kind(n.id.as_str())
                }
            };
            Self::locked_style_dim(kind, n.locked)
        }

        fn resolve_handle_style_kind(&self, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(h.style.as_deref()) {
                return Self::locked_style_dim(kind, h.locked);
            }
            let kind = match pass {
                StyleChromePass::CachedBase => {
                    if self.preserve_original_element_style {
                        BoardElementStyleKind::Original
                    } else {
                        BoardElementStyleKind::Neutral
                    }
                }
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                        return Self::locked_style_dim(kind, h.locked);
                    }
                    if let Some(kind) = self.highlighted_style_kind(h.id.as_str()) {
                        return Self::locked_style_dim(kind, h.locked);
                    }
                    self.resolve_interaction_style_kind(h.id.as_str())
                }
            };
            Self::locked_style_dim(kind, h.locked)
        }

        fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(e.style.as_deref()) {
                return Self::locked_style_dim(kind, e.locked);
            }
            let kind = match pass {
                StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                        return Self::locked_style_dim(kind, e.locked);
                    }
                    if let Some(kind) = self.highlighted_style_kind(e.id.as_str()) {
                        return Self::locked_style_dim(kind, e.locked);
                    }
                    self.resolve_interaction_style_kind(e.id.as_str())
                }
            };
            Self::locked_style_dim(kind, e.locked)
        }

        fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(w.style.as_deref()) {
                return Self::locked_style_dim(kind, w.locked);
            }
            let kind = match pass {
                StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                        return Self::locked_style_dim(kind, w.locked);
                    }
                    if let Some(kind) = self.highlighted_style_kind(w.id.as_str()) {
                        return Self::locked_style_dim(kind, w.locked);
                    }
                    self.resolve_interaction_style_kind(w.id.as_str())
                }
            };
            Self::locked_style_dim(kind, w.locked)
        }

        /// @emoji 💠️ Entity ids whose selection/preselect/hover chrome tints fills and strokes without rebuilding {@link BoardHost.world_content_cache}.
        fn interaction_overlay_entity_ids(&self) -> BTreeSet<String> {
            let mut ids = BTreeSet::new();
            if self.is_preselect_active() {
                ids.extend(self.preselect.iter().cloned());
                ids.extend(self.selection.iter().cloned());
                ids.extend(self.preselect_removed.iter().cloned());
            } else {
                ids.extend(self.selection.iter().cloned());
                ids.extend(self.selection_exit_highlight.iter().cloned());
            }
            if let Some(ref hover_id) = self.hovered_id {
                if !self.is_preselect_active() && !self.selection.contains(hover_id) {
                    ids.insert(hover_id.clone());
                }
            }
            if self.hovered_id.is_none() && !self.is_preselect_active() {
                for id in self.ids_matching_kind_hover() {
                    ids.insert(id);
                }
            }
            for id in self.highlighted_ids.iter() {
                if !self.selection.contains(id) && self.hovered_id.as_deref() != Some(id.as_str()) {
                    ids.insert(id.clone());
                }
            }
            ids
        }

        fn chrome_pass_for_entity(&self, entity_id: &str, overlay_ids: &BTreeSet<String>) -> StyleChromePass {
            if overlay_ids.contains(entity_id) {
                StyleChromePass::InteractionOverlay
            } else {
                StyleChromePass::CachedBase
            }
        }

        fn node_fill_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.node_fill_hovered,
                BoardElementStyleKind::Selected => theme.node_fill_selected,
                BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
                BoardElementStyleKind::Disabled => theme.node_fill_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
            }
        }

        fn node_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
                BoardElementStyleKind::Selected => theme.node_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
            }
        }

        fn resolve_handle_fill_color(&self, h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
                BoardElementStyleKind::Selected => theme.handle_fill_selected,
                BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
                BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                    if let Some(c) = h.color_fill {
                        return c;
                    }
                    if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
                        return def.color;
                    }
                    theme.handle_fill
                }
            }
        }

        fn resolve_handle_stroke_color(&self, _h: &HandleData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
                BoardElementStyleKind::Selected => theme.handle_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
            }
        }

        fn edge_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
                BoardElementStyleKind::Selected => theme.edge_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
            }
        }

        fn lerp_color(a: Color, b: Color, t: f64) -> Color {
            let t = t.clamp(0.0, 1.0);
            let ac = a.to_rgba8();
            let bc = b.to_rgba8();
            Color::from_rgba8(
                (f64::from(ac.r) * (1.0 - t) + f64::from(bc.r) * t).round() as u8,
                (f64::from(ac.g) * (1.0 - t) + f64::from(bc.g) * t).round() as u8,
                (f64::from(ac.b) * (1.0 - t) + f64::from(bc.b) * t).round() as u8,
                (f64::from(ac.a) * (1.0 - t) + f64::from(bc.a) * t).round() as u8,
            )
        }

        fn resolve_node_fill_color(&self, n: &NodeData, theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            let theme_fill = Self::node_fill_for_style(theme, kind);
            match kind {
                BoardElementStyleKind::Hovered | BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Disabled => theme_fill,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                    let kind_id = n.node_kind.trim();
                    if kind_id.is_empty() {
                        return theme_fill;
                    }
                    self.node_kinds.get(kind_id).and_then(|def| def.color_fill).unwrap_or(theme_fill)
                }
            }
        }

        fn edge_stroke_for_kind_pattern(pattern: EdgeStrokePattern, width: f64) -> Stroke {
            use crate::infinite::canvas::Cap;
            let mut stroke = Stroke::new(width);
            match pattern {
                EdgeStrokePattern::Solid => {}
                EdgeStrokePattern::Dashed => {
                    stroke.set_dash_pattern(vec![width * 3.0, width * 2.0]);
                }
                EdgeStrokePattern::Dotted => {
                    stroke.set_dash_pattern(vec![width * 0.35, width * 1.65]);
                    stroke.set_start_cap(Cap::Round);
                    stroke.set_end_cap(Cap::Round);
                }
            }
            stroke
        }

        fn resolve_edge_stroke_paint(&self, e: &EdgeData, chrome_pass: StyleChromePass, lod: BoardDrawLod, lod_scale_width: f64) -> (Color, Stroke, f64) {
            let style_kind = self.resolve_edge_style_kind(e, chrome_pass);
            let chrome = Self::edge_stroke_for_style(&self.canvas_theme, style_kind);
            let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
            let base_color = kind_def.and_then(|d| d.color).unwrap_or(self.canvas_theme.edge_stroke);
            let stroke_color = match style_kind {
                BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
                _ if lod == BoardDrawLod::Minimap => chrome,
                _ => Self::lerp_color(base_color, chrome, 0.55),
            };
            let catalog_w = kind_def.map(|d| d.stroke_width).unwrap_or(2.0);
            let width_mult = match style_kind {
                BoardElementStyleKind::Selected => ui_styling::strokes::EDGE_SELECTED_MULT,
                BoardElementStyleKind::Hovered => ui_styling::strokes::EDGE_HOVERED_MULT,
                _ => 1.0,
            };
            let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
            let pattern = kind_def.map(|d| d.pattern).unwrap_or(EdgeStrokePattern::Solid);
            (stroke_color, Self::edge_stroke_for_kind_pattern(pattern, width), width)
        }

        fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
            let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
            let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
            let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
            let source = self.resolve_tip_slot(source_slot);
            let mut target = self.resolve_tip_slot(target_slot);
            if target.is_none() && target_slot.is_none() {
                let directed = kind_def.map(|d| d.directed).unwrap_or(true);
                if directed {
                    target = self.lookup_edge_tip("arrow");
                }
            }
            (source, target)
        }

        fn append_edge_tip(scene: &mut Scene, tip: Point, dir: Vec2, color: Color, stroke_width: f64, tip_def: &EdgeTipDef) {
            use crate::infinite::canvas::BezPath;
            let len = dir.hypot();
            if len < 1e-9 {
                return;
            }
            let d = dir / len;
            let n = Vec2::new(-d.y, d.x);
            let sw = stroke_width.max(1.0) * tip_def.scale.max(0.25);
            match tip_def.geometry {
                EdgeTipGeometry::Arrow => {
                    let length = sw * 4.2;
                    let half_w = sw * 1.15;
                    let base = tip - d * length;
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(base + n * half_w);
                    path.line_to(base - n * half_w);
                    path.close_path();
                    if tip_def.filled {
                        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                    }
                }
                EdgeTipGeometry::FineArrow => {
                    let length = sw * 3.2;
                    let half_w = sw * 0.75;
                    let base = tip - d * length;
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(base + n * half_w);
                    path.move_to(tip);
                    path.line_to(base - n * half_w);
                    let outline = Stroke::new((sw * 0.9).max(1.0));
                    scene.stroke(&outline, Affine::IDENTITY, color, None, &path);
                }
                EdgeTipGeometry::Diamond => {
                    let length = sw * 3.6;
                    let half_w = sw * 1.05;
                    let back = tip - d * length;
                    let mid = tip - d * (length * 0.5);
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(mid + n * half_w);
                    path.line_to(back);
                    path.line_to(mid - n * half_w);
                    path.close_path();
                    if tip_def.filled {
                        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &path);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                    }
                }
                EdgeTipGeometry::Circle => {
                    let r = sw * 1.4;
                    let center = tip - d * r;
                    let circle = Circle::new(center, r);
                    if tip_def.filled {
                        scene.fill(FillRule::NonZero, Affine::IDENTITY, color, None, &circle);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &circle);
                    }
                }
                EdgeTipGeometry::Bar => {
                    let half = sw * ui_styling::strokes::EDGE_TIP_MIN;
                    let center = tip - d * (sw * 0.5);
                    let mut path = BezPath::new();
                    path.move_to(center + n * half);
                    path.line_to(center - n * half);
                    scene.stroke(&Stroke::new(sw.max(ui_styling::strokes::EDGE_TIP_MIN)), Affine::IDENTITY, color, None, &path);
                }
            }
        }

        fn append_edge_tips_on_curve(scene: &mut Scene, curve: &CubicBez, color: Color, stroke_w: f64, source: Option<&EdgeTipDef>, target: Option<&EdgeTipDef>) {
            let inset = stroke_w * 0.35;
            if let Some(tip_def) = target {
                let mut tangent = curve.p3() - curve.p2();
                if tangent.hypot() < 1e-9 {
                    tangent = curve.p3() - curve.p1();
                }
                if tangent.hypot() >= 1e-9 {
                    let dir = tangent / tangent.hypot();
                    let tip = curve.p3() - dir * inset;
                    Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
                }
            }
            if let Some(tip_def) = source {
                let mut tangent = curve.p0() - curve.p1();
                if tangent.hypot() < 1e-9 {
                    tangent = curve.p0() - curve.p2();
                }
                if tangent.hypot() >= 1e-9 {
                    let dir = tangent / tangent.hypot();
                    let tip = curve.p0() - dir * inset;
                    Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
                }
            }
        }

        fn wire_stroke_for_style(theme: &CanvasPalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
                BoardElementStyleKind::Selected => theme.wire_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
                BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
            }
        }

        fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
            if !Self::handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
                return false;
            }
            if self.link_compat_rules.is_empty() {
                return true;
            }
            let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies(rule, source, target)).collect();
            if matched.is_empty() {
                return false;
            }
            if matched.iter().any(|r| r.important) {
                matched.retain(|r| r.important);
            } else {
                let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
                matched.retain(|r| (r.specificity as i32) == max_rank);
            }
            !matched.is_empty()
        }

        fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
            if rule.source == a && rule.target == b {
                return true;
            }
            if rule.bidirectional && rule.source == b && rule.target == a {
                return true;
            }
            false
        }

        fn handle_port_shape(handle_kind: &str) -> Option<&'static str> {
            if handle_kind.contains(" circular ") {
                Some("circular")
            } else if handle_kind.contains(" rectangular ") {
                Some("rectangular")
            } else {
                None
            }
        }

        fn handle_port_shapes_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
            match (Self::handle_port_shape(source_handle_kind), Self::handle_port_shape(target_handle_kind)) {
                (Some(a), Some(b)) => a == b,
                _ => true,
            }
        }

        fn single_letter_port_family(handle_kind: &str) -> Option<char> {
            let head = handle_kind.split('-').next()?;
            if head.len() == 1 {
                head.chars().next().filter(|c| c.is_ascii_lowercase())
            } else {
                None
            }
        }

        fn single_letter_port_families_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
            match (Self::single_letter_port_family(source_handle_kind), Self::single_letter_port_family(target_handle_kind)) {
                (Some(a), Some(b)) => a == b,
                _ => true,
            }
        }

        fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
            self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
        }

        fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
            self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
        }

        fn link_gesture_rule_applies(&self, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
            let w_src = self.resolve_default_wire_kind_for_handle(source);
            let w_tgt = self.resolve_default_wire_kind_for_handle(target);
            let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
            let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
            let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let tn = self.nodes.get(&target.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let sh = source.handle_kind.as_str();
            let th = target.handle_kind.as_str();
            match rule.specificity {
                CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
                CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
                CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src.as_str(), th),
            }
        }

        fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
            let wk = self.resolve_default_wire_kind_for_handle(source);
            self.resolve_default_edge_kind_for_wire_kind(&wk)
        }

        fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
            self.handle_kinds.get(handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
        }

        fn board_fill_default_wire_kind<'a>(&'a self, handle_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
            let Some(value) = self.handle_kinds.get(handle_kind).and_then(|kind| kind.default_wire_kind.as_deref()) else {
                return Ok(DEFAULT_WIRE_KIND_ID);
            };
            if value.len() > BOARD_FILL_TEXT_BYTES {
                return Err(BoardFillCaptureFault::TextCapacity);
            }
            let value = value.trim();
            Ok(if value.is_empty() { DEFAULT_WIRE_KIND_ID } else { value })
        }

        fn board_fill_default_edge_kind<'a>(&'a self, wire_kind: &str) -> Result<&'a str, BoardFillCaptureFault> {
            let Some(value) = self.wire_kinds.get(wire_kind).and_then(|kind| kind.default_edge_kind.as_deref()) else {
                return Ok("");
            };
            if value.len() > BOARD_FILL_TEXT_BYTES {
                return Err(BoardFillCaptureFault::TextCapacity);
            }
            Ok(value.trim())
        }

        fn link_gesture_rule_applies_kind_strings(&self, rule: &LinkCompatRule, sn: &str, sh: &str, w_src: &str, e_src: &str, tn: &str, th: &str, _w_tgt: &str, e_tgt: &str) -> bool {
            match rule.specificity {
                CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
                CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src, e_tgt),
                CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src, th),
            }
        }

        fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
            if !Self::handle_port_shapes_compatible(sh, th) {
                return false;
            }
            if !Self::single_letter_port_families_compatible(sh, th) {
                return false;
            }
            if self.link_compat_rules.is_empty() {
                return true;
            }
            let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
            let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
            let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
            let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
            let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt)).collect();
            if matched.is_empty() {
                return false;
            }
            if matched.iter().any(|r| r.important) {
                matched.retain(|r| r.important);
            } else {
                let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
                matched.retain(|r| (r.specificity as i32) == max_rank);
            }
            !matched.is_empty()
        }

        fn brush_slot_hit_radius_world(&self) -> f64 {
            (self.brush_node_size * 0.5).max(1.0)
        }

        /// @emoji 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
        fn brush_handle_anchor_world(&self, h: &HandleData) -> Option<Point> {
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                self.indirect_handle_world_pos(h).or_else(|| self.handle_world_pos(h))
            } else {
                self.handle_world_pos(h)
            }
        }

        fn brush_effective_suggestion_offset(&self) -> f64 {
            if self.brush_alt_pressed || self.brush_slot_suggestions_active {
                self.suggestion_offset
            } else {
                0.0
            }
        }

        fn handle_slot_center_world(&self, node_id: &str, hw: Point, offset: f64) -> Option<Point> {
            let n = self.nodes.get(node_id)?;
            let nc = Point::new(n.x, n.y);
            let normal = normalize_or_zero(hw - nc);
            Some(hw + normal * offset)
        }

        fn brush_slot_center_world(&self, h: &HandleData) -> Option<Point> {
            let hw = self.brush_handle_anchor_world(h)?;
            self.handle_slot_center_world(h.node_id.as_str(), hw, self.brush_effective_suggestion_offset())
        }

        /// @emoji 🖌️ World distance from pointer to brush slot when the pointer is on the slot, anchor, or sole-free node body.
        fn brush_slot_pointer_hit_distance(&self, world: Point, handle_id: &str, h: &HandleData) -> Option<f64> {
            let slot_center = self.brush_slot_center_world(h)?;
            let zoom = self.camera.zoom.max(1e-9);
            let slot_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.brush_slot_hit_radius_world();
            let d_slot = distance_between(world, slot_center);
            if d_slot <= slot_hit_r {
                return Some(d_slot);
            }
            let anchor = self.brush_handle_anchor_world(h)?;
            let anchor_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h).max(self.effective_handle_radius(h));
            if distance_between(world, anchor) <= anchor_hit_r {
                return Some(d_slot);
            }
            if self.sole_eligible_indirect_handle_on_node(&h.node_id).as_deref() == Some(handle_id) {
                let n = self.nodes.get(&h.node_id)?;
                if self.point_in_node_world(n, world) {
                    return Some(d_slot);
                }
            }
            None
        }

        fn brush_nearest_slot_source(&self, world: Point) -> Option<String> {
            let mut best: Option<(f64, String)> = None;
            for (hid, h) in &self.handles {
                if !self.handle_effectively_visible(hid.as_str()) || self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                let Some(d) = self.brush_slot_pointer_hit_distance(world, hid.as_str(), h) else {
                    continue;
                };
                if best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                    best = Some((d, hid.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        fn brush_compatible_candidates(&self, source: &HandleData) -> Option<BrushCandidatePage> {
            let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let sh = source.handle_kind.as_str();
            let mut out = BrushCandidatePage::default();
            for (kind_id, kind) in &self.node_kinds {
                if kind.handles.is_empty() {
                    continue;
                }
                let tn = kind_id.as_str();
                for (i, tmpl) in kind.handles.iter().enumerate() {
                    if self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                        let delta = Self::brush_handle_alignment_delta(source.angle, tmpl.angle);
                        out.push(kind_id, i, delta).ok()?;
                    }
                }
            }
            out.sort();
            Some(out)
        }

        fn brush_handle_alignment_delta(source_handle_angle: f64, target_template_angle: f64) -> f64 {
            let desired = source_handle_angle + std::f64::consts::PI;
            let mut d = (target_template_angle - desired).abs();
            if d > std::f64::consts::PI {
                d = std::f64::consts::TAU - d;
            }
            d
        }

        fn brush_template_world_pos(&self, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, angle: f64) -> Point {
            match shape {
                NodeShape::Circle => handle_position_on_circle(center, radius, angle),
                NodeShape::Rectangle => handle_position_on_rectangle(center, width, height, angle),
            }
        }

        fn brush_build_preview(&self, source_handle_id: &str, candidate: BrushCandidateRef<'_>) -> Option<BrushPreviewSnapshot> {
            self.brush_build_preview_with_offset(source_handle_id, candidate, self.brush_effective_suggestion_offset())
        }

        fn brush_build_preview_with_offset(&self, source_handle_id: &str, candidate: BrushCandidateRef<'_>, offset: f64) -> Option<BrushPreviewSnapshot> {
            let source = self.handles.get(source_handle_id)?;
            let kind = self.node_kinds.get(candidate.node_kind_id)?;
            let center = self.handle_slot_center_world(&source.node_id, self.brush_handle_anchor_world(source)?, offset)?;
            let target_handle_index = candidate.target_handle_index;
            kind.handles.get(target_handle_index)?;
            let node_kind_id = candidate.node_kind_id;
            let radius = self.brush_node_size * 0.5 * kind.scale;
            let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
            Some(BrushPreviewSnapshot {
                source_handle_id: source_handle_id.to_string(),
                node_kind_id: node_kind_id.to_string(),
                x: center.x,
                y: center.y,
                shape: kind.shape,
                radius,
                width,
                height,
                handles: kind.handles.clone(),
                target_handle_index,
                icon_kind: kind.icon.clone(),
            })
        }

        fn brush_sync_preview_events(&mut self) {
            if self.brush_candidates.len() > BOARD_POINTER_ITEM_CAPACITY || self.brush_preview.as_ref().is_some_and(|preview| preview.handles.len() > BOARD_POINTER_ITEM_CAPACITY) {
                self.event_schema_fault = true;
                return;
            }
            let key = self.brush_preview.as_ref().map(|p| format!("{}|{}|{}|{}|{}", p.source_handle_id, p.node_kind_id, p.target_handle_index, p.x, p.y)).unwrap_or_default();
            let mut candidates_key = format!("{}|", self.brush_slot_source_id.as_deref().unwrap_or(""));
            for (index, candidate) in self.brush_candidates.iter().enumerate() {
                if index > 0 {
                    candidates_key.push(',');
                }
                candidates_key.push_str(candidate.node_kind_id);
                candidates_key.push('#');
                candidates_key.push_str(&candidate.target_handle_index.to_string());
            }
            candidates_key.push('|');
            candidates_key.push_str(&self.brush_candidate_index.to_string());
            let preview_changed = self.brush_preview_emit_key.as_deref() != Some(key.as_str());
            let candidates_changed = self.brush_candidates_emit_key.as_deref() != Some(candidates_key.as_str());
            let preview_event = preview_changed.then(|| BoardOwnedEvent::brush_preview(self.brush_preview.as_ref()));
            let candidate_event = candidates_changed.then(|| BoardOwnedEvent::brush_candidates(self.brush_slot_source_id.as_deref().unwrap_or(""), &self.brush_candidates, self.brush_candidate_index, self.brush_slot_suggestions_active));
            let (first, second) = match (preview_event, candidate_event) {
                (Some(first), second) => (first, second),
                (None, Some(first)) => (first, None),
                (None, None) => return,
            };
            let Some(reservation) = self.reserve_owned_batch(first, second) else {
                return;
            };
            if preview_changed {
                self.brush_preview_emit_key = Some(key);
            }
            if candidates_changed {
                self.brush_candidates_emit_key = Some(candidates_key);
            }
            self.publish_event_batch(reservation);
        }

        fn brush_clear_slot(&mut self) {
            let had_preview = self.brush_preview.is_some();
            let clear_hover = self.brush_slot_suggestions_active;
            self.brush_slot_suggestions_active = false;
            self.brush_slot_source_id = None;
            self.brush_candidates.clear();
            self.brush_candidate_index = 0;
            self.brush_preview = None;
            if clear_hover {
                self.set_hovered_id(None);
            }
            if had_preview {
                self.bump_content_scene_generation();
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.brush_sync_preview_events();
            }
        }

        fn brush_commit_preview(&mut self) {
            let Some(preview) = self.brush_preview.as_ref() else {
                return;
            };
            let serial = self.brush_placement_serial.wrapping_add(1);
            let node_id = format!("puzzle2d.brush.{serial}");
            let edge_id = format!("puzzle2d.brush.edge.{serial}");
            let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::brush_place(preview, &node_id, &edge_id)) else {
                return;
            };
            self.brush_placement_serial = serial;
            let _ = self.brush_preview.take();
            self.publish_event_reservation(reservation);
            self.bump_content_scene_generation();
            self.brush_preview_emit_key = None;
        }

        fn brush_finish_slot(&mut self) {
            if self.brush_alt_pressed {
                self.brush_commit_preview();
            }
            self.brush_clear_slot();
        }

        fn brush_update_alt(&mut self, alt: bool) {
            if self.brush_alt_pressed == alt {
                return;
            }
            self.brush_alt_pressed = alt;
            if self.brush_slot_source_id.is_some() {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        //#region 🪣️Fill
        /// 📸️ Begins a fixed-page snapshot capture without traversing a host collection.
        pub fn begin_board_fill_snapshot(&self) -> BoardFillSnapshotCapture {
            BoardFillSnapshotCapture::new(self.suggestion_offset)
        }

        //#endregion 🪣️Fill

        //#region 🧵️FillJob
    }

    impl BoardFillSnapshotCapture {
        fn new(suggestion_offset: f64) -> Self {
            Self {
                snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),
                phase: BoardFillCapturePhase::Nodes,
                last_key: None,
                node: None,
                handle: None,
                kind: None,
                kind_handle_cursor: 0,
                rule: None,
                rule_cursor: 0,
                closing: false,
            }
        }

        fn next_map_entry<'a, T>(&self, values: &'a BTreeMap<String, T>) -> Option<(&'a String, &'a T)> {
            Self::next_map_entry_after(self.last_key, values)
        }

        fn next_map_entry_after<'a, T>(last_key: Option<BoardFillText>, values: &'a BTreeMap<String, T>) -> Option<(&'a String, &'a T)> {
            use std::ops::Bound::{Excluded, Unbounded};
            match last_key {
                Some(last) => values.range::<str, _>((Excluded(last.as_str()), Unbounded)).next(),
                None => values.iter().next(),
            }
        }

        fn capture_node(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
            if let Some(current) = self.node.as_mut() {
                if current.bounds.is_none() {
                    let node = host.nodes.get(current.id.as_str()).ok_or(BoardFillCaptureFault::StaleNode)?;
                    let bounds = host.node_world_bounds(node, 0.0);
                    current.bounds = Some([bounds.min_x, bounds.min_y, bounds.max_x, bounds.max_y]);
                    return Ok(());
                }
                let completed = self.node.take().ok_or(BoardFillCaptureFault::NodeCapacity)?;
                let Some(bounds) = completed.bounds else {
                    self.node = Some(completed);
                    return Err(BoardFillCaptureFault::StaleNode);
                };
                let id = completed.id;
                if let Err(node) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::NodeCapacity)?.nodes.try_push_owned(BoardFillNodeSnapshot { id, bounds }) {
                    self.node = Some(BoardFillNodeCapture { id: node.id, bounds: Some(node.bounds) });
                    return Err(BoardFillCaptureFault::NodeCapacity);
                }
                self.last_key = Some(id);
                return Ok(());
            }
            let Some((key, _)) = self.next_map_entry(&host.nodes) else {
                self.phase = BoardFillCapturePhase::Handles;
                self.last_key = None;
                return Ok(());
            };
            let id = BoardFillText::try_from_str(key)?;
            self.node = Some(BoardFillNodeCapture { id, bounds: None });
            Ok(())
        }

        fn capture_handle(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
            if let Some(current) = self.handle.as_mut() {
                match current.stage {
                    BoardFillHandleCaptureStage::NodeKind => {
                        let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        let node = host.nodes.get(&handle.node_id).ok_or(BoardFillCaptureFault::StaleNode)?;
                        current.handle.node_kind = BoardFillText::try_from_str(&node.node_kind)?;
                        current.stage = BoardFillHandleCaptureStage::HandleKind;
                    }
                    BoardFillHandleCaptureStage::HandleKind => {
                        let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        current.handle.handle_kind = BoardFillText::try_from_str(&handle.handle_kind)?;
                        current.stage = BoardFillHandleCaptureStage::Slot;
                    }
                    BoardFillHandleCaptureStage::Slot => {
                        let handle = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        let anchor = host.brush_handle_anchor_world(handle).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        let slot = host.handle_slot_center_world(&handle.node_id, anchor, host.suggestion_offset).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        current.handle.slot = [slot.x, slot.y];
                        current.stage = BoardFillHandleCaptureStage::WireKind;
                    }
                    BoardFillHandleCaptureStage::WireKind => {
                        let wire = host.board_fill_default_wire_kind(current.handle.handle_kind.as_str())?;
                        current.handle.wire_kind = BoardFillText::try_from_str(wire)?;
                        current.stage = BoardFillHandleCaptureStage::EdgeKind;
                    }
                    BoardFillHandleCaptureStage::EdgeKind => {
                        let edge = host.board_fill_default_edge_kind(current.handle.wire_kind.as_str())?;
                        current.handle.edge_kind = BoardFillText::try_from_str(edge)?;
                        current.stage = BoardFillHandleCaptureStage::Visible;
                    }
                    BoardFillHandleCaptureStage::Visible => {
                        let _ = host.handles.get(current.handle.id.as_str()).ok_or(BoardFillCaptureFault::StaleHandle)?;
                        current.handle.visible = host.handle_effectively_visible(current.handle.id.as_str());
                        current.stage = BoardFillHandleCaptureStage::Connected;
                        current.edge_key = None;
                    }
                    BoardFillHandleCaptureStage::Connected => {
                        if let Some((edge_key, edge)) = Self::next_map_entry_after(current.edge_key, &host.edges) {
                            current.edge_key = Some(BoardFillText::try_from_str(edge_key)?);
                            if edge.source == current.handle.id.as_str() || edge.target == current.handle.id.as_str() {
                                current.handle.connected = true;
                                current.stage = BoardFillHandleCaptureStage::Weight;
                            }
                        } else {
                            current.stage = BoardFillHandleCaptureStage::Weight;
                        }
                    }
                    BoardFillHandleCaptureStage::Weight => {
                        current.handle.weight = host.brush_handle_kind_weights.get(current.handle.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                        current.stage = BoardFillHandleCaptureStage::Publish;
                    }
                    BoardFillHandleCaptureStage::Publish => {
                        let completed = self.handle.take().ok_or(BoardFillCaptureFault::HandleCapacity)?;
                        let id = completed.handle.id;
                        if let Err(handle) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::HandleCapacity)?.handles.try_push_owned(completed.handle) {
                            self.handle = Some(BoardFillHandleCapture { handle, edge_key: completed.edge_key, stage: BoardFillHandleCaptureStage::Publish });
                            return Err(BoardFillCaptureFault::HandleCapacity);
                        }
                        self.last_key = Some(id);
                    }
                }
                return Ok(());
            }
            let Some((key, _)) = self.next_map_entry(&host.handles) else {
                self.phase = BoardFillCapturePhase::Kinds;
                self.last_key = None;
                return Ok(());
            };
            let id = BoardFillText::try_from_str(key)?;
            self.handle = Some(BoardFillHandleCapture {
                handle: BoardFillHandleSnapshot {
                    id,
                    node_kind: BoardFillText::empty(),
                    handle_kind: BoardFillText::empty(),
                    wire_kind: BoardFillText::empty(),
                    edge_kind: BoardFillText::empty(),
                    slot: [0.0; 2],
                    visible: false,
                    connected: false,
                    weight: 0.0,
                },
                edge_key: None,
                stage: BoardFillHandleCaptureStage::NodeKind,
            });
            Ok(())
        }

        fn capture_kind(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
            if let Some(current) = self.kind.as_mut() {
                let source = host.node_kinds.get(current.value.id.as_str()).ok_or(BoardFillCaptureFault::StaleKind)?;
                match current.stage {
                    BoardFillKindCaptureStage::Icon => {
                        current.value.icon = source.icon.as_deref().map(BoardFillText::try_from_str).transpose()?;
                        current.stage = BoardFillKindCaptureStage::Shape;
                    }
                    BoardFillKindCaptureStage::Shape => {
                        current.value.shape = if source.shape == NodeShape::Rectangle { BoardFillShape::Rectangle } else { BoardFillShape::Circle };
                        current.stage = BoardFillKindCaptureStage::Radius;
                    }
                    BoardFillKindCaptureStage::Radius => {
                        current.value.radius = host.brush_node_size * 0.5 * source.scale;
                        current.stage = BoardFillKindCaptureStage::Width;
                    }
                    BoardFillKindCaptureStage::Width => {
                        current.value.width = if source.shape == NodeShape::Rectangle { host.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                        current.stage = BoardFillKindCaptureStage::Height;
                    }
                    BoardFillKindCaptureStage::Height => {
                        current.value.height = if source.shape == NodeShape::Rectangle { host.brush_node_size * source.scale } else { current.value.radius * 2.0 };
                        current.stage = BoardFillKindCaptureStage::Weight;
                    }
                    BoardFillKindCaptureStage::Weight => {
                        current.value.weight = host.brush_node_kind_weights.get(current.value.id.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                        current.stage = BoardFillKindCaptureStage::Templates;
                    }
                    BoardFillKindCaptureStage::Templates => {
                        if let Some(template) = current.template.as_mut() {
                            let source_template = source.handles.get(self.kind_handle_cursor).ok_or(BoardFillCaptureFault::StaleKind)?;
                            match template.stage {
                                BoardFillTemplateCaptureStage::HandleKind => {
                                    template.value.handle_kind = BoardFillText::try_from_str(&source_template.handle_kind)?;
                                    template.stage = BoardFillTemplateCaptureStage::WireKind;
                                }
                                BoardFillTemplateCaptureStage::WireKind => {
                                    let wire = host.board_fill_default_wire_kind(template.value.handle_kind.as_str())?;
                                    template.value.wire_kind = BoardFillText::try_from_str(wire)?;
                                    template.stage = BoardFillTemplateCaptureStage::EdgeKind;
                                }
                                BoardFillTemplateCaptureStage::EdgeKind => {
                                    let edge = host.board_fill_default_edge_kind(template.value.wire_kind.as_str())?;
                                    template.value.edge_kind = BoardFillText::try_from_str(edge)?;
                                    template.stage = BoardFillTemplateCaptureStage::Angle;
                                }
                                BoardFillTemplateCaptureStage::Angle => {
                                    template.value.angle = source_template.angle;
                                    template.stage = BoardFillTemplateCaptureStage::Radius;
                                }
                                BoardFillTemplateCaptureStage::Radius => {
                                    template.value.radius = source_template.radius;
                                    template.stage = BoardFillTemplateCaptureStage::Weight;
                                }
                                BoardFillTemplateCaptureStage::Weight => {
                                    template.value.weight = host.brush_handle_kind_weights.get(template.value.handle_kind.as_str()).copied().filter(|value| value.is_finite() && *value > 0.0).unwrap_or(1.0);
                                    template.stage = BoardFillTemplateCaptureStage::Publish;
                                }
                                BoardFillTemplateCaptureStage::Publish => {
                                    let completed = current.template.take().ok_or(BoardFillCaptureFault::KindHandleCapacity)?;
                                    if let Err(value) = current.value.handles.try_push_owned(completed.value) {
                                        current.template = Some(BoardFillTemplateCapture { value, stage: BoardFillTemplateCaptureStage::Publish });
                                        return Err(BoardFillCaptureFault::KindHandleCapacity);
                                    }
                                    self.kind_handle_cursor += 1;
                                }
                            }
                        } else if source.handles.get(self.kind_handle_cursor).is_some() {
                            current.template = Some(BoardFillTemplateCapture {
                                value: BoardFillTemplateSnapshot { handle_kind: BoardFillText::empty(), wire_kind: BoardFillText::empty(), edge_kind: BoardFillText::empty(), angle: 0.0, radius: None, weight: 0.0 },
                                stage: BoardFillTemplateCaptureStage::HandleKind,
                            });
                        } else {
                            current.stage = BoardFillKindCaptureStage::Publish;
                        }
                    }
                    BoardFillKindCaptureStage::Publish => {
                        let completed = self.kind.take().ok_or(BoardFillCaptureFault::KindCapacity)?;
                        let id = completed.value.id;
                        let template = completed.template;
                        if let Err(value) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::KindCapacity)?.kinds.try_push_owned(completed.value) {
                            self.kind = Some(BoardFillKindCapture { value, stage: BoardFillKindCaptureStage::Publish, template });
                            return Err(BoardFillCaptureFault::KindCapacity);
                        }
                        self.last_key = Some(id);
                        self.kind_handle_cursor = 0;
                    }
                }
                return Ok(());
            }
            let Some((key, _)) = self.next_map_entry(&host.node_kinds) else {
                self.phase = BoardFillCapturePhase::Rules;
                self.last_key = None;
                return Ok(());
            };
            let id = BoardFillText::try_from_str(key)?;
            self.kind = Some(BoardFillKindCapture {
                value: BoardFillKindSnapshot {
                    id,
                    shape: BoardFillShape::Circle,
                    radius: 0.0,
                    width: 0.0,
                    height: 0.0,
                    icon: None,
                    weight: 0.0,
                    handles: BoardFillFixedPages::try_new().map_err(|_| BoardFillCaptureFault::KindHandleCapacity)?,
                },
                stage: BoardFillKindCaptureStage::Icon,
                template: None,
            });
            Ok(())
        }

        fn capture_rule(&mut self, host: &BoardHost) -> Result<(), BoardFillCaptureFault> {
            if let Some(current) = self.rule.as_mut() {
                let source = host.link_compat_rules.get(self.rule_cursor).ok_or(BoardFillCaptureFault::StaleRule)?;
                match current.stage {
                    BoardFillRuleCaptureStage::Target => {
                        current.value.target = BoardFillText::try_from_str(&source.target)?;
                        current.stage = BoardFillRuleCaptureStage::Bidirectional;
                    }
                    BoardFillRuleCaptureStage::Bidirectional => {
                        current.value.bidirectional = source.bidirectional;
                        current.stage = BoardFillRuleCaptureStage::Specificity;
                    }
                    BoardFillRuleCaptureStage::Specificity => {
                        current.value.specificity = source.specificity;
                        current.stage = BoardFillRuleCaptureStage::Publish;
                    }
                    BoardFillRuleCaptureStage::Publish => {
                        let completed = self.rule.take().ok_or(BoardFillCaptureFault::RuleCapacity)?;
                        if let Err(value) = self.snapshot.as_mut().ok_or(BoardFillCaptureFault::RuleCapacity)?.rules.try_push_owned(completed.value) {
                            self.rule = Some(BoardFillRuleCapture { value, stage: BoardFillRuleCaptureStage::Publish });
                            return Err(BoardFillCaptureFault::RuleCapacity);
                        }
                        self.rule_cursor += 1;
                    }
                }
                return Ok(());
            }
            let Some(source) = host.link_compat_rules.get(self.rule_cursor) else {
                self.phase = BoardFillCapturePhase::Complete;
                return Ok(());
            };
            self.rule = Some(BoardFillRuleCapture {
                value: BoardFillRuleSnapshot { source: BoardFillText::try_from_str(&source.source)?, target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::General },
                stage: BoardFillRuleCaptureStage::Target,
            });
            Ok(())
        }

        pub fn step(&mut self, host: &BoardHost) -> BoardFillCaptureStep {
            if self.closing {
                return BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted);
            }
            let result = match self.phase {
                BoardFillCapturePhase::Nodes => self.capture_node(host),
                BoardFillCapturePhase::Handles => self.capture_handle(host),
                BoardFillCapturePhase::Kinds => self.capture_kind(host),
                BoardFillCapturePhase::Rules => self.capture_rule(host),
                BoardFillCapturePhase::Complete => return BoardFillCaptureStep::Complete,
                BoardFillCapturePhase::Closing => return BoardFillCaptureStep::Fault(BoardFillCaptureFault::GenerationExhausted),
            };
            match result {
                Ok(()) if self.phase == BoardFillCapturePhase::Complete => BoardFillCaptureStep::Complete,
                Ok(()) => BoardFillCaptureStep::Pending,
                Err(fault) => BoardFillCaptureStep::Fault(fault),
            }
        }

        pub fn take_snapshot(&mut self) -> Option<BoardFillSnapshot> {
            if self.phase != BoardFillCapturePhase::Complete || self.node.is_some() || self.handle.is_some() || self.kind.is_some() || self.rule.is_some() {
                return None;
            }
            self.closing = true;
            self.phase = BoardFillCapturePhase::Closing;
            self.snapshot.take()
        }

        pub fn begin_close(&mut self) {
            self.closing = true;
            self.phase = BoardFillCapturePhase::Closing;
        }

        pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.begin_close();
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.node.take().is_some() || self.handle.take().is_some() || self.rule.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(kind) = self.kind.as_mut() {
                if kind.template.take().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if kind.value.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = kind.value.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = kind.value.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
                self.kind = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            let Some(snapshot) = self.snapshot.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
            if let Some(kind) = snapshot.kinds.len.checked_sub(1).and_then(|index| snapshot.kinds.get_mut(index)) {
                if kind.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = kind.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = kind.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
            }
            if snapshot.kinds.pop().is_some() || snapshot.rules.pop().is_some() || snapshot.handles.pop().is_some() || snapshot.nodes.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            macro_rules! retire_snapshot_page {
                ($owners:expr) => {
                    if let Some(bytes) = $owners.retained_page_bytes() {
                        if bytes > maximum_bytes {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        let Some(released_bytes) = $owners.retire_empty_page() else {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        };
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                    }
                };
            }
            retire_snapshot_page!(snapshot.kinds);
            retire_snapshot_page!(snapshot.rules);
            retire_snapshot_page!(snapshot.handles);
            retire_snapshot_page!(snapshot.nodes);
            self.snapshot = None;
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.closing && self.snapshot.is_none() && self.node.is_none() && self.handle.is_none() && self.kind.is_none() && self.rule.is_none()
        }
    }

    impl Drop for BoardFillSnapshotCapture {
        fn drop(&mut self) {
            assert!(self.terminal_is_empty(), "Puzzle2d fill capture must reach exact terminal-empty before Drop");
        }
    }

    impl BoardFillSnapshotIngress {
        pub fn new(suggestion_offset: f64) -> Self {
            Self {
                snapshot: Some(BoardFillSnapshot { nodes: BoardFillFixedPages::new(), handles: BoardFillFixedPages::new(), kinds: BoardFillFixedPages::new(), rules: BoardFillFixedPages::new(), suggestion_offset }),
                node: None,
                handle: None,
                kind: None,
                template: None,
                rule: None,
                closing: false,
            }
        }

        pub fn begin_node(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.node.is_some() {
                return Err(BoardFillCaptureFault::StaleNode);
            }
            self.node = Some(BoardFillNodeSnapshot { id: BoardFillText::empty(), bounds: [0.0; 4] });
            Ok(())
        }

        pub fn push_node_id_byte(&mut self, byte: u8) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleNode);
            }
            self.node.as_mut().ok_or(BoardFillCaptureFault::StaleNode)?.id.try_push_byte(byte)
        }

        pub fn set_node_bound(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !value.is_finite() {
                return Err(BoardFillCaptureFault::StaleNode);
            }
            let bound = self.node.as_mut().and_then(|node| node.bounds.get_mut(index)).ok_or(BoardFillCaptureFault::StaleNode)?;
            *bound = value;
            Ok(())
        }

        pub fn publish_node(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleNode);
            }
            let node = self.node.take().ok_or(BoardFillCaptureFault::NodeCapacity)?;
            let Some(snapshot) = self.snapshot.as_mut() else {
                self.node = Some(node);
                return Err(BoardFillCaptureFault::NodeCapacity);
            };
            if let Err(node) = snapshot.nodes.try_push_owned(node) {
                self.node = Some(node);
                return Err(BoardFillCaptureFault::NodeCapacity);
            }
            Ok(())
        }

        pub fn begin_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.handle.is_some() {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            self.handle = Some(BoardFillHandleSnapshot {
                id: BoardFillText::empty(),
                node_kind: BoardFillText::empty(),
                handle_kind: BoardFillText::empty(),
                wire_kind: BoardFillText::empty(),
                edge_kind: BoardFillText::empty(),
                slot: [0.0; 2],
                visible: false,
                connected: false,
                weight: 0.0,
            });
            Ok(())
        }

        pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            let handle = self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle)?;
            match field {
                BoardFillIngressHandleText::Id => handle.id.try_push_byte(byte),
                BoardFillIngressHandleText::NodeKind => handle.node_kind.try_push_byte(byte),
                BoardFillIngressHandleText::HandleKind => handle.handle_kind.try_push_byte(byte),
                BoardFillIngressHandleText::WireKind => handle.wire_kind.try_push_byte(byte),
                BoardFillIngressHandleText::EdgeKind => handle.edge_kind.try_push_byte(byte),
            }
        }

        pub fn set_handle_slot(&mut self, index: usize, value: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !value.is_finite() {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            let slot = self.handle.as_mut().and_then(|handle| handle.slot.get_mut(index)).ok_or(BoardFillCaptureFault::StaleHandle)?;
            *slot = value;
            Ok(())
        }

        pub fn set_handle_visible(&mut self, visible: bool) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.visible = visible)
        }

        pub fn set_handle_connected(&mut self, connected: bool) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.connected = connected)
        }

        pub fn set_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !weight.is_finite() || weight <= 0.0 {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            self.handle.as_mut().ok_or(BoardFillCaptureFault::StaleHandle).map(|handle| handle.weight = weight)
        }

        pub fn publish_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleHandle);
            }
            let handle = self.handle.take().ok_or(BoardFillCaptureFault::HandleCapacity)?;
            let Some(snapshot) = self.snapshot.as_mut() else {
                self.handle = Some(handle);
                return Err(BoardFillCaptureFault::HandleCapacity);
            };
            if let Err(handle) = snapshot.handles.try_push_owned(handle) {
                self.handle = Some(handle);
                return Err(BoardFillCaptureFault::HandleCapacity);
            }
            Ok(())
        }

        pub fn begin_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.kind.is_some() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind = Some(BoardFillKindSnapshot {
                id: BoardFillText::empty(),
                shape: BoardFillShape::Circle,
                radius: 0.0,
                width: 0.0,
                height: 0.0,
                icon: None,
                weight: 0.0,
                handles: BoardFillFixedPages::try_new().map_err(|_| BoardFillCaptureFault::KindHandleCapacity)?,
            });
            Ok(())
        }

        pub fn begin_kind_icon(&mut self) -> Result<(), BoardFillCaptureFault> {
            let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
            if self.closing || kind.icon.is_some() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            kind.icon = Some(BoardFillText::empty());
            Ok(())
        }

        pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            let kind = self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
            match field {
                BoardFillIngressKindText::Id => kind.id.try_push_byte(byte),
                BoardFillIngressKindText::Icon => kind.icon.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?.try_push_byte(byte),
            }
        }

        pub fn set_kind_rectangle(&mut self, rectangle: bool) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.shape = if rectangle { BoardFillShape::Rectangle } else { BoardFillShape::Circle })
        }

        pub fn set_kind_radius(&mut self, radius: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !radius.is_finite() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.radius = radius)
        }

        pub fn set_kind_width(&mut self, width: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !width.is_finite() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.width = width)
        }

        pub fn set_kind_height(&mut self, height: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !height.is_finite() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.height = height)
        }

        pub fn set_kind_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !weight.is_finite() || weight <= 0.0 {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.kind.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|kind| kind.weight = weight)
        }

        pub fn begin_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.kind.is_none() || self.template.is_some() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.template = Some(BoardFillTemplateSnapshot { handle_kind: BoardFillText::empty(), wire_kind: BoardFillText::empty(), edge_kind: BoardFillText::empty(), angle: 0.0, radius: None, weight: 0.0 });
            Ok(())
        }

        pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            let template = self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind)?;
            match field {
                BoardFillIngressTemplateText::HandleKind => template.handle_kind.try_push_byte(byte),
                BoardFillIngressTemplateText::WireKind => template.wire_kind.try_push_byte(byte),
                BoardFillIngressTemplateText::EdgeKind => template.edge_kind.try_push_byte(byte),
            }
        }

        pub fn set_kind_handle_angle(&mut self, angle: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !angle.is_finite() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.angle = angle)
        }

        pub fn set_kind_handle_radius(&mut self, radius: Option<f64>) -> Result<(), BoardFillCaptureFault> {
            if self.closing || radius.is_some_and(|radius| !radius.is_finite()) {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.radius = radius)
        }

        pub fn set_kind_handle_weight(&mut self, weight: f64) -> Result<(), BoardFillCaptureFault> {
            if self.closing || !weight.is_finite() || weight <= 0.0 {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            self.template.as_mut().ok_or(BoardFillCaptureFault::StaleKind).map(|template| template.weight = weight)
        }

        pub fn publish_kind_handle(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            let template = self.template.take().ok_or(BoardFillCaptureFault::KindHandleCapacity)?;
            let Some(kind) = self.kind.as_mut() else {
                self.template = Some(template);
                return Err(BoardFillCaptureFault::StaleKind);
            };
            if let Err(template) = kind.handles.try_push_owned(template) {
                self.template = Some(template);
                return Err(BoardFillCaptureFault::KindHandleCapacity);
            }
            Ok(())
        }

        pub fn publish_kind(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.template.is_some() {
                return Err(BoardFillCaptureFault::StaleKind);
            }
            let kind = self.kind.take().ok_or(BoardFillCaptureFault::StaleKind)?;
            let Some(snapshot) = self.snapshot.as_mut() else {
                self.kind = Some(kind);
                return Err(BoardFillCaptureFault::KindCapacity);
            };
            if let Err(kind) = snapshot.kinds.try_push_owned(kind) {
                self.kind = Some(kind);
                return Err(BoardFillCaptureFault::KindCapacity);
            }
            Ok(())
        }

        pub fn begin_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing || self.rule.is_some() {
                return Err(BoardFillCaptureFault::StaleRule);
            }
            self.rule = Some(BoardFillRuleSnapshot { source: BoardFillText::empty(), target: BoardFillText::empty(), bidirectional: false, specificity: CompatSpecificity::Handle });
            Ok(())
        }

        pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleRule);
            }
            let rule = self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule)?;
            match field {
                BoardFillIngressRuleText::Source => rule.source.try_push_byte(byte),
                BoardFillIngressRuleText::Target => rule.target.try_push_byte(byte),
            }
        }

        pub fn set_rule_bidirectional(&mut self, bidirectional: bool) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleRule);
            }
            self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.bidirectional = bidirectional)
        }

        pub fn set_rule_specificity(&mut self, specificity: &str) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleRule);
            }
            let specificity = match specificity {
                "general" => CompatSpecificity::General,
                "node" => CompatSpecificity::Node,
                "edge" => CompatSpecificity::Edge,
                "wire" => CompatSpecificity::Wire,
                "handle" | "vortex" => CompatSpecificity::Handle,
                _ => return Err(BoardFillCaptureFault::StaleRule),
            };
            self.rule.as_mut().ok_or(BoardFillCaptureFault::StaleRule).map(|rule| rule.specificity = specificity)
        }

        pub fn publish_rule(&mut self) -> Result<(), BoardFillCaptureFault> {
            if self.closing {
                return Err(BoardFillCaptureFault::StaleRule);
            }
            let rule = self.rule.take().ok_or(BoardFillCaptureFault::RuleCapacity)?;
            let Some(snapshot) = self.snapshot.as_mut() else {
                self.rule = Some(rule);
                return Err(BoardFillCaptureFault::RuleCapacity);
            };
            if let Err(rule) = snapshot.rules.try_push_owned(rule) {
                self.rule = Some(rule);
                return Err(BoardFillCaptureFault::RuleCapacity);
            }
            Ok(())
        }

        pub fn take_snapshot(&mut self) -> Option<BoardFillSnapshot> {
            if self.closing || self.node.is_some() || self.handle.is_some() || self.kind.is_some() || self.template.is_some() || self.rule.is_some() {
                return None;
            }
            self.closing = true;
            self.snapshot.take()
        }

        pub fn begin_close(&mut self) {
            self.closing = true;
        }

        pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.begin_close();
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.node.take().is_some() || self.handle.take().is_some() || self.template.take().is_some() || self.rule.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(kind) = self.kind.as_mut() {
                if kind.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = kind.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = kind.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
                self.kind = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            let Some(snapshot) = self.snapshot.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
            if let Some(kind) = snapshot.kinds.len.checked_sub(1).and_then(|index| snapshot.kinds.get_mut(index)) {
                if kind.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = kind.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = kind.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
            }
            if snapshot.kinds.pop().is_some() || snapshot.rules.pop().is_some() || snapshot.handles.pop().is_some() || snapshot.nodes.pop().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            macro_rules! retire_ingress_page {
                ($owners:expr) => {
                    if let Some(bytes) = $owners.retained_page_bytes() {
                        if bytes > maximum_bytes {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        let Some(released_bytes) = $owners.retire_empty_page() else {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        };
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                    }
                };
            }
            retire_ingress_page!(snapshot.kinds);
            retire_ingress_page!(snapshot.rules);
            retire_ingress_page!(snapshot.handles);
            retire_ingress_page!(snapshot.nodes);
            self.snapshot = None;
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        pub fn terminal_is_empty(&self) -> bool {
            self.closing && self.snapshot.is_none() && self.node.is_none() && self.handle.is_none() && self.kind.is_none() && self.template.is_none() && self.rule.is_none()
        }
    }

    impl Drop for BoardFillSnapshotIngress {
        fn drop(&mut self) {
            assert!(self.terminal_is_empty(), "Puzzle2d artifact fill ingress must reach exact terminal-empty before Drop");
        }
    }

    impl BoardFillJob {
        pub fn with_operation(snapshot: BoardFillSnapshot, max_count: u32, operation: semio_framework_job::Operation) -> Self {
            Self {
                operation,
                state: Some(BoardFillJobState {
                    snapshot,
                    stage: if max_count == 0 { BoardFillStage::Complete } else { BoardFillStage::PrepareSources },
                    max_count,
                    rng_state: operation.seed,
                    sources: BoardFillFixedPages::new(),
                    source_scan_cursor: 0,
                    source_capture: None,
                    target_selection_cursor: 0,
                    target_best: None,
                    current_target: None,
                    candidates: BoardFillFixedPages::new(),
                    kind_cursor: 0,
                    template_cursor: 0,
                    compatibility_candidate: None,
                    compatibility_cursor: 0,
                    compatibility_matched: false,
                    candidate_selection_cursor: 0,
                    candidate_best: None,
                    current_candidate: None,
                    current_preview: None,
                    host_collision_cursor: 0,
                    virtual_collision_cursor: 0,
                    virtual_nodes: BoardFillFixedPages::new(),
                    virtual_handles: BoardFillFixedPages::new(),
                    pending_placement: None,
                    accept_pending_virtual_node: None,
                    accept_handle_template: None,
                    accept_handle_id: None,
                    accept_handle_pending_virtual: None,
                    accept_handle_pending_placement: None,
                    accept_handle_cursor: 0,
                    accepted_count: 0,
                    next_serial: operation.operation.0,
                    stalled: false,
                    rejection: None,
                    search_count: 0,
                    preview_sequence: 0,
                }),
                checkpoint: None,
                preview: None,
                commit_encoder: None,
                commit_writer: None,
                fault: None,
                closing: false,
            }
        }

        pub fn restore(mut checkpoint: BoardFillCheckpoint, operation: semio_framework_job::Operation) -> Result<Self, BoardFillCheckpoint> {
            if checkpoint.operation.operation != operation.operation || checkpoint.operation.base_revision != operation.base_revision || checkpoint.operation.generation != operation.generation {
                return Err(checkpoint);
            }
            let Some(state) = checkpoint.state.take() else { return Err(checkpoint) };
            Ok(Self { operation, state: Some(state), checkpoint: None, preview: None, commit_encoder: None, commit_writer: None, fault: None, closing: false })
        }

        pub fn operation(&self) -> semio_framework_job::Operation {
            self.operation
        }

        pub fn take_checkpoint(&mut self) -> Option<BoardFillCheckpoint> {
            self.checkpoint.take()
        }

        pub fn adopt_checkpoint(&mut self, mut checkpoint: BoardFillCheckpoint) -> Result<(), BoardFillCheckpoint> {
            if self.state.is_some() || checkpoint.operation.operation != self.operation.operation || checkpoint.operation.base_revision != self.operation.base_revision || checkpoint.operation.generation != self.operation.generation {
                return Err(checkpoint);
            }
            let Some(state) = checkpoint.state.take() else { return Err(checkpoint) };
            self.state = Some(state);
            Ok(())
        }

        pub fn take_preview(&mut self) -> Option<BoardFillPreview> {
            self.preview.take()
        }

        pub fn take_fault(&mut self) -> Option<&'static str> {
            self.fault.take()
        }

        pub fn stage(&self) -> BoardFillStage {
            match self.state.as_ref() {
                Some(state) => state.stage,
                None => match self.checkpoint.as_ref().and_then(|checkpoint| checkpoint.state.as_ref()) {
                    Some(state) => state.stage,
                    None => BoardFillStage::Complete,
                },
            }
        }

        fn stage_label(&self) -> &'static str {
            match self.stage() {
                BoardFillStage::ResetSources => "puzzle2d-fill-reset-sources",
                BoardFillStage::PrepareSources => "puzzle2d-fill-prepare-sources",
                BoardFillStage::SelectTarget => "puzzle2d-fill-select-target",
                BoardFillStage::ResetCandidates => "puzzle2d-fill-reset-candidates",
                BoardFillStage::PrepareCandidates => "puzzle2d-fill-prepare-candidates",
                BoardFillStage::ScanCompatibility => "puzzle2d-fill-scan-compatibility",
                BoardFillStage::SelectCandidate => "puzzle2d-fill-select-candidate",
                BoardFillStage::ConstructPreview => "puzzle2d-fill-construct-preview",
                BoardFillStage::ScanHostCollision => "puzzle2d-fill-scan-host-collision",
                BoardFillStage::ScanVirtualCollision => "puzzle2d-fill-scan-virtual-collision",
                BoardFillStage::AcceptCandidate => "puzzle2d-fill-accept-candidate",
                BoardFillStage::AcceptNodeId => "puzzle2d-fill-accept-node-id",
                BoardFillStage::AcceptEdgeId => "puzzle2d-fill-accept-edge-id",
                BoardFillStage::AcceptEdgeKind => "puzzle2d-fill-accept-edge-kind",
                BoardFillStage::AcceptNodeKind => "puzzle2d-fill-accept-node-kind",
                BoardFillStage::AcceptSourceHandle => "puzzle2d-fill-accept-source-handle",
                BoardFillStage::AcceptTargetHandle => "puzzle2d-fill-accept-target-handle",
                BoardFillStage::AcceptIcon => "puzzle2d-fill-accept-icon",
                BoardFillStage::AcceptVirtualNode => "puzzle2d-fill-accept-virtual-node",
                BoardFillStage::AcceptSourceConnection => "puzzle2d-fill-accept-source-connection",
                BoardFillStage::AcceptHandles => "puzzle2d-fill-accept-handles",
                BoardFillStage::AcceptHandleId => "puzzle2d-fill-accept-handle-id",
                BoardFillStage::AcceptHandleVirtual => "puzzle2d-fill-accept-handle-virtual",
                BoardFillStage::AcceptHandlePublish => "puzzle2d-fill-accept-handle-publish",
                BoardFillStage::PublishPlanPrefix => "puzzle2d-fill-publish-prefix",
                BoardFillStage::Complete => "puzzle2d-fill-complete",
            }
        }

        fn next_seed(state: &mut BoardFillJobState) -> u64 {
            state.rng_state = state.rng_state.wrapping_mul(6364136223846793005).wrapping_add(1);
            state.rng_state
        }

        fn weighted_rank(state: &mut BoardFillJobState, weight: f64) -> f64 {
            let unit = ((Self::next_seed(state) >> 11) as f64 + 1.0) / ((1_u64 << 53) as f64 + 1.0);
            -unit.ln() / weight.max(f64::EPSILON)
        }

        fn boxes_overlap(left: [f64; 4], right: [f64; 4]) -> bool {
            left[0] <= right[2] && left[2] >= right[0] && left[1] <= right[3] && left[3] >= right[1]
        }

        fn prepare_source(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let base_len = state.snapshot.handles.len;
            let total_len = base_len.checked_add(state.virtual_handles.len).ok_or("source-capacity")?;
            if let Some(current) = state.source_capture.as_mut() {
                match current.stage {
                    BoardFillSourceCaptureStage::Id => {
                        current.value.id = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.id,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.id,
                        };
                        current.stage = BoardFillSourceCaptureStage::NodeKind;
                    }
                    BoardFillSourceCaptureStage::NodeKind => {
                        current.value.node_kind = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.node_kind,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.node_kind,
                        };
                        current.stage = BoardFillSourceCaptureStage::HandleKind;
                    }
                    BoardFillSourceCaptureStage::HandleKind => {
                        current.value.handle_kind = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.handle_kind,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.handle_kind,
                        };
                        current.stage = BoardFillSourceCaptureStage::WireKind;
                    }
                    BoardFillSourceCaptureStage::WireKind => {
                        current.value.wire_kind = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.wire_kind,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.wire_kind,
                        };
                        current.stage = BoardFillSourceCaptureStage::EdgeKind;
                    }
                    BoardFillSourceCaptureStage::EdgeKind => {
                        current.value.edge_kind = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.edge_kind,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.edge_kind,
                        };
                        current.stage = BoardFillSourceCaptureStage::Slot;
                    }
                    BoardFillSourceCaptureStage::Slot => {
                        current.value.slot = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.slot,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.slot,
                        };
                        current.stage = BoardFillSourceCaptureStage::Weight;
                    }
                    BoardFillSourceCaptureStage::Weight => {
                        current.value.weight = match current.value.owner {
                            BoardFillSourceOwner::Host(index) => state.snapshot.handles.get(index).ok_or("missing-host-handle")?.weight,
                            BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get(index).ok_or("missing-virtual-handle")?.weight,
                        };
                        current.stage = BoardFillSourceCaptureStage::Publish;
                    }
                    BoardFillSourceCaptureStage::Publish => {
                        let completed = state.source_capture.take().ok_or("missing-source-capture")?;
                        if let Err(value) = state.sources.try_push_owned(completed.value) {
                            state.source_capture = Some(BoardFillSourceCapture { value, stage: BoardFillSourceCaptureStage::Publish });
                            return Err("source-capacity");
                        }
                        state.source_scan_cursor += 1;
                    }
                }
            } else if state.source_scan_cursor < total_len {
                let owner = if state.source_scan_cursor < base_len {
                    let handle = state.snapshot.handles.get(state.source_scan_cursor).ok_or("missing-host-handle")?;
                    if handle.connected || !handle.visible {
                        state.source_scan_cursor += 1;
                        return Ok(());
                    }
                    BoardFillSourceOwner::Host(state.source_scan_cursor)
                } else {
                    let index = state.source_scan_cursor - base_len;
                    let handle = state.virtual_handles.get(index).ok_or("missing-virtual-handle")?;
                    if handle.connected {
                        state.source_scan_cursor += 1;
                        return Ok(());
                    }
                    BoardFillSourceOwner::Virtual(index)
                };
                state.source_capture = Some(BoardFillSourceCapture {
                    value: BoardFillSource {
                        id: BoardFillText::empty(),
                        node_kind: BoardFillText::empty(),
                        handle_kind: BoardFillText::empty(),
                        wire_kind: BoardFillText::empty(),
                        edge_kind: BoardFillText::empty(),
                        slot: [0.0; 2],
                        weight: 0.0,
                        owner,
                        rejected: false,
                    },
                    stage: BoardFillSourceCaptureStage::Id,
                });
            }
            if state.source_capture.is_none() && state.source_scan_cursor >= total_len {
                if state.sources.is_empty() {
                    state.stalled = true;
                    state.stage = BoardFillStage::Complete;
                } else {
                    state.stage = BoardFillStage::SelectTarget;
                    state.target_selection_cursor = 0;
                    state.target_best = None;
                }
            }
            Ok(())
        }

        fn select_target(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            if state.target_selection_cursor < state.sources.len {
                let index = state.target_selection_cursor;
                state.target_selection_cursor += 1;
                let source = *state.sources.get(index).ok_or("missing-source")?;
                if !source.rejected {
                    let rank = Self::weighted_rank(state, source.weight);
                    if state.target_best.is_none_or(|best| rank < best.0 || (rank == best.0 && index < best.1)) {
                        state.target_best = Some((rank, index));
                    }
                }
                return Ok(());
            }
            let Some((_, index)) = state.target_best.take() else {
                state.stalled = true;
                state.stage = BoardFillStage::Complete;
                return Ok(());
            };
            state.current_target = Some(index);
            state.kind_cursor = 0;
            state.template_cursor = 0;
            state.compatibility_candidate = None;
            state.compatibility_cursor = 0;
            state.compatibility_matched = false;
            state.rejection = None;
            state.stage = if state.candidates.terminal_is_empty() { BoardFillStage::PrepareCandidates } else { BoardFillStage::ResetCandidates };
            Ok(())
        }

        fn compatibility_rule_matches(rule: &BoardFillRuleSnapshot, source: &BoardFillSource, target_node: BoardFillText, target: &BoardFillTemplateSnapshot) -> bool {
            let (left, right) = match rule.specificity {
                CompatSpecificity::General | CompatSpecificity::Handle => (source.handle_kind, target.handle_kind),
                CompatSpecificity::Node => (source.node_kind, target_node),
                CompatSpecificity::Edge => (source.edge_kind, target.edge_kind),
                CompatSpecificity::Wire => (source.wire_kind, target.handle_kind),
            };
            (rule.source == left && rule.target == right) || (rule.bidirectional && rule.source == right && rule.target == left)
        }

        fn prepare_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let Some(target_index) = state.current_target else {
                Self::reject_target(state, "missing-target")?;
                return Ok(());
            };
            let Some(kind) = state.snapshot.kinds.get(state.kind_cursor) else {
                if state.candidates.is_empty() {
                    Self::reject_target(state, "no-compatible-candidate")?;
                } else {
                    state.candidate_selection_cursor = 0;
                    state.candidate_best = None;
                    state.stage = BoardFillStage::SelectCandidate;
                }
                return Ok(());
            };
            if let Some(template) = kind.handles.get(state.template_cursor) {
                let target = *state.sources.get(target_index).ok_or("missing-target-source")?;
                if !BoardHost::handle_port_shapes_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) || !BoardHost::single_letter_port_families_compatible(target.handle_kind.as_str(), template.handle_kind.as_str()) {
                    state.template_cursor += 1;
                    return Ok(());
                }
                let candidate = BoardFillCandidate { kind_index: state.kind_cursor, target_handle_index: state.template_cursor, weight: kind.weight * template.weight, rejected: false };
                if state.snapshot.rules.is_empty() {
                    if let Err(candidate) = state.candidates.try_push_owned(candidate) {
                        state.compatibility_candidate = Some(candidate);
                        return Err("candidate-capacity");
                    }
                    state.template_cursor += 1;
                } else {
                    state.compatibility_candidate = Some(candidate);
                    state.compatibility_cursor = 0;
                    state.compatibility_matched = false;
                    state.stage = BoardFillStage::ScanCompatibility;
                }
            } else {
                state.kind_cursor += 1;
                state.template_cursor = 0;
            }
            Ok(())
        }

        fn scan_compatibility(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let candidate = state.compatibility_candidate.ok_or("missing-compatibility-candidate")?;
            let target_index = state.current_target.ok_or("missing-target")?;
            let source = *state.sources.get(target_index).ok_or("missing-target-source")?;
            let kind = state.snapshot.kinds.get(candidate.kind_index).ok_or("missing-candidate-kind")?;
            let template = kind.handles.get(candidate.target_handle_index).ok_or("missing-candidate-template")?;
            if let Some(rule) = state.snapshot.rules.get(state.compatibility_cursor) {
                state.compatibility_matched |= Self::compatibility_rule_matches(rule, &source, kind.id, template);
                state.compatibility_cursor += 1;
                return Ok(());
            }
            if state.compatibility_matched {
                if let Err(candidate) = state.candidates.try_push_owned(candidate) {
                    state.compatibility_candidate = Some(candidate);
                    return Err("candidate-capacity");
                }
            }
            state.compatibility_candidate = None;
            state.template_cursor += 1;
            state.stage = BoardFillStage::PrepareCandidates;
            Ok(())
        }

        fn select_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            if state.candidate_selection_cursor < state.candidates.len {
                let index = state.candidate_selection_cursor;
                state.candidate_selection_cursor += 1;
                let candidate = *state.candidates.get(index).ok_or("missing-candidate")?;
                if !candidate.rejected {
                    let rank = Self::weighted_rank(state, candidate.weight);
                    if state.candidate_best.is_none_or(|best| rank < best.0 || (rank == best.0 && index < best.1)) {
                        state.candidate_best = Some((rank, index));
                    }
                }
                return Ok(());
            }
            let Some((_, index)) = state.candidate_best.take() else {
                Self::reject_target(state, "candidates-exhausted")?;
                return Ok(());
            };
            state.current_candidate = Some(index);
            state.stage = BoardFillStage::ConstructPreview;
            Ok(())
        }

        fn construct_preview(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let Some(target_index) = state.current_target else {
                Self::reject_target(state, "missing-target")?;
                return Ok(());
            };
            let Some(candidate_index) = state.current_candidate else {
                Self::reject_target(state, "missing-candidate")?;
                return Ok(());
            };
            let target = *state.sources.get(target_index).ok_or("missing-target-source")?;
            let candidate = *state.candidates.get(candidate_index).ok_or("missing-candidate")?;
            let kind = state.snapshot.kinds.get(candidate.kind_index).ok_or("missing-candidate-kind")?;
            let [x, y] = target.slot;
            let bounds = match kind.shape {
                BoardFillShape::Rectangle => [x - kind.width / 2.0, y - kind.height / 2.0, x + kind.width / 2.0, y + kind.height / 2.0],
                BoardFillShape::Circle => [x - kind.radius, y - kind.radius, x + kind.radius, y + kind.radius],
            };
            state.current_preview = Some(BoardFillCandidatePreview { source_id: target.id, kind_index: candidate.kind_index, target_handle_index: candidate.target_handle_index, x, y, bounds });
            state.host_collision_cursor = 0;
            state.virtual_collision_cursor = 0;
            state.search_count = state.search_count.checked_add(1).ok_or("search-sequence-exhausted")?;
            state.stage = BoardFillStage::ScanHostCollision;
            Ok(())
        }

        fn scan_host_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let Some(preview) = state.current_preview else {
                Self::reject_candidate(state, "missing-preview")?;
                return Ok(());
            };
            if let Some(node) = state.snapshot.nodes.get(state.host_collision_cursor) {
                state.host_collision_cursor += 1;
                if Self::boxes_overlap(preview.bounds, node.bounds) {
                    Self::reject_candidate(state, "host-collision")?;
                }
            } else {
                state.stage = BoardFillStage::ScanVirtualCollision;
            }
            Ok(())
        }

        fn scan_virtual_collision(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let Some(preview) = state.current_preview else {
                Self::reject_candidate(state, "missing-preview")?;
                return Ok(());
            };
            if let Some(node) = state.virtual_nodes.get(state.virtual_collision_cursor) {
                state.virtual_collision_cursor += 1;
                if Self::boxes_overlap(preview.bounds, node.bounds) {
                    Self::reject_candidate(state, "virtual-collision")?;
                }
            } else {
                state.stage = BoardFillStage::AcceptCandidate;
            }
            Ok(())
        }

        fn mark_source_connected(state: &mut BoardFillJobState, owner: BoardFillSourceOwner) -> Result<(), &'static str> {
            match owner {
                BoardFillSourceOwner::Host(index) => state.snapshot.handles.get_mut(index).ok_or("missing-host-source").map(|handle| handle.connected = true),
                BoardFillSourceOwner::Virtual(index) => state.virtual_handles.get_mut(index).ok_or("missing-virtual-source").map(|handle| handle.connected = true),
            }
        }

        fn accept_candidate(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
            let kind_handle_count = kind.handles.len;
            let virtual_handle_count = state.virtual_handles.len.checked_add(kind_handle_count).ok_or("virtual-handle-capacity")?;
            if state.virtual_nodes.len == BOARD_FILL_PLACEMENT_CAPACITY || virtual_handle_count > BOARD_FILL_SOURCE_CAPACITY {
                return Err("placement-capacity");
            }
            state.pending_placement = Some(BoardFillPlacement {
                node_id: BoardFillText::empty(),
                edge_id: BoardFillText::empty(),
                edge_kind: BoardFillText::empty(),
                node_kind: BoardFillText::empty(),
                source_handle_id: BoardFillText::empty(),
                target_handle_id: BoardFillText::empty(),
                target_handle_index: preview.target_handle_index,
                x: preview.x,
                y: preview.y,
                shape: if kind.shape == BoardFillShape::Rectangle { "rectangle" } else { "circle" },
                radius: kind.radius,
                width: kind.width,
                height: kind.height,
                icon_kind: None,
                handles: BoardFillFixedPages::try_new().map_err(|_| "placement-handle-descriptor-capacity")?,
            });
            state.accept_handle_cursor = 0;
            state.accept_handle_template = None;
            state.accept_handle_id = None;
            state.accept_handle_pending_virtual = None;
            state.stage = BoardFillStage::AcceptNodeId;
            Ok(())
        }

        fn accept_node_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
            placement.node_id = BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
            state.stage = BoardFillStage::AcceptEdgeId;
            Ok(())
        }

        fn accept_edge_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
            placement.edge_id = BoardFillText::try_indexed("puzzle2d.fill.edge.", state.next_serial, "", None).map_err(|_| "placement-id-capacity")?;
            state.stage = BoardFillStage::AcceptEdgeKind;
            Ok(())
        }

        fn accept_edge_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let source_index = state.current_target.ok_or("missing-target")?;
            let edge_kind = state.sources.get(source_index).ok_or("missing-target-source")?.edge_kind;
            state.pending_placement.as_mut().ok_or("missing-placement")?.edge_kind = edge_kind;
            state.stage = BoardFillStage::AcceptNodeKind;
            Ok(())
        }

        fn accept_node_kind(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
            state.pending_placement.as_mut().ok_or("missing-placement")?.node_kind = kind.id;
            state.stage = BoardFillStage::AcceptSourceHandle;
            Ok(())
        }

        fn accept_source_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            state.pending_placement.as_mut().ok_or("missing-placement")?.source_handle_id = preview.source_id;
            state.stage = BoardFillStage::AcceptTargetHandle;
            Ok(())
        }

        fn accept_target_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            state.pending_placement.as_mut().ok_or("missing-placement")?.target_handle_id = BoardFillText::try_indexed("puzzle2d.fill.", state.next_serial, ":h", Some(preview.target_handle_index)).map_err(|_| "placement-id-capacity")?;
            state.stage = BoardFillStage::AcceptIcon;
            Ok(())
        }

        fn accept_icon(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
            state.pending_placement.as_mut().ok_or("missing-placement")?.icon_kind = kind.icon;
            state.stage = BoardFillStage::AcceptVirtualNode;
            Ok(())
        }

        fn accept_virtual_node(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            if state.accept_pending_virtual_node.is_none() {
                let preview = state.current_preview.ok_or("missing-preview")?;
                let id = state.pending_placement.as_ref().ok_or("missing-placement")?.node_id;
                state.accept_pending_virtual_node = Some(BoardFillVirtualNode { id, bounds: preview.bounds });
                return Ok(());
            }
            let node = state.accept_pending_virtual_node.take().ok_or("missing-virtual-node")?;
            if let Err(node) = state.virtual_nodes.try_push_owned(node) {
                state.accept_pending_virtual_node = Some(node);
                return Err("placement-capacity");
            }
            state.stage = BoardFillStage::AcceptSourceConnection;
            Ok(())
        }

        fn accept_source_connection(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let source_index = state.current_target.ok_or("missing-target")?;
            let owner = state.sources.get(source_index).ok_or("missing-target-source")?.owner;
            Self::mark_source_connected(state, owner)?;
            state.next_serial = state.next_serial.checked_add(1).ok_or("placement-sequence-exhausted")?;
            state.stage = BoardFillStage::AcceptHandles;
            Ok(())
        }

        fn accept_handle(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
            let Some(template) = kind.handles.get(state.accept_handle_cursor).copied() else {
                state.accepted_count = state.accepted_count.checked_add(1).ok_or("accepted-count-exhausted")?;
                state.stage = BoardFillStage::PublishPlanPrefix;
                return Ok(());
            };
            state.accept_handle_template = Some(template);
            state.stage = BoardFillStage::AcceptHandleId;
            Ok(())
        }

        fn accept_handle_id(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let template = state.accept_handle_template.ok_or("missing-handle-template")?;
            if state.accept_handle_cursor == preview.target_handle_index {
                let id = state.pending_placement.as_ref().ok_or("missing-placement")?.target_handle_id;
                state.accept_handle_pending_placement = Some(BoardFillPlacementHandle { id, template });
                state.stage = BoardFillStage::AcceptHandlePublish;
                return Ok(());
            }
            let serial = state.next_serial.checked_sub(1).ok_or("placement-sequence-underflow")?;
            state.accept_handle_id = Some(BoardFillText::try_indexed("puzzle2d.fill.", serial, ":h", Some(state.accept_handle_cursor)).map_err(|_| "placement-id-capacity")?);
            state.stage = BoardFillStage::AcceptHandleVirtual;
            Ok(())
        }

        fn accept_handle_virtual(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            let preview = state.current_preview.ok_or("missing-preview")?;
            let kind = state.snapshot.kinds.get(preview.kind_index).ok_or("missing-candidate-kind")?;
            let template = state.accept_handle_template.ok_or("missing-handle-template")?;
            let id = state.accept_handle_id.take().ok_or("missing-handle-id")?;
            let center = Point::new(preview.x, preview.y);
            let anchor = match kind.shape {
                BoardFillShape::Circle => handle_position_on_circle(center, kind.radius, template.angle),
                BoardFillShape::Rectangle => handle_position_on_rectangle(center, kind.width, kind.height, template.angle),
            };
            let normal = normalize_or_zero(anchor - center);
            let slot = anchor + normal * state.snapshot.suggestion_offset;
            state.accept_handle_pending_virtual =
                Some(BoardFillVirtualHandle { id, node_kind: kind.id, handle_kind: template.handle_kind, wire_kind: template.wire_kind, edge_kind: template.edge_kind, slot: [slot.x, slot.y], weight: template.weight, connected: false });
            state.accept_handle_pending_placement = Some(BoardFillPlacementHandle { id, template });
            state.stage = BoardFillStage::AcceptHandlePublish;
            Ok(())
        }

        fn accept_handle_publish(state: &mut BoardFillJobState) -> Result<(), &'static str> {
            if let Some(handle) = state.accept_handle_pending_virtual.take() {
                if let Err(handle) = state.virtual_handles.try_push_owned(handle) {
                    state.accept_handle_pending_virtual = Some(handle);
                    return Err("virtual-handle-capacity");
                }
                return Ok(());
            }
            let handle = state.accept_handle_pending_placement.take().ok_or("missing-placement-handle")?;
            let placement = state.pending_placement.as_mut().ok_or("missing-placement")?;
            if let Err(handle) = placement.handles.try_push_owned(handle) {
                state.accept_handle_pending_placement = Some(handle);
                return Err("placement-handle-capacity");
            }
            state.accept_handle_template = None;
            state.accept_handle_cursor += 1;
            state.stage = BoardFillStage::AcceptHandles;
            Ok(())
        }

        fn publish_prefix(&mut self) -> Result<semio_framework_job::StepOutcome, &'static str> {
            let state = self.state.as_mut().ok_or("missing-fill-state")?;
            if state.accepted_count >= state.max_count {
                state.stage = BoardFillStage::Complete;
                return Ok(semio_framework_job::StepOutcome::Yield);
            }
            state.stage = BoardFillStage::ResetSources;
            let state = self.state.take().ok_or("missing-fill-state")?;
            self.checkpoint = Some(BoardFillCheckpoint { operation: self.operation, state: Some(state) });
            Ok(semio_framework_job::StepOutcome::CheckpointReady(semio_framework_job::Checkpoint {
                state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState),
                applied_progress: self.checkpoint.as_ref().map_or(0, |checkpoint| u64::from(checkpoint.accepted_count())),
            }))
        }

        fn reject_candidate(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
            if let Some(index) = state.current_candidate.take() {
                if let Some(candidate) = state.candidates.get_mut(index) {
                    candidate.rejected = true;
                }
            }
            state.rejection = Some(BoardFillText::try_from_str(reason).map_err(|_| "rejection-text-capacity")?);
            state.current_preview = None;
            state.host_collision_cursor = 0;
            state.virtual_collision_cursor = 0;
            state.candidate_selection_cursor = 0;
            state.candidate_best = None;
            state.stage = BoardFillStage::SelectCandidate;
            Ok(())
        }

        fn reject_target(state: &mut BoardFillJobState, reason: &'static str) -> Result<(), &'static str> {
            if let Some(index) = state.current_target.take() {
                if let Some(source) = state.sources.get_mut(index) {
                    source.rejected = true;
                }
            }
            state.rejection = Some(BoardFillText::try_from_str(reason).map_err(|_| "rejection-text-capacity")?);
            state.current_candidate = None;
            state.current_preview = None;
            state.target_selection_cursor = 0;
            state.target_best = None;
            state.stage = BoardFillStage::ResetCandidates;
            Ok(())
        }

        fn preview(state: &BoardFillJobState, operation: semio_framework_job::Operation, sequence: u64) -> BoardFillPreview {
            let target_handle_id = state.current_target.and_then(|index| state.sources.get(index)).map(|source| source.id);
            let candidate_node_kind_id = state.current_candidate.and_then(|index| state.candidates.get(index)).and_then(|candidate| state.snapshot.kinds.get(candidate.kind_index)).map(|kind| kind.id);
            let tested_collision_id = match state.stage {
                BoardFillStage::ScanHostCollision => state.host_collision_cursor.checked_sub(1).and_then(|index| state.snapshot.nodes.get(index)).map(|node| node.id),
                BoardFillStage::ScanVirtualCollision => state.virtual_collision_cursor.checked_sub(1).and_then(|index| state.virtual_nodes.get(index)).map(|node| node.id),
                _ => None,
            };
            BoardFillPreview {
                sequence,
                generation: operation.generation.0,
                stage: state.stage,
                accepted_count: state.accepted_count,
                target_handle_id,
                candidate_node_kind_id,
                host_collision_cursor: state.host_collision_cursor,
                virtual_collision_cursor: state.virtual_collision_cursor,
                tested_collision_id,
                rejection: state.rejection,
                search_count: state.search_count,
            }
        }

        fn complete(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            let Some(state) = self.state.as_ref() else { return self.fault_outcome("missing-fill-state") };
            if self.commit_encoder.is_none() {
                self.commit_encoder = Some(BoardFillCommitEncoder::new());
                context.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            let encoder_complete = self.commit_encoder.as_ref().is_some_and(|encoder| encoder.stage == BoardFillCommitEncodeStage::Complete);
            if !encoder_complete {
                if let Err(code) = self.commit_encoder.as_mut().ok_or("fill-commit-encoder").and_then(|encoder| encoder.step(state)) {
                    return self.fault_outcome(code);
                }
                context.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            if self.commit_writer.is_none() {
                self.commit_writer = Some(semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CommitOutput));
                context.consume_fuel(1);
                return semio_framework_job::StepOutcome::Yield;
            }
            let Some(encoder) = self.commit_encoder.as_ref() else { return self.fault_outcome("fill-commit-encoder") };
            let start = encoder.output_cursor;
            if start < BOARD_FILL_COMMIT_BYTES {
                let Some(writer) = self.commit_writer.as_mut() else { return self.fault_outcome("fill-commit-writer") };
                let mut page = match writer.admit_page(context) {
                    Ok(page) => page,
                    Err(_) => return semio_framework_job::StepOutcome::Yield,
                };
                let end = start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).min(BOARD_FILL_COMMIT_BYTES);
                if page.write(&encoder.bytes[start..end]).is_err() {
                    drop(page);
                    return self.fault_outcome("fill-commit-output");
                }
                page.commit();
                let Some(encoder) = self.commit_encoder.as_mut() else { return self.fault_outcome("fill-commit-encoder") };
                encoder.output_cursor = end;
                if end < BOARD_FILL_COMMIT_BYTES {
                    context.consume_fuel(1);
                    return semio_framework_job::StepOutcome::Yield;
                }
            }
            let Some(writer) = self.commit_writer.take() else { return self.fault_outcome("fill-commit-writer") };
            let output = match writer.finish() {
                Ok(output) => output,
                Err(writer) => {
                    self.commit_writer = Some(writer);
                    return semio_framework_job::StepOutcome::Yield;
                }
            };
            self.commit_encoder = None;
            semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate { state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState), output })
        }

        fn fault_outcome(&mut self, code: &'static str) -> semio_framework_job::StepOutcome {
            self.fault = Some(code);
            semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
        }
    }

    impl semio_framework_job::InteractiveJob for BoardFillJob {
        fn step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
            if context.is_cancelled() {
                return semio_framework_job::StepOutcome::Cancelled;
            }
            if context.operation() != self.operation.operation || context.generation() != self.operation.generation {
                return self.fault_outcome("stale-puzzle2d-fill-operation");
            }
            if context.should_yield() {
                return semio_framework_job::StepOutcome::Yield;
            }
            context.set_stage(self.stage_label());
            let Some(stage) = self.state.as_ref().map(|state| state.stage) else {
                return self.fault_outcome("fill-state-not-adopted");
            };
            if stage == BoardFillStage::PublishPlanPrefix {
                let outcome = match self.publish_prefix() {
                    Ok(outcome) => outcome,
                    Err(code) => self.fault_outcome(code),
                };
                context.consume_fuel(1);
                return outcome;
            }
            if stage == BoardFillStage::Complete {
                return self.complete(context);
            }
            let state = match self.state.as_mut() {
                Some(state) => state,
                None => return self.fault_outcome("missing-fill-state"),
            };
            let result = match stage {
                BoardFillStage::ResetSources => {
                    if state.sources.pop().is_none() {
                        if state.sources.retire_empty_page().is_none() {
                            state.source_scan_cursor = 0;
                            state.current_target = None;
                            state.current_candidate = None;
                            state.current_preview = None;
                            state.stage = BoardFillStage::PrepareSources;
                        }
                    }
                    Ok(())
                }
                BoardFillStage::PrepareSources => Self::prepare_source(state),
                BoardFillStage::SelectTarget => Self::select_target(state),
                BoardFillStage::ResetCandidates => {
                    if state.candidates.pop().is_none() {
                        if state.candidates.retire_empty_page().is_none() {
                            state.current_candidate = None;
                            state.compatibility_candidate = None;
                            state.compatibility_cursor = 0;
                            state.compatibility_matched = false;
                            state.stage = if state.current_target.is_some() { BoardFillStage::PrepareCandidates } else { BoardFillStage::SelectTarget };
                        }
                    }
                    Ok(())
                }
                BoardFillStage::PrepareCandidates => Self::prepare_candidate(state),
                BoardFillStage::ScanCompatibility => Self::scan_compatibility(state),
                BoardFillStage::SelectCandidate => Self::select_candidate(state),
                BoardFillStage::ConstructPreview => Self::construct_preview(state),
                BoardFillStage::ScanHostCollision => Self::scan_host_collision(state),
                BoardFillStage::ScanVirtualCollision => Self::scan_virtual_collision(state),
                BoardFillStage::AcceptCandidate => Self::accept_candidate(state),
                BoardFillStage::AcceptNodeId => Self::accept_node_id(state),
                BoardFillStage::AcceptEdgeId => Self::accept_edge_id(state),
                BoardFillStage::AcceptEdgeKind => Self::accept_edge_kind(state),
                BoardFillStage::AcceptNodeKind => Self::accept_node_kind(state),
                BoardFillStage::AcceptSourceHandle => Self::accept_source_handle(state),
                BoardFillStage::AcceptTargetHandle => Self::accept_target_handle(state),
                BoardFillStage::AcceptIcon => Self::accept_icon(state),
                BoardFillStage::AcceptVirtualNode => Self::accept_virtual_node(state),
                BoardFillStage::AcceptSourceConnection => Self::accept_source_connection(state),
                BoardFillStage::AcceptHandles => Self::accept_handle(state),
                BoardFillStage::AcceptHandleId => Self::accept_handle_id(state),
                BoardFillStage::AcceptHandleVirtual => Self::accept_handle_virtual(state),
                BoardFillStage::AcceptHandlePublish => Self::accept_handle_publish(state),
                BoardFillStage::PublishPlanPrefix | BoardFillStage::Complete => Ok(()),
            };
            if let Err(code) = result {
                return self.fault_outcome(code);
            }
            context.consume_fuel(1);
            if context.is_cancelled() {
                return semio_framework_job::StepOutcome::Cancelled;
            }
            if context.should_yield() {
                return semio_framework_job::StepOutcome::Yield;
            }
            let preview_sequence = match context.next_preview_sequence() {
                Ok(sequence) => sequence,
                Err(_) => return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) }),
            };
            let Some(next_preview) = preview_sequence.checked_add(1) else { return self.fault_outcome("preview-sequence-exhausted") };
            self.operation.preview_sequence = next_preview;
            let Some(state) = self.state.as_mut() else { return self.fault_outcome("missing-fill-state") };
            state.preview_sequence = next_preview;
            self.preview = Some(Self::preview(state, self.operation, preview_sequence));
            semio_framework_job::StepOutcome::PreviewReady(semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Preview))
        }

        fn begin_close(&mut self) {
            self.closing = true;
        }

        fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
            self.closing = true;
            if maximum_items == 0 {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
            }
            if self.state.is_none() {
                if let Some(mut checkpoint) = self.checkpoint.take() {
                    self.state = checkpoint.state.take();
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
            }
            if self.commit_encoder.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(writer) = self.commit_writer.as_mut() {
                writer.begin_close();
                let step = writer.close_step(maximum_items, maximum_bytes);
                if writer.terminal_is_empty() {
                    self.commit_writer = None;
                }
                return match step {
                    semio_framework_job::JobPayloadCloseStep::Pending { released_items, released_bytes } => semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes },
                    semio_framework_job::JobPayloadCloseStep::Complete => semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 },
                };
            }
            if self.preview.take().is_some() || self.fault.take().is_some() {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            let Some(state) = self.state.as_mut() else { return semio_framework_job::InteractiveJobCloseStep::Complete };
            if let Some(placement) = state.pending_placement.as_mut() {
                if placement.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = placement.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = placement.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
                state.pending_placement = None;
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if state.accept_handle_template.take().is_some()
                || state.accept_handle_id.take().is_some()
                || state.accept_handle_pending_virtual.take().is_some()
                || state.accept_handle_pending_placement.take().is_some()
                || state.accept_pending_virtual_node.take().is_some()
                || state.source_capture.take().is_some()
                || state.compatibility_candidate.take().is_some()
                || state.current_preview.take().is_some()
                || state.rejection.take().is_some()
            {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(kind) = state.snapshot.kinds.len.checked_sub(1).and_then(|index| state.snapshot.kinds.get_mut(index)) {
                if kind.handles.pop().is_some() {
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                if let Some(bytes) = kind.handles.retained_page_bytes() {
                    if bytes > maximum_bytes {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    }
                    let Some(released_bytes) = kind.handles.retire_empty_page() else {
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                    };
                    return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                }
            }
            if state.snapshot.kinds.pop().is_some()
                || state.virtual_handles.pop().is_some()
                || state.virtual_nodes.pop().is_some()
                || state.candidates.pop().is_some()
                || state.sources.pop().is_some()
                || state.snapshot.rules.pop().is_some()
                || state.snapshot.handles.pop().is_some()
                || state.snapshot.nodes.pop().is_some()
            {
                return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            macro_rules! retire_page {
                ($owners:expr) => {
                    if let Some(bytes) = $owners.retained_page_bytes() {
                        if bytes > maximum_bytes {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        }
                        let Some(released_bytes) = $owners.retire_empty_page() else {
                            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
                        };
                        return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
                    }
                };
            }
            retire_page!(state.virtual_handles);
            retire_page!(state.virtual_nodes);
            retire_page!(state.candidates);
            retire_page!(state.sources);
            retire_page!(state.snapshot.rules);
            retire_page!(state.snapshot.handles);
            retire_page!(state.snapshot.nodes);
            retire_page!(state.snapshot.kinds);
            self.state = None;
            semio_framework_job::InteractiveJobCloseStep::Complete
        }

        fn terminal_is_empty(&self) -> bool {
            self.closing && self.state.is_none() && self.checkpoint.is_none() && self.preview.is_none() && self.commit_encoder.is_none() && self.commit_writer.is_none() && self.fault.is_none()
        }
    }

    impl Drop for BoardFillJob {
        fn drop(&mut self) {
            assert!(semio_framework_job::InteractiveJob::terminal_is_empty(self), "Puzzle2d fill job must reach exact terminal-empty before Drop");
        }
    }

    const _: fn() = || {
        fn assert_send<T: Send>() {}
        assert_send::<BoardFillSnapshot>();
        assert_send::<BoardFillSnapshotCapture>();
        assert_send::<BoardFillSnapshotIngress>();
        assert_send::<BoardFillCheckpoint>();
        assert_send::<BoardFillPlacement>();
        assert_send::<BoardFillJob>();
    };

    impl BoardHost {
        //#endregion 🧵️FillJob

        fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
            let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                Some("rectangle") => NodeShape::Rectangle,
                _ => NodeShape::Circle,
            };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
                NodeShape::Rectangle => {
                    let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    (0.0, w, h)
                }
            };
            let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
            let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
            if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
                for row in arr {
                    let ho = row.as_object()?;
                    let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
                    let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                    handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                }
            }
            if handles.is_empty() {
                return None;
            }
            let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            Some(BrushPreviewSnapshot { source_handle_id: source_handle_id.to_string(), node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, handles, target_handle_index, icon_kind })
        }

        /// @emoji 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
        pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.trim().is_empty() {
                self.brush_slot_suggestions_active = false;
                self.brush_slot_source_id = None;
                self.brush_candidates.clear();
                self.brush_candidate_index = 0;
                self.brush_preview = None;
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            if json.len() > BOARD_EVENT_BYTE_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::BrushSessionJson)?;
            let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            self.brush_slot_source_id = source.clone();
            let mut candidates = BrushCandidatePage::default();
            if let Some(rows) = v.get("candidates").and_then(|x| x.as_array()) {
                if rows.len() > BOARD_POINTER_ITEM_CAPACITY {
                    return Err(NormalPortError::EventCredits);
                }
                for row in rows {
                    let (node_kind_id, target_handle_index) = if let Some(node_kind_id) = row.as_str().map(str::trim).filter(|value| !value.is_empty()) {
                        (node_kind_id, 0)
                    } else {
                        let node_kind_id = row.get("nodeKind").or_else(|| row.get("nodeKindId")).and_then(|value| value.as_str()).map(str::trim).filter(|value| !value.is_empty()).ok_or(NormalPortError::EventCredits)?;
                        let target_handle_index = row.get("targetHandleIndex").and_then(|value| value.as_u64()).unwrap_or(0);
                        let target_handle_index = usize::try_from(target_handle_index).map_err(|_| NormalPortError::EventCredits)?;
                        (node_kind_id, target_handle_index)
                    };
                    candidates.push(node_kind_id, target_handle_index, 0.0).map_err(|_| NormalPortError::EventCredits)?;
                }
            }
            self.brush_candidates = candidates;
            self.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map(|i| i as usize).unwrap_or(0);
            if self.brush_candidates.is_empty() {
                self.brush_candidate_index = 0;
            } else {
                self.brush_candidate_index %= self.brush_candidates.len();
            }
            self.brush_preview = match (source.as_deref(), v.get("preview")) {
                (Some(source_id), Some(preview)) if !preview.is_null() => {
                    let node = preview.get("node").filter(|n| !n.is_null());
                    let edge = preview.get("edge").filter(|e| !e.is_null());
                    match (node, edge) {
                        (Some(node), Some(edge)) => Self::brush_preview_snapshot_from_session_json(node, edge, source_id),
                        _ => None,
                    }
                }
                _ => None,
            };
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_slot_suggestions_active = v.get("suggestionsActive").and_then(|x| x.as_bool()).unwrap_or(false);
            if self.brush_preview.is_none() && !self.brush_candidates.is_empty() {
                self.brush_rebuild_preview();
            } else {
                self.brush_sync_preview_events();
            }
            self.bump_content_scene_generation();
            Ok(())
        }

        fn brush_enter_slot(&mut self, source_handle_id: String) {
            if self.brush_slot_source_id.as_deref() == Some(source_handle_id.as_str()) {
                return;
            }
            if self.brush_slot_source_id.is_some() {
                self.brush_finish_slot();
            }
            self.brush_slot_source_id = Some(source_handle_id.clone());
            let Some(source) = self.handles.get(source_handle_id.as_str()).cloned() else {
                self.brush_candidates.clear();
                self.brush_candidate_index = 0;
                self.brush_rebuild_preview();
                return;
            };
            let Some(candidates) = self.brush_compatible_candidates(&source) else {
                self.event_schema_fault = true;
                return;
            };
            self.brush_candidates = candidates;
            self.brush_candidate_index = 0;
            self.brush_rebuild_preview();
        }

        fn brush_rebuild_preview(&mut self) {
            let Some(ref source_id) = self.brush_slot_source_id else {
                self.brush_preview = None;
                self.brush_sync_preview_events();
                return;
            };
            let preview = self.brush_candidates.get(self.brush_candidate_index).and_then(|candidate| self.brush_build_preview(source_id, candidate));
            self.brush_preview = preview;
            if self.brush_preview.is_some() {
                self.bump_content_scene_generation();
            }
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_sync_preview_events();
        }

        fn brush_pointer_move(&mut self, world: Point) {
            if let Some(slot) = self.brush_nearest_slot_source(world) {
                self.brush_enter_slot(slot);
                self.set_hovered_id(self.brush_slot_source_id.clone());
            } else if self.brush_slot_source_id.is_some() {
                self.brush_finish_slot();
                self.set_hovered_id(None);
            }
        }

        pub fn set_active_utility(&mut self, label: &str) {
            let next = if label == "brush" { ActiveUtility::Brush } else { ActiveUtility::Select };
            if self.active_utility == next {
                return;
            }
            if self.active_utility == ActiveUtility::Brush {
                self.brush_finish_slot();
            }
            self.active_utility = next;
            self.interaction = Interaction::None;
            self.bump_content_scene_generation();
        }

        pub fn set_suggestion_offset(&mut self, distance: f64) {
            let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_SUGGESTION_OFFSET };
            if (self.suggestion_offset - d).abs() < 1e-9 {
                return;
            }
            self.suggestion_offset = d;
            if self.active_utility == ActiveUtility::Brush {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn set_brush_kind_weights(&mut self, json: &str) {
            if json.is_empty() {
                return;
            }
            let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
                return;
            };
            self.brush_node_kind_weights.clear();
            self.brush_handle_kind_weights.clear();
            if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
                for (k, val) in obj {
                    if let Some(w) = val.as_f64() {
                        if w.is_finite() && w >= 0.0 {
                            self.brush_node_kind_weights.insert(k.clone(), w);
                        }
                    }
                }
            }
            if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
                for (k, val) in obj {
                    if let Some(w) = val.as_f64() {
                        if w.is_finite() && w >= 0.0 {
                            self.brush_handle_kind_weights.insert(k.clone(), w);
                        }
                    }
                }
            }
            if self.active_utility != ActiveUtility::Brush {
                return;
            }
            if let Some(source) = self.brush_slot_source_id.clone() {
                self.brush_slot_source_id = None;
                self.brush_enter_slot(source);
            } else {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn set_brush_node_size(&mut self, size: f64) {
            let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
            if (self.brush_node_size - s).abs() < 1e-9 {
                return;
            }
            self.brush_node_size = s;
            if self.active_utility == ActiveUtility::Brush {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn brush_cycle_candidate(&mut self, forward: bool) {
            if self.brush_candidates.len() < 2 {
                return;
            }
            let len = self.brush_candidates.len();
            self.brush_candidate_index = if forward { (self.brush_candidate_index + 1) % len } else { (self.brush_candidate_index + len - 1) % len };
            self.brush_rebuild_preview();
        }

        pub fn brush_set_candidate_index(&mut self, index: usize) {
            if self.brush_candidates.is_empty() {
                return;
            }
            self.brush_candidate_index = index % self.brush_candidates.len();
            self.brush_rebuild_preview();
        }

        /// @emoji 🖌️ Opens a brush slot on a free handle (suggestions menu; works outside brush utility).
        pub fn brush_open_slot(&mut self, handle_id: &str) {
            if !self.handles.contains_key(handle_id) {
                return;
            }
            self.brush_enter_slot(handle_id.to_string());
            self.brush_slot_suggestions_active = true;
            self.brush_rebuild_preview();
            self.set_hovered_id(Some(handle_id.to_string()));
        }

        /// @emoji 🖌️ Commits the active brush preview and clears the slot.
        pub fn brush_commit_slot(&mut self) {
            self.brush_commit_preview();
            self.brush_clear_slot();
        }

        /// @emoji 🖌️ Discards the active brush slot without placing.
        pub fn brush_cancel_slot(&mut self) {
            self.brush_clear_slot();
        }

        fn append_brush_node_icon_paint(&self, scene: &mut Scene, lod: BoardDrawLod, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: &str, world_space: bool) {
            if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                return;
            }
            let preserve_original_style = false;
            let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
            let Some(paint) = self.get_or_build_icon_paint(icon_kind, icon_fg, icon_bg, preserve_original_style) else {
                return;
            };
            let (bx, by, bw, bh) = paint.bounds();
            let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
            let fit_inset = ui_styling::metrics::icon::FIT_INSET;
            let (sx_half, sy_half) = match shape {
                NodeShape::Circle => {
                    let s = self.draw_space_len(radius, world_space) * fit_inset;
                    (s, s)
                }
                NodeShape::Rectangle => (self.draw_space_len(width, world_space) * fit_inset * 0.5, self.draw_space_len(height, world_space) * fit_inset * 0.5),
            };
            let center_ds = self.draw_space_point(center, world_space);
            let cx = bx + bw * 0.5;
            let cy = by + bh * 0.5;
            let avail_w = 2.0 * sx_half;
            let avail_h = 2.0 * sy_half;
            let scale = (avail_w / bw).min(avail_h / bh);
            let aff = Affine::IDENTITY.translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::IDENTITY.scale(scale);
            match shape {
                NodeShape::Circle => {
                    let r_clip = self.draw_space_len(radius, world_space) * clip_inset;
                    let disc = Circle::new(center_ds, r_clip);
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
                NodeShape::Rectangle => {
                    let hw = self.draw_space_len(width, world_space) * clip_inset * 0.5;
                    let hh = self.draw_space_len(height, world_space) * clip_inset * 0.5;
                    let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
                    scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                    match paint.body() {
                        CachedIconBody::Vector(icon_scene) => {
                            scene.append(icon_scene, Some(aff));
                        }
                        CachedIconBody::Raster(img) => {
                            scene.draw_image(img, aff);
                        }
                    }
                    scene.pop_layer();
                }
            }
        }

        fn paint_highlighted_node_preview(&self, scene: &mut Scene, _lod: BoardDrawLod, x: f64, y: f64, shape: NodeShape, radius: f64, width: f64, height: f64, icon_kind: Option<&str>, world_space: bool) {
            let center = Point::new(x, y);
            let style = BoardElementStyleKind::Highlighted;
            let fill = Self::node_fill_for_style(&self.canvas_theme, style);
            let stroke_c = Self::node_stroke_for_style(&self.canvas_theme, style);
            let stroke = Stroke::new(ui_styling::strokes::NODE_BODY);
            match shape {
                NodeShape::Circle => {
                    let c = self.draw_space_point(center, world_space);
                    let r = self.draw_space_len(radius, world_space);
                    let circle = Circle::new(c, r);
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
                    scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
                }
                NodeShape::Rectangle => {
                    let hw = width * 0.5;
                    let hh = height * 0.5;
                    let p0 = self.draw_space_point(Point::new(x - hw, y - hh), world_space);
                    let p1 = self.draw_space_point(Point::new(x + hw, y + hh), world_space);
                    let rect = Rect::from_points(p0, p1);
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
                    scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
                }
            }
            if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
                self.append_brush_node_icon_paint(scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, world_space);
            }
        }

        fn fixture_drop_preview_effective_dims(&self, preview: &FixtureDropPreviewSnapshot) -> (NodeShape, f64, f64, f64) {
            if let Some(kind) = self.node_kinds.get(preview.node_kind_id.as_str()) {
                let radius = self.brush_node_size * 0.5 * kind.scale;
                let (width, height) = if kind.shape == NodeShape::Rectangle { (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale) } else { (radius * 2.0, radius * 2.0) };
                return (kind.shape, radius, width, height);
            }
            (preview.shape, preview.radius, preview.width, preview.height)
        }

        fn fixture_drop_preview_from_json(&self, node: &serde_json::Value) -> Option<FixtureDropPreviewSnapshot> {
            let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let (x, y) = match (node.get("screenX").and_then(|v| v.as_f64()).filter(|v| v.is_finite()), node.get("screenY").and_then(|v| v.as_f64()).filter(|v| v.is_finite())) {
                (Some(sx), Some(sy)) => {
                    let world = self.screen_to_world(Point::new(sx, sy));
                    (world.x, world.y)
                }
                _ => {
                    let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    (x, y)
                }
            };
            let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                Some("rectangle") => NodeShape::Rectangle,
                _ => NodeShape::Circle,
            };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
                NodeShape::Rectangle => {
                    let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    (0.0, w, h)
                }
            };
            let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            Some(FixtureDropPreviewSnapshot { node_kind_id: node_kind_id.to_string(), x, y, shape, radius, width, height, icon_kind })
        }

        /// @emoji 👻️ Sets or clears the workbench palette fixture drop ghost node (independent of brush utility).
        pub fn set_fixture_drop_preview_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            if json.trim().is_empty() {
                self.fixture_drop_preview = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(NormalPortError::FixtureDropPreviewJson)?;
            self.fixture_drop_preview = self.fixture_drop_preview_from_json(&v);
            if self.fixture_drop_preview.is_none() {
                return Err(NormalPortError::FixtureDropPreviewInvalid);
            }
            self.bump_content_scene_generation();
            Ok(())
        }

        fn append_fixture_drop_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
            let Some(ref preview) = self.fixture_drop_preview else {
                return;
            };
            let (shape, radius, width, height) = self.fixture_drop_preview_effective_dims(preview);
            let icon_kind = preview.icon_kind.as_deref().filter(|s| !s.is_empty()).or_else(|| self.node_kinds.get(preview.node_kind_id.as_str()).and_then(|k| k.icon.as_deref()));
            self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, shape, radius, width, height, icon_kind, world_space);
        }

        fn append_brush_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod, world_space: bool) {
            let Some(ref preview) = self.brush_preview else {
                return;
            };
            let _ = lod;
            self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, preview.shape, preview.radius, preview.width, preview.height, preview.icon_kind.as_deref(), world_space);
            let center = Point::new(preview.x, preview.y);
            let source = match self.handles.get(preview.source_handle_id.as_str()) {
                Some(h) => h,
                None => return,
            };
            let src_pos = match self.handle_world_pos(source) {
                Some(p) => p,
                None => return,
            };
            let tmpl = match preview.handles.get(preview.target_handle_index) {
                Some(t) => t,
                None => return,
            };
            let tgt_pos = self.brush_template_world_pos(center, preview.shape, preview.radius, preview.width, preview.height, tmpl.angle);
            let Some(src_node) = self.nodes.get(&source.node_id) else {
                return;
            };
            let tgt_center = center;
            let curve = compute_edge_bezier_points(src_pos, tgt_pos, Point::new(src_node.x, src_node.y), tgt_center);
            let p0 = self.draw_space_point(curve.p0(), world_space);
            let p1 = self.draw_space_point(curve.p1(), world_space);
            let p2 = self.draw_space_point(curve.p2(), world_space);
            let p3 = self.draw_space_point(curve.p3(), world_space);
            let bez = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT), Affine::IDENTITY, self.canvas_theme.wire_stroke_highlighted, None, &bez);
        }

        /// @emoji 🧩️ Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
        pub fn set_world_raster_tiling(&mut self, mode: &str) {
            let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
            if self.world_raster_tiling == next {
                return;
            }
            self.world_raster_tiling = next;
        }

        pub fn set_original_element_style(&mut self, enabled: bool) {
            if self.preserve_original_element_style == enabled {
                return;
            }
            self.preserve_original_element_style = enabled;
            self.icon_paint_cache.clear();
        }

        pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
            if points.is_none() {
                self.selection_preview_crossing = false;
            }
            self.selection_screen_preview = points;
        }

        pub fn set_canvas_theme_from_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            self.canvas_theme.merge_from_json(json).map_err(NormalPortError::Theme)?;
            self.icon_paint_cache.clear();
            Ok(())
        }

        fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
            if screen_points.len() < 2 {
                self.selection_screen_preview = None;
                self.selection_preview_crossing = false;
                return;
            }
            let last = *screen_points.last().unwrap_or(&start_screen);
            self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
            self.selection_screen_preview = Some(if self.selection_options.method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] });
        }

        fn reserve_selection_event(&mut self, kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Option<BoardEventReservation> {
            self.reserve_owned_event(BoardOwnedEvent::selection(kind, ids, anchor_ids, removed_ids, gesture))
        }

        fn reserve_owned_event(&mut self, event: Result<BoardOwnedEvent, BoardEventFault>) -> Option<BoardEventReservation> {
            let event = match event {
                Ok(event) => event,
                Err(_) => {
                    self.event_schema_fault = true;
                    return None;
                }
            };
            match self.events.reserve_event(event) {
                Ok(reservation) => Some(reservation),
                Err(event) => {
                    if self.event_overflow.is_none() {
                        self.event_overflow = Some(event);
                    } else {
                        self.event_schema_fault = true;
                    }
                    None
                }
            }
        }

        fn reserve_owned_batch(&mut self, first: Result<BoardOwnedEvent, BoardEventFault>, second: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> Option<BoardEventBatchReservation> {
            let first = match first {
                Ok(event) => event,
                Err(_) => {
                    self.event_schema_fault = true;
                    return None;
                }
            };
            let mut batch = BoardEventBatchReservation::one(first);
            if let Some(second) = second {
                let second = match second {
                    Ok(event) => event,
                    Err(_) => {
                        self.event_schema_fault = true;
                        return None;
                    }
                };
                if batch.push(second).is_err() {
                    self.event_schema_fault = true;
                    return None;
                }
            }
            match self.events.reserve_batch(batch) {
                Ok(batch) => Some(batch),
                Err(batch) => {
                    if self.event_batch_overflow.is_none() {
                        self.event_batch_overflow = Some(batch);
                    } else {
                        self.event_schema_fault = true;
                    }
                    None
                }
            }
        }

        fn publish_event_reservation(&mut self, reservation: BoardEventReservation) {
            if self.events.publish_reserved(reservation).is_err() {
                self.event_schema_fault = true;
            }
        }

        fn publish_event_batch(&mut self, reservation: BoardEventBatchReservation) {
            if self.events.publish_batch(reservation).is_err() {
                self.event_schema_fault = true;
            }
        }

        /// @emoji 🏁️ Emits final node coordinates after a drag gesture so hosts can commit declarative fixture state once.
        fn push_node_drag_end_events(&mut self, start_positions: &BTreeMap<String, (f64, f64)>) {
            if !start_positions.keys().any(|id| self.nodes.contains_key(id)) {
                return;
            }
            let event = BoardOwnedEvent::node_drag_end(start_positions.keys().filter_map(|id| self.nodes.get(id).map(|node| (id.as_str(), node.x, node.y))));
            let Some(reservation) = self.reserve_owned_event(event) else {
                return;
            };
            self.publish_event_reservation(reservation);
        }

        #[cfg(test)]
        pub fn drain_events_json(&mut self) -> String {
            let mut out = String::from("[");
            let mut first = true;
            while let Some(event) = self.events.pop() {
                if !first {
                    out.push(',');
                }
                first = false;
                event.write_json(&mut out);
            }
            if let Some(event) = self.event_overflow.take() {
                if !first {
                    out.push(',');
                }
                event.write_json(&mut out);
            }
            out.push(']');
            out
        }

        pub fn pop_owned_event(&mut self) -> Option<BoardOwnedEvent> {
            if let Some(event) = self.events.pop().or_else(|| self.event_overflow.take()) {
                return Some(event);
            }
            let batch = self.event_batch_overflow.as_mut()?;
            let event = batch.pop();
            if batch.is_empty() {
                self.event_batch_overflow = None;
            }
            event
        }

        pub fn peek_owned_event(&self) -> Option<&BoardOwnedEvent> {
            self.events.iter().next().or(self.event_overflow.as_ref()).or_else(|| self.event_batch_overflow.as_ref().and_then(BoardEventBatchReservation::peek))
        }

        pub fn event_terminal_faulted(&self) -> bool {
            self.event_schema_fault
        }

        pub fn step_event_authority(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
            if context.should_yield() {
                return BoardAuthorityStep::Pending;
            }
            let faulted_before = self.event_schema_fault;
            if self.pending_delete_planning.is_some() {
                if context.is_cancelled() {
                    if let Some(planning) = self.pending_delete_planning.as_mut() {
                        planning.cancelling = true;
                    }
                }
                let result = self.step_delete_planning();
                context.consume_fuel(1);
                return result;
            }
            if self.pending_delete_operation.is_some() {
                if context.is_cancelled() {
                    if let Some(operation) = self.pending_delete_operation.as_mut() {
                        if !operation.claimed {
                            operation.cancelling = true;
                        }
                    }
                }
                if self.pending_delete_operation.as_ref().is_some_and(|operation| operation.cancelling && !operation.claimed) {
                    self.pending_delete_operation = None;
                    context.consume_fuel(1);
                    return BoardAuthorityStep::Cancelled;
                }
                self.step_pending_delete_operation();
                context.consume_fuel(1);
                if self.event_schema_fault && !faulted_before {
                    return BoardAuthorityStep::Fault;
                }
                return if self.pending_delete_operation.is_some() { BoardAuthorityStep::Pending } else { BoardAuthorityStep::Complete };
            }
            BoardAuthorityStep::Complete
        }

        fn set_pointer_selection_flag(&mut self, id: &str, selected: bool) {
            if let Some(node) = self.nodes.get_mut(id) {
                node.selected = selected;
            }
            if let Some(handle) = self.handles.get_mut(id) {
                handle.selected = selected;
            }
            if let Some(edge) = self.edges.get_mut(id) {
                edge.selected = selected;
            }
            if let Some(wire) = self.wires.get_mut(id) {
                wire.selected = selected;
            }
        }

        fn step_pointer_commit_operation(&mut self, operation: &mut BoardPointerCommitOperation) -> bool {
            match operation.plan.kind {
                BoardPointerPlanKind::Idle => true,
                BoardPointerPlanKind::Pan { camera } => {
                    self.camera = Camera { x: camera[0], y: camera[1], zoom: infinite::canvas::camera::clamp_zoom(camera[2]) };
                    true
                }
                BoardPointerPlanKind::FinishPan { camera } => match operation.phase {
                    0 => {
                        self.camera = Camera { x: camera[0], y: camera[1], zoom: infinite::canvas::camera::clamp_zoom(camera[2]) };
                        operation.phase = 1;
                        false
                    }
                    _ => {
                        self.interaction = Interaction::None;
                        true
                    }
                },
                BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => {
                    if operation.cursor < operation.plan.delta_len {
                        let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("pointer commit delta");
                        if let Some(node) = self.nodes.get_mut(operation.plan.id(delta.id)) {
                            operation.changed |= (node.x - delta.x).abs() > 1e-9 || (node.y - delta.y).abs() > 1e-9;
                            node.x = delta.x;
                            node.y = delta.y;
                        }
                        operation.cursor += 1;
                        return false;
                    }
                    if operation.phase == 0 {
                        if operation.changed {
                            self.bump_content_scene_generation();
                        }
                        operation.phase = 1;
                        return false;
                    }
                    if matches!(operation.plan.kind, BoardPointerPlanKind::FinishDrag) {
                        self.interaction = Interaction::None;
                    }
                    true
                }
                BoardPointerPlanKind::SelectionPreview { start, start_screen } => self.step_selection_preview_commit(operation, start, start_screen),
                BoardPointerPlanKind::SelectionCommit => self.step_selection_commit(operation),
                BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world, activated, start_screen } => match operation.phase {
                    0 => {
                        if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                            operation.faulted = true;
                            return true;
                        }
                        operation.phase = 1;
                        false
                    }
                    1 => {
                        self.link_compat_nodes_emit_key = Some(operation.plan.id(compat_key).to_owned());
                        operation.phase = 2;
                        false
                    }
                    2 => {
                        self.link_target_ring_emit_key = Some(operation.plan.id(ring_key).to_owned());
                        operation.phase = 3;
                        false
                    }
                    3 => {
                        self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                        self.hovered_kind = None;
                        operation.phase = 4;
                        false
                    }
                    4 => {
                        let source_id = operation.plan.id(source).to_owned();
                        let target_id = target.map(|span| operation.plan.id(span).to_owned());
                        self.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                        operation.phase = 5;
                        false
                    }
                    _ => {
                        self.bump_content_scene_generation();
                        true
                    }
                },
                BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key } => match operation.phase {
                    0 => {
                        if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                            operation.faulted = true;
                            return true;
                        }
                        operation.phase = 1;
                        false
                    }
                    1 => {
                        if let (Some(target), Some(edge)) = (target, edge) {
                            let source_id = operation.plan.id(source).to_owned();
                            let target_id = operation.plan.id(target).to_owned();
                            let edge_id = operation.plan.id(edge).to_owned();
                            let Some(source_row) = self.handles.get(&source_id) else {
                                operation.faulted = true;
                                return true;
                            };
                            let Some(target_row) = self.handles.get(&target_id) else {
                                operation.faulted = true;
                                return true;
                            };
                            let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
                            self.edges.insert(
                                edge_id.clone(),
                                EdgeData { id: edge_id, source: source_id, target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: graph::PropertyBag::new() },
                            );
                        }
                        operation.phase = 2;
                        false
                    }
                    2 => {
                        self.link_compat_nodes_emit_key = compat_key.map(|span| operation.plan.id(span).to_owned());
                        operation.phase = 3;
                        false
                    }
                    3 => {
                        self.link_target_ring_emit_key = ring_key.map(|span| operation.plan.id(span).to_owned());
                        operation.phase = 4;
                        false
                    }
                    4 => {
                        self.interaction =
                            if let Some(target_node) = target_node { Interaction::LinkTargetNode { source_id: operation.plan.id(source).to_owned(), target_node_id: operation.plan.id(target_node).to_owned() } } else { Interaction::None };
                        operation.phase = 5;
                        false
                    }
                    5 => {
                        self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                        self.hovered_kind = None;
                        operation.phase = 6;
                        false
                    }
                    _ => {
                        self.bump_content_scene_generation();
                        true
                    }
                },
                BoardPointerPlanKind::LinkRetain { hover } | BoardPointerPlanKind::Hover { hover } => {
                    self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                    self.hovered_kind = None;
                    true
                }
                BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => self.step_brush_pointer_commit(operation, source, hover, alt, commit_old),
                BoardPointerPlanKind::LeaveIdle => {
                    self.hovered_id = None;
                    self.hovered_kind = None;
                    true
                }
            }
        }

        fn step_selection_preview_commit(&mut self, operation: &mut BoardPointerCommitOperation, start: Point, start_screen: Point) -> bool {
            match operation.phase {
                0 => {
                    if !matches!(self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                        operation.faulted = true;
                        return true;
                    }
                    operation.phase = 1;
                    false
                }
                1 => {
                    if let Some(id) = self.preselect.pop_first() {
                        self.set_pointer_selection_flag(&id, false);
                        return false;
                    }
                    operation.phase = 2;
                    false
                }
                2 => {
                    if self.preselect_removed.pop_first().is_some() {
                        return false;
                    }
                    operation.phase = 3;
                    false
                }
                3 => {
                    if operation.cursor < operation.plan.delta_len {
                        let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection preview id");
                        let id = operation.plan.id(delta.id).to_owned();
                        self.preselect.insert(id.clone());
                        self.set_pointer_selection_flag(&id, true);
                        operation.cursor += 1;
                        return false;
                    }
                    operation.cursor = 0;
                    operation.phase = 4;
                    false
                }
                4 => {
                    let next = match &self.interaction {
                        Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => match operation.scan_after.as_ref() {
                            Some(after) => initial_ids.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next().cloned(),
                            None => initial_ids.first().cloned(),
                        },
                        _ => None,
                    };
                    if let Some(id) = next {
                        operation.scan_after = Some(id.clone());
                        if !self.preselect.contains(&id) {
                            self.preselect_removed.insert(id);
                        }
                        return false;
                    }
                    operation.scan_after = None;
                    operation.phase = 5;
                    false
                }
                5 => {
                    if operation.cursor < operation.plan.point_len {
                        let index = usize::from(operation.cursor);
                        operation.points.push(operation.plan.points[index]);
                        operation.screen_points.push(operation.plan.screen_points[index]);
                        operation.cursor += 1;
                        return false;
                    }
                    operation.cursor = 0;
                    operation.phase = 6;
                    false
                }
                6 => {
                    if operation.screen_points.len() < 2 {
                        operation.phase = 7;
                        return false;
                    }
                    if self.selection_options.method == "lasso" {
                        if usize::from(operation.cursor) < operation.screen_points.len() {
                            operation.overlay_points.push(operation.screen_points[usize::from(operation.cursor)]);
                            operation.cursor += 1;
                            return false;
                        }
                    } else if operation.cursor < 4 {
                        let last = *operation.screen_points.last().unwrap_or(&start_screen);
                        let point = match operation.cursor {
                            0 => start_screen,
                            1 => Point::new(last.x, start_screen.y),
                            2 => last,
                            _ => Point::new(start_screen.x, last.y),
                        };
                        operation.overlay_points.push(point);
                        operation.cursor += 1;
                        return false;
                    }
                    operation.phase = 7;
                    false
                }
                7 => {
                    let previous = std::mem::replace(&mut self.interaction, Interaction::None);
                    let initial_ids = match previous {
                        Interaction::SelectionPending { initial_ids, .. } => initial_ids,
                        Interaction::Selection { initial_ids, points, screen_points, .. } => {
                            operation.retiring_points = points;
                            operation.retiring_screen_points = screen_points;
                            initial_ids
                        }
                        _ => {
                            operation.faulted = true;
                            return true;
                        }
                    };
                    self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, &operation.screen_points);
                    operation.retiring_overlay_points = self.selection_screen_preview.take().unwrap_or_default();
                    self.selection_screen_preview = (!operation.overlay_points.is_empty()).then(|| std::mem::take(&mut operation.overlay_points));
                    self.interaction = Interaction::Selection { initial_ids, points: std::mem::take(&mut operation.points), screen_points: std::mem::take(&mut operation.screen_points), start, start_screen };
                    if let Some((ids, removed, gesture)) = self.last_preselect_emit_sig.take() {
                        operation.retiring_signature_b = ids;
                        operation.retiring_signature_c = removed;
                        operation.retiring_gestures[1] = gesture;
                    }
                    operation.phase = 8;
                    false
                }
                8 => {
                    if operation.retire_one() {
                        return false;
                    }
                    true
                }
                _ => true,
            }
        }

        fn step_selection_commit(&mut self, operation: &mut BoardPointerCommitOperation) -> bool {
            match operation.phase {
                0 => {
                    let previous = std::mem::replace(&mut self.interaction, Interaction::None);
                    match previous {
                        Interaction::SelectionPending { initial_ids, .. } => operation.retiring_ids = initial_ids,
                        Interaction::Selection { initial_ids, points, screen_points, .. } => {
                            operation.retiring_ids = initial_ids;
                            operation.retiring_points = points;
                            operation.retiring_screen_points = screen_points;
                        }
                        other => {
                            self.interaction = other;
                            operation.faulted = true;
                            return true;
                        }
                    }
                    operation.phase = 1;
                    false
                }
                1 => {
                    if let Some(id) = self.selection.pop_first() {
                        self.set_pointer_selection_flag(&id, false);
                        return false;
                    }
                    operation.phase = 2;
                    false
                }
                2 => {
                    if let Some(id) = self.preselect.pop_first() {
                        self.set_pointer_selection_flag(&id, false);
                        return false;
                    }
                    operation.phase = 3;
                    false
                }
                3 => {
                    if self.preselect_removed.pop_first().is_some() {
                        return false;
                    }
                    operation.phase = 4;
                    false
                }
                4 => {
                    if self.selection_exit_highlight.pop_first().is_some() {
                        return false;
                    }
                    operation.phase = 5;
                    false
                }
                5 => {
                    if operation.retire_one() {
                        return false;
                    }
                    operation.phase = 6;
                    false
                }
                6 => {
                    if operation.cursor < operation.plan.delta_len {
                        let delta = operation.plan.deltas[usize::from(operation.cursor)].expect("selection commit id");
                        let id = operation.plan.id(delta.id).to_owned();
                        self.selection.insert(id.clone());
                        self.set_pointer_selection_flag(&id, true);
                        operation.cursor += 1;
                        return false;
                    }
                    operation.phase = 7;
                    false
                }
                7 => {
                    if let Some((ids, gesture)) = self.last_select_emit_sig.take() {
                        operation.retiring_signature_a = ids;
                        operation.retiring_gestures[0] = gesture;
                    }
                    if let Some((ids, removed, gesture)) = self.last_preselect_emit_sig.take() {
                        operation.retiring_signature_b = ids;
                        operation.retiring_signature_c = removed;
                        operation.retiring_gestures[1] = gesture;
                    }
                    operation.retiring_overlay_points = self.selection_screen_preview.take().unwrap_or_default();
                    self.selection_preview_crossing = false;
                    operation.phase = 8;
                    false
                }
                8 => {
                    if operation.retire_one() {
                        return false;
                    }
                    operation.phase = 9;
                    false
                }
                _ => {
                    self.bump_content_scene_generation();
                    true
                }
            }
        }

        fn step_brush_pointer_commit(&mut self, operation: &mut BoardPointerCommitOperation, source: Option<BoardPointerSpan>, hover: Option<BoardPointerSpan>, alt: bool, commit_old: bool) -> bool {
            match operation.phase {
                0 => {
                    if self.active_utility != ActiveUtility::Brush || matches!(self.interaction, Interaction::Pan { .. }) {
                        operation.faulted = true;
                        return true;
                    }
                    operation.phase = 1;
                    false
                }
                1 => {
                    if operation.cursor < operation.plan.delta_len {
                        let candidate = operation.plan.deltas[usize::from(operation.cursor)].expect("brush candidate");
                        if operation.brush_candidates.push(operation.plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                            operation.faulted = true;
                            return true;
                        }
                        operation.cursor += 1;
                        return false;
                    }
                    operation.phase = 2;
                    false
                }
                2 => {
                    if commit_old {
                        self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
                    }
                    operation.phase = 3;
                    false
                }
                3 => {
                    self.brush_alt_pressed = alt;
                    self.brush_slot_suggestions_active = false;
                    operation.phase = 4;
                    false
                }
                4 => {
                    self.brush_slot_source_id = source.map(|span| operation.plan.id(span).to_owned());
                    operation.phase = 5;
                    false
                }
                5 => {
                    self.brush_candidates = std::mem::take(&mut operation.brush_candidates);
                    self.brush_candidate_index = 0;
                    operation.phase = 6;
                    false
                }
                6 => {
                    let offset = if alt { self.suggestion_offset } else { 0.0 };
                    self.brush_preview = self.brush_slot_source_id.as_deref().and_then(|source_id| self.brush_candidates.first().and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
                    operation.phase = 7;
                    false
                }
                7 => {
                    self.brush_preview_emit_key = None;
                    operation.phase = 8;
                    false
                }
                8 => {
                    self.brush_candidates_emit_key = None;
                    operation.phase = 9;
                    false
                }
                9 => {
                    self.hovered_id = hover.map(|span| operation.plan.id(span).to_owned());
                    self.hovered_kind = None;
                    operation.phase = 10;
                    false
                }
                10 => {
                    self.interaction = Interaction::None;
                    operation.phase = 11;
                    false
                }
                _ => {
                    self.bump_content_scene_generation();
                    true
                }
            }
        }

        pub fn close_event_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
            if context.should_yield() {
                return false;
            }
            if let Some(planning) = self.pending_delete_planning.as_mut() {
                planning.cancelling = true;
            }
            if self.pending_delete_planning.is_some() {
                let _ = self.step_delete_planning();
                context.consume_fuel(1);
                return false;
            }
            if let Some(operation) = self.pending_delete_operation.as_mut() {
                if !operation.claimed {
                    operation.cancelling = true;
                }
            }
            if self.pending_delete_operation.is_some() {
                let _ = self.step_event_authority(context);
                return false;
            }
            if self.event_overflow.take().is_some() {
                context.consume_fuel(1);
                return false;
            }
            if let Some(batch) = self.event_batch_overflow.as_mut() {
                let _ = batch.pop();
                if batch.is_empty() {
                    self.event_batch_overflow = None;
                }
                context.consume_fuel(1);
                return false;
            }
            let closed = self.events.close_step();
            context.consume_fuel(1);
            closed
        }

        pub fn event_authority_terminal_is_empty(&self) -> bool {
            self.pending_delete_planning.is_none() && self.pending_delete_operation.is_none() && self.event_overflow.is_none() && self.event_batch_overflow.is_none() && self.events.terminal_is_empty()
        }

        fn is_preselecting(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }

        /// @emoji 💠️ Live area-select preview ids, or committed selection when not preselecting.
        fn selection_chrome_ids(&self) -> BTreeSet<String> {
            if self.is_preselecting() || !self.preselect.is_empty() {
                self.preselect.clone()
            } else {
                self.selection.clone()
            }
        }

        /// @emoji 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
        fn clear_selection_on_background_click(&mut self) {
            if self.selection.is_empty() {
                return;
            }
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &[], None, None, None) else {
                return;
            };
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.last_select_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection.clear();
            self.sync_selection_flags_to_objects();
            self.publish_event_reservation(reservation);
        }

        fn sync_selection_flags_to_objects(&mut self) {
            let chrome = self.selection_chrome_ids();
            for n in self.nodes.values_mut() {
                n.selected = chrome.contains(&n.id);
            }
            for h in self.handles.values_mut() {
                h.selected = chrome.contains(&h.id);
            }
            for e in self.edges.values_mut() {
                e.selected = chrome.contains(&e.id);
            }
            for w in self.wires.values_mut() {
                w.selected = chrome.contains(&w.id);
            }
        }

        fn push_select_event(&mut self) {
            self.last_select_emit_sig = None;
            let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
            sorted.sort();
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
                return;
            };
            self.publish_event_reservation(reservation);
        }

        pub fn set_selection_ids(&mut self, ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            if next == self.selection {
                return;
            }
            let mut sorted: Vec<_> = next.iter().cloned().collect();
            sorted.sort();
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, None) else {
                return;
            };
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
            self.last_select_emit_sig = None;
            self.publish_event_reservation(reservation);
        }

        /// @emoji 🔇️ Updates committed selection without emitting `select` (controlled React sync).
        pub fn set_selection_ids_silent(&mut self, ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            if next == self.selection {
                return;
            }
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
        }

        /// @emoji 🔇️ Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
        pub fn set_preselect_state_silent(&mut self, ids: &[String], removed_ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
            if self.preselect == next && self.preselect_removed == removed {
                return;
            }
            self.preselect = next;
            self.preselect_removed = removed;
            self.sync_selection_flags_to_objects();
        }

        fn set_selection_ids_gestured(&mut self, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let mut sorted: Vec<_> = next.iter().cloned().collect();
            sorted.sort();
            let gesture_owned = gesture.map(ToOwned::to_owned);
            let sig = (sorted.clone(), gesture_owned.clone());
            if next == self.selection && self.last_select_emit_sig.as_ref() == Some(&sig) {
                return;
            }
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, None, None, gesture) else {
                return;
            };
            self.last_select_emit_sig = Some(sig);
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            if next != self.selection {
                self.selection_exit_highlight.clear();
                self.selection = next;
                self.sync_selection_flags_to_objects();
            }
            self.publish_event_reservation(reservation);
        }

        /// @emoji 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
        fn apply_area_preselect(&mut self, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let sorted = Self::sorted_selection_ids(&next);
            let removed = Self::sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
            let gesture_owned = gesture.map(ToOwned::to_owned);
            let sig = (sorted.clone(), removed.clone(), gesture_owned.clone());
            if self.preselect == next && self.last_preselect_emit_sig.as_ref() == Some(&sig) {
                return;
            }
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Preselect, &sorted, None, Some(&removed), gesture) else {
                return;
            };
            self.last_preselect_emit_sig = Some(sig);
            self.preselect = next;
            self.preselect_removed = anchor_ids.difference(&self.preselect).cloned().collect();
            self.set_hovered_id_silent(None);
            self.sync_selection_flags_to_objects();
            self.publish_event_reservation(reservation);
        }

        fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
            let mut v: Vec<_> = set.iter().cloned().collect();
            v.sort();
            v
        }

        /// @emoji 🧿️ Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
        fn commit_area_select_from_initial(&mut self, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let sorted = Self::sorted_selection_ids(&next);
            let anchor = Self::sorted_selection_ids(initial_ids);
            let gesture_owned = gesture.map(ToOwned::to_owned);
            let Some(reservation) = self.reserve_selection_event(BoardEventKind::Select, &sorted, Some(&anchor), None, gesture) else {
                return;
            };
            self.last_select_emit_sig = None;
            self.last_preselect_emit_sig = None;
            self.preselect.clear();
            self.preselect_removed.clear();
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
            let _ = gesture_owned;
            self.publish_event_reservation(reservation);
        }

        /// @emoji 🧿️ True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
        pub fn is_dragging_area_select(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }

        /// @emoji 🧿️ True during area select, link gestures, node drag, or camera pan so JS can defer full `syncDescriptorJson` round-trips.
        pub fn defers_descriptor_sync_from_js(&self) -> bool {
            matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. } | Interaction::DragNodes { .. } | Interaction::Pan { .. })
        }

        pub fn world_to_screen(&self, p: Point) -> Point {
            infinite::canvas::camera::world_to_screen(&self.camera, &self.viewport(), p)
        }

        pub fn screen_to_world(&self, p: Point) -> Point {
            infinite::canvas::camera::screen_to_world(&self.camera, &self.viewport(), p)
        }

        fn node_kind_scale(&self, node_kind: &str) -> f64 {
            self.node_kinds.get(node_kind).map(|k| k.scale).unwrap_or(1.0)
        }

        fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
            self.handle_kinds.get(handle_kind).map(|k| k.scale).unwrap_or(1.0)
        }

        fn effective_node_scale(&self, n: &NodeData) -> f64 {
            (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
        }

        fn scaled_node_radius(&self, n: &NodeData) -> f64 {
            n.radius * self.effective_node_scale(n)
        }

        fn scaled_node_width(&self, n: &NodeData) -> f64 {
            n.width * self.effective_node_scale(n)
        }

        fn scaled_node_height(&self, n: &NodeData) -> f64 {
            n.height * self.effective_node_scale(n)
        }

        fn effective_handle_scale(&self, h: &HandleData) -> f64 {
            let node_scale = self.nodes.get(h.node_id.as_str()).map(|n| self.effective_node_scale(n)).unwrap_or(1.0);
            (node_scale * h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
        }

        pub(crate) fn effective_handle_radius(&self, h: &HandleData) -> f64 {
            h.radius * self.effective_handle_scale(h)
        }

        pub(crate) fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
            let n = self.nodes.get(&h.node_id)?;
            Some(match n.shape {
                NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n), h.angle),
                NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n), self.scaled_node_height(n), h.angle),
            })
        }

        /// @emoji 📐️ Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
        fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
            match n.shape {
                NodeShape::Circle => self.scaled_node_radius(n),
                NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
            }
        }

        /// @emoji 📐️ Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
        fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
            (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
        }

        /// @emoji 📐️ Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
        pub fn indirect_handle_world_pos(&self, h: &HandleData) -> Option<Point> {
            let n = self.nodes.get(&h.node_id)?;
            let offset = self.indirect_handle_ring_offset_world(n);
            Some(match n.shape {
                NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n) + offset, h.angle),
                NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n) + 2.0 * offset, self.scaled_node_height(n) + 2.0 * offset, h.angle),
            })
        }

        /// @emoji 📐️ Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
        pub fn indirect_handle_marker_radius_world(&self, h: &HandleData) -> f64 {
            let Some(n) = self.nodes.get(&h.node_id) else {
                return (self.effective_handle_radius(h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
            };
            let handle_local_scale = (h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
            (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
        }

        /// @emoji 🧭️ Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
        fn active_link_source_handle_id(&self) -> Option<&str> {
            match &self.interaction {
                Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
                _ => None,
            }
        }

        /// @emoji 🧭️ Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
        fn link_drag_compatible_target_node_ids(&self, source_handle_id: &str) -> Vec<String> {
            let Some(source) = self.handles.get(source_handle_id) else {
                return Vec::new();
            };
            let source_node_id = source.node_id.as_str();
            let mut out = Vec::new();
            let mut seen = BTreeSet::new();
            for (hid, h) in &self.handles {
                if h.node_id == source_node_id || !self.handle_effectively_visible(hid.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source, h) {
                    continue;
                }
                if !self.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
                    continue;
                }
                if seen.insert(h.node_id.clone()) {
                    out.push(h.node_id.clone());
                }
            }
            out.sort();
            out
        }

        /// @emoji 🧭️ Count of visible free handles on `node_id` compatible with `source_handle_id`.
        fn link_compatible_handle_count_on_node(&self, source_handle_id: &str, node_id: &str) -> usize {
            let Some(source) = self.handles.get(source_handle_id) else {
                return 0;
            };
            if source.node_id == node_id {
                return 0;
            }
            self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) && self.handles_link_compatible_for_drag(source, h)).count()
        }

        /// @emoji 🧭️ Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
        fn link_compatible_handle_ids_on_node(&self, source_handle_id: &str, node_id: &str) -> Vec<String> {
            let Some(source) = self.handles.get(source_handle_id) else {
                return Vec::new();
            };
            let mut out: Vec<String> = self
                .handles
                .iter()
                .filter_map(|(id, h)| {
                    if h.node_id != node_id {
                        return None;
                    }
                    if !self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) {
                        return None;
                    }
                    self.handles_link_compatible_for_drag(source, h).then(|| id.clone())
                })
                .collect();
            out.sort();
            out
        }

        /// @emoji 🧭️ Compatible target node under `world` while a link wire is active (node body hit).
        fn link_drag_ring_target_node_id(&self, source_handle_id: &str, world: Point) -> Option<String> {
            let nid = self.resolve_node_hit_world(world)?;
            if self.handles.get(source_handle_id)?.node_id == nid {
                return None;
            }
            self.node_has_any_free_link_compatible_handle(source_handle_id, nid.as_str()).then_some(nid)
        }

        /// @emoji 🧭️ Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
        fn indirect_ring_node_id(&self, lod: BoardDrawLod) -> Option<String> {
            if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                return None;
            }
            if let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction {
                if self.link_compatible_handle_count_on_node(source_id, target_node_id) > 1 {
                    return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
                }
                return None;
            }
            if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &self.interaction {
                if ring_handle_ids.len() > 1 {
                    return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
                }
                return None;
            }
            if let Interaction::LinkDragSnap { source_id, end_world, .. } = &self.interaction {
                let ring_nid = self.link_drag_ring_target_node_id(source_id, *end_world)?;
                if self.link_compatible_handle_count_on_node(source_id, ring_nid.as_str()) > 1 {
                    return Some(ring_nid);
                }
                return None;
            }
            if self.active_link_source_handle_id().is_some() {
                return None;
            }
            let ring_nid = if self.selection.len() == 1 {
                self.selection.iter().next()?.clone()
            } else {
                return None;
            };
            let n = self.nodes.get(&ring_nid).filter(|n| n.visible)?;
            if self.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
                Some(ring_nid)
            } else {
                None
            }
        }

        fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
            self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
        }

        /// @emoji 🧭️ Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
        fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
            let mut found: Option<String> = None;
            for (id, h) in &self.handles {
                if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
                    continue;
                }
                if found.is_some() {
                    return None;
                }
                found = Some(id.clone());
            }
            found
        }

        /// @emoji 🧭️ When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
        fn node_sole_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> Option<String> {
            let source = self.handles.get(source_handle_id)?;
            if source.node_id == target_node_id {
                return None;
            }
            let mut found: Option<String> = None;
            for (id, h) in &self.handles {
                if h.node_id != target_node_id || !self.handle_effectively_visible(id.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(id.as_str()) {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source, h) {
                    continue;
                }
                if found.is_some() {
                    return None;
                }
                found = Some(id.clone());
            }
            found
        }

        fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
                }
                NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
            }
        }

        fn sole_indirect_handle_hit_link_target(&self, point: Point) -> Option<String> {
            let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction else {
                return None;
            };
            let th = self.node_sole_free_link_compatible_handle(source_id, target_node_id)?;
            let n = self.nodes.get(target_node_id)?;
            if !n.visible {
                return None;
            }
            if !self.point_in_node_world(n, point) {
                return None;
            }
            Some(th)
        }

        fn sole_indirect_handle_hit_idle_selected_node(&self, point: Point) -> Option<String> {
            if !matches!(self.interaction, Interaction::None) {
                return None;
            }
            if self.selection.len() != 1 {
                return None;
            }
            let nid = self.selection.iter().next()?;
            if !self.nodes.contains_key(nid) {
                return None;
            }
            let sole = self.sole_eligible_indirect_handle_on_node(nid)?;
            let n = self.nodes.get(nid)?;
            if !n.visible {
                return None;
            }
            if !self.point_in_node_world(n, point) {
                return None;
            }
            Some(sole)
        }

        /// @emoji 🧭️ True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
        fn node_has_any_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> bool {
            let Some(source) = self.handles.get(source_handle_id) else {
                return false;
            };
            if source.node_id == target_node_id {
                return false;
            }
            for (hid, h) in &self.handles {
                if h.node_id != target_node_id || !self.handle_effectively_visible(hid.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                if self.handles_link_compatible_for_drag(source, h) {
                    return true;
                }
            }
            false
        }

        /// @emoji 💫️ True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
        fn handle_eligible_link_target_ring(&self, handle_id: &str, source_handle_id: &str) -> bool {
            if !self.handle_effectively_visible(handle_id) || self.handle_has_incident_edge(handle_id) {
                return false;
            }
            let Some(source) = self.handles.get(source_handle_id) else {
                return false;
            };
            let Some(target) = self.handles.get(handle_id) else {
                return false;
            };
            if source.node_id == target.node_id {
                return false;
            }
            self.handles_link_compatible_for_drag(source, target)
        }

        fn indirect_ring_handle_eligible(&self, handle_id: &str, ring_node_id: &str) -> bool {
            if self.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
                return false;
            }
            if let Some(source_id) = self.active_link_source_handle_id() {
                self.handle_eligible_link_target_ring(handle_id, source_id)
            } else {
                self.handle_eligible_indirect_connect_ring(handle_id)
            }
        }

        fn link_drag_target_ring_hit(&self, source_id: &str, point: Point) -> Option<String> {
            if !matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                return None;
            }
            if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &self.interaction {
                if active_source != source_id {
                    return None;
                }
                let zoom = self.camera.zoom;
                for hid in ring_handle_ids {
                    let Some(h) = self.handles.get(hid) else {
                        continue;
                    };
                    if !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else {
                        continue;
                    };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
                return None;
            }
            let node_id = self.link_drag_ring_target_node_id(source_id, point)?;
            if self.link_compatible_handle_count_on_node(source_id, node_id.as_str()) <= 1 {
                return None;
            }
            let zoom = self.camera.zoom;
            for h in self.handles.values().rev() {
                if h.node_id != node_id || !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                    continue;
                }
                let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
            None
        }

        fn link_target_ring_snapshot(&self, source_handle_id: &str) -> (Option<String>, Vec<String>) {
            if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &self.interaction {
                if source_id == source_handle_id {
                    return (ring_node_id.clone(), ring_handle_ids.clone());
                }
            }
            let node_id = match &self.interaction {
                Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
                Interaction::LinkDragSnap { end_world, .. } => self.link_drag_ring_target_node_id(source_handle_id, *end_world),
                _ => None,
            };
            let Some(nid) = node_id else {
                return (None, Vec::new());
            };
            if self.link_compatible_handle_count_on_node(source_handle_id, nid.as_str()) <= 1 {
                return (None, Vec::new());
            }
            (Some(nid.clone()), self.link_compatible_handle_ids_on_node(source_handle_id, nid.as_str()))
        }

        fn sync_link_gesture_events(&mut self) {
            if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = &self.interaction {
                if compatible_node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
                    self.event_schema_fault = true;
                    return;
                }
                let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
                let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
                let compat_changed = self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
                let ring_changed = self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
                let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(source_id, compatible_node_ids));
                let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), ring_handle_ids));
                let (first, second) = match (compat_event, ring_event) {
                    (Some(first), second) => (first, second),
                    (None, Some(first)) => (first, None),
                    (None, None) => return,
                };
                let Some(reservation) = self.reserve_owned_batch(first, second) else {
                    return;
                };
                if compat_changed {
                    self.link_compat_nodes_emit_key = Some(compat_key);
                }
                if ring_changed {
                    self.link_target_ring_emit_key = Some(ring_key);
                }
                self.publish_event_batch(reservation);
                return;
            }
            let Some(source) = self.active_link_source_handle_id().map(str::to_string) else {
                self.clear_link_gesture_events();
                return;
            };
            let node_ids = self.link_drag_compatible_target_node_ids(&source);
            if node_ids.len() > BOARD_POINTER_ITEM_CAPACITY {
                self.event_schema_fault = true;
                return;
            }
            let compat_key = format!("{}|{}", source, node_ids.join(","));
            let (ring_node_id, ring_handle_ids) = self.link_target_ring_snapshot(&source);
            if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
                self.event_schema_fault = true;
                return;
            }
            let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
            let compat_changed = self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str());
            let ring_changed = self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str());
            let compat_event = compat_changed.then(|| BoardOwnedEvent::link_compatible(&source, &node_ids));
            let ring_event = ring_changed.then(|| BoardOwnedEvent::link_ring(&source, ring_node_id.as_deref(), &ring_handle_ids));
            let (first, second) = match (compat_event, ring_event) {
                (Some(first), second) => (first, second),
                (None, Some(first)) => (first, None),
                (None, None) => return,
            };
            let Some(reservation) = self.reserve_owned_batch(first, second) else {
                return;
            };
            if compat_changed {
                self.link_compat_nodes_emit_key = Some(compat_key);
            }
            if ring_changed {
                self.link_target_ring_emit_key = Some(ring_key);
            }
            self.publish_event_batch(reservation);
        }

        fn clear_link_gesture_events(&mut self) {
            let compat = self.link_compat_nodes_emit_key.is_some();
            let ring = self.link_target_ring_emit_key.is_some();
            let empty: &[String] = &[];
            let compat_event = compat.then(|| BoardOwnedEvent::link_compatible("", empty));
            let ring_event = ring.then(|| BoardOwnedEvent::link_ring("", None, empty));
            let (first, second) = match (compat_event, ring_event) {
                (Some(first), second) => (first, second),
                (None, Some(first)) => (first, None),
                (None, None) => return,
            };
            let Some(reservation) = self.reserve_owned_batch(first, second) else {
                return;
            };
            if compat {
                self.link_compat_nodes_emit_key = None;
            }
            if ring {
                self.link_target_ring_emit_key = None;
            }
            self.publish_event_batch(reservation);
        }

        fn node_center_world(&self, node_id: &str) -> Option<Point> {
            let n = self.nodes.get(node_id)?;
            Some(Point::new(n.x, n.y))
        }

        fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
            if !self.has_ports() {
                let source_node = self.nodes.get(&e.source)?;
                let target_node = self.nodes.get(&e.target)?;
                let source_center = Point::new(source_node.x, source_node.y);
                let target_center = Point::new(target_node.x, target_node.y);
                let source_pos = self.node_rim_point_toward(source_node, target_center)?;
                let target_pos = self.node_rim_point_toward(target_node, source_center)?;
                return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
            }
            let source_handle = self.handles.get(&e.source)?;
            let target_handle = self.handles.get(&e.target)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_pos = self.handle_world_pos(source_handle)?;
            let target_pos = self.handle_world_pos(target_handle)?;
            Some(compute_edge_bezier_points(source_pos, target_pos, Point::new(source_node.x, source_node.y), Point::new(target_node.x, target_node.y)))
        }

        fn link_drag_wire_curve_world(&self, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
            let source_handle = self.handles.get(source_id)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let source_pos = self.handle_world_pos(source_handle)?;
            let source_center = Point::new(source_node.x, source_node.y);
            let (target_pos, target_center) = if let Some(tid) = target_id {
                let th = self.handles.get(tid)?;
                let tn = self.nodes.get(&th.node_id)?;
                (self.handle_world_pos(th)?, Point::new(tn.x, tn.y))
            } else {
                (end_world, end_world)
            };
            Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center))
        }

        fn active_link_wire_curve(&self) -> Option<CubicBez> {
            match &self.interaction {
                Interaction::LinkDragSnap { source_id, target_id, end_world } => self.link_drag_wire_curve_world(source_id.as_str(), target_id.as_deref(), *end_world),
                Interaction::LinkTargetNode { source_id, target_node_id } => self.link_drag_wire_curve_world(source_id.as_str(), None, self.node_center_world(target_node_id)?),
                Interaction::ExternalLinkPreview { source_id, end_world, .. } => self.link_drag_wire_curve_world(source_id.as_str(), None, *end_world),
                Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => self.link_drag_wire_curve_world(src.as_str(), Some(tgt.as_str()), Point::ZERO),
                _ => None,
            }
        }

        fn wire_curve(&self, w: &WireData) -> Option<CubicBez> {
            let end_world = match (&w.target, w.end_x, w.end_y) {
                (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
                (Some(tid), _, _) => {
                    self.handles.get(tid)?;
                    return self.edge_curve(&EdgeData {
                        id: w.id.clone(),
                        source: w.source.clone(),
                        target: tid.clone(),
                        selected: w.selected,
                        visible: w.visible,
                        locked: w.locked,
                        style: w.style.clone(),
                        edge_kind: String::new(),
                        source_tip: None,
                        target_tip: None,
                        properties: w.properties.clone(),
                    });
                }
                _ => return None,
            };
            self.link_drag_wire_curve_world(w.source.as_str(), None, end_world)
        }

        fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
            if let Some(tid) = target_handle_id {
                self.set_hovered_id(Some(tid.to_string()));
            } else {
                self.update_hover_from_world(world);
            }
        }

        /// @emoji 🧭️ Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
        fn lod_disables_discrete_pick(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
        }

        /// @emoji 🔗️ Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
        fn resolve_overview_free_link_handle_pointer_world(&self, point: Point) -> Option<String> {
            if !matches!(self.current_draw_lod(), BoardDrawLod::Overview) {
                return None;
            }
            if !self.selection_options.select_handles {
                return None;
            }
            const MAX_D_WORLD: f64 = 2.25;
            let mut best: Option<(f64, String)> = None;
            for h in self.handles.values() {
                if !self.handle_effectively_visible(h.id.as_str()) || self.handle_has_incident_edge(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = self.handle_world_pos(h) else {
                    continue;
                };
                let d = distance_between(point, pos);
                if d <= MAX_D_WORLD && best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                    best = Some((d, h.id.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        /// @emoji 🧭️ Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
        fn lod_uses_bounded_drag(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
        }

        fn resolve_hover_world(&self, point: Point) -> Option<String> {
            let lod = self.current_draw_lod();
            let zoom = self.camera.zoom;
            if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
                if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                        return Some(hid);
                    }
                    if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                        if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                            return Some(hid);
                        }
                    }
                }
                if let Some(ring_node_id) = self.indirect_ring_node_id(lod) {
                    for h in self.handles.values().rev() {
                        if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                            continue;
                        }
                        if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    for h in self.handles.values().rev() {
                        if !self.handle_selectable(h.id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                        if self.handle_selectable(hid.as_str()) {
                            return Some(hid);
                        }
                    }
                }
            }
            for n in self.nodes.values().rev() {
                if !self.node_selectable(n.id.as_str()) {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
            for w in self.wires.values().rev() {
                if !self.wire_selectable(w) {
                    continue;
                }
                if let Some(c) = self.wire_curve(w) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(w.id.clone());
                    }
                }
            }
            for e in self.edges.values().rev() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(e.id.clone());
                    }
                }
            }
            None
        }

        fn push_pick_target(out: &mut Vec<BoardPickTargetJson>, domain: &str, id: String, generality: u32, label: Option<String>) {
            if out.iter().any(|row| row.domain == domain && row.id == id) {
                return;
            }
            out.push(BoardPickTargetJson { domain: domain.to_string(), id, generality, label });
        }

        fn resolve_pick_targets_world(&self, point: Point) -> Vec<BoardPickTargetJson> {
            let mut out = Vec::new();
            let lod = self.current_draw_lod();
            let zoom = self.camera.zoom;
            if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                for h in self.handles.values().rev() {
                    if !self.handle_selectable(h.id.as_str()) {
                        continue;
                    }
                    let Some(pos) = self.handle_world_pos(h) else { continue };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                    if distance_between(point, pos) <= tol {
                        Self::push_pick_target(&mut out, "handle", h.id.clone(), 2, Some(h.id.clone()));
                    }
                }
            }
            for n in self.nodes.values().rev() {
                if !self.node_selectable(n.id.as_str()) {
                    continue;
                }
                let hit = match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
                    }
                    NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
                };
                if hit {
                    Self::push_pick_target(&mut out, "node", n.id.clone(), 0, n.text.clone());
                }
            }
            for w in self.wires.values().rev() {
                if !self.wire_selectable(w) {
                    continue;
                }
                if let Some(c) = self.wire_curve(w) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        Self::push_pick_target(&mut out, "wire", w.id.clone(), 1, Some(w.id.clone()));
                    }
                }
            }
            for e in self.edges.values().rev() {
                if !self.edge_selectable(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        Self::push_pick_target(&mut out, "edge", e.id.clone(), 1, Some(e.id.clone()));
                    }
                }
            }
            out
        }

        /// @emoji 🎯️ All pick targets under a screen point as JSON (`domain`, `id`, `generality`).
        pub fn pick_targets_at_screen_json(&self, sx: f64, sy: f64) -> String {
            let world = self.screen_to_world(Point::new(sx, sy));
            serde_json::to_string(&self.resolve_pick_targets_world(world)).unwrap_or_else(|_| "[]".into())
        }

        pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
            if self.lod_disables_discrete_pick() {
                return None;
            }
            let zoom = self.camera.zoom;
            let o = &self.selection_options;
            if self.has_ports() && o.select_handles {
                if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                        return Some(hid);
                    }
                    if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                        if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                            return Some(hid);
                        }
                    }
                }
                if let Some(ring_node_id) = self.indirect_ring_node_id(self.current_draw_lod()) {
                    for h in self.handles.values().rev() {
                        if h.node_id != ring_node_id || !self.handle_selectable(h.id.as_str()) {
                            continue;
                        }
                        if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    for h in self.handles.values().rev() {
                        if !self.handle_selectable(h.id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                        return Some(hid);
                    }
                }
            }
            if o.select_nodes {
                for n in self.nodes.values().rev() {
                    if !self.node_selectable(n.id.as_str()) {
                        continue;
                    }
                    match n.shape {
                        NodeShape::Rectangle => {
                            let hw = self.scaled_node_width(n) / 2.0;
                            let hh = self.scaled_node_height(n) / 2.0;
                            if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                                return Some(n.id.clone());
                            }
                        }
                        NodeShape::Circle => {
                            if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                                return Some(n.id.clone());
                            }
                        }
                    }
                }
            }
            if o.select_edges {
                for e in self.edges.values().rev() {
                    if !self.edge_selectable(e) {
                        continue;
                    }
                    if let Some(c) = self.edge_curve(e) {
                        if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                            return Some(e.id.clone());
                        }
                    }
                }
            }
            None
        }

        fn resolve_node_hit_world(&self, point: Point) -> Option<String> {
            for n in self.nodes.values().rev() {
                if !n.visible {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
            None
        }

        pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) -> Result<(), NormalPortError> {
            let entity_count = desc.nodes.len().checked_add(desc.handles.len()).and_then(|count| count.checked_add(desc.edges.len())).and_then(|count| count.checked_add(desc.wires.len())).ok_or(NormalPortError::EventCredits)?;
            if entity_count > BOARD_POINTER_ITEM_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            let entity_id_bytes = desc
                .nodes
                .iter()
                .map(|node| node.id.len())
                .chain(desc.handles.iter().map(|handle| handle.id.len()))
                .chain(desc.edges.iter().map(|edge| edge.id.len()))
                .chain(desc.wires.iter().map(|wire| wire.id.len()))
                .try_fold(0usize, usize::checked_add)
                .ok_or(NormalPortError::EventCredits)?;
            if entity_id_bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(NormalPortError::EventCredits);
            }
            if matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
            let mut created_edge_count = 0usize;
            let mut created_edge_bytes = 0usize;
            for edge in desc.edges.iter().filter(|edge| !self.edges.contains_key(&edge.id)) {
                created_edge_count = created_edge_count.checked_add(1).ok_or(NormalPortError::EventCredits)?;
                let bytes = board_edge_event_owned_bytes(&edge.id, &edge.source, &edge.target).ok_or(NormalPortError::EventCredits)?;
                created_edge_bytes = created_edge_bytes.checked_add(bytes).ok_or(NormalPortError::EventCredits)?;
            }
            self.events.reserve(created_edge_count, created_edge_bytes).map_err(|_| NormalPortError::EventCredits)?;
            let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
            let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
            let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
            let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
            self.edges.retain(|id, _| want_edges.contains(id));
            self.wires.retain(|id, _| want_wires.contains(id));
            self.handles.retain(|id, _| want_handles.contains(id));
            self.nodes.retain(|id, _| want_nodes.contains(id));
            for n in &desc.nodes {
                let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
                let (radius, width, height) = match shape {
                    NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
                    NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
                };
                let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
                let properties = n.user_data.as_ref().map(property_bag_from_json).unwrap_or_default();
                self.nodes.insert(
                    n.id.clone(),
                    NodeData {
                        id: n.id.clone(),
                        x: n.x,
                        y: n.y,
                        shape,
                        radius,
                        width,
                        height,
                        scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                        draggable: n.draggable.unwrap_or(true),
                        selected: n.selected.unwrap_or(false),
                        visible: n.visible.unwrap_or(true),
                        locked: n.locked.unwrap_or(false),
                        root: n.root.unwrap_or(false),
                        style: n.style.clone(),
                        text: n.text.clone(),
                        icon_kind: n.icon_kind.clone(),
                        node_kind,
                        properties,
                    },
                );
            }
            for h in &desc.handles {
                let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
                let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    None => None,
                    Some(s) => Some(Self::parse_css_color(s).ok_or_else(|| NormalPortError::InvalidHandleColor(h.id.clone(), s.to_string()))?),
                };
                let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let properties = h.user_data.as_ref().map(property_bag_from_json).unwrap_or_default();
                self.handles.insert(
                    h.id.clone(),
                    HandleData {
                        id: h.id.clone(),
                        node_id: h.node_id.clone(),
                        angle: h.angle,
                        radius: h.radius.unwrap_or(ui_styling::radii::HANDLE_DEFAULT),
                        scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                        selected: h.selected.unwrap_or(false),
                        visible: h.visible.unwrap_or(true),
                        locked: h.locked.unwrap_or(false),
                        style: h.style.clone(),
                        handle_kind: kind,
                        color_fill,
                        icon_kind,
                        properties,
                    },
                );
            }
            for e in &desc.edges {
                let existed = self.edges.contains_key(&e.id);
                let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
                let source_tip = Self::parse_catalog_tip_slot(e.source_tip.as_deref());
                let target_tip = Self::parse_catalog_tip_slot(e.target_tip.as_deref());
                let properties = e.user_data.as_ref().map(property_bag_from_json).unwrap_or_default();
                self.edges.insert(
                    e.id.clone(),
                    EdgeData {
                        id: e.id.clone(),
                        source: e.source.clone(),
                        target: e.target.clone(),
                        selected: e.selected.unwrap_or(false),
                        visible: e.visible.unwrap_or(true),
                        locked: e.locked.unwrap_or(false),
                        style: e.style.clone(),
                        edge_kind,
                        source_tip,
                        target_tip,
                        properties,
                    },
                );
                if !existed {
                    let event = BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &e.id, &e.source, &e.target).expect("new descriptor edges were exactly preflighted");
                    self.events.push(event).expect("new descriptor edge credits were reserved before mutation");
                }
            }
            for w in &desc.wires {
                let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
                let (end_x, end_y) = match &target {
                    Some(_) => (None, None),
                    None => {
                        let x = match w.end_x {
                            Some(v) if v.is_finite() => Some(v),
                            _ => None,
                        };
                        let y = match w.end_y {
                            Some(v) if v.is_finite() => Some(v),
                            _ => None,
                        };
                        if x.is_none() || y.is_none() {
                            continue;
                        }
                        (x, y)
                    }
                };
                let wire_kind = w
                    .wire_kind
                    .as_ref()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .or_else(|| self.handles.get(w.source.as_str()).map(|h| self.resolve_default_wire_kind_for_handle(h)))
                    .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
                let properties = w.user_data.as_ref().map(property_bag_from_json).unwrap_or_default();
                self.wires.insert(
                    w.id.clone(),
                    WireData {
                        id: w.id.clone(),
                        source: w.source.clone(),
                        target,
                        end_x,
                        end_y,
                        selected: w.selected.unwrap_or(false),
                        visible: w.visible.unwrap_or(true),
                        locked: w.locked.unwrap_or(false),
                        style: w.style.clone(),
                        wire_kind,
                        properties,
                    },
                );
            }
            if !self.is_preselect_active() {
                let mut new_selection = BTreeSet::new();
                for n in &desc.nodes {
                    if n.selected == Some(true) {
                        new_selection.insert(n.id.clone());
                    }
                }
                for h in &desc.handles {
                    if h.selected == Some(true) {
                        new_selection.insert(h.id.clone());
                    }
                }
                for e in &desc.edges {
                    if e.selected == Some(true) {
                        new_selection.insert(e.id.clone());
                    }
                }
                for w in &desc.wires {
                    if w.selected == Some(true) {
                        new_selection.insert(w.id.clone());
                    }
                }
                let prev_sel = self.selection.clone();
                if prev_sel != new_selection {
                    self.selection_exit_highlight.clear();
                }
                self.selection = new_selection;
                if prev_sel != self.selection {
                    self.push_select_event();
                }
            }
            self.sync_selection_flags_to_objects();
            self.bump_content_scene_generation();
            Ok(())
        }

        /// @emoji 📍️ Applies peer-pane node drags without a full descriptor re-sync.
        pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)]) {
            let mut geometry_changed = false;
            for (id, x, y) in moves {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                if let Some(node) = self.nodes.get_mut(id.as_str()) {
                    if (node.x - *x).abs() > 1e-9 || (node.y - *y).abs() > 1e-9 {
                        node.x = *x;
                        node.y = *y;
                        geometry_changed = true;
                    }
                }
            }
            if geometry_changed {
                self.bump_content_scene_generation();
            }
        }

        /// @emoji 📍️ Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
        pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), NormalPortError> {
            #[derive(Deserialize)]
            struct NodePositionMoveJson {
                id: String,
                x: f64,
                y: f64,
            }
            let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json)?;
            let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
            self.set_node_positions(&moves);
            Ok(())
        }

        pub fn clear_scene(&mut self) {
            self.edges.clear();
            self.wires.clear();
            self.handles.clear();
            self.nodes.clear();
            self.selection.clear();
            self.preselect.clear();
            self.preselect_removed.clear();
            self.selection_exit_highlight.clear();
        }

        pub fn parse_fixture_v1(&mut self, raw: &serde_json::Value) -> bool {
            let f: FixtureJson = match serde_json::from_value(raw.clone()) {
                Ok(v) => v,
                Err(_) => return false,
            };
            self.port_mode = match f.schema.as_str() {
                "reasoning.mindmap.fixture" => GraphPortMode::Normal,
                "puzzle.2d.fixture" => GraphPortMode::Ported,
                _ => return false,
            };
            if !self.has_ports() {
                self.selection_options.select_handles = false;
            }
            self.set_camera(f.camera.x, f.camera.y, f.camera.zoom);
            self.clear_scene();
            let mut desc = SceneDescriptorJson::default();
            for entry in f.nodes {
                let Some(obj) = entry.as_object() else {
                    return false;
                };
                let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
                    return false;
                };
                let Some(x) = obj.get("x").and_then(|v| v.as_f64()) else {
                    return false;
                };
                let Some(y) = obj.get("y").and_then(|v| v.as_f64()) else {
                    return false;
                };
                if !x.is_finite() || !y.is_finite() {
                    return false;
                }
                let text = obj.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                if self.has_ports() {
                    let Some(handles_arr) = obj.get("handles").and_then(|v| v.as_array()) else {
                        return false;
                    };
                    let mut handles: Vec<HandleDescJson> = Vec::new();
                    for h in handles_arr {
                        let Some(ho) = h.as_object() else {
                            return false;
                        };
                        let Some(hid) = ho.get("id").and_then(|v| v.as_str()) else {
                            return false;
                        };
                        let Some(angle) = ho.get("angle").and_then(|v| v.as_f64()) else {
                            return false;
                        };
                        if !angle.is_finite() {
                            return false;
                        }
                        let handle_kind = ho.get("handleKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from).unwrap_or_else(|| "port".into());
                        let handle_color = ho.get("color").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                        let handle_icon_kind = ho.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                        let handle_scale = ho.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                        handles.push(HandleDescJson {
                            id: hid.into(),
                            node_id: id.into(),
                            angle,
                            radius: None,
                            scale: handle_scale,
                            selected: None,
                            style: None,
                            handle_kind: Some(handle_kind),
                            color: handle_color,
                            icon_kind: handle_icon_kind,
                            user_data: None,
                            visible: board_json_visible_option(ho),
                            locked: board_json_locked_option(ho),
                        });
                    }
                    desc.handles.extend(handles);
                } else if obj.get("handles").is_some() {
                    return false;
                }
                let shape_str = obj.get("shape").and_then(|v| v.as_str());
                let fixture_node_kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let fixture_node_scale = obj.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                if shape_str == Some("rectangle") {
                    let Some(width) = obj.get("width").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    let Some(height) = obj.get("height").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    if width <= 0.0 || height <= 0.0 {
                        return false;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescJson {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: fixture_node_kind.clone(),
                        user_data: None,
                        visible: board_json_visible_option(obj),
                        locked: board_json_locked_option(obj),
                        root,
                        shape: Some("rectangle".into()),
                        radius: None,
                        width: Some(width),
                        height: Some(height),
                        scale: fixture_node_scale,
                    });
                } else {
                    let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    if radius <= 0.0 {
                        return false;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescJson {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: fixture_node_kind.clone(),
                        user_data: None,
                        visible: board_json_visible_option(obj),
                        locked: board_json_locked_option(obj),
                        root,
                        shape: Some("circle".into()),
                        radius: Some(radius),
                        width: None,
                        height: None,
                        scale: fixture_node_scale,
                    });
                }
            }
            for entry in f.edges {
                let Some(e) = entry.as_object() else {
                    return false;
                };
                let Some(id) = e.get("id").and_then(|v| v.as_str()) else {
                    return false;
                };
                let Some((source, target)) = fixture_edge_handle_ids_from_object(e) else {
                    return false;
                };
                if !self.has_ports() {
                    let node_ids: BTreeSet<&str> = desc.nodes.iter().map(|n| n.id.as_str()).collect();
                    if !node_ids.contains(source) || !node_ids.contains(target) {
                        return false;
                    }
                }
                let edge_kind = e.get("edgeKind").or_else(|| e.get("edge_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let source_tip = e.get("sourceTip").or_else(|| e.get("source_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let target_tip = e.get("targetTip").or_else(|| e.get("target_tip")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                desc.edges.push(EdgeDescJson {
                    id: id.into(),
                    source: source.into(),
                    target: target.into(),
                    edge_kind,
                    source_tip,
                    target_tip,
                    selected: None,
                    style: None,
                    user_data: None,
                    visible: board_json_visible_option(e),
                    locked: board_json_locked_option(e),
                });
            }
            if self.sync_descriptor(&desc).is_err() {
                return false;
            }
            true
        }

        fn drawable_cull_pad_world(&self) -> f64 {
            16.0 / self.camera.zoom.max(1e-9)
        }

        fn handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
            let pos = self.handle_world_pos(h)?;
            let pad = self.drawable_cull_pad_world() + self.effective_handle_radius(h).max(1.0);
            Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
        }

        fn indirect_handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
            let pos = self.indirect_handle_world_pos(h)?;
            let pad = self.drawable_cull_pad_world() + self.indirect_handle_marker_radius_world(h).max(1.0);
            Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
        }

        fn edge_world_bounds_for_cull(&self, e: &EdgeData) -> Option<WorldBox> {
            let c = self.edge_curve(e)?;
            let axis = cubic_bezier_axis_bounds(c);
            let half_w_world = self.camera.zoom.max(0.75) / self.camera.zoom.max(1e-9);
            Some(inflate_world_box(axis, half_w_world + self.drawable_cull_pad_world()))
        }

        fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
            let step = world_step * self.camera.zoom;
            if step < min_step_screen {
                return;
            }
            let stroke = Stroke::new(stroke_px);
            let w = self.width as f64;
            let h = self.height as f64;
            let origin = self.world_to_screen(Point::new(0.0, 0.0));
            let x_off = ((origin.x % step) + step) % step;
            let y_off = ((origin.y % step) + step) % step;
            let mut p = infinite::canvas::BezPath::new();
            let mut x = x_off;
            while x <= w {
                p.move_to(Point::new(x, 0.0));
                p.line_to(Point::new(x, h));
                x += step;
            }
            let mut y = y_off;
            while y <= h {
                p.move_to(Point::new(0.0, y));
                p.line_to(Point::new(w, y));
                y += step;
            }
            scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
        }

        fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
            if world_space {
                world
            } else {
                self.world_to_screen(world)
            }
        }

        fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
            if world_space {
                len_world.max(1e-9)
            } else {
                (len_world * self.camera.zoom).max(1.0)
            }
        }

        fn append_handle_marker(
            &self,
            scene: &mut Scene,
            h: &HandleData,
            center: Point,
            radius_world: f64,
            draw_icon: bool,
            style_kind: BoardElementStyleKind,
            paint_override: Option<(Color, Color, f64)>,
            world_space: bool,
            layer: NodeHandlePaintLayer,
            exterior_cap: bool,
        ) {
            let c = self.draw_space_point(center, world_space);
            let r = self.draw_space_len(radius_world, world_space);
            let (fill, stroke_c, stroke_px) =
                if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (self.resolve_handle_fill_color(h, &self.canvas_theme, style_kind), self.resolve_handle_stroke_color(h, &self.canvas_theme, style_kind), 2.0_f64) };
            let paint_fill = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill);
            let paint_stroke = matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
            let paint_icons = draw_icon && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
            let outward = if exterior_cap { self.nodes.get(h.node_id.as_str()).and_then(|n| handle_outward_at_node_rim(center, Point::new(n.x, n.y), n.shape, n.radius, n.width, n.height)) } else { None };
            if paint_fill {
                if let Some(out) = outward {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &handle_exterior_cap_fill_path(c, out, r));
                } else {
                    scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &Circle::new(c, r));
                }
            }
            if paint_stroke {
                if let Some(out) = outward {
                    scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &handle_exterior_cap_stroke_path(c, out, r));
                } else {
                    scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &Circle::new(c, r));
                }
            }
            if paint_icons {
                if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                    let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
                    if let Some(paint) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                        let (bx, by, bw, bh) = paint.bounds();
                        let fit_inset = 0.62;
                        let s = self.draw_space_len(radius_world, world_space) * fit_inset;
                        let cx = bx + bw * 0.5;
                        let cy = by + bh * 0.5;
                        let avail = 2.0 * s;
                        let scale = (avail / bw).min(avail / bh);
                        let aff = Affine::IDENTITY.translate((c.x - scale * cx, c.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                        let r_clip = self.draw_space_len(radius_world, world_space) * 0.82;
                        let disc = Circle::new(c, r_clip);
                        scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                        match paint.body() {
                            CachedIconBody::Vector(icon_scene) => {
                                scene.append(icon_scene, Some(aff));
                            }
                            CachedIconBody::Raster(img) => {
                                scene.draw_image(img, aff);
                            }
                        }
                        scene.pop_layer();
                    }
                }
            }
        }

        fn append_indirect_handle_ring(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
            for h in self.handles.values() {
                if h.node_id != node_id || !self.handle_effectively_visible(h.id.as_str()) {
                    continue;
                }
                if !self.indirect_ring_handle_eligible(h.id.as_str(), node_id) {
                    continue;
                }
                if let Some(tb) = tile_filter {
                    let Some(hb) = self.indirect_handle_world_bounds_cull(h) else { continue };
                    if !world_boxes_overlap(*tb, hb) {
                        continue;
                    }
                }
                let Some(wp) = self.indirect_handle_world_pos(h) else { continue };
                let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
                let stroke_px = 2.0_f64;
                let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((self.canvas_theme.indirect_handle_fill, self.canvas_theme.indirect_handle_stroke, stroke_px)) } else { None };
                self.append_handle_marker(scene, h, wp, self.indirect_handle_marker_radius_world(h), false, style_kind, paint_override, world_space, NodeHandlePaintLayer::Full, false);
            }
        }

        /// @emoji 📏️ Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
        fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
            match lod {
                BoardDrawLod::Minimap => ui_styling::strokes::EDGE_MINIMAP,
                BoardDrawLod::Overview | BoardDrawLod::Compact => (ui_styling::strokes::EDGE_OVERVIEW).max(ui_styling::strokes::EDGE_BASE * self.camera.zoom),
                _ => 2.0 * self.camera.zoom.max(0.75),
            }
        }

        /// @emoji 📏️ Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
        fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
            let screen_px = self.edge_screen_stroke_width_px(lod);
            let z = self.camera.zoom.max(1e-9);
            (screen_px / z).max(1e-3)
        }

        fn paint_node_geometry(&self, scene: &mut Scene, n: &NodeData, lod: BoardDrawLod, world_space: bool, layer: NodeHandlePaintLayer, chrome_pass: StyleChromePass, link_compat: bool) {
            let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
            let resolved_style_kind = self.resolve_node_style_kind(n, chrome_pass);
            let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
            let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
            let stroke_c = Self::node_stroke_for_style(&self.canvas_theme, style_kind);
            let fill = if lod == BoardDrawLod::Minimap { stroke_c } else { self.resolve_node_fill_color(n, &self.canvas_theme, style_kind) };
            let sw = 2.0_f64;
            let paint_fill = if lod == BoardDrawLod::Minimap {
                matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill)
            } else {
                matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Fill) && !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral)
            };
            let paint_stroke = draw_node_stroke && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Stroke);
            let paint_icons = draw_node_icons && matches!(layer, NodeHandlePaintLayer::Full | NodeHandlePaintLayer::Icons);
            match n.shape {
                NodeShape::Circle => {
                    let c = self.draw_space_point(Point::new(n.x, n.y), world_space);
                    let r = self.draw_space_len(self.scaled_node_radius(n), world_space);
                    let circle = Circle::new(c, r);
                    if paint_fill {
                        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &circle);
                    }
                    if paint_stroke {
                        scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
                    }
                    if paint_icons {
                        self.paint_node_icon(scene, n, world_space, style_kind, Some(&circle), None);
                    }
                }
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    let p0 = self.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
                    let p1 = self.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
                    let rect = Rect::from_points(p0, p1);
                    if paint_fill {
                        scene.fill(FillRule::NonZero, Affine::IDENTITY, fill, None, &rect);
                    }
                    if paint_stroke {
                        scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &rect);
                    }
                    if paint_icons {
                        self.paint_node_icon(scene, n, world_space, style_kind, None, Some(rect));
                    }
                }
            }
        }

        fn paint_node_icon(&self, scene: &mut Scene, n: &NodeData, world_space: bool, style_kind: BoardElementStyleKind, circle_clip: Option<&Circle>, rect_clip: Option<Rect>) {
            if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                let (icon_fg, icon_bg) = IconPaintCache::board_icon_paint_colors(&self.canvas_theme);
                if let Some(paint) = self.get_or_build_icon_paint(k, icon_fg, icon_bg, preserve_original_style) {
                    let (bx, by, bw, bh) = paint.bounds();
                    let clip_inset = ui_styling::metrics::icon::CLIP_INSET;
                    let fit_inset = ui_styling::metrics::icon::FIT_INSET;
                    let (sx_half, sy_half) = match n.shape {
                        NodeShape::Circle => {
                            let s = self.draw_space_len(self.scaled_node_radius(n), world_space) * fit_inset;
                            (s, s)
                        }
                        NodeShape::Rectangle => (self.draw_space_len(self.scaled_node_width(n), world_space) * fit_inset * 0.5, self.draw_space_len(self.scaled_node_height(n), world_space) * fit_inset * 0.5),
                    };
                    let center = self.draw_space_point(Point::new(n.x, n.y), world_space);
                    let cx = bx + bw * 0.5;
                    let cy = by + bh * 0.5;
                    let avail_w = 2.0 * sx_half;
                    let avail_h = 2.0 * sy_half;
                    let scale = (avail_w / bw).min(avail_h / bh);
                    let aff = Affine::IDENTITY.translate((center.x - scale * cx, center.y - scale * cy)) * Affine::IDENTITY.scale(scale);
                    match n.shape {
                        NodeShape::Circle => {
                            let r_clip = self.draw_space_len(self.scaled_node_radius(n), world_space) * clip_inset;
                            let disc = circle_clip.copied().unwrap_or_else(|| Circle::new(center, r_clip));
                            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &disc);
                            match paint.body() {
                                CachedIconBody::Vector(icon_scene) => {
                                    scene.append(icon_scene, Some(aff));
                                }
                                CachedIconBody::Raster(img) => {
                                    scene.draw_image(img, aff);
                                }
                            }
                            scene.pop_layer();
                        }
                        NodeShape::Rectangle => {
                            let hw = self.draw_space_len(self.scaled_node_width(n), world_space) * clip_inset * 0.5;
                            let hh = self.draw_space_len(self.scaled_node_height(n), world_space) * clip_inset * 0.5;
                            let clip_r = rect_clip.unwrap_or_else(|| Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh)));
                            scene.push_clip_layer(FillRule::NonZero, Affine::IDENTITY, &clip_r);
                            match paint.body() {
                                CachedIconBody::Vector(icon_scene) => {
                                    scene.append(icon_scene, Some(aff));
                                }
                                CachedIconBody::Raster(img) => {
                                    scene.draw_image(img, aff);
                                }
                            }
                            scene.pop_layer();
                        }
                    }
                }
            }
        }

        fn append_nodes_and_handles_with_overlay_chrome(
            &self,
            scene: &mut Scene,
            tile_filter: Option<&WorldBox>,
            lod: BoardDrawLod,
            world_space: bool,
            only_ids: Option<&BTreeSet<String>>,
            overlay_ids: &BTreeSet<String>,
            layer: NodeHandlePaintLayer,
        ) {
            let pad = self.drawable_cull_pad_world();
            let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
            let draw_handle_icons = lod == BoardDrawLod::Micro;
            let link_source = self.active_link_source_handle_id().map(str::to_string);
            let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
            for h in self.handles.values() {
                if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&h.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                    if !world_boxes_overlap(*tb, hb) {
                        continue;
                    }
                }
                let Some(wp) = self.handle_world_pos(h) else { continue };
                let style_kind = self.resolve_handle_style_kind(h, self.chrome_pass_for_entity(&h.id, overlay_ids));
                self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
            }
            let paint_node = |scene: &mut Scene, n: &NodeData, chrome_pass: StyleChromePass| {
                self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
            };
            for n in self.nodes.values() {
                if !n.visible {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&n.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                        continue;
                    }
                }
                if overlay_ids.contains(&n.id) {
                    continue;
                }
                paint_node(scene, n, StyleChromePass::CachedBase);
            }
            for n in self.nodes.values() {
                if !n.visible {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&n.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                        continue;
                    }
                }
                if !overlay_ids.contains(&n.id) {
                    continue;
                }
                paint_node(scene, n, StyleChromePass::InteractionOverlay);
            }
        }

        fn append_nodes_and_handles(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, chrome_pass: StyleChromePass, layer: NodeHandlePaintLayer) {
            let pad = self.drawable_cull_pad_world();
            let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
            let draw_handle_icons = lod == BoardDrawLod::Micro;
            let link_source = self.active_link_source_handle_id().map(str::to_string);
            let link_compat_nodes: BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
            for h in self.handles.values() {
                if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&h.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                    if !world_boxes_overlap(*tb, hb) {
                        continue;
                    }
                }
                let Some(wp) = self.handle_world_pos(h) else { continue };
                let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
                self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space, layer, true);
            }
            for n in self.nodes.values() {
                if !n.visible {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&n.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                        continue;
                    }
                }
                self.paint_node_geometry(scene, n, lod, world_space, layer, chrome_pass, link_compat_nodes.contains(&n.id));
            }
        }

        fn append_edges_wires_and_link(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool, only_ids: Option<&BTreeSet<String>>, overlay_ids: Option<&BTreeSet<String>>) {
            let edge_sw = if world_space { self.edge_world_stroke_width(lod) } else { self.edge_screen_stroke_width_px(lod) };
            for e in self.edges.values() {
                if !self.edge_effectively_visible(e) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&e.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    let Some(eb) = self.edge_world_bounds_for_cull(e) else { continue };
                    if !world_boxes_overlap(*tb, eb) {
                        continue;
                    }
                }
                if let Some(c) = self.edge_curve(e) {
                    let p0 = self.draw_space_point(c.p0(), world_space);
                    let p1 = self.draw_space_point(c.p1(), world_space);
                    let p2 = self.draw_space_point(c.p2(), world_space);
                    let p3 = self.draw_space_point(c.p3(), world_space);
                    let curve = CubicBez::new(p0, p1, p2, p3);
                    let chrome_pass = overlay_ids.map(|ids| self.chrome_pass_for_entity(&e.id, ids)).unwrap_or(StyleChromePass::CachedBase);
                    let (stroke_color, edge_stroke, stroke_w) = self.resolve_edge_stroke_paint(e, chrome_pass, lod, edge_sw);
                    scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
                    let (source_tip, target_tip) = self.resolve_edge_tips(e);
                    Self::append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
                }
            }
            let wire_sw = 2.25_f64;
            let wire_stroke = Stroke::new(wire_sw);
            for w in self.wires.values() {
                if !self.wire_effectively_visible(w) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&w.id) {
                        continue;
                    }
                }
                if let Some(c) = self.wire_curve(w) {
                    let p0 = self.draw_space_point(c.p0(), world_space);
                    let p1 = self.draw_space_point(c.p1(), world_space);
                    let p2 = self.draw_space_point(c.p2(), world_space);
                    let p3 = self.draw_space_point(c.p3(), world_space);
                    let curve = CubicBez::new(p0, p1, p2, p3);
                    let chrome_pass = overlay_ids.map(|ids| self.chrome_pass_for_entity(&w.id, ids)).unwrap_or(StyleChromePass::CachedBase);
                    let wc = Self::wire_stroke_for_style(&self.canvas_theme, self.resolve_wire_style_kind(w, chrome_pass));
                    scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
                }
            }
            let link_wire_sw = 2.85_f64;
            let link_wire_stroke = Stroke::new(link_wire_sw);
            let link_wire_color = self.canvas_theme.node_stroke;
            if let Some(c) = self.active_link_wire_curve() {
                let p0 = self.draw_space_point(c.p0(), world_space);
                let p1 = self.draw_space_point(c.p1(), world_space);
                let p2 = self.draw_space_point(c.p2(), world_space);
                let p3 = self.draw_space_point(c.p3(), world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
            }
        }

        fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
            let generation = self.content_scene_generation;
            let cam_aff = self.camera_content_affine();
            let overlay_ids = self.interaction_overlay_entity_ids();
            let mut fill_layer = Scene::new();
            self.append_nodes_and_handles_with_overlay_chrome(&mut fill_layer, None, lod, true, None, &overlay_ids, NodeHandlePaintLayer::Fill);
            scene.append(&fill_layer, Some(cam_aff));
            let mut cache = self.world_content_cache.borrow_mut();
            let needs_rebuild = cache.as_ref().map(|c| c.0 != generation || c.1 != lod).unwrap_or(true);
            if needs_rebuild {
                if cache.is_some() {
                    let Some(token) = crate::infinite::canvas::reserve_opaque_scene_retirement() else {
                        self.opaque_scene_fault.set(true);
                        return;
                    };
                    let (_, _, stale) = cache.take().expect("stale world content cache was witnessed occupied");
                    crate::infinite::canvas::publish_opaque_scene_retirement(token, stale);
                }
                let mut content = Scene::new();
                self.append_nodes_and_handles(&mut content, None, lod, true, None, StyleChromePass::CachedBase, NodeHandlePaintLayer::Icons);
                *cache = Some((generation, lod, content));
            }
            if let Some(cached) = cache.as_ref() {
                scene.append(&cached.2, Some(cam_aff));
            }
            let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
            if edges_in_world_space {
                let mut edge_layer = Scene::new();
                self.append_edges_wires_and_link(&mut edge_layer, None, lod, true, None, Some(&overlay_ids));
                scene.append(&edge_layer, Some(cam_aff));
            } else {
                self.append_edges_wires_and_link(scene, None, lod, false, None, Some(&overlay_ids));
            }
            let mut stroke_layer = Scene::new();
            self.append_nodes_and_handles_with_overlay_chrome(&mut stroke_layer, None, lod, false, None, &overlay_ids, NodeHandlePaintLayer::Stroke);
            scene.append(&stroke_layer, None);
            if let Some(c) = self.active_link_wire_curve() {
                let link_wire_stroke = Stroke::new(ui_styling::strokes::WIRE_HIGHLIGHT);
                let link_wire_color = self.canvas_theme.node_stroke;
                let p0 = self.draw_space_point(c.p0(), false);
                let p1 = self.draw_space_point(c.p1(), false);
                let p2 = self.draw_space_point(c.p2(), false);
                let p3 = self.draw_space_point(c.p3(), false);
                let curve = CubicBez::new(p0, p1, p2, p3);
                scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
            }
            if self.has_ports() {
                if let Some(node_id) = self.indirect_ring_node_id(lod) {
                    self.append_indirect_handle_ring(scene, None, &node_id, StyleChromePass::CachedBase, false);
                }
            }
            let previews_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
            if previews_in_world_space {
                let mut preview_layer = Scene::new();
                if self.fixture_drop_preview.is_some() {
                    self.append_fixture_drop_preview_paint(&mut preview_layer, lod, true);
                }
                if self.active_utility == ActiveUtility::Brush || self.brush_preview.is_some() {
                    self.append_brush_preview_paint(&mut preview_layer, lod, true);
                }
                scene.append(&preview_layer, Some(cam_aff));
            } else {
                if self.fixture_drop_preview.is_some() {
                    self.append_fixture_drop_preview_paint(scene, lod, false);
                }
                if self.active_utility == ActiveUtility::Brush || self.brush_preview.is_some() {
                    self.append_brush_preview_paint(scene, lod, false);
                }
            }
        }

        pub fn set_wheel_zoom_active(&mut self, active: bool) {
            if active && !self.wheel_zoom_active {
                self.wheel_zoom_render_lod = Some(self.current_draw_lod());
            }
            if !active {
                self.wheel_zoom_render_lod = None;
            }
            self.wheel_zoom_active = active;
        }

        pub fn build_vector_scene(&self) -> Scene {
            let mut inner = Scene::new();
            let lod = self.draw_lod_for_frame();
            if !self.wheel_zoom_active {
                let grid_color = self.canvas_theme.grid_minor_stroke;
                if lod != BoardDrawLod::Minimap {
                    self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_LARGE, self.grid_step_large_world(), 0.0);
                    match lod {
                        BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                            self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MEDIUM, self.grid_step_medium_world(), 0.0);
                        }
                        BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                    }
                    if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                        self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_SMALL, self.grid_step_small_world(), 0.0);
                    }
                    if lod == BoardDrawLod::Micro {
                        self.stroke_world_step_grid(&mut inner, grid_color, ui_styling::strokes::GRID_MICRO, self.grid_step_micro_world(), 0.0);
                    }
                }
            }
            if let Some(ref pts) = self.selection_screen_preview {
                if pts.len() >= 2 {
                    let mut path = infinite::canvas::BezPath::new();
                    path.move_to(pts[0]);
                    for p in pts.iter().skip(1) {
                        path.line_to(*p);
                    }
                    path.close_path();
                    inner.fill(FillRule::NonZero, Affine::IDENTITY, self.canvas_theme.selection_preview_fill, None, &path);
                    let mut preview_stroke = Stroke::new(ui_styling::strokes::SELECTION_PREVIEW);
                    if self.selection_preview_crossing {
                        preview_stroke.set_dash_pattern(vec![5.0, 4.0]);
                    }
                    inner.stroke(&preview_stroke, Affine::IDENTITY, self.canvas_theme.selection_preview_stroke, None, &path);
                }
            }
            self.append_cached_world_content(&mut inner, lod);
            let scale = self.dpr.max(1.0);
            if (scale - 1.0).abs() < f64::EPSILON {
                inner
            } else {
                let mut scene = Scene::new();
                scene.append(&inner, Some(Affine::IDENTITY.scale(scale)));
                scene
            }
        }

        pub fn encoded_scene_hint(&self) -> usize {
            let s = self.build_vector_scene();
            s.path_count()
        }

        pub fn update_hover_from_world(&mut self, world: Point) {
            let next = self.resolve_hover_world(world);
            self.set_hovered_id(next);
        }

        pub fn set_hovered_id(&mut self, id: Option<String>) {
            let event_kind = id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            if self.hovered_id == id && self.hovered_kind.is_none() {
                return;
            }
            let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::hover(id.as_deref(), event_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
                return;
            };
            self.hovered_id = id.clone();
            self.hovered_kind = None;
            self.publish_event_reservation(reservation);
        }

        /// @emoji 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
        pub fn set_hovered_kind(&mut self, domain: Option<String>, kind_id: Option<String>) {
            let next_kind = domain.zip(kind_id);
            if self.hovered_id.is_none() && self.hovered_kind == next_kind {
                return;
            }
            let Some(reservation) = self.reserve_owned_event(BoardOwnedEvent::hover(None, next_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str())))) else {
                return;
            };
            self.hovered_id = None;
            self.hovered_kind = next_kind.clone();
            self.publish_event_reservation(reservation);
        }

        /// @emoji 🔇️ Updates hover chrome without emitting `hover` (controlled React sync).
        pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
            if self.hovered_id == id && self.hovered_kind.is_none() {
                return;
            }
            self.hovered_id = id;
            self.hovered_kind = None;
        }

        /// @emoji 💠️ Externally driven highlight set (cross-panel binding); does not emit events.
        pub fn set_highlighted_ids(&mut self, ids: Vec<String>) {
            let next: BTreeSet<String> = ids.into_iter().collect();
            if self.highlighted_ids == next {
                return;
            }
            self.highlighted_ids = next;
        }

        /// @emoji 💠️ Current externally driven highlight ids as JSON array.
        pub fn highlighted_ids_json(&self) -> Result<String, NormalPortError> {
            Ok(serde_json::to_string(&self.highlighted_ids.iter().cloned().collect::<Vec<_>>())?)
        }

        /// @emoji 🔇️ Mirrors controlled kind hover without emitting `hover`.
        pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
            let next_kind = domain.zip(kind_id);
            if self.hovered_id.is_none() && self.hovered_kind == next_kind {
                return;
            }
            self.hovered_id = None;
            self.hovered_kind = next_kind;
        }

        pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
            let plan = self.plan_wheel(sx, sy, delta_y);
            let _ = self.commit_wheel(plan);
        }

        pub fn plan_wheel(&self, sx: f64, sy: f64, delta_y: f64) -> BoardWheelPlan {
            let mut next = self.camera.clone();
            infinite::canvas::camera::wheel_screen(&mut next, &self.viewport(), sx, sy, delta_y);
            BoardWheelPlan { revision: self.interaction_revision, expected: self.camera.clone(), next }
        }

        pub fn commit_wheel(&mut self, plan: BoardWheelPlan) -> bool {
            if self.interaction_revision != plan.revision || [self.camera.x.to_bits(), self.camera.y.to_bits(), self.camera.zoom.to_bits()] != [plan.expected.x.to_bits(), plan.expected.y.to_bits(), plan.expected.zoom.to_bits()] {
                return false;
            }
            self.set_camera_silent(plan.next.x, plan.next.y, plan.next.zoom);
            self.interaction_revision = plan.revision.wrapping_add(1);
            true
        }

        fn begin_delete_property_audit(&mut self, planning: &mut BoardDeletePlanningOperation, kind: BoardDeleteKind, id: &str) {
            if planning.plan.contains(kind, id) {
                return;
            }
            if id.len() > BOARD_POINTER_BYTE_CAPACITY.saturating_sub(usize::from(planning.plan.byte_len)) {
                planning.fault = Some(BoardEventFault::ByteCredits);
                return;
            }
            let properties = match kind {
                BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
                BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
                BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
                BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| std::mem::take(&mut entity.properties)),
            };
            let Some(properties) = properties else {
                planning.fault = Some(BoardEventFault::Schema);
                return;
            };
            planning.property_audit = Some(BoardPropertyAudit::new(kind, id.to_owned(), properties));
        }

        fn restore_delete_properties(&mut self, kind: BoardDeleteKind, id: &str, properties: graph::PropertyBag) -> bool {
            match kind {
                BoardDeleteKind::Edge => self.edges.get_mut(id).map(|entity| entity.properties = properties),
                BoardDeleteKind::Node => self.nodes.get_mut(id).map(|entity| entity.properties = properties),
                BoardDeleteKind::Handle => self.handles.get_mut(id).map(|entity| entity.properties = properties),
                BoardDeleteKind::Wire => self.wires.get_mut(id).map(|entity| entity.properties = properties),
            }
            .is_some()
        }

        fn step_delete_planning(&mut self) -> BoardAuthorityStep {
            let Some(mut planning) = self.pending_delete_planning.take() else {
                return BoardAuthorityStep::Complete;
            };
            if let Some(audit) = planning.property_audit.as_mut() {
                if !audit.step() {
                    self.pending_delete_planning = Some(planning);
                    return BoardAuthorityStep::Pending;
                }
                let Some((kind, id, properties, nodes, bytes, fault)) = audit.take_result() else {
                    planning.fault = Some(BoardEventFault::Schema);
                    self.pending_delete_planning = Some(planning);
                    return BoardAuthorityStep::Pending;
                };
                planning.property_audit = None;
                if !self.restore_delete_properties(kind, &id, properties) {
                    planning.fault = Some(BoardEventFault::Schema);
                } else if let Some(fault) = fault {
                    planning.fault = Some(fault);
                } else if !planning.cancelling {
                    if let Err(fault) = planning.plan.push_admitted_entity(kind, &id, nodes, bytes) {
                        planning.fault = Some(fault);
                    }
                }
                self.pending_delete_planning = Some(planning);
                return BoardAuthorityStep::Pending;
            }
            if planning.cancelling {
                return BoardAuthorityStep::Cancelled;
            }
            if planning.fault.is_some() || self.interaction_revision != planning.plan.revision {
                self.event_schema_fault = true;
                return BoardAuthorityStep::Fault;
            }
            match planning.phase {
                BoardDeletePlanningPhase::SelectedEdges => {
                    let next = match planning.scan_after.as_ref() {
                        Some(after) => self.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.selection.first(),
                    };
                    let next = next.map(|id| admitted_board_pointer_id(id).map(|owned| (self.edges.contains_key(id), owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((is_edge, id)) => {
                                planning.scan_after = Some(id.clone());
                                if is_edge {
                                    self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                                }
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else {
                        planning.scan_after = None;
                        planning.phase = BoardDeletePlanningPhase::Nodes;
                    }
                }
                BoardDeletePlanningPhase::Wires => {
                    let handle_id = planning.handle_id.as_deref().expect("wire planning owns a handle");
                    let next = match planning.relation_after.as_ref() {
                        Some(after) => self.wires.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.wires.first_key_value(),
                    }
                    .map(|(id, wire)| admitted_board_pointer_id(id).map(|owned| (wire.source == handle_id || wire.target.as_deref() == Some(handle_id), owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((remove, id)) => {
                                planning.relation_after = Some(id.clone());
                                if remove {
                                    self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Wire, &id);
                                }
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else {
                        planning.relation_after = None;
                        planning.phase = BoardDeletePlanningPhase::Edges;
                    }
                }
                BoardDeletePlanningPhase::Handles => {
                    let node_id = planning.node_id.as_deref().expect("handle planning owns a node");
                    let next = match planning.handle_after.as_ref() {
                        Some(after) => self.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.handles.first_key_value(),
                    }
                    .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id, owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((belongs, id)) => {
                                planning.handle_after = Some(id.clone());
                                if belongs {
                                    planning.handle_id = Some(id);
                                    planning.relation_after = None;
                                    planning.phase = BoardDeletePlanningPhase::Wires;
                                }
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else {
                        planning.handle_after = None;
                        planning.phase = BoardDeletePlanningPhase::Node;
                    }
                }
                BoardDeletePlanningPhase::Edges => {
                    let endpoint_id = planning.handle_id.as_deref().or(planning.node_id.as_deref()).expect("edge planning owns an endpoint");
                    let next = match planning.relation_after.as_ref() {
                        Some(after) => self.edges.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.edges.first_key_value(),
                    }
                    .map(|(id, edge)| admitted_board_pointer_id(id).map(|owned| (edge.source == endpoint_id || edge.target == endpoint_id, owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((remove, id)) => {
                                planning.relation_after = Some(id.clone());
                                if remove {
                                    self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Edge, &id);
                                }
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else {
                        planning.relation_after = None;
                        planning.phase = if planning.handle_id.is_some() { BoardDeletePlanningPhase::Handle } else { BoardDeletePlanningPhase::Node };
                    }
                }
                BoardDeletePlanningPhase::Nodes => {
                    let next = match planning.node_after.as_ref() {
                        Some(after) => self.nodes.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.nodes.first_key_value(),
                    }
                    .map(|(id, _)| admitted_board_pointer_id(id).map(|owned| (self.selection.contains(id), owned)));
                    if let Some(next) = next {
                        match next {
                            Ok((selected, id)) => {
                                planning.node_after = Some(id.clone());
                                planning.node_id = Some(id);
                                planning.node_relevant = selected;
                                planning.handle_after = None;
                                planning.phase = BoardDeletePlanningPhase::DiscoverNode;
                            }
                            Err(fault) => planning.fault = Some(fault),
                        }
                    } else {
                        planning.node_after = None;
                        planning.phase = BoardDeletePlanningPhase::SelectionEvent;
                    }
                }
                BoardDeletePlanningPhase::DiscoverNode => {
                    if !self.has_ports() {
                        if planning.node_relevant {
                            planning.relation_after = None;
                            planning.phase = BoardDeletePlanningPhase::Edges;
                        } else {
                            planning.node_id = None;
                            planning.phase = BoardDeletePlanningPhase::Nodes;
                        }
                    } else {
                        let node_id = planning.node_id.as_deref().expect("node discovery owns a node");
                        let next = match planning.handle_after.as_ref() {
                            Some(after) => self.handles.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                            None => self.handles.first_key_value(),
                        }
                        .map(|(id, handle)| admitted_board_pointer_id(id).map(|owned| (handle.node_id == node_id && self.selection.contains(id), owned)));
                        if let Some(next) = next {
                            match next {
                                Ok((selected, id)) => {
                                    planning.handle_after = Some(id);
                                    if selected {
                                        planning.node_relevant = true;
                                        planning.handle_after = None;
                                        planning.phase = BoardDeletePlanningPhase::Handles;
                                    }
                                }
                                Err(fault) => planning.fault = Some(fault),
                            }
                        } else if planning.node_relevant {
                            planning.handle_after = None;
                            planning.phase = BoardDeletePlanningPhase::Handles;
                        } else {
                            planning.handle_after = None;
                            planning.node_id = None;
                            planning.phase = BoardDeletePlanningPhase::Nodes;
                        }
                    }
                }
                BoardDeletePlanningPhase::Handle => {
                    let id = planning.handle_id.take().expect("handle deletion owns an id");
                    planning.phase = BoardDeletePlanningPhase::Handles;
                    self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Handle, &id);
                }
                BoardDeletePlanningPhase::Node => {
                    let id = planning.node_id.take().expect("node deletion owns an id");
                    planning.phase = BoardDeletePlanningPhase::Nodes;
                    self.begin_delete_property_audit(&mut planning, BoardDeleteKind::Node, &id);
                }
                BoardDeletePlanningPhase::SelectionEvent => {
                    let next = match planning.scan_after.as_ref() {
                        Some(after) => self.selection.range((std::ops::Bound::Excluded(after.clone()), std::ops::Bound::Unbounded)).next(),
                        None => self.selection.first(),
                    }
                    .map(|id| admitted_board_pointer_id(id));
                    match next {
                        Some(Ok(id)) => {
                            planning.scan_after = Some(id.clone());
                            if !planning.plan.removes_selection_id(&id) {
                                let write = (|| {
                                    if !planning.select_first {
                                        planning.select_builder.raw(",")?;
                                    }
                                    planning.select_builder.string(&id)?;
                                    Ok::<(), BoardEventFault>(())
                                })();
                                if let Err(fault) = write {
                                    planning.fault = Some(fault);
                                }
                                planning.select_first = false;
                            }
                        }
                        Some(Err(fault)) => planning.fault = Some(fault),
                        None => planning.phase = BoardDeletePlanningPhase::Finish,
                    }
                }
                BoardDeletePlanningPhase::Finish => {
                    if let Err(fault) = planning.select_builder.raw("],\"exitHighlightIds\":[]}") {
                        planning.fault = Some(fault);
                    }
                    if planning.fault.is_some() {
                        self.event_schema_fault = true;
                        return BoardAuthorityStep::Fault;
                    }
                    let select_event = std::mem::replace(&mut planning.select_builder, BoardPayloadBuilder::new()).finish_owned(BoardEventKind::Select);
                    self.pending_delete_operation = Some(BoardDeleteOperation {
                        plan: planning.plan,
                        select_event: Some(select_event),
                        mutation_cursor: 0,
                        publication_cursor: 0,
                        remaining_claimed_items: 0,
                        remaining_claimed_bytes: 0,
                        claimed: false,
                        retiring_entity: None,
                        cancelling: false,
                    });
                    return BoardAuthorityStep::Pending;
                }
            }
            self.pending_delete_planning = Some(planning);
            BoardAuthorityStep::Pending
        }

        #[cfg(test)]
        fn plan_delete_selection(&self) -> Result<BoardDeletePlan, BoardEventFault> {
            let mut plan = BoardDeletePlan::new(self.interaction_revision);
            for id in self.selection.iter().filter(|id| self.edges.contains_key(*id)) {
                let edge = self.edges.get(id).expect("selected edge exists");
                plan.push_entity(BoardDeleteKind::Edge, id, &edge.properties)?;
            }
            if self.has_ports() {
                for (node_id, node) in self.nodes.iter().filter(|(node_id, _)| self.selection.contains(*node_id) || self.handles.values().any(|handle| handle.node_id == **node_id && self.selection.contains(&handle.id))) {
                    for (handle_id, handle) in self.handles.iter().filter(|(_, handle)| handle.node_id == *node_id) {
                        for (wire_id, wire) in self.wires.iter().filter(|(_, wire)| wire.source == *handle_id || wire.target.as_ref() == Some(handle_id)) {
                            plan.push_entity(BoardDeleteKind::Wire, wire_id, &wire.properties)?;
                        }
                        for (edge_id, edge) in self.edges.iter().filter(|(_, edge)| edge.source == *handle_id || edge.target == *handle_id) {
                            plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                        }
                        plan.push_entity(BoardDeleteKind::Handle, handle_id, &handle.properties)?;
                    }
                    plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
                }
            } else {
                for (node_id, node) in self.nodes.iter().filter(|(node_id, _)| self.selection.contains(*node_id)) {
                    for (edge_id, edge) in self.edges.iter().filter(|(_, edge)| edge.source == *node_id || edge.target == *node_id) {
                        plan.push_entity(BoardDeleteKind::Edge, edge_id, &edge.properties)?;
                    }
                    plan.push_entity(BoardDeleteKind::Node, node_id, &node.properties)?;
                }
            }
            Ok(plan)
        }

        fn step_pending_delete_operation(&mut self) {
            let Some(mut operation) = self.pending_delete_operation.take() else {
                return;
            };
            if let Some(retiring) = operation.retiring_entity.as_mut() {
                match retiring.step() {
                    Ok(true) => {
                        debug_assert!(retiring.terminal_is_empty());
                        operation.retiring_entity = None;
                    }
                    Ok(false) => {}
                    Err(()) => self.event_schema_fault = true,
                }
                self.pending_delete_operation = Some(operation);
                return;
            }
            if !operation.claimed {
                if self.interaction_revision != operation.plan.revision {
                    self.event_schema_fault = true;
                    return;
                }
                let event_count = operation.plan.emitted_event_count() + 1;
                let Some(event_bytes) = operation.plan.emitted_event_bytes().and_then(|bytes| bytes.checked_add(operation.select_event.as_ref()?.owned_bytes())) else {
                    self.event_schema_fault = true;
                    return;
                };
                if self.events.claim(event_count, event_bytes).is_err() {
                    self.pending_delete_operation = Some(operation);
                    return;
                }
                operation.claimed = true;
                operation.remaining_claimed_items = event_count as u16;
                operation.remaining_claimed_bytes = event_bytes;
                self.pending_delete_operation = Some(operation);
                return;
            }
            if operation.mutation_cursor < operation.plan.len {
                let entry = operation.plan.entries[usize::from(operation.mutation_cursor)].expect("delete plan entry");
                let id = operation.plan.id(entry.id);
                let retiring = match entry.kind {
                    BoardDeleteKind::Edge => {
                        let removed = self.edges.remove(id).map(BoardRemovedEntity::Edge);
                        self.selection.remove(id);
                        removed
                    }
                    BoardDeleteKind::Node => {
                        let removed = self.nodes.remove(id).map(BoardRemovedEntity::Node);
                        self.selection.remove(id);
                        removed
                    }
                    BoardDeleteKind::Handle => {
                        let removed = self.handles.remove(id).map(BoardRemovedEntity::Handle);
                        self.selection.remove(id);
                        removed
                    }
                    BoardDeleteKind::Wire => {
                        let removed = self.wires.remove(id).map(BoardRemovedEntity::Wire);
                        self.selection.remove(id);
                        removed
                    }
                };
                operation.mutation_cursor += 1;
                operation.retiring_entity = retiring.map(BoardEntityRetirement::new);
                self.pending_delete_operation = Some(operation);
                return;
            }
            if operation.publication_cursor < operation.plan.len {
                let entry = operation.plan.entries[usize::from(operation.publication_cursor)].expect("delete plan entry");
                operation.publication_cursor += 1;
                let id = operation.plan.id(entry.id);
                let event = match entry.kind {
                    BoardDeleteKind::Edge => Some(BoardOwnedEvent::id(BoardEventKind::EdgeDelete, id).expect("delete edge payload was exactly preflighted")),
                    BoardDeleteKind::Node => Some(BoardOwnedEvent::id(BoardEventKind::NodeDelete, id).expect("delete node payload was exactly preflighted")),
                    BoardDeleteKind::Handle | BoardDeleteKind::Wire => None,
                };
                if let Some(event) = event {
                    operation.remaining_claimed_items -= 1;
                    operation.remaining_claimed_bytes -= event.owned_bytes();
                    self.events.push_claimed(event).expect("delete event owns claimed queue credits");
                }
                self.pending_delete_operation = Some(operation);
                return;
            }
            if let Some(event) = operation.select_event.take() {
                operation.remaining_claimed_items -= 1;
                operation.remaining_claimed_bytes -= event.owned_bytes();
                self.events.push_claimed(event).expect("delete selection owns claimed queue credits");
                self.pending_delete_operation = Some(operation);
                return;
            }
            if let Some(id) = self.selection_exit_highlight.pop_first() {
                drop(id);
                self.pending_delete_operation = Some(operation);
                return;
            }
            if operation.remaining_claimed_items != 0 || operation.remaining_claimed_bytes != 0 {
                let _ = self.events.release_claim(usize::from(operation.remaining_claimed_items), operation.remaining_claimed_bytes);
                self.event_schema_fault = true;
            }
            self.bump_content_scene_generation();
            self.interaction_revision = operation.plan.revision.wrapping_add(1);
        }

        pub fn delete_selection(&mut self) {
            if self.pending_delete_planning.is_some() || self.pending_delete_operation.is_some() {
                return;
            }
            let mut select_builder = BoardPayloadBuilder::new();
            if select_builder.raw("{\"ids\":[").is_err() {
                self.event_schema_fault = true;
                return;
            }
            self.pending_delete_planning = Some(BoardDeletePlanningOperation {
                plan: BoardDeletePlan::new(self.interaction_revision),
                phase: BoardDeletePlanningPhase::SelectedEdges,
                scan_after: None,
                node_after: None,
                handle_after: None,
                relation_after: None,
                node_id: None,
                handle_id: None,
                node_relevant: false,
                property_audit: None,
                select_builder,
                select_first: true,
                fault: None,
                cancelling: false,
            });
        }

        fn link_snap_drag_tolerance_screen(&self, h: &HandleData) -> f64 {
            let z = self.camera.zoom.max(1e-9);
            HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + self.effective_handle_radius(h) * z
        }

        fn link_snap_commit_proximity_ok(&self, target_handle_id: &str, world: Point) -> bool {
            let Some(h) = self.handles.get(target_handle_id) else {
                return false;
            };
            if !self.handle_selectable(target_handle_id) {
                return false;
            }
            let Some(pw) = self.handle_world_pos(h) else {
                return false;
            };
            let z = self.camera.zoom.max(1e-9);
            let d_screen = distance_between(self.world_to_screen(world), self.world_to_screen(pw));
            let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + self.effective_handle_radius(h) * z;
            d_screen <= tol_commit
        }

        /// @emoji 🔗️ True when any edge uses this handle as `source` or `target` (handle already participates in a link).
        fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
            self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
        }

        fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
            self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
        }

        fn lod_allows_node_proximity_connect(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
        }

        /// @emoji 🧲️ While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
        fn node_drag_proximity_handle_pair(&self, moving_node_id: &str) -> Option<(String, String)> {
            if !self.lod_allows_node_proximity_connect() {
                return None;
            }
            if !self.node_selectable(moving_node_id) {
                return None;
            }
            if self.node_has_any_incident_edge(moving_node_id) {
                return None;
            }
            let moving = self.nodes.get(moving_node_id)?;
            let moving_bounds = self.node_world_bounds(moving, 0.0);
            let mut best: Option<(f64, String, String)> = None;
            for (target_id, target) in &self.nodes {
                if target_id == moving_node_id || !self.node_selectable(target_id.as_str()) {
                    continue;
                }
                let target_bounds = self.node_world_bounds(target, 0.0);
                if !world_boxes_overlap(moving_bounds, target_bounds) {
                    continue;
                }
                let moving_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
                let target_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && self.handle_selectable(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
                for (src_id, src_h) in &moving_handles {
                    let Some(src_pos) = self.handle_world_pos(src_h) else {
                        continue;
                    };
                    for (tgt_id, tgt_h) in &target_handles {
                        let Some(tgt_pos) = self.handle_world_pos(tgt_h) else {
                            continue;
                        };
                        let d = distance_between(src_pos, tgt_pos);
                        let pair = if self.handles_link_compatible_for_drag(src_h, tgt_h) {
                            Some(((*src_id).clone(), (*tgt_id).clone()))
                        } else if self.handles_link_compatible_for_drag(tgt_h, src_h) {
                            Some(((*tgt_id).clone(), (*src_id).clone()))
                        } else {
                            None
                        };
                        if let Some((s, t)) = pair {
                            if best.as_ref().map(|(bd, _, _)| d < *bd).unwrap_or(true) {
                                best = Some((d, s.to_string(), t.to_string()));
                            }
                        }
                    }
                }
            }
            best.map(|(_, s, t)| (s, t))
        }

        fn node_effectively_visible(&self, node_id: &str) -> bool {
            self.nodes.get(node_id).is_some_and(|n| n.visible)
        }

        fn node_selectable(&self, node_id: &str) -> bool {
            self.nodes.get(node_id).is_some_and(|n| n.visible && !n.locked)
        }

        fn handle_selectable(&self, handle_id: &str) -> bool {
            self.handles.get(handle_id).is_some_and(|h| h.visible && !h.locked && self.node_selectable(h.node_id.as_str()))
        }

        fn edge_selectable(&self, edge: &EdgeData) -> bool {
            if !edge.visible || edge.locked {
                return false;
            }
            if !self.has_ports() {
                return self.node_selectable(edge.source.as_str()) && self.node_selectable(edge.target.as_str());
            }
            self.handle_selectable(edge.source.as_str()) && self.handle_selectable(edge.target.as_str())
        }

        fn wire_selectable(&self, wire: &WireData) -> bool {
            if !wire.visible || wire.locked {
                return false;
            }
            if !self.handle_selectable(wire.source.as_str()) {
                return false;
            }
            wire.target.as_ref().map(|id| self.handle_selectable(id.as_str())).unwrap_or(true)
        }

        fn handle_effectively_visible(&self, handle_id: &str) -> bool {
            self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
        }

        fn edge_effectively_visible(&self, edge: &EdgeData) -> bool {
            if !self.has_ports() {
                return edge.visible && self.node_effectively_visible(edge.source.as_str()) && self.node_effectively_visible(edge.target.as_str());
            }
            edge.visible && self.handle_effectively_visible(edge.source.as_str()) && self.handle_effectively_visible(edge.target.as_str())
        }

        fn wire_effectively_visible(&self, wire: &WireData) -> bool {
            wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().map(|id| self.handle_effectively_visible(id.as_str())).unwrap_or(true)
        }

        /// @emoji 💫️ True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
        fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
            self.handle_selectable(handle_id) && !self.handle_has_incident_edge(handle_id)
        }

        /// @emoji 📍️ Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
        fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
            if matches!(self.current_draw_lod(), BoardDrawLod::Minimap) {
                return None;
            }
            let source_handle = self.handles.get(source_handle_id)?;
            if !self.handle_selectable(source_handle_id) {
                return None;
            }
            let source_node_id = source_handle.node_id.as_str();
            let p_scr = self.world_to_screen(world);
            let mut best: Option<(f64, String)> = None;
            for (id, h) in &self.handles {
                if id == source_handle_id || !self.handle_selectable(id.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(id.as_str()) {
                    continue;
                }
                if h.node_id == source_node_id {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source_handle, h) {
                    continue;
                }
                let pw = self.handle_world_pos(h)?;
                let h_scr = self.world_to_screen(pw);
                let d_screen = distance_between(p_scr, h_scr);
                let tol_screen = self.link_snap_drag_tolerance_screen(h);
                if d_screen <= tol_screen && best.as_ref().map(|(bd, _)| d_screen < *bd).unwrap_or(true) {
                    best = Some((d_screen, id.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str, also_emit: Option<BoardEventKind>) -> bool {
            if source_handle_id == target_handle_id {
                return false;
            }
            if !self.handle_selectable(source_handle_id) || !self.handle_selectable(target_handle_id) {
                return false;
            }
            let Some(source_row) = self.handles.get(source_handle_id) else {
                return false;
            };
            let Some(target_row) = self.handles.get(target_handle_id) else {
                return false;
            };
            if source_row.node_id == target_row.node_id {
                return false;
            }
            if !self.handles_link_compatible_for_drag(source_row, target_row) {
                return false;
            }
            if self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
                return false;
            }
            for e in self.edges.values() {
                if e.source == source_handle_id && e.target == target_handle_id {
                    return false;
                }
            }
            let mut n = self.edges.len().saturating_add(1);
            let id = loop {
                let candidate = format!("edge-link-{n}");
                if !self.edges.contains_key(&candidate) {
                    break candidate;
                }
                n = n.saturating_add(1);
            };
            let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
            let Some(reservation) = self.reserve_owned_batch(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, &id, source_handle_id, target_handle_id), also_emit.map(|kind| BoardOwnedEvent::edge(kind, &id, source_handle_id, target_handle_id)))
            else {
                return false;
            };
            self.edges.insert(
                id.clone(),
                EdgeData {
                    id: id.clone(),
                    source: source_handle_id.to_string(),
                    target: target_handle_id.to_string(),
                    selected: false,
                    visible: true,
                    locked: false,
                    style: None,
                    edge_kind,
                    source_tip: None,
                    target_tip: None,
                    properties: graph::PropertyBag::new(),
                },
            );
            self.publish_event_batch(reservation);
            true
        }

        fn build_selection_pointer_plan(&self, kind: BoardPointerPlanKind, next: &BTreeSet<String>, anchor: &BTreeSet<String>, gesture: Option<&str>, points: &[Point], screen_points: &[Point]) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            if next.len() > BOARD_POINTER_ITEM_CAPACITY || anchor.len() > BOARD_POINTER_ITEM_CAPACITY || points.len() != screen_points.len() || points.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            if anchor.iter().map(String::len).try_fold(0usize, usize::checked_add).is_none_or(|bytes| bytes > BOARD_POINTER_BYTE_CAPACITY) {
                return Err(BoardPointerPlanFault::ByteCredits);
            }
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, kind);
            for id in next {
                plan.push_delta(id, 0.0, 0.0)?;
            }
            for (world, screen) in points.iter().copied().zip(screen_points.iter().copied()) {
                plan.push_point(world, screen)?;
            }
            let event = match kind {
                BoardPointerPlanKind::SelectionPreview { .. } => {
                    let removed: BTreeSet<String> = anchor.difference(next).cloned().collect();
                    BoardOwnedEvent::preselect_sets(next, &removed, gesture)
                }
                BoardPointerPlanKind::SelectionCommit => BoardOwnedEvent::select_set(next, Some(anchor), gesture),
                _ => return Err(BoardPointerPlanFault::Unsupported),
            }
            .map_err(|fault| match fault {
                BoardEventFault::ItemCredits => BoardPointerPlanFault::ItemCredits,
                BoardEventFault::ByteCredits | BoardEventFault::KeyCredits | BoardEventFault::Schema => BoardPointerPlanFault::ByteCredits,
            })?;
            plan.seal_owned_event(&event)?;
            Ok(plan)
        }

        fn plan_selection_pending_pointer(&self, intent: BoardPointerIntent, initial_ids: &BTreeSet<String>, start: Point, start_screen: Point, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            match intent.phase {
                BoardPointerPhase::Move if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
                BoardPointerPhase::Move => {
                    let points = [start, world];
                    let screen_points = [start_screen, screen];
                    let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_selection_with_initial(initial_ids, start, &points, merge_mode.as_str());
                    let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
                    self.build_selection_pointer_plan(BoardPointerPlanKind::SelectionPreview { start, start_screen }, &next, initial_ids, gesture, &points, &screen_points)
                }
                BoardPointerPhase::Up => {
                    let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
                    let next = if intent.ctrl_or_meta || intent.shift { self.resolve_area_selection_with_initial(initial_ids, start, &[start], merge_mode.as_str()) } else { BTreeSet::new() };
                    let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
                    self.build_selection_pointer_plan(BoardPointerPlanKind::SelectionCommit, &next, initial_ids, gesture, &[], &[])
                }
                BoardPointerPhase::Leave => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
            }
        }

        fn plan_selection_pointer(
            &self,
            intent: BoardPointerIntent,
            initial_ids: &BTreeSet<String>,
            points: &[Point],
            screen_points: &[Point],
            start: Point,
            start_screen: Point,
            screen: Point,
            world: Point,
        ) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            if intent.phase == BoardPointerPhase::Leave {
                return Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle));
            }
            if points.len() != screen_points.len() || points.len() >= BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            let mut next_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
            let mut next_screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]> = Box::new([Point::ZERO; BOARD_POINTER_ITEM_CAPACITY]);
            next_points[..points.len()].copy_from_slice(points);
            next_screen_points[..screen_points.len()].copy_from_slice(screen_points);
            let mut len = points.len();
            let last_screen = screen_points.last().copied().unwrap_or(start_screen);
            let add_point = intent.phase == BoardPointerPhase::Up || self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
            if add_point {
                next_points[len] = world;
                next_screen_points[len] = screen;
                len += 1;
            } else if len > 0 {
                next_points[len - 1] = world;
                next_screen_points[len - 1] = screen;
            }
            let merge_mode = pick_merge_mode_for_modifiers(intent.ctrl_or_meta, intent.shift, self.selection_options.mode.as_str());
            let end_screen = next_screen_points[len.saturating_sub(1)];
            let click_only = intent.phase == BoardPointerPhase::Up && distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
            let next = if click_only { BTreeSet::new() } else { self.resolve_area_selection_with_initial(initial_ids, start, &next_points[..len], merge_mode.as_str()) };
            let gesture = (intent.ctrl_or_meta || intent.shift).then_some(merge_mode.as_str());
            let kind = if intent.phase == BoardPointerPhase::Up { BoardPointerPlanKind::SelectionCommit } else { BoardPointerPlanKind::SelectionPreview { start, start_screen } };
            self.build_selection_pointer_plan(kind, &next, initial_ids, gesture, &next_points[..len], &next_screen_points[..len])
        }

        fn plan_link_move_pointer(&self, source_id: &str, start_screen: Option<Point>, screen: Point, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let activated = start_screen.is_none_or(|start_screen| distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX);
            let target_id = activated.then(|| self.nearest_link_snap_handle_world(source_id, world)).flatten();
            let hover_id = target_id.clone().or_else(|| self.resolve_hover_world(world));
            let node_ids = self.link_drag_compatible_target_node_ids(source_id);
            let ring_node_id = activated.then(|| self.link_drag_ring_target_node_id(source_id, world)).flatten().filter(|node_id| self.link_compatible_handle_count_on_node(source_id, node_id) > 1);
            let ring_handle_ids = ring_node_id.as_deref().map(|node_id| self.link_compatible_handle_ids_on_node(source_id, node_id)).unwrap_or_default();
            if node_ids.len().saturating_add(ring_handle_ids.len()) > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            let compat_key = format!("{}|{}", source_id, node_ids.join(","));
            let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let events = [
                (self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str())).then(|| BoardOwnedEvent::link_compatible(source_id, &node_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
                (self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str())).then(|| BoardOwnedEvent::link_ring(source_id, ring_node_id.as_deref(), &ring_handle_ids)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
                (self.hovered_id != hover_id || self.hovered_kind.is_some())
                    .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                    .transpose()
                    .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            ];
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let source = plan.push_id(source_id)?;
            let target = target_id.as_deref().map(|target_id| plan.push_id(target_id)).transpose()?;
            let hover = hover_id.as_deref().map(|hover_id| plan.push_id(hover_id)).transpose()?;
            let compat_key = plan.push_id(&compat_key)?;
            let ring_key = plan.push_id(&ring_key)?;
            plan.kind = BoardPointerPlanKind::LinkMove { source, target, hover, compat_key, ring_key, end_world: world, activated, start_screen: start_screen.unwrap_or(screen) };
            plan.seal_optional_events(&events)?;
            Ok(plan)
        }

        fn planned_link_edge_id(&self, source_handle_id: &str, target_handle_id: &str) -> Option<String> {
            if source_handle_id == target_handle_id || !self.handle_selectable(source_handle_id) || !self.handle_selectable(target_handle_id) {
                return None;
            }
            let source = self.handles.get(source_handle_id)?;
            let target = self.handles.get(target_handle_id)?;
            if source.node_id == target.node_id || !self.handles_link_compatible_for_drag(source, target) || self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
                return None;
            }
            if self.edges.values().any(|edge| edge.source == source_handle_id && edge.target == target_handle_id) {
                return None;
            }
            let mut serial = self.edges.len().saturating_add(1);
            loop {
                let candidate = format!("edge-link-{serial}");
                if !self.edges.contains_key(&candidate) {
                    return Some(candidate);
                }
                serial = serial.saturating_add(1);
            }
        }

        fn plan_link_finish_pointer(&self, source_id: &str, target_id: Option<&str>, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let direct_target = target_id.filter(|target_id| self.link_snap_commit_proximity_ok(target_id, world));
            let mut target_node_id = None;
            let mut edge_target = direct_target.map(str::to_string);
            let mut also_emit = direct_target.map(|_| BoardEventKind::ProximityConnect);
            if edge_target.is_none() {
                if let Some(node_id) = self.resolve_node_hit_world(world).filter(|node_id| self.handles.get(source_id).is_some_and(|source| source.node_id.as_str() != node_id.as_str())) {
                    if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id, &node_id) {
                        edge_target = Some(sole_target);
                        also_emit = Some(BoardEventKind::IndirectConnect);
                    } else if self.node_has_any_free_link_compatible_handle(source_id, &node_id) {
                        target_node_id = Some(node_id);
                    }
                }
            }
            let edge_id = edge_target.as_deref().and_then(|target| self.planned_link_edge_id(source_id, target));
            if edge_id.is_none() {
                edge_target = None;
                also_emit = None;
            }
            let hover_id = target_node_id.clone().or_else(|| self.resolve_hover_world(world));
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let mut events: [Option<BoardOwnedEvent>; 5] = std::array::from_fn(|_| None);
            let mut compat_key_owned = None;
            let mut ring_key_owned = None;
            if let (Some(edge_id), Some(edge_target)) = (edge_id.as_deref(), edge_target.as_deref()) {
                events[0] = Some(BoardOwnedEvent::edge(BoardEventKind::EdgeCreate, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
                if let Some(also_emit) = also_emit {
                    events[1] = Some(BoardOwnedEvent::edge(also_emit, edge_id, source_id, edge_target).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
                }
            }
            if let Some(target_node_id) = target_node_id.as_deref() {
                let node_ids = self.link_drag_compatible_target_node_ids(source_id);
                let ring_handle_ids = self.link_compatible_handle_ids_on_node(source_id, target_node_id);
                compat_key_owned = Some(format!("{}|{}", source_id, node_ids.join(",")));
                ring_key_owned = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
                events[2] = Some(BoardOwnedEvent::link_compatible(source_id, &node_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
                events[3] = Some(BoardOwnedEvent::link_ring(source_id, Some(target_node_id), &ring_handle_ids).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
            } else {
                let empty: &[String] = &[];
                events[2] = self.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
                events[3] = self.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?;
            }
            if self.hovered_id != hover_id || self.hovered_kind.is_some() {
                events[4] = Some(BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))).map_err(|_| BoardPointerPlanFault::ByteCredits)?);
            }
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let source = plan.push_id(source_id)?;
            let target = edge_target.as_deref().map(|target| plan.push_id(target)).transpose()?;
            let edge = edge_id.as_deref().map(|edge| plan.push_id(edge)).transpose()?;
            let target_node = target_node_id.as_deref().map(|target_node| plan.push_id(target_node)).transpose()?;
            let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
            let compat_key = compat_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
            let ring_key = ring_key_owned.as_deref().map(|key| plan.push_id(key)).transpose()?;
            plan.kind = BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, compat_key, ring_key };
            plan.seal_optional_events(&events)?;
            Ok(plan)
        }

        fn plan_link_retain_pointer(&self, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let hover_id = self.resolve_hover_world(world);
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let event = (self.hovered_id != hover_id || self.hovered_kind.is_some())
                .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                .transpose()
                .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
            plan.kind = BoardPointerPlanKind::LinkRetain { hover };
            plan.seal_optional_events(&[event])?;
            Ok(plan)
        }

        fn plan_link_clear_pointer(&self, source_id: &str, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let hover_id = self.resolve_hover_world(world);
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let empty: &[String] = &[];
            let events = [
                self.link_compat_nodes_emit_key.is_some().then(|| BoardOwnedEvent::link_compatible("", empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
                self.link_target_ring_emit_key.is_some().then(|| BoardOwnedEvent::link_ring("", None, empty)).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
                (self.hovered_id != hover_id || self.hovered_kind.is_some())
                    .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                    .transpose()
                    .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            ];
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let source = plan.push_id(source_id)?;
            let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
            plan.kind = BoardPointerPlanKind::LinkFinish { source, target: None, edge: None, target_node: None, hover, compat_key: None, ring_key: None };
            plan.seal_optional_events(&events)?;
            Ok(plan)
        }

        fn plan_hover_pointer(&self, hover_id: Option<String>) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let event = (self.hovered_id != hover_id || self.hovered_kind.is_some())
                .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                .transpose()
                .map_err(|_| BoardPointerPlanFault::ByteCredits)?;
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
            plan.kind = BoardPointerPlanKind::Hover { hover };
            plan.seal_optional_events(&[event])?;
            Ok(plan)
        }

        fn plan_brush_pointer(&self, intent: BoardPointerIntent, world: Point) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            let next_source = if intent.phase == BoardPointerPhase::Move { self.brush_nearest_slot_source(world) } else { None };
            let offset = if intent.alt || self.brush_slot_suggestions_active { self.suggestion_offset } else { 0.0 };
            let next_candidates = if let Some(source_id) = next_source.as_deref() {
                if self.brush_slot_source_id.as_deref() == Some(source_id) {
                    self.brush_candidates.clone()
                } else {
                    let source = self.handles.get(source_id).ok_or(BoardPointerPlanFault::Unsupported)?;
                    self.brush_compatible_candidates(source).ok_or(BoardPointerPlanFault::ItemCredits)?
                }
            } else {
                BrushCandidatePage::default()
            };
            if next_candidates.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            let next_index = if self.brush_slot_source_id == next_source { self.brush_candidate_index.min(next_candidates.len().saturating_sub(1)) } else { 0 };
            let next_preview = next_source.as_deref().and_then(|source_id| next_candidates.get(next_index).and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
            let leaving_old = self.brush_slot_source_id.is_some() && self.brush_slot_source_id != next_source;
            let commit_old = intent.alt && leaving_old && self.brush_preview.is_some();
            let old_preview = self.brush_slot_source_id.as_deref().and_then(|source_id| self.brush_candidates.get(self.brush_candidate_index).and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
            let hover_id = next_source.clone();
            let hover_kind = hover_id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            let serial = self.brush_placement_serial.wrapping_add(1);
            let node_id = format!("puzzle2d.brush.{serial}");
            let edge_id = format!("puzzle2d.brush.edge.{serial}");
            let events = [
                commit_old.then(|| old_preview.as_ref().ok_or(BoardEventFault::Schema).and_then(|preview| BoardOwnedEvent::brush_place(preview, &node_id, &edge_id))).transpose().map_err(|_| BoardPointerPlanFault::ByteCredits)?,
                Some(BoardOwnedEvent::brush_preview(next_preview.as_ref()).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
                Some(BoardOwnedEvent::brush_candidates(next_source.as_deref().unwrap_or(""), &next_candidates, next_index, false).map_err(|_| BoardPointerPlanFault::ByteCredits)?),
                (self.hovered_id != hover_id || self.hovered_kind.is_some())
                    .then(|| BoardOwnedEvent::hover(hover_id.as_deref(), hover_kind.as_ref().map(|(domain, kind_id)| (domain.as_str(), kind_id.as_str()))))
                    .transpose()
                    .map_err(|_| BoardPointerPlanFault::ByteCredits)?,
            ];
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle);
            let source = next_source.as_deref().map(|source| plan.push_id(source)).transpose()?;
            let hover = hover_id.as_deref().map(|hover| plan.push_id(hover)).transpose()?;
            for candidate in next_candidates.iter() {
                plan.push_delta(candidate.node_kind_id, candidate.target_handle_index as f64, 0.0)?;
            }
            plan.kind = BoardPointerPlanKind::Brush { source, hover, alt: intent.alt, commit_old };
            plan.seal_optional_events(&events)?;
            Ok(plan)
        }

        pub fn plan_pointer(&self, intent: BoardPointerIntent) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            if self.interaction_revision == u64::MAX {
                return Err(BoardPointerPlanFault::Unsupported);
            }
            let screen = Point::new(intent.x, intent.y);
            let world = self.screen_to_world(screen);
            if self.active_utility == ActiveUtility::Brush && !matches!(self.interaction, Interaction::Pan { .. }) {
                return self.plan_brush_pointer(intent, world);
            }
            match (intent.phase, &self.interaction) {
                (BoardPointerPhase::Move, Interaction::Pan { origin, start_screen }) => {
                    let delta = screen - *start_screen;
                    let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                    Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Pan { camera }))
                }
                (BoardPointerPhase::Move, Interaction::DragNodes { primary_id, offset, start_positions, .. }) => self.plan_drag_pointer(world, primary_id, *offset, start_positions, BoardPointerPlanKind::DragMove),
                (_, Interaction::SelectionPending { initial_ids, start, start_screen }) => self.plan_selection_pending_pointer(intent, initial_ids, *start, *start_screen, screen, world),
                (_, Interaction::Selection { initial_ids, points, screen_points, start, start_screen }) => self.plan_selection_pointer(intent, initial_ids, points, screen_points, *start, *start_screen, screen, world),
                (BoardPointerPhase::Move, Interaction::LinkAtSourceHandle { source_id, start_screen }) => self.plan_link_move_pointer(source_id, Some(*start_screen), screen, world),
                (BoardPointerPhase::Move, Interaction::LinkDragSnap { source_id, .. }) => self.plan_link_move_pointer(source_id, None, screen, world),
                (BoardPointerPhase::Up, Interaction::LinkDragSnap { source_id, target_id, .. }) => self.plan_link_finish_pointer(source_id, target_id.as_deref(), world),
                (BoardPointerPhase::Up, Interaction::LinkAtSourceHandle { source_id, .. }) => self.plan_link_finish_pointer(source_id, None, world),
                (BoardPointerPhase::Move, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) => self.plan_link_retain_pointer(world),
                (BoardPointerPhase::Up, Interaction::LinkTargetNode { source_id, .. }) => self.plan_link_clear_pointer(source_id, world),
                (BoardPointerPhase::Up, Interaction::ExternalLinkPreview { .. }) => self.plan_link_retain_pointer(world),
                (BoardPointerPhase::Move, Interaction::None) => self.plan_hover_pointer(self.resolve_hover_world(world)),
                (BoardPointerPhase::Up, Interaction::Pan { origin, start_screen }) => {
                    let delta = screen - *start_screen;
                    let camera = [origin.x - delta.x / origin.zoom, origin.y - delta.y / origin.zoom, origin.zoom];
                    let mut plan = BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::FinishPan { camera });
                    plan.seal_events()?;
                    Ok(plan)
                }
                (BoardPointerPhase::Up, Interaction::DragNodes { primary_id, offset, start_positions, .. }) => self.plan_drag_pointer(world, primary_id, *offset, start_positions, BoardPointerPlanKind::FinishDrag),
                (BoardPointerPhase::Up, Interaction::None) => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
                (BoardPointerPhase::Leave, Interaction::None) => self.plan_hover_pointer(None),
                (BoardPointerPhase::Leave, _) => Ok(BoardPointerPlan::empty(self.interaction_revision, BoardPointerPlanKind::Idle)),
            }
        }

        fn plan_drag_pointer(&self, world: Point, primary_id: &str, offset: Vec2, start_positions: &BTreeMap<String, (f64, f64)>, kind: BoardPointerPlanKind) -> Result<BoardPointerPlan, BoardPointerPlanFault> {
            if start_positions.len() > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardPointerPlanFault::ItemCredits);
            }
            let Some((px0, py0)) = start_positions.get(primary_id).copied() else {
                return Err(BoardPointerPlanFault::Unsupported);
            };
            let nx = world.x - offset.x;
            let ny = world.y - offset.y;
            let (dx, dy) = if self.grid_snap_enabled {
                let (snx, sny) = self.snap_world_pair(nx, ny);
                (snx - px0, sny - py0)
            } else {
                (nx - px0, ny - py0)
            };
            let mut plan = BoardPointerPlan::empty(self.interaction_revision, kind);
            for (id, (x, y)) in start_positions {
                plan.push_delta(id, x + dx, y + dy)?;
            }
            if matches!(kind, BoardPointerPlanKind::FinishDrag) {
                plan.seal_events()?;
            }
            Ok(plan)
        }

        pub fn begin_pointer_commit(&mut self, plan: BoardPointerPlan) -> Result<(), BoardPointerPlan> {
            if self.pointer_publication.is_some() || self.interaction_revision != plan.revision || !plan.requires_retained_commit() {
                return Err(plan);
            }
            if let Some(active) = self.pending_pointer_commit.as_ref() {
                if active.plan.revision != plan.revision || !matches!(active.plan.kind, BoardPointerPlanKind::DragMove) || !matches!(plan.kind, BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag) {
                    return Err(plan);
                }
                match self.queued_pointer_commit.as_ref().map(|queued| queued.kind) {
                    None => {
                        self.queued_pointer_commit = Some(plan);
                        return Ok(());
                    }
                    Some(BoardPointerPlanKind::DragMove) => {
                        self.queued_pointer_commit = Some(plan);
                        return Ok(());
                    }
                    Some(_) => return Err(plan),
                }
            }
            self.pending_pointer_commit = Some(BoardPointerCommitOperation::new(plan));
            Ok(())
        }

        pub fn step_pointer_commit(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> BoardAuthorityStep {
            let Some(mut operation) = self.pending_pointer_commit.take() else {
                return BoardAuthorityStep::Complete;
            };
            if context.should_yield() {
                self.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if context.is_cancelled() && operation.phase == 0 {
                operation.cancelling = true;
            }
            if self.interaction_revision != operation.plan.revision && operation.phase == 0 {
                operation.cancelling = true;
                operation.faulted = true;
            }
            if operation.cancelling {
                if operation.cursor < operation.plan.delta_len {
                    operation.plan.deltas[usize::from(operation.cursor)] = None;
                    operation.cursor += 1;
                    context.consume_fuel(1);
                    self.pending_pointer_commit = Some(operation);
                    return BoardAuthorityStep::Pending;
                }
                if operation.retire_one() || operation.points.pop().is_some() || operation.screen_points.pop().is_some() || operation.overlay_points.pop().is_some() {
                    context.consume_fuel(1);
                    self.pending_pointer_commit = Some(operation);
                    return BoardAuthorityStep::Pending;
                }
                if operation.scan_after.take().is_some() {
                    context.consume_fuel(1);
                    self.pending_pointer_commit = Some(operation);
                    return BoardAuthorityStep::Pending;
                }
                return if operation.faulted { BoardAuthorityStep::Fault } else { BoardAuthorityStep::Cancelled };
            }
            let complete = self.step_pointer_commit_operation(&mut operation);
            context.consume_fuel(1);
            if !complete {
                self.pending_pointer_commit = Some(operation);
                return BoardAuthorityStep::Pending;
            }
            if operation.faulted {
                return BoardAuthorityStep::Fault;
            }
            let completed_revision = operation.plan.revision;
            if self.interaction_revision != completed_revision {
                return BoardAuthorityStep::Fault;
            }
            self.interaction_revision = completed_revision.wrapping_add(1);
            if operation.plan.output_len > 2 {
                self.pointer_publication = Some(BoardPointerPublication { bytes: operation.plan.output, len: operation.plan.output_len });
            }
            if let Some(mut queued) = self.queued_pointer_commit.take() {
                if queued.revision != completed_revision {
                    return BoardAuthorityStep::Fault;
                }
                queued.revision = self.interaction_revision;
                self.pending_pointer_commit = Some(BoardPointerCommitOperation::new(queued));
                return BoardAuthorityStep::Pending;
            }
            BoardAuthorityStep::Complete
        }

        pub fn take_pointer_publication(&mut self) -> Option<BoardPointerPublication> {
            self.pointer_publication.take()
        }

        pub fn pointer_publication(&self) -> Option<&BoardPointerPublication> {
            self.pointer_publication.as_ref()
        }

        pub fn close_pointer_authority_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
            if let Some(operation) = self.pending_pointer_commit.as_mut() {
                if operation.cursor == 0 {
                    operation.cancelling = true;
                }
                let _ = self.step_pointer_commit(context);
                return false;
            }
            if self.queued_pointer_commit.take().is_some() {
                context.consume_fuel(1);
                return false;
            }
            if let Some(publication) = self.pointer_publication.as_mut() {
                if publication.close_step() {
                    self.pointer_publication = None;
                }
                return false;
            }
            true
        }

        pub fn pointer_authority_terminal_is_empty(&self) -> bool {
            self.pending_pointer_commit.is_none() && self.queued_pointer_commit.is_none() && self.pointer_publication.is_none()
        }

        pub fn commit_pointer(&mut self, plan: &BoardPointerPlan) -> bool {
            if self.interaction_revision != plan.revision {
                return false;
            }
            if !matches!(plan.kind, BoardPointerPlanKind::Idle) {
                return false;
            }
            self.interaction_revision = plan.revision.wrapping_add(1);
            true
        }

        #[cfg(test)]
        fn commit_pointer_legacy_differential(&mut self, plan: &BoardPointerPlan) -> bool {
            match plan.kind {
                BoardPointerPlanKind::Idle => {}
                BoardPointerPlanKind::Pan { camera } => {
                    self.camera = Camera { x: camera[0], y: camera[1], zoom: infinite::canvas::camera::clamp_zoom(camera[2]) };
                }
                BoardPointerPlanKind::DragMove | BoardPointerPlanKind::FinishDrag => unreachable!("drag plans are committed through the retained pointer authority"),
                BoardPointerPlanKind::FinishPan { camera } => {
                    self.camera = Camera { x: camera[0], y: camera[1], zoom: infinite::canvas::camera::clamp_zoom(camera[2]) };
                    self.interaction = Interaction::None;
                }
                BoardPointerPlanKind::SelectionPreview { start, start_screen } => {
                    let grabbed = std::mem::take(&mut self.interaction);
                    let initial_ids = match grabbed {
                        Interaction::SelectionPending { initial_ids, .. } | Interaction::Selection { initial_ids, .. } => initial_ids,
                        other => {
                            self.interaction = other;
                            return false;
                        }
                    };
                    let next: BTreeSet<String> = plan.selection_ids().map(str::to_string).collect();
                    self.preselect_removed = initial_ids.difference(&next).cloned().collect();
                    self.preselect = next;
                    self.last_preselect_emit_sig = None;
                    self.sync_selection_flags_to_objects();
                    let points = plan.points[..usize::from(plan.point_len)].to_vec();
                    let screen_points = plan.screen_points[..usize::from(plan.point_len)].to_vec();
                    self.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
                BoardPointerPlanKind::SelectionCommit => {
                    if !matches!(self.interaction, Interaction::SelectionPending { .. } | Interaction::Selection { .. }) {
                        return false;
                    }
                    self.interaction = Interaction::None;
                    self.selection = plan.selection_ids().map(str::to_string).collect();
                    self.preselect.clear();
                    self.preselect_removed.clear();
                    self.selection_exit_highlight.clear();
                    self.last_select_emit_sig = None;
                    self.last_preselect_emit_sig = None;
                    self.selection_screen_preview = None;
                    self.selection_preview_crossing = false;
                    self.sync_selection_flags_to_objects();
                    self.bump_content_scene_generation();
                }
                BoardPointerPlanKind::LinkMove { source, target, hover, end_world, activated, start_screen, .. } => {
                    if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. }) {
                        return false;
                    }
                    let source_id = plan.id(source).to_string();
                    let target_id = target.map(|target| plan.id(target).to_string());
                    let hover_id = hover.map(|hover| plan.id(hover).to_string());
                    let node_ids = self.link_drag_compatible_target_node_ids(&source_id);
                    let ring_node_id = activated.then(|| self.link_drag_ring_target_node_id(&source_id, end_world)).flatten().filter(|node_id| self.link_compatible_handle_count_on_node(&source_id, node_id) > 1);
                    let ring_handle_ids = ring_node_id.as_deref().map(|node_id| self.link_compatible_handle_ids_on_node(&source_id, node_id)).unwrap_or_default();
                    self.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                    self.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(",")));
                    self.hovered_id = hover_id;
                    self.hovered_kind = None;
                    self.interaction = if activated { Interaction::LinkDragSnap { source_id, target_id, end_world } } else { Interaction::LinkAtSourceHandle { source_id, start_screen } };
                    self.bump_content_scene_generation();
                }
                BoardPointerPlanKind::LinkFinish { source, target, edge, target_node, hover, .. } => {
                    if !matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. }) {
                        return false;
                    }
                    let source_id = plan.id(source).to_string();
                    let target_id = target.map(|target| plan.id(target).to_string());
                    let edge_id = edge.map(|edge| plan.id(edge).to_string());
                    let target_node_id = target_node.map(|target_node| plan.id(target_node).to_string());
                    let hover_id = hover.map(|hover| plan.id(hover).to_string());
                    if let (Some(target_id), Some(edge_id)) = (target_id, edge_id) {
                        let Some(source_row) = self.handles.get(&source_id) else {
                            return false;
                        };
                        let Some(target_row) = self.handles.get(&target_id) else {
                            return false;
                        };
                        let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
                        self.edges.insert(
                            edge_id.clone(),
                            EdgeData { id: edge_id, source: source_id.clone(), target: target_id, selected: false, visible: true, locked: false, style: None, edge_kind, source_tip: None, target_tip: None, properties: graph::PropertyBag::new() },
                        );
                    }
                    if let Some(target_node_id) = target_node_id {
                        let node_ids = self.link_drag_compatible_target_node_ids(&source_id);
                        let ring_handle_ids = self.link_compatible_handle_ids_on_node(&source_id, &target_node_id);
                        self.link_compat_nodes_emit_key = Some(format!("{}|{}", source_id, node_ids.join(",")));
                        self.link_target_ring_emit_key = Some(format!("{}|{}|{}", source_id, target_node_id, ring_handle_ids.join(",")));
                        self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                    } else {
                        self.link_compat_nodes_emit_key = None;
                        self.link_target_ring_emit_key = None;
                        self.interaction = Interaction::None;
                    }
                    self.hovered_id = hover_id;
                    self.hovered_kind = None;
                    self.bump_content_scene_generation();
                }
                BoardPointerPlanKind::LinkRetain { hover } => {
                    if !matches!(self.interaction, Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                        return false;
                    }
                    self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                    self.hovered_kind = None;
                }
                BoardPointerPlanKind::Hover { hover } => {
                    if !matches!(self.interaction, Interaction::None) {
                        return false;
                    }
                    self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                    self.hovered_kind = None;
                }
                BoardPointerPlanKind::Brush { source, hover, alt, commit_old } => {
                    if self.active_utility != ActiveUtility::Brush || matches!(self.interaction, Interaction::Pan { .. }) {
                        return false;
                    }
                    let source_id = source.map(|source| plan.id(source).to_string());
                    let mut candidates = BrushCandidatePage::default();
                    for candidate in plan.deltas[..usize::from(plan.delta_len)].iter().flatten() {
                        if candidates.push(plan.id(candidate.id), candidate.x as usize, 0.0).is_err() {
                            return false;
                        }
                    }
                    if commit_old {
                        self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
                    }
                    self.brush_alt_pressed = alt;
                    self.brush_slot_suggestions_active = false;
                    self.brush_slot_source_id = source_id.clone();
                    self.brush_candidates = candidates;
                    self.brush_candidate_index = 0;
                    let offset = if alt { self.suggestion_offset } else { 0.0 };
                    self.brush_preview = source_id.as_deref().and_then(|source_id| self.brush_candidates.first().and_then(|candidate| self.brush_build_preview_with_offset(source_id, candidate, offset)));
                    self.brush_preview_emit_key = None;
                    self.brush_candidates_emit_key = None;
                    self.hovered_id = hover.map(|hover| plan.id(hover).to_string());
                    self.hovered_kind = None;
                    self.interaction = Interaction::None;
                    self.bump_content_scene_generation();
                }
                BoardPointerPlanKind::LeaveIdle => {
                    self.hovered_id = None;
                    self.hovered_kind = None;
                }
            }
            self.interaction_revision = plan.revision.wrapping_add(1);
            true
        }

        pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
            self.set_selection_screen_preview(None);
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_utility == ActiveUtility::Brush {
                if button == 1 {
                    self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
                }
                return;
            }
            let hit = self.resolve_hit_world(world).or_else(|| self.resolve_overview_free_link_handle_pointer_world(world));
            if let Interaction::LinkTargetNode { source_id, target_node_id } = self.interaction.clone() {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                if button == 0 {
                    if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.try_commit_link_edge(&source_id, &th, Some(BoardEventKind::IndirectConnect));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    if let Some(hid) = hit.as_ref().filter(|id| self.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && self.handle_eligible_link_target_ring(id.as_str(), source_id.as_str())) {
                        self.try_commit_link_edge(&source_id, hid, Some(BoardEventKind::IndirectConnect));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.update_hover_from_world(world);
                return;
            }
            if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
                if button == 0 {
                    if let Some(target_node_id) = ring_node_id {
                        if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                            if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                                self.interaction = Interaction::None;
                                self.clear_link_gesture_events();
                                self.try_commit_link_edge(&source_id, &th, Some(BoardEventKind::IndirectConnect));
                                self.update_hover_from_world(world);
                                return;
                            }
                        }
                    }
                    if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                        self.interaction = Interaction::None;
                        self.clear_link_gesture_events();
                        self.try_commit_link_edge(&source_id, hid, Some(BoardEventKind::IndirectConnect));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.update_hover_from_world(world);
                return;
            }
            let merge_from_modifiers = ctrl_or_meta || shift;
            let pick_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
            if button == 0 && !merge_from_modifiers && self.try_begin_bounded_selection_drag_at(world) {
                return;
            }
            if button == 1 {
                self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
                return;
            }
            if let Some(ref hid) = hit {
                if let Some(node) = self.nodes.get(hid) {
                    if node.draggable && !node.locked {
                        let nid = hid.clone();
                        let nx = node.x;
                        let ny = node.y;
                        let members_before: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                        let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                        let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                        if !drag_group_before || force_pick_merge {
                            let next = merge_pick_into_selection(&self.selection, &nid, pick_mode.as_str());
                            let ids: Vec<_> = next.iter().cloned().collect();
                            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                            self.set_selection_ids_gestured(&ids, gesture);
                        }
                        let members: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                        let drag_group = members.contains(&nid) && members.len() > 1;
                        let mut start_positions = BTreeMap::new();
                        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                            if let Some(n) = self.nodes.get(id) {
                                start_positions.insert(id.clone(), (n.x, n.y));
                            }
                        }
                        self.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None };
                        self.set_hovered_id(hit);
                        return;
                    }
                }
            }
            if let Some(ref hid) = hit {
                if button == 0 && self.handles.contains_key(hid) && !self.handle_has_incident_edge(hid.as_str()) {
                    let next = merge_pick_into_selection(&self.selection, hid, pick_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                    self.set_selection_ids_gestured(&ids, gesture);
                    self.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                    self.set_hovered_id(Some(hid.clone()));
                    return;
                }
            }
            if hit.is_none() && button == 0 {
                self.interaction = Interaction::SelectionPending { initial_ids: self.selection.clone(), start: world, start_screen: screen };
                self.set_hovered_id(None);
                return;
            }
            self.interaction = Interaction::None;
            if let Some(id) = hit {
                let next = merge_pick_into_selection(&self.selection, &id, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&ids, gesture);
                self.set_hovered_id(Some(id));
            } else {
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&[], gesture);
                self.set_hovered_id(None);
            }
        }

        pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_utility == ActiveUtility::Brush {
                self.brush_update_alt(alt);
                match std::mem::replace(&mut self.interaction, Interaction::None) {
                    Interaction::Pan { origin, start_screen } => {
                        let delta = screen - start_screen;
                        let nx = origin.x - delta.x / origin.zoom;
                        let ny = origin.y - delta.y / origin.zoom;
                        self.set_camera(nx, ny, origin.zoom);
                        self.interaction = Interaction::Pan { origin, start_screen };
                    }
                    _ => {
                        self.interaction = Interaction::None;
                        self.brush_pointer_move(world);
                    }
                }
                return;
            }
            match std::mem::replace(&mut self.interaction, Interaction::None) {
                Interaction::DragNodes { primary_id, offset, start_positions, proximity_pair: retained_proximity_pair } => {
                    let primary_id = primary_id.clone();
                    let start_positions_cloned = start_positions.clone();
                    let (px0, py0) = start_positions.get(&primary_id).copied().unwrap_or((0.0, 0.0));
                    let nx = world.x - offset.x;
                    let ny = world.y - offset.y;
                    let mut dx = nx - px0;
                    let mut dy = ny - py0;
                    if self.grid_snap_enabled {
                        let (snx, sny) = self.snap_world_pair(nx, ny);
                        dx = snx - px0;
                        dy = sny - py0;
                    }
                    let mut event_count = 0usize;
                    let mut event_bytes = 0usize;
                    for (id, (ox0, oy0)) in &start_positions {
                        if self.nodes.contains_key(id) {
                            event_count += 1;
                            let Some(bytes) = board_node_move_owned_bytes(id, ox0 + dx, oy0 + dy) else {
                                self.event_schema_fault = true;
                                self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair: retained_proximity_pair };
                                return;
                            };
                            let Some(total) = event_bytes.checked_add(bytes) else {
                                self.event_schema_fault = true;
                                self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair: retained_proximity_pair };
                                return;
                            };
                            event_bytes = total;
                        }
                    }
                    if self.events.reserve(event_count, event_bytes).is_err() {
                        self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair: retained_proximity_pair };
                        return;
                    }
                    let mut geometry_changed = false;
                    for (id, (ox0, oy0)) in &start_positions {
                        if let Some(n) = self.nodes.get_mut(id) {
                            let mx = ox0 + dx;
                            let my = oy0 + dy;
                            if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                                geometry_changed = true;
                            }
                            n.x = mx;
                            n.y = my;
                            let event = BoardOwnedEvent::node_move(id, mx, my).expect("node move event was exactly preflighted");
                            self.events.push(event).expect("node move event credits were reserved before mutation");
                        }
                    }
                    if geometry_changed {
                        self.bump_content_scene_generation();
                    }
                    let proximity_pair = if start_positions.len() == 1 { self.node_drag_proximity_handle_pair(primary_id.as_str()) } else { None };
                    self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair };
                }
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    self.set_camera(nx, ny, origin.zoom);
                    self.interaction = Interaction::Pan { origin, start_screen };
                }
                Interaction::SelectionPending { initial_ids, start, start_screen } => {
                    if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                        self.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                    } else {
                        let points = vec![start, world];
                        let screen_points = vec![start_screen, screen];
                        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        let merge_from_modifiers = ctrl_or_meta || shift;
                        let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                        self.apply_area_preselect(&initial_ids, &ids, gesture);
                        self.sync_selection_screen_overlay(start_screen, &screen_points);
                        self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                    }
                }
                Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                    let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                    let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                    if add_point {
                        points.push(world);
                        screen_points.push(screen);
                    } else if !points.is_empty() {
                        let last = points.len() - 1;
                        points[last] = world;
                        let ls = screen_points.len() - 1;
                        screen_points[ls] = screen;
                    }
                    let initial = initial_ids.clone();
                    let pts = points.clone();
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial, start, &pts, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    self.apply_area_preselect(&initial, &ids, gesture);
                    self.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
                Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                    if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                        let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                        self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                        self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                        self.sync_link_gesture_events();
                    } else {
                        self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                        self.update_hover_from_world(world);
                    }
                }
                Interaction::LinkDragSnap { source_id, .. } => {
                    let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    self.sync_link_gesture_events();
                }
                Interaction::LinkTargetNode { source_id, target_node_id } => {
                    self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                    self.update_hover_from_world(world);
                }
                Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                    self.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                    self.update_hover_from_world(world);
                }
                Interaction::None => {
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
            }
        }

        pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool, alt: bool) {
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_utility == ActiveUtility::Brush {
                self.brush_update_alt(alt);
                if matches!(self.interaction, Interaction::Pan { .. }) {
                    self.interaction = Interaction::None;
                }
                self.brush_finish_slot();
                self.set_hovered_id(None);
                return;
            }
            let grabbed = std::mem::take(&mut self.interaction);
            match grabbed {
                Interaction::LinkDragSnap { source_id, target_id, .. } => {
                    if let Some(ref target_handle_id) = target_id {
                        if self.link_snap_commit_proximity_ok(target_handle_id, world) && self.try_commit_link_edge(&source_id, target_handle_id, Some(BoardEventKind::ProximityConnect)) {
                            self.interaction = Interaction::None;
                            self.clear_link_gesture_events();
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    if let Some(target_node_id) = self.resolve_node_hit_world(world) {
                        let source_node_id = self.handles.get(&source_id).map(|h| h.node_id.clone());
                        if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                            if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id.as_str(), target_node_id.as_str()) {
                                self.try_commit_link_edge(&source_id, &sole_target, Some(BoardEventKind::IndirectConnect));
                                self.clear_link_gesture_events();
                            } else {
                                self.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                                self.set_hovered_id(Some(target_node_id));
                                self.sync_link_gesture_events();
                            }
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.update_hover_from_world(world);
                }
                Interaction::LinkAtSourceHandle { .. } => {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.update_hover_from_world(world);
                }
                Interaction::DragNodes { start_positions, proximity_pair: Some((src, tgt)), .. } => {
                    let _ = self.try_commit_link_edge(&src, &tgt, Some(BoardEventKind::ProximityConnect));
                    self.push_node_drag_end_events(&start_positions);
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
                Interaction::DragNodes { start_positions, .. } => {
                    self.push_node_drag_end_events(&start_positions);
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
                Interaction::SelectionPending { initial_ids, start, start_screen } => {
                    let _ = (start, start_screen);
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    if !merge_from_modifiers {
                        self.clear_selection_on_background_click();
                    } else {
                        let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                        let gesture = Some(merge_mode.as_str());
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &[start], merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        self.set_selection_ids_gestured(&ids, gesture);
                    }
                    self.set_selection_screen_preview(None);
                    self.update_hover_from_world(world);
                }
                Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                    points.push(world);
                    screen_points.push(screen);
                    let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                    let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let merge_mode = pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    if click_only {
                        self.commit_area_select_from_initial(&initial_ids, &[], gesture);
                    } else {
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        self.commit_area_select_from_initial(&initial_ids, &ids, gesture);
                    }
                    self.set_selection_screen_preview(None);
                    self.update_hover_from_world(world);
                }
                Interaction::ExternalLinkPreview { .. } => {
                    self.interaction = grabbed;
                    self.update_hover_from_world(world);
                }
                _ => {
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
            }
        }

        pub fn pointer_leave_screen(&mut self, alt: bool) {
            self.interaction_revision = self.interaction_revision.wrapping_add(1);
            if self.active_utility == ActiveUtility::Brush {
                self.brush_update_alt(alt);
                self.brush_finish_slot();
                self.set_hovered_id(None);
                return;
            }
            if matches!(self.interaction, Interaction::None) {
                self.set_hovered_id(None);
            }
        }

        /// @emoji ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
        pub fn cancel_area_select(&mut self) -> bool {
            let reservation = match &self.interaction {
                Interaction::Selection { initial_ids, .. } => {
                    let event = BoardOwnedEvent::id_list(BoardEventKind::PreselectCancel, "ids", initial_ids.iter().map(String::as_str));
                    let Some(reservation) = self.reserve_owned_event(event) else {
                        return false;
                    };
                    Some(reservation)
                }
                _ => None,
            };
            let prev = std::mem::replace(&mut self.interaction, Interaction::None);
            match prev {
                Interaction::SelectionPending { .. } => {
                    self.set_selection_screen_preview(None);
                    true
                }
                Interaction::Selection { initial_ids, .. } => {
                    self.set_selection_screen_preview(None);
                    self.preselect.clear();
                    self.preselect_removed.clear();
                    self.last_preselect_emit_sig = None;
                    self.selection = initial_ids.clone();
                    self.sync_selection_flags_to_objects();
                    self.bump_content_scene_generation();
                    self.last_select_emit_sig = None;
                    self.publish_event_reservation(reservation.expect("selection cancellation reserved one event"));
                    true
                }
                other => {
                    self.interaction = other;
                    false
                }
            }
        }

        fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
            let raw = match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    WorldBox { min_x: n.x - hw, min_y: n.y - hh, max_x: n.x + hw, max_y: n.y + hh }
                }
                NodeShape::Circle => WorldBox { min_x: n.x - self.scaled_node_radius(n), min_y: n.y - self.scaled_node_radius(n), max_x: n.x + self.scaled_node_radius(n), max_y: n.y + self.scaled_node_radius(n) },
            };
            inflate_world_box(raw, pad)
        }

        fn selection_draggable_node_members(&self) -> Vec<String> {
            self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
        }

        fn selection_union_bounds_world(&self) -> Option<WorldBox> {
            let mut corners: Vec<Point> = Vec::new();
            for id in &self.selection {
                let Some(n) = self.nodes.get(id) else {
                    continue;
                };
                let b = self.node_world_bounds(n, 0.0);
                corners.push(Point::new(b.min_x, b.min_y));
                corners.push(Point::new(b.max_x, b.max_y));
            }
            world_box_from_points(&corners)
        }

        /// @emoji 📦️ Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
        fn try_begin_bounded_selection_drag_at(&mut self, world: Point) -> bool {
            if !self.lod_uses_bounded_drag() {
                return false;
            }
            let members = self.selection_draggable_node_members();
            if members.is_empty() {
                return false;
            }
            let Some(bounds) = self.selection_union_bounds_world() else {
                return false;
            };
            let pad = BOUNDED_DRAG_HIT_PAD_PX / self.camera.zoom.max(1e-9);
            if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
                return false;
            }
            let primary_id = members
                .iter()
                .min_by(|a, b| {
                    let da = self.nodes.get(*a).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                    let db = self.nodes.get(*b).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned()
                .unwrap_or_else(|| members[0].clone());
            let (px0, py0) = self.nodes.get(&primary_id).map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0));
            let mut start_positions = BTreeMap::new();
            for id in &members {
                if let Some(n) = self.nodes.get(id) {
                    start_positions.insert(id.clone(), (n.x, n.y));
                }
            }
            self.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None };
            self.set_hovered_id(None);
            true
        }

        fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
            selection_drag_shape(self.selection_options.method.as_str(), start, points)
        }

        fn selection_contains_node(&self, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            let bounds = self.node_world_bounds(n, 0.0);
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    polygon_contains_world_box(polygon, bounds)
                } else {
                    world_box_contains_box(box_, bounds)
                }
            } else if lasso {
                polygon_intersects_world_box(polygon, bounds)
            } else {
                world_boxes_overlap(box_, bounds)
            }
        }

        fn selection_contains_handle(&self, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            let Some(pos) = self.handle_world_pos(h) else {
                return false;
            };
            let pad = self.effective_handle_radius(h).max(1.0);
            let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    polygon_contains_world_box(polygon, bounds)
                } else {
                    world_box_contains_box(box_, bounds)
                }
            } else if lasso {
                polygon_intersects_world_box(polygon, bounds)
            } else {
                world_boxes_overlap(box_, bounds)
            }
        }

        fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            const STEPS: usize = 24;
            let mut samples = Vec::with_capacity(STEPS + 1);
            for i in 0..=STEPS {
                let t = i as f64 / STEPS as f64;
                samples.push(cubic_bezier_point(c, t));
            }
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    samples.iter().all(|&p| point_in_polygon(p, polygon))
                } else {
                    samples.iter().all(|&p| world_box_contains_point(box_, p))
                }
            } else if lasso {
                (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
            } else {
                (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
            }
        }

        fn resolve_area_selection_with_initial(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
            let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
                return initial.clone();
            };
            let mut hits = BTreeSet::new();
            let o = &self.selection_options;
            if o.select_nodes {
                for n in self.nodes.values() {
                    if self.node_selectable(n.id.as_str()) && self.selection_contains_node(n, box_, enclosing, polygon) {
                        hits.insert(n.id.clone());
                    }
                }
            }
            if o.select_handles {
                for h in self.handles.values() {
                    if self.handle_selectable(h.id.as_str()) && self.selection_contains_handle(h, box_, enclosing, polygon) {
                        hits.insert(h.id.clone());
                    }
                }
            }
            if o.select_edges {
                for e in self.edges.values() {
                    if !self.edge_selectable(e) {
                        continue;
                    }
                    if let Some(c) = self.edge_curve(e) {
                        if self.selection_contains_edge(c, box_, enclosing, polygon) {
                            hits.insert(e.id.clone());
                        }
                    }
                }
            }
            merge_ids_into_selection(initial, &hits, merge_mode)
        }
    }

    #[doc(hidden)]
    impl BoardHost {
        pub fn test_resolve_node_style_kind(&self, node_id: &str) -> Option<BoardElementStyleKind> {
            self.nodes.get(node_id).map(|n| self.resolve_node_style_kind(n, StyleChromePass::InteractionOverlay))
        }
    }

    impl infinite::canvas::canvas_content::CanvasContent for BoardHost {
        fn build_scene(&self) -> Scene {
            self.build_vector_scene()
        }

        fn clear_color(&self) -> Color {
            self.canvas_theme.raster_clear
        }
    }

    #[cfg(test)]
    #[test]
    fn board_fill_descriptor_backings_are_exact_heap_owners_with_bounded_transfer_frames() {
        type SourcePages = BoardFillFixedPages<BoardFillSource, { (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        type VirtualHandlePages = BoardFillFixedPages<BoardFillVirtualHandle, { (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS }>;
        let sources = SourcePages::try_new().expect("source descriptor owner");
        let virtual_handles = VirtualHandlePages::try_new().expect("virtual-handle descriptor owner");
        assert_eq!(sources.pages.len(), (BOARD_FILL_SOURCE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(virtual_handles.pages.len(), (BOARD_FILL_PLACEMENT_CAPACITY * BOARD_FILL_KIND_HANDLE_CAPACITY + BOARD_FILL_PAGE_ITEMS - 1) / BOARD_FILL_PAGE_ITEMS);
        assert_eq!(std::mem::size_of::<SourcePages>(), 4 * std::mem::size_of::<usize>());
        assert_eq!(std::mem::size_of::<VirtualHandlePages>(), 4 * std::mem::size_of::<usize>());
        assert!(std::mem::size_of::<BoardFillSnapshot>() <= 1_024);
        assert!(std::mem::size_of::<BoardFillSnapshotCapture>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillSnapshotIngress>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillPlacement>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillCommitPlacement>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillCommitCandidate>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillCommitEncoder>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillJobState>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillCheckpoint>() <= 32 * 1_024);
        assert!(std::mem::size_of::<BoardFillJob>() <= 32 * 1_024);
        assert!(std::mem::size_of::<Result<BoardFillJob, BoardFillCheckpoint>>() <= 32 * 1_024);
        assert!(std::mem::size_of::<semio_framework_job::WorkerJobOutcome<BoardFillJob>>() <= 32 * 1_024);
        assert!(std::mem::size_of::<semio_framework_job::MountedWorkerJobSession<BoardFillJob>>() <= 32 * 1_024);
        assert!(std::mem::size_of::<semio_framework_job::WorkerJobSessionAdmissionRejected<BoardFillJob>>() <= 32 * 1_024);
        assert!(std::mem::size_of::<Result<semio_framework_job::MountedWorkerJobSession<BoardFillJob>, semio_framework_job::WorkerJobSessionAdmissionRejected<BoardFillJob>>>() <= 32 * 1_024);
        assert!(sources.terminal_is_empty());
        assert!(virtual_handles.terminal_is_empty());
        assert!(BoardFillFixedPages::<u8, { usize::MAX }>::try_new().is_err());
    }

    #[cfg(test)]
    fn with_board_step_context<T>(fuel: u64, cancel: semio_framework_job::CancelToken, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
        let mut sequence = 0;
        let mut context =
            semio_framework_job::StepContext::new(semio_framework_job::OperationId(1), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(fuel, u64::MAX), cancel, semio_framework_job::default_now_us, &mut sequence);
        step(&mut context)
    }

    #[cfg(test)]
    fn drive_pointer_commit(host: &mut BoardHost) {
        let cancel = semio_framework_job::root_cancel_token();
        for _ in 0..4096 {
            match with_board_step_context(1, cancel.clone(), |context| host.step_pointer_commit(context)) {
                BoardAuthorityStep::Pending => {}
                BoardAuthorityStep::Complete => return,
                other => panic!("pointer authority failed: {other:?}"),
            }
        }
        panic!("pointer authority did not reach a terminal step");
    }

    #[cfg(test)]
    #[test]
    fn wheel_plan_matches_direct_and_rejects_stale_interaction() {
        let mut direct = BoardHost::default();
        let mut planned = BoardHost::default();
        direct.set_size(800, 600, 1.0);
        planned.set_size(800, 600, 1.0);
        direct.wheel_screen(320.0, 240.0, -12.0);
        let plan = planned.plan_wheel(320.0, 240.0, -12.0);
        assert!(planned.commit_wheel(plan));
        assert_eq!([direct.camera.x, direct.camera.y, direct.camera.zoom], [planned.camera.x, planned.camera.y, planned.camera.zoom]);

        let stale = planned.plan_wheel(320.0, 240.0, -12.0);
        planned.pointer_down_screen(10.0, 10.0, 1, false, false);
        let replacement = [planned.camera.x, planned.camera.y, planned.camera.zoom];
        assert!(!planned.commit_wheel(stale));
        assert_eq!([planned.camera.x, planned.camera.y, planned.camera.zoom], replacement);
    }

    #[cfg(test)]
    #[test]
    fn pointer_pan_plan_matches_direct_and_rejects_stale_interaction() {
        let mut direct = BoardHost::default();
        let mut planned = BoardHost::default();
        direct.set_size(800, 600, 1.0);
        planned.set_size(800, 600, 1.0);
        direct.pointer_down_screen(100.0, 120.0, 1, false, false);
        planned.pointer_down_screen(100.0, 120.0, 1, false, false);
        direct.pointer_move_screen(140.0, 150.0, false, false, false);
        let plan = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 140.0, y: 150.0, shift: false, ctrl_or_meta: false, alt: false }).expect("pan plan");
        planned.begin_pointer_commit(plan).expect("retained pan plan");
        drive_pointer_commit(&mut planned);
        assert_eq!([direct.camera.x, direct.camera.y, direct.camera.zoom], [planned.camera.x, planned.camera.y, planned.camera.zoom]);

        let stale = planned.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 160.0, y: 170.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale pan plan");
        planned.set_size(801, 600, 1.0);
        let camera = [planned.camera.x, planned.camera.y, planned.camera.zoom];
        assert!(planned.begin_pointer_commit(stale).is_err());
        assert_eq!([planned.camera.x, planned.camera.y, planned.camera.zoom], camera);
    }

    #[cfg(test)]
    #[test]
    fn pointer_plan_rejects_drag_item_overflow_and_retires_one_delta_per_step() {
        let mut host = BoardHost::default();
        let start_positions = (0..=BOARD_POINTER_ITEM_CAPACITY).map(|index| (format!("node-{index}"), (index as f64, 0.0))).collect();
        host.interaction = Interaction::DragNodes { primary_id: "node-0".into(), offset: Vec2::ZERO, start_positions, proximity_pair: None };
        assert_eq!(host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 1.0, y: 1.0, shift: false, ctrl_or_meta: false, alt: false }).unwrap_err(), BoardPointerPlanFault::ItemCredits);

        let mut plan = BoardPointerPlan::empty(0, BoardPointerPlanKind::DragMove);
        plan.push_delta("a", 1.0, 2.0).unwrap();
        plan.push_delta("b", 3.0, 4.0).unwrap();
        let mut retirement = BoardPointerPlanRetirement::new(plan);
        assert!(!retirement.close_step());
        assert!(!retirement.close_step());
        assert!(retirement.close_step());
        assert!(retirement.terminal_is_empty());

        let mut escaped = BoardPointerPlan::empty(0, BoardPointerPlanKind::FinishDrag);
        escaped.push_delta("a\"\\\n", 1.0, 2.0).unwrap();
        let mut json = String::new();
        escaped.write_events_json(&mut json).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value[0]["payload"]["moves"][0]["id"], "a\"\\\n");

        let mut oversized = BoardPointerPlan::empty(0, BoardPointerPlanKind::FinishDrag);
        oversized.push_delta(&"\n".repeat(BOARD_POINTER_BYTE_CAPACITY / 2), 1.0, 2.0).unwrap();
        assert_eq!(oversized.write_events_json(&mut json), Err(BoardPointerPlanFault::ByteCredits));
    }

    #[cfg(test)]
    #[test]
    fn drag_commit_obeys_zero_budget_one_delta_turn_cancel_and_publication_witness() {
        let mut host = deletion_fixture("node-a");
        host.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0)), ("node-b".to_string(), (20.0, 0.0))].into_iter().collect(), proximity_pair: None };
        let plan = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 10.0, y: 5.0, shift: false, ctrl_or_meta: false, alt: false }).expect("finish drag plan");
        host.begin_pointer_commit(plan).expect("retained drag commit");
        let before = host.nodes.get("node-a").map(|node| (node.x, node.y));
        let zero = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(0, zero, |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(host.nodes.get("node-a").map(|node| (node.x, node.y)), before);

        let live = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_ne!(host.nodes.get("node-a").map(|node| (node.x, node.y)), before);
        assert_eq!(with_board_step_context(1, live.clone(), |context| host.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, live, |context| host.step_pointer_commit(context)), BoardAuthorityStep::Complete);
        let publication = host.take_pointer_publication().expect("complete drag publication");
        assert!(publication.events_json().contains("nodeDragEnd"));
        assert!(host.pointer_authority_terminal_is_empty());

        let mut cancelled = deletion_fixture("node-a");
        cancelled.interaction = Interaction::DragNodes { primary_id: "node-a".into(), offset: Vec2::ZERO, start_positions: [("node-a".to_string(), (0.0, 0.0))].into_iter().collect(), proximity_pair: None };
        let plan = cancelled.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 8.0, y: 3.0, shift: false, ctrl_or_meta: false, alt: false }).expect("cancelled drag plan");
        cancelled.begin_pointer_commit(plan).expect("cancelled retained commit");
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel, |context| cancelled.step_pointer_commit(context)), BoardAuthorityStep::Cancelled);
        assert_eq!(cancelled.nodes.get("node-a").map(|node| (node.x, node.y)), Some((0.0, 0.0)));
        assert!(cancelled.pointer_authority_terminal_is_empty());
    }

    #[cfg(test)]
    #[test]
    fn selection_move_and_up_are_revisioned_fixed_page_plans() {
        let mut host = BoardHost::default();
        host.set_size(800, 600, 1.0);
        host.pointer_down_screen(100.0, 100.0, 0, false, false);
        assert!(matches!(host.interaction, Interaction::SelectionPending { .. }));
        let preview = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 180.0, y: 160.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection preview plan");
        assert_eq!(preview.event_count(), 1);
        assert!(preview.events_json().contains("preselect"));
        host.begin_pointer_commit(preview).expect("retained selection preview");
        drive_pointer_commit(&mut host);
        let mut publication = host.take_pointer_publication().expect("preview publication");
        assert!(publication.close_step());
        assert!(matches!(host.interaction, Interaction::Selection { .. }));

        let stale = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Move, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("stale selection plan");
        host.interaction_revision = host.interaction_revision.wrapping_add(1);
        assert!(host.begin_pointer_commit(stale).is_err());

        let commit = host.plan_pointer(BoardPointerIntent { phase: BoardPointerPhase::Up, x: 200.0, y: 180.0, shift: false, ctrl_or_meta: false, alt: false }).expect("selection commit plan");
        assert_eq!(commit.event_count(), 1);
        assert!(commit.events_json().contains("select"));
        host.begin_pointer_commit(commit).expect("retained selection commit");
        drive_pointer_commit(&mut host);
        assert!(matches!(host.interaction, Interaction::None));
    }

    #[cfg(test)]
    #[test]
    fn non_drag_pointer_commit_yields_between_selection_link_and_brush_items() {
        let live = semio_framework_job::root_cancel_token();
        let mut selection = deletion_fixture("node-a");
        selection.interaction = Interaction::SelectionPending { initial_ids: ["node-a".to_owned()].into_iter().collect(), start: Point::new(0.0, 0.0), start_screen: Point::new(0.0, 0.0) };
        let mut selection_plan = BoardPointerPlan::empty(selection.interaction_revision, BoardPointerPlanKind::SelectionCommit);
        selection_plan.push_delta("node-a", 0.0, 0.0).unwrap();
        selection_plan.seal_events().unwrap();
        selection.begin_pointer_commit(selection_plan).unwrap();
        assert_eq!(with_board_step_context(0, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.interaction, Interaction::SelectionPending { .. }));
        assert_eq!(with_board_step_context(1, live.clone(), |context| selection.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(matches!(selection.interaction, Interaction::None));
        drive_pointer_commit(&mut selection);
        assert!(selection.selection.contains("node-a"));

        let mut link = BoardHost::default();
        link.interaction = Interaction::LinkAtSourceHandle { source_id: "source".into(), start_screen: Point::new(0.0, 0.0) };
        let mut link_plan = BoardPointerPlan::empty(link.interaction_revision, BoardPointerPlanKind::Idle);
        let source = link_plan.push_id("source").unwrap();
        let compat_key = link_plan.push_id("source|").unwrap();
        let ring_key = link_plan.push_id("source||").unwrap();
        link_plan.kind = BoardPointerPlanKind::LinkMove { source, target: None, hover: None, compat_key, ring_key, end_world: Point::new(2.0, 3.0), activated: true, start_screen: Point::new(0.0, 0.0) };
        link.begin_pointer_commit(link_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert!(link.link_compat_nodes_emit_key.is_none());
        assert_eq!(with_board_step_context(1, live.clone(), |context| link.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(link.link_compat_nodes_emit_key.as_deref(), Some("source|"));
        drive_pointer_commit(&mut link);

        let mut brush = BoardHost::default();
        brush.active_utility = ActiveUtility::Brush;
        let mut brush_plan = BoardPointerPlan::empty(brush.interaction_revision, BoardPointerPlanKind::Idle);
        brush_plan.push_delta("kind-a", 0.0, 0.0).unwrap();
        brush_plan.push_delta("kind-b", 1.0, 0.0).unwrap();
        brush_plan.kind = BoardPointerPlanKind::Brush { source: None, hover: None, alt: false, commit_old: false };
        brush.begin_pointer_commit(brush_plan).unwrap();
        assert_eq!(with_board_step_context(1, live.clone(), |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush_candidates.len(), 0);
        assert_eq!(with_board_step_context(1, live, |context| brush.step_pointer_commit(context)), BoardAuthorityStep::Pending);
        assert_eq!(brush.brush_candidates.len(), 0);
        drive_pointer_commit(&mut brush);
        assert_eq!(brush.brush_candidates.len(), 2);
    }

    #[cfg(test)]
    #[test]
    fn typed_event_queue_preserves_fifo_saturation_and_one_event_close_progress() {
        let mut queue = BoardEventQueue::default();
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let payload = format!(r#"{{"id":"node-{index}"}}"#);
            queue.push(BoardOwnedEvent::from_payload(BoardEventKind::NodeMove, &payload, Some("node")).unwrap()).unwrap();
        }
        let overflow = BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":[]}"#, None).unwrap();
        assert!(queue.push(overflow).is_err());
        assert_eq!(queue.len(), BOARD_EVENT_ITEM_CAPACITY);
        for index in 0..BOARD_EVENT_ITEM_CAPACITY {
            let event = queue.pop().expect("fifo event");
            assert_eq!(event.kind(), BoardEventKind::NodeMove);
            assert!(event.payload_json().contains(&format!("node-{index}")));
        }
        assert!(queue.terminal_is_empty());

        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Select, r#"{"ids":["a"]}"#, None).unwrap()).unwrap();
        queue.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":"a"}"#, None).unwrap()).unwrap();
        assert!(!queue.close_step());
        assert!(!queue.close_step());
        assert!(queue.close_step());
        assert!(queue.terminal_is_empty());
    }

    #[cfg(test)]
    #[test]
    fn selection_event_reservation_is_flat_exact_and_precedes_mutation() {
        let mut host = BoardHost::default();
        host.set_selection_ids(&["a\"\\\n".into(), "b".into()]);
        assert_eq!(host.selection.iter().cloned().collect::<Vec<_>>(), vec!["a\"\\\n".to_string(), "b".to_string()]);
        let event = host.pop_owned_event().expect("flat select event");
        assert_eq!(event.kind(), BoardEventKind::Select);
        let payload: serde_json::Value = serde_json::from_str(event.payload_json()).unwrap();
        assert_eq!(payload["ids"], serde_json::json!(["a\"\\\n", "b"]));

        let mut saturated = BoardHost::default();
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.events.push(BoardOwnedEvent::from_payload(BoardEventKind::Hover, r#"{"id":null}"#, None).unwrap()).unwrap();
        }
        saturated.set_selection_ids(&["retained".into()]);
        assert!(saturated.selection.is_empty());
        assert!(saturated.event_overflow.is_some());
        assert_eq!(saturated.events.len(), BOARD_EVENT_ITEM_CAPACITY);
    }

    #[cfg(test)]
    fn deletion_fixture(node_id: &str) -> BoardHost {
        let mut host = BoardHost::default();
        let fixture = serde_json::json!({
            "schema": "reasoning.mindmap.fixture",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": node_id, "x": 0.0, "y": 0.0, "shape": "circle", "radius": 10.0 },
                { "id": "node-b", "x": 20.0, "y": 0.0, "shape": "circle", "radius": 10.0 }
            ],
            "edges": [{ "id": "edge-a-b", "source": node_id, "target": "node-b" }]
        });
        assert!(host.parse_fixture_v1(&fixture));
        while host.pop_owned_event().is_some() {}
        host.set_selection_ids_silent(&[node_id.to_string()]);
        host
    }

    #[cfg(test)]
    #[test]
    fn delete_plan_retains_fifo_until_exact_credits_and_rejects_stale_or_oversized() {
        let mut saturated = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            saturated.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        saturated.delete_selection();
        assert!(saturated.nodes.contains_key("node-a"));
        assert!(saturated.pending_delete_planning.is_some());
        let live = semio_framework_job::root_cancel_token();
        let mut turns = 0usize;
        while saturated.pending_delete_planning.is_some() || saturated.pending_delete_operation.is_some() {
            turns += 1;
            assert!(turns <= 768);
            let _ = with_board_step_context(1, live.clone(), |context| saturated.step_event_authority(context));
            if let Some(event) = saturated.pop_owned_event() {
                assert_eq!(event.kind(), BoardEventKind::Hover);
            }
        }
        assert!(turns > 4);
        assert!(saturated.pending_delete_operation.is_none());
        assert!(!saturated.nodes.contains_key("node-a"));
        while saturated.pop_owned_event().is_some() {}

        let mut stale = deletion_fixture("node-a");
        for _ in 0..BOARD_EVENT_ITEM_CAPACITY {
            stale.events.push(BoardOwnedEvent::hover(None, None).unwrap()).unwrap();
        }
        stale.delete_selection();
        stale.interaction_revision = stale.interaction_revision.wrapping_add(1);
        assert_eq!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(stale.pending_delete_operation.is_none());
        assert!(stale.nodes.contains_key("node-a"));
        assert!(stale.event_terminal_faulted());

        let oversized_id = "x".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        let mut oversized = deletion_fixture("node-a");
        let mut oversized_node = oversized.nodes.remove("node-a").unwrap();
        oversized_node.id.clone_from(&oversized_id);
        oversized.nodes.insert(oversized_id.clone(), oversized_node);
        oversized.selection.clear();
        oversized.selection.insert(oversized_id.clone());
        oversized.delete_selection();
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, live.clone(), |context| oversized.step_event_authority(context)), BoardAuthorityStep::Fault);
        assert!(oversized.nodes.contains_key(&oversized_id));
        assert!(oversized.pending_delete_planning.is_none());
        assert!(oversized.pending_delete_operation.is_none());
        assert!(oversized.event_terminal_faulted());

        let mut interrupted = deletion_fixture("node-a");
        interrupted.delete_selection();
        assert!(interrupted.pending_delete_planning.is_some());
        let cancel = semio_framework_job::root_cancel_token();
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert_eq!(with_board_step_context(1, cancel.clone(), |context| interrupted.step_event_authority(context)), BoardAuthorityStep::Pending);
        assert!(!with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)));
        let mut close_turns = 1usize;
        while !with_board_step_context(1, cancel.clone(), |context| interrupted.close_event_authority_step(context)) {
            close_turns += 1;
            assert!(close_turns <= 32);
        }
        assert!(close_turns > 4);
        assert!(interrupted.event_authority_terminal_is_empty());
    }

    #[cfg(test)]
    #[test]
    fn delete_property_pre_admission_rejects_hostile_nodes_without_transfer_or_mutation() {
        let mut host = deletion_fixture("node-a");
        host.nodes.get_mut("node-a").unwrap().properties.insert("hostile".into(), graph::manifest::PropertyValue::Array((0..=BOARD_POINTER_ITEM_CAPACITY).map(|_| graph::manifest::PropertyValue::Null).collect()));
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..1024 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.pending_delete_planning.is_none());
        assert!(host.pending_delete_operation.is_none());
        assert!(host.nodes.contains_key("node-a"));
        assert!(host.event_terminal_faulted());

        let mut node = host.nodes.remove("node-a").unwrap();
        let Some(graph::manifest::PropertyValue::Array(mut values)) = node.properties.remove("hostile") else { panic!("hostile property remains retained") };
        while values.pop().is_some() {}
        drop(values);
        drop(node);
    }

    #[cfg(test)]
    #[test]
    fn delete_property_derivation_is_one_node_per_turn_and_cancel_restores_exact_owner() {
        let mut host = deletion_fixture("node-a");
        host.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), graph::manifest::PropertyValue::Array((0..32).map(|index| graph::manifest::PropertyValue::String(format!("value-{index}"))).collect()));
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        let mut observed_audit = false;
        let mut previous_nodes = 0usize;
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| host.step_event_authority(context));
            if let Some(audit) = host.pending_delete_planning.as_ref().and_then(|planning| planning.property_audit.as_ref()) {
                observed_audit = true;
                assert!(audit.nodes.saturating_sub(previous_nodes) <= 1);
                previous_nodes = audit.nodes;
                if audit.nodes >= 4 {
                    break;
                }
            }
        }
        assert!(observed_audit);
        let cancel = semio_framework_job::root_cancel_token();
        cancel.cancel_now();
        let mut turns = 0usize;
        while !with_board_step_context(1, cancel.clone(), |context| host.close_event_authority_step(context)) {
            turns += 1;
            assert!(turns <= 256);
        }
        assert!(host.nodes.contains_key("node-a"));
        let Some(graph::manifest::PropertyValue::Array(values)) = host.nodes.get("node-a").unwrap().properties.get("nested") else { panic!("cancelled property audit restored the original root") };
        assert_eq!(values.len(), 32);
        assert!(host.event_authority_terminal_is_empty());
    }

    #[cfg(test)]
    #[test]
    fn retained_delete_plan_preserves_legacy_fifo_and_retires_mid_audit_stale_generation() {
        let mut legacy = deletion_fixture("node-a");
        legacy.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
        let legacy_plan = legacy.plan_delete_selection().unwrap();
        let legacy_order: Vec<_> = legacy_plan.entries[..usize::from(legacy_plan.len)].iter().flatten().map(|entry| (entry.kind, legacy_plan.id(entry.id).to_owned())).collect();

        let mut retained = deletion_fixture("node-a");
        retained.set_selection_ids_silent(&["node-a".into(), "node-b".into()]);
        retained.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..512 {
            if retained.pending_delete_operation.is_some() {
                break;
            }
            assert_eq!(with_board_step_context(1, live.clone(), |context| retained.step_event_authority(context)), BoardAuthorityStep::Pending);
        }
        let retained_plan = &retained.pending_delete_operation.as_ref().expect("retained plan completed").plan;
        let retained_order: Vec<_> = retained_plan.entries[..usize::from(retained_plan.len)].iter().flatten().map(|entry| (entry.kind, retained_plan.id(entry.id).to_owned())).collect();
        assert_eq!(retained_order, legacy_order);

        let mut stale = deletion_fixture("node-a");
        stale.nodes.get_mut("node-a").unwrap().properties.insert("nested".into(), graph::manifest::PropertyValue::Array(vec![graph::manifest::PropertyValue::String("retained".into())]));
        stale.delete_selection();
        for _ in 0..128 {
            let _ = with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context));
            if stale.pending_delete_planning.as_ref().is_some_and(|planning| planning.property_audit.is_some()) {
                break;
            }
        }
        stale.interaction_revision = stale.interaction_revision.wrapping_add(1);
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| stale.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(stale.nodes.contains_key("node-a"));
        assert!(stale.nodes.get("node-a").unwrap().properties.contains_key("nested"));
        assert!(stale.pending_delete_planning.is_none());
        assert!(stale.event_terminal_faulted());
    }

    #[cfg(test)]
    #[test]
    fn delete_property_key_overflow_faults_after_bounded_restore_without_mutation() {
        let mut host = deletion_fixture("node-a");
        let hostile = "k".repeat(BOARD_POINTER_BYTE_CAPACITY + 1);
        host.nodes.get_mut("node-a").unwrap().properties.insert(hostile.clone(), graph::manifest::PropertyValue::Null);
        host.delete_selection();
        let live = semio_framework_job::root_cancel_token();
        for _ in 0..128 {
            if matches!(with_board_step_context(1, live.clone(), |context| host.step_event_authority(context)), BoardAuthorityStep::Fault) {
                break;
            }
        }
        assert!(host.event_terminal_faulted());
        assert!(host.nodes.contains_key("node-a"));
        assert!(host.nodes.get("node-a").unwrap().properties.contains_key(&hostile));
        assert!(host.pending_delete_planning.is_none());
    }

    #[cfg(test)]
    #[test]
    fn removed_entity_retirement_witness_survives_interruption_and_releases_one_owner_per_turn() {
        let mut host = deletion_fixture("node-a");
        let node = host.nodes.remove("node-a").unwrap();
        let mut retirement = BoardEntityRetirement::new(BoardRemovedEntity::Node(node));
        assert!(!retirement.step().unwrap());
        assert!(!retirement.terminal_is_empty());
        let mut turns = 1usize;
        while !retirement.step().unwrap() {
            turns += 1;
            assert!(turns <= 32);
        }
        assert!(turns > 4);
        assert!(retirement.terminal_is_empty());
    }

    #[cfg(test)]
    #[test]
    fn board_host_nonopaque_close_is_interruptible_and_terminal_witnessed() {
        let host = deletion_fixture("node-a");
        let mut retirement = BoardHostRetirement::new(host);
        let live = semio_framework_job::root_cancel_token();
        assert!(!with_board_step_context(0, live.clone(), |context| retirement.close_step(context)));
        let mut turns = 0usize;
        while !with_board_step_context(1, live.clone(), |context| retirement.close_step(context)) {
            turns += 1;
            assert!(turns < 8_192, "fixed BoardHost close reaches an exact terminal witness");
        }
        assert!(turns > 16);
        assert!(retirement.terminal_nonopaque_is_empty());
    }

    #[cfg(test)]
    fn fill_full_commit_candidate_contract(source: &str) -> bool {
        let Some(start) = source.find("pub struct BoardFillResult") else { return false };
        let Some(end) = source[start..].find("/// 🧵️ Persistent worker-owned fill search") else { return false };
        let codec = &source[start..start + end];
        let Some(job_start) = source.find("fn complete(&mut self, context: &mut semio_framework_job::StepContext") else { return false };
        let Some(job_end) = source[job_start..].find("fn fault_outcome") else { return false };
        let job = &source[job_start..job_start + job_end];
        codec.contains("pub struct BoardFillCommitPlacement")
            && codec.contains("pub edge_kind: BoardFillText")
            && codec.contains("pub handles: [Option<BoardFillCommitHandle>; BOARD_FILL_KIND_HANDLE_CAPACITY]")
            && codec.contains("pub placement: Option<BoardFillCommitPlacement>")
            && codec.contains("struct BoardFillCommitPayload")
            && codec.contains("BoardFillCommitPayload::new(&candidate.output)")
            && codec.contains("payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT")
            && codec.contains("payload.page(page_index)")
            && codec.contains("text_byte: usize")
            && codec.contains("fn step_text")
            && codec.contains("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte]")
            && !codec.contains("String")
            && !codec.contains("Vec<")
            && !codec.contains("BTreeMap")
            && !codec.contains("single_page()")
            && !codec.contains("const BOARD_FILL_COMMIT_BYTES: usize = 13")
            && !codec.contains("result.encode")
            && job.contains("start.saturating_add(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)")
            && job.contains("if end < BOARD_FILL_COMMIT_BYTES")
            && job.contains("encoder.output_cursor = end")
    }

    /// 📦️ Dynamic placement strings and summary-only or single-page terminal restorations fail the full candidate law.
    #[cfg(test)]
    #[test]
    fn fill_full_commit_candidate_mutations_are_rejected() {
        let source = include_str!("🦀️component.rs");
        assert_eq!(BOARD_FILL_COMMIT_BYTES, 10_406);
        assert_eq!(BOARD_FILL_COMMIT_PAGE_COUNT, 1);
        assert!(BOARD_FILL_COMMIT_BYTES <= semio_framework_job::JOB_PAYLOAD_PAGE_BYTES * BOARD_FILL_COMMIT_PAGE_COUNT);
        assert!(fill_full_commit_candidate_contract(source));
        let dynamic = source.replacen("pub struct BoardFillCommitPlacement {\n        pub node_id: BoardFillText,", "pub struct BoardFillCommitPlacement {\n        pub dynamic: String,\n        pub node_id: BoardFillText,", 1);
        assert!(!fill_full_commit_candidate_contract(&dynamic));
        let summary = source.replacen("const BOARD_FILL_COMMIT_BYTES: usize = BOARD_FILL_COMMIT_HANDLE_OFFSET + BOARD_FILL_KIND_HANDLE_CAPACITY * BOARD_FILL_COMMIT_HANDLE_BYTES;", "const BOARD_FILL_COMMIT_BYTES: usize = 13;", 1);
        assert!(!fill_full_commit_candidate_contract(&summary));
    }

    /// 🔡️ Re-coalescing fixed text fails the retained candidate cursor law.
    #[cfg(test)]
    #[test]
    fn fill_commit_candidate_granularity_mutations_are_rejected() {
        let source = include_str!("🦀️component.rs");
        assert!(fill_full_commit_candidate_contract(source));
        let text = source.replacen("self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte];", "self.bytes[offset + 2..offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES].copy_from_slice(&text.bytes);", 1);
        assert!(!fill_full_commit_candidate_contract(&text));
    }

    #[cfg(test)]
    fn fill_ingress_granularity_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillSnapshotIngress") else { return false };
        let ingress = &source[start..start + end];
        [
            "pub fn begin_node(&mut self)",
            "pub fn push_node_id_byte(&mut self, byte: u8)",
            "pub fn set_node_bound(&mut self, index: usize, value: f64)",
            "pub fn begin_handle(&mut self)",
            "pub fn push_handle_text_byte(&mut self, field: BoardFillIngressHandleText, byte: u8)",
            "pub fn begin_kind(&mut self)",
            "pub fn push_kind_text_byte(&mut self, field: BoardFillIngressKindText, byte: u8)",
            "pub fn push_kind_handle_text_byte(&mut self, field: BoardFillIngressTemplateText, byte: u8)",
            "pub fn begin_rule(&mut self)",
            "pub fn push_rule_text_byte(&mut self, field: BoardFillIngressRuleText, byte: u8)",
        ]
        .iter()
        .all(|marker| ingress.contains(marker))
            && ingress.matches("try_push_byte(byte)").count() == 13
            && !ingress.contains("pub fn push_node(")
            && !ingress.contains("pub fn push_handle(")
            && !ingress.contains("pub fn push_kind(")
            && !ingress.contains("pub fn push_rule(")
            && !ingress.contains("for byte")
    }

    /// 🧬️ The mounted capture ingress exposes only retained scalar and byte opportunities.
    #[cfg(test)]
    #[test]
    fn fill_ingress_granularity_mutations_are_rejected() {
        let source = include_str!("🦀️component.rs");
        assert!(fill_ingress_granularity_contract(source));
        let fields = source.replacen("pub fn set_node_bound(&mut self, index: usize, value: f64)", "pub fn push_node(&mut self, bounds: [f64; 4])", 1);
        assert!(!fill_ingress_granularity_contract(&fields));
        let text = source.replacen("pub fn push_node_id_byte(&mut self, byte: u8)", "pub fn push_node_id(&mut self, bytes: &[u8])", 1);
        assert!(!fill_ingress_granularity_contract(&text));
    }

    #[cfg(test)]
    fn fill_owned_page_handback_contract(source: &str) -> bool {
        let Some(start) = source.find("impl BoardFillSnapshotIngress {") else { return false };
        let Some(end) = source[start..].find("impl Drop for BoardFillJob") else { return false };
        let retained = &source[start..start + end];
        let mut insertions = 0usize;
        for line in retained.lines().filter(|line| line.contains("try_push_owned")) {
            insertions += 1;
            if !line.contains("if let Err(") {
                return false;
            }
        }
        insertions == 11 && !retained.contains("try_push_owned(candidate).is_err()")
    }

    /// 🫴️ All eleven fixed-page insertions bind the exact rejected owner, including compatibility MAX+1.
    #[cfg(test)]
    #[test]
    fn fill_owned_page_handback_mutation_is_rejected() {
        let source = include_str!("🦀️component.rs");
        assert!(fill_owned_page_handback_contract(source));
        let marker = "if let Err(candidate) = state.candidates.try_push_owned(candidate) {";
        let production_end = source.find("impl Drop for BoardFillJob").expect("live fill job drop");
        let index = source[..production_end].rfind(marker).expect("live scan-compatibility handback");
        let mut mutant = String::with_capacity(source.len());
        mutant.push_str(&source[..index]);
        mutant.push_str("if state.candidates.try_push_owned(candidate).is_err() {");
        mutant.push_str(&source[index + marker.len()..]);
        assert!(!fill_owned_page_handback_contract(&mutant));
    }
    // #endregion board_host
}

pub use crate::infinite::board::normal::undirected::{
    apply_force_graph_layout_to_fixture_v1_json as apply_undirected_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value as apply_undirected_force_graph_layout_to_fixture_v1_value,
    apply_redraw_layout_to_fixture_v1_json as apply_normal_undirected_redraw_layout_to_fixture_v1_json, ForceGraphLayoutOptions as UndirectedForceGraphLayoutOptions,
};
pub use crate::infinite::board::ports::directed::*;
pub use crate::infinite::canvas;
pub use board_host::*;
