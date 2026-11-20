//! 🪞️ Drawing duplicate rewrite: the duplicate reference search and the duplicate rewrite authority.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedWorkPoll, ValueError};
use store::os_store::snapshot_read::settle_close_poll;

struct DrawingDuplicateReferenceSearch {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub owners: Option<(usize, usize)>,
    pub equality: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingTextEqualityCursor,
    pub result: Option<bool>,
}

impl DrawingDuplicateReferenceSearch {
    pub fn new() -> Self { Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, equality: Default::default(), result: None } }

    pub fn node_at<'a>(root: &'a DrawingLayerNode, path: &[usize]) -> Option<&'a DrawingLayerNode> {
        let mut node = root;
        for index in path { let DrawingLayerNode::Group(group) = node else { return None };node = group.children.get(*index)?; }
        Some(node)
    }

    pub fn step(&mut self, root: &DrawingLayerNode, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        let owners = (root as *const _ as usize, target as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-reference-source-changed"); }
        self.owners = Some(owners);
        if self.result.is_some() { return Ok(true); }
        let node = Self::node_at(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-reference-path")?;
        if self.frames[self.depth].phase == 0 {
            let step = self.equality.step(semio_s_artifact_draw_drawing_model::layer_id(node), target, 1, DRAWING_OWNED_FIELD_BYTES)?;
            cx.consume_fuel(step.compared_bytes.max(1) as u64);
            if !step.complete { return Ok(false); }
            let equal = self.equality.result().ok_or("drawing-store.duplicate-reference-equality")?;
            self.equality.close_step(1);self.equality = Default::default();
            if equal { self.result = Some(true);return Ok(true); }
            self.frames[self.depth].phase = 1;
            return Ok(false);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-reference-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 { self.result = Some(false);Ok(true) } else { self.depth -= 1;cx.consume_fuel(1);Ok(false) }
    }

    pub fn close(&mut self) { self.equality.close_step(1);self.owners = None;self.result = None; }
}

pub struct DrawingDuplicateRewriteAuthority {
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub owners: Option<(usize, usize)>,
    pub identity: semio_s_artifact_draw_drawing_host_arena::native_text_footprint::DrawingDuplicateIdentityCursor,
    pub identity_index:Option<usize>,
    pub assignment_scan:usize,
    pub identities_observed:usize,
    pub collision:Option<DrawingLayerLocator>,
    pub append: semio_framework_value::paged::PagedUtf8AppendCursor,
    pub append_active: bool,
    pub append_chunk: usize,
    pub append_complete: bool,
    pub native_id: std::mem::ManuallyDrop<DrawingNativeText>,
    pub displaced: Option<DrawingDecodedFieldRetirement<DrawingNativeText>>,
    pub search: Option<DrawingDuplicateReferenceSearch>,
    pub reference: usize,
    pub mode: u8,
    pub pending_id: std::mem::ManuallyDrop<Option<String>>,
    pub pending_name: std::mem::ManuallyDrop<Option<String>>,
    pub terminal: bool,
}

impl DrawingDuplicateRewriteAuthority {
    pub fn new(mut pending_id: String, mut pending_name: String) -> Self {
        pending_id.clear();pending_name.clear();
        Self { depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], owners: None, identity: Default::default(), identity_index:None, assignment_scan:0, identities_observed:0, collision:None, append: Default::default(), append_active: false, append_chunk: 0, append_complete: false, native_id: std::mem::ManuallyDrop::new(Default::default()), displaced: None, search: None, reference: 0, mode: 0, pending_id: std::mem::ManuallyDrop::new(Some(pending_id)), pending_name: std::mem::ManuallyDrop::new(Some(pending_name)), terminal: false }
    }

    fn run_effect(cx: &mut semio_framework_job::StepContext<'_>, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, &'static str> {
        let grant = cx.retained_grant();
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| run(effect)).map_err(|_| "drawing-store.duplicate-close-work")? else {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        };
        let progress = result.as_ref().map_or_else(ValueError::retained_progress, |step| step.progress());
        let receipt = cx.consume_retained(progress);
        cx.consume_fuel(u64::try_from(work).map_err(|_| "drawing-store.duplicate-close-work")?);
        let step = result.map_err(|_| "drawing-store.duplicate-close-fault")?;
        receipt.map_err(|_| "drawing-store.duplicate-close-grant")?;
        Ok(step)
    }

    fn close_append(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        if !self.append_active { return Ok(RetainedCloneStep::Complete(Default::default())); }
        self.append.begin_close();
        let before = effect.performed_work();
        let poll = self.append.close_step(effect.grant().maximum_copy_bytes, effect);
        let unpaid = poll.work().saturating_sub(effect.performed_work() - before);
        if unpaid != 0 {
            let unpaid = u64::try_from(unpaid).map_err(|_| ValueError::invariant("Drawing duplicate append close Work overflow"))?;
            if !effect.performed(unpaid)? { return Err(ValueError::invariant("Drawing duplicate append close exceeded its Work")); }
        }
        let step = settle_close_poll(poll)?;
        if matches!(step, RetainedCloneStep::Complete(_)) && self.append.terminal_is_empty() {
            self.append = Default::default();
            self.append_active = false;
        }
        Ok(step)
    }

    fn append_text<const N: usize>(append: &mut semio_framework_value::paged::PagedUtf8AppendCursor, source: &str, output: &mut semio_framework_value::paged::PagedUtf8<N>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<RetainedCloneStep, &'static str> {
        let grant = cx.retained_grant();
        let turn = append.advance(source, output, source.len().min(grant.maximum_copy_bytes), grant);
        let progress = turn.result.as_ref().map_or_else(ValueError::retained_progress, |step| step.progress());
        let receipt = cx.consume_retained(progress);
        cx.consume_fuel(u64::try_from(turn.work).map_err(|_| "drawing-store.duplicate-append-work")?);
        let step = turn.result.map_err(|_| "drawing-store.duplicate-append-fault")?;
        receipt.map_err(|_| "drawing-store.duplicate-append-grant")?;
        Ok(step)
    }

    fn append_paged(&mut self, source: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.append_complete {
            if matches!(Self::run_effect(cx, |effect| self.close_append(effect))?, RetainedCloneStep::Complete(_)) {
                self.append_complete = false;
                self.append_chunk += 1;
            }
            return Ok(false);
        }
        let Some(chunk) = source.retained_chunks().get(self.append_chunk) else { self.append_chunk = 0;return Ok(true); };
        self.append_active = true;
        let step = Self::append_text(&mut self.append, chunk.as_str(), &mut self.native_id, cx)?;
        if matches!(step, RetainedCloneStep::Complete(_)) { self.append_complete = true; }
        Ok(false)
    }

    fn retire_displaced(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        if let Some(displaced) = self.displaced.as_mut() {
            let step = match displaced.step(effect.grant().maximum_copy_bytes, effect) {
                RetainedWorkPoll::Pending { .. } => RetainedCloneStep::Progress(displaced.owner.step_progress()),
                poll => settle_close_poll(poll)?,
            };
            if matches!(step, RetainedCloneStep::Complete(_)) && displaced.terminal_is_empty() { self.displaced = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DrawingNativeText>(), ..Default::default() };
        if !progress.fits(effect.grant()) || effect.remaining_work_units()? == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !effect.performed(1)? { return Err(ValueError::invariant("Drawing duplicate displaced owner transfer exceeded its Work")); }
        let value = std::mem::take(&mut *self.native_id);
        match DrawingDecodedFieldRetirement::try_new(value) {
            Ok(retirement) => self.displaced = Some(retirement),
            Err((error, value)) => { *self.native_id = value;return Err(error); }
        }
        Ok(RetainedCloneStep::Progress(progress))
    }

    pub fn step(&mut self, root: &mut DrawingLayerNode, original: &DrawingLayerNode, document:&DrawingSnapshot, identities:&semio_framework_value_persistent::list::PagedList<semio_s_artifact_draw_drawing_model::identity::DrawingIdentityAssignment,{usize::MAX}>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal { return Ok(true); }
        if cx.should_yield() { return Ok(false); }
        let owners = (root as *const _ as usize, original as *const _ as usize);
        if self.owners.is_some_and(|held| held != owners) { return Err("drawing-store.duplicate-root-changed"); }
        self.owners = Some(owners);
        let source = DrawingDuplicateReferenceSearch::node_at(original, &self.path[..self.depth]).ok_or("drawing-store.duplicate-source-path")?;
        let node = DrawingLayerCloneAuthority::target_at_mut(root, &self.path[..self.depth]).ok_or("drawing-store.duplicate-path")?;
        let phase = self.frames[self.depth].phase;
        if self.mode==0&&phase==0{
            if let Some(assignment)=identities.get(self.assignment_scan){
                if assignment.source==*semio_s_artifact_draw_drawing_model::layer_id(source){if self.identity_index.is_some(){return Err("drawing-store.duplicate-identity-repeated-source");}self.identity_index=Some(self.assignment_scan);}
                self.assignment_scan+=1;cx.consume_fuel(1);return Ok(false);
            }
            let identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-identity-missing")?).ok_or("drawing-store.duplicate-identity-index")?;
            if identity.target.is_empty(){return Err("drawing-store.duplicate-identity-empty");}
            let locator=self.collision.get_or_insert_with(DrawingLayerLocator::new);
            if !locator.step(document,&identity.target,cx)?{return Ok(false);}
            if locator.found().is_some(){return Err("drawing-store.mutation-duplicate-layer");}self.collision=None;self.frames[self.depth].phase=8;
        }
        let target_identity=if self.mode==0{identities.get(self.identity_index.ok_or("drawing-store.duplicate-identity-missing")?).ok_or("drawing-store.duplicate-identity-index")?}else{identities.get(self.identity_index.unwrap_or(0)).ok_or("drawing-store.duplicate-identity-missing")?};
        if self.mode==0&&self.frames[self.depth].phase==8{
            if self.assignment_scan>=identities.len(){self.assignment_scan=0;}
            if let Some(assignment)=identities.get(self.assignment_scan){
                if Some(self.assignment_scan)!=self.identity_index&&assignment.target==target_identity.target{return Err("drawing-store.duplicate-identity-repeated-target");}
                self.assignment_scan+=1;cx.consume_fuel(1);if self.assignment_scan<identities.len(){return Ok(false);}
            }
            self.frames[self.depth].phase=9;return Ok(false);
        }
        let phase=if self.frames[self.depth].phase==9{0}else{self.frames[self.depth].phase};
        if self.mode == 0 && phase < 7 {
            match phase {
                0 => {
                    let step = self.identity.step(&target_identity.target, 1, DRAWING_OWNED_FIELD_BYTES)?;
                    cx.consume_fuel(step.observed_bytes.max(1) as u64);
                    if step.complete { self.frames[self.depth].phase = 1; }
                }
                1 => {
                    let id = self.identity.identity(&target_identity.target).ok_or("drawing-store.duplicate-identity-missing")?;
                    let done = self.append_paged(id, cx)?;
                    if done { self.frames[self.depth].phase = 2; }
                }
                2 => { if matches!(Self::run_effect(cx, |effect| self.close_append(effect))?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 3; } }
                3 => {
                    std::mem::swap(&mut semio_s_artifact_draw_drawing_model::layer_base_mut(node).id, &mut self.native_id);
                    self.identity.close_step(1);self.identity = Default::default();self.identities_observed+=1;self.frames[self.depth].phase = 4;
                    cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                }
                4 => { if matches!(Self::run_effect(cx, |effect| self.retire_displaced(effect))?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 5; } }
                5 if self.depth == 0 => {
                    self.append_active = true;
                    let step = Self::append_text(&mut self.append, " copy", &mut semio_s_artifact_draw_drawing_model::layer_base_mut(node).name, cx)?;
                    if matches!(step, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 6; }
                }
                5 => { self.frames[self.depth].phase = 7;cx.consume_fuel(1); }
                6 => { if matches!(Self::run_effect(cx, |effect| self.close_append(effect))?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 7; } }
                _ => unreachable!(),
            }
            return Ok(false);
        }
        if self.mode == 1 {
            if let (DrawingLayerNode::Boolean(source), DrawingLayerNode::Boolean(node)) = (source, &mut *node) {
                if self.reference < source.children.len() {
                    let target = source.children.get(self.reference).ok_or("drawing-store.duplicate-reference-target")?;

                    match phase {
                        0 => {
                            let search = self.search.get_or_insert_with(DrawingDuplicateReferenceSearch::new);
                            if !search.step(original, target, cx)? { return Ok(false); }
                            let internal = search.result.ok_or("drawing-store.duplicate-reference-result")?;
                            search.close();self.search = None;
                            if internal { self.identity_index=None;self.assignment_scan=0;self.frames[self.depth].phase = 1; } else { self.reference += 1; }
                        }
                        1 => {
                            if let Some(assignment)=identities.get(self.assignment_scan){if assignment.source==*target{self.identity_index=Some(self.assignment_scan);}self.assignment_scan+=1;cx.consume_fuel(1);return Ok(false);}
                            let reference_identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-reference-identity-missing")?).ok_or("drawing-store.duplicate-reference-identity-index")?;
                            let step = self.identity.step(&reference_identity.target, 1, DRAWING_OWNED_FIELD_BYTES)?;
                            cx.consume_fuel(step.observed_bytes.max(1) as u64);
                            if step.complete { self.frames[self.depth].phase = 2; }
                        }
                        2 => {
                            let reference_identity=identities.get(self.identity_index.ok_or("drawing-store.duplicate-reference-identity-missing")?).ok_or("drawing-store.duplicate-reference-identity-index")?;
                            let id = self.identity.identity(&reference_identity.target).ok_or("drawing-store.duplicate-reference-identity")?;
                            let done = self.append_paged(id, cx)?;
                            if done { self.frames[self.depth].phase = 3; }
                        }
                        3 => { if matches!(Self::run_effect(cx, |effect| self.close_append(effect))?, RetainedCloneStep::Complete(_)) { self.frames[self.depth].phase = 4; } }
                        4 => {
                            let child = node.children.get_mut(self.reference).ok_or("drawing-store.duplicate-reference-destination")?;
                            std::mem::swap(child, &mut self.native_id);self.identity.close_step(1);self.identity = Default::default();self.frames[self.depth].phase = 5;
                            cx.consume_fuel(size_of::<DrawingNativeText>() as u64);
                        }
                        5 => { if matches!(Self::run_effect(cx, |effect| self.retire_displaced(effect))?, RetainedCloneStep::Complete(_)) { self.reference += 1;self.frames[self.depth].phase = 0; } }
                        _ => return Err("drawing-store.duplicate-reference-phase"),
                    }
                    return Ok(false);
                }
            }
        }
        if let DrawingLayerNode::Group(group) = source {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH { return Err("drawing-store.duplicate-depth-capacity"); }
                self.frames[self.depth].child += 1;self.path[self.depth] = child;self.identity_index=None;self.assignment_scan=0;self.depth += 1;self.frames[self.depth] = DrawingTraversalFrame::EMPTY;self.reference = 0;
                cx.consume_fuel(1);return Ok(false);
            }
        }
        if self.depth == 0 {
            if self.mode == 0 { if self.identities_observed!=identities.len(){return Err("drawing-store.duplicate-identity-census");}self.mode = 1;self.frames = [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH];self.path = [0; DRAWING_MAXIMUM_LAYER_DEPTH];self.reference = 0;cx.consume_fuel(1);return Ok(false); }
            self.identity_index=None;self.assignment_scan=0;self.identities_observed=0;self.collision=None;self.terminal = true;self.owners = None;Ok(true)
        } else { self.depth -= 1;self.reference = 0;cx.consume_fuel(1);Ok(false) }
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::{ErasedSnapshotRetirement, RetirementDemand};
        if self.terminal { return Ok(RetirementDemand::default()); }
        if !self.identity.terminal_is_empty() || self.search.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if self.append_active { return self.append.next_demand(body); }
        if let Some(displaced) = &self.displaced { return displaced.owner.next_demand(body); }
        let copy_bytes = if self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty() { 0 } else { size_of::<DrawingNativeText>() };
        Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() })
    }

    pub fn close_step(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = effect.grant();
        if effect.remaining_work_units()? == 0 || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, self.close_demands(grant.maximum_copy_bytes)?) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if !self.identity.terminal_is_empty() {
            if !effect.performed(1)? { return Err(ValueError::invariant("Drawing duplicate identity close exceeded its Work")); }
            self.identity.close_step(1);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(search) = self.search.as_mut() {
            if !effect.performed(1)? { return Err(ValueError::invariant("Drawing duplicate reference close exceeded its Work")); }
            search.close();
            self.search = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if self.append_active {
            return Ok(RetainedCloneStep::Progress(self.close_append(effect)?.progress()));
        }
        let step = self.retire_displaced(effect)?;
        if !matches!(step, RetainedCloneStep::Complete(_)) { return Ok(step); }
        if !effect.performed(1)? { return Err(ValueError::invariant("Drawing duplicate terminal close exceeded its Work")); }
        self.identity_index = None;
        self.assignment_scan = 0;
        self.identities_observed = 0;
        self.collision = None;
        self.owners = None;
        self.terminal = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    pub fn take_owners(&mut self) -> Option<(String, String)> {
        if !self.terminal || self.append_active || self.displaced.is_some() || self.search.is_some() || !self.identity.terminal_is_empty() { return None; }
        let mut id = self.pending_id.take()?;let mut name = self.pending_name.take()?;id.clear();name.clear();Some((id, name))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.pending_id.is_none() && self.pending_name.is_none() && self.owners.is_none() && !self.append_active && self.displaced.is_none() && self.search.is_none() && self.collision.is_none() && self.identity_index.is_none() && self.assignment_scan==0 && self.identities_observed==0 && self.identity.terminal_is_empty() && self.native_id.retained_chunks().allocated_bytes() == 0 && self.native_id.is_empty()
    }
}

impl Drop for DrawingDuplicateRewriteAuthority {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "Drawing duplicate rewrite reached Drop before staged id/name retirement"); }
}
