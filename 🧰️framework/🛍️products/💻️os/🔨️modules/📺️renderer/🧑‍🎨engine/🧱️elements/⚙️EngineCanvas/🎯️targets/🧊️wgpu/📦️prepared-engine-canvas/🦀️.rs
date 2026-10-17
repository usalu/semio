//! 📦️ Prepared engine canvas packets, staging, GPU slots and the presenter.
//!
//! The owned vector packets the worker produces, the staged scene registry, the build context, the GPU candidate
//! and slot and the presenter that realizes a packet on the UI thread.

use super::engine_surface_registry::{
    engine_canvas_debug_log, engine_surface_live_freshness, EngineSurfaceId, EngineSurfaceIdentity,
    EngineSurfaceLiveFreshness, EngineSurfaceSnapshot, EngineSurfaceToken, ENGINE_SURFACE_CAPACITY,
};
use crate::element_contracts::WorkerCell;
use canvas::Color;
use semio_framework_canvas as canvas;
use std::mem::ManuallyDrop;
use ui_gpu::{GpuContext,EngineRasterBuild,EngineRasterStageFault};
pub use ui_gpu::EngineRasterBuildPhase as EngineGpuBuildPhase;

//#region 📦️PreparedEngineCanvas
/// 📦️ An owned vector scene produced during worker-side product traversal. Device, queue, texture,
/// and window handles are deliberately absent; the UI presenter realizes this packet only after its
/// enclosing frame generation has passed the prepared-render gate.
pub struct EngineCanvasPacket {
    pub surface: EngineSurfaceIdentity,
    pub document_generation: u64,
    pub scene_revision: u64,
    pub metrics_generation: u64,
    pub scene: ManuallyDrop<canvas::Scene>,
    pub scene_retirement: ManuallyDrop<canvas::CanvasSceneRetirement>,
    pub clear: Color,
    pub width: u32,
    pub height: u32,
}

impl EngineCanvasPacket {
    pub fn next_close_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if !self.surface.id.terminal_is_empty(){return Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()})}self.scene_retirement.next_demands(&self.scene,copy)}
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        if self.terminal_is_empty(){return Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()})}
        let demand=self.next_close_demands(cx.retained_grant().maximum_copy_bytes).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|{if !self.surface.id.terminal_is_empty(){self.surface.id.close_step();Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))}else{self.scene_retirement.close_step(&mut self.scene,grant)}})?;
        if self.terminal_is_empty(){Ok(semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()})}else{Ok(step)}
    }
    pub fn terminal_is_empty(&self) -> bool {self.surface.id.terminal_is_empty()&&self.scene_retirement.terminal_is_empty(&self.scene)}

}

impl Drop for EngineCanvasPacket{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original EngineCanvas scene dropped before paid retirement");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.scene);ManuallyDrop::drop(&mut self.scene_retirement)}}}}

pub const ENGINE_CANVAS_FRAME_PACKET_CAPACITY: usize = 256;

/// 🎨️ A painted engine surface waiting for the frame build's own packet phase to admit it. Paint
/// happens deep inside the retained chrome walk, which owns no `EngineCanvasBuildContext`; staging
/// here is the one seam between the two, and the newest paint for a surface replaces the older one
/// so a frame that never drained cannot accumulate scenes.
pub struct StagedEngineScene {
    pub surface: EngineSurfaceSnapshot,
    pub document_generation: u64,
    pub scene_revision: u64,
    pub scene: canvas::Scene,
    pub clear: Color,
    pub width: u32,
    pub height: u32,
}

pub struct StagedEngineScenes {
    pub(super) slots: Box<[Option<StagedEngineScene>; ENGINE_CANVAS_FRAME_PACKET_CAPACITY]>,
    pub len: usize,
    pub(super) faulted: bool,
}

impl Default for StagedEngineScenes {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), len: 0, faulted: false }
    }
}

