//! 🏁️ Collect, publish and close: the collection, the preview and rejection queries, the publish step, the close steps, the immediate layout, the fault display and the interactive job. See [`super`].

use super::*;

//#region 🧵️MountedLayoutText
impl MountedLayoutJob {
    /// 📐️ Reads exactly ONE solved box back out of the flex tree, in admission order so
    /// `results[i]` stays the box of `nodes[i]` — the invariant `publish_one` re-checks.
    pub(super) fn collect_one(&mut self) -> (usize, usize) {
        let Some(input) = self.nodes.as_deref().expect("live layout list owner").get(self.collect_cursor).copied() else {
            self.stage = LayoutJobStage::PublishResults;
            return (0, 0);
        };
        let Some(rect) = self.flex.rect(self.collect_cursor) else {
            self.fault = Some(MountedLayoutFault::Solver);
            return (0, 0);
        };
        let index = self.collect_cursor;
        self.collect_cursor += 1;
        if matches!(input.kind, LayoutNodeKind::Text | LayoutNodeKind::Control { label_padding: Some(_), .. } | LayoutNodeKind::OverlayRow { .. } | LayoutNodeKind::Field { .. } | LayoutNodeKind::Section { .. })
            || matches!(input.chrome, ChromeTextRanges::SliderUnit { .. })
        {
            if let Err(owner) = self.lines.as_deref_mut().expect("live layout list owner").try_push(RetainedLine { node: index, width: rect.width, height: rect.height }) {
                self.rejected_line = Some(owner);
                self.fault = Some(MountedLayoutFault::NodeCredits);
                return (0, 0);
            }
            self.line_cursor += 1;
        }
        let inline_suffix_width = if matches!(input.chrome, ChromeTextRanges::SliderUnit { .. }) { input.intrinsic.width } else { 0.0 };
        if let Err(owner) = self.results.as_deref_mut().expect("live layout list owner").try_push(MountedLayoutResult { id: input.id, x: rect.x, y: rect.y, width: rect.width, height: rect.height, inline_suffix_width }) {
            self.rejected_result = Some(owner);
            self.fault = Some(MountedLayoutFault::NodeCredits);
            return (0, 0);
        }
        (1, 0)
    }

    pub fn take_preview_one(&mut self) -> Option<MountedLayoutResult> {
        let preview = self.results.as_deref().expect("live layout list owner").get(self.preview_cursor).copied()?;
        self.preview_cursor += 1;
        Some(preview)
    }

    pub fn latest_glyph_preview(&self) -> Option<RetainedGlyphPreview> {
        self.glyph_previews.as_deref().expect("live layout list owner").get(self.glyph_cursor.saturating_sub(1)).copied()
    }

    pub fn root_intrinsic_height(&self) -> Option<f32> {
        if matches!(self.stage, LayoutJobStage::CollectNodes | LayoutJobStage::ShapeText | LayoutJobStage::MeasureLayout) {
            return None;
        }
        self.flex.intrinsic(0).map(|(_, height)| height)
    }

    #[cfg(test)]
    pub fn worker_thread_observed(&self) -> bool {
        self.worker_thread_observed
    }

    #[cfg(test)]
    pub(super) fn rejected_glyph(&self) -> Option<RetainedGlyphInput> {
        self.rejected_glyph
    }

    #[cfg(test)]
    pub(super) fn rejected_node(&self) -> Option<LayoutInputNode> {
        self.rejected_node
    }

    #[cfg(test)]
    pub(super) fn cancel_after_shape(&mut self, cancel: semio_framework_job::CancelToken) {
        self.text_worker.cancel_after_shape = Some(cancel);
    }

    pub fn publish_one(&mut self, tree: &mut UiTree, identity: (UiSurfaceToken, u64, u64, u64, u64, f32, f32)) -> LayoutJobStep {
        if self.publication_committed {
            return LayoutJobStep::Complete;
        }
        if self.identity() != identity {
            self.fault = Some(MountedLayoutFault::Stale);
            return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
        }
        let generation = identity.1;
        let Some(result) = self.results.as_deref().expect("live layout list owner").get(self.publish_cursor).copied() else {
            if self.results.as_deref().expect("live layout list owner").len() != self.nodes.as_deref().expect("live layout list owner").len() || self.glyph_previews.as_deref().expect("live layout list owner").len() != self.glyphs.as_deref().expect("live layout list owner").len() || self.lines.as_deref().expect("live layout list owner").len() != self.runs.as_deref().expect("live layout list owner").len() {
                self.fault = Some(MountedLayoutFault::Stale);
                return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
            }
            tree.commit_inactive_layout(generation);
            self.publication_committed = true;
            if let Some(root) = tree.node_mut(self.root) {
                root.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                root.flags.set(NodeFlags::SUBTREE_DIRTY, false);
                root.flags.set(NodeFlags::DIRTY_PAINT, true);
            }
            return LayoutJobStep::Complete;
        };
        if !tree.write_inactive_layout(result.id, generation, AcceptedLayout { x: result.x, y: result.y, width: result.width, height: result.height, inline_suffix_width: result.inline_suffix_width }) {
            self.fault = Some(MountedLayoutFault::Stale);
            return LayoutJobStep::Fault(MountedLayoutFault::Stale.label());
        }
        self.publish_cursor += 1;
        LayoutJobStep::Yield { stage: LayoutJobStage::PublishResults, nodes: 1, glyphs: 0 }
    }

