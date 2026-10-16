//! 🍱️ `shell-view-chrome` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 📏️WindowMeasures
impl<'ctx> ChromeViewCtx<'ctx> {
    /// 🖌️ One paint opportunity of a window's Measures overlay, over the window body it belongs to:
    /// the overlay backdrop once, then its retained document through the same stepped paint every body
    /// uses, then its hit registry above the body's. `true` once the overlay is off the walk's critical
    /// path — painted, absent, folded, or faulted after its opportunity ceiling.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub fn paint_window_measures_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        overlay: &mut Option<&mut DrawList>,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        window_id: &str,
        window_rect: Rect,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        if !self.docs.window_measures_documents.contains_key(window_id) {
            return true;
        }
        if self.windows.measures_rail_folded(window_id) {
            return true;
        }
        let Some(document) = self.docs.window_measures_documents.remove(window_id) else { return true };
        let surface = window_measures_surface_id(window_id);
        semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::set_ui_document_flow(&surface, ui_contract::UiFlow::for_anchor(ui_contract::Anchor::TopEnd));
        let rect = self.windows.window_measures_rect(window_id, window_rect, theme);
        if !cursor.flag && cursor.document.layout_is_accepted() && cursor.rect.is_some_and(|previous| previous != rect) {
            cursor.document.restart_viewport_after_host_reflow();
        }
        cursor.rect = Some(rect);
        if !cursor.flag && cursor.document.phase_name() == "paint" {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.docs.document_controller_id(&self.plugins);
        let complete = {
            let scroll_offsets = &mut self.input.scroll_offsets;
            let collapsed_sections = &mut self.input.collapsed_sections;
            let open_selects = &mut self.input.open_selects;
            let widget_maps = &mut self.input.widget_maps_staging;
            let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
        };
        self.docs.window_measures_documents.insert(window_id.to_string(), document);
        if complete {
            self.as_session().note_retained_body_painted(&surface);
        } else {
            if document_opportunity_remains(cursor, SHELL_WINDOW_PAINT_OPPORTUNITIES) {
                return false;
            }
            self.as_session().record_document_paint_fault(&surface);
        }
        push_chrome_group_border(draw, rect, theme);
        self.input.register_retained_body_hits(&surface, rect, input);
        true
    }
}

//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle

