//! 📐️ Text admission and measure steps: the tree content, the text admission, the unwind and worker steps, the shaping, the child gathering, the measuring and the arrangement. See [`super`].

use super::*;

//#region 🧵️MountedLayoutText
/// 👶️ One admitted node's direct children in document order, read straight off the sibling links admission built — a cloneable cursor, so no scan over the node array and no scratch buffer is ever needed.
#[derive(Clone)]
pub(super) struct Siblings<'a> {
    nodes: &'a ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>,
    next: Option<usize>,
}

impl<'a> Siblings<'a> {
    pub(super) fn of(nodes: &'a ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>, parent: usize) -> Self {
        Self { nodes, next: nodes.get(parent).and_then(|owner| owner.first_child) }
    }
}

impl Iterator for Siblings<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let current = self.next?;
        self.next = self.nodes.get(current).and_then(|node| node.next_sibling);
        Some(current)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TextPush {
    Glyph,
    Run,
    Neither,
}

impl MountedLayoutJob {
    /// 🧺️ How the node `id` under the admitted node `parent` lays out as tree-row content, when it is: a direct content
    /// child of a row whose content is more than one inline control takes its slot below the content before it
    /// (`reconcile::tree_row_content_lines`), a content container's children stack as columns and lines.
    pub(super) fn tree_content(&self, tree: &UiTree, id: NodeId, parent: Option<usize>) -> Option<TreeContentFlow> {
        let owner = self.nodes.as_deref().expect("live layout list owner").get(parent?)?;
        let document = tree.document()?;
        let record = document.record(tree.document_id(id)?)?;
        let metrics = retained_tree_row_metrics(tree, id, &self.row_metrics);
        let (line, band) = (metrics.content_line_height(), metrics.content_line_height() + metrics.gap);
        let (above, below) = match &record.component {
            ui_contract::Component::Container(props) => semio_framework_ui_retained_tree::reconcile::tree_content_label_lines(props),
            _ => (0, 0),
        };
        let (above, below) = (above as f32 * band, below as f32 * band);
        let container = matches!(record.component, ui_contract::Component::Container(_));
        if owner.content.is_some() {
            return Some(if container { TreeContentFlow::Column { above, below } } else { TreeContentFlow::Line { height: line } });
        }
        if !matches!(owner.kind, LayoutNodeKind::TreeRow { .. }) {
            return None;
        }
        let row = document.record(tree.document_id(owner.id)?)?;
        let total = semio_framework_ui_retained_tree::reconcile::tree_row_content_lines(document, row)?;
        let lines_of = |child: &ui_contract::UiNodeRecord| u16::try_from(semio_framework_ui_retained_tree::reconcile::tree_content_lines(document, child, 0)).unwrap_or(u16::MAX);
        let mut before = 0_u32;
        for child in semio_framework_ui_retained_tree::reconcile::tree_row_content(document, row) {
            if child.id == record.id {
                let top = (metrics.row_height - tree_content_height(u16::try_from(total).unwrap_or(u16::MAX), line, metrics.gap)) * 0.5 + before as f32 * band;
                return Some(TreeContentFlow::Slot { top, height: tree_content_height(lines_of(child), line, metrics.gap), above, below, container });
            }
            before += u32::from(lines_of(child));
        }
        None
    }

    /// 🎟️ Admits at most one backing page of `list` under the caller's own grant and debits exactly what it allocated;
    /// `true` means the next push owns an admitted slot (or the list is at its credit and the push itself reports it).
    fn fund_list<T, const N: usize>(list: &mut ui_contract::UiFixedList<T, N>, cx: &mut semio_framework_job::StepContext<'_>, fault: MountedLayoutFault) -> Result<bool, MountedLayoutFault> {
        if list.has_reserved_slot() || list.len() == N {
            return Ok(true);
        }
        let progress = list.try_reserve_one_funded(cx.retained_grant()).map_err(|_| fault)?;
        cx.consume_retained(progress).map_err(|_| fault)?;
        Ok(false)
    }

