//! 📥 Owned, bounded transfer of one retained draw candidate.
use super::*;
use semio_framework_job::{RetainedCloneGrant, RetainedCloneProgress};

/// 🪪 Identifies the active append transaction on one draw list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetainedAppendToken(pub(super) u64);

/// ⏩ One append or rollback result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedAppendStep { Pending, Complete }

#[derive(Clone)]
pub(super) struct RetainedAppendWork {
    token: RetainedAppendToken,
    source: DrawList,
    layer_base: usize,
    border_base: usize,
    scene_base: usize,
    glass_base: usize,
    prepared_items: usize,
    prepared_bytes: usize,
    retained_output: Option<RetainedOutputGrant>,
    phase: u8,
    reverse: usize,
    pending: Option<DrawLayer>,
    local_clip: Option<ClipRegion>,
    piece: Option<ScissorRect>,
    left: usize,
    right: usize,
    cancelling: bool,
}

fn add<T>(target: &mut PagedVec<T>, value: T, progress: &mut RetainedCloneProgress) -> Result<(), RetainedOutputError> {
    target.push_reserved(value).map_err(|_| RetainedOutputError::Capacity)?;
    progress.copied_bytes = size_of::<T>();
    Ok(())
}

fn reserve<T>(target: &mut PagedVec<T>, progress: &mut RetainedCloneProgress) -> Result<(), RetainedOutputError> {
    let bytes = target.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
    match target.reserve_one(bytes) {
        Ok(value) => { progress.retained_capacity_bytes = value.allocated_bytes; progress.copied_bytes = value.placed_bytes; Ok(()) },
        Err(error) => { progress.retained_capacity_bytes = error.allocated_bytes; Err(RetainedOutputError::Allocation) },
    }
}

fn demand<T>(target: &PagedVec<T>) -> Result<(usize, usize, usize), RetainedOutputError> {
    if target.has_reserved_slot() {
        Ok((0, size_of::<T>(), target.next_push_depth_demand().map_err(|_| RetainedOutputError::Capacity)?))
    } else {
        Ok((target.next_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?, target.next_reserve_copy_byte_demand().map_err(|_| RetainedOutputError::Capacity)?, target.next_reserve_depth_demand().map_err(|_| RetainedOutputError::Capacity)?))
    }
}

impl RetainedAppendWork {
    fn new(token: RetainedAppendToken, source: DrawList, target: &DrawList, prepared_items: usize, prepared_bytes: usize, retained_output: Option<RetainedOutputGrant>) -> Self {
        Self { token, source, layer_base: target.layers.len(), border_base: target.shared_borders.len(), scene_base: target.scene_passes.len(), glass_base: target.glass_regions.len(), prepared_items, prepared_bytes, retained_output, phase: 0, reverse: 0, pending: None, local_clip: None, piece: None, left: 0, right: 0, cancelling: false }
    }
}

