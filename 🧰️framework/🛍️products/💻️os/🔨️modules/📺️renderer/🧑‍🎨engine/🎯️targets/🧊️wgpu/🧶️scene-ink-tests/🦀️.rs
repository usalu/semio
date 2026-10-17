//! 🧶️ Companion tests of the scene crates: the Canvas2d, raster frame cost, event feed and graph timeline laws.
//!
//! 🧭️ These laws name the frame action fixtures (`collect_fixture_actions`), the scene render wire and the retained UI engine next to the scene state, so they sit above all of them; each law file imports its names from the owning crates.

#![cfg(test)]

#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-canvas2d/🦀️.rs"]
mod canvas2d_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-event-feed/🦀️.rs"]
mod event_feed_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs"]
mod graph_timeline_tests;
#[path = "../../../🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-raster-frame-cost/🦀️.rs"]
mod raster_frame_cost_tests;