impl StagedEngineScenes {
    pub fn upsert(&mut self, staged: StagedEngineScene) {
        if let Some(slot) = self.slots[..self.len].iter_mut().flatten().find(|slot| slot.surface.identity.id == staged.surface.identity.id) {
            *slot = staged;
            return;
        }
        if self.len == ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            self.faulted = true;
            return;
        }
        self.slots[self.len] = Some(staged);
        self.len += 1;
    }

    pub fn take_one(&mut self) -> Option<StagedEngineScene> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.slots[index].take()
    }

    pub fn remove_surface(&mut self, surface_id: &str) {
        let Some(index) = self.slots[..self.len].iter().position(|slot| slot.as_ref().is_some_and(|staged| staged.surface.identity.id.as_str() == surface_id)) else {
            return;
        };
        self.len -= 1;
        self.slots[index] = self.slots[self.len].take();
    }

    pub(super) fn remove_exact(&mut self, identity: EngineSurfaceIdentity) {
        let Some(index) = self.slots[..self.len].iter().position(|slot| slot.as_ref().is_some_and(|staged| staged.surface.identity == identity)) else {
            return;
        };
        self.len -= 1;
        self.slots[index] = self.slots[self.len].take();
    }
}

pub static STAGED_ENGINE_SCENES: WorkerCell<StagedEngineScenes> = WorkerCell::new(Default::default);

/// 📦️ One turn of the staged-paint → frame-packet transfer, driven from the frame build's own
/// `EngineTransfer` phase. Returns `true` when nothing is left staged.
pub fn stage_engine_packet_step(resources: &mut EngineCanvasBuildContext) -> bool {
    let Some(staged) = STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().take_one()) else {
        return true;
    };
    if let Err(staged) = resources.publish_staged(staged) {
        STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().upsert(staged));
        return true;
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(any(test, feature = "testkit"))]
pub enum EngineCanvasPacketDestination {
    Ready(usize),
    Rejected(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(test)]
struct EngineCanvasPacketReservation {
    pub(super) generation: u64,
    pub sequence: u64,
    pub destination: EngineCanvasPacketDestination,
    pub(super) surface: EngineSurfaceSnapshot,
}

/// 🧰️ Worker-owned fixed resource sink threaded through chrome/scene traversal.
pub struct EngineCanvasBuildContext {
    #[cfg(test)]
    pub document_generation: u64,
    #[cfg(test)]
    pub scene_revision: u64,
    packets: semio_framework_async::OriginalFixedSlotPages<EngineCanvasPacket,16,16>,
    rejected: semio_framework_async::OriginalFixedSlotPages<EngineCanvasPacket,16,16>,
    pub len: usize,
    pub rejected_len: usize,
    reservation_sequence: u64,
    published_reservation_sequence: u64,
    outstanding_reservations: usize,
}

impl Default for EngineCanvasBuildContext {
    fn default() -> Self {
        Self {
            #[cfg(test)]
            document_generation: 0,
            #[cfg(test)]
            scene_revision: 0,
            packets: semio_framework_async::OriginalFixedSlotPages::new(),
            rejected: semio_framework_async::OriginalFixedSlotPages::new(),
            len: 0,
            rejected_len: 0,
            reservation_sequence: 0,
            published_reservation_sequence: 0,
            outstanding_reservations: 0,
        }
    }
}

impl EngineCanvasBuildContext {
    #[cfg(any(test, feature = "testkit"))]
    pub fn new(document_generation: u64, scene_revision: u64) -> Self {
        Self { document_generation, scene_revision, ..Self::default() }
    }

