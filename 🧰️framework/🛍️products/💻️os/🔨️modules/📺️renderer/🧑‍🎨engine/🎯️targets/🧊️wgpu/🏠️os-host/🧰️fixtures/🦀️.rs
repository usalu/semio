//! 🧰️ Test fixtures of the OS host: the component close drivers every law that retires a retained scene, world or map ends with.
//!
//! 🧭️ Behind `testkit`: the monolith test unit held these functions in the os-host standalone law file; laws in the test bins name them through this module.

use super::*;
use semio_framework_os_renderer_wgpu_app_runtime::{AppMailbox, AppRuntime};
use semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry;
use semio_framework_os_renderer_wgpu_frame_actions::{AppInteractionState, FrameActionOwners};
use semio_framework_os_renderer_wgpu_runtime_mailbox::RuntimeMailbox;
use semio_framework_os_renderer_wgpu_scene_state::{publish_component_surface_close_terminal, take_component_surface_close_request};
use semio_framework_os_renderer_wgpu_shell_input::PointerCapture;
use semio_framework_os_renderer_wgpu_shell_state::ShellState;
use semio_framework_ui_locale::{Locale, Terminology};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_text::FontAtlas;
use semio_framework_ui_wgpu_input::{InputState, PointerModifiers};
use semio_framework_ui_theme::Theme;

pub fn finish_retired_cpu_only_map_fixture_close() {
    finish_cpu_only_fixture_component_close(|owner| assert_eq!(owner.kind, semio_framework_ui_declaration::ui::SurfaceKind::TiledMap));
}

pub fn finish_cpu_only_fixture_component_close(close: impl FnOnce(&semio_framework_os_renderer_wgpu_contracts::ScenePointerTarget)) {
    let Some(request) = take_component_surface_close_request() else { return };
    assert!(matches!(
        request.owner.kind,
        semio_framework_ui_declaration::ui::SurfaceKind::TiledMap | semio_framework_ui_declaration::ui::SurfaceKind::NodeGraph | semio_framework_ui_declaration::ui::SurfaceKind::Board2d | semio_framework_ui_declaration::ui::SurfaceKind::TextEditor
    ));
    if let Some(token) = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token(&request.owner.host_id) {
        assert_eq!(request.engine_token, Some(token), "the scene close owns the exact engine token bound during paint");
    }
    close(&request.owner);
    assert!(semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token(&request.owner.host_id).is_none());
    if let Some(token) = request.engine_token {
        assert_eq!(semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::engine_surface_token_at(usize::from(token.slot)), Ok(None));
    }
    assert!(publish_component_surface_close_terminal(request.token));
}

pub fn runtime_with_shell(shell: semio_framework_os_renderer_wgpu_shell_state::ShellState) -> AppMailbox {
    RuntimeMailbox::new(semio_framework_os_renderer_wgpu_app_runtime::AppRuntime {
        atlas: semio_framework_ui_text::FontAtlas::builtin(),
        icons: semio_framework_ui_draw_contract::IconAtlas::default(),
        icon_rebuild: None,
        icon_raster_scale: 1.0,
        interaction: Some(semio_framework_os_renderer_wgpu_frame_actions::AppInteractionState {
            shell,
            input: semio_framework_ui_wgpu_input::InputState::default(),
            theme: semio_framework_ui_theme::Theme::default(),
            theme_dark: false,
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            pointer_capture: semio_framework_os_renderer_wgpu_shell_input::PointerCapture::default(),
            modifiers: semio_framework_ui_wgpu_input::PointerModifiers::default(),
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            last_sync_pump_ms: 0.0,
        }),
        checkout: Default::default(),
        draw: semio_framework_ui_draw_contract::DrawList::default(),
        overlay: semio_framework_ui_draw_contract::DrawList::default(),
        pending_frame_deferred: None,
        frame_actions: semio_framework_os_renderer_wgpu_frame_actions::FrameActionOwners::default(),
        pending_frame_maintenance_refusal: None,
        plugin_modules_root: Default::default(),
        native_hot_swap_scan: None,
        native_hot_swap_program: None,
        native_hot_swap_cursor: 0,
        native_reload_pending: false,
    })
}

/// 🌐️ Drives the actual asset and World retirement bridge for a retained Shell fixture.
pub fn finish_world_fixture_component_close(shell: semio_framework_os_renderer_wgpu_shell_state::ShellState) -> semio_framework_os_renderer_wgpu_shell_state::ShellState {
    let Some(request) = take_component_surface_close_request() else { return shell };
    assert_eq!(request.owner.kind, semio_framework_ui_declaration::ui::SurfaceKind::World3d);
    assert!(request.engine_token.is_none(), "this World fixture has no paired GPU engine token");
    let runtime = runtime_with_shell(shell);
    let mut close = ComponentSurfaceCloseOwner::new(request);
    let mut original = semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn::FixtureTurnV1::new(semio_framework_os_renderer_wgpu_frame_job::frame_authority::FRAME_BOOTSTRAP_ROOT_GRANT);
    for _ in 0..262_144 {
        if close.terminal_is_empty() {
            break;
        }
        original.turn(|cx| close.close_asset_world_step(&runtime, cx)).expect("same original component cleanup");
    }
    assert!(close.terminal_is_empty(), "the production World close lane returns its exact owner");
    assert!(publish_component_surface_close_terminal(close.request.token));
    let mut owner = runtime.try_lock().expect("the fixture returns its Shell owner");
    std::mem::replace(
        &mut owner.interaction.as_mut().expect("the interaction is returned").shell,
        semio_framework_os_renderer_wgpu_shell_state::ShellState::new(Vec::new(), "closed-fixture".to_string(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native),
    )
}
