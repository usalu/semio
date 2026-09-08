//! @emoji 🚦️ Pure surface/device state machine: zero-size parking, [`DeviceStatus`] transitions, and
//! the scene-phase classification a [`ui_render::DrawBatch`] falls into. No `wgpu::` type appears in
//! this file — every fn here is exercised by `#[cfg(test)]` without a device, per this packet's brief.

use ui_render::{DeviceStatus, LayerState, LossReason, PhysicalSize};

//#region 🔖️SurfaceState

//#region 🅿️Parking

/// 🅿️ Tracks the surface's physical size/dpr and whether it is parked (zero-size). A backend calls
/// [`Self::resize`] from [`ui_render::GraphicsBackend::resize`] and consults [`Self::is_parked`] before
/// touching the OS surface — the trait's invariant that a zero-size surface parks rather than errors.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SurfaceState {
    pub size: PhysicalSize,
    pub dpr: f32,
}

impl SurfaceState {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn resize(&mut self, size: PhysicalSize, dpr: f32) {
        self.size = size;
        self.dpr = dpr.max(0.0001);
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn is_parked(&self) -> bool {
        self.size.is_zero()
    }
}

//#endregion 🅿️Parking

//#region 🚦️Status

/// 🚦️ The device/surface health tracker behind [`ui_render::GraphicsBackend::device_status`] /
/// [`ui_render::GraphicsBackend::recover`]. `Suboptimal` never escalates itself to `Lost` — only a real
/// surface/device fault (or, under `backend-testing`, [`Self::force_lost`]) does that.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum DeviceHealth {
    #[default]
    Healthy,
    Suboptimal,
    Lost(LossReason),
}

impl DeviceHealth {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub(crate) fn as_status(self) -> DeviceStatus {
        match self {
            DeviceHealth::Healthy => DeviceStatus::Healthy,
            DeviceHealth::Suboptimal => DeviceStatus::Suboptimal,
            DeviceHealth::Lost(reason) => DeviceStatus::Lost(reason),
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub(crate) fn is_lost(self) -> bool {
        matches!(self, DeviceHealth::Lost(_))
    }
}

//#endregion 🚦️Status

//#region 🎬️ScenePhase

/// 🎬️ Which of the frame's four replay passes a [`ui_render::DrawBatch`] belongs to, derived from its
/// `layer_state` alone. `BackdropContent`/`BackdropOverlay` render into the offscreen scene target
/// before the blur/glass composite; `ForegroundContent`/`ForegroundOverlay` render directly onto the
/// swapchain afterward, since glass foreground content must sample the backdrop it sits in front of.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ScenePhase {
    BackdropContent,
    BackdropOverlay,
    ForegroundContent,
    ForegroundOverlay,
}

impl ScenePhase {
    /// 🌐️ Ordered exactly as [`ui_render::RenderPacket::batches`] is built by `Scene::finish::batch`
    /// (`for overlay in [false, true] { for want_foreground in [false, true] { .. } }`), so a backend
    /// can bucket batches by phase in one pass and replay each bucket in this order.
    #[cfg(test)]
    pub(crate) const ORDER: [ScenePhase; 4] = [ScenePhase::BackdropContent, ScenePhase::ForegroundContent, ScenePhase::BackdropOverlay, ScenePhase::ForegroundOverlay];

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    #[cfg(test)]
    pub(crate) fn renders_offscreen(self) -> bool {
        matches!(self, ScenePhase::BackdropContent | ScenePhase::BackdropOverlay)
    }
}

/// 🔀️ Pure classification — no ordering/batching/clipping decision, only reading the two bits
/// `Scene::finish` already decided (`foreground_of`, `overlay`) off the batch's own `layer_state`.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
pub(crate) fn classify_batch_phase(layer_state: &LayerState) -> ScenePhase {
    match (layer_state.foreground_of.is_some(), layer_state.overlay) {
        (false, false) => ScenePhase::BackdropContent,
        (true, false) => ScenePhase::ForegroundContent,
        (false, true) => ScenePhase::BackdropOverlay,
        (true, true) => ScenePhase::ForegroundOverlay,
    }
}

//#endregion 🎬️ScenePhase

//#endregion 🔖️SurfaceState

//#region Tests

#[cfg(test)]
#[path = "../../../../🧪️tests/🔬️webgpu-packages-rust-surface-state-unit/🦀️.rs"]
mod tests;

//#endregion Tests