    pub(crate) fn prepared(&self)->bool{self.packets.prepared()&&self.rejected.prepared()}
    pub fn prepare_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(semio_framework_job::OriginalTurnFailure::debit_original(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::Canceled,"original EngineCanvas page birth canceled"),cx))}
        if self.prepared(){return Ok(true)}
        let bytes=if !self.packets.prepared(){self.packets.birth_bytes()}else{self.rejected.birth_bytes()};
        semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{capacity_bytes:bytes,depth:1,..Default::default()},|grant|if !self.packets.prepared(){self.packets.birth(grant)}else{self.rejected.birth(grant)})?;Ok(self.prepared())
    }
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_job::InteractiveJobCloseStep as Close;
        if self.terminal_is_empty(){return Ok(Close::Complete{progress:Default::default()})}
        let index=self.rejected_len.checked_sub(1).map(|index|(true,index)).or_else(||self.len.checked_sub(1).map(|index|(false,index)));
        if let Some((rejected,index))=index{
            let slot=if rejected{&mut self.rejected[index]}else{&mut self.packets[index]};
            if let Some(packet)=slot.as_mut(){let step=packet.close_step(cx)?;if packet.terminal_is_empty(){*slot=None;if rejected{self.rejected_len=index}else{self.len=index}}return Ok(match step{Close::Complete{progress}=>Close::Pending{progress},step=>step})}
            return semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|_|{if rejected{self.rejected_len=index}else{self.len=index}Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))})
        }
        let bytes=if !self.packets.empty(){self.packets.release_bytes()}else{self.rejected.release_bytes()};
        if bytes!=0{return semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{release_bytes:bytes,depth:1,..Default::default()},|grant|if !self.packets.empty(){self.packets.release(grant)}else{self.rejected.release(grant)})}
        semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{depth:1,..Default::default()},|_|{self.outstanding_reservations=0;self.published_reservation_sequence=self.reservation_sequence;Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))})
    }

    pub fn take_packet_step(&mut self) -> Result<Option<EngineCanvasPacket>, EngineCanvasPacket> {
        if let Some(index) = self.rejected_len.checked_sub(1) {
            self.rejected_len = index;
            if let Some(rejected) = self.rejected[index].take() {
                return Err(rejected);
            }
        }
        let Some(index) = self.len.checked_sub(1) else { return Ok(None) };
        self.len = index;
        Ok(self.packets[index].take())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.rejected_len == 0 && self.reservation_sequence == self.published_reservation_sequence && self.outstanding_reservations == 0 && self.packets.empty() && self.rejected.empty()
    }

    /// 📦️ Admits one painted engine surface as a frame packet. The staged scene carries the surface's
    /// OWN document/scene/metrics generations rather than the build context's, so the GPU freshness
    /// gate (`engine_gpu_freshness_matches`) compares like with like and a graph that did not move
    /// reuses its texture. Returns the scene back when the frame's packet credits are spent.
    fn publish_staged(&mut self, staged: StagedEngineScene) -> Result<(), StagedEngineScene> {
        if !self.prepared()||self.len == ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            return Err(staged);
        }
        self.packets[self.len] = Some(EngineCanvasPacket {
            surface: staged.surface.identity,
            document_generation: staged.document_generation,
            scene_revision: staged.scene_revision,
            metrics_generation: staged.surface.metrics_generation,
            scene: ManuallyDrop::new(staged.scene),
            scene_retirement: Default::default(),
            clear: staged.clear,
            width: staged.width.max(1),
            height: staged.height.max(1),
        });
        self.len += 1;
        Ok(())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn try_reserve_fresh_packet(&mut self, surface: EngineSurfaceSnapshot) -> Result<EngineCanvasPacketReservation, EngineSurfaceSnapshot> {
        let destination = if self.len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            let index = self.len;
            self.len += 1;
            EngineCanvasPacketDestination::Ready(index)
        } else if self.rejected_len < ENGINE_CANVAS_FRAME_PACKET_CAPACITY {
            let index = self.rejected_len;
            self.rejected_len += 1;
            EngineCanvasPacketDestination::Rejected(index)
        } else {
            return Err(surface);
        };
        let Some(sequence) = self.reservation_sequence.checked_add(1) else {
            match destination {
                EngineCanvasPacketDestination::Ready(_) => self.len -= 1,
                EngineCanvasPacketDestination::Rejected(_) => self.rejected_len -= 1,
            }
            return Err(surface);
        };
        let Some(outstanding_reservations) = self.outstanding_reservations.checked_add(1) else {
            match destination {
                EngineCanvasPacketDestination::Ready(_) => self.len -= 1,
                EngineCanvasPacketDestination::Rejected(_) => self.rejected_len -= 1,
            }
            return Err(surface);
        };
        self.reservation_sequence = sequence;
        self.outstanding_reservations = outstanding_reservations;
        Ok(EngineCanvasPacketReservation { generation: self.document_generation, sequence, destination, surface })
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn publish_reserved(&mut self, reservation: EngineCanvasPacketReservation, scene: canvas::Scene, clear: Color, width: u32, height: u32) {
        self.published_reservation_sequence = reservation.sequence;
        let packet = EngineCanvasPacket {
            surface: reservation.surface.identity,
            document_generation: reservation.generation,
            scene_revision: self.scene_revision,
            metrics_generation: reservation.surface.metrics_generation,
            scene:ManuallyDrop::new(scene),
            scene_retirement: Default::default(),
            clear,
            width,
            height,
        };
        match reservation.destination {
            EngineCanvasPacketDestination::Ready(index) => self.packets[index] = Some(packet),
            EngineCanvasPacketDestination::Rejected(index) => self.rejected[index] = Some(packet),
        }
        self.outstanding_reservations -= 1;
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<EngineCanvasPacket>();
    assert_send::<EngineCanvasBuildContext>();
};

