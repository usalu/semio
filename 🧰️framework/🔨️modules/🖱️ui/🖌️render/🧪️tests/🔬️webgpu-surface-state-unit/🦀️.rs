
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
