//! 🫀️ The runtime-owned settle pump, driven by the language-agnostic oracle
//! `🧫️fixtures/🫀️settle-pump/🔣️.json` that `🧪️tests/🫀️settle-pump/🟦️.ts` re-derives independently.
//!
//! Everything the transcript touches is production code: the shell's own
//! [`shell::settle_pump_owes`] predicate, its own [`shell::settle_watch_verdict`] watchdog over the
//! SAME `World3dScene.statusJson` parser the status pill reads ([`shell::world3d_compute_status`]),
//! and the real [`FrameDeferredCursor`] the runtime hands the shell one work item at a time.
//!
//! The three defects this pins, all measured on 6118
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END):
//!
//! * `settle_boot` converged the whole chain before the first paint — `boot_shell leave 7 226 ms`,
//!   chrome at 9 212 ms, against 1 880 ms / 4 101 ms with no example
//!   (`📓️wgpu-progress-visibility-2026-09-14.md` §8.1);
//! * the preview wedged at `meshingFaces 36/56 ratio=0.64912283` for 200 s and went deaf to
//!   `setActiveExample`, because the non-terminal run standing on it refuses every restart (§8.2);
//! * narrowing `refresh_ui` on `UiDirtyScope` froze 14 of 16 examples mid-solve, because the render
//!   it withdrew was also the guest crossing funding the solve
//!   (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4).