pub struct EngineGpuCandidate {
    pub surface: EngineSurfaceIdentity,
    pub document_generation: u64,
    pub scene_revision: u64,
    pub metrics_generation: u64,
    primary_metrics_generation: u64,
    raster_generation: ui_gpu::RasterTextureWitness,
    pub width: u32,
    pub height: u32,
    gpu: Option<EngineRasterBuild>,
    pub phase: EngineGpuBuildPhase,
}

impl EngineGpuCandidate {
    pub fn new(packet: &EngineCanvasPacket, raster_generation: ui_gpu::RasterTextureWitness, primary_metrics_generation: u64) -> Self {
        Self {
            surface: packet.surface,
            document_generation: packet.document_generation,
            scene_revision: packet.scene_revision,
            metrics_generation: packet.metrics_generation,
            primary_metrics_generation,
            raster_generation,
            width: packet.width.max(1),
            height: packet.height.max(1),
            gpu: None,
            phase: EngineGpuBuildPhase::Reserve,
        }
    }

    pub fn matches(&self, packet: &EngineCanvasPacket, expected: ui_gpu::RasterTextureWitness, primary_metrics_generation: u64) -> bool {
        self.surface == packet.surface
            && self.document_generation == packet.document_generation
            && self.scene_revision == packet.scene_revision
            && self.metrics_generation == packet.metrics_generation
            && self.primary_metrics_generation == primary_metrics_generation
            && self.raster_generation == expected
            && self.width == packet.width.max(1)
            && self.height == packet.height.max(1)
    }

    fn matches_live(&self, live: EngineSurfaceLiveFreshness) -> bool {
        engine_gpu_freshness_matches(self.surface, self.document_generation, self.scene_revision, self.metrics_generation, live)
    }

    fn content_identity(&self) -> Result<ui_wgpu::wgpu::RasterContentIdentity, String> {
        ui_wgpu::wgpu::RasterContentIdentity::revision(
            self.width,
            self.height,
            [u64::from(self.surface.token.slot), self.surface.token.generation, self.document_generation, self.scene_revision, self.metrics_generation, self.primary_metrics_generation],
        )
        .ok_or_else(|| "engine raster content identity overflowed".to_string())
    }

    pub fn begin_close(&mut self) {if self.phase!=EngineGpuBuildPhase::Terminal{if let Some(build)=self.gpu.as_mut(){build.begin_close();}self.phase=EngineGpuBuildPhase::ClosingAdmission;}}
    pub fn close_step(&mut self,gpu:&mut GpuContext)->Result<bool,String>{if let Some(build)=self.gpu.as_mut(){if !gpu.close_engine_raster_step(build)?{return Ok(false)}if !build.terminal_is_empty(){return Err("engine GPU build retained original resources".into())}self.gpu=None;return Ok(false)}self.phase=EngineGpuBuildPhase::Terminal;Ok(true)}
    pub fn terminal_is_empty(&self)->bool{self.phase==EngineGpuBuildPhase::Terminal&&self.gpu.as_ref().is_none_or(EngineRasterBuild::terminal_is_empty)}

}

pub fn engine_gpu_freshness_matches(surface: EngineSurfaceIdentity, document_generation: u64, scene_revision: u64, metrics_generation: u64, live: EngineSurfaceLiveFreshness) -> bool {
    surface == live.identity && document_generation == live.document_generation && scene_revision == live.scene_revision && metrics_generation == live.metrics_generation
}

pub struct EngineGpuSlot {
    pub id: Option<EngineSurfaceId>,
    pub generation: u64,
    pub exhausted: bool,
    pub candidate: Option<EngineGpuCandidate>,
    closing: bool,
}

impl EngineGpuSlot {
    pub fn new() -> Self {
        Self { id: None, generation: 0, exhausted: false, candidate: None, closing: false }
    }

    pub fn token(&self, slot: usize) -> Option<EngineSurfaceToken> {
        (self.id.is_some() || self.candidate.is_some()).then_some(EngineSurfaceToken { slot: slot as u16, generation: self.generation })
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.id.is_none() && self.candidate.is_none()
    }

