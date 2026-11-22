//! 🚦️ Frame demand and frame traces, driven by the language-agnostic oracle
//! `🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json` that `🧪️tests/🚦️frame-demand-and-traces/🟦️.ts` re-derives independently.

use crate::frame_demand::{frame_build_admitted_line, FrameDemand};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🚦️frame-demand-and-traces/🔣️.json")).expect("frame-demand-and-traces fixture JSON")
}

fn text<'a>(row: &'a serde_json::Value, key: &str) -> &'a str {
    row[key].as_str().unwrap_or_else(|| panic!("fixture row lacks {key}"))
}

fn demand_of(set: &[&str]) -> FrameDemand {
    let mut demand = FrameDemand::default();
    for name in set {
        match *name {
            "invalidated" => demand.invalidated = true,
            "first-frame" => demand.first_frame = true,
            "input-queued" => demand.input_queued = true,
            "frame-ready" => demand.frame_ready = true,
            "live-build" => demand.live_build = true,
            "presentation-pending" => demand.presentation_pending = true,
            "surface-resize" => demand.surface_resize = true,
            "snapshot-retained" => demand.snapshot_retained = true,
            "component-close" => demand.component_close = true,
            "cursor-wake" => demand.cursor_wake = true,
            "runtime-pending" => demand.runtime_pending = true,
            other => panic!("unknown demand flag {other}"),
        }
    }
    demand
}

#[test]
fn every_fixture_row_demands_a_build_exactly_when_the_oracle_says_so() {
    for row in fixture()["demand"].as_array().expect("demand rows") {
        let set: Vec<&str> = row["set"].as_array().expect("set").iter().map(|name| name.as_str().expect("flag name")).collect();
        let demand = demand_of(&set);
        let id = text(row, "id");
        assert_eq!(demand.is_demanded(), row["demanded"].as_bool().expect("demanded"), "{id}");
        assert_eq!(demand.reasons(), text(row, "reasons"), "{id}");
        assert_eq!(frame_build_admitted_line(7, demand), text(row, "admittedLine"), "{id}");
    }
}

#[test]
fn an_idle_tick_demands_no_build_and_each_single_flag_alone_demands_one() {
    assert!(!FrameDemand::default().is_demanded());
    let names = ["invalidated", "first-frame", "input-queued", "frame-ready", "live-build", "presentation-pending", "surface-resize", "snapshot-retained", "component-close", "cursor-wake", "runtime-pending"];
    for name in names {
        let demand = demand_of(&[name]);
        assert!(demand.is_demanded(), "{name} alone must demand a frame build");
        assert_eq!(demand.reasons(), name);
    }
}

#[test]
fn the_browser_tick_gates_both_the_generation_advance_and_the_build_on_demand() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs");
    assert!(source.contains("if demand.is_demanded(){self.prepare_redraw(cx)?;}"));
    assert!(source.contains("if demand.is_demanded(){self.build_and_publish_snapshot_on_original_caller(cx,demand)?;}"));
    assert!(source.contains("log_debug(&frame_build_admitted_line(generation.0,demand))"));
}

#[test]
fn every_fixture_action_names_its_controller_and_action_in_one_trace_line() {
    for row in fixture()["actions"].as_array().expect("action rows") {
        assert_eq!(semio_renderer_wgpu_frame_actions::frame_input_action_line(text(row, "controller"), text(row, "action")), text(row, "line"), "{}", text(row, "id"));
    }
}

#[test]
fn the_input_action_transfer_emits_exactly_one_trace_line_per_materialized_action() {
    let source = include_str!("../../🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs");
    let transfer = &source[source.find("pub fn transfer_frame_input_action(").expect("transfer owner")..];
    assert_eq!(transfer.matches("frame_input_action_line(").count(), 1);
    let emit = transfer.find("log_debug(&frame_input_action_line(").expect("trace emission");
    assert!(emit > transfer.find("into_envelope()").expect("materialization"), "the line is emitted only after the action materialized");
    assert!(emit < transfer.find("stage_batch_action(action)").expect("publication"), "the line is emitted before publication");
}
