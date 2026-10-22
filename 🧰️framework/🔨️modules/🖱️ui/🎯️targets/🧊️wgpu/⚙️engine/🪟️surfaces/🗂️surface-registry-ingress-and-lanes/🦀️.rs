//! 🗂️ Surface registry, ingress and lanes: the frame step, the presented source identity, the surface registry, the document ingress, the surface lanes, the theme propagation and the layout step. See [`super`].

use super::*;

//#region 🔖️UiWindow
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiFrameStep {
    Pending,
    Ready,
    Missing,
    Fault,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct UiPresentedSourceIdentity{pub operation:semio_framework_job::OperationId,pub generation:semio_framework_job::Generation,pub revision:u64,pub accessibility:u64,pub interaction:u64,pub clock:usize,pub deadline:u64}

/// 🎟️ One original caller holds the accepted semantic tree until its paid producer returns this loan.
pub struct UiPresentedTreeLoan{pub surface:UiSurfaceToken,pub identity:UiPresentedSourceIdentity,pub active:bool}

impl Drop for UiPresentedTreeLoan{fn drop(&mut self){assert!(std::thread::panicking()||!self.active,"accepted tree loan reached Drop before original paid return")}}

#[derive(Debug, PartialEq, Eq)]
pub struct UiSurfaceAdmissionRejected {
    pub id: SurfaceId,
}

pub struct UiSurfaceSlot {
    pub id: SurfaceId,
    pub generation: u64,
    pub window: UiWindow,
}

/// 🧱️ The retained window table. `slots` is boxed because `UiSurfaceSlot` carries a whole
/// [`UiWindow`] (~186 KiB): held inline, the 64-slot array is ~11.9 MiB of `Ui`, materialised in the
/// constructing frame by `array::from_fn` and copied again by every by-value move of `Ui`
/// (`Ui::new` → `Mutex::new` → `OnceLock::get_or_init` stacked ~88 MiB). Boxed, the array is built
/// straight on the heap by [`semio_framework_async_slots::boxed_fixed_slots`] and `Ui` stays pointer-sized
/// here. `generations` is `[u64; 64]` = 512 B and stays inline.
pub struct UiSurfaceRegistry {
    pub slots: Box<[Option<UiSurfaceSlot>; UI_LAYOUT_SURFACE_SLOTS]>,
    pub generations: [u64; UI_LAYOUT_SURFACE_SLOTS],
}

impl Default for UiSurfaceRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| None), generations: [0; UI_LAYOUT_SURFACE_SLOTS] }
    }
}

