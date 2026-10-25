//! 🧮️ Flex tree: the `FlexTree` with its funded node and track tables, its measure and arrange steps, the flow style distribution and the owned wrapped and grid solvers. See [`super`].

use super::*;

// #region flex
//#region 🧮️FlexTree
type Flows = ui_contract::UiFixedList<FlowStyle, FLEX_NODE_CREDITS>;
type Sizes = ui_contract::UiFixedList<(f32, f32), FLEX_NODE_CREDITS>;
type Rects = ui_contract::UiFixedList<FlexRect, FLEX_NODE_CREDITS>;
type Tracks = ui_contract::UiFixedList<TrackSize, FLEX_TRACK_CREDITS>;
type Measure<'a> = &'a mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32);

/// 🛤️ One grid track while its axis is being sized: the track function, what the span-1 items under it contribute, and the
/// base size, growth limit and offset the sizing steps move.
#[derive(Clone, Copy)]
pub(super) struct TrackSize {
    kind: GridTrack,
    minimum: f32,
    max_content: f32,
    min_content: f32,
    base: f32,
    limit: f32,
    increase: f32,
    offset: f32,
}

impl TrackSize {
    fn new(kind: GridTrack) -> Self {
        Self { kind, minimum: 0.0, max_content: 0.0, min_content: 0.0, base: 0.0, limit: 0.0, increase: 0.0, offset: 0.0 }
    }

    fn fraction(&self) -> Option<f32> {
        match self.kind {
            GridTrack::Fraction(count) => Some(f32::from(count)),
            _ => None,
        }
    }
}

/// 🌳️ Owns one bounded layout pass's styles and its solved boxes.
///
/// **The pass is per CONTAINER, never global, and every container is resolved arithmetically.** A single
/// whole-tree solve is not resumable and starves every other interactive job on the lane (`engine`'s own
/// eight-millisecond slice law). So layout runs the way CSS itself resolves a box — a bottom-up
/// intrinsic (max-content) measurement, then a top-down arrange — and each grant handles exactly ONE
/// container against its own already-resolved box: single-line flex, wrapped multi-line flex and grid track
/// sizing are all pure arithmetic over the container's direct children. Taffy remains the third-party
/// oracle every one of the three solvers is validated against in `🧪️tests`, and no taffy value exists in the
/// shipped engine.
///
/// **Every byte a step touches is funded.** The four node tables and the track table are paged
/// [`ui_contract::UiFixedList`]s whose pages are admitted one per turn through the caller's grant
/// ([`Self::fund_next_node`], [`Self::fund_scratch`]); a step never allocates: the children arrive as a cloneable
/// sibling iterator, the single-line and wrapped solvers park each child's base size in that child's own
/// not-yet-solved `resolved` slot, and the grid solver sizes its tracks inside the funded track table. Close
/// releases the table pages one owner per grant.
pub struct FlexTree {
    pub(super) flows: Flows,
    pub(super) grids: ui_contract::UiFixedList<Option<Box<ui_contract::GridLayout>>, FLEX_NODE_CREDITS>,
    pub(super) intrinsic: Sizes,
    pub(super) resolved: Rects,
    pub(super) tracks: Tracks,
}

/// 🎟️ What one funding turn for the next node's table slots achieved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeFunding {
    Ready,
    Funded(semio_framework_value::retained_clone::RetainedCloneProgress),
    Denied,
}

impl Default for FlexTree {
    fn default() -> Self {
        Self::new()
    }
}

/// 👶️ The in-flow children of one container, in document order: an out-of-flow child takes no cell, line or share.
fn in_flow<'a>(flows: &'a Flows, children: impl Iterator<Item = usize> + Clone + 'a) -> impl Iterator<Item = usize> + Clone + 'a {
    children.filter(move |&child| flows.get(child).is_some_and(|flow| !flow.absolute))
}

/// 🅿️ Parks a child's main and cross base size in its own `resolved` slot until its final box replaces them.
fn park(resolved: &mut Rects, child: usize, (main, cross): (f32, f32)) -> Option<()> {
    *resolved.get_mut(child)? = FlexRect { x: 0.0, y: 0.0, width: main, height: cross };
    Some(())
}

fn parked(resolved: &Rects, child: usize) -> Option<(f32, f32)> {
    resolved.get(child).map(|slot| (slot.width, slot.height))
}

/// ✖️ The cross-axis [`Dim`] of a child inside a `row` or column container.
fn cross_dim(child: FlowStyle, row: bool) -> Dim {
    if row { child.height } else { child.width }
}

/// 📖️ One text run's wrapped height at a definite outer `width`, padding included.
fn text_height(flow: FlowStyle, node: usize, width: f32, measure: Measure<'_>) -> f32 {
    measure(node, MeasureConstraint::Definite((width - flow.padding.left - flow.padding.right).max(0.0))).1 + flow.padding.top + flow.padding.bottom
}

impl FlexTree {
    pub fn new() -> Self {
        Self { flows: Default::default(), grids: Default::default(), intrinsic: Default::default(), resolved: Default::default(), tracks: Default::default() }
    }

    pub fn len(&self) -> usize {
        self.flows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.flows.is_empty()
    }