impl DrawList {
    /// 📦 Quotes the bounded transaction owner allocation.
    pub fn retained_append_begin_demand() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<RetainedAppendWork>(), maximum_capacity_bytes: size_of::<RetainedAppendWork>(), maximum_release_bytes: size_of::<RetainedAppendWork>(), maximum_depth: 1, ..Default::default() }
    }

    /// 📥 Takes custody of a balanced candidate and returns its generation token.
    pub fn begin_retained_append(&mut self, candidate: &mut DrawList) -> (Result<RetainedAppendToken, RetainedOutputError>, RetainedCloneProgress) {
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.retained_append.is_some() { return (Err(RetainedOutputError::AppendActive), progress); }
        if self.border_finalize.is_some() { return (Err(RetainedOutputError::AppendActive), progress); }
        if self.border_finalized || candidate.border_finalized { return (Err(RetainedOutputError::Finalized), progress); }
        if !candidate.scissor_stack.is_empty() || !candidate.clip_stack.is_empty() || !candidate.glass_content_stack.is_empty() || candidate.retained_output.is_some() || candidate.border_finalize.is_some() || candidate.retained_append.is_some() || candidate.overlay_route != 0 { return (Err(RetainedOutputError::Capacity), progress); }
        let Some(sequence) = self.append_sequence.checked_add(1) else { return (Err(RetainedOutputError::Capacity), progress) };
        let mut owner = Vec::new();
        if owner.try_reserve_exact(1).is_err() { return (Err(RetainedOutputError::Allocation), progress); }
        progress.retained_capacity_bytes = owner.capacity() * size_of::<RetainedAppendWork>();
        let base_items = self.prepared_items; let base_bytes = self.prepared_bytes; let base_grant = self.retained_output;
        if !self.claim_retained_output(candidate.prepared_items, candidate.prepared_bytes) {
            progress.released_bytes = progress.retained_capacity_bytes;
            return (Err(RetainedOutputError::LimitExceeded), progress);
        }
        let source = std::mem::replace(candidate, DrawList::empty());
        progress.copied_bytes = size_of::<RetainedAppendWork>();
        let token = RetainedAppendToken(sequence);
        owner.push(RetainedAppendWork::new(token, source, self, base_items, base_bytes, base_grant));
        self.append_sequence = sequence;
        self.retained_append = Some(owner.into_boxed_slice());
        (Ok(token), progress)
    }

    /// 🪪 Returns the active transaction identity when one is owned.
    pub fn active_retained_append_token(&self) -> Option<RetainedAppendToken> { self.retained_append.as_ref().and_then(|owner| owner.first()).map(|work| work.token) }

    /// 📋 Bounds the next active append or rollback step.
    pub fn next_retained_append_demand(&self, token: RetainedAppendToken) -> Result<RetainedCloneGrant, RetainedOutputError> {
        let work = self.retained_append.as_ref().and_then(|owner| owner.first()).ok_or(RetainedOutputError::NoActiveGrant)?;
        if work.token != token { return Err(RetainedOutputError::InvalidAppendToken); }
        let mut result = RetainedCloneGrant { maximum_items: 1, maximum_depth: 1, ..Default::default() };
        if work.cancelling { return self.next_retained_append_cancel_demand(token); }
        let pair = match work.phase {
            0 if work.pending.is_none() && !work.source.layers.is_empty() => Some(demand(&self.layers)?),
            1 if work.piece.is_some() => Some(demand(&work.pending.as_ref().and_then(|layer| layer.clip.as_ref()).ok_or(RetainedOutputError::Capacity)?.scissors)?),
            2 if work.pending.is_some() => Some(demand(&self.layers)?),
            4 if !work.source.shared_borders.is_empty() => Some(demand(&self.shared_borders)?),
            6 if !work.source.scene_passes.is_empty() => Some(demand(&self.scene_passes)?),
            8 if !work.source.glass_regions.is_empty() => Some(demand(&self.glass_regions)?),
            _ => None,
        };
        if let Some((capacity, copied, depth)) = pair { result.maximum_capacity_bytes = capacity; result.maximum_copy_bytes = copied; result.maximum_depth = result.maximum_depth.max(depth); }
        match work.phase {
            3 if work.reverse < (self.layers.len() - work.layer_base) / 2 => { result.maximum_copy_bytes = 2 * size_of::<DrawLayer>(); result.maximum_depth = 32; }
            5 if work.reverse < (self.shared_borders.len() - work.border_base) / 2 => { result.maximum_copy_bytes = 2 * size_of::<SharedBorderPaint>(); result.maximum_depth = 32; }
            7 if work.reverse < (self.scene_passes.len() - work.scene_base) / 2 => { result.maximum_copy_bytes = 2 * size_of::<ScenePass3d>(); result.maximum_depth = 32; }
            9 if work.reverse < (self.glass_regions.len() - work.glass_base) / 2 => { result.maximum_copy_bytes = 2 * size_of::<GlassRegion>(); result.maximum_depth = 32; }
            _ => {}
        }
        if matches!(work.phase, 0 | 4 | 6 | 8) { result.maximum_depth = result.maximum_depth.max(32); }
        if work.phase == 1 && work.piece.is_none() { result.maximum_copy_bytes = size_of::<ScissorRect>(); result.maximum_depth = 16; }
        if work.phase == 1 && work.local_clip.is_some() && work.left == usize::MAX {
            let clip = work.local_clip.as_ref().unwrap();
            result.maximum_release_bytes = if clip.scissors.is_empty() { clip.scissors.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)? } else { 0 };
        }
        if work.phase == 10 {
            result.maximum_release_bytes = work.source.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
            result.maximum_depth = work.source.next_retirement_depth_demand().map_err(|_| RetainedOutputError::Capacity)?;
        }
        if work.phase == 11 { result.maximum_release_bytes = size_of::<RetainedAppendWork>(); }
        Ok(result)
    }

    fn next_retained_append_cancel_release(&self, work: &RetainedAppendWork) -> Result<usize, RetainedOutputError> {
        if let Some(clip) = &work.local_clip {
            return if clip.scissors.is_empty() { clip.scissors.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity) } else { Ok(0) };
        }
        if work.pending.is_some() || self.layers.len() > work.layer_base || self.shared_borders.len() > work.border_base || self.scene_passes.len() > work.scene_base || self.glass_regions.len() > work.glass_base { return Ok(0); }
        if !work.source.retirement_is_empty() { return work.source.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity); }
        Ok(size_of::<RetainedAppendWork>())
    }
}

