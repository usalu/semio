//! 🪀️ `shell-view-panels` staging of `window-actions-and-commands`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🎬️WindowActionsAndSearchPanes
impl<'ctx> PanelsViewCtx<'ctx> {
    pub(crate) fn chrome_accessibility_checked(&self, id: &str, kind: &HitKind) -> Option<bool> {
        self.input.widget_maps
            .toggle_metas
            .get(id)
            .map(|(pressed, _)| *pressed)
            .or_else(|| (kind == &HitKind::Toggle).then(|| self.dock.panel_tab_anchor(id).is_some_and(|anchor| self.dock.anchor_open(anchor) && self.dock.anchor_state(anchor).path.first().is_some_and(|path| path == id))))
    }

    pub(crate) fn chrome_accessibility_pressed(&self, id: &str, kind: &HitKind) -> Option<bool> {
        if !matches!(kind, HitKind::PanelTab | HitKind::Toggle) {
            return None;
        }
        let anchor = self.dock.panel_tab_anchor(id);
        (kind == &HitKind::PanelTab || anchor.is_some()).then(|| anchor.is_some_and(|anchor| self.dock.anchor_state(anchor).path.iter().any(|path| path == id)))
    }

    pub(crate) fn chrome_accessibility_selected(&self, id: &str, _kind: &HitKind) -> Option<bool> {
        if let Some((palette, index)) = ShellPaletteKind::row_index(id) {
            return Some(match palette {
                ShellPaletteKind::Search => self.commands.search_selected == index,
                ShellPaletteKind::Find => self.commands.find_selected == index,
            });
        }
        None
    }

    /// 🖌️ One paint opportunity of one window's Actions OR Search pane body: the panel backdrop once,
    /// then its retained document through the same stepped paint the Measures overlay takes, then its
    /// hit registry above the window body's. `true` once the body is off the walk's critical path —
    /// painted, absent, folded, or faulted after its opportunity ceiling.
    ///
    /// ⚖️ Each body reads its own fold, matching the two independent React pane controls.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub(crate) fn paint_window_pane_body_step(
        &mut self,
        search: bool,
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
        let folded = if search { self.windows.window_search_folded(window_id) } else { self.windows.window_actions_folded(window_id) };
        if folded || !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let documents = if search { &mut self.docs.window_search_documents } else { &mut self.docs.window_actions_documents };
        let Some(document) = documents.remove(window_id) else { return true };
        let surface = if search { window_search_surface_id(window_id) } else { window_actions_surface_id(window_id) };
        let height = crate::interpreter::retained_content_height(&surface).unwrap_or(theme.control_height * 2.0);
        let rect = if search { window_search_rect(theme, window_rect, height) } else { window_actions_rect(theme, window_rect, height) };
        if !cursor.flag {
            draw.push_solid([rect.x, rect.y, rect.w, rect.h], theme.panel);
            cursor.flag = true;
        }
        let controller = self.docs.document_controller_id(&self.plugins);
        let complete = {
            let scroll_offsets = &mut self.input.scroll_offsets;
            let collapsed_sections = &mut self.input.collapsed_sections;
            let open_selects = &mut self.input.open_selects;
            let widget_maps = &mut self.input.widget_maps_staging;
            let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: surface.as_str() };
            let viewport_height_for_widgets = draw.screen_height();
            let mut ctx = framework_widget_context(draw, overlay.as_deref_mut(), atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
            ctx.pick_clip = Some(rect);
            render_ui_document_step(&mut cursor.document, &document, rect, &mut ctx, surface.as_str(), controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
        };
        if search {
            self.docs.window_search_documents.insert(window_id.to_string(), document);
        } else {
            self.docs.window_actions_documents.insert(window_id.to_string(), document);
        }
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

//#endregion 🎬️WindowActionsAndSearchPanes
//#endregion ActionPanelAndUtilities

