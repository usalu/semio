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
    pub(crate) const ORDER: [ScenePhase; 4] = [ScenePhase::BackdropContent, ScenePhase::ForegroundContent, ScenePhase::BackdropOverlay, ScenePhase::ForegroundOverlay];

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
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
mod tests {
    use super::*;

    #[test]
    fn fresh_surface_state_is_parked() {
        let state = SurfaceState::default();
        assert!(state.is_parked());
    }

    #[test]
    fn resize_to_nonzero_unparks() {
        let mut state = SurfaceState::default();
        state.resize(PhysicalSize::new(800, 600), 2.0);
        assert!(!state.is_parked());
        assert_eq!(state.size, PhysicalSize::new(800, 600));
        assert_eq!(state.dpr, 2.0);
    }

    #[test]
    fn resize_to_zero_and_back_restores_unparked_state() {
        let mut state = SurfaceState::default();
        state.resize(PhysicalSize::new(800, 600), 1.0);
        state.resize(PhysicalSize::ZERO, 1.0);
        assert!(state.is_parked());
        state.resize(PhysicalSize::new(400, 300), 1.0);
        assert!(!state.is_parked());
    }

    #[test]
    fn one_zero_dimension_still_parks() {
        let mut state = SurfaceState::default();
        state.resize(PhysicalSize::new(800, 0), 1.0);
        assert!(state.is_parked());
    }

    #[test]
    fn device_health_round_trips_through_status() {
        assert_eq!(DeviceHealth::Healthy.as_status(), DeviceStatus::Healthy);
        assert_eq!(DeviceHealth::Suboptimal.as_status(), DeviceStatus::Suboptimal);
        assert_eq!(DeviceHealth::Lost(LossReason::Device).as_status(), DeviceStatus::Lost(LossReason::Device));
    }

    #[test]
    fn only_lost_reports_is_lost() {
        assert!(!DeviceHealth::Healthy.is_lost());
        assert!(!DeviceHealth::Suboptimal.is_lost());
        assert!(DeviceHealth::Lost(LossReason::Surface).is_lost());
    }

    fn layer_state(foreground_of: Option<usize>, overlay: bool) -> LayerState {
        LayerState { scissor: None, clip: None, foreground_of, overlay }
    }

    #[test]
    fn classifies_all_four_phases() {
        assert_eq!(classify_batch_phase(&layer_state(None, false)), ScenePhase::BackdropContent);
        assert_eq!(classify_batch_phase(&layer_state(Some(0), false)), ScenePhase::ForegroundContent);
        assert_eq!(classify_batch_phase(&layer_state(None, true)), ScenePhase::BackdropOverlay);
        assert_eq!(classify_batch_phase(&layer_state(Some(0), true)), ScenePhase::ForegroundOverlay);
    }

    #[test]
    fn backdrop_phases_render_offscreen_foreground_does_not() {
        assert!(ScenePhase::BackdropContent.renders_offscreen());
        assert!(ScenePhase::BackdropOverlay.renders_offscreen());
        assert!(!ScenePhase::ForegroundContent.renders_offscreen());
        assert!(!ScenePhase::ForegroundOverlay.renders_offscreen());
    }

    #[test]
    fn phase_order_matches_scene_finish_batch_emission_order() {
        assert_eq!(ScenePhase::ORDER, [ScenePhase::BackdropContent, ScenePhase::ForegroundContent, ScenePhase::BackdropOverlay, ScenePhase::ForegroundOverlay]);
    }
}

//#endregion Tests
