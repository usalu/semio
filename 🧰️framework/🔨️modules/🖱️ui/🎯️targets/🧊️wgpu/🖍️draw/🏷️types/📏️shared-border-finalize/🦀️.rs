//! 📏 Incremental ownership and ordered expansion of semantic border records.

use super::*;
use semio_framework_job::RetainedCloneProgress;
use semio_framework_value_paged_list::PagedList;

type BorderPages<T> = PagedList<T, 262144>;

fn reserve_one<T>(values: &mut Vec<T>, progress: &mut RetainedCloneProgress) -> Result<(), RetainedOutputError> {
    let old = values.capacity();
    if values.len() == old {
        let capacity = old.max(2).checked_mul(2).ok_or(RetainedOutputError::Capacity)?;
        values.try_reserve_exact(capacity - values.len()).map_err(|_| RetainedOutputError::Allocation)?;
    }
    if values.capacity() != old {
        progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(values.capacity().saturating_mul(size_of::<T>()));
        progress.released_bytes = progress.released_bytes.saturating_add(old.saturating_mul(size_of::<T>()));
        progress.copied_bytes = progress.copied_bytes.saturating_add(values.len().saturating_mul(size_of::<T>()));
    }
    Ok(())
}

fn copied<T>(progress: &mut RetainedCloneProgress) { progress.copied_bytes = progress.copied_bytes.saturating_add(size_of::<T>()); }
fn include_progress(progress: &mut RetainedCloneProgress, child: RetainedCloneProgress) {
    progress.copied_bytes = progress.copied_bytes.saturating_add(child.copied_bytes);
    progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(child.retained_capacity_bytes);
    progress.released_bytes = progress.released_bytes.saturating_add(child.released_bytes);
}
fn reserve_page<T>(values: &mut BorderPages<T>, progress: &mut RetainedCloneProgress) -> Result<(), RetainedOutputError> {
    let bytes = values.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
    let copied = values.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
    match values.reserve_one_cold(bytes) {
        Ok(receipt) => { progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(receipt.allocated_bytes); progress.released_bytes = progress.released_bytes.saturating_add(receipt.released_allocation_bytes); if receipt.progressed { progress.copied_bytes = progress.copied_bytes.saturating_add(copied); Ok(()) } else { Err(RetainedOutputError::Capacity) } }
        Err(error) => { progress.retained_capacity_bytes = progress.retained_capacity_bytes.saturating_add(error.allocated_bytes); Err(RetainedOutputError::Allocation) }
    }
}
fn retire_page<T>(values: &mut BorderPages<T>, progress: &mut RetainedCloneProgress) -> Result<bool, RetainedOutputError> {
    if values.pop().is_some() { copied::<T>(progress); return Ok(false); }
    if values.terminal_is_empty() { return Ok(true); }
    let bytes = values.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
    let receipt = values.release_empty_page_leased(bytes).map_err(|_| RetainedOutputError::Capacity)?;
    progress.released_bytes = progress.released_bytes.saturating_add(receipt.released_allocation_bytes);
    Ok(values.terminal_is_empty())
}

#[derive(Clone, Copy)]
struct GeneratedBorder { anchor: usize, owner: usize, overlay: bool, instance: UiInstance }

#[derive(Clone)]
pub(super) struct BorderFinalize {
    layer: usize,
    overlay: bool,
    phase: u8,
    seed: usize,
    seed_layer: usize,
    seeded: bool,
    origins: BorderPages<(usize, usize)>,
    clip: Option<SharedBorderClipCursor>,
    clip_piece: usize,
    clip_phase: u8,
    cursor: SharedBorderOwnershipCursor,
    scan: usize,
    mark_layer: usize,
    cleanup: bool,
    generated: BorderPages<Option<GeneratedBorder>>,
    pending_segment: Option<SharedBorderSegment>,
    pending_generated: Option<GeneratedBorder>,
    selected: Option<GeneratedBorder>,
    insertion: Option<DrawLayer>,
    insertion_clip: usize,
    insertion_at: usize,
    insertion_scan: usize,
    insertion_phase: u8,
    selected_index: Option<usize>,
    base_layers_capacity: usize,
    permanent_bytes: usize,
    released_baseline: usize,
    progress: RetainedCloneProgress,
}

impl BorderFinalize {
    pub(super) fn rebased_clone(&self, base_layers_capacity: usize) -> Self {
        let mut cloned = self.clone();
        cloned.base_layers_capacity = base_layers_capacity;
        cloned.permanent_bytes = 0;
        cloned.released_baseline = 0;
        cloned
    }

    fn new(base_layers_capacity: usize) -> Self {
        Self { layer: 0, overlay: false, phase: 8, seed: 0, seed_layer: 0, seeded: false, origins: BorderPages::new(), clip: None, clip_piece: 0, clip_phase: 0, cursor: SharedBorderOwnershipCursor::new(), scan: 0, mark_layer: 0, cleanup: false, generated: BorderPages::new(), pending_segment: None, pending_generated: None, selected: None, insertion: None, insertion_clip: 0, insertion_at: 0, insertion_scan: 0, insertion_phase: 0, selected_index: None, base_layers_capacity, permanent_bytes: 0, released_baseline: 0, progress: RetainedCloneProgress::default() }
    }