    /// 🔎️ What the next text admission turn pushes: the same decision `admit_text_one` takes, read without moving.
    fn text_push_need(&self, tree: &UiTree) -> TextPush {
        let Some(input) = self.text_node.and_then(|index| self.nodes.as_deref().expect("live layout list owner").get(index)) else { return TextPush::Neither };
        let Some(spec) = tree.node(input.id).map(|node| &node.spec.0) else { return TextPush::Neither };
        let part: Option<Option<&str>> = match (spec, self.text_part) {
            (UiNode::Text(text), 0) => Some(Some(text.value.as_str())),
            (UiNode::Button(button), 0) => Some(Some(button.label.as_str())),
            (UiNode::Section(section), 0) => Some(section.label.as_ref().map(|label| label.as_str())),
            (UiNode::Field(field), 0) => Some(Some(field.label.as_str())),
            (UiNode::Field(field), 1) => Some(field.description.as_deref()),
            (UiNode::Field(field), 2) => Some(field.error.as_deref()),
            (UiNode::Slider(slider), 0) => Some(slider.shown_unit()),
            (UiNode::Text(_) | UiNode::Button(_) | UiNode::Section(_) | UiNode::Slider(_), 1..) | (UiNode::Field(_), 3..) => None,
            _ => return TextPush::Neither,
        };
        match part {
            None => TextPush::Run,
            Some(None) => TextPush::Neither,
            Some(Some(value)) => if matches!(spec, UiNode::Slider(_)) || value.get(self.text_byte..).and_then(|tail| tail.chars().next()).is_some() { TextPush::Glyph } else { TextPush::Neither },
        }
    }

