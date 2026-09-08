
use super::*;

type CachedTextMeasure = ((String, Option<u32>), (f32, f32));

#[derive(Clone, Debug, PartialEq)]
struct LayoutResult {
    id: NodeId,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    cached_text_measure: Option<CachedTextMeasure>,
}

#[derive(Clone, Copy, Debug, Default)]
struct IntrinsicSize {
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, Debug, Default)]
struct ChildAggregate {
    count: usize,
    width_sum: f32,
    height_sum: f32,
    max_width: f32,
    max_height: f32,
}

struct LayoutWalkFrame {
    id: NodeId,
    next_child: Option<NodeId>,
}

/// 🧵 Persistent layout and text-shaping state. One call performs at most one retained-node
/// traversal/sync/result unit or one uncached glyph-shaping unit, apart from taffy's opaque solve.
pub(super) struct LayoutJob {
    stage: LayoutJobStage,
    root: NodeId,
    theme: Theme,
    available_width: f32,
    available_height: f32,
    pending_root: Option<NodeId>,
    walk: Vec<LayoutWalkFrame>,
    preorder: Vec<NodeId>,
    postorder: Vec<NodeId>,
    text_nodes: Vec<NodeId>,
    text_node: usize,
    text_byte: usize,
    prune_index: usize,
    sync_index: usize,
    result_index: usize,
    results: Vec<LayoutResult>,
    fallback_measure_index: usize,
    fallback_arrange_index: usize,
    intrinsic: HashMap<NodeId, IntrinsicSize>,
    child_aggregates: HashMap<NodeId, ChildAggregate>,
    arranged: HashMap<NodeId, LayoutResult>,
    child_offsets: HashMap<NodeId, f32>,
}

const MAX_TAFFY_SLICE_NODES: usize = 128;

impl LayoutJob {
    pub(crate) fn new(tree: &UiTree, root: NodeId, theme: Theme, available_width: f32, available_height: f32) -> Option<Self> {
        let root_node = tree.node(root)?;
        if !root_node.flags.contains(NodeFlags::DIRTY_LAYOUT) && !root_node.flags.contains(NodeFlags::SUBTREE_DIRTY) {
            return None;
        }
        Some(Self {
            stage: LayoutJobStage::CollectNodes,
            root,
            theme,
            available_width,
            available_height,
            pending_root: Some(root),
            walk: Vec::new(),
            preorder: Vec::new(),
            postorder: Vec::new(),
            text_nodes: Vec::new(),
            text_node: 0,
            text_byte: 0,
            prune_index: 0,
            sync_index: 0,
            result_index: 0,
            results: Vec::new(),
            fallback_measure_index: 0,
            fallback_arrange_index: 0,
            intrinsic: HashMap::new(),
            child_aggregates: HashMap::new(),
            arranged: HashMap::new(),
            child_offsets: HashMap::new(),
        })
    }

