//! 🧮️ Flex tree: the scratch leaf, the `FlexTree` with its measure and arrange steps, the flow style distribution and the scratch solver. See [`super`].

use super::*;

// #region flex
//#region 🧮️FlexTree
/// 🍃️ What one scratch leaf answers the taffy fallback with: a text run re-measured against the
/// width it is actually offered (so it wraps the way CSS does), or an already-measured box.
#[derive(Clone, Copy)]
struct ScratchLeaf {
    pub(super) node: usize,
    pub(super) text: bool,
    pub(super) intrinsic: (f32, f32),
}

/// 🧮️ Owns one bounded layout pass's styles and its solved boxes.
///
/// **The pass is per CONTAINER, never global, and plain flex is resolved arithmetically.** A single
/// whole-tree solve is not resumable, and even a single taffy solve of ONE 1,024-child container
/// measured ~11 ms, which starves every other interactive job on the lane (`engine`'s own
/// eight-millisecond slice law). So layout runs the way CSS itself resolves a box — a bottom-up
/// intrinsic (max-content) measurement, then a top-down arrange — and each grant handles exactly ONE
/// container against its own already-resolved box. For a plain single-line flex container (the
/// overwhelming majority, and the only shape the legacy chrome ever builds) that grant is pure
/// arithmetic over its direct children: flex-basis, grow/shrink, justify distribution, per-child
/// cross alignment, out-of-flow placement. `Wrapped` and `Grid` containers hand the same one-container
/// grant to taffy instead, which keeps multi-line reflow and real grid track sizing exact; those two
/// shapes are small in practice (a toolbar, a property grid), so the grant stays bounded either way.
/// Taffy also remains the third-party oracle the arithmetic path is validated against in `🧪️tests`.
///
/// No taffy value is RETAINED here — a local `taffy::TaffyTree` lives inside the two fallback methods
/// and dies with them. That is what keeps this type `Send` without an `unsafe impl`, which matters
/// because `mounted_layout::MountedLayoutJob` owns one and runs on a shared worker thread
/// (`taffy::style::CompactLength` packs its payload into a `*const ()`, so a retained taffy tree is
/// not `Send`).
pub struct FlexTree {
    pub(super) flows: Vec<FlowStyle>,
    pub(super) grids: Vec<Option<Box<ui_contract::GridLayout>>>,
    pub(super) intrinsic: Vec<(f32, f32)>,
    pub(super) resolved: Vec<FlexRect>,
    pub(super) main: Vec<f32>,
    pub(super) cross: Vec<f32>,
}

impl Default for FlexTree {
    fn default() -> Self {
        Self::new()
    }
}

