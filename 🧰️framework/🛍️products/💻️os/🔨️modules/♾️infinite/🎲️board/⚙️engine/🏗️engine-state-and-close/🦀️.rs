//! 🏗️ Engine state and close of the board engine: the engine selection options, the graph engine struct, its backing retirement step, default and the close step. See [`super`].

use super::*;

// #region 🔖️Engine
/// 🎯️ Engine-local area-select options.
#[derive(Clone, Debug, PartialEq)]
pub struct EngineSelectionOptions {
    pub method: String,
    pub mode: String,
    pub select_nodes: bool,
    pub select_handles: bool,
    pub select_edges: bool,
}

impl Default for EngineSelectionOptions {
    fn default() -> Self {
        Self { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_handles: true, select_edges: true }
    }
}

/// ⚙️ Retained graph engine parameterized by port model and directedness.
#[derive(Clone, Debug)]
pub struct GraphEngine<P: GraphPortModel, D: Directedness> {
    pub camera: Camera,
    pub edges: HistoryFoldIndex<EdgeId, GraphEdge<P::Endpoint>>,
    pub edge_semantics: HistoryFoldIndex<EdgeId, ElementSemantics>,
    pub enforce_acyclic: bool,
    pub events: Vec<BoardEvent>,
    pub handles: HistoryFoldIndex<HandleId, Handle>,
    pub hover: Option<u64>,
    pub interaction: InteractionMode,
    pub nodes: HistoryFoldIndex<NodeId, Node>,
    pub selection: Selection,
    pub preselect: Selection,
    pub preselect_removed: Selection,
    pub selection_options: EngineSelectionOptions,
    pub handle_pointer_picking: bool,
    pub proximity_distance_world: f64,
    pub(super) proximity_distance_override: Option<f64>,
    pub selection_preview_points: Vec<Point>,
    pub selection_preview_crossing: bool,
    pub(super) area_initial: Selection,
    pub(super) area_points: Vec<Point>,
    pub(super) area_screen_points: Vec<Point>,
    pub(super) drag_start_positions: HistoryFoldIndex<NodeId, RetainedPoint>,
    pub(super) proximity_connection: Option<ProximityConnection>,
    pub(super) next_edge_id: u64,
    pub(super) retirement_backing_credited_bytes: usize,
    pub(super) _directedness: std::marker::PhantomData<D>,
    pub(super) _port: std::marker::PhantomData<P>,
}

/// 🧱️ One graph-backing retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphEngineBackingRetirementStep {
    Blocked,
    Pending { credited_bytes: usize, released_bytes: usize },
    Complete,
}

impl<P: GraphPortModel, D: Directedness> Default for GraphEngine<P, D> {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            edges: HistoryFoldIndex::new(),
            edge_semantics: HistoryFoldIndex::new(),
            enforce_acyclic: false,
            events: Vec::new(),
            handles: HistoryFoldIndex::new(),
            hover: None,
            interaction: InteractionMode::default(),
            nodes: HistoryFoldIndex::new(),
            selection: Selection::default(),
            preselect: Selection::default(),
            preselect_removed: Selection::default(),
            selection_options: EngineSelectionOptions::default(),
            handle_pointer_picking: true,
            proximity_distance_world: DEFAULT_PROXIMITY_DISTANCE_WORLD,
            proximity_distance_override: None,
            selection_preview_points: Vec::new(),
            selection_preview_crossing: false,
            area_initial: Selection::default(),
            area_points: Vec::new(),
            area_screen_points: Vec::new(),
            drag_start_positions: HistoryFoldIndex::new(),
            proximity_connection: None,
            next_edge_id: 1000,
            retirement_backing_credited_bytes: 0,
            _directedness: std::marker::PhantomData,
            _port: std::marker::PhantomData,
        }
    }
}

impl<P: GraphPortModel, D: Directedness> GraphEngine<P, D> {
    /// 🧹️ Releases one retained graph owner or scalar from the engine per call.
    pub fn close_step(&mut self) -> bool {
        if self.edges.pop_first().is_some()
            || self.edge_semantics.pop_first().is_some()
            || self.events.pop().is_some()
            || self.handles.pop_first().is_some()
            || self.nodes.pop_first().is_some()
            || self.selection.edge_ids.pop_first().is_some()
            || self.selection.handle_ids.pop_first().is_some()
            || self.selection.node_ids.pop_first().is_some()
            || self.preselect.edge_ids.pop_first().is_some()
            || self.preselect.handle_ids.pop_first().is_some()
            || self.preselect.node_ids.pop_first().is_some()
            || self.preselect_removed.edge_ids.pop_first().is_some()
            || self.preselect_removed.handle_ids.pop_first().is_some()
            || self.preselect_removed.node_ids.pop_first().is_some()
            || self.selection_preview_points.pop().is_some()
            || self.area_initial.edge_ids.pop_first().is_some()
            || self.area_initial.handle_ids.pop_first().is_some()
            || self.area_initial.node_ids.pop_first().is_some()
            || self.area_points.pop().is_some()
            || self.area_screen_points.pop().is_some()
            || self.drag_start_positions.pop_first().is_some()
            || self.selection_options.method.pop().is_some()
            || self.selection_options.mode.pop().is_some()
        {
            return false;
        }
        self.hover = None;
        self.interaction = InteractionMode::default();
        self.proximity_distance_override = None;
        self.proximity_connection = None;
        true
    }