    pub fn publish_candidate(&mut self, packet: &EngineCanvasPacket, expected: ui_gpu::RasterTextureWitness, primary_metrics_generation: u64) -> Result<bool, String> {
        let Some(candidate) = self.candidate.as_mut() else {
            return Err("engine GPU publication candidate disappeared".to_string());
        };
        if !candidate.matches(packet, expected, primary_metrics_generation) {
            candidate.begin_close();
            return Err("engine CPU/GPU publication freshness changed".to_string());
        }
        if candidate.gpu.as_ref().is_some_and(|build|!build.terminal_is_empty()) {
            candidate.begin_close();
            return Err("engine GPU publication candidate was incomplete".to_string());
        }
        let Some(mut candidate) = self.candidate.take() else {
            return Err("engine GPU publication candidate transfer failed".to_string());
        };
        candidate.phase = EngineGpuBuildPhase::Terminal;
        if !candidate.terminal_is_empty() {
            candidate.begin_close();
            self.candidate = Some(candidate);
            return Err("engine GPU publication retained an unclassified candidate owner".to_string());
        }
        self.candidate = Some(candidate);
        Ok(false)
    }
}

/// 🖥️ UI-capability owner for EngineCanvas GPU realization.
pub struct EngineCanvasPresenter {
    pub slots: ManuallyDrop<Option<Box<[EngineGpuSlot; ENGINE_SURFACE_CAPACITY]>>>,
    primary_metrics_generation: u64,
    pub metrics_invalidation_scan: Option<usize>,
    /// 🩺️ Consecutive `Ok(false)` answers from [`Self::realize_step`], and which arm gave the last
    /// one. A permanent `Ok(false)` here stops EVERY frame in the shell — the present cursor stays
    /// pending and the frame gate admits no build — and the two arms that can answer it forever are
    /// indistinguishable from outside, so the host's own `frame gate blocked=true phase=Some(Engine)`
    /// line names the phase and nothing names the holder
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4).
    stall_steps: u64,
    stall_arm: &'static str,
}

impl Default for EngineCanvasPresenter {
    fn default() -> Self {
        Self { slots: ManuallyDrop::new(Some(semio_framework_async::boxed_fixed_slots(EngineGpuSlot::new))), primary_metrics_generation: 0, metrics_invalidation_scan: None, stall_steps: 0, stall_arm: "" }
    }
}

impl EngineCanvasPresenter {
    pub fn slots(&self) -> Result<&[EngineGpuSlot; ENGINE_SURFACE_CAPACITY], String> {
        self.slots.as_deref().ok_or_else(|| "engine presenter fixed slot authority was abandoned".to_string())
    }

    pub fn slots_mut(&mut self) -> Result<&mut [EngineGpuSlot; ENGINE_SURFACE_CAPACITY], String> {
        self.slots.as_deref_mut().ok_or_else(|| "engine presenter fixed slot authority was abandoned".to_string())
    }

    pub fn observe_primary_metrics_generation(&mut self, generation: u64) -> bool {
        if generation <= self.primary_metrics_generation {
            return generation == self.primary_metrics_generation;
        }
        self.primary_metrics_generation = generation;
        self.metrics_invalidation_scan = Some(0);
        true
    }

    pub fn invalidate_primary_metrics_step(&mut self) -> bool {
        let Some(index) = self.metrics_invalidation_scan else { return true };
        if index == ENGINE_SURFACE_CAPACITY {
            self.metrics_invalidation_scan = None;
            return true;
        }
        if let Ok(slots) = self.slots_mut() {
            if let Some(candidate) = slots.get_mut(index).and_then(|slot| slot.candidate.as_mut()) {
                candidate.begin_close();
            }
        }
        self.metrics_invalidation_scan = index.checked_add(1);
        self.metrics_invalidation_scan.is_none()
    }

    /// 🩺️ Names the arm that answered `Ok(false)`, on a power-of-two cadence so a healthy step costs
    /// one line and a wedge names itself instead of leaving `phase=Some(Engine)` unexplained. Reset by
    /// the first `Ok(true)`/`Err` [`Self::realize_step`] takes.
    fn note_realize_stall(&mut self, arm: &'static str) {
        if self.stall_arm != arm {
            self.stall_arm = arm;
            self.stall_steps = 0;
        }
        self.stall_steps = self.stall_steps.saturating_add(1);
        if self.stall_steps.is_power_of_two() && self.stall_steps >= 64 {
            engine_canvas_debug_log(&format!("[TRACE] engine realize stalled arm={arm} steps={} scan={:?}", self.stall_steps, self.metrics_invalidation_scan));
        }
    }

