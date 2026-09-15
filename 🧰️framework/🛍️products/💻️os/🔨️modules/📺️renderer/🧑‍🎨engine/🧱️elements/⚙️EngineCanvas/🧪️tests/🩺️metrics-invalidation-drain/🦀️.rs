//! 🩺️ The primary-metrics invalidation scan is DRAINED inside one `realize_step`, never advanced one
//! fixed slot per present step.
//!
//! ⚖️ `AppPresenter::admit_next_frame` builds no frame while a presentation is pending, so every
//! `Ok(false)` this arm answers is a blocked frame for the whole host. One slot per step spends
//! `ENGINE_SURFACE_CAPACITY` of them on a scan whose entire work is one `begin_close()` per slot.
//! Measured on 6118 as `[DEBUG] engine realize stalled arm=metrics-invalidation-scan steps=128
//! scan=Some(212)` on every example boot, with the host's own frame gate reading
//! `blocked=true phase=Some(Engine)` across it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §6.2, `📓️wgpu-regressions-sweep-2026-09-15.md`).

use super::*;

/// 🩺️ The scan an `AppSurfaceResizePhase::Apply` arms is bounded by the fixed slot table and by
/// nothing else — the ceiling every drain in this module is written against.
#[test]
fn the_primary_metrics_invalidation_scan_terminates_within_the_fixed_slot_table() {
    let mut presenter = EngineCanvasPresenter::default();
    assert!(presenter.invalidate_primary_metrics_step(), "an unarmed scan is already terminal");
    assert!(presenter.observe_primary_metrics_generation(7), "a newer metrics generation arms the scan");
    let mut steps = 0;
    while !presenter.invalidate_primary_metrics_step() {
        steps += 1;
        assert!(steps <= ENGINE_SURFACE_CAPACITY + 1, "the scan must terminate within the fixed slot table");
    }
    assert_eq!(steps, ENGINE_SURFACE_CAPACITY, "every fixed slot is visited exactly once");
    assert!(presenter.metrics_invalidation_scan.is_none(), "a drained scan holds no cursor");
    assert!(presenter.terminal_is_empty(), "a drained scan leaves the presenter terminal-empty");
}

/// 🩺️ …and `realize_step` drains the WHOLE of it before it answers, so the arm costs one blocked
/// frame rather than `ENGINE_SURFACE_CAPACITY` of them. A source scan, because the arm needs a live
/// `GpuContext` no unit test can mint.
#[test]
fn realize_step_drains_the_whole_metrics_invalidation_scan_in_one_answer() {
    const SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🦀️.rs");
    let start = SOURCE.find("if self.metrics_invalidation_scan.is_some() {").expect("the realize ladder still guards on the scan");
    let arm = &SOURCE[start..start + SOURCE[start..].find("return Ok(false);").expect("the arm answers not-yet")];
    assert!(arm.contains("for _ in 0..=ENGINE_SURFACE_CAPACITY {"), "the arm drains the scan against the fixed slot ceiling");
    assert!(arm.contains("if self.invalidate_primary_metrics_step() {"), "the drain stops on the scan's own terminal answer");
    assert!(!arm.contains("self.invalidate_primary_metrics_step();\n            self.note_realize_stall"), "no single-slot advance remains");
}
