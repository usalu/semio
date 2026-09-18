//! 💓️ Packet W5c's standing law: the shared chrome I/O lane is PRESSURE-DRIVEN. A chrome walk that
//! changed nothing asks it for nothing.
//!
//! 🩸️ The defect it pins: `render_chrome_step` armed `presence_requested`, `layout_requested` and
//! `persist_requested` unconditionally, at `ShellChromeFramePhase::Presence`, `PersistLayout` and
//! `PersistPreferences`, on EVERY walk. Each armed flag is one `FrameDeferredWork::ShellMaintenance`
//! step, and every one of those steps is a full `check_out_interaction("frame-deferred")` plus an
//! async hand-back — while `FrameTransaction::step` refuses to run at all with the interaction state
//! checked out (`AppFrameTransactionStep::Superseded`). A settled shell therefore re-armed five
//! no-op steps per frame, the deferred lane held the state, and the frame build that armed them was
//! superseded by them. Measured on the live puzzle3d wgpu playground as 186 frame builds and ONE
//! presented frame in 45 s (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w5c-world-pass-mesh-draws.md`).

use super::*;

fn chrome_lane_source() -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs").canonicalize().expect("wgpu shell source");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn lane_body(source: &str, name: &str) -> String {
    let after = source.split(&format!("fn {name}(&mut self) {{")).nth(1).unwrap_or_else(|| panic!("the {name} arm"));
    after[..after.find("\n    }\n").expect("the arm ends")].to_string()
}

/// ⚖️ Every maintenance arm consults its own pressure before it arms the lane.
#[test]
fn no_chrome_maintenance_lane_arms_itself_without_pressure() {
    let source = chrome_lane_source();
    let persist = lane_body(&source, "request_chrome_preferences_persist");
    assert!(persist.contains("last_synced_preferences.as_ref() != Some(&UiPrefsSnapshot::capture(self))"), "preferences persist only when a field actually changed");
    assert!(persist.contains("if "), "and the arm is guarded, not unconditional");

    let layout = lane_body(&source, "request_panel_layout_persist");
    assert!(layout.contains("last_persisted_dock_ui.as_ref() != Some(&self.dock_ui_snapshot())"), "the dock UI page is armed only when the dock moved");
    assert!(layout.contains("last_persisted_dock_skeleton.as_ref() != Some(&self.dock_override)"), "and so is the skeleton half");

    let presence = lane_body(&source, "request_presence_preview");
    assert!(presence.contains("#[cfg(not(target_arch = \"wasm32\"))]"), "the browser has no sync backbone to heartbeat to");
    assert!(presence.contains("self.sync_channel.is_some()"), "and a shell with no channel has no peer to heartbeat to");
}

/// ⚖️ The three phases that used to arm unconditionally still run in the chrome walk — the fix is the
/// guard inside the arm, never a deleted phase.
#[test]
fn the_chrome_walk_still_visits_every_maintenance_phase() {
    let source = chrome_lane_source();
    for (phase, arm) in [
        ("ShellChromeFramePhase::Presence => {", "self.request_presence_preview();"),
        ("ShellChromeFramePhase::PersistLayout => {", "self.request_panel_layout_persist();"),
        ("ShellChromeFramePhase::PersistPreferences => {", "self.request_chrome_preferences_persist();"),
    ] {
        let after = source.split(phase).nth(1).unwrap_or_else(|| panic!("the {phase} arm"));
        assert!(after[..200].contains(arm), "{phase} still calls {arm}");
    }
}