    pub(super) fn stage_label(&self) -> &'static str {
        match self.stage {
            LayoutJobStage::CollectNodes => "Layout.CollectNodes",
            LayoutJobStage::ShapeText => "Layout.ShapeText",
            LayoutJobStage::MeasureLayout => "Layout.MeasureLayout",
            LayoutJobStage::SolveLayout => "Layout.SolveLayout",
            LayoutJobStage::CollectResults => "Layout.CollectResults",
            LayoutJobStage::PublishResults => "Layout.PublishResults",
        }
    }

    pub fn begin_close(&mut self) {
        self.close_requested = true;
    }

    pub(super) fn next_close_release_bytes(&self)->Result<usize,semio_framework_value::ValueError>{
        if self.rejected_result.is_some()||self.rejected_line.is_some()||self.rejected_glyph.is_some()||self.rejected_preview.is_some()||self.rejected_run.is_some()||self.rejected_walk.is_some()||self.rejected_node.is_some(){return Ok(0)}
        if self.results.as_deref().is_some_and(|owner|!owner.is_empty())||self.lines.as_deref().is_some_and(|owner|!owner.is_empty())||self.glyph_previews.as_deref().is_some_and(|owner|!owner.is_empty())||self.glyphs.as_deref().is_some_and(|owner|!owner.is_empty())||self.runs.as_deref().is_some_and(|owner|!owner.is_empty())||self.walk.as_deref().is_some_and(|owner|!owner.is_empty())||self.nodes.as_deref().is_some_and(|owner|!owner.is_empty()){return Ok(0)}
        fn list<T,const N:usize>(owner:&ui_contract::UiFixedList<T,N>)->Result<usize,semio_framework_value::ValueError>{if !owner.is_empty(){Ok(0)}else if !owner.terminal_is_empty(){owner.next_release_allocation_bytes().map_err(semio_framework_value::ValueError::from)}else{Ok(size_of::<ui_contract::UiFixedList<T,N>>())}}
        if let Some(owner)=self.results.as_deref(){return list(owner)}
        if let Some(owner)=self.lines.as_deref(){return list(owner)}
        if let Some(owner)=self.glyph_previews.as_deref(){return list(owner)}
        if let Some(owner)=self.glyphs.as_deref(){return list(owner)}
        if let Some(owner)=self.runs.as_deref(){return list(owner)}
        if let Some(owner)=self.walk.as_deref(){return list(owner)}
        if let Some(owner)=self.nodes.as_deref(){return list(owner)}
        if self.child_scratch.capacity()!=0{return self.child_scratch.capacity().checked_mul(size_of::<usize>()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"layout scratch backing overflow"))}
        if !self.flex.close_is_empty(){return self.flex.next_close_release_byte_demand()}
        if self.atlas_candidate.pages.iter().any(Option::is_some){return Ok(LAYOUT_ATLAS_PAGE_BYTES)}
        self.text_worker.next_close_release_bytes()
    }
    fn next_close_depth(&self)->Result<usize,semio_framework_value::ValueError>{
        if !self.fault_payload.terminal_is_empty(){return self.fault_payload.retirement_demands().map(|demand|demand.depth)}
        if self.rejected_result.is_some()||self.rejected_line.is_some()||self.rejected_glyph.is_some()||self.rejected_preview.is_some()||self.rejected_run.is_some()||self.rejected_walk.is_some()||self.rejected_node.is_some(){return Ok(1)}
        fn depth<T,const N:usize>(owner:&ui_contract::UiFixedList<T,N>,pop:bool)->Result<usize,semio_framework_value::ValueError>{
            if owner.terminal_is_empty(){return Ok(1)}let depth=if pop{owner.next_pop_depth_demand()}else{owner.next_release_depth_demand()}.map_err(semio_framework_value::ValueError::from)?;depth.checked_add(1).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original mounted list parent depth overflow"))
        }
        macro_rules! quote_pop{($($field:ident),+)=>{$(if let Some(owner)=self.$field.as_deref().filter(|owner|!owner.is_empty()){return depth(owner,true)})+}}
        quote_pop!(results,lines,glyph_previews,glyphs,runs,walk,nodes);
        macro_rules! quote_release{($($field:ident),+)=>{$(if let Some(owner)=self.$field.as_deref(){return depth(owner,false)})+}}
        quote_release!(results,lines,glyph_previews,glyphs,runs,walk,nodes);
        if self.child_scratch.capacity()!=0{return Ok(1)}if !self.flex.close_is_empty(){return self.flex.next_close_depth_demand()}Ok(usize::from(!self.terminal_is_empty()))
    }
    fn close_granted(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_value::ValueError>{
        use semio_framework_job::{InteractiveJobCloseStep as Step,RetainedCloneProgress};
        let empty=RetainedCloneProgress::default();if !self.close_requested{return Ok(Step::Blocked)}if self.terminal_is_empty(){return Ok(Step::Complete{progress:empty})}
        if grant.maximum_items==0{return Ok(Step::Pending{progress:empty})}
        if !self.fault_payload.terminal_is_empty(){return match self.fault_payload.close_step_granted(grant){Ok(step)=>Ok(Step::Pending{progress:step.progress()}),Err(error)=>Err(error)}}
        let bytes=match self.next_close_release_bytes(){Ok(bytes)=>bytes,Err(error)=>return Err(error)};
        let depth=self.next_close_depth()?;if bytes>grant.maximum_release_bytes||grant.maximum_depth<depth{return Ok(Step::Pending{progress:empty})}
        if self.rejected_result.take().is_some()||self.rejected_line.take().is_some()||self.rejected_glyph.take().is_some()||self.rejected_preview.take().is_some()||self.rejected_run.take().is_some()||self.rejected_walk.take().is_some()||self.rejected_node.take().is_some(){return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}
        macro_rules! close_item{($field:ident)=>{if let Some(owner)=self.$field.as_deref_mut(){if owner.pop().is_some(){return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})}}}}
        close_item!(results);close_item!(lines);close_item!(glyph_previews);close_item!(glyphs);close_item!(runs);close_item!(walk);close_item!(nodes);
        macro_rules! close_list{($field:ident)=>{if let Some(owner)=self.$field.as_deref_mut(){
            if !owner.terminal_is_empty(){return match owner.release_empty_page(grant.maximum_release_bytes){Ok(receipt)=>Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:usize::from(receipt.progressed),released_bytes:receipt.released_allocation_bytes,..Default::default()}}),Err(error)=>{let error=semio_framework_value::ValueError::from(error);Err(error)}}}
            self.$field.take();return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})
        }}}
        close_list!(results);close_list!(lines);close_list!(glyph_previews);close_list!(glyphs);close_list!(runs);close_list!(walk);close_list!(nodes);
        if self.child_scratch.capacity()!=0{drop(std::mem::take(&mut self.child_scratch));return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})}
        if !self.flex.close_is_empty(){return self.flex.close_granted(grant)}
        if !self.atlas_candidate.is_empty(){self.atlas_candidate.close_one();return Ok(Step::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()}})}
        Ok(if self.text_worker.terminal_is_empty(){Step::Complete{progress:empty}}else{Step::Blocked})
    }
    #[cfg(any(test, feature = "testkit"))]
    pub fn close_one(&mut self,grant:semio_framework_job::RetainedCloneGrant)->Result<bool,semio_framework_value::ValueError>{
        Ok(matches!(self.close_granted(grant)?,semio_framework_job::InteractiveJobCloseStep::Complete{..}))
    }

    pub fn terminal_is_empty(&self)->bool{
        self.layout_is_empty()&&self.text_worker.terminal_is_empty()
    }
    fn layout_is_empty(&self)->bool{
        self.fault_payload.terminal_is_empty()&&self.rejected_result.is_none()&&self.rejected_line.is_none()&&self.rejected_glyph.is_none()&&self.rejected_preview.is_none()&&self.rejected_run.is_none()&&self.rejected_walk.is_none()&&self.rejected_node.is_none()
        &&self.results.is_none()&&self.lines.is_none()&&self.glyph_previews.is_none()&&self.glyphs.is_none()&&self.runs.is_none()&&self.walk.is_none()&&self.nodes.is_none()
        &&self.child_scratch.capacity()==0&&self.flex.close_is_empty()&&self.atlas_candidate.is_empty()
    }

}