    /// ⏭️ Admits at most one backing page of the four node tables under the caller's grant, first table first, so that [`Self::push`] never allocates a table page on its own.
    pub fn fund_next_node(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<NodeFunding, ui_contract::PagedListAllocationError> {
        macro_rules! fund {($table:ident)=>{if !self.$table.has_reserved_slot(){let progress=self.$table.try_reserve_one_funded(grant)?;return Ok(if progress.copied_items==0{NodeFunding::Denied}else{NodeFunding::Funded(progress)})}}}
        fund!(flows);fund!(grids);fund!(intrinsic);fund!(resolved);
        Ok(NodeFunding::Ready)
    }

    /// 👣️ How many track slots node `index` needs while its measure or arrange step runs: a grid's explicit and implicit columns and rows, nothing for every other container.
    fn scratch_demand(&self, index: usize, children: impl Iterator<Item = usize> + Clone) -> usize {
        let Some(FlowStyle { kind: FlowKind::Grid, .. }) = self.flows.get(index) else { return 0 };
        let grid = self.grids.get(index).and_then(|grid| grid.as_deref());
        let columns = grid.map_or(0, |grid| grid.columns.len()).max(1);
        columns + grid.map_or(0, |grid| grid.rows.len()).max(in_flow(&self.flows, children).count().div_ceil(columns))
    }

    /// 📃️ Admits at most one backing page of the track table under the caller's grant, until it holds every slot node `index`'s grid sizing needs, so that [`Self::measure_one`] and [`Self::arrange_one`] never allocate.
    pub fn fund_scratch(&mut self, index: usize, children: impl Iterator<Item = usize> + Clone, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<NodeFunding, ui_contract::PagedListAllocationError> {
        let demand = self.scratch_demand(index, children);
        if demand <= self.tracks.capacity() || demand > FLEX_TRACK_CREDITS {
            return Ok(NodeFunding::Ready);
        }
        let progress = self.tracks.try_reserve_capacity_one_funded(demand, grant)?;
        Ok(if progress.copied_items == 0 { NodeFunding::Denied } else { NodeFunding::Funded(progress) })
    }

    /// 🌱️ Admits the job's next node (nodes arrive in the admission walk's depth-first preorder) into slots
    /// already funded by [`Self::fund_next_node`] and returns the exact heap bytes the node itself owns.
    /// Only a `Grid` retains its authored track lists — everything else layout needs lives in the
    /// [`FlowStyle`], so the common node costs one small `Copy` record and no allocation at all.
    pub fn push(&mut self, kind: LayoutNodeKind, parent: Option<usize>, authored: Option<&LayoutSpec>, metrics: &TreeRowMetrics, parent_kind: Option<LayoutNodeKind>, content: Option<TreeContentFlow>) -> Option<usize> {
        let mut flow = match content {
            Some(content) => tree_content_flow(content, flow_for(kind, None, authored, metrics), metrics),
            None => flow_for(kind, parent_kind, authored, metrics),
        };
        if parent.is_some_and(|parent| self.flows.get(parent).is_some_and(|owner| owner.grows_children)) && !flow.absolute {
            flow.grow = 1.0;
        }
        flow.grows_children = content.is_none() && grows_children(kind, authored.is_some());
        let grid = match authored {
            Some(LayoutSpec::Grid(tracks)) if matches!(flow.kind, FlowKind::Grid) => Some(Box::new(tracks.clone())),
            _ => None,
        };
        let owned = grid.as_deref().map_or(0, |grid| std::mem::size_of::<ui_contract::GridLayout>() + grid.columns.allocated_bytes() + grid.rows.allocated_bytes());
        let admitted = self.flows.try_push_reserved(flow).is_ok() && self.grids.try_push_reserved(grid).is_ok() && self.intrinsic.try_push_reserved((0.0, 0.0)).is_ok() && self.resolved.try_push_reserved(FlexRect::default()).is_ok();
        admitted.then_some(owned)
    }

    /// 🪟️ Seeds the root's own box — the viewport it is laid out against.
    pub fn set_root_box(&mut self, index: usize, rect: FlexRect) -> bool {
        match self.resolved.get_mut(index) {
            Some(slot) => {
                *slot = rect;
                true
            }
            None => false,
        }
    }

    /// 📐️ The solved, parent-relative box for the job's node `index`.
    pub fn rect(&self, index: usize) -> Option<FlexRect> {
        self.resolved.get(index).copied()
    }

    pub fn intrinsic(&self, index: usize) -> Option<(f32, f32)> {
        self.intrinsic.get(index).copied()
    }

    pub fn set_composite_chrome(&mut self, index: usize, top: f32, bottom: f32) -> bool {
        let Some(flow) = self.flows.get_mut(index) else { return false };
        flow.padding.top = top.max(0.0);
        flow.padding.bottom = bottom.max(0.0);
        true
    }

    /// 🗂️ Node `index`'s intrinsic (max-content) size, from its `children`'s already-measured intrinsic
    /// sizes — the bottom-up half. `children` must be in document order, and every one of them must be
    /// measured already (reverse admission order guarantees it). A `Wrapped` container measures at max-content,
    /// where nothing wraps, so it measures exactly like a single line.
    pub fn measure_one(&mut self, index: usize, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let Some(flow) = self.flows.get(index).copied() else { return false };
        if matches!(flow.kind, FlowKind::Grid) {
            return self.measure_grid(index, flow, children, measure);
        }
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (mut main_sum, mut cross_max, mut count) = (0.0_f32, 0.0_f32, 0_usize);
        for child in children {
            let Some(child_flow) = self.flows.get(child).copied() else { return false };
            if child_flow.absolute {
                continue;
            }
            let Some(&(width, height)) = self.intrinsic.get(child) else { return false };
            let (main, cross) = if flow.row { (width, height) } else { (height, width) };
            main_sum += main + child_flow.lead;
            cross_max = cross_max.max(cross);
            count += 1;
        }
        let mut main = main_sum + flow.gap_main * count.saturating_sub(1) as f32 + main_start + main_end;
        let mut cross = cross_max + cross_start + cross_end;
        if flow.text {
            let (width, height) = measure(index, MeasureConstraint::MaxContent);
            let (own_main, own_cross) = if flow.row { (width, height) } else { (height, width) };
            main = main.max(own_main + main_start + main_end);
            cross = cross.max(own_cross + cross_start + cross_end);
        }
        let (width, height) = if flow.row { (main, cross) } else { (cross, main) };
        self.finish_measure(index, flow, (width, height))
    }

    /// 📏️ Applies the container's own sizing to its content-derived `(width, height)` and records it as its intrinsic size.
    fn finish_measure(&mut self, index: usize, flow: FlowStyle, (mut width, mut height): (f32, f32)) -> bool {
        if let Dim::Length(value) = flow.width {
            width = value;
        }
        if let Dim::Length(value) = flow.height {
            height = value;
        }
        match self.intrinsic.get_mut(index) {
            Some(slot) => *slot = (width.max(flow.min_width), height.max(flow.min_height)),
            None => return false,
        }
        true
    }

    /// 📦️ Arranges `children` inside node `index`'s own resolved box — the top-down half. Each child's
    /// parent-relative rect is written into this tree, so a later [`Self::rect`] reads it back.
    pub fn arrange_one(&mut self, index: usize, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let (Some(flow), Some(&owner)) = (self.flows.get(index).copied(), self.resolved.get(index)) else { return false };
        if children.clone().next().is_none() {
            return true;
        }
        match flow.kind {
            FlowKind::Flow => self.arrange_flow(flow, owner, children, measure),
            FlowKind::Wrapped => self.arrange_wrapped(flow, owner, children, measure),
            FlowKind::Grid => self.arrange_grid(index, flow, owner, children, measure),
        }
    }

    /// 🔂️ The single-line arrange: base sizes, one grow or shrink distribution, justify distribution, per-child cross alignment, out-of-flow placement.
    fn arrange_flow(&mut self, flow: FlowStyle, owner: FlexRect, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (outer_main, outer_cross) = if flow.row { (owner.width, owner.height) } else { (owner.height, owner.width) };
        let content_main = (outer_main - main_start - main_end).max(0.0);
        let content_cross = (outer_cross - cross_start - cross_end).max(0.0);
        let stretch = matches!(flow.align, Align::Stretch);
        let Self { flows, intrinsic, resolved, .. } = self;

        let (mut base_sum, mut grow_sum, mut shrink_sum, mut count) = (0.0_f32, 0.0_f32, 0.0_f32, 0_usize);
        for child in children.clone() {
            let (Some(child_flow), Some(&size)) = (flows.get(child).copied(), intrinsic.get(child)) else { return false };
            let base = if child_flow.absolute {
                (0.0, 0.0)
            } else {
                let (main, cross) = child_flow.flow_size(flow, content_main, content_cross, size, child, stretch, measure);
                base_sum += main + child_flow.lead;
                grow_sum += child_flow.grow;
                shrink_sum += child_flow.shrink * main;
                count += 1;
                (main, cross)
            };
            if park(resolved, child, base).is_none() {
                return false;
            }
        }

        let gaps = flow.gap_main * count.saturating_sub(1) as f32;
        let free = content_main - base_sum - gaps;
        if free > 0.0 && grow_sum > 0.0 {
            for child in children.clone() {
                let (Some(child_flow), Some((main, cross))) = (flows.get(child).copied(), parked(resolved, child)) else { return false };
                if !child_flow.absolute && park(resolved, child, (main + free * child_flow.grow / grow_sum, cross)).is_none() {
                    return false;
                }
            }
        } else if free < 0.0 && shrink_sum > 0.0 {
            for child in children.clone() {
                let (Some(child_flow), Some((main, cross))) = (flows.get(child).copied(), parked(resolved, child)) else { return false };
                if !child_flow.absolute && park(resolved, child, ((main + free * (child_flow.shrink * main) / shrink_sum).max(0.0), cross)).is_none() {
                    return false;
                }
            }
        }

        let placed: f32 = children.clone().map(|child| parked(resolved, child).map_or(0.0, |(main, _)| main)).sum();
        let remaining = (content_main - placed - gaps).max(0.0);
        let (lead, between) = distribute(flow.justify, remaining, count);
        let mut cursor = if flow.reverse { outer_main - main_end - lead } else { main_start + lead };
        for child in children {
            let (Some(child_flow), Some((main, cross)), Some(&size)) = (flows.get(child).copied(), parked(resolved, child), intrinsic.get(child)) else { return false };
            let rect = if child_flow.absolute {
                child_flow.absolute_rect(owner, size)
            } else {
                let offset = match flow.align {
                    Align::Center => (content_cross - cross) * 0.5,
                    Align::End => content_cross - cross,
                    Align::Start | Align::Stretch | Align::Baseline => 0.0,
                };
                if flow.reverse {
                    cursor -= main + child_flow.lead;
                } else {
                    cursor += child_flow.lead;
                }
                let (x, y, width, height) = if flow.row { (cursor, cross_start + offset, main, cross) } else { (cross_start + offset, cursor, cross, main) };
                if flow.reverse {
                    cursor -= flow.gap_main + between;
                } else {
                    cursor += main + flow.gap_main + between;
                }
                FlexRect { x, y, width, height }
            };
            match resolved.get_mut(child) {
                Some(slot) => *slot = rect,
                None => return false,
            }
        }
        true
    }

    /// 🎞️ The multi-line arrange: children break into lines where the next base size no longer fits the container's main extent
    /// (the first child of a line always fits), each line distributes its own free space, the lines stack along the cross axis and
    /// share any spare cross space equally (`align-content: stretch`), and a child's stretch is its line's cross size.
    fn arrange_wrapped(&mut self, flow: FlowStyle, owner: FlexRect, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (outer_main, outer_cross) = if flow.row { (owner.width, owner.height) } else { (owner.height, owner.width) };
        let content_main = (outer_main - (main_start + main_end)).max(0.0);
        let content_cross = (outer_cross - (cross_start + cross_end)).max(0.0);
        let stretch = matches!(flow.align, Align::Stretch);
        let Self { flows, intrinsic, resolved, .. } = self;

        for child in children.clone() {
            let (Some(child_flow), Some(&size)) = (flows.get(child).copied(), intrinsic.get(child)) else { return false };
            let base = if child_flow.absolute { (0.0, 0.0) } else { child_flow.flow_size(flow, content_main, content_cross, size, child, false, measure) };
            if park(resolved, child, base).is_none() {
                return false;
            }
        }

        let items = in_flow(flows, children.clone());
        let (mut lines, mut cross_sum, mut rest) = (0_usize, 0.0_f32, items.clone());
        while let Some(line) = take_line(flows, resolved, &mut rest, content_main, flow.gap_main) {
            lines += 1;
            cross_sum += line.cross;
        }
        let spare = content_cross - cross_sum - flow.gap_cross * lines.saturating_sub(1) as f32;
        let share = if lines > 0 && spare > 0.0 { spare / lines as f32 } else { 0.0 };

        let (mut rest, mut line_start) = (items, cross_start);
        loop {
            let first = rest.clone();
            let Some(line) = take_line(flows, resolved, &mut rest, content_main, flow.gap_main) else { break };
            let gaps = flow.gap_main * line.count.saturating_sub(1) as f32;
            let free = content_main - line.used;
            if free > 0.0 && line.grow > 0.0 {
                for child in first.clone().take(line.count) {
                    let (Some(child_flow), Some((main, cross))) = (flows.get(child).copied(), parked(resolved, child)) else { return false };
                    if park(resolved, child, (main + free * child_flow.grow / line.grow, cross)).is_none() {
                        return false;
                    }
                }
            } else if free < 0.0 && line.shrink > 0.0 {
                for child in first.clone().take(line.count) {
                    let (Some(child_flow), Some((main, cross))) = (flows.get(child).copied(), parked(resolved, child)) else { return false };
                    if park(resolved, child, ((main + free * (child_flow.shrink * main) / line.shrink).max(0.0), cross)).is_none() {
                        return false;
                    }
                }
            }
            let placed: f32 = first.clone().take(line.count).map(|child| parked(resolved, child).map_or(0.0, |(main, _)| main)).sum();
            let (lead, between) = distribute(flow.justify, (content_main - placed - gaps).max(0.0), line.count);
            let line_cross = line.cross + share;
            let mut cursor = if flow.reverse { outer_main - main_end - lead } else { main_start + lead };
            for child in first.take(line.count) {
                let (Some(child_flow), Some((main, hypothetical))) = (flows.get(child).copied(), parked(resolved, child)) else { return false };
                let min_cross = if flow.row { child_flow.min_height } else { child_flow.min_width };
                let cross = if stretch && matches!(cross_dim(child_flow, flow.row), Dim::Auto) { line_cross.max(min_cross) } else { hypothetical };
                let offset = match flow.align {
                    Align::Center => (line_cross - cross) * 0.5,
                    Align::End => line_cross - cross,
                    Align::Start | Align::Stretch | Align::Baseline => 0.0,
                };
                if flow.reverse {
                    cursor -= main + child_flow.lead;
                } else {
                    cursor += child_flow.lead;
                }
                let (x, y, width, height) = if flow.row { (cursor, line_start + offset, main, cross) } else { (line_start + offset, cursor, cross, main) };
                if flow.reverse {
                    cursor -= flow.gap_main + between;
                } else {
                    cursor += main + flow.gap_main + between;
                }
                match resolved.get_mut(child) {
                    Some(slot) => *slot = FlexRect { x, y, width, height },
                    None => return false,
                }
            }
            line_start += line_cross + flow.gap_cross;
        }
        for child in children {
            let (Some(child_flow), Some(&size)) = (flows.get(child).copied(), intrinsic.get(child)) else { return false };
            if child_flow.absolute {
                match resolved.get_mut(child) {
                    Some(slot) => *slot = child_flow.absolute_rect(owner, size),
                    None => return false,
                }
            }
        }
        true
    }

    /// 📊️ The `Grid` intrinsic measurement: columns then rows sized at max-content, plus the container's own padding.
    fn measure_grid(&mut self, index: usize, flow: FlowStyle, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let Self { flows, grids, intrinsic, tracks, .. } = self;
        let grid = grids.get(index).and_then(|grid| grid.as_deref());
        let solved = solve_tracks(flows, intrinsic, tracks, grid, flow, children, (None, None), measure);
        let extent = solved.and_then(|(columns, rows)| Some((axis_extent(tracks, 0, columns, flow.gap_cross)?, axis_extent(tracks, columns, rows, flow.gap_main)?)));
        clear_tracks(tracks);
        let Some((columns, rows)) = extent else { return false };
        self.finish_measure(index, flow, (columns + flow.padding.left + flow.padding.right, rows + flow.padding.top + flow.padding.bottom))
    }

    /// 🏠️ The `Grid` arrange: tracks sized against the container's content box, aligned (columns by `justify`, rows packed and stretched), then every in-flow child placed into its row-major cell.
    fn arrange_grid(&mut self, index: usize, flow: FlowStyle, owner: FlexRect, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> bool {
        let Self { flows, grids, intrinsic, resolved, tracks } = self;
        let grid = grids.get(index).and_then(|grid| grid.as_deref());
        let placed = place_grid(flows, grid, intrinsic, resolved, tracks, flow, owner, children, measure);
        clear_tracks(tracks);
        placed.is_some()
    }

    /// 🧹️ Releases exactly ONE owner per grant, newest first: a grid's own track pages and box, then one logical
    /// node (one row of every node table), then one table page — the same
    /// one-owner-per-grant close discipline every other bounded list in `mounted_layout` follows.
    pub fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if let Some(Some(grid))=self.grids.last(){
            if !grid.columns.is_empty(){return Ok(0)}
            if !grid.columns.terminal_is_empty(){return grid.columns.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)}
            if !grid.rows.is_empty(){return Ok(0)}
            if !grid.rows.terminal_is_empty(){return grid.rows.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)}
            return Ok(size_of::<ui_contract::GridLayout>())
        }
        if self.nodes_remain(){return Ok(0)}
        macro_rules! table{($($table:ident),+)=>{$(if !self.$table.terminal_is_empty(){return self.$table.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)})+}}
        table!(flows,grids,intrinsic,resolved,tracks);
        Ok(0)
    }
    fn nodes_remain(&self)->bool{!self.flows.is_empty()||!self.grids.is_empty()||!self.intrinsic.is_empty()||!self.resolved.is_empty()||!self.tracks.is_empty()}
    pub fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        let overflow=||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"flex close native parent depth overflow");
        if let Some(Some(grid))=self.grids.last(){
            let depth=if !grid.columns.is_empty(){grid.columns.next_pop_depth_demand()}else if !grid.columns.terminal_is_empty(){grid.columns.next_release_depth_demand()}else if !grid.rows.is_empty(){grid.rows.next_pop_depth_demand()}else if !grid.rows.terminal_is_empty(){grid.rows.next_release_depth_demand()}else{return Ok(1)}.map_err(semio_framework_value::ValueError::from)?;
            return depth.checked_add(1).ok_or_else(overflow)
        }
        if self.nodes_remain(){
            let mut depth=0;
            macro_rules! pop{($($table:ident),+)=>{$(if !self.$table.is_empty(){depth=depth.max(self.$table.next_pop_depth_demand().map_err(semio_framework_value::ValueError::from)?)})+}}
            pop!(flows,grids,intrinsic,resolved,tracks);
            return depth.checked_add(1).ok_or_else(overflow)
        }
        macro_rules! release{($($table:ident),+)=>{$(if !self.$table.terminal_is_empty(){return self.$table.next_release_depth_demand().map_err(semio_framework_value::ValueError::from)?.checked_add(1).ok_or_else(overflow)})+}}
        release!(flows,grids,intrinsic,resolved,tracks);
        Ok(usize::from(!self.close_is_empty()))
    }
    pub fn close_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();if self.close_is_empty(){return Ok(Step::Complete{progress:empty})}
        if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let bytes=self.next_close_release_byte_demand()?;
        let depth=self.next_close_depth_demand()?;if bytes>grant.maximum_release_bytes||grant.maximum_depth<depth{return Ok(Step::Pending{progress:empty})}
        let pending=|progress:RetainedCloneProgress|Ok(Step::Pending{progress});
        if let Some(index)=self.grids.len().checked_sub(1){
            if let Some(Some(grid))=self.grids.get_mut(index){
                let result=if grid.columns.pop().is_some(){None}else if !grid.columns.terminal_is_empty(){Some(grid.columns.release_empty_page(grant.maximum_release_bytes))}else if grid.rows.pop().is_some(){None}else if !grid.rows.terminal_is_empty(){Some(grid.rows.release_empty_page(grant.maximum_release_bytes))}else{*self.grids.get_mut(index).unwrap()=None;return pending(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})};
                return match result{Some(Ok(receipt))=>pending(RetainedCloneProgress{copied_items:usize::from(receipt.progressed),released_bytes:receipt.released_allocation_bytes,..Default::default()}),Some(Err(error))=>Err(semio_framework_value::ValueError::from(error)),None=>pending(RetainedCloneProgress{copied_items:1,..Default::default()})};
            }
        }
        if self.nodes_remain(){self.flows.pop();self.grids.pop();self.intrinsic.pop();self.resolved.pop();self.tracks.pop();return pending(RetainedCloneProgress{copied_items:1,..Default::default()})}
        macro_rules! table{($($table:ident),+)=>{$(if !self.$table.terminal_is_empty(){return match self.$table.release_empty_page(grant.maximum_release_bytes){Ok(receipt)=>pending(RetainedCloneProgress{copied_items:usize::from(receipt.progressed),released_bytes:receipt.released_allocation_bytes,..Default::default()}),Err(error)=>Err(semio_framework_value::ValueError::from(error))}})+}}
        table!(flows,grids,intrinsic,resolved,tracks);
        Ok(Step::Complete{progress:empty})
    }
    pub fn close_is_empty(&self)->bool{self.flows.terminal_is_empty()&&self.grids.terminal_is_empty()&&self.intrinsic.terminal_is_empty()&&self.resolved.terminal_is_empty()&&self.tracks.terminal_is_empty()}
}