    pub(super) fn next_release_bytes(&self) -> Result<usize, RetainedOutputError> {
        if let Some(clip) = &self.clip { return clip.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity); }
        if let Some(layer) = &self.insertion {
            if let Some(clip) = &layer.clip { return if clip.scissors.is_empty() { clip.scissors.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity) } else { Ok(0) }; }
            if !layer.ui_instances.is_empty() || !layer.overlay_ui_instances.is_empty() { return Ok(0); }
            if layer.ui_instances.capacity() != 0 { return Ok(layer.ui_instances.capacity() * size_of::<UiInstance>()); }
            return Ok(layer.overlay_ui_instances.capacity() * size_of::<UiInstance>());
        }
        if !self.generated.is_empty() { return Ok(0); }
        if !self.generated.terminal_is_empty() { return self.generated.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity); }
        if !self.origins.is_empty() { return Ok(0); }
        if !self.origins.terminal_is_empty() { return self.origins.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity); }
        Ok(self.cursor.next_retirement_release_byte_demand())
    }

    pub(super) fn next_retirement_depth(&self) -> Result<usize, RetainedOutputError> {
        fn depth<T>(values: &BorderPages<T>) -> Result<usize, RetainedOutputError> {
            if !values.is_empty() { values.next_pop_depth_demand().map_err(|_| RetainedOutputError::Capacity) }
            else if !values.terminal_is_empty() { values.next_release_depth_demand().map_err(|_| RetainedOutputError::Capacity) }
            else { Ok(0) }
        }
        if let Some(clip) = &self.clip { return clip.next_retirement_depth_demand().map_err(|_| RetainedOutputError::Capacity); }
        if let Some(layer) = &self.insertion {
            if let Some(clip) = &layer.clip { return Ok(depth(&clip.scissors)?.max(1)); }
            return Ok(1);
        }
        if !self.generated.terminal_is_empty() { return Ok(depth(&self.generated)?.max(1)); }
        if !self.origins.terminal_is_empty() { return Ok(depth(&self.origins)?.max(1)); }
        Ok(self.cursor.next_retirement_depth_demand().max(1))
    }

    pub(super) fn retire_step(&mut self) -> bool {
        if let Some(clip) = &mut self.clip { if clip.retire_step().expect("valid shared-border clip retirement") { self.clip = None; } return false; }
        if let Some(layer) = &mut self.insertion {
            if let Some(clip) = &mut layer.clip {
                if clip.scissors.pop().is_some() { return false; }
                if !clip.scissors.terminal_is_empty() { let bytes = clip.scissors.next_release_allocation_bytes().expect("valid shared-border clip backing"); clip.scissors.release_empty_page_leased(bytes).expect("valid shared-border clip release"); return false; }
                layer.clip = None; return false;
            }
            if layer.ui_instances.pop().is_some() || layer.overlay_ui_instances.pop().is_some() { return false; }
            if layer.ui_instances.capacity() != 0 { layer.ui_instances = Vec::new(); return false; }
            if layer.overlay_ui_instances.capacity() != 0 { layer.overlay_ui_instances = Vec::new(); return false; }
            self.insertion = None; return false;
        }
        if !self.generated.terminal_is_empty() { retire_page(&mut self.generated, &mut RetainedCloneProgress::default()).expect("valid generated-border retirement"); return false; }
        if !self.origins.terminal_is_empty() { retire_page(&mut self.origins, &mut RetainedCloneProgress::default()).expect("valid border-origin retirement"); return false; }
        self.cursor.retire_step()
    }

    fn insert_step(&mut self, draw: &mut DrawList) -> Result<bool, RetainedOutputError> {
        match self.insertion_phase {
            0 => {
                if let Some(entry) = self.generated.get(self.insertion_scan) {
                    if let Some(candidate) = entry {
                        if self.selected_index.is_none_or(|index| self.generated.get(index).and_then(|entry| *entry).is_some_and(|current| candidate.anchor > current.anchor)) { self.selected_index = Some(self.insertion_scan); }
                    }
                    self.insertion_scan += 1;
                } else if let Some(index) = self.selected_index.take() {
                    let selected = self.generated.get_mut(index).and_then(Option::take).ok_or(RetainedOutputError::Capacity)?;
                    let owner = &draw.layers[selected.owner];
                    self.insertion = Some(DrawLayer { scissor: owner.scissor, clip: owner.clip.as_ref().map(|_| ClipRegion { scissors: BorderPages::new() }), foreground_of: owner.foreground_of, ..DrawLayer::default() });
                    self.selected = Some(selected); copied::<GeneratedBorder>(&mut self.progress); self.insertion_clip = 0; self.insertion_scan = 0; self.insertion_phase = 1;
                } else if retire_page(&mut self.generated, &mut self.progress)? { return Ok(true); }
            }
            1 => {
                let selected = self.selected.ok_or(RetainedOutputError::Capacity)?;
                let piece = draw.layers[selected.owner].clip.as_ref().and_then(|clip| clip.scissors.get(self.insertion_clip)).copied();
                if let Some(piece) = piece {
                    let clip = self.insertion.as_mut().and_then(|layer| layer.clip.as_mut()).ok_or(RetainedOutputError::Capacity)?;
                    if !clip.scissors.has_reserved_slot() {
                        let bytes = clip.scissors.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                        if !draw.claim_retained_output(1, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                        reserve_page(&mut clip.scissors, &mut self.progress)?; return Ok(false);
                    }
                    clip.scissors.push_reserved(piece).map_err(|_| RetainedOutputError::Capacity)?; copied::<ScissorRect>(&mut self.progress); self.insertion_clip += 1;
                } else {
                    if !draw.claim_retained_output(1, size_of::<UiInstance>()) { return Err(RetainedOutputError::LimitExceeded); }
                    let layer = self.insertion.as_mut().ok_or(RetainedOutputError::Capacity)?;
                    let lane = if selected.overlay { &mut layer.overlay_ui_instances } else { &mut layer.ui_instances };
                    reserve_one(lane, &mut self.progress)?;
                    lane.push(selected.instance); copied::<UiInstance>(&mut self.progress); self.insertion_scan = 0; self.insertion_phase = 6;
                }
            }
            2 => {
                if !draw.layers.has_reserved_slot() {
                    let bytes = draw.layers.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                    if !draw.claim_retained_output(1, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                    reserve_page(&mut draw.layers, &mut self.progress)?; return Ok(false);
                }
                let layer = self.insertion.take().ok_or(RetainedOutputError::Capacity)?;
                self.permanent_bytes = self.permanent_bytes.saturating_add(layer.ui_instances.capacity().saturating_add(layer.overlay_ui_instances.capacity()).saturating_mul(size_of::<UiInstance>())).saturating_add(layer.clip.as_ref().map_or(0, |clip| clip.scissors.allocated_bytes()));
                draw.layers.push_reserved(layer).map_err(|_| RetainedOutputError::Capacity)?; copied::<DrawLayer>(&mut self.progress); self.insertion_at = draw.layers.len() - 1; self.insertion_phase = 3;
            }
            3 => {
                let anchor = self.selected.ok_or(RetainedOutputError::Capacity)?.anchor;
                if self.insertion_at > anchor + 1 { draw.layers.swap(self.insertion_at, self.insertion_at - 1); self.progress.copied_bytes = self.progress.copied_bytes.saturating_add(2 * size_of::<DrawLayer>()); self.insertion_at -= 1; }
                else { self.insertion_scan = 0; self.insertion_phase = 4; }
            }
            4 => {
                if let Some(pass) = draw.scene_passes.get_mut(self.insertion_scan) {
                    if pass.layer_index >= self.insertion_at { pass.layer_index = pass.layer_index.checked_add(1).ok_or(RetainedOutputError::Capacity)?; }
                    self.insertion_scan += 1;
                } else { self.insertion_scan = 0; self.insertion_phase = 5; }
            }
            6 => {
                let selected = self.selected.ok_or(RetainedOutputError::Capacity)?;
                let current = self.insertion.as_ref().ok_or(RetainedOutputError::Capacity)?;
                if (if selected.overlay { current.overlay_ui_instances.len() } else { current.ui_instances.len() }) >= 256 { self.insertion_scan = 0; self.insertion_phase = 2; return Ok(false); }
                if let Some(entry) = self.generated.get_mut(self.insertion_scan) {
                    if entry.is_some_and(|candidate| candidate.anchor == selected.anchor && candidate.owner == selected.owner && candidate.overlay == selected.overlay) {
                        if !draw.claim_retained_output(1, size_of::<UiInstance>()) { return Err(RetainedOutputError::LimitExceeded); }
                        let layer = self.insertion.as_mut().ok_or(RetainedOutputError::Capacity)?;
                        let lane = if selected.overlay { &mut layer.overlay_ui_instances } else { &mut layer.ui_instances };
                        reserve_one(lane, &mut self.progress)?;
                        lane.push(entry.take().unwrap().instance); copied::<UiInstance>(&mut self.progress);
                    }
                    self.insertion_scan += 1;
                } else { self.insertion_scan = 0; self.insertion_phase = 2; }
            }
            _ => {
                if let Some(region) = draw.glass_regions.get_mut(self.insertion_scan) {
                    if region.layer_index >= self.insertion_at { region.layer_index = region.layer_index.checked_add(1).ok_or(RetainedOutputError::Capacity)?; }
                    self.insertion_scan += 1;
                } else { self.selected = None; self.insertion_scan = 0; self.insertion_phase = 0; }
            }
        }
        Ok(false)
    }

    fn step(&mut self, draw: &mut DrawList) -> Result<bool, RetainedOutputError> {
        if self.cleanup {
            if draw.shared_borders.pop().is_some() { copied::<SharedBorderPaint>(&mut self.progress); }
            else if draw.shared_borders.terminal_is_empty() { self.layer = usize::MAX; return self.insert_step(draw); }
            else {
                let bytes = draw.shared_borders.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                let receipt = draw.shared_borders.release_empty_page_leased(bytes).map_err(|_| RetainedOutputError::Capacity)?;
                self.released_baseline = self.released_baseline.saturating_add(receipt.released_allocation_bytes);
                self.progress.released_bytes = self.progress.released_bytes.saturating_add(receipt.released_allocation_bytes);
            }
            return Ok(false);
        }
        let Some(layer) = draw.layers.get_mut(self.layer) else {
            self.cleanup = true; self.layer = usize::MAX;
            return Ok(false);
        };
        match self.phase {
            8 => {
                let finalized = if self.overlay { layer.shared_overlay_finalized } else { layer.shared_normal_finalized };
                if finalized {
                    if self.overlay { self.phase = 7; } else { self.overlay = true; }
                } else if let Some(record) = draw.shared_borders.get(self.seed) {
                    if record.layer == self.layer && record.overlay == self.overlay { self.seed = 0; self.phase = 0; }
                    else { self.seed += 1; }
                } else if self.overlay { self.phase = 7; self.seed = 0; }
                else { self.overlay = true; self.seed = 0; }
            }
            0 => {
                if let Some(segment) = self.pending_segment {
                    if !self.origins.has_reserved_slot() {
                        let bytes = self.origins.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                        if !draw.claim_retained_output(1, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                        reserve_page(&mut self.origins, &mut self.progress)?; return Ok(false);
                    }
                    let bytes = self.cursor.next_push_allocation_byte_demand(&segment).map_err(|_| RetainedOutputError::Capacity)?;
                    if !draw.claim_retained_output(1, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                    let receipt = self.cursor.push_with_progress(segment).map_err(|_| RetainedOutputError::Capacity)?;
                    include_progress(&mut self.progress, receipt);
                    self.origins.push_reserved((self.seed_layer, self.seed)).map_err(|_| RetainedOutputError::Capacity)?;
                    copied::<(usize, usize)>(&mut self.progress); self.pending_segment = None; self.seeded = true;
                    return Ok(false);
                }
                let Some(record) = draw.shared_borders.get(self.seed).copied() else {
                    if self.seeded { self.phase = 1; }
                    else { self.phase = 2; self.mark_layer = 0; self.scan = 0; }
                    return Ok(false);
                };
                self.seed_layer = record.layer;
                let source = draw.layers.get(self.seed_layer).ok_or(RetainedOutputError::Capacity)?;
                let target = &draw.layers[self.layer];
                let source_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < self.seed_layer), draw.glass_regions.partition_point(|region| region.layer_index <= self.seed_layer));
                let target_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < self.layer), draw.glass_regions.partition_point(|region| region.layer_index <= self.layer));
                if source_epoch != target_epoch || source.foreground_of != target.foreground_of {
                    self.seed += 1;
                } else {
                    if record.overlay == self.overlay {
                        if self.clip.is_none() {
                            let side = match record.side { 0 => SharedBorderSide::Top, 1 => SharedBorderSide::Right, 2 => SharedBorderSide::Bottom, _ => SharedBorderSide::Left };
                            self.clip = Some(SharedBorderClipCursor::new(record.segment, record.stroke, side, source.scissor, source.clip.is_some()));
                            self.clip_piece = 0; self.clip_phase = 0;
                        } else if self.clip_phase == 0 {
                            if let Some(piece) = source.clip.as_ref().and_then(|clip| clip.scissors.get(self.clip_piece)).copied() {
                                let clip = self.clip.as_mut().unwrap();
                                let bytes = clip.next_push_allocation_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
                                if !draw.claim_retained_output(0, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                                let result = clip.push_clip(piece);
                                include_progress(&mut self.progress, clip.take_progress());
                                if result.map_err(|_| RetainedOutputError::Allocation)? { self.clip_piece += 1; }
                            } else { self.clip_phase = 1; }
                        } else if self.clip_phase == 1 {
                            let clip = self.clip.as_mut().unwrap();
                            let clipped = clip.step();
                            include_progress(&mut self.progress, clip.take_progress());
                            match clipped {
                                SharedBorderClipStep::Pending => {},
                                SharedBorderClipStep::Complete => self.clip_phase = 2,
                                SharedBorderClipStep::Segment(mut segment) => {
                                    segment.id = self.origins.len(); segment.owner = segment.id;
                                    self.pending_segment = Some(segment); copied::<SharedBorderSegment>(&mut self.progress);
                                }
                            }
                        } else {
                            let clip = self.clip.as_mut().unwrap();
                            let done = clip.retire_step();
                            include_progress(&mut self.progress, clip.take_progress());
                            if done.map_err(|_| RetainedOutputError::Capacity)? { self.clip = None; self.seed += 1; }
                        }
                    } else { self.seed += 1; }
                }
            }
            1 => {
                if let Some(generated) = self.pending_generated {
                    if !self.generated.has_reserved_slot() {
                        let bytes = self.generated.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                        if !draw.claim_retained_output(1, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                        reserve_page(&mut self.generated, &mut self.progress)?; return Ok(false);
                    }
                    self.generated.push_reserved(Some(generated)).map_err(|_| RetainedOutputError::Capacity)?;
                    copied::<Option<GeneratedBorder>>(&mut self.progress); self.pending_generated = None;
                    return Ok(false);
                }
                let bytes = self.cursor.next_step_allocation_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
                if !draw.claim_retained_output(0, bytes) { return Err(RetainedOutputError::LimitExceeded); }
                let (step, receipt) = self.cursor.step_with_progress();
                include_progress(&mut self.progress, receipt);
                match step {
                SharedBorderOwnershipStep::Pending => {},
                SharedBorderOwnershipStep::Segment(resolved) => {
                    let (owner_layer, owner_record) = *self.origins.get(resolved.segment.id).ok_or(RetainedOutputError::Capacity)?;
                    let (last_layer, _) = *self.origins.get(resolved.last_contributor).ok_or(RetainedOutputError::Capacity)?;
                    let record = *draw.shared_borders.get(owner_record).ok_or(RetainedOutputError::Capacity)?;
                    let [x, y] = resolved.segment.from.map(|v| v as f32);
                    let [end_x, end_y] = resolved.segment.to.map(|v| v as f32);
                    let width = record.stroke;
                    let rect = match record.side { 0 => [x, y, end_x - x, width], 1 => [x - width, y, width, end_y - y], 2 => [x, y - width, end_x - x, width], _ => [x, y, width, end_y - y] };
                    let instance = if let Some(mut effect) = record.effect {
                        effect.uv_rect = [rect[0], rect[1], rect[0] + rect[2], rect[1] + rect[3]];
                        effect.params[3] = 1.0;
                        effect
                    } else { UiInstance::solid(rect, record.color) };
                    self.pending_generated = Some(GeneratedBorder { anchor: last_layer, owner: owner_layer, overlay: self.overlay, instance }); copied::<GeneratedBorder>(&mut self.progress);
                }
                SharedBorderOwnershipStep::Complete => {
                    self.scan = 0; self.mark_layer = 0;
                    self.phase = 2;
                }
                }
            },
            2 => {
                let Some(record) = draw.shared_borders.get(self.scan).copied() else { self.mark_layer = 0; self.phase = 9; return Ok(false); };
                self.scan += 1;
                if record.overlay != self.overlay { return Ok(false); }
                let Some(mark) = draw.layers.get(record.layer) else { return Err(RetainedOutputError::Capacity); };
                let target = &draw.layers[self.layer];
                let mark_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < record.layer), draw.glass_regions.partition_point(|region| region.layer_index <= record.layer));
                let target_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < self.layer), draw.glass_regions.partition_point(|region| region.layer_index <= self.layer));
                if mark_epoch == target_epoch && mark.foreground_of == target.foreground_of {
                    let layer = &mut draw.layers[record.layer];
                    let lane = if self.overlay { &mut layer.overlay_ui_instances } else { &mut layer.ui_instances };
                    *lane.get_mut(record.slot).ok_or(RetainedOutputError::Capacity)? = UiInstance::solid([0.0; 4], Rgba::TRANSPARENT);
                    copied::<UiInstance>(&mut self.progress);
                }
            }
            9 => {
                let Some(mark) = draw.layers.get(self.mark_layer) else { self.phase = 4; return Ok(false); };
                let target = &draw.layers[self.layer];
                let mark_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < self.mark_layer), draw.glass_regions.partition_point(|region| region.layer_index <= self.mark_layer));
                let target_epoch = (draw.scene_passes.partition_point(|pass| pass.layer_index < self.layer), draw.glass_regions.partition_point(|region| region.layer_index <= self.layer));
                if mark_epoch == target_epoch && mark.foreground_of == target.foreground_of {
                    let layer = &mut draw.layers[self.mark_layer];
                    if self.overlay { layer.shared_overlay_finalized = true; } else { layer.shared_normal_finalized = true; }
                }
                self.mark_layer += 1;
            }
            4 => {
                self.progress.released_bytes = self.progress.released_bytes.saturating_add(self.cursor.next_retirement_release_byte_demand());
                if self.cursor.retire_step() { self.phase = 5; }
            }
            5 => {
                if retire_page(&mut self.origins, &mut self.progress)? { self.phase = 6; }
            }
            6 => {
                self.seed = 0; self.seed_layer = 0; self.seeded = false; self.scan = 0;
                if self.overlay { self.phase = 7; } else { self.overlay = true; self.cursor = SharedBorderOwnershipCursor::new(); self.phase = 8; }
            }
            _ => {
                self.layer += 1; self.overlay = false; self.seed_layer = 0; self.seed = 0; self.cursor = SharedBorderOwnershipCursor::new(); self.phase = 8;
            }
        }
        Ok(false)
    }
}