    /// 🔏️ Funds, one page per turn, every list the next admission turn pushes into; `true` once the turn may do its work.
    pub(super) fn fund_admission_slot(&mut self, tree: &UiTree, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, MountedLayoutFault> {
        match self.admission {
            AdmissionPhase::Visit | AdmissionPhase::VisitReversed => {
                if self.pending_node.is_none() {
                    return Ok(true);
                }
                if !Self::fund_list(self.nodes.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::NodeCredits)? {
                    return Ok(false);
                }
                if self.nodes.as_deref().expect("live layout list owner").len() == LAYOUT_NODE_CREDITS {
                    return Ok(true);
                }
                if !Self::fund_list(self.walk.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::DepthCredits)? {
                    return Ok(false);
                }
                match self.flex.fund_next_node(cx.retained_grant()).map_err(|_| MountedLayoutFault::NodeCredits)? {
                    NodeFunding::Ready => Ok(true),
                    NodeFunding::Funded(progress) => {
                        cx.consume_retained(progress).map_err(|_| MountedLayoutFault::NodeCredits)?;
                        Ok(false)
                    }
                    NodeFunding::Denied => Ok(false),
                }
            }
            AdmissionPhase::Text => match self.text_push_need(tree) {
                TextPush::Glyph => Self::fund_list(self.glyphs.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::GlyphCredits),
                TextPush::Run => Self::fund_list(self.runs.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::NodeCredits),
                TextPush::Neither => Ok(true),
            },
            AdmissionPhase::Ready if !self.glyphs.as_deref().expect("live layout list owner").is_empty() => Self::fund_list(self.glyph_previews.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::GlyphCredits),
            AdmissionPhase::Unwind | AdmissionPhase::Ready => Ok(true),
        }
    }

    /// 🐝️ Funds, one page per turn, the lists the next worker turn of the current stage pushes into.
    fn fund_worker_slot(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, MountedLayoutFault> {
        match self.stage {
            LayoutJobStage::ShapeText if self.glyph_cursor < self.glyphs.as_deref().expect("live layout list owner").len() => Self::fund_list(self.glyph_previews.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::GlyphCredits),
            LayoutJobStage::CollectResults => {
                let Some(input) = self.nodes.as_deref().expect("live layout list owner").get(self.collect_cursor) else { return Ok(true) };
                let line = matches!(input.kind, LayoutNodeKind::Text | LayoutNodeKind::Control { label_padding: Some(_), .. } | LayoutNodeKind::OverlayRow { .. } | LayoutNodeKind::Field { .. } | LayoutNodeKind::Section { .. }) || matches!(input.chrome, ChromeTextRanges::SliderUnit { .. });
                if line && !Self::fund_list(self.lines.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::NodeCredits)? {
                    return Ok(false);
                }
                Self::fund_list(self.results.as_deref_mut().expect("live layout list owner"), cx, MountedLayoutFault::NodeCredits)
            }
            LayoutJobStage::MeasureLayout | LayoutJobStage::SolveLayout => {
                let index = if self.stage == LayoutJobStage::MeasureLayout { self.measure_cursor.checked_sub(1) } else { Some(self.arrange_cursor) };
                let Self { flex, nodes, .. } = self;
                let (Some(index), Some(nodes)) = (index, nodes.as_deref()) else { return Ok(true) };
                if nodes.get(index).is_none() {
                    return Ok(true);
                }
                match flex.fund_scratch(index, Siblings::of(nodes, index), cx.retained_grant()).map_err(|_| MountedLayoutFault::Solver)? {
                    NodeFunding::Ready => Ok(true),
                    NodeFunding::Funded(progress) => {
                        cx.consume_retained(progress).map_err(|_| MountedLayoutFault::Solver)?;
                        Ok(false)
                    }
                    NodeFunding::Denied => Ok(false),
                }
            }
            _ => Ok(true),
        }
    }

    pub(super) fn admit_text_one(&mut self, tree: &UiTree) -> (usize, usize) {
        let Some(index) = self.text_node else {
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        let Some(input) = self.nodes.as_deref().expect("live layout list owner").get(index).copied() else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let Some(spec) = tree.node(input.id).map(|node| &node.spec.0) else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let slider_unit_label = match spec {
            UiNode::Slider(slider) => slider.unit_label(),
            _ => None,
        };
        let part = match (spec, self.text_part) {
            (UiNode::Text(text), 0) => Some(Some(text.value.as_str())),
            (UiNode::Button(button), 0) => Some(Some(button.label.as_str())),
            (UiNode::Section(section), 0) => Some(section.label.as_ref().map(|label| label.as_str())),
            (UiNode::Field(field), 0) => Some(Some(field.label.as_str())),
            (UiNode::Field(field), 1) => Some(field.description.as_deref()),
            (UiNode::Field(field), 2) => Some(field.error.as_deref()),
            (UiNode::Slider(_), 0) => Some(slider_unit_label.as_deref()),
            (UiNode::Text(_) | UiNode::Button(_) | UiNode::Section(_) | UiNode::Slider(_), 1..) | (UiNode::Field(_), 3..) => None,
            _ => {
                self.fault = Some(MountedLayoutFault::Stale);
                return (0, 0);
            }
        };
        let Some(value) = part else {
            let run = RetainedTextRun { node: index, glyph_start: input.glyph_start, glyph_end: self.glyphs.as_deref().expect("live layout list owner").len() };
            if let Err(owner) = self.runs.as_deref_mut().expect("live layout list owner").try_push(run) {
                self.rejected_run = Some(owner);
                self.fault = Some(MountedLayoutFault::NodeCredits);
                return (0, 0);
            }
            if let Some(node) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(index) {
                node.glyph_start = run.glyph_start;
                node.glyph_end = run.glyph_end;
            }
            self.text_node = None;
            self.text_byte = 0;
            self.text_part = 0;
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        let Some(value) = value else {
            self.record_chrome_range(index, self.text_part, None);
            self.text_part += 1;
            self.text_byte = 0;
            self.text_glyph_start = self.glyphs.as_deref().expect("live layout list owner").len();
            return (0, 0);
        };
        let Some(scalar) = value.get(self.text_byte..).and_then(|tail| tail.chars().next()) else {
            self.record_chrome_range(index, self.text_part, Some(GlyphRange { start: self.text_glyph_start, end: self.glyphs.as_deref().expect("live layout list owner").len() }));
            self.text_part += 1;
            self.text_byte = 0;
            self.text_glyph_start = self.glyphs.as_deref().expect("live layout list owner").len();
            return (0, 0);
        };
        self.text_byte += scalar.len_utf8();
        if let Err(owner) = self.glyphs.as_deref_mut().expect("live layout list owner").try_push(RetainedGlyphInput { node: index, scalar }) {
            self.rejected_glyph = Some(owner);
            self.fault = Some(MountedLayoutFault::GlyphCredits);
            return (0, 0);
        }
        (0, 1)
    }

    fn record_chrome_range(&mut self, index: usize, part: u8, range: Option<GlyphRange>) {
        let Some(node) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(index) else {
            self.fault = Some(MountedLayoutFault::Stale);
            return;
        };
        match (&mut node.chrome, part) {
            (ChromeTextRanges::Section { title }, 0) => *title = range,
            (ChromeTextRanges::Field { label, .. }, 0) => *label = range.unwrap_or_default(),
            (ChromeTextRanges::Field { description, .. }, 1) => *description = range,
            (ChromeTextRanges::Field { error, .. }, 2) => *error = range,
            (ChromeTextRanges::SliderUnit { label }, 0) => *label = range.unwrap_or_default(),
            (ChromeTextRanges::None, _) => {}
            _ => self.fault = Some(MountedLayoutFault::Stale),
        }
    }

    pub(super) fn unwind_one(&mut self, tree: &UiTree) -> (usize, usize) {
        let Some(frame) = self.walk.as_deref_mut().expect("live layout list owner").last_mut() else {
            self.admission = AdmissionPhase::Ready;
            return (0, 0);
        };
        if let Some(child) = frame.next_child {
            frame.next_child = tree.node(child).and_then(|node| node.next_sibling);
            self.pending_node = Some((child, Some(frame.node)));
            self.admission = if self.block_reversed { AdmissionPhase::VisitReversed } else { AdmissionPhase::Visit };
            return (0, 0);
        }
        self.walk.as_deref_mut().expect("live layout list owner").pop();
        (1, 0)
    }

    fn publish_fault<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        if !self.fault_payload.advance_initialization(cx)? || !self.fault_payload.seal(cx)? { return Ok(None); }
        semio_framework_job::JobOutcomeBorrow::admit_fault(cx, self.fault_payload.published().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout original fault payload was not sealed"))?)
    }

    pub(super) fn worker_one<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_value::ValueError> {
        use semio_framework_job::JobOutcomeBorrow;
        #[cfg(test)]
        {
            self.worker_thread_observed = std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-"));
        }
        if self.close_requested || cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx); }
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        if cx.should_yield() { return JobOutcomeBorrow::admit_yield(cx); }
        if self.fault.is_some() { return self.publish_fault(cx); }
        cx.set_stage(self.stage_label());
        match self.fund_worker_slot(cx) {
            Ok(true) => {}
            Ok(false) => {
                cx.consume_fuel(1);
                return JobOutcomeBorrow::admit_yield(cx);
            }
            Err(fault) => {
                self.fault = Some(fault);
                return self.publish_fault(cx);
            }
        }
        match self.stage {
            LayoutJobStage::ShapeText => self.shape_one(),
            LayoutJobStage::MeasureLayout => self.measure_one(),
            LayoutJobStage::SolveLayout => self.arrange_one(),
            LayoutJobStage::CollectResults => self.collect_one(),
            _ => (0, 0),
        };
        cx.consume_fuel(1);
        if cx.is_cancelled() { return JobOutcomeBorrow::admit_cancelled(cx); }
        if self.fault.is_some() { return self.publish_fault(cx); }
        JobOutcomeBorrow::admit_yield(cx)
    }

    pub(super) fn shape_one(&mut self) -> (usize, usize) {
        let Some(run) = self.runs.as_deref().expect("live layout list owner").get(self.run_cursor).copied() else {
            if self.flex.len() != self.nodes.as_deref().expect("live layout list owner").len() {
                self.fault = Some(MountedLayoutFault::Solver);
                return (0, 0);
            }
            self.measure_cursor = self.nodes.as_deref().expect("live layout list owner").len();
            self.stage = LayoutJobStage::MeasureLayout;
            return (0, 0);
        };
        if self.glyph_cursor < run.glyph_start {
            self.glyph_cursor = run.glyph_start;
            return (0, 0);
        }
        if self.glyph_cursor >= run.glyph_end {
            self.run_cursor += 1;
            return (0, 0);
        }
        let Some(input) = self.glyphs.as_deref().expect("live layout list owner").get(self.glyph_cursor).copied() else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let next = self.glyphs.as_deref().expect("live layout list owner").get(self.glyph_cursor + 1).filter(|_| self.glyph_cursor + 1 < run.glyph_end).map(|glyph| glyph.scalar);
        let raw_preview = self.text_worker.shape_one(input, next);
        let preview = match self.atlas_candidate.retain_one(input.scalar, self.generation, self.revision, raw_preview) {
            Ok(Some(preview)) => preview,
            Ok(None) => return (0, 0),
            Err(fault) => {
                self.fault = Some(fault);
                return (0, 0);
            }
        };
        if let Err(owner) = self.glyph_previews.as_deref_mut().expect("live layout list owner").try_push(preview) {
            self.rejected_preview = Some(owner);
            self.fault = Some(MountedLayoutFault::GlyphCredits);
            return (0, 0);
        }
        if let Some(node) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(input.node) {
            node.intrinsic.width += preview.advance + preview.kerning;
            node.intrinsic.height = node.intrinsic.height.max(preview.height);
        }
        self.glyph_cursor += 1;
        (0, 1)
    }

    /// 📏️ Measures exactly ONE node's intrinsic size, walking the admission order in REVERSE so every
    /// child is already measured when its parent is reached. Work in this grant is that node's child
    /// count, never its subtree.
    fn measure_one(&mut self) -> (usize, usize) {
        let Some(index) = self.measure_cursor.checked_sub(1) else {
            self.arrange_cursor = 0;
            let root = FlexRect { x: 0.0, y: 0.0, width: self.width, height: self.height };
            if !self.flex.set_root_box(0, root) {
                self.fault = Some(MountedLayoutFault::Solver);
            }
            self.stage = LayoutJobStage::SolveLayout;
            return (0, 0);
        };
        self.measure_cursor = index;
        let chrome = self.nodes.as_deref().expect("live layout list owner").get(index).and_then(|node| composite_chrome(node, self.glyphs.as_deref().expect("live layout list owner"), self.glyph_previews.as_deref().expect("live layout list owner"), self.width, self.theme.gap_standard));
        if let Some((top, bottom)) = chrome {
            if !self.flex.set_composite_chrome(index, top, bottom) {
                self.fault = Some(MountedLayoutFault::Solver);
                return (0, 0);
            }
        }
        let Self { flex, nodes, glyphs, glyph_previews, .. } = self;
        let (Some(nodes_ref), Some(glyphs_ref), Some(previews_ref)) = (nodes.as_deref(), glyphs.as_deref(), glyph_previews.as_deref()) else { self.fault = Some(MountedLayoutFault::Stale); return (0, 0); };
        let mut measure = |node: usize, constraint: MeasureConstraint| measure_text(nodes_ref, glyphs_ref, previews_ref, node, constraint);
        if !flex.measure_one(index, Siblings::of(nodes_ref, index), &mut measure) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        (1, 0)
    }

    /// 🗃️ Arranges exactly ONE container's direct children inside its own already-resolved box,
    /// walking the admission order FORWARD so a parent's box is always settled before its children
    /// are placed. This is the grant the eight-millisecond slice law is charged against: its cost is
    /// the container's child count, so a 1,025-node surface is ~1,025 small solves, never one large one.
    fn arrange_one(&mut self) -> (usize, usize) {
        if self.nodes.as_deref().expect("live layout list owner").get(self.arrange_cursor).is_none() {
            self.stage = LayoutJobStage::CollectResults;
            return (0, 0);
        }
        let index = self.arrange_cursor;
        self.arrange_cursor += 1;
        if self.nodes.as_deref().expect("live layout list owner").get(index).is_none_or(|node| node.first_child.is_none()) {
            return (1, 0);
        }
        let width = self.flex.rect(index).map_or(self.width, |rect| rect.width);
        let chrome = self.nodes.as_deref().expect("live layout list owner").get(index).and_then(|node| composite_chrome(node, self.glyphs.as_deref().expect("live layout list owner"), self.glyph_previews.as_deref().expect("live layout list owner"), width, self.theme.gap_standard));
        if let Some((top, bottom)) = chrome {
            if !self.flex.set_composite_chrome(index, top, bottom) {
                self.fault = Some(MountedLayoutFault::Solver);
                return (0, 0);
            }
        }
        let Self { flex, nodes, glyphs, glyph_previews, .. } = self;
        let (Some(nodes_ref), Some(glyphs_ref), Some(previews_ref)) = (nodes.as_deref(), glyphs.as_deref(), glyph_previews.as_deref()) else { self.fault = Some(MountedLayoutFault::Stale); return (0, 0); };
        let mut measure = |node: usize, constraint: MeasureConstraint| measure_text(nodes_ref, glyphs_ref, previews_ref, node, constraint);
        if !flex.arrange_one(index, Siblings::of(nodes_ref, index), &mut measure) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        (1, 0)
    }

}
//#endregion 🧵️MountedLayoutText