/// 📜️ One line of a wrapped container: how many children it holds, the main extent they use together with their gaps, its cross extent
/// and the grow and shrink weights its free space is distributed by.
#[derive(Clone, Copy, Default)]
struct FlexLine {
    count: usize,
    used: f32,
    cross: f32,
    grow: f32,
    shrink: f32,
}

/// ✂️ Takes the next line off `rest`: children join while the running main extent (base sizes, leads and gaps) still fits `content_main`,
/// and the first child of a line always joins. `None` once `rest` is exhausted.
fn take_line<I: Iterator<Item = usize> + Clone>(flows: &Flows, resolved: &Rects, rest: &mut I, content_main: f32, gap: f32) -> Option<FlexLine> {
    let mut line = FlexLine::default();
    loop {
        let mut probe = rest.clone();
        let Some(child) = probe.next() else { break };
        let (Some(child_flow), Some((main, cross))) = (flows.get(child), parked(resolved, child)) else { break };
        let length = line.used + (main + child_flow.lead + if line.count == 0 { 0.0 } else { gap });
        if length > content_main && line.count != 0 {
            break;
        }
        line = FlexLine { count: line.count + 1, used: length, cross: line.cross.max(cross), grow: line.grow + child_flow.grow, shrink: line.shrink + child_flow.shrink * main };
        *rest = probe;
    }
    (line.count > 0).then_some(line)
}