impl UiSurfaceRegistry {
    pub fn token(&self, id: &str) -> Option<UiSurfaceToken> {
        let slot = self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.id.as_ref() == id))?;
        Some(UiSurfaceToken::new(slot as u8, self.slots[slot].as_ref()?.generation))
    }

    #[expect(clippy::result_large_err, reason = "Surface and document admission return the exact refused identity or page without allocating a rejection wrapper.")]
    pub fn try_admit(&mut self, id: SurfaceId) -> Result<UiSurfaceToken, UiSurfaceAdmissionRejected> {
        if let Some(token) = self.token(id.as_ref()) {
            return if self.get_token(token).is_some_and(|window| window.closing.is_none()) { Ok(token) } else { Err(UiSurfaceAdmissionRejected { id }) };
        }
        let Some(slot) = self.slots.iter().position(Option::is_none) else { return Err(UiSurfaceAdmissionRejected { id }) };
        let Some(generation) = self.generations[slot].checked_add(1) else { return Err(UiSurfaceAdmissionRejected { id }) };
        self.generations[slot] = generation;
        let token = UiSurfaceToken::new(slot as u8, generation);
        self.slots[slot] = Some(UiSurfaceSlot { window: UiWindow::new(id.as_ref()), id, generation });
        Ok(token)
    }

    pub fn get(&self, id: &str) -> Option<&UiWindow> {
        self.get_token(self.token(id)?)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut UiWindow> {
        self.get_token_mut(self.token(id)?)
    }

    pub fn get_token(&self, token: UiSurfaceToken) -> Option<&UiWindow> {
        let slot = self.slots.get(token.slot() as usize)?.as_ref()?;
        (slot.generation == token.generation()).then_some(&slot.window)
    }

    pub fn get_token_mut(&mut self, token: UiSurfaceToken) -> Option<&mut UiWindow> {
        let slot = self.slots.get_mut(token.slot() as usize)?.as_mut()?;
        (slot.generation == token.generation()).then_some(&mut slot.window)
    }

    pub fn remove_terminal(&mut self, token: UiSurfaceToken) -> bool {
        let Some(window) = self.get_token(token) else { return true };
        if window.closing != Some(UiSurfaceClosePhase::Complete)||window.layout_close_failure.is_some(){
            return false;
        }
        self.slots[token.slot() as usize] = None;
        true
    }

    pub fn id(&self, token: UiSurfaceToken) -> Option<&SurfaceId> {
        let slot = self.slots.get(token.slot() as usize)?.as_ref()?;
        (slot.generation == token.generation()).then_some(&slot.id)
    }

    pub fn token_at(&self, index: usize) -> Option<UiSurfaceToken> {
        let slot = self.slots.get(index)?.as_ref()?;
        Some(UiSurfaceToken::new(index as u8, slot.generation))
    }

    pub fn values(&self) -> impl Iterator<Item = &UiWindow> {
        self.slots.iter().filter_map(|slot| slot.as_ref().map(|slot| &slot.window))
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut UiWindow> {
        self.slots.iter_mut().filter_map(|slot| slot.as_mut().map(|slot| &mut slot.window))
    }

    pub fn ids(&self) -> impl Iterator<Item = &SurfaceId> {
        self.slots.iter().filter_map(|slot| slot.as_ref().map(|slot| &slot.id))
    }
}

pub struct UiSealedVisibilityCandidate {
    pub witness: u64,
    pub surfaces: [Option<(UiSurfaceToken, bool)>; UI_LAYOUT_SURFACE_SLOTS],
}
//#endregion 🔖️UiWindow

//#region 📄️DocumentIngress
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentIngressFault {
    Cancelled,
    Deadline,
    StaleGeneration,
    InterruptedClose,
    ValidationPending,
    Invalid(UiDocumentTreeFault),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiDocumentIngressStatus {
    Vacant,
    Pending { next_page: usize, node_count: usize },
    Published,
}

/// 🪪️ The ingress generation one surface's next document must carry, given what this producer last
/// minted for that surface (`(published_revision, generation)`) and the revision the producer is
/// publishing now.
///
/// ⚖️ Declared HERE, beside the two admission rules it has to satisfy, because a producer that gets it
/// wrong is refused silently: [`Ui::document_status`] answers `Published` while the window's published
/// document carries the same generation — so a repeat generation makes the whole ingress phase a
/// no-op — and [`Ui::begin_document`] refuses `StaleGeneration` unless the incoming generation is
/// strictly GREATER than the published one. The generation is therefore "which ingress is this", never
/// "who produced it".
///
/// 🩸️ The browser producer minted the plugin INSTANCE id, a constant for the session and identical for
/// every surface: each surface ingested exactly one document ever, and every later document — a
/// correctly re-rendered one — was answered `Published` and dropped on the floor, so the arena
/// repainted the first tree forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-generation-publication-2026-09-13.md`).
///
/// 🧮️ Keyed on the producer's own revision so an UNCHANGED surface keeps its generation and pays no
/// ingress at all, and COUNTED rather than mirrored from that revision so a surface whose revision
/// restarts (a retired surface reopened under a fresh owner) still moves forward instead of being
/// refused as stale for the rest of the session.
pub fn ui_document_ingress_generation(minted: Option<(u64, u64)>, revision: u64) -> u64 {
    match minted {
        Some((published_revision, generation)) if published_revision == revision => generation,
        Some((_, generation)) => generation.saturating_add(1),
        None => 1,
    }
}

pub struct UiDocumentIngress {
    pub document: UiDocumentTree,
    pub next_page: usize,
    pub node_count: usize,
    pub validation_cursor: usize,
    pub validation_started: bool,
    pub validation_stack: UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
    pub validation_seen: UiFixedList<UiNodeId, UI_DOCUMENT_NODES>,
}
//#endregion 📄️DocumentIngress

//#region 🚦️SurfaceScheduling
/// 🚦️Priority lane for resumable per-surface layout. The weighted wheel favors direct
/// interaction without allowing user-visible or background surfaces to starve.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SurfaceLane {
    Interactive,
    #[default]
    UserVisible,
    Background,
}

impl SurfaceLane {
    pub const fn index(self) -> usize {
        match self {
            Self::Interactive => 0,
            Self::UserVisible => 1,
            Self::Background => 2,
        }
    }
}

pub struct SurfaceLaneRing {
    pub slots: [Option<SurfaceLaneEntry>; UI_LAYOUT_SURFACE_SLOTS],
    pub head: usize,
    pub len: usize,
}

impl Default for SurfaceLaneRing {
    fn default() -> Self {
        Self { slots: [None; UI_LAYOUT_SURFACE_SLOTS], head: 0, len: 0 }
    }
}

impl SurfaceLaneRing {
    pub fn remove_token_one(&mut self, token: UiSurfaceToken) -> bool {
        let Some(offset) = (0..self.len).find(|offset| self.slots[(self.head + offset) % UI_LAYOUT_SURFACE_SLOTS].is_some_and(|entry| entry.token == token)) else { return false };
        for offset in offset..self.len - 1 {
            self.slots[(self.head + offset) % UI_LAYOUT_SURFACE_SLOTS] = self.slots[(self.head + offset + 1) % UI_LAYOUT_SURFACE_SLOTS].take();
        }
        self.slots[(self.head + self.len - 1) % UI_LAYOUT_SURFACE_SLOTS] = None;
        self.len -= 1;
        true
    }

    pub fn try_push(&mut self, entry: SurfaceLaneEntry) -> Result<(), SurfaceLaneEntry> {
        if self.len == UI_LAYOUT_SURFACE_SLOTS {
            return Err(entry);
        }
        let tail = (self.head + self.len) % UI_LAYOUT_SURFACE_SLOTS;
        self.slots[tail] = Some(entry);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<SurfaceLaneEntry> {
        if self.len == 0 {
            return None;
        }
        let token = self.slots[self.head].take();
        self.head = (self.head + 1) % UI_LAYOUT_SURFACE_SLOTS;
        self.len -= 1;
        token
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceLayoutReason {
    Dirty,
    Metrics,
    Theme,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SurfaceLaneEntry {
    pub token: UiSurfaceToken,
    pub reason: SurfaceLayoutReason,
    pub epoch: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemePropagationPhase {
    Validate,
    Apply,
    Publish,
}

pub struct ThemePropagationCursor {
    pub theme: Theme,
    pub tokens: [Option<UiSurfaceToken>; UI_LAYOUT_SURFACE_SLOTS],
    pub slot: usize,
    pub phase: ThemePropagationPhase,
}

impl ThemePropagationCursor {
    pub fn new(theme: Theme) -> Self {
        Self { theme, tokens: [None; UI_LAYOUT_SURFACE_SLOTS], slot: 0, phase: ThemePropagationPhase::Validate }
    }
}

pub const LANE_WHEEL: [SurfaceLane; 6] = [SurfaceLane::Interactive, SurfaceLane::Interactive, SurfaceLane::UserVisible, SurfaceLane::Interactive, SurfaceLane::UserVisible, SurfaceLane::Background];

/// 🧭️Observable result of exactly one bounded surface-layout scheduling call. `Awaiting`: `window_id`'s layout step runs on a
/// pool worker and has not answered yet — nothing ran on this thread.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UiLayoutStep {
    Idle,
    Yielded { window_id: SurfaceId, lane: SurfaceLane, stage: &'static str, nodes: usize, glyphs: usize },
    Awaiting { window_id: SurfaceId, lane: SurfaceLane },
    Ready { window_id: SurfaceId, lane: SurfaceLane },
    Cancelled { window_id: SurfaceId, lane: SurfaceLane },
    Refused { window_id: SurfaceId, lane: SurfaceLane, refusal: UiWorkerRetirementRefusal },
}

pub fn stage_label(stage: LayoutJobStage) -> &'static str {
    match stage {
        LayoutJobStage::CollectNodes => "Layout.CollectNodes",
        LayoutJobStage::ShapeText => "Layout.ShapeText",
        LayoutJobStage::MeasureLayout => "Layout.MeasureLayout",
        LayoutJobStage::SolveLayout => "Layout.SolveLayout",
        LayoutJobStage::CollectResults => "Layout.CollectResults",
        LayoutJobStage::PublishResults => "Layout.PublishResults",
    }
}

pub fn theme_layout_identity(theme: &Theme) -> [u32; 4] {
    [theme.gap_standard.to_bits(), theme.padding_standard.to_bits(), theme.font_size_small.to_bits(), theme.font_size_body.to_bits()]
}

pub fn worker_lane(lane: SurfaceLane) -> semio_framework_async::Lane {
    match lane {
        SurfaceLane::Interactive => semio_framework_async::Lane::Interactive,
        SurfaceLane::UserVisible => semio_framework_async::Lane::UserVisible,
        SurfaceLane::Background => semio_framework_async::Lane::Background,
    }
}
//#endregion 🚦️SurfaceScheduling