use super::*;
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/🫀️settle-pump/🔣️.json");
const SHELL_SOURCE: &str = include_str!("../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
const RENDERER_SOURCE: &str = include_str!("../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("settle pump fixture parses")
}

fn scope_named(name: &str) -> UiDirtyScope {
    match name {
        "none" => UiDirtyScope::None,
        "full" => UiDirtyScope::Full,
        "partial" => UiDirtyScope::Partial { window_bodies: vec!["procedural.window.preview".to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
        other => panic!("the fixture names scope {other}, which the kernel never declares"),
    }
}

#[test]
fn the_frame_owes_a_settle_step_exactly_while_a_chain_is_live() {
    let fixture = fixture();
    let rows = fixture["owesRows"].as_array().expect("the fixture declares its owes rows");
    assert!(rows.len() >= 7, "the oracle carries every term of the predicate");
    for row in rows {
        let id = row["id"].as_str().expect("every row is named");
        let answer = shell::settle_pump_owes(
            row["settling"].as_bool().expect("settling"),
            row["session"].as_bool().expect("session"),
            row["armedWork"].as_bool().expect("armedWork"),
            &scope_named(row["scope"].as_str().expect("scope")),
            row["armed"].as_bool().expect("armed"),
            row["computing"].as_bool().expect("computing"),
        );
        assert_eq!(answer, row["expect"].as_bool().expect("expect"), "{id}");
    }
}

#[test]
fn a_producer_is_funded_while_it_advances_and_driven_to_a_terminal_state_when_its_witness_freezes() {
    let fixture = fixture();
    let stall_steps = fixture["stallSteps"].as_u64().expect("the fixture declares the stall budget");
    let terminal_drives = fixture["terminalDrives"].as_u64().expect("the fixture declares the terminal-drive budget");
    assert_eq!(stall_steps, u64::from(shell::SHELL_SETTLE_STALL_STEPS), "the oracle and the shell price the stall identically");
    assert_eq!(terminal_drives, u64::from(shell::SHELL_SETTLE_TERMINAL_DRIVES), "the oracle and the shell price the terminal drives identically");
    for row in fixture["watchRows"].as_array().expect("the fixture declares its watch rows") {
        let id = row["id"].as_str().expect("every row is named");
        let mut watch = shell::ShellSettleWatch::default();
        let mut funds = 0u64;
        let mut terminals = 0u64;
        let mut stands = 0u64;
        let mut first_terminal: Option<u64> = None;
        let mut step = 0u64;
        for entry in row["statuses"].as_array().expect("every watch row publishes statuses") {
            let json = entry["json"].as_str().expect("every status entry carries one published statusJson");
            let status = shell::world3d_compute_status(Some(json));
            assert!(status.computing, "{id}: every watched status says its producer is working");
            for _ in 0..entry["repeat"].as_u64().unwrap_or(1) {
                step += 1;
                match shell::settle_watch_verdict(&mut watch, &status) {
                    shell::ShellSettleVerdict::Fund => funds += 1,
                    shell::ShellSettleVerdict::Stand => stands += 1,
                    shell::ShellSettleVerdict::Terminal => {
                        terminals += 1;
                        first_terminal.get_or_insert(step);
                    }
                }
            }
        }
        let expect = &row["expect"];
        assert_eq!(funds, expect["funds"].as_u64().expect("funds"), "{id}: crossings spent");
        assert_eq!(terminals, expect["terminals"].as_u64().expect("terminals"), "{id}: terminal drives");
        assert_eq!(stands, expect["stands"].as_u64().expect("stands"), "{id}: steps the pump stood down for");
        assert_eq!(first_terminal, expect["firstTerminalAtStep"].as_u64(), "{id}: the step the watchdog first drove the run");
    }
}

#[test]
fn a_frame_drains_its_input_actions_before_it_takes_a_settle_step() {
    let fixture = fixture();
    for row in fixture["frameRows"].as_array().expect("the fixture declares its frame rows") {
        let id = row["id"].as_str().expect("every row is named");
        let mut actions = FrameActionOwners::default();
        for index in 0..row["actions"].as_u64().expect("actions") {
            let descriptor = ActionDescriptor { controller_id: "s.procedural.generation3d@1/*#editor".to_string(), action: format!("setNodeInput{index}"), args: None };
            assert!(actions.try_push(descriptor).is_ok(), "{id}: the ledger admits the frame's action");
        }
        let mut cursor = FrameDeferredCursor::new(
            actions,
            row["pumpSync"].as_bool().expect("pumpSync"),
            row["flushTutorial"].as_bool().expect("flushTutorial"),
            row["shellMaintenance"].as_bool().expect("shellMaintenance"),
            row["settle"].as_bool().expect("settle"),
            1,
            semio_framework_job::root_cancel_token(),
        );
        let mut order: Vec<&'static str> = Vec::new();
        while let Some(work) = cursor.take_next() {
            order.push(match work {
                FrameDeferredWork::ShellMaintenance => "shellMaintenance",
                FrameDeferredWork::PumpSync => "pumpSync",
                FrameDeferredWork::Action(_) => "action",
                FrameDeferredWork::FlushTutorial => "flushTutorial",
                FrameDeferredWork::Settle => "settle",
            });
        }
        let expected: Vec<&str> = row["expect"]["order"].as_array().expect("order").iter().map(|entry| entry.as_str().expect("order entry")).collect();
        assert_eq!(order, expected, "{id}: the deferred order the runtime drains");
        assert_eq!(cursor.terminal_is_empty(), row["expect"]["terminalIsEmpty"].as_bool().expect("terminalIsEmpty"), "{id}: a drained owner is terminal-empty");
    }
}

#[test]
fn the_boot_arms_the_settle_lane_instead_of_converging_and_the_frame_loop_drives_it() {
    assert!(SHELL_SOURCE.contains("self.owe_settle();\n        Ok(())\n    }"), "settle_boot arms the settle lane and returns instead of driving flush_deferred_actions to a fixed point");
    let settle_boot = SHELL_SOURCE.split("async fn settle_boot").nth(1).expect("the shell declares settle_boot");
    let settle_boot = settle_boot.split("\n    async fn ").next().expect("settle_boot has a body");
    assert!(!settle_boot.contains("self.flush_deferred_actions("), "settle_boot no longer converges the chain before the first paint");
    assert!(RENDERER_SOURCE.contains("cursor.settle = self.shell.settle_pump_pending();"), "the frame-finish boundary re-reads the settle predicate every frame");
    assert!(RENDERER_SOURCE.contains("interaction.shell.settle_pump_step().await"), "the frame's deferred owner is the ONE caller of the settle step");
}

/// 🎥️ Scene paint precedes typed snapshot sealing in a frame transaction, so the snapshot
/// boundary must advance camera residency itself. The fit cursor then participates in the ordinary
/// World3d pending predicate and funds later corners without a global frame-loop poll.
#[test]
fn a_sealed_world_snapshot_hands_camera_fit_to_the_retained_wake_lane() {
    let snapshot = RENDERER_SOURCE.split("AppFrameTransactionPhase::World3dSnapshot =>").nth(1).expect("the frame transaction declares its World snapshot phase");
    let snapshot = &snapshot[..snapshot.find("AppFrameTransactionPhase::World3dAuthority =>").expect("the snapshot phase terminates")];
    assert!(
        snapshot.contains("World3dSnapshotApplyStep::Idle | World3dSnapshotApplyStep::Complete => {\n                        step_world3d_camera_fit_after_snapshot(state);"),
        "an idle or newly sealed snapshot advances one bounded camera-fit step before leaving its surface"
    );
    assert!(RENDERER_SOURCE.contains("fn step_world3d_camera_fit_after_snapshot(state: &mut infinite_world::world::World3dState)"), "the transaction has one explicit post-snapshot camera handoff");
}

#[test]
fn the_refresh_honours_the_dirty_scope_now_that_the_pump_funds_the_guest() {
    let refresh = SHELL_SOURCE.split("pub async fn refresh_ui").nth(1).expect("the shell declares refresh_ui");
    assert!(refresh.contains("if !scope.wants_window_body(&kind.body_key) {"), "a window body is rendered only when the scope names it");
    assert!(refresh.contains("if !scope.wants_panel_body(&body_key) {"), "a panel body is rendered only when the scope names it");
    assert!(refresh.contains("if scope.wants_section(UiDirtySection::Measures) {"), "the measures section is fetched only when the scope names it");
    assert!(refresh.contains("if scope.wants_section(UiDirtySection::Engagements) {"), "the engagements section is fetched only when the scope names it");
}

/// 🩸️ The gesture half of the same authority, measured on 6118 before it was law
/// (`📓️wgpu-generate-add-port-fit-2026-09-14.md`): a stray port drag rewired the graph, the pointer
/// move that followed held the runtime's `dispatch-event` checkout for **8 515 ms** across 6 refresh
/// passes of `settle_ui_chain`, and in that window the host dispatched **no input at all** — the `F`
/// the user pressed 1.1 s in reached `fit_node_graph_camera` 7 632 ms after its key-down, and the
/// probe that had already given up called the verb dead.
#[test]
fn an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch() {
    let fixture = fixture();
    let rows = fixture["gestureRows"].as_array().expect("the fixture declares its gesture rows");
    assert!(rows.len() >= 4, "the oracle carries every input entry point that used to converge");
    for row in rows {
        let id = row["id"].as_str().expect("every row is named");
        let source = match row["source"].as_str().expect("source") {
            "shell" => SHELL_SOURCE,
            "renderer" => RENDERER_SOURCE,
            other => panic!("{id}: the fixture names source {other}, which this suite does not read"),
        };
        let signature = row["owns"].as_str().expect("owns");
        let after = source.split(signature).nth(1).unwrap_or_else(|| panic!("{id}: {signature} is declared"));
        let body = after.split("\n    }\n").next().unwrap_or_else(|| panic!("{id}: {signature} has a body"));
        let forbidden = row["forbids"].as_str().expect("forbids");
        assert!(!body.contains(forbidden), "{id}: {signature} must not reach {forbidden} — a gesture that converges its own chain starves every later input");
        if let Some(declares) = row["declares"].as_str() {
            assert!(body.contains(declares), "{id}: {signature} declares the chain with {declares}");
        }
    }
    assert!(SHELL_SOURCE.contains("pub fn owe_settle(&mut self)"), "the shell publishes the one way a producer declares a live chain");
}

#[test]
fn a_gesture_runs_its_user_activation_bound_work_before_it_returns() {
    let fixture = fixture();
    let rows = fixture["gestureBoundRows"].as_array().expect("the fixture declares its gesture-bound rows");
    assert!(rows.len() >= 3, "every door a user-activation-bound request can arrive through is covered");
    for row in rows {
        let id = row["id"].as_str().expect("every row is named");
        let source = match row["source"].as_str().expect("source") {
            "shell" => SHELL_SOURCE,
            "renderer" => RENDERER_SOURCE,
            other => panic!("{id}: the fixture names source {other}, which this suite does not read"),
        };
        let signature = row["owns"].as_str().expect("owns");
        let after = source.split(signature).nth(1).unwrap_or_else(|| panic!("{id}: {signature} is declared"));
        let body = after.split("\n    }\n").next().unwrap_or_else(|| panic!("{id}: {signature} has a body"));
        let runs = row["runs"].as_str().expect("runs");
        assert!(body.contains(runs), "{id}: {signature} must run {runs} — a picker handed to the settle pump has lost the gesture that was allowed to open it");
    }
    let wrapper = SHELL_SOURCE.split("pub async fn handle_pointer_button(").nth(1).expect("the default-pointer entry point is declared").split("\n    }\n").next().expect("the default-pointer entry point has a body");
    assert!(wrapper.contains("self.handle_pointer_button_for(") && wrapper.contains(".await"), "the default-pointer entry point must await the identity-aware gesture in the same user-activation turn");
    assert!(SHELL_SOURCE.contains("async fn drain_gesture_bound_work(&mut self) {"), "the shell owns one narrow drain for user-activation-bound work");
    let drain = SHELL_SOURCE.split("async fn drain_gesture_bound_work(&mut self) {").nth(1).expect("the drain has a body");
    let drain = drain.split("\n    }\n").next().expect("the drain has a body");
    assert!(drain.contains("pending_file_opens"), "the drain takes the parked file opens");
    assert!(!drain.contains("deferred_actions"), "and NOTHING else — every other kind of armed work belongs to the pump");
}

#[test]
fn every_declared_law_is_answered_here() {
    let fixture = fixture();
    let laws = fixture["laws"].as_array().expect("the fixture declares its laws");
    assert!(laws.len() >= 12, "the oracle declares every law this suite answers");
    for consumer in fixture["consumers"].as_array().expect("the fixture names its consumers") {
        let path = consumer.as_str().expect("every consumer is a path");
        assert!(path.ends_with("🦀️.rs") || path.ends_with("🟦️.ts"), "the oracle is answered by one implementation per language");
    }
}