impl DrawList {
    /// ♻️ Bounds one rollback step before cancellation begins.
    pub fn next_retained_append_cancel_demand(&self, token: RetainedAppendToken) -> Result<RetainedCloneGrant, RetainedOutputError> {
        let work = self.retained_append.as_ref().and_then(|owner| owner.first()).ok_or(RetainedOutputError::NoActiveGrant)?;
        if work.token != token { return Err(RetainedOutputError::InvalidAppendToken); }
        let mut grant = RetainedCloneGrant { maximum_items: 1, maximum_depth: 32, maximum_release_bytes: self.next_retained_append_cancel_release(work)?, ..Default::default() };
        let pair = if work.local_clip.is_none() && (work.pending.is_some() || self.layers.len() > work.layer_base) { Some(demand(&work.source.layers)?) }
        else if work.local_clip.is_none() && self.shared_borders.len() > work.border_base { Some(demand(&work.source.shared_borders)?) }
        else if work.local_clip.is_none() && self.scene_passes.len() > work.scene_base { Some(demand(&work.source.scene_passes)?) }
        else if work.local_clip.is_none() && self.glass_regions.len() > work.glass_base { Some(demand(&work.source.glass_regions)?) } else { None };
        if let Some((capacity, copied, depth)) = pair { grant.maximum_capacity_bytes = capacity; grant.maximum_copy_bytes = copied; grant.maximum_depth = grant.maximum_depth.max(depth); }
        if pair.is_none() && work.local_clip.is_none() && work.pending.is_none() && self.layers.len() == work.layer_base && self.shared_borders.len() == work.border_base && self.scene_passes.len() == work.scene_base && self.glass_regions.len() == work.glass_base && !work.source.retirement_is_empty() {
            grant.maximum_depth = work.source.next_retirement_depth_demand().map_err(|_| RetainedOutputError::Capacity)?;
        }
        Ok(grant)
    }

    /// 🧩 Advances one append step and reports actual ownership work.
    pub fn step_retained_append(&mut self, token: RetainedAppendToken) -> (Result<RetainedAppendStep, RetainedOutputError>, RetainedCloneProgress) {
        self.run_retained_append(token, false)
    }

