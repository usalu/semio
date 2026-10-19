//! 🧰️ Test fixtures of the engine attach: the teardown every law that attaches an engine surface ends with.
//!
//! 🧭️ Behind `testkit`: the monolith test unit held these functions in the node graph and EngineCanvas standalone law files; laws in the test bins name them through this crate.

use super::engine_surface_attach::take_engine_surface_registrations;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::{begin_engine_surface_close_token, close_engine_surface_step, ENGINE_SURFACES};
use semio_framework_os_renderer_wgpu_engine_registry::fixtures::with_engine_close_context;
use semio_framework_os_renderer_wgpu_engine_registry::prepared_engine_canvas::STAGED_ENGINE_SCENES;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_wgpu_input::InputState;

/// 🧹️ Drains one attached surface through the registry's OWN retirement ladder. `FlowHostSnapshot`'s
/// ordered maps refuse to drop unretired, so a plain `remove` panics inside the registry mutex and
/// poisons it for every later lane — the ladder is the only correct teardown.
pub fn drop_engine_surface(surface_id: &str) {
    let _ = take_engine_surface_registrations();
    STAGED_ENGINE_SCENES.with(|cell| cell.borrow_mut().remove_surface(surface_id));
    let Some(token) = ENGINE_SURFACES.with(|cell| cell.borrow_mut().token(surface_id)) else {
        return;
    };
    assert_eq!(begin_engine_surface_close_token(token), Ok(true), "the attached surface begins closing");
    let mut input = InputState::<ActionDescriptor>::default();
    let mut turns = 0usize;
    while !with_engine_close_context(1, |context| close_engine_surface_step(token, context, &mut input)) {
        turns += 1;
        assert!(turns < 262_144, "the attached surface reaches a fixed terminal witness");
    }
}

/// 🧹️ Retires a test's directly attached host through the bounded engine close ladder.
pub fn retire_engine_surface_fixture(host_id: &str) {
    drop_engine_surface(host_id);
}