    /// 🔍️ Proves every non-scalar graph owner governed by `close_step` is empty.
    pub fn terminal_is_empty(&self) -> bool {
        self.edges.is_empty()
            && self.edge_semantics.is_empty()
            && self.events.is_empty()
            && self.handles.is_empty()
            && self.nodes.is_empty()
            && self.selection.edge_ids.is_empty()
            && self.selection.handle_ids.is_empty()
            && self.selection.node_ids.is_empty()
            && self.preselect.edge_ids.is_empty()
            && self.preselect.handle_ids.is_empty()
            && self.preselect.node_ids.is_empty()
            && self.preselect_removed.edge_ids.is_empty()
            && self.preselect_removed.handle_ids.is_empty()
            && self.preselect_removed.node_ids.is_empty()
            && self.selection_preview_points.is_empty()
            && self.area_initial.edge_ids.is_empty()
            && self.area_initial.handle_ids.is_empty()
            && self.area_initial.node_ids.is_empty()
            && self.area_points.is_empty()
            && self.area_screen_points.is_empty()
            && self.drag_start_positions.is_empty()
            && self.selection_options.method.is_empty()
            && self.selection_options.mode.is_empty()
            && self.hover.is_none()
            && self.proximity_distance_override.is_none()
            && self.proximity_connection.is_none()
    }

    /// 🧹️ Releases one empty engine vector backing only when the caller grants its full retained capacity.
    pub fn close_backing_page(&mut self, maximum_bytes: usize) -> GraphEngineBackingRetirementStep {
        if !self.terminal_is_empty() {
            return GraphEngineBackingRetirementStep::Blocked;
        }
        macro_rules! retire_backing {
            ($field:expr, $element:ty) => {
                if $field.capacity() != 0 {
                    let released_bytes = $field.capacity().saturating_mul(std::mem::size_of::<$element>());
                    let remaining_bytes = released_bytes.saturating_sub(self.retirement_backing_credited_bytes);
                    let credited_bytes = maximum_bytes.min(remaining_bytes);
                    self.retirement_backing_credited_bytes = self.retirement_backing_credited_bytes.saturating_add(credited_bytes);
                    if self.retirement_backing_credited_bytes != released_bytes {
                        return GraphEngineBackingRetirementStep::Pending { credited_bytes, released_bytes: 0 };
                    }
                    drop(std::mem::take($field));
                    self.retirement_backing_credited_bytes = 0;
                    return GraphEngineBackingRetirementStep::Pending { credited_bytes, released_bytes };
                }
            };
        }
        retire_backing!(&mut self.events, BoardEvent);
        retire_backing!(&mut self.selection_preview_points, Point);
        retire_backing!(&mut self.area_points, Point);
        retire_backing!(&mut self.area_screen_points, Point);
        for selection in [&mut self.selection,&mut self.preselect,&mut self.preselect_removed,&mut self.area_initial]{
            for values in [&mut selection.edge_ids,&mut selection.handle_ids,&mut selection.node_ids]{
                if !values.terminal_is_empty(){let bytes=values.next_close_release_byte_demand().expect("finite numeric selection backing demand");let credited_bytes=maximum_bytes.min(bytes.saturating_sub(self.retirement_backing_credited_bytes));self.retirement_backing_credited_bytes+=credited_bytes;if self.retirement_backing_credited_bytes<bytes{return GraphEngineBackingRetirementStep::Pending{credited_bytes,released_bytes:0}}let progress=values.close_copy_step(1,self.retirement_backing_credited_bytes).expect("admitted numeric selection backing release");if progress.progressed{self.retirement_backing_credited_bytes=0;}return GraphEngineBackingRetirementStep::Pending{credited_bytes,released_bytes:progress.released_allocation_bytes};}
            }
        }
        debug_assert_eq!(self.retirement_backing_credited_bytes, 0);
        GraphEngineBackingRetirementStep::Complete
    }
}
// #endregion 🔖️Engine