fn clear_tracks(tracks: &mut Tracks) {
    while tracks.pop().is_some() {}
}

/// 🧱️ What a span-1 grid item contributes to its column: `(base, min-content, max-content)` — `base` is what an `auto` or `fr`
/// track grows its base size to (the minimum contribution, or the min-content contribution when the grid is measured and
/// the item is no scroll container). A `Fill` size is a percentage of the grid container's inner size (`available`), content-sized while that is unknown.
fn contribute_column(flow: FlowStyle, intrinsic: (f32, f32), node: usize, available: Option<f32>, measure: Measure<'_>) -> (f32, f32, f32) {
    let (min_content, max_content) = match (flow.width, available) {
        (Dim::Length(value), _) => (value, value),
        (Dim::Fill, Some(inner)) => (inner, inner),
        _ if flow.text => (measure(node, MeasureConstraint::MinContent).0 + flow.padding.left + flow.padding.right, intrinsic.0),
        _ => (intrinsic.0, intrinsic.0),
    };
    let (min_content, max_content) = (min_content.max(flow.min_width), max_content.max(flow.min_width));
    let minimum = match flow.width {
        Dim::Length(value) => value,
        Dim::Fill => available.unwrap_or(flow.min_width),
        Dim::Auto => flow.min_width,
    };
    (if available.is_none() && !flow.clips { min_content.max(minimum) } else { minimum }, min_content, max_content)
}