    pub(crate) fn step(&mut self, engine: &mut LayoutEngine, tree: &mut UiTree, atlas: &mut FontAtlas, cx: &mut semio_framework_job::StepContext<'_>) -> LayoutJobStep {
        if cx.is_cancelled() {
            return LayoutJobStep::Cancelled;
        }
        if cx.should_yield() {
            return LayoutJobStep::Yield { stage: self.stage, nodes: 0, glyphs: 0 };
        }
        cx.set_stage(self.stage_label());
        loop {
            let progress = match self.stage {
                LayoutJobStage::CollectNodes => self.collect_node(tree),
                LayoutJobStage::ShapeText => self.shape_text(tree, atlas),
                LayoutJobStage::PruneRemoved => self.prune_removed(engine, tree),
                LayoutJobStage::SyncNodes => self.sync_node(engine, tree),
                LayoutJobStage::SolveLayout => Some(self.solve(engine, atlas)),
                LayoutJobStage::MeasureFallback => self.measure_fallback(tree, atlas),
                LayoutJobStage::ArrangeFallback => self.arrange_fallback(tree, atlas),
                LayoutJobStage::CollectResults => self.collect_result(engine, tree, atlas),
                LayoutJobStage::PublishResults => {
                    self.publish(tree, atlas);
                    return LayoutJobStep::Complete;
                }
            };
            if let Some((nodes, glyphs)) = progress {
                cx.consume_fuel(1);
                if cx.is_cancelled() {
                    return LayoutJobStep::Cancelled;
                }
                return LayoutJobStep::Yield { stage: self.stage, nodes, glyphs };
            }
        }
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            LayoutJobStage::CollectNodes => "Layout.CollectNodes",
            LayoutJobStage::ShapeText => "Layout.ShapeText",
            LayoutJobStage::PruneRemoved => "Layout.PruneRemoved",
            LayoutJobStage::SyncNodes => "Layout.SyncNodes",
            LayoutJobStage::SolveLayout => "Layout.SolveLayout",
            LayoutJobStage::MeasureFallback => "Layout.MeasureFallback",
            LayoutJobStage::ArrangeFallback => "Layout.ArrangeFallback",
            LayoutJobStage::CollectResults => "Layout.CollectResults",
            LayoutJobStage::PublishResults => "Layout.PublishResults",
        }
    }

    fn collect_node(&mut self, tree: &UiTree) -> Option<(usize, usize)> {
        let next = if let Some(root) = self.pending_root.take() {
            Some(root)
        } else {
            loop {
                let Some(frame) = self.walk.last_mut() else {
                    break None;
                };
                if let Some(child) = frame.next_child {
                    frame.next_child = tree.node(child).and_then(|node| node.next_sibling);
                    break Some(child);
                }
                let complete = self.walk.pop().expect("layout walk frame existed");
                self.postorder.push(complete.id);
                if self.walk.is_empty() {
                    break None;
                }
            }
        };
        if let Some(id) = next {
            let node = tree.node(id).expect("layout walk names a retained node");
            if matches!(node.spec.0, UiNode::Text(_)) {
                self.text_nodes.push(id);
            }
            self.preorder.push(id);
            self.walk.push(LayoutWalkFrame { id, next_child: node.first_child });
            return Some((1, 0));
        }
        self.results.reserve(self.preorder.len());
        self.stage = LayoutJobStage::ShapeText;
        None
    }

    fn shape_text(&mut self, tree: &UiTree, atlas: &mut FontAtlas) -> Option<(usize, usize)> {
        while let Some(&id) = self.text_nodes.get(self.text_node) {
            let value = match tree.node(id).map(|node| &node.spec.0) {
                Some(UiNode::Text(text)) => text.value.as_str(),
                _ => "",
            };
            if let Some(ch) = value.get(self.text_byte..).and_then(|tail| tail.chars().next()) {
                self.text_byte += ch.len_utf8();
                atlas.ensure_glyph(ch, DEFAULT_TEXT_SIZE_PX);
                return Some((0, 1));
            }
            self.text_node += 1;
            self.text_byte = 0;
        }
        self.stage = LayoutJobStage::PruneRemoved;
        None
    }

    fn prune_removed(&mut self, engine: &mut LayoutEngine, tree: &UiTree) -> Option<(usize, usize)> {
        #[cfg(not(test))]
        {
            let _ = (engine, tree);
            self.stage = LayoutJobStage::MeasureFallback;
            return None;
        }
        #[cfg(test)]
        if let Some(&id) = engine.nodes.known.get(self.prune_index) {
            if !tree.contains(id) {
                if let Some(taffy_id) = engine.nodes.by_ui.remove(&id) {
                    let _ = engine.taffy.remove(taffy_id);
                }
            }
            self.prune_index += 1;
            return Some((1, 0));
        }
        #[cfg(test)]
        {
            engine.nodes.known.clear();
            engine.nodes.known.reserve(self.preorder.len());
            self.stage = if self.preorder.len() > MAX_TAFFY_SLICE_NODES { LayoutJobStage::MeasureFallback } else { LayoutJobStage::SyncNodes };
            None
        }
    }

    fn sync_node(&mut self, engine: &mut LayoutEngine, tree: &UiTree) -> Option<(usize, usize)> {
        #[cfg(not(test))]
        {
            let _ = (engine, tree);
            self.stage = LayoutJobStage::MeasureFallback;
            return None;
        }
        #[cfg(test)]
        if let Some(&id) = self.postorder.get(self.sync_index) {
            let node = tree.node(id).expect("sync cursor names a retained node");
            let flex_grow_child = node.parent.and_then(|parent| tree.node(parent)).is_some_and(|parent| matches!(parent.spec.0, UiNode::Stack(_) | UiNode::Field(_)));
            engine.sync_one(tree, &self.theme, id, flex_grow_child);
            engine.nodes.known.push(id);
            self.sync_index += 1;
            return Some((1, 0));
        }
        #[cfg(test)]
        {
            self.stage = LayoutJobStage::SolveLayout;
            None
        }
    }

    fn solve(&mut self, engine: &mut LayoutEngine, atlas: &mut FontAtlas) -> (usize, usize) {
        #[cfg(not(test))]
        {
            let _ = (engine, atlas);
            self.stage = LayoutJobStage::MeasureFallback;
            return (0, 0);
        }
        #[cfg(test)]
        {
            let root_taffy = engine.nodes.by_ui[&self.root];
            let mut root_style = engine.taffy.style(root_taffy).cloned().unwrap_or_default();
            root_style.size = Size { width: length(self.available_width), height: length(self.available_height) };
            let _ = engine.taffy.set_style(root_taffy, root_style);
            let available = Size { width: AvailableSpace::Definite(self.available_width), height: AvailableSpace::Definite(self.available_height) };
            let _ = engine.taffy.compute_layout_with_measure(root_taffy, available, |known_dimensions, available_space, _node_id, node_context, _style| {
                if let (Some(width), Some(height)) = (known_dimensions.width, known_dimensions.height) {
                    return Size { width, height };
                }
                match node_context {
                    Some(LeafContext::Text(text)) => {
                        let max_width = known_dimensions.width.or_else(|| available_space.width.into_option());
                        let (measured_w, measured_h) = atlas.measure(text, max_width);
                        Size { width: known_dimensions.width.unwrap_or(measured_w), height: known_dimensions.height.unwrap_or(measured_h) }
                    }
                    _ => Size::ZERO,
                }
            });
            self.stage = LayoutJobStage::CollectResults;
            (1, 0)
        }
    }

    fn measure_fallback(&mut self, tree: &UiTree, atlas: &mut FontAtlas) -> Option<(usize, usize)> {
        if let Some(&id) = self.postorder.get(self.fallback_measure_index) {
            let node = tree.node(id).expect("fallback measure cursor names a retained node");
            let aggregate = self.child_aggregates.get(&id).copied().unwrap_or_default();
            let size = match &node.spec.0 {
                UiNode::Text(text) => {
                    let measured = atlas.measure(text.value.as_str(), Some(self.available_width));
                    IntrinsicSize { width: measured.0, height: measured.1 }
                }
                UiNode::Stack(stack) => {
                    let gap = gap_for_token(&self.theme, stack.gap.as_deref()) * aggregate.count.saturating_sub(1) as f32;
                    let padding = padding_for_token(&self.theme, stack.padding.as_deref()) * 2.0;
                    if stack.direction == "horizontal" {
                        IntrinsicSize { width: aggregate.width_sum + gap + padding, height: aggregate.max_height + padding }
                    } else {
                        IntrinsicSize { width: aggregate.max_width + padding, height: aggregate.height_sum + gap + padding }
                    }
                }
                UiNode::Field(_) => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum + self.theme.font_size_small + gap_for_token(&self.theme, Some("standard")) },
                UiNode::Section(_) => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum + SECTION_HEADER_HEIGHT + self.theme.gap_standard * aggregate.count.saturating_sub(1) as f32 },
                _ => IntrinsicSize { width: aggregate.max_width, height: aggregate.height_sum },
            };
            self.intrinsic.insert(id, size);
            if let Some(parent) = node.parent {
                let parent_aggregate = self.child_aggregates.entry(parent).or_default();
                parent_aggregate.count += 1;
                parent_aggregate.width_sum += size.width;
                parent_aggregate.height_sum += size.height;
                parent_aggregate.max_width = parent_aggregate.max_width.max(size.width);
                parent_aggregate.max_height = parent_aggregate.max_height.max(size.height);
            }
            self.fallback_measure_index += 1;
            return Some((1, 0));
        }
        self.stage = LayoutJobStage::ArrangeFallback;
        None
    }

    fn arrange_fallback(&mut self, tree: &UiTree, atlas: &mut FontAtlas) -> Option<(usize, usize)> {
        if let Some(&id) = self.preorder.get(self.fallback_arrange_index) {
            let node = tree.node(id).expect("fallback arrange cursor names a retained node");
            let intrinsic = self.intrinsic.get(&id).copied().unwrap_or_default();
            let (x, y, width, height) = if id == self.root {
                (0.0, 0.0, self.available_width, self.available_height)
            } else {
                let parent = node.parent.expect("non-root fallback node has a parent");
                let parent_result = self.arranged.get(&parent).expect("fallback preorder arranged the parent first");
                let parent_node = tree.node(parent).expect("fallback parent exists");
                let aggregate = self.child_aggregates.get(&parent).copied().unwrap_or_default();
                let offset = *self.child_offsets.get(&parent).unwrap_or(&0.0);
                match &parent_node.spec.0 {
                    UiNode::Stack(stack) if stack.direction == "horizontal" => {
                        let padding = padding_for_token(&self.theme, stack.padding.as_deref());
                        let gap = gap_for_token(&self.theme, stack.gap.as_deref());
                        let content = (parent_result.width - padding * 2.0).max(0.0);
                        let extra = ((content - aggregate.width_sum - gap * aggregate.count.saturating_sub(1) as f32) / aggregate.count.max(1) as f32).max(0.0);
                        (padding + offset, padding, intrinsic.width + extra, (parent_result.height - padding * 2.0).max(0.0))
                    }
                    UiNode::Stack(stack) => {
                        let padding = padding_for_token(&self.theme, stack.padding.as_deref());
                        let gap = gap_for_token(&self.theme, stack.gap.as_deref());
                        let content = (parent_result.height - padding * 2.0).max(0.0);
                        let extra = ((content - aggregate.height_sum - gap * aggregate.count.saturating_sub(1) as f32) / aggregate.count.max(1) as f32).max(0.0);
                        (padding, padding + offset, (parent_result.width - padding * 2.0).max(0.0), intrinsic.height + extra)
                    }
                    UiNode::Field(_) => {
                        let top = self.theme.font_size_small + gap_for_token(&self.theme, Some("standard"));
                        (0.0, top, parent_result.width, (parent_result.height - top).max(0.0))
                    }
                    UiNode::Section(_) => (0.0, SECTION_HEADER_HEIGHT + offset, parent_result.width, intrinsic.height),
                    _ => (0.0, offset, parent_result.width, intrinsic.height),
                }
            };
            let cached_text_measure = match &node.spec.0 {
                UiNode::Text(text) => {
                    let value = text.value.clone().into_string();
                    Some(((value.clone(), quantize_width(Some(width))), atlas.measure(&value, Some(width))))
                }
                _ => None,
            };
            let result = LayoutResult { id, x, y, width, height, cached_text_measure };
            self.arranged.insert(id, result.clone());
            self.results.push(result);
            if let Some(parent) = node.parent {
                let gap = match tree.node(parent).map(|parent| &parent.spec.0) {
                    Some(UiNode::Stack(stack)) => gap_for_token(&self.theme, stack.gap.as_deref()),
                    Some(UiNode::Section(_)) => self.theme.gap_standard,
                    _ => 0.0,
                };
                let horizontal = tree.node(parent).is_some_and(|parent| matches!(&parent.spec.0, UiNode::Stack(stack) if stack.direction == "horizontal"));
                *self.child_offsets.entry(parent).or_default() += if horizontal { width + gap } else { height + gap };
            }
            self.fallback_arrange_index += 1;
            return Some((1, 0));
        }
        self.stage = LayoutJobStage::PublishResults;
        None
    }

    fn collect_result(&mut self, engine: &LayoutEngine, tree: &UiTree, atlas: &mut FontAtlas) -> Option<(usize, usize)> {
        #[cfg(not(test))]
        {
            let _ = (engine, tree, atlas);
            self.stage = LayoutJobStage::PublishResults;
            return None;
        }
        #[cfg(test)]
        if let Some(&id) = self.preorder.get(self.result_index) {
            if let Some(&taffy_id) = engine.nodes.by_ui.get(&id) {
                if let Ok(layout) = engine.taffy.layout(taffy_id) {
                    let cached_text_measure = match tree.node(id).map(|node| &node.spec.0) {
                        Some(UiNode::Text(text)) => {
                            let value = text.value.clone().into_string();
                            Some(((value.clone(), quantize_width(Some(layout.size.width))), atlas.measure(&value, Some(layout.size.width))))
                        }
                        _ => None,
                    };
                    self.results.push(LayoutResult { id, x: layout.location.x, y: layout.location.y, width: layout.size.width, height: layout.size.height, cached_text_measure });
                }
            }
            self.result_index += 1;
            return Some((1, 0));
        }
        #[cfg(test)]
        {
            self.stage = LayoutJobStage::PublishResults;
            None
        }
    }

    fn publish(&mut self, tree: &mut UiTree, _atlas: &mut FontAtlas) {
        for result in &self.results {
            if let Some(node) = tree.node_mut(result.id) {
                node.layout.x = result.x;
                node.layout.y = result.y;
                node.layout.width = result.width;
                node.layout.height = result.height;
                node.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                node.flags.set(NodeFlags::SUBTREE_DIRTY, false);
                if let Some(cached) = &result.cached_text_measure {
                    if node.layout.cached_text_measure.as_ref() != Some(cached) {
                        node.layout.cached_text_measure = Some(cached.clone());
                    }
                }
            }
        }
    }
}