    /// ↩️ Rolls back one staged transfer and retires the consumed candidate.
    pub fn cancel_retained_append_step(&mut self, token: RetainedAppendToken) -> (Result<RetainedAppendStep, RetainedOutputError>, RetainedCloneProgress) {
        self.run_retained_append(token, true)
    }

    fn run_retained_append(&mut self, token: RetainedAppendToken, cancel: bool) -> (Result<RetainedAppendStep, RetainedOutputError>, RetainedCloneProgress) {
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let Some(mut owner) = self.retained_append.take() else { return (Err(RetainedOutputError::NoActiveGrant), progress) };
        if owner[0].token != token { self.retained_append = Some(owner); return (Err(RetainedOutputError::InvalidAppendToken), progress); }
        let work = &mut owner[0];
        if cancel { work.cancelling = true; }
        let result = if work.cancelling { self.cancel_append_work(work, &mut progress) } else { self.advance_append_work(work, &mut progress) };
        if matches!(result, Ok(RetainedAppendStep::Complete)) {
            progress.released_bytes = progress.released_bytes.saturating_add(size_of::<RetainedAppendWork>());
        } else { self.retained_append = Some(owner); }
        (result, progress)
    }

    fn advance_append_work(&mut self, work: &mut RetainedAppendWork, progress: &mut RetainedCloneProgress) -> Result<RetainedAppendStep, RetainedOutputError> {
        match work.phase {
            0 => {
                if work.source.layers.is_empty() { work.phase = 3; work.reverse = 0; }
                else if !self.layers.has_reserved_slot() { reserve(&mut self.layers, progress)?; }
                else {
                    let foreground = match work.source.layers.last().ok_or(RetainedOutputError::Capacity)?.foreground_of { Some(region) => Some(region.checked_add(work.glass_base).ok_or(RetainedOutputError::Capacity)?), None => self.glass_content_stack.last().copied() };
                    let mut layer = work.source.layers.pop().ok_or(RetainedOutputError::Capacity)?;
                    layer.scissor = match (layer.scissor, self.scissor_stack.last().copied()) { (Some(local), Some(parent)) => Some(local.intersect(&parent)), (local, parent) => local.or(parent) };
                    layer.foreground_of = foreground;
                    work.local_clip = layer.clip.take();
                    if work.local_clip.is_some() || self.clip_stack.last().is_some() { layer.clip = Some(ClipRegion { scissors: PagedVec::new() }); }
                    work.pending = Some(layer); work.left = 0; work.right = 0; work.phase = 1;
                    progress.copied_bytes = size_of::<DrawLayer>();
                }
            }
            1 => {
                if let Some(piece) = work.piece {
                    let clip = &mut work.pending.as_mut().ok_or(RetainedOutputError::Capacity)?.clip.as_mut().ok_or(RetainedOutputError::Capacity)?.scissors;
                    if !clip.has_reserved_slot() { reserve(clip, progress)?; } else { add(clip, piece, progress)?; work.piece = None; }
                } else if work.left == usize::MAX {
                    if let Some(clip) = &mut work.local_clip {
                        if clip.scissors.pop().is_none() {
                            if clip.scissors.terminal_is_empty() { work.local_clip = None; work.phase = 2; }
                            else {
                                let bytes = clip.scissors.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                                let result = clip.scissors.release_empty_page(bytes).map_err(|_| RetainedOutputError::Capacity)?;
                                progress.released_bytes = result.released_allocation_bytes;
                            }
                        }
                    } else { work.phase = 2; }
                } else {
                    let parent = self.clip_stack.last();
                    let local = work.local_clip.as_ref();
                    let (local_len, parent_len) = (local.map_or(1, |clip| clip.scissors.len()), parent.map_or(1, |clip| clip.scissors.len()));
                    if work.left >= local_len || parent_len == 0 {
                        work.left = usize::MAX;
                    } else {
                        let a = local.and_then(|clip| clip.scissors.get(work.left)).copied();
                        let b = parent.and_then(|clip| clip.scissors.get(work.right)).copied();
                        let piece = match (a, b) { (Some(a), Some(b)) => Some(a.intersect(&b)), (Some(a), None) => Some(a), (None, Some(b)) => Some(b), _ => None };
                        work.right += 1;
                        if work.right >= parent_len { work.right = 0; work.left += 1; }
                        if let Some(piece) = piece.filter(|piece| piece.w > 0 && piece.h > 0) { work.piece = Some(piece); progress.copied_bytes = size_of::<ScissorRect>(); }
                    }
                }
            }
            2 => {
                if !self.layers.has_reserved_slot() { reserve(&mut self.layers, progress)?; }
                else { add(&mut self.layers, work.pending.take().ok_or(RetainedOutputError::Capacity)?, progress)?; work.phase = 0; }
            }
            3 => {
                let len = self.layers.len() - work.layer_base;
                if work.reverse < len / 2 { self.layers.swap(work.layer_base + work.reverse, self.layers.len() - 1 - work.reverse); work.reverse += 1; progress.copied_bytes = 2 * size_of::<DrawLayer>(); }
                else { work.phase = 4; work.reverse = 0; }
            }
            4 => {
                if work.source.shared_borders.is_empty() { work.phase = 5; work.reverse = 0; }
                else if !self.shared_borders.has_reserved_slot() { reserve(&mut self.shared_borders, progress)?; }
                else {
                    let original = work.source.shared_borders.last().ok_or(RetainedOutputError::Capacity)?;
                    let layer = original.layer.checked_add(work.layer_base).ok_or(RetainedOutputError::Capacity)?;
                    let id = original.segment.id.checked_add(work.border_base).ok_or(RetainedOutputError::Capacity)?;
                    let owner = original.segment.owner.checked_add(work.border_base).ok_or(RetainedOutputError::Capacity)?;
                    let mut record = work.source.shared_borders.pop().ok_or(RetainedOutputError::Capacity)?;
                    record.layer = layer; record.segment.id = id; record.segment.owner = owner;
                    add(&mut self.shared_borders, record, progress)?;
                }
            }
            5 => {
                let len = self.shared_borders.len() - work.border_base;
                if work.reverse < len / 2 { self.shared_borders.swap(work.border_base + work.reverse, self.shared_borders.len() - 1 - work.reverse); work.reverse += 1; progress.copied_bytes = 2 * size_of::<SharedBorderPaint>(); }
                else { work.phase = 6; work.reverse = 0; }
            }
            6 => {
                if work.source.scene_passes.is_empty() { work.phase = 7; work.reverse = 0; }
                else if !self.scene_passes.has_reserved_slot() { reserve(&mut self.scene_passes, progress)?; }
                else {
                    let layer = work.source.scene_passes.last().ok_or(RetainedOutputError::Capacity)?.layer_index.checked_add(work.layer_base).ok_or(RetainedOutputError::Capacity)?;
                    let mut pass = work.source.scene_passes.pop().ok_or(RetainedOutputError::Capacity)?;
                    pass.layer_index = layer;
                    add(&mut self.scene_passes, pass, progress)?;
                }
            }
            7 => {
                let len = self.scene_passes.len() - work.scene_base;
                if work.reverse < len / 2 { self.scene_passes.swap(work.scene_base + work.reverse, self.scene_passes.len() - 1 - work.reverse); work.reverse += 1; progress.copied_bytes = 2 * size_of::<ScenePass3d>(); }
                else { work.phase = 8; work.reverse = 0; }
            }
            8 => {
                if work.source.glass_regions.is_empty() { work.phase = 9; work.reverse = 0; }
                else if !self.glass_regions.has_reserved_slot() { reserve(&mut self.glass_regions, progress)?; }
                else {
                    let layer = work.source.glass_regions.last().ok_or(RetainedOutputError::Capacity)?.layer_index.checked_add(work.layer_base).ok_or(RetainedOutputError::Capacity)?;
                    let mut region = work.source.glass_regions.pop().ok_or(RetainedOutputError::Capacity)?;
                    region.layer_index = layer;
                    add(&mut self.glass_regions, region, progress)?;
                }
            }
            9 => {
                let len = self.glass_regions.len() - work.glass_base;
                if work.reverse < len / 2 { self.glass_regions.swap(work.glass_base + work.reverse, self.glass_regions.len() - 1 - work.reverse); work.reverse += 1; progress.copied_bytes = 2 * size_of::<GlassRegion>(); }
                else { work.phase = 10; }
            }
            10 => {
                if work.source.retirement_is_empty() { work.phase = 11; }
                else {
                    progress.released_bytes = work.source.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
                    work.source.retire_step();
                }
            }
            _ => return Ok(RetainedAppendStep::Complete),
        }
        Ok(RetainedAppendStep::Pending)
    }