    /// 🌱 A pending primary-metrics invalidation is THIS authority's own unfinished work, not a
    /// foreign fault: the surface-resize cursor arms the scan and drains it on a different cadence
    /// from the present cursor, so a frame that reaches the engine mid-scan used to quarantine the
    /// whole surface with `engine primary metrics invalidation is pending`. Drive one scan unit and
    /// answer "not yet" — the scan is bounded by `ENGINE_SURFACE_CAPACITY`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    /// 🐕️ …and it is DRAINED HERE, not advanced one slot per frame. One slot per `realize_step`
    /// spends `ENGINE_SURFACE_CAPACITY` (256) present steps on a scan whose whole work is
    /// `candidate.begin_close()` per slot — microseconds of work paid for with a quarter of a
    /// thousand blocked frames, during which `admit_next_frame` builds nothing and the whole host
    /// stops. Measured on 6118: `[TRACE] engine realize stalled arm=metrics-invalidation-scan
    /// steps=128 scan=Some(212)` on every example boot, with the frame gate reading
    /// `blocked=true phase=Some(Engine)` across it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
    pub fn realize_step(&mut self, gpu: &mut GpuContext, packet: &EngineCanvasPacket, candidate_generation: ui_gpu::RasterTextureWitness, expected: ui_gpu::RasterTextureWitness) -> Result<bool, String> {
        if candidate_generation != expected {
            return Err("engine raster operation authority was stale before realization".to_string());
        }
        if self.metrics_invalidation_scan.is_some() {
            for _ in 0..=ENGINE_SURFACE_CAPACITY {
                if self.invalidate_primary_metrics_step() {
                    break;
                }
            }
            self.note_realize_stall("metrics-invalidation-scan");
            return Ok(false);
        }
        let primary_metrics_generation = self.primary_metrics_generation;
        let index = usize::from(packet.surface.token.slot);
        let slot = self.slots_mut()?.get_mut(index).ok_or_else(|| "engine surface token exceeded fixed GPU slots".to_string())?;
        if slot.closing || slot.exhausted {
            return Err("engine surface slot was closing or exhausted".to_string());
        }
        self.stall_arm = "";
        self.stall_steps = 0;
        let slot = self.slots_mut()?.get_mut(index).ok_or_else(|| "engine surface token exceeded fixed GPU slots".to_string())?;
        if slot.id.is_none() {
            if slot.generation != 0 && slot.generation != packet.surface.token.generation {
                return Err("engine surface GPU generation was stale before reservation".to_string());
            }
            slot.id = Some(packet.surface.id);
            slot.generation = packet.surface.token.generation;
        }
        if slot.generation != packet.surface.token.generation || slot.id != Some(packet.surface.id) {
            return Err("engine surface CPU/GPU identity pair was stale".to_string());
        }
        if slot.candidate.is_none() {
            slot.candidate = Some(EngineGpuCandidate::new(packet, candidate_generation, primary_metrics_generation));
            return Ok(false);
        }
        let build = slot.candidate.as_mut().ok_or_else(|| "engine GPU candidate disappeared".to_string())?;
        if !build.matches(packet, expected, primary_metrics_generation) {
            return Err("engine GPU candidate freshness changed before step".to_string());
        }
        match build.phase {
            EngineGpuBuildPhase::Reserve => {
                let identity = build.content_identity()?;
                let surface_id = build.surface.id;
                surface_id.with_raster_key(|key| -> Result<(), String> {
                    if !gpu.prepare_raster_admission_step(key, build.width, build.height, identity, candidate_generation)? {
                        return Ok(());
                    }
                    if gpu.raster_content_is_reusable(key, identity, candidate_generation, expected)? {
                        build.phase = EngineGpuBuildPhase::Publish;
                    } else {
                        build.gpu = Some(gpu.begin_engine_raster(key, build.width, build.height, identity, candidate_generation, expected)?);
                        build.phase = EngineGpuBuildPhase::Texture;
                    }
                    Ok(())
                })?;
            }
            EngineGpuBuildPhase::Texture|EngineGpuBuildPhase::View|EngineGpuBuildPhase::Renderer|EngineGpuBuildPhase::Render|EngineGpuBuildPhase::Stage|EngineGpuBuildPhase::RetireRenderer=>{
                let owner=build.gpu.take().ok_or_else(||"engine GPU build owner was missing".to_string())?;
                match gpu.engine_raster_step(owner,&packet.scene,packet.clear,expected){Ok(owner)=>{build.phase=owner.phase();build.gpu=Some(owner);},Err(EngineRasterStageFault::Returned{fault,build:owner})|Err(EngineRasterStageFault::Retained{fault,build:owner})=>{build.gpu=Some(owner);build.begin_close();return Err(fault);}}
            }
            EngineGpuBuildPhase::Publish => {
                let live = match engine_surface_live_freshness(packet.surface.token) {
                    Ok(Some(live)) => live,
                    Ok(None) => {
                        build.begin_close();
                        return Err("engine CPU surface disappeared before GPU publication".to_string());
                    }
                    Err(()) => return Ok(false),
                };
                if !build.matches_live(live) {
                    build.begin_close();
                    return Err("engine CPU surface freshness changed before GPU publication".to_string());
                }
                return slot.publish_candidate(packet, expected, primary_metrics_generation);
            }
            EngineGpuBuildPhase::ClosingAdmission | EngineGpuBuildPhase::ClosingRenderer | EngineGpuBuildPhase::ClosingView | EngineGpuBuildPhase::ClosingTexture => {
                return Err("engine GPU candidate requires retained close".to_string());
            }
            EngineGpuBuildPhase::Terminal => {
                slot.candidate = None;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn close_active_candidate_step(&mut self, gpu: &mut GpuContext) -> Result<bool, String> {
        let slots = self.slots_mut()?;
        let Some(index) = slots.iter().position(|slot| slot.candidate.is_some()) else { return Ok(true) };
        let Some(candidate) = slots[index].candidate.as_mut() else { return Ok(false) };
        candidate.begin_close();
        if candidate.close_step(gpu)? && candidate.terminal_is_empty() {
            slots[index].candidate = None;
        }
        Ok(false)
    }

    pub fn surface_token_at(&self, index: usize) -> Option<EngineSurfaceToken> {
        self.slots().ok()?.get(index)?.token(index)
    }

    pub fn begin_surface_close(&mut self, token: EngineSurfaceToken) -> bool {
        let Ok(slots) = self.slots_mut() else { return false };
        let Some(slot) = slots.get_mut(usize::from(token.slot)) else { return false };
        if slot.generation != token.generation {
            return false;
        }
        slot.closing = true;
        if let Some(candidate) = slot.candidate.as_mut() {
            candidate.begin_close();
        }
        true
    }

    pub fn close_surface_step(&mut self, gpu: &mut GpuContext, token: EngineSurfaceToken) -> Result<bool, String> {
        let slot = self.slots_mut()?.get_mut(usize::from(token.slot)).ok_or_else(|| "engine close token exceeded fixed GPU slots".to_string())?;
        if slot.generation != token.generation || !slot.closing {
            return Err("engine GPU close token was stale".to_string());
        }
        if let Some(candidate) = slot.candidate.as_mut() {
            candidate.begin_close();
            if candidate.close_step(gpu)? && candidate.terminal_is_empty() {
                slot.candidate = None;
            }
            return Ok(false);
        }
        if let Some(id) = slot.id.as_mut() {
            if !id.terminal_is_empty() {
                id.close_step();
                return Ok(false);
            }
            slot.id = None;
            return Ok(false);
        }
        let Some(generation) = slot.generation.checked_add(1) else {
            slot.exhausted = true;
            slot.closing = false;
            return Ok(true);
        };
        slot.generation = generation;
        slot.closing = false;
        Ok(true)
    }

    pub fn surface_terminal_is_empty(&self, token: EngineSurfaceToken) -> bool {
        self.slots().ok().and_then(|slots| slots.get(usize::from(token.slot))).is_none_or(|slot| slot.generation != token.generation || slot.terminal_is_empty())
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.metrics_invalidation_scan.is_none() && self.slots().is_ok_and(|slots| slots.iter().all(EngineGpuSlot::terminal_is_empty))
    }
}

impl Drop for EngineCanvasPresenter {
    fn drop(&mut self) {
        debug_assert!(self.terminal_is_empty() || std::thread::panicking(), "EngineCanvasPresenter must reach terminal-empty before release");
        if self.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.slots) };
        }
    }
}
//#endregion 📦️PreparedEngineCanvas