/// 🧪️ Drives ONE whole bounded layout pass to completion in a single call and mirrors the solved
/// boxes into both the accepted (double-buffered) layout the painter reads and the immediate-mode
/// `LayoutBucket` older probes read. It drives the REAL [`MountedLayoutJob`] stage ladder rather than
/// re-deriving geometry, so a suite laying a tree out this way can never disagree with the shipped
/// renderer, which walks the identical stages one grant at a time through `engine::Ui`.
#[cfg(any(test, feature = "testkit"))]
pub fn layout_tree_now(tree: &mut UiTree, root: NodeId, theme: Theme, width: f32, height: f32) -> bool {
    let identity = MountedLayoutIdentity { surface: UiSurfaceToken::new(0, 1), generation: 1, revision: 0, theme_revision: 0, viewport_revision: 0 };
    let Ok(mut job) = MountedLayoutJob::try_new(tree, root, identity, theme, width, height, false, ui_contract::FlowInline::Ltr) else { return false };
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview = 0;
    while !job.is_admitted() {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(0), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), || Some(0), &mut preview,&mut actual_retained_progress);
        if matches!(job.admit_one(tree, &mut cx), LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled) {
            return false;
        }
    }
    while job.stage() != LayoutJobStage::PublishResults {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(0), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), || Some(0), &mut preview,&mut actual_retained_progress);
        let outcome = match semio_framework_job::InteractiveJob::step(&mut job, &mut cx) { Ok(outcome) => outcome.map(semio_framework_job::JobOutcomeBorrow::into_descriptor), Err(_) => return false };
        if let Some(mut descriptor) = outcome {
            let failed = matches!(descriptor.kind(), semio_framework_job::JobOutcomeKind::Fault | semio_framework_job::JobOutcomeKind::Cancelled);
            descriptor.acknowledge(ui_contract::UI_WORKER_RETIREMENT_POLICY);
            if failed { job.begin_close(); while !job.close_one(ui_contract::UI_WORKER_RETIREMENT_POLICY).unwrap() {} return false; }
        }
    }
    while let Some(result) = job.take_preview_one() {
        if let Some(node) = tree.node_mut(result.id) {
            node.layout.x = result.x;
            node.layout.y = result.y;
            node.layout.width = result.width;
            node.layout.height = result.height;
            node.layout.inline_suffix_width = result.inline_suffix_width;
        }
    }
    let published = job.identity();
    loop {
        match job.publish_one(tree, published) {
            LayoutJobStep::Complete => break,
            LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled => return false,
            LayoutJobStep::Yield { .. } => {}
        }
    }
    job.begin_close();
    while !job.close_one(ui_contract::UI_WORKER_RETIREMENT_POLICY).unwrap() {}
    true
}

