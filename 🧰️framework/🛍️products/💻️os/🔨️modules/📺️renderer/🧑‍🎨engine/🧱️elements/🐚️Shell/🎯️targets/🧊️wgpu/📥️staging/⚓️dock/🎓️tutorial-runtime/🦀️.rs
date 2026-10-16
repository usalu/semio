//! ⚓️ `shell-dock` staging of `tutorial-runtime`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🎬️Tutorial
//#region 👻️GestureOverlay
/// 👻️ Resolves an `IntroductionPoint` to a viewport pixel through chrome geometry or the attached
/// surface owner's live camera and semantic geometry registry.
pub fn tutorial_resolve_gesture_point(chrome: &ChromeState, surfaces: &SurfaceState, dock: &ShellDockState, point: &semio_framework::onboarding::IntroductionPoint) -> Option<(f32, f32)> {
    use semio_framework::onboarding::IntroductionPoint as P;
    match point {
        P::Screen { x, y } => Some((*x as f32, *y as f32)),
        P::ScreenNormalized { x, y } => Some((*x as f32 * chrome.screen_w, *y as f32 * chrome.screen_h)),
        P::Element { id, offset } => {
            let rect = chrome.chrome_build.resolve_element_rect(id)?;
            let [ox, oy] = offset.unwrap_or([0.5, 0.5]);
            Some((rect.x + rect.w * ox as f32, rect.y + rect.h * oy as f32))
        }
        P::Window { id, x, y } => {
            let rect = dock.window_content_rects.get(id)?;
            Some((rect.x + *x as f32, rect.y + *y as f32))
        }
        P::WindowNormalized { id, x, y } => {
            let rect = dock.window_content_rects.get(id)?;
            Some((rect.x + rect.w * (*x as f32), rect.y + rect.h * (*y as f32)))
        }
        P::Scene { id, position } => {
            let rect = dock.window_content_rects.get(id).copied().or_else(|| surfaces.world3d_state_for_window(id).map(|world| world.bounds))?;
            let world = surfaces.world3d_state_for_window(id)?;
            let local = infinite_world::world::world3d_tutorial_scene_point(&world.session, &world.camera, *position)?;
            Some((rect.x + local[0], rect.y + local[1]))
        }
        P::Canvas { id, x, y } => {
            let rect = dock.window_content_rects.get(id).copied().or_else(|| surfaces.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = surfaces
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_scene_point(&world.session, &world.camera, [*x, *y, 0.0]).map(|point| (point[0], point[1])))
                .or_else(|| surfaces.engine_host_id_for_window(id).and_then(|host_id| semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Entity { id, domain, entity, .. } => {
            let rect = dock.window_content_rects.get(id).copied().or_else(|| surfaces.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = surfaces
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_entity_geometry(&world.session, &world.camera, &world.scene, domain, entity).map(|geometry| (geometry.point[0], geometry.point[1])))
                .or_else(|| surfaces.engine_host_id_for_window(id).and_then(|host_id| semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Curve { id, domain, entity, t } => {
            let rect = dock.window_content_rects.get(id).copied().or_else(|| surfaces.world3d_state_for_window(id).map(|world| world.bounds))?;
            let local = surfaces
                .world3d_state_for_window(id)
                .and_then(|world| infinite_world::world::world3d_tutorial_entity_geometry(&world.session, &world.camera, &world.scene, domain, entity))
                .and_then(|geometry| geometry.polyline.and_then(|polyline| semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::tutorial_polyline_point(&polyline, *t)).map(|point| (point[0], point[1])))
                .or_else(|| surfaces.engine_host_id_for_window(id).and_then(|host_id| semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::resolve_tutorial_surface_point(host_id, point)))?;
            Some((rect.x + local.0, rect.y + local.1))
        }
        P::Domain { id, .. } => {
            let rect = *dock.window_content_rects.get(id)?;
            let local = semio_framework_os_renderer_wgpu_engine_registry::tutorial_surface_geometry::resolve_tutorial_surface_point(surfaces.engine_host_id_for_window(id)?, point)?;
            Some((rect.x + local.0, rect.y + local.1))
        }
    }
}

//#endregion 👻️GestureOverlay
//#endregion 🎬️Tutorial
//#endregion ShellChrome