impl DrawList {
    /// 🪜 Bounds collection traversal depth for the next shared-border step.
    pub fn next_shared_border_finalization_depth_demand(&self) -> Result<usize, RetainedOutputError> {
        fn read<T>(values: &BorderPages<T>) -> Result<usize, RetainedOutputError> {
            if values.is_empty() { Ok(0) } else { values.next_get_depth_demand(values.len() - 1).map_err(|_| RetainedOutputError::Capacity) }
        }
        fn push<T>(values: &BorderPages<T>) -> Result<usize, RetainedOutputError> {
            if values.has_reserved_slot() { values.next_push_depth_demand() } else { values.next_reserve_depth_demand() }.map_err(|_| RetainedOutputError::Capacity)
        }
        fn retire<T>(values: &BorderPages<T>) -> Result<usize, RetainedOutputError> {
            if !values.is_empty() { values.next_pop_depth_demand().map_err(|_| RetainedOutputError::Capacity) }
            else if !values.terminal_is_empty() { values.next_release_depth_demand().map_err(|_| RetainedOutputError::Capacity) }
            else { Ok(0) }
        }
        let Some(state) = &self.border_finalize else { return Ok(usize::from(!self.border_finalized)); };
        let mut depth = 1usize.max(read(&self.layers)?).max(read(&self.shared_borders)?).max(read(&self.scene_passes)?).max(read(&self.glass_regions)?).max(read(&state.origins)?).max(read(&state.generated)?);
        if let Some(clip) = &state.clip {
            if let Some(region) = self.layers.get(state.seed_layer).and_then(|layer| layer.clip.as_ref()) { depth = depth.max(read(&region.scissors)?); }
            let demand = match state.clip_phase {
                0 if self.layers.get(state.seed_layer).and_then(|layer| layer.clip.as_ref()).is_some_and(|region| state.clip_piece < region.scissors.len()) => clip.next_push_depth_demand(),
                0 => Ok(0),
                1 => clip.next_step_depth_demand(),
                _ => clip.next_retirement_depth_demand(),
            }.map_err(|_| RetainedOutputError::Capacity)?;
            depth = depth.max(demand);
        }
        if state.cleanup {
            if !self.shared_borders.terminal_is_empty() { depth = depth.max(retire(&self.shared_borders)?); }
            else if state.insertion_phase == 0 && state.insertion_scan >= state.generated.len() && state.selected_index.is_none() { depth = depth.max(retire(&state.generated)?); }
            else if state.insertion_phase == 2 { depth = depth.max(push(&self.layers)?); }
            else if state.insertion_phase == 1 {
                if let Some(clip) = state.insertion.as_ref().and_then(|layer| layer.clip.as_ref()) {
                    depth = depth.max(read(&clip.scissors)?);
                    let source = state.selected.and_then(|selected| self.layers.get(selected.owner)).and_then(|layer| layer.clip.as_ref());
                    if let Some(region) = source { depth = depth.max(read(&region.scissors)?); }
                    let has_piece = source.is_some_and(|region| state.insertion_clip < region.scissors.len());
                    if has_piece { depth = depth.max(push(&clip.scissors)?); }
                }
            }
        } else {
            match state.phase {
                0 if state.pending_segment.is_some() => { depth = depth.max(push(&state.origins)?); if state.origins.has_reserved_slot() { depth = depth.max(state.cursor.maximum_push_depth_demand()); } },
                1 if state.pending_generated.is_some() => depth = depth.max(push(&state.generated)?),
                1 if state.pending_generated.is_none() => depth = depth.max(state.cursor.next_step_depth_demand()),
                4 => depth = depth.max(state.cursor.next_retirement_depth_demand()),
                5 => depth = depth.max(retire(&state.origins)?),
                _ => {},
            }
        }
        Ok(depth)
    }