/// 🪞️ The row-axis twin of [`contribute_column`], measured at the `width` the item gets from its already sized column.
fn contribute_row(flow: FlowStyle, intrinsic: (f32, f32), node: usize, width: f32, available: Option<f32>, measure: Measure<'_>) -> (f32, f32, f32) {
    let content = if flow.text { text_height(flow, node, width, measure) } else { intrinsic.1 };
    let (min_content, max_content) = match (flow.height, available) {
        (Dim::Length(value), _) => (value, value),
        (Dim::Fill, Some(inner)) => (inner, inner),
        _ => (content, content),
    };
    let (min_content, max_content) = (min_content.max(flow.min_height), max_content.max(flow.min_height));
    let minimum = match flow.height {
        Dim::Length(value) => value,
        Dim::Fill => available.unwrap_or(flow.min_height),
        Dim::Auto => flow.min_height,
    };
    (if available.is_none() && !flow.clips { min_content.max(minimum) } else { minimum }, min_content, max_content)
}

/// 🪜️ A grid item's width inside a column of `area` width: its own length, else the column (`justify-items: stretch`).
fn grid_width(flow: FlowStyle, area: f32) -> f32 {
    match flow.width {
        Dim::Length(value) => value,
        _ => area,
    }
    .max(flow.min_width.max(flow.padding.left + flow.padding.right))
}