impl FlexTree {
    pub fn new() -> Self {
        Self { flows: Vec::new(), grids: Vec::new(), intrinsic: Vec::new(), resolved: Vec::new(), main: Vec::new(), cross: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.flows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.flows.is_empty()
    }

    /// 🌱️ Admits the job's next node (nodes arrive in the admission walk's depth-first preorder).
    /// Only a `Grid` retains its authored track lists — everything else layout needs lives in the
    /// [`FlowStyle`], so the common node costs one small `Copy` record and no allocation at all.
    pub fn push(&mut self, kind: LayoutNodeKind, parent: Option<usize>, authored: Option<&LayoutSpec>, metrics: &TreeRowMetrics, parent_kind: Option<LayoutNodeKind>, content: Option<TreeContentFlow>) -> bool {
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
        self.flows.push(flow);
        self.grids.push(grid);
        self.intrinsic.push((0.0, 0.0));
        self.resolved.push(FlexRect::default());
        true
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

    /// 📏️ Node `index`'s intrinsic (max-content) size, from its `children`'s already-measured intrinsic
    /// sizes — the bottom-up half. `children` must be in document order, and every one of them must be
    /// measured already (reverse admission order guarantees it).
    pub fn measure_one(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let Some(flow) = self.flows.get(index).copied() else { return false };
        if !matches!(flow.kind, FlowKind::Flow) {
            return self.measure_via_taffy(index, children, measure);
        }
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (mut main_sum, mut cross_max, mut count) = (0.0_f32, 0.0_f32, 0_usize);
        for &child in children {
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
        let (mut width, mut height) = if flow.row { (main, cross) } else { (cross, main) };
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

    /// 📐️ Arranges `children` inside node `index`'s own resolved box — the top-down half. Each child's
    /// parent-relative rect is written into this tree, so a later [`Self::rect`] reads it back.
    pub fn arrange_one(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let (Some(flow), Some(&owner)) = (self.flows.get(index).copied(), self.resolved.get(index)) else { return false };
        if children.is_empty() {
            return true;
        }
        if !matches!(flow.kind, FlowKind::Flow) {
            return self.arrange_via_taffy(index, children, measure);
        }
        let (main_start, main_end) = flow.padding_main();
        let (cross_start, cross_end) = flow.padding_cross();
        let (outer_main, outer_cross) = if flow.row { (owner.width, owner.height) } else { (owner.height, owner.width) };
        let content_main = (outer_main - main_start - main_end).max(0.0);
        let content_cross = (outer_cross - cross_start - cross_end).max(0.0);

        self.main.clear();
        self.cross.clear();
        let (mut base_sum, mut grow_sum, mut shrink_sum, mut count) = (0.0_f32, 0.0_f32, 0.0_f32, 0_usize);
        for &child in children {
            let (Some(child_flow), Some(&intrinsic)) = (self.flows.get(child).copied(), self.intrinsic.get(child)) else { return false };
            if child_flow.absolute {
                self.main.push(0.0);
                self.cross.push(0.0);
                continue;
            }
            let (main, cross) = child_flow.flow_size(flow, content_main, content_cross, intrinsic, child, measure);
            base_sum += main + child_flow.lead;
            grow_sum += child_flow.grow;
            shrink_sum += child_flow.shrink * main;
            count += 1;
            self.main.push(main);
            self.cross.push(cross);
        }

        let gaps = flow.gap_main * count.saturating_sub(1) as f32;
        let free = content_main - base_sum - gaps;
        if free > 0.0 && grow_sum > 0.0 {
            for (slot, &child) in self.main.iter_mut().zip(children) {
                let Some(child_flow) = self.flows.get(child) else { return false };
                if !child_flow.absolute {
                    *slot += free * child_flow.grow / grow_sum;
                }
            }
        } else if free < 0.0 && shrink_sum > 0.0 {
            for (slot, &child) in self.main.iter_mut().zip(children) {
                let Some(child_flow) = self.flows.get(child) else { return false };
                if !child_flow.absolute {
                    *slot = (*slot + free * (child_flow.shrink * *slot) / shrink_sum).max(0.0);
                }
            }
        }

        let placed: f32 = self.main.iter().sum();
        let remaining = (content_main - placed - gaps).max(0.0);
        let (lead, between) = distribute(flow.justify, remaining, count);
        let mut cursor = if flow.reverse { outer_main - main_end - lead } else { main_start + lead };
        for (ordinal, &child) in children.iter().enumerate() {
            let (Some(child_flow), Some(&main), Some(&cross)) = (self.flows.get(child).copied(), self.main.get(ordinal), self.cross.get(ordinal)) else { return false };
            let rect = if child_flow.absolute {
                child_flow.absolute_rect(owner, self.intrinsic.get(child).copied().unwrap_or((0.0, 0.0)))
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
            match self.resolved.get_mut(child) {
                Some(slot) => *slot = rect,
                None => return false,
            }
        }
        true
    }

    fn taffy_style_of(&self, index: usize) -> Option<taffy::Style> {
        Some(taffy_style(self.flows.get(index)?, self.grids.get(index)?.as_deref()))
    }

    fn scratch_child(&self, scratch: &mut taffy::TaffyTree<ScratchLeaf>, index: usize) -> Option<taffy::NodeId> {
        let style = self.taffy_style_of(index)?;
        let leaf = ScratchLeaf { node: index, text: self.flows.get(index)?.text, intrinsic: *self.intrinsic.get(index)? };
        scratch.new_leaf_with_context(style, leaf).ok()
    }

    /// 📦️ The container's own taffy style for a local solve: it IS the root of that solve, so its own
    /// out-of-flow placement and grow factor (its parent's business, already settled) are cleared.
    fn own_style(&self, index: usize) -> Option<taffy::Style> {
        let mut style = self.taffy_style_of(index)?;
        style.position = taffy::style::Position::Relative;
        style.inset = taffy::geometry::Rect::auto();
        style.flex_grow = 0.0;
        Some(style)
    }

    /// 📏️ The `Wrapped`/`Grid` intrinsic measurement: one local taffy solve over this container and
    /// its already-measured children, at max-content.
    fn measure_via_taffy(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let Some(style) = self.own_style(index) else { return false };
        let mut scratch: taffy::TaffyTree<ScratchLeaf> = taffy::TaffyTree::new();
        scratch.disable_rounding();
        let mut kids = Vec::with_capacity(children.len());
        for &child in children {
            let Some(node) = self.scratch_child(&mut scratch, child) else { return false };
            kids.push(node);
        }
        let Ok(root) = scratch.new_with_children(style, &kids) else { return false };
        let available = taffy::geometry::Size { width: taffy::AvailableSpace::MaxContent, height: taffy::AvailableSpace::MaxContent };
        if solve_scratch(&mut scratch, root, available, measure).is_none() {
            return false;
        }
        let Ok(layout) = scratch.layout(root) else { return false };
        match self.intrinsic.get_mut(index) {
            Some(slot) => *slot = (layout.size.width, layout.size.height),
            None => return false,
        }
        true
    }

    /// 📐️ The `Wrapped`/`Grid` arrange: one local taffy solve against this container's resolved box.
    fn arrange_via_taffy(&mut self, index: usize, children: &[usize], measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> bool {
        let (Some(mut style), Some(&owner)) = (self.own_style(index), self.resolved.get(index)) else { return false };
        style.size = taffy::geometry::Size { width: taffy::style::Dimension::length(owner.width), height: taffy::style::Dimension::length(owner.height) };
        style.min_size = taffy::geometry::Size { width: taffy::style::Dimension::length(0.0), height: taffy::style::Dimension::length(0.0) };
        style.max_size = taffy::geometry::Size { width: taffy::style::Dimension::auto(), height: taffy::style::Dimension::auto() };
        let mut scratch: taffy::TaffyTree<ScratchLeaf> = taffy::TaffyTree::new();
        scratch.disable_rounding();
        let mut kids = Vec::with_capacity(children.len());
        for &child in children {
            let Some(node) = self.scratch_child(&mut scratch, child) else { return false };
            kids.push(node);
        }
        let Ok(root) = scratch.new_with_children(style, &kids) else { return false };
        let available = taffy::geometry::Size { width: taffy::AvailableSpace::Definite(owner.width), height: taffy::AvailableSpace::Definite(owner.height) };
        if solve_scratch(&mut scratch, root, available, measure).is_none() {
            return false;
        }
        for (&child, node) in children.iter().zip(kids) {
            let Ok(layout) = scratch.layout(node) else { return false };
            match self.resolved.get_mut(child) {
                Some(slot) => *slot = FlexRect { x: layout.location.x, y: layout.location.y, width: layout.size.width, height: layout.size.height },
                None => return false,
            }
        }
        true
    }

    /// 🧹️ Releases exactly ONE node, newest first — the same one-owner-per-grant close discipline
    /// every other bounded list in `mounted_layout` follows. Returns `true` once already empty.
    pub fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if let Some(Some(grid))=self.grids.last(){
            if !grid.columns.is_empty(){return Ok(0)}
            if !grid.columns.terminal_is_empty(){return grid.columns.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)}
            if !grid.rows.is_empty(){return Ok(0)}
            if !grid.rows.terminal_is_empty(){return grid.rows.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)}
            return Ok(size_of::<ui_contract::GridLayout>())
        }
        if !self.flows.is_empty(){return Ok(0)}
        fn bytes<T>(value:&Vec<T>)->Result<usize,semio_framework_value::ValueError>{value.capacity().checked_mul(size_of::<T>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"flex backing extent overflow"))}
        if self.flows.capacity()!=0{return bytes(&self.flows)}if self.grids.capacity()!=0{return bytes(&self.grids)}if self.intrinsic.capacity()!=0{return bytes(&self.intrinsic)}if self.resolved.capacity()!=0{return bytes(&self.resolved)}if self.main.capacity()!=0{return bytes(&self.main)}bytes(&self.cross)
    }
    pub fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{
        if let Some(Some(grid))=self.grids.last(){
            let depth=if !grid.columns.is_empty(){grid.columns.next_pop_depth_demand()}else if !grid.columns.terminal_is_empty(){grid.columns.next_release_depth_demand()}else if !grid.rows.is_empty(){grid.rows.next_pop_depth_demand()}else if !grid.rows.terminal_is_empty(){grid.rows.next_release_depth_demand()}else{return Ok(1)}.map_err(semio_framework_value::ValueError::from)?;
            return depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original grid close native parent depth overflow"))
        }
        Ok(usize::from(!self.close_is_empty()))
    }
    pub fn close_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();if self.close_is_empty(){return Ok(Step::Complete{progress:empty})}
        if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        let bytes=match self.next_close_release_byte_demand(){Ok(bytes)=>bytes,Err(error)=>return Err(error)};
        let depth=self.next_close_depth_demand()?;if bytes>grant.maximum_release_bytes||grant.maximum_depth<depth{return Ok(Step::Pending{progress:empty})}
        if let Some(Some(grid))=self.grids.last_mut(){
            let result=if grid.columns.pop().is_some(){None}else if !grid.columns.terminal_is_empty(){Some(grid.columns.release_empty_page(grant.maximum_release_bytes))}else if grid.rows.pop().is_some(){None}else if !grid.rows.terminal_is_empty(){Some(grid.rows.release_empty_page(grant.maximum_release_bytes))}else{*self.grids.last_mut().unwrap()=None;return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})};
            return match result{Some(Ok(receipt))=>Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:usize::from(receipt.progressed),released_bytes:receipt.released_allocation_bytes,..Default::default()}}),Some(Err(error))=>{let error=semio_framework_value::ValueError::from(error);Err(error)},None=>Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        }
        if self.flows.pop().is_some(){self.grids.pop();self.intrinsic.pop();self.resolved.pop();return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        macro_rules! release{($field:ident)=>{if self.$field.capacity()!=0{drop(std::mem::take(&mut self.$field));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})}}}
        release!(flows);release!(grids);release!(intrinsic);release!(resolved);release!(main);release!(cross);Ok(Step::Complete{progress:empty})
    }
    pub fn close_is_empty(&self)->bool{self.flows.capacity()==0&&self.grids.capacity()==0&&self.intrinsic.capacity()==0&&self.resolved.capacity()==0&&self.main.capacity()==0&&self.cross.capacity()==0}


}