impl MountedLayoutFault {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::NodeCredits => "layout.node-credits",
            Self::GlyphCredits => "layout.glyph-credits",
            Self::DepthCredits => "layout.depth-credits",
            Self::Stale => "layout.stale",
            Self::Solver => "layout.solver",
        }
    }
}

impl semio_framework_job::InteractiveJob for MountedLayoutJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        self.worker_one(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        match descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => descriptor.yielded(),
            semio_framework_job::JobOutcomeKind::Cancelled => descriptor.cancelled(),
            semio_framework_job::JobOutcomeKind::Fault if self.fault.is_some() => descriptor.fault(self.fault_payload.published().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout original fault custody was not published"))?),
            _ => Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "layout descriptor does not match its original outcome")),
        }
    }

    fn begin_close(&mut self) {
        MountedLayoutJob::begin_close(self);
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()})}
        if self.close_requested&&self.layout_is_empty(){return self.text_worker.close_step(cx)}
        let step=self.close_granted(grant).and_then(|step|step.admit(grant,self.terminal_is_empty())).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        if step.progress().copied_items!=0{cx.consume_fuel(1)}
        Ok(step)
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{if self.fault_payload.terminal_is_empty(){Ok(0)}else{self.fault_payload.retirement_demands().map(|demand|demand.copy_bytes)}}
    fn next_close_capacity_byte_demand(&self,_copy:usize)->Result<usize,semio_framework_value::ValueError>{if self.fault_payload.terminal_is_empty(){Ok(0)}else{self.fault_payload.retirement_demands().map(|demand|demand.capacity_bytes)}}
    fn next_close_release_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.next_close_release_bytes()}
    fn next_close_depth_demand(&self)->Result<usize,semio_framework_value::ValueError>{self.next_close_depth()}

    fn terminal_is_empty(&self) -> bool {
        MountedLayoutJob::terminal_is_empty(self)
    }
}
//#endregion 🧵️MountedLayoutText