/// 🧩️ Sizes the columns, then — against those widths — the rows of one grid into the funded `tracks` (columns first). `available` is the
/// container's content box per axis, `None` when the grid is measured at max-content. Returns `(columns, rows)`.
#[allow(clippy::too_many_arguments)]
fn solve_tracks(flows: &Flows, intrinsic: &Sizes, tracks: &mut Tracks, grid: Option<&ui_contract::GridLayout>, flow: FlowStyle, children: impl Iterator<Item = usize> + Clone, available: (Option<f32>, Option<f32>), measure: Measure<'_>) -> Option<(usize, usize)> {
    let items = in_flow(flows, children);
    let columns = grid.map_or(0, |grid| grid.columns.len()).max(1);
    let rows = grid.map_or(0, |grid| grid.rows.len()).max(items.clone().count().div_ceil(columns));
    if !tracks.is_empty() || tracks.capacity() < columns + rows {
        return None;
    }
    for column in 0..columns {
        tracks.try_push_reserved(TrackSize::new(grid.and_then(|grid| grid.columns.get(column).copied()).unwrap_or(GridTrack::Auto))).ok()?;
    }
    for row in 0..rows {
        tracks.try_push_reserved(TrackSize::new(grid.and_then(|grid| grid.rows.get(row).copied()).unwrap_or(GridTrack::Auto))).ok()?;
    }
    let raise = |track: &mut TrackSize, (minimum, min_content, max_content): (f32, f32, f32)| {
        track.minimum = track.minimum.max(minimum);
        track.min_content = track.min_content.max(min_content);
        track.max_content = track.max_content.max(max_content);
    };
    for (cell, child) in items.clone().enumerate() {
        let (child_flow, size) = (flows.get(child).copied()?, intrinsic.get(child).copied()?);
        raise(tracks.get_mut(cell % columns)?, contribute_column(child_flow, size, child, available.0, measure));
    }
    size_axis(tracks, 0, columns, available.0, flow.gap_cross, flow.min_width, false)?;
    for (cell, child) in items.enumerate() {
        let (child_flow, size, area) = (flows.get(child).copied()?, intrinsic.get(child).copied()?, tracks.get(cell % columns)?.base);
        raise(tracks.get_mut(columns + cell / columns)?, contribute_row(child_flow, size, child, grid_width(child_flow, area), available.1, measure));
    }
    size_axis(tracks, columns, rows, available.1, flow.gap_main, flow.min_height, true)?;
    Some((columns, rows))
}

/// ➕️ The sum of `count` sized tracks and the gaps between them.
fn axis_extent(tracks: &Tracks, from: usize, count: usize, gap: f32) -> Option<f32> {
    let mut sum = gap * count.saturating_sub(1) as f32;
    for index in from..from + count {
        sum += tracks.get(index)?.base;
    }
    Some(sum)
}

/// 🎚️ One axis of the CSS grid track sizing algorithm for span-1 items: initial base sizes and growth limits from the track
/// functions and what the items contribute, maximise (up to the growth limits), expand the `fr` tracks, stretch the `auto`
/// tracks. `available` is the content box along the axis (`None` at max-content).
fn size_axis(tracks: &mut Tracks, from: usize, count: usize, available: Option<f32>, gap: f32, container_minimum: f32, stretch_auto: bool) -> Option<()> {
    let gaps = gap * count.saturating_sub(1) as f32;
    let measuring = available.is_none();
    for index in from..from + count {
        let track = tracks.get_mut(index)?;
        let (base, limit) = match track.kind {
            GridTrack::Fixed(token) => (token.px(), token.px()),
            GridTrack::MinContent => (track.min_content, track.min_content),
            GridTrack::MaxContent => (track.max_content, track.max_content),
            GridTrack::Auto => (track.minimum, track.max_content.max(track.minimum)),
            GridTrack::Fraction(_) => {
                let base = if measuring { track.minimum.max(track.max_content) } else { track.minimum };
                (base, base)
            }
        };
        (track.base, track.limit, track.increase) = (base, limit, 0.0);
    }
    match available {
        None => {
            for index in from..from + count {
                let track = tracks.get_mut(index)?;
                track.base = track.limit;
            }
        }
        Some(space) => {
            let free = space - axis_extent(tracks, from, count, gap)?;
            if free > 0.0 {
                fill_to_limits(tracks, from, count, free)?;
            }
        }
    }
    let fraction = match available {
        Some(space) if space - axis_extent(tracks, from, count, gap)? <= 0.0 => 0.0,
        Some(space) => find_fraction(tracks, from, count, gaps, space)?,
        None => {
            let mut fraction = 0.0_f32;
            for index in from..from + count {
                let track = tracks.get(index)?;
                if let Some(flex) = track.fraction() {
                    let from_base = if flex > 1.0 { track.base / flex } else { track.base };
                    fraction = fraction.max(from_base).max(find_fraction(tracks, index, 1, 0.0, track.max_content)?);
                }
            }
            let mut hypothetical = gaps;
            for index in from..from + count {
                let track = tracks.get(index)?;
                hypothetical += track.fraction().map_or(track.base, |flex| track.base.max(flex * fraction));
            }
            if hypothetical < container_minimum { find_fraction(tracks, from, count, gaps, container_minimum)? } else { fraction }
        }
    };
    for index in from..from + count {
        let track = tracks.get_mut(index)?;
        if let Some(flex) = track.fraction() {
            track.base = track.base.max(flex * fraction);
        }
    }
    if stretch_auto {
        let mut autos = 0_usize;
        for index in from..from + count {
            autos += usize::from(matches!(tracks.get(index)?.kind, GridTrack::Auto));
        }
        let used = axis_extent(tracks, from, count, gap)?;
        let free = available.unwrap_or(container_minimum) - used;
        if autos > 0 && free > 0.0 {
            for index in from..from + count {
                let track = tracks.get_mut(index)?;
                if matches!(track.kind, GridTrack::Auto) {
                    track.base += free / autos as f32;
                }
            }
        }
    }
    Some(())
}

