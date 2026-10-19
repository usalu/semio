//! 🧱️ Job state and admission: the layout result, the admission phase, the mounted layout job and identity and the job constructor with the node admission. See [`super`].

use super::*;

//#region 🧵️MountedLayoutText
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MountedLayoutResult {
    pub id: NodeId,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inline_suffix_width: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AdmissionPhase {
    Visit,
    VisitReversed,
    Text,
    Unwind,
    Ready,
}

#[derive(Clone, Copy)]
struct WalkFrame {
    pub(super) next_child: Option<NodeId>,
    pub(super) node: usize,
}

/// 📦️ Exact fixed working-set owner retained across admission, worker, preview, and close.
pub struct MountedLayoutJob {
    pub(super) stage: LayoutJobStage,
    pub(super) root: NodeId,
    pub(super) surface: UiSurfaceToken,
    pub(super) generation: u64,
    pub(super) revision: u64,
    pub(super) theme_revision: u64,
    pub(super) viewport_revision: u64,
    pub(super) theme: Theme,
    pub(super) row_metrics: TreeRowMetrics,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) block_reversed: bool,
    pub(super) admission: AdmissionPhase,
    pub(super) pending_node: Option<(NodeId, Option<usize>)>,
    pub(super) walk: Option<Box<ui_contract::UiFixedList<WalkFrame, LAYOUT_DEPTH_CREDITS>>>,
    pub(super) nodes: Option<Box<ui_contract::UiFixedList<LayoutInputNode, LAYOUT_NODE_CREDITS>>>,
    pub(super) glyphs: Option<Box<ui_contract::UiFixedList<RetainedGlyphInput, LAYOUT_GLYPH_CREDITS>>>,
    pub(super) runs: Option<Box<ui_contract::UiFixedList<RetainedTextRun, LAYOUT_NODE_CREDITS>>>,
    pub(super) lines: Option<Box<ui_contract::UiFixedList<RetainedLine, LAYOUT_NODE_CREDITS>>>,
    pub(super) glyph_previews: Option<Box<ui_contract::UiFixedList<RetainedGlyphPreview, LAYOUT_GLYPH_CREDITS>>>,
    pub(super) atlas_candidate: RetainedAtlasCandidate,
    pub(super) results: Option<Box<ui_contract::UiFixedList<MountedLayoutResult, LAYOUT_NODE_CREDITS>>>,
    pub(super) rejected_node: Option<LayoutInputNode>,
    pub(super) rejected_walk: Option<WalkFrame>,
    pub(super) rejected_glyph: Option<RetainedGlyphInput>,
    pub(super) rejected_preview: Option<RetainedGlyphPreview>,
    pub(super) rejected_run: Option<RetainedTextRun>,
    pub(super) rejected_line: Option<RetainedLine>,
    pub(super) rejected_result: Option<MountedLayoutResult>,
    pub(super) text_node: Option<usize>,
    pub(super) text_byte: usize,
    pub(super) text_part: u8,
    pub(super) text_glyph_start: usize,
    pub(super) run_cursor: usize,
    pub(super) glyph_cursor: usize,
    pub(super) line_cursor: usize,
    pub(super) measure_cursor: usize,
    pub(super) arrange_cursor: usize,
    pub(super) collect_cursor: usize,
    pub(super) child_scratch: Vec<usize>,
    pub(super) flex: FlexTree,
    pub(super) preview_cursor: usize,
    pub(super) publish_cursor: usize,
    pub(super) publication_committed: bool,
    pub(super) close_requested: bool,
    #[cfg(test)]
    pub(super) worker_thread_observed: bool,
    pub(super) fault: Option<MountedLayoutFault>,
    pub(super) fault_payload: semio_framework_job_payload::RetainedPayloadBuilder,
    pub(super) text_worker: DeterministicTextWorker,
}

#[derive(Clone, Copy)]
pub struct MountedLayoutIdentity {
    pub surface: UiSurfaceToken,
    pub generation: u64,
    pub revision: u64,
    pub theme_revision: u64,
    pub viewport_revision: u64,
}