impl FlowStyle {
    /// 📏️ One in-flow child's `(main, cross)` base size inside `parent`'s content box. A measured text
    /// run is re-measured against the size its container actually offers it, so it wraps where CSS
    /// would: in a column its width is settled first and the wrapped height becomes its main size; in
    /// a row its width is the main size and the wrapped height becomes its cross size.
    ///
    /// 🧭️ A child's own axes are read along its PARENT's main axis, never its own `row` flag —
    /// that flag describes how IT arranges ITS children, which says nothing about how it is placed.
    fn flow_size(self, parent: FlowStyle, content_main: f32, content_cross: f32, intrinsic: (f32, f32), node: usize, measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> (f32, f32) {
        let (own_main, own_cross, intrinsic_main, intrinsic_cross, min_main, min_cross) =
            if parent.row { (self.width, self.height, intrinsic.0, intrinsic.1, self.min_width, self.min_height) } else { (self.height, self.width, intrinsic.1, intrinsic.0, self.min_height, self.min_width) };
        let stretched = matches!(parent.align, Align::Stretch) && matches!(own_cross, Dim::Auto);
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

/// 🏁️ Runs one local taffy solve, answering every scratch leaf from its own measurement rule.
fn solve_scratch(scratch: &mut taffy::TaffyTree<ScratchLeaf>, root: taffy::NodeId, available: taffy::geometry::Size<taffy::AvailableSpace>, measure: &mut dyn FnMut(usize, MeasureConstraint) -> (f32, f32)) -> Option<()> {
    scratch
        .compute_layout_with_measure(root, available, |known, space, _node, context, _style| {
            if let (Some(width), Some(height)) = (known.width, known.height) {
                return taffy::geometry::Size { width, height };
            }
            let Some(leaf) = context else { return taffy::geometry::Size::ZERO };
            let (measured_width, measured_height) = if leaf.text {
                let constraint = match known.width {
                    Some(width) => MeasureConstraint::Definite(width),
                    None => match space.width {
                        taffy::AvailableSpace::Definite(width) => MeasureConstraint::Definite(width),
                        taffy::AvailableSpace::MinContent => MeasureConstraint::MinContent,
                        taffy::AvailableSpace::MaxContent => MeasureConstraint::MaxContent,
                    },
                };
                measure(leaf.node, constraint)
            } else {
                leaf.intrinsic
            };
            taffy::geometry::Size { width: known.width.unwrap_or(measured_width), height: known.height.unwrap_or(measured_height) }
        })
        .ok()
}
//#endregion 🧮️FlexTree
// #endregion flex