/// 💧️ CSS grid "maximise tracks": hands `space` out equally to the tracks that are still below their growth limit, never past it.
fn fill_to_limits(tracks: &mut Tracks, from: usize, count: usize, mut space: f32) -> Option<()> {
    const THRESHOLD: f32 = 0.01;
    while space > THRESHOLD {
        let (mut growable, mut least) = (0.0_f32, f32::INFINITY);
        for index in from..from + count {
            let track = tracks.get(index)?;
            if track.base + track.increase < track.limit {
                growable += 1.0;
                least = least.min(track.limit - track.base);
            }
        }
        if growable == 0.0 {
            break;
        }
        let step = least.min(space / growable);
        for index in from..from + count {
            let track = tracks.get_mut(index)?;
            if step > 0.0 && track.base + step <= track.limit + THRESHOLD {
                track.increase += step;
                space -= step;
            }
        }
    }
    for index in from..from + count {
        let track = tracks.get_mut(index)?;
        track.base += track.increase;
        track.increase = 0.0;
    }
    Some(())
}

/// 🔎️ CSS grid "find the size of an fr": the largest fr that fits `space` once the inflexible tracks and `fixed` (the gaps) are paid for,
/// restarting with every track whose share would fall under its base size treated as inflexible.
fn find_fraction(tracks: &Tracks, from: usize, count: usize, fixed: f32, space: f32) -> Option<f32> {
    if space == 0.0 {
        return Some(0.0);
    }
    let mut hypothetical = f32::INFINITY;
    for _ in 0..=count {
        let (mut used, mut flex_sum) = (fixed, 0.0_f32);
        for index in from..from + count {
            let track = tracks.get(index)?;
            match track.fraction() {
                Some(flex) if flex * hypothetical >= track.base => flex_sum += flex,
                _ => used += track.base,
            }
        }
        let previous = hypothetical;
        hypothetical = (space - used) / flex_sum.max(1.0);
        let mut valid = true;
        for index in from..from + count {
            let track = tracks.get(index)?;
            if let Some(flex) = track.fraction() {
                valid &= flex * hypothetical >= track.base || flex * previous < track.base;
            }
        }
        if valid {
            break;
        }
    }
    Some(hypothetical)
}

/// 🧲️ The first track's lead offset and the extra offset of every later track when `free` space is distributed by `justify`
/// (`None` is `stretch`, which packs at the start): CSS alignment with the distributed modes falling back to start or center.
fn track_alignment(free: f32, count: usize, justify: Option<Justify>) -> (f32, f32) {
    let degenerate = count <= 1 || free <= 0.0;
    let around = |lead: f32, between: f32| if !degenerate { (lead, between) } else if free <= 0.0 { (0.0, 0.0) } else { (free * 0.5, 0.0) };
    match justify {
        None | Some(Justify::Start) => (0.0, 0.0),
        Some(Justify::End) => (free, 0.0),
        Some(Justify::Center) => (free * 0.5, 0.0),
        Some(Justify::SpaceBetween) if !degenerate => (0.0, free / (count - 1) as f32),
        Some(Justify::SpaceBetween) => (0.0, 0.0),
        Some(Justify::SpaceAround) => around(free / count as f32 * 0.5, free / count as f32),
        Some(Justify::SpaceEvenly) => around(free / (count + 1) as f32, free / (count + 1) as f32),
    }
}

/// 📍️ Gives the `count` sized tracks of one axis their offsets inside the content box that starts at `origin`.
fn place_tracks(tracks: &mut Tracks, from: usize, count: usize, origin: f32, content: f32, gap: f32, justify: Option<Justify>) -> Option<()> {
    let (lead, between) = track_alignment(content - axis_extent(tracks, from, count, gap)?, count, justify);
    let mut cursor = origin;
    for (ordinal, index) in (from..from + count).enumerate() {
        let track = tracks.get_mut(index)?;
        track.offset = cursor + if ordinal == 0 { lead } else { between };
        cursor = track.offset + track.base + gap;
    }
    Some(())
}

/// 🔲️ One in-flow grid item's box inside its cell: its own length or the cell for each size, aligned inside the cell by `align`.
fn grid_cell(flow: FlowStyle, align: Align, intrinsic: (f32, f32), node: usize, column: TrackSize, row: TrackSize, measure: Measure<'_>) -> FlexRect {
    let width = grid_width(flow, column.base);
    let height = match flow.height {
        Dim::Length(value) => value,
        Dim::Fill => row.base,
        Dim::Auto if matches!(align, Align::Stretch) => row.base,
        Dim::Auto if flow.text => text_height(flow, node, width, measure),
        Dim::Auto => intrinsic.1,
    }
    .max(flow.min_height.max(flow.padding.top + flow.padding.bottom));
    let offset = match align {
        Align::Center => (row.base - height) * 0.5,
        Align::End => row.base - height,
        Align::Start | Align::Stretch | Align::Baseline => 0.0,
    };
    FlexRect { x: column.offset, y: row.offset + offset, width, height }
}