impl MountedLayoutJob {
    pub fn try_new(tree: &UiTree, root: NodeId, identity: MountedLayoutIdentity, theme: Theme, width: f32, height: f32, block_reversed: bool, inline: ui_contract::FlowInline) -> Result<Self, MountedLayoutFault> {
        let MountedLayoutIdentity { surface, generation, revision, theme_revision, viewport_revision } = identity;
        let root_node = tree.node(root).ok_or(MountedLayoutFault::Stale)?;
        if !root_node.flags.contains(NodeFlags::DIRTY_LAYOUT) && !root_node.flags.contains(NodeFlags::SUBTREE_DIRTY) {
            return Err(MountedLayoutFault::Stale);
        }
        Ok(Self {
            stage: LayoutJobStage::CollectNodes,
            root,
            surface,
            generation,
            revision,
            theme_revision,
            viewport_revision,
            row_metrics: TreeRowMetrics::from_theme(&theme).with_inline(inline),
            theme,
            width,
            height,
            block_reversed,
            admission: if block_reversed { AdmissionPhase::VisitReversed } else { AdmissionPhase::Visit },
            pending_node: Some((root, None)),
            walk: Some(Box::new(ui_contract::UiFixedList::default())),
            nodes: Some(Box::new(ui_contract::UiFixedList::default())),
            glyphs: Some(Box::new(ui_contract::UiFixedList::default())),
            runs: Some(Box::new(ui_contract::UiFixedList::default())),
            lines: Some(Box::new(ui_contract::UiFixedList::default())),
            glyph_previews: Some(Box::new(ui_contract::UiFixedList::default())),
            atlas_candidate: RetainedAtlasCandidate::new(),
            results: Some(Box::new(ui_contract::UiFixedList::default())),
            rejected_node: None,
            rejected_walk: None,
            rejected_glyph: None,
            rejected_preview: None,
            rejected_run: None,
            rejected_line: None,
            rejected_result: None,
            text_node: None,
            text_byte: 0,
            text_part: 0,
            text_glyph_start: 0,
            run_cursor: 0,
            glyph_cursor: 0,
            line_cursor: 0,
            measure_cursor: 0,
            arrange_cursor: 0,
            collect_cursor: 0,
            child_scratch: Vec::new(),
            flex: FlexTree::new(),
            preview_cursor: 0,
            publish_cursor: 0,
            publication_committed: false,
            close_requested: false,
            #[cfg(test)]
            worker_thread_observed: false,
            fault: None,
            fault_payload: semio_framework_job_payload::RetainedPayloadBuilder::new(semio_framework_job_payload::JobPayloadStream::Fault),
            text_worker: DeterministicTextWorker::default(),
        })
    }

    pub fn identity(&self) -> (UiSurfaceToken, u64, u64, u64, u64, f32, f32) {
        (self.surface, self.generation, self.revision, self.theme_revision, self.viewport_revision, self.width, self.height)
    }

    pub fn stage(&self) -> LayoutJobStage {
        self.stage
    }

    pub fn is_admitted(&self) -> bool {
        self.stage != LayoutJobStage::CollectNodes
    }

    pub fn admit_one(&mut self, tree: &UiTree, cx: &mut semio_framework_job::StepContext<'_>) -> LayoutJobStep {
        if cx.is_cancelled() {
            return LayoutJobStep::Cancelled;
        }
        if cx.should_yield() {
            return LayoutJobStep::Yield { stage: self.stage, nodes: 0, glyphs: 0 };
        }
        cx.set_stage("Layout.AdmitOne");
        let progress = match self.admission {
            AdmissionPhase::Visit => self.admit_node_one(tree, false),
            AdmissionPhase::VisitReversed => self.admit_node_one(tree, true),
            AdmissionPhase::Text => self.admit_text_one(tree),
            AdmissionPhase::Unwind => self.unwind_one(tree),
            AdmissionPhase::Ready => {
                self.stage = LayoutJobStage::ShapeText;
                (0, 0)
            }
        };
        cx.consume_fuel(1);
        if let Some(fault) = self.fault {
            return LayoutJobStep::Fault(fault.label());
        }
        LayoutJobStep::Yield { stage: self.stage, nodes: progress.0, glyphs: progress.1 }
    }