    /// 📋 Bounds bytes copied by the next shared-border work step.
    pub fn next_shared_border_finalization_copy_byte_demand(&self) -> Result<usize, RetainedOutputError> {
        fn append<T>(values: &Vec<T>) -> usize { size_of::<T>().saturating_add(if values.len() == values.capacity() { values.len().saturating_mul(size_of::<T>()) } else { 0 }) }
        let Some(state) = &self.border_finalize else { return Ok(0); };
        if state.cleanup && self.shared_borders.terminal_is_empty() {
            return Ok(match state.insertion_phase {
                1 => {
                    let selected = state.selected.ok_or(RetainedOutputError::Capacity)?;
                    let has_piece = self.layers.get(selected.owner).and_then(|layer| layer.clip.as_ref()).is_some_and(|clip| state.insertion_clip < clip.scissors.len());
                    if has_piece {
                        let clip = state.insertion.as_ref().and_then(|layer| layer.clip.as_ref()).ok_or(RetainedOutputError::Capacity)?;
                        if clip.scissors.has_reserved_slot() { size_of::<ScissorRect>() } else { clip.scissors.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)? }
                    } else { size_of::<UiInstance>() }
                },
                2 => if self.layers.has_reserved_slot() { size_of::<DrawLayer>() } else { self.layers.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)? },
                3 => 2 * size_of::<DrawLayer>(),
                6 => state.insertion.as_ref().map_or(0, |layer| { let lane = if state.selected.is_some_and(|item| item.overlay) { &layer.overlay_ui_instances } else { &layer.ui_instances }; if lane.len() >= 256 { 0 } else { append(lane) } }),
                0 => size_of::<Option<GeneratedBorder>>(),
                _ => size_of::<usize>(),
            });
        }
        if state.cleanup { return Ok(size_of::<SharedBorderPaint>()); }
        Ok(match state.phase {
            0 if state.pending_segment.is_some() => if state.origins.has_reserved_slot() { state.cursor.maximum_push_copy_byte_demand().saturating_add(size_of::<(usize, usize)>()) } else { state.origins.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)? },
            0 => match state.clip.as_ref() {
                Some(clip) if state.clip_phase == 0 && self.layers.get(state.seed_layer).and_then(|layer| layer.clip.as_ref()).is_some_and(|region| state.clip_piece < region.scissors.len()) => clip.next_push_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)?,
                Some(clip) if state.clip_phase == 1 => { let bytes = clip.next_step_copy_byte_demand(); bytes.saturating_add(if bytes != 0 { size_of::<SharedBorderSegment>() } else { 0 }) },
                _ => 0,
            },
            1 if state.pending_generated.is_some() => if state.generated.has_reserved_slot() { size_of::<Option<GeneratedBorder>>() } else { state.generated.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)? },
            1 => state.cursor.next_step_copy_byte_demand().saturating_add(if state.cursor.next_step_emits_segment() { size_of::<GeneratedBorder>() } else { 0 }),
            2 => size_of::<UiInstance>(),
            5 => size_of::<(usize, usize)>(),
            _ => size_of::<usize>(),
        })
    }

    /// 📉 Bounds backing released by the next shared-border step.
    pub fn next_shared_border_finalization_release_byte_demand(&self) -> Result<usize, RetainedOutputError> {
        fn growth_release<T>(values: &Vec<T>) -> usize { if values.len() == values.capacity() { values.capacity().saturating_mul(size_of::<T>()) } else { 0 } }
        let Some(state) = &self.border_finalize else { return Ok(0); };
        if state.cleanup && self.shared_borders.terminal_is_empty() {
            return Ok(match state.insertion_phase {
                0 if state.insertion_scan >= state.generated.len() && state.selected_index.is_none() && state.generated.is_empty() => state.generated.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?,
                1 => 0,
                2 => 0,
                6 => state.insertion.as_ref().map_or(0, |layer| { let lane = if state.selected.is_some_and(|item| item.overlay) { &layer.overlay_ui_instances } else { &layer.ui_instances }; if lane.len() >= 256 { 0 } else { growth_release(lane) } }),
                _ => 0,
            });
        }
        if state.cleanup {
            return if self.shared_borders.is_empty() { self.shared_borders.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity) } else { Ok(0) };
        }
        Ok(match state.phase {
            0 => {
                if state.clip_phase == 2 { state.clip.as_ref().map(|clip| clip.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity)).transpose()?.unwrap_or(0) }
                else { state.clip.as_ref().map_or(0, SharedBorderClipCursor::next_push_release_byte_demand) }
            }
            1 => state.cursor.next_step_transferred_byte_demand(),
            2 => 0,
            4 => state.cursor.next_retirement_release_byte_demand(),
            5 if state.origins.is_empty() => state.origins.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?,
            _ => 0,
        })
    }

    /// 📊 Reports retained allocation added by shared-border finalization over its starting backing.
    pub fn shared_border_retained_capacity_bytes(&self) -> usize {
        let Some(state) = &self.border_finalize else { return self.border_retained_bytes; };
        let insertion = state.insertion.as_ref().map_or(0, |layer| layer.ui_instances.capacity().saturating_add(layer.overlay_ui_instances.capacity()).saturating_mul(size_of::<UiInstance>()).saturating_add(layer.clip.as_ref().map_or(0, |clip| clip.scissors.allocated_bytes())));
        let scratch = state.cursor.retained_capacity_bytes()
            .saturating_add(state.clip.as_ref().map_or(0, SharedBorderClipCursor::retained_capacity_bytes))
            .saturating_add(state.origins.allocated_bytes())
            .saturating_add(state.generated.allocated_bytes())
            .saturating_add(insertion);
        state.permanent_bytes.saturating_add(scratch).saturating_add(self.layers.allocated_bytes().saturating_sub(state.base_layers_capacity)).saturating_sub(state.released_baseline)
    }

    /// 📦 Bounds allocation needed by the next resolution step before admitting it.
    pub fn next_shared_border_finalization_allocation_demand(&self) -> Result<(usize, usize), RetainedOutputError> {
        fn growth<T>(values: &Vec<T>) -> Result<usize, RetainedOutputError> {
            if values.len() < values.capacity() { Ok(0) } else { values.capacity().max(2).checked_mul(2).and_then(|count| count.checked_mul(size_of::<T>())).ok_or(RetainedOutputError::Capacity) }
        }
        let Some(state) = &self.border_finalize else { return Ok((usize::from(!self.border_finalized), 0)); };
        if state.cleanup && self.shared_borders.terminal_is_empty() {
            return match state.insertion_phase {
                1 => {
                    let selected = state.selected.ok_or(RetainedOutputError::Capacity)?;
                    let has_piece = self.layers.get(selected.owner).and_then(|layer| layer.clip.as_ref()).is_some_and(|clip| state.insertion_clip < clip.scissors.len());
                    if has_piece {
                        let clip = state.insertion.as_ref().and_then(|layer| layer.clip.as_ref()).ok_or(RetainedOutputError::Capacity)?;
                        Ok((1, if clip.scissors.has_reserved_slot() { 0 } else { clip.scissors.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)? }))
                    } else {
                        let layer = state.insertion.as_ref().ok_or(RetainedOutputError::Capacity)?;
                        Ok((1, growth(if selected.overlay { &layer.overlay_ui_instances } else { &layer.ui_instances })?))
                    }
                }
                2 => Ok((1, if self.layers.has_reserved_slot() { 0 } else { self.layers.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)? })),
                6 => {
                    let layer = state.insertion.as_ref().ok_or(RetainedOutputError::Capacity)?;
                    let selected = state.selected.ok_or(RetainedOutputError::Capacity)?;
                    let lane = if selected.overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
                    if lane.len() >= 256 { return Ok((1, 0)); }
                    let matches = state.generated.get(state.insertion_scan).and_then(|entry| *entry).is_some_and(|item| item.anchor == selected.anchor && item.owner == selected.owner && item.overlay == selected.overlay);
                    if matches { Ok((1, growth(lane)?)) } else { Ok((1, 0)) }
                }
                _ => Ok((1, 0)),
            };
        }
        match state.phase {
            0 => {
                if let Some(segment) = state.pending_segment {
                    return Ok((1, if state.origins.has_reserved_slot() { state.cursor.next_push_allocation_byte_demand(&segment).map_err(|_| RetainedOutputError::Capacity)? } else { state.origins.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)? }));
                }
                let Some(clip) = state.clip.as_ref() else { return Ok((1, 0)); };
                if state.clip_phase == 0 {
                    let has_piece = self.layers.get(state.seed_layer).and_then(|layer| layer.clip.as_ref()).is_some_and(|region| state.clip_piece < region.scissors.len());
                    Ok((1, if has_piece { clip.next_push_allocation_byte_demand().map_err(|_| RetainedOutputError::Capacity)? } else { 0 }))
                } else { Ok((1, 0)) }
            }
            1 => {
                if state.pending_generated.is_some() { return Ok((1, if state.generated.has_reserved_slot() { 0 } else { state.generated.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)? })); }
                let cursor = state.cursor.next_step_allocation_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
                Ok((1, cursor))
            }
            2 => Ok((1, 0)),
            _ => Ok((1, 0)),
        }
    }

    /// 🧩 Advances one bounded shared-border resolution or paint reconstruction step.
    pub fn finalize_shared_borders_step(&mut self) -> Result<bool, RetainedOutputError> {
        self.finalize_shared_borders_step_with_progress().0
    }

    /// 🧾 Advances one step and returns its work receipt even if admission fails.
    pub fn finalize_shared_borders_step_with_progress(&mut self) -> (Result<bool, RetainedOutputError>, RetainedCloneProgress) {
        if self.retained_append.is_some() { return (Err(RetainedOutputError::AppendActive), RetainedCloneProgress::default()); }
        if self.border_finalized { return (Ok(true), RetainedCloneProgress::default()); }
        let mut cursor = self.border_finalize.take().unwrap_or_else(|| { self.border_retained_bytes = 0; BorderFinalize::new(self.layers.allocated_bytes()) });
        cursor.progress = RetainedCloneProgress { copied_items: 1, ..RetainedCloneProgress::default() };
        let result = cursor.step(self);
        let progress = std::mem::take(&mut cursor.progress);
        self.border_finalize = Some(cursor);
        if matches!(result, Ok(true)) { self.border_retained_bytes = self.shared_border_retained_capacity_bytes(); self.border_finalize = None; self.border_finalized = true; }
        (result, progress)
    }
}