/// 🔳️ Sizes and aligns a grid's tracks against `owner`'s content box and writes every child's box into `resolved`.
#[allow(clippy::too_many_arguments)]
fn place_grid(flows: &Flows, grid: Option<&ui_contract::GridLayout>, intrinsic: &Sizes, resolved: &mut Rects, tracks: &mut Tracks, flow: FlowStyle, owner: FlexRect, children: impl Iterator<Item = usize> + Clone, measure: Measure<'_>) -> Option<()> {
    let content = ((owner.width - flow.padding.left - flow.padding.right).max(0.0), (owner.height - flow.padding.top - flow.padding.bottom).max(0.0));
    let (columns, rows) = solve_tracks(flows, intrinsic, tracks, grid, flow, children.clone(), (Some(content.0), Some(content.1)), measure)?;
    place_tracks(tracks, 0, columns, flow.padding.left, content.0, flow.gap_cross, Some(flow.justify))?;
    place_tracks(tracks, columns, rows, flow.padding.top, content.1, flow.gap_main, None)?;
    let mut cell = 0_usize;
    for child in children {
        let (child_flow, size) = (flows.get(child).copied()?, intrinsic.get(child).copied()?);
        let rect = if child_flow.absolute {
            child_flow.absolute_rect(owner, size)
        } else {
            let (column, row) = (*tracks.get(cell % columns)?, *tracks.get(columns + cell / columns)?);
            cell += 1;
            grid_cell(child_flow, flow.align, size, child, column, row, measure)
        };
        *resolved.get_mut(child)? = rect;
    }
    Some(())
}

impl FlowStyle {
    /// 🌊️ One in-flow child's `(main, cross)` base size inside `parent`'s content box. A measured text
    /// run is re-measured against the size its container actually offers it, so it wraps where CSS
    /// would: in a column its width is settled first and the wrapped height becomes its main size; in
    /// a row its width is the main size and the wrapped height becomes its cross size.
    ///
    /// 🧭️ A child's own axes are read along its PARENT's main axis, never its own `row` flag —
    /// that flag describes how IT arranges ITS children, which says nothing about how it is placed.
    fn flow_size(self, parent: FlowStyle, content_main: f32, content_cross: f32, intrinsic: (f32, f32), node: usize, stretch: bool, measure: Measure<'_>) -> (f32, f32) {
        let (own_main, own_cross, intrinsic_main, intrinsic_cross, min_main, min_cross) =
            if parent.row { (self.width, self.height, intrinsic.0, intrinsic.1, self.min_width, self.min_height) } else { (self.height, self.width, intrinsic.1, intrinsic.0, self.min_height, self.min_width) };
        let stretched = stretch && matches!(own_cross, Dim::Auto);
        let (mut main, mut cross) = if parent.row {
            let main = own_main.resolve(content_main).unwrap_or(intrinsic_main);
            let cross = if stretched {
                content_cross
            } else if self.text {
                own_cross.resolve(content_cross).unwrap_or_else(|| measure(node, MeasureConstraint::Definite(main)).1)
            } else {
                own_cross.resolve(content_cross).unwrap_or(intrinsic_cross)
            };
            (main, cross)
        } else {
            let cross = if stretched { content_cross } else { own_cross.resolve(content_cross).unwrap_or(intrinsic_cross) };
            let main = match own_main {
                Dim::Auto if self.text => measure(node, MeasureConstraint::Definite(cross)).1,
                dim => dim.resolve(content_main).unwrap_or(intrinsic_main),
            };
            (main, cross)
        };
        main = main.max(min_main);
        cross = cross.max(min_cross);
        (main, cross)
    }

    /// 📌️ An out-of-flow child's rect inside `owner`: each side comes from its own inset, its size
    /// from its own sizing, and — when both insets on an axis are given — from the span between them.
    fn absolute_rect(self, owner: FlexRect, intrinsic: (f32, f32)) -> FlexRect {
        let span = |size: Dim, start: Option<f32>, end: Option<f32>, available: f32, fallback: f32| {
            size.resolve(available).unwrap_or_else(|| match (start, end) {
                (Some(start), Some(end)) => (available - start - end).max(0.0),
                _ => fallback,
            })
        };
        let width = span(self.width, self.inset[3], self.inset[1], owner.width, intrinsic.0);
        let height = span(self.height, self.inset[0], self.inset[2], owner.height, intrinsic.1);
        let x = self.inset[3].unwrap_or_else(|| self.inset[1].map_or(0.0, |right| owner.width - right - width));
        let y = self.inset[0].unwrap_or_else(|| self.inset[2].map_or(0.0, |bottom| owner.height - bottom - height));
        FlexRect { x, y, width, height }
    }
}

/// ↔️ CSS `justify-content`: how leftover main-axis space is spent once no child can grow into it —
/// as a lead offset, as spacing between children, or both.
fn distribute(justify: Justify, remaining: f32, count: usize) -> (f32, f32) {
    let count = count as f32;
    match justify {
        Justify::Start => (0.0, 0.0),
        Justify::Center => (remaining * 0.5, 0.0),
        Justify::End => (remaining, 0.0),
        Justify::SpaceBetween if count > 1.0 => (0.0, remaining / (count - 1.0)),
        Justify::SpaceBetween => (0.0, 0.0),
        Justify::SpaceAround if count > 0.0 => (remaining / (count * 2.0), remaining / count),
        Justify::SpaceEvenly if count > 0.0 => (remaining / (count + 1.0), remaining / (count + 1.0)),
        Justify::SpaceAround | Justify::SpaceEvenly => (0.0, 0.0),
    }
}

//#endregion 🧮️FlexTree
// #endregion flex