    /// 🧩️ The panel content-projection ESCAPE HATCH. An `ExternalSlot` is the one node whose
    /// pixels this engine does not author: its host paints inside the solved rect. It used to
    /// fall into `Leaf`, which measures from arena children — a slot has none, so the band
    /// collapsed to nothing and any host-provided content (React hosts an arbitrary subtree
    /// here through `Tree`'s `emptyState`: an agent chat transcript, a marketplace list) had no
    /// box at all. The host declares its band in the slot's own `params_json`
    /// (`{"hostContentHeight": <px>}`), so the reservation costs one node and no measurement,
    /// and every other panel's node credit is untouched.
    fn admit_node_one(&mut self, tree: &UiTree, root_reversed: bool) -> (usize, usize) {
        let Some((id, parent)) = self.pending_node.take() else {
            self.admission = AdmissionPhase::Unwind;
            return (0, 0);
        };
        let Some(node) = tree.node(id) else {
            self.fault = Some(MountedLayoutFault::Stale);
            return (0, 0);
        };
        let parent_kind = parent.and_then(|index| self.nodes.as_deref().expect("live layout list owner").get(index)).map(|input| input.kind);
        let content = self.tree_content(tree, id, parent);
        let tree_inline_control = matches!(parent_kind, Some(LayoutNodeKind::TreeRow { .. } | LayoutNodeKind::TableRow { .. }));
        let tree_detail = parent.and_then(|index| self.nodes.as_deref().expect("live layout list owner").get(index)).is_some_and(|owner| is_tree_item_detail(tree, id, owner.id));
        let popup_overlay_row = tree.is_open_select_popup_row(id);
        let kind = if popup_overlay_row {
            LayoutNodeKind::OverlayRow { rect: FlexRect { x: node.layout.x, y: node.layout.y, width: node.layout.width, height: node.layout.height } }
        } else {
            if tree_detail {
                LayoutNodeKind::TreeDetail { height: TREE_DETAIL_HEIGHT }
            } else {
                match &node.spec.0 {
                    UiNode::Text(_) => LayoutNodeKind::Text,
                    UiNode::Tree(tree_node) => {
                        LayoutNodeKind::Tree { height: retained_tree_height(tree, id, tree_node, &self.row_metrics), header: if document_table(tree, id).is_some() { self.row_metrics.header_height } else { 0.0 }, reversed: root_reversed }
                    }
                    UiNode::Stack(stack) => content.is_none().then(|| tree_row_kind(tree, id, parent_kind, &self.row_metrics)).flatten().unwrap_or(LayoutNodeKind::Stack {
                        horizontal: stack.direction == "horizontal",
                        gap: gap_for_token(&self.theme, stack.gap.as_deref()),
                        padding: padding_for_token(&self.theme, stack.padding.as_deref()),
                    }),
                    UiNode::Field(_) | UiNode::Section(_) if content.is_some() => LayoutNodeKind::Leaf,
                    UiNode::Field(_) => LayoutNodeKind::Field { top: 0.0, bottom: 0.0 },
                    UiNode::Section(_) => LayoutNodeKind::Section { gap: self.theme.gap_standard, top: 0.0, bottom: 0.0 },
                    UiNode::Button(_) => LayoutNodeKind::Control { height: self.theme.control_height, label_padding: Some(self.theme.padding_standard) },
                    UiNode::Input(_) | UiNode::Select(_) => LayoutNodeKind::Control { height: if tree_inline_control { self.theme.control_height_small } else { self.theme.control_height }, label_padding: None },
                    UiNode::Toggle(_) | UiNode::Slider(_) | UiNode::NumberStepper(_) | UiNode::Ring(_) | UiNode::IconSelect(_) => LayoutNodeKind::Control { height: self.theme.control_height, label_padding: None },
                    UiNode::ExternalSlot(slot) => LayoutNodeKind::HostContent { height: host_content_height(&slot.params_json, &self.theme) },
                    UiNode::ComponentScene(_) => LayoutNodeKind::EngineSurface,
                    UiNode::Progress(_) => LayoutNodeKind::Control { height: semio_framework_ui_wgpu_layout::chrome::SIZE_TINY, label_padding: None },
                    _ => LayoutNodeKind::Leaf,
                }
            }
        };
        let index = self.nodes.as_deref().expect("live layout list owner").len();
        let chrome = match &node.spec.0 {
            UiNode::Section(_) | UiNode::Field(_) if content.is_some() => ChromeTextRanges::None,
            UiNode::Section(section) => ChromeTextRanges::Section { title: section.label.as_ref().map(|_| GlyphRange::default()) },
            UiNode::Field(field) => ChromeTextRanges::Field { label: GlyphRange::default(), description: field.description.as_ref().map(|_| GlyphRange::default()), error: field.error.as_ref().map(|_| GlyphRange::default()) },
            UiNode::Slider(slider) if slider.unit.as_deref().is_some_and(|unit| !unit.is_empty()) => ChromeTextRanges::SliderUnit { label: GlyphRange::default() },
            _ => ChromeTextRanges::None,
        };
        let input = LayoutInputNode { id, parent, first_child: None, last_child: None, next_sibling: None, kind, intrinsic: IntrinsicSize::default(), glyph_start: 0, glyph_end: 0, chrome, content };
        if let Err(owner) = self.nodes.as_deref_mut().expect("live layout list owner").try_push(input) {
            self.rejected_node = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        let metrics = retained_tree_row_metrics(tree, id, &self.row_metrics);
        if !self.flex.push(kind, parent, node.layout_spec.as_ref(), &metrics, parent_kind, content) {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        }
        if let Some(parent) = parent {
            let previous = self.nodes.as_deref().expect("live layout list owner").get(parent).and_then(|owner| owner.last_child);
            match previous {
                Some(sibling) => {
                    if let Some(sibling) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(sibling) {
                        sibling.next_sibling = Some(index);
                    }
                }
                None => {
                    if let Some(owner) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(parent) {
                        owner.first_child = Some(index);
                    }
                }
            }
            if let Some(owner) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(parent) {
                owner.last_child = Some(index);
            }
        }
        let expanded = matches!(kind, LayoutNodeKind::TreeRow { .. } | LayoutNodeKind::TreeSection { .. }) || tree.disclosure_open(id).unwrap_or(true);
        if let Err(owner) = self.walk.as_deref_mut().expect("live layout list owner").try_push(WalkFrame { next_child: expanded.then_some(node.first_child).flatten(), node: index }) {
            self.rejected_walk = Some(owner);
            self.fault = Some(MountedLayoutFault::DepthCredits);
            return (0, 0);
        }
        if popup_overlay_row || !matches!(chrome, ChromeTextRanges::None) || matches!(kind, LayoutNodeKind::Text | LayoutNodeKind::Control { label_padding: Some(_), .. }) {
            self.text_node = Some(index);
            self.text_byte = 0;
            self.text_part = 0;
            self.text_glyph_start = self.glyphs.as_deref().expect("live layout list owner").len();
            if let Some(node) = self.nodes.as_deref_mut().expect("live layout list owner").get_mut(index) {
                node.glyph_start = self.glyphs.as_deref().expect("live layout list owner").len();
            }
            self.admission = AdmissionPhase::Text;
        } else {
            self.admission = AdmissionPhase::Unwind;
        }
        (1, 0)
    }

}
//#endregion 🧵️MountedLayoutText