    fn cancel_append_work(&mut self, work: &mut RetainedAppendWork, progress: &mut RetainedCloneProgress) -> Result<RetainedAppendStep, RetainedOutputError> {
        if let Some(clip) = &mut work.local_clip {
            if clip.scissors.pop().is_none() {
                if clip.scissors.terminal_is_empty() { work.local_clip = None; }
                else {
                    let bytes = clip.scissors.next_release_allocation_bytes().map_err(|_| RetainedOutputError::Capacity)?;
                    progress.released_bytes = clip.scissors.release_empty_page(bytes).map_err(|_| RetainedOutputError::Capacity)?.released_allocation_bytes;
                }
            }
        } else if work.pending.is_some() || self.layers.len() > work.layer_base {
            if !work.source.layers.has_reserved_slot() { reserve(&mut work.source.layers, progress)?; }
            else {
                let layer = match work.pending.take() { Some(layer) => layer, None => self.layers.pop().ok_or(RetainedOutputError::Capacity)? };
                add(&mut work.source.layers, layer, progress)?;
            }
        } else if self.shared_borders.len() > work.border_base {
            if !work.source.shared_borders.has_reserved_slot() { reserve(&mut work.source.shared_borders, progress)?; }
            else { add(&mut work.source.shared_borders, self.shared_borders.pop().ok_or(RetainedOutputError::Capacity)?, progress)?; }
        } else if self.scene_passes.len() > work.scene_base {
            if !work.source.scene_passes.has_reserved_slot() { reserve(&mut work.source.scene_passes, progress)?; }
            else { add(&mut work.source.scene_passes, self.scene_passes.pop().ok_or(RetainedOutputError::Capacity)?, progress)?; }
        } else if self.glass_regions.len() > work.glass_base {
            if !work.source.glass_regions.has_reserved_slot() { reserve(&mut work.source.glass_regions, progress)?; }
            else { add(&mut work.source.glass_regions, self.glass_regions.pop().ok_or(RetainedOutputError::Capacity)?, progress)?; }
        } else if !work.source.retirement_is_empty() {
            progress.released_bytes = work.source.next_retirement_release_byte_demand().map_err(|_| RetainedOutputError::Capacity)?;
            work.source.retire_step();
        } else {
            self.prepared_items = work.prepared_items; self.prepared_bytes = work.prepared_bytes; self.retained_output = work.retained_output;
            return Ok(RetainedAppendStep::Complete);
        }
        Ok(RetainedAppendStep::Pending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_ui_theme::geometry::Rect;

    fn receipt_fits(grant: RetainedCloneGrant, progress: RetainedCloneProgress) {
        assert!(progress.copied_items <= grant.maximum_items);
        assert!(progress.copied_bytes <= grant.maximum_copy_bytes);
        assert!(progress.retained_capacity_bytes <= grant.maximum_capacity_bytes);
        assert!(progress.released_bytes <= grant.maximum_release_bytes);
        assert!(grant.maximum_depth <= 64);
        assert!(grant.maximum_copy_bytes <= 65536 && grant.maximum_capacity_bytes <= 65536 && grant.maximum_release_bytes <= 262144);
    }

    #[test]
    fn append_receipts_and_every_rollback_preserve_target_and_retire() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("cases.json")).unwrap();
        let rect = |value: &serde_json::Value| ScissorRect { x: value[0].as_u64().unwrap() as u32, y: value[1].as_u64().unwrap() as u32, w: value[2].as_u64().unwrap() as u32, h: value[3].as_u64().unwrap() as u32 };
        let mut target = DrawList::default();
        target.push_solid([1.0, 1.0, 2.0, 2.0], Rgba::TRANSPARENT);
        target.clip_stack.push(ClipRegion { scissors: vec![rect(&fixture["parent"])].into() });
        let base_layers = target.layers.len();
        let base_items = target.prepared_items;
        let base_bytes = target.prepared_bytes;
        let mut candidate = DrawList::default();
        candidate.layers[0].clip = Some(ClipRegion { scissors: vec![rect(&fixture["child"])].into() });
        candidate.push_shared_border(Rect::new(0.0, 0.0, 20.0, 20.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Hover);
        candidate.push_scene_pass(ScenePass3d::default());
        let (started, progress) = target.begin_retained_append(&mut candidate);
        receipt_fits(DrawList::retained_append_begin_demand(), progress);
        let token = started.unwrap();
        assert!(candidate.retirement_is_empty());
        let mut complete = false;
        for _ in 0..100_000 {
            let mut rollback = target.clone();
            for _ in 0..100_000 {
                let demand = rollback.next_retained_append_cancel_demand(token).unwrap();
                let (step, progress) = rollback.cancel_retained_append_step(token);
                receipt_fits(demand, progress);
                if step.unwrap() == RetainedAppendStep::Complete { break; }
            }
            assert_eq!(rollback.active_retained_append_token(), None);
            assert_eq!(rollback.layers.len(), base_layers);
            assert_eq!(rollback.prepared_items, base_items);
            assert_eq!(rollback.prepared_bytes, base_bytes);
            assert_eq!(rollback.layers[0].ui_instances.len(), 1);
            while !rollback.retire_step() {}
            assert!(rollback.retirement_is_empty());
            let demand = target.next_retained_append_demand(token).unwrap();
            let (step, progress) = target.step_retained_append(token);
            receipt_fits(demand, progress);
            complete = step.unwrap() == RetainedAppendStep::Complete;
            if complete { break; }
        }
        assert!(complete);
        assert_eq!(target.layers.len(), base_layers + 2);
        assert_eq!(target.shared_borders.len(), 4);
        assert_eq!(target.scene_passes.len(), 1);
        assert_eq!(target.scene_passes[0].layer_index, base_layers);
        let clip = target.layers[base_layers].clip.as_ref().unwrap();
        assert_eq!(clip.scissors.iter().copied().collect::<Vec<_>>(), vec![rect(&fixture["intersection"])]);
        while !target.retire_step() {}
        assert!(target.retirement_is_empty());
    }

    #[test]
    fn append_refuses_an_active_finalizer_without_taking_candidate() {
        let mut target = DrawList::default();
        target.push_shared_border(Rect::new(0.0, 0.0, 10.0, 10.0), 1.0, Rgba::TRANSPARENT, SharedBorderState::Normal);
        assert!(!target.finalize_shared_borders_step().unwrap());
        let mut source = DrawList::default();
        let before = source.layers.len();
        let (result, _) = target.begin_retained_append(&mut source);
        assert!(matches!(result, Err(RetainedOutputError::AppendActive)));
        assert_eq!(source.layers.len(), before);
    }
}
