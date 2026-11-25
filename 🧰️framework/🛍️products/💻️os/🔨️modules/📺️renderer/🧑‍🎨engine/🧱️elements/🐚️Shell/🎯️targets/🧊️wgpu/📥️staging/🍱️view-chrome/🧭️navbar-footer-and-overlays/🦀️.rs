//! 🍱️ `shell-view-chrome` staging of `navbar-footer-and-overlays`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> ChromeViewCtx<'ctx> {
    /// 🔝️ The navbar is a glass band above the canvas (`Theme::band_glass`); its content is the glass's foreground, opened and closed around every opportunity of the walk.
    pub fn render_navbar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        if cursor.band_glass == 0 {
            cursor.band_glass = draw.push_glass([0.0, 0.0, width, theme.navbar_height], 0.0, theme.band_glass()).saturating_add(1);
        }
        draw.begin_glass_content(cursor.band_glass - 1);
        let complete = self.render_navbar_content_step(cursor, draw, atlas, icons, input, theme, width);
        draw.end_glass_content();
        complete
    }

    fn render_navbar_content_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        let mobile = self.chrome.mobile_panel_active();
        match cursor.phase {
            0 => {
                cursor.x = theme.padding_standard;
                cursor.right = width - theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, 0.0, width, theme.navbar_height).contains(input.pointer_x, input.pointer_y);
                let line = semio_renderer_wgpu_shell_session::shell_navbar_line_rect(width, theme);
                draw.push_solid([line.x, line.y, line.w, line.h], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.item = 0;
                cursor.rect = None;
                cursor.phase = 10;
            }
            10 => {
                if mobile {
                    let fullscreen_width = self.prefs.fullscreen_item_width(atlas, theme, &self.chrome);
                    let toggle = ChromeGroupItem { control_id: "ui.mobilePanel.toggle", icon_id: Some("panel-left"), label: None, active: self.dock.mobile_panel_visible, disabled: false, kind: HitKind::Toggle };
                    let toggle_width = retained_chrome_group_item_width(atlas, theme, &toggle).unwrap_or(btn_h);
                    cursor.x = theme.padding_standard;
                    cursor.right = (width - theme.padding_standard - fullscreen_width - theme.gap_standard - toggle_width - theme.gap_standard).max(cursor.x);
                    cursor.item = 0;
                    cursor.phase = 2;
                    return false;
                }
                self.prepare_panel_row_borders(cursor, atlas, input, theme, PanelAnchor::TopLeft, theme.padding_standard, width - theme.padding_standard, btn_y, btn_h, false);
                let Some((anchor, node, chip)) = self.navbar_leading_tab_row_rect(atlas, theme, cursor.item, cursor.x, btn_y, btn_h) else {
                    cursor.shared_border_plan.clear();
                    cursor.shared_border_rects.clear();
                    let layout = self.navbar_chrome_layout(atlas, theme, width);
                    cursor.x = layout.center.centered.left;
                    cursor.right = layout.center.centered.right;
                    cursor.item = 0;
                    cursor.rect = None;
                    cursor.phase = 2;
                    return false;
                };
                let control_id = node.id.clone();
                let item = ChromeGroupItem {
                    control_id: control_id.as_str(),
                    icon_id: Some(node.icon_id.as_str()),
                    label: Some(node.label.as_str()),
                    active: self.dock.anchor_open(anchor) && self.dock.anchor_state(anchor).path.first() == Some(&node.id),
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    cursor.rect = shell_chrome_clip_horizontal(chip, ShellChromeBandSpan { left: theme.padding_standard, right: width - theme.padding_standard });
                }
                let Some(rect) = cursor.rect else {
                    cursor.item += 1;
                    return false;
                };
                let borders = cursor.shared_border_plan.get(&control_id).map(Vec::as_slice);
                match render_retained_panel_chrome_item_step_with_borders(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true, borders) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell navbar leading panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                cursor.x = rect.x + rect.w;
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            2 => {
                let size = theme.gap_standard * SHELL_NAVBAR_LOGO_UI_SPACING;
                if cursor.x + size <= cursor.right {
                    chrome_icon(draw, icons, "semio-logo", cursor.x, btn_y + (btn_h - size) * 0.5, size, LOGO_UNTINTED);
                }
                cursor.x = (cursor.x + size + theme.gap_standard).min(cursor.right);
                cursor.phase = 3;
            }
            3 => {
                if cursor.window.is_none() {
                    let title = self.docs.session.as_ref().map_or_else(|| SHELL_NAVBAR_SESSIONLESS_TITLE.to_string(), |session| shell_navbar_title(&session.app, &self.prefs.terminology_id, self.prefs.active_locale()));
                    let Ok(title) = UiText::try_from_string(title) else {
                        self.chrome.error = Some("Shell navbar title exceeded the retained text boundary".to_string());
                        cursor.phase = 11;
                        return false;
                    };
                    cursor.window = Some(title);
                }
                let role_width = self.docs
                    .session
                    .as_ref()
                    .map(|session| {
                        let label = shell_surface_role_chip_text(session.app.role, self.prefs.locale_id == "de");
                        let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
                        retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(0.0) + theme.gap_standard
                    })
                    .unwrap_or(0.0);
                let Some(title) = cursor.window.as_ref() else { return false };
                if cursor.rect.is_none() {
                    let wanted = atlas.measure_text(title.as_str(), theme.font_size_body).0 + theme.padding_standard * 2.0;
                    let available = (cursor.right - cursor.x - role_width).max(0.0);
                    cursor.rect = Some(Rect::new(cursor.x, btn_y, wanted.min(available), btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                if rect.w > theme.padding_standard * 2.0 {
                    match chrome_text_complete_step(
                        draw,
                        atlas,
                        title.as_str(),
                        rect.x + theme.padding_standard,
                        btn_y + (btn_h + theme.font_size_body) * 0.5 - 2.0,
                        (rect.w - theme.padding_standard * 2.0).max(1.0),
                        theme.font_size_body,
                        theme.text,
                        &mut cursor.glyph,
                    ) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.chrome.error = Some("Shell navbar text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    }
                }
                cursor.x = (rect.x + rect.w + theme.gap_standard).min(cursor.right);
                cursor.window = None;
                cursor.rect = None;
                cursor.phase = 11;
            }
            11 => {
                let Some(role) = self.docs.session.as_ref().map(|session| session.app.role) else {
                    cursor.phase = if mobile { 7 } else { 4 };
                    return false;
                };
                let label = shell_surface_role_chip_text(role, self.prefs.locale_id == "de");
                let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        self.chrome.error = Some("Shell navbar role chip exceeded the retained chrome boundary".to_string());
                        cursor.phase = if mobile { 7 } else { 4 };
                        return false;
                    };
                    cursor.rect = shell_chrome_clip_horizontal(Rect::new(cursor.x, btn_y, item_w, btn_h), ShellChromeBandSpan { left: cursor.x, right: cursor.right });
                }
                let Some(rect) = cursor.rect else {
                    cursor.phase = if mobile { 7 } else { 4 };
                    return false;
                };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell navbar role chip exceeded the retained glyph boundary".to_string()),
                }
                cursor.x = rect.x + rect.w + if mobile { 0.0 } else { theme.gap_standard * 2.0 };
                cursor.rect = None;
                cursor.phase = if mobile { 7 } else { 4 };
            }
            4 => {
                let rows = if semio_renderer_wgpu_host_base::boot_locks().example_id.is_empty() { self.docs.session_example_rows(&self.plugins, &self.prefs) } else { Vec::new() };
                let controls: Vec<ShellNavbarControl> = shell_example_control(&rows, matches!(&self.input.overlay_state, OverlayState::Dropdown(id) if id == "example"), self.prefs.locale_id == "de").into_iter().collect();
                if !self.render_navbar_cluster_step(cursor, draw, atlas, icons, input, theme, &controls, btn_y, btn_h) {
                    return false;
                }
                if !controls.is_empty() {
                    cursor.x += theme.gap_standard * 2.0;
                }
                cursor.phase = 5;
            }
            5 => {
                let controls = self.docs.session.as_ref().map(|session| shell_mode_controls(&session.app, session.view_state.active_mode_id.as_deref(), self.prefs.active_terminology(), self.prefs.active_locale())).unwrap_or_default();
                if !self.render_navbar_cluster_step(cursor, draw, atlas, icons, input, theme, &controls, btn_y, btn_h) {
                    return false;
                }
                if !controls.is_empty() {
                    cursor.x += theme.gap_standard * 2.0;
                }
                cursor.phase = 6;
            }
            6 => {
                let controls = self.docs
                    .session
                    .as_ref()
                    .and_then(|session| {
                        self.plugins.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| shell_role_controls(&entry.manifest.apps, &session.app, self.prefs.active_terminology(), self.prefs.active_locale(), &self.chrome.shortcut_table()))
                    })
                    .unwrap_or_default();
                if !self.render_navbar_cluster_step(cursor, draw, atlas, icons, input, theme, &controls, btn_y, btn_h) {
                    return false;
                }
                if !controls.is_empty() {
                    cursor.x += theme.gap_standard * 2.0;
                }
                cursor.item = 0;
                cursor.phase = 12;
            }
            12 => {
                let (left, right) = (cursor.x, cursor.right);
                self.prepare_panel_row_borders(cursor, atlas, input, theme, PanelAnchor::TopMiddle, left, right, btn_y, btn_h, false);
                let Some((anchor, node)) = self.navbar_center_tab_row_item(cursor.item) else {
                    cursor.shared_border_plan.clear();
                    cursor.shared_border_rects.clear();
                    cursor.item = 0;
                    cursor.rect = None;
                    cursor.phase = 7;
                    return false;
                };
                let control_id = node.id.clone();
                let item = ChromeGroupItem {
                    control_id: control_id.as_str(),
                    icon_id: Some(node.icon_id.as_str()),
                    label: Some(node.label.as_str()),
                    active: self.dock.anchor_open(anchor) && self.dock.anchor_state(anchor).path.first() == Some(&node.id),
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    let Some(item_width) = retained_panel_chrome_item_width(atlas, theme, &item) else {
                        cursor.item += 1;
                        return false;
                    };
                    cursor.rect = shell_chrome_clip_horizontal(Rect::new(cursor.x, btn_y, item_width, btn_h), ShellChromeBandSpan { left: cursor.x, right: cursor.right });
                }
                let Some(rect) = cursor.rect else {
                    cursor.item += 1;
                    return false;
                };
                let borders = cursor.shared_border_plan.get(&control_id).map(Vec::as_slice);
                match render_retained_panel_chrome_item_step_with_borders(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true, borders) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell navbar centered panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                cursor.x = rect.x + rect.w;
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            7 => {
                let is_de = self.prefs.locale_id == "de";
                let item = ChromeGroupItem {
                    control_id: "ui.fullscreen.toggle",
                    icon_id: Some(if self.chrome.fullscreen_active { "minimize-2" } else { "maximize-2" }),
                    label: Some(if self.chrome.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
                    active: self.chrome.fullscreen_active,
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    let item_w = retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(btn_h);
                    cursor.rect = shell_chrome_clip_horizontal(Rect::new(width - theme.padding_standard - item_w, btn_y, item_w, btn_h), ShellChromeBandSpan { left: theme.padding_standard, right: width - theme.padding_standard });
                }
                let Some(rect) = cursor.rect else {
                    cursor.phase = 8;
                    return false;
                };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell fullscreen item exceeded the retained glyph boundary".to_string()),
                }
                cursor.rect = None;
                cursor.item = 0;
                cursor.phase = 8;
            }
            8 => {
                if !mobile {
                    let right = self.navbar_chrome_layout(atlas, theme, width).trailing_right;
                    self.prepare_panel_row_borders(cursor, atlas, input, theme, PanelAnchor::TopRight, theme.padding_standard, right, btn_y, btn_h, true);
                }
                let row = (!mobile).then(|| self.navbar_trailing_tab_row_item(cursor.item)).flatten();
                if !mobile && row.is_none() {
                    cursor.shared_border_plan.clear();
                    cursor.shared_border_rects.clear();
                    cursor.phase = 9;
                    return false;
                }
                if mobile && cursor.item > 0 {
                    cursor.phase = 9;
                    return false;
                }
                let control_id = row.as_ref().map_or_else(|| "ui.mobilePanel.toggle".to_string(), |(_, node)| node.id.clone());
                let item = match row.as_ref() {
                    Some((anchor, node)) => ChromeGroupItem {
                        control_id: control_id.as_str(),
                        icon_id: Some(node.icon_id.as_str()),
                        label: Some(node.label.as_str()),
                        active: self.dock.anchor_open(*anchor) && self.dock.anchor_state(*anchor).path.first() == Some(&node.id),
                        disabled: false,
                        kind: HitKind::Toggle,
                    },
                    None => ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some("panel-left"), label: None, active: self.dock.mobile_panel_visible, disabled: false, kind: HitKind::Toggle },
                };
                if cursor.rect.is_none() {
                    let item_w = if row.is_some() { retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(btn_h) } else { retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(btn_h) };
                    let right = if mobile {
                        width - theme.padding_standard - self.prefs.fullscreen_item_width(atlas, theme, &self.chrome) - theme.gap_standard
                    } else {
                        self.navbar_chrome_layout(atlas, theme, width).trailing_right
                            - self.dock
                                .dock_tabs
                                .tabs(PanelAnchor::TopRight)
                                .iter()
                                .rev()
                                .take(cursor.item)
                                .map(|node| {
                                    let item = ChromeGroupItem { control_id: "", icon_id: Some(node.icon_id.as_str()), label: Some(node.label.as_str()), active: false, disabled: false, kind: HitKind::Toggle };
                                    retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(0.0)
                                })
                                .sum::<f32>()
                    };
                    cursor.rect = shell_chrome_clip_horizontal(Rect::new(right - item_w, btn_y, item_w, btn_h), ShellChromeBandSpan { left: theme.padding_standard, right: width - theme.padding_standard });
                }
                let Some(rect) = cursor.rect else {
                    cursor.item += 1;
                    return false;
                };
                let step = if row.is_some() {
                    let borders = cursor.shared_border_plan.get(&control_id).map(Vec::as_slice);
                    render_retained_panel_chrome_item_step_with_borders(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true, borders)
                } else {
                    render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true)
                };
                match step {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell panel toggle exceeded the retained glyph boundary".to_string()),
                }
                if row.is_some() {
                    self.chrome.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                }
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            9 => return true,
            _ => return false,
        }
        false
    }

    /// 🛑️🖼️ The overlay controls each LIVE engine surface offers, with the anchor each is painted at.
    ///
    /// - World3d → the compute CANCEL, offered exactly while the scene's own status document says
    ///   `cancellable` AND names a non-empty `cancelAction`. The shell never learns `cancelPreviewEval`
    ///   from code: it dispatches whatever id the surface's status contract published
    ///   (`🌐️World3dHost/🟦️.tsx`'s `WorldComputeStatusPane` does exactly the same in React).
    ///
    /// Framing the flow graph is an app-declared action (`zoomToFlow`), not shell overlay chrome.
    pub fn surface_overlay_controls(&mut self, theme: &Theme) -> Vec<(ShellNavbarControl, [f32; 2])> {
        let panels = self.open_floating_panel_rects(theme);
        let graphs: Vec<(&str, Rect)> = self.surfaces.node_graph_states.iter().map(|(surface_id, surface)| (surface_id.as_str(), surface.bounds)).collect();
        let worlds = self.surfaces.world3d_status_rows();
        surface_overlay_controls_for(&graphs, &worlds, &panels, theme, self.prefs.locale_id == "de")
    }

    /// 🛟️ The boxes this frame's OPEN floating panels occupy, in the same screen space the dock lays its
    /// windows out in — the wgpu twin of React's `publishShellChromePanelBox`. A panel paints over the
    /// whole body (`render_panel_step` runs after `plan_dock_windows` over the same rect), so this is the
    /// one publication a surface's overlay row reserves against (`chrome_panel_safe_area`).
    pub fn open_floating_panel_rects(&mut self, theme: &Theme) -> Vec<Rect> {
        let body = self.as_session().body_rect(theme);
        self.dock.open_anchors().into_iter().map(|anchor| self.anchor_rect(anchor, body, theme)).collect()
    }

    /// ⏳️🖼️ The compute-status pills each LIVE World3d surface asks for — the wgpu twin of React's
    /// `WorldComputeStatusPane` (`🌐️World3dHost/🟦️.tsx`): the producer's own phase label in the
    /// reader's language, its `unitsDone/unitsTotal` count and the fraction the bar fills to, painted
    /// exactly while the scene's own status document is unsettled.
    pub fn surface_status_pills(&mut self, theme: &Theme) -> Vec<(World3dStatusPill, Rect)> {
        let panels = self.open_floating_panel_rects(theme);
        let worlds = self.surfaces.world3d_status_rows();
        let pills = surface_status_pills_for(&worlds, &panels, theme, self.prefs.locale_id == "de");
        let live: Vec<&str> = pills.iter().map(|(pill, _)| pill.surface_id.as_str()).collect();
        for (pill, rect) in &pills {
            let label = pill.label();
            if self.surfaces.world3d_status_pill_trace.get(&pill.surface_id) == Some(&label) {
                continue;
            }
            debug_log(&format!(
                "[TRACE] wgpu world3d status pill surface={} phase={} label={label:?} ratio={:?} computing={} rect={}x{}+{},{}",
                pill.surface_id,
                pill.phase,
                pill.ratio,
                pill.computing,
                rect.w.round(),
                rect.h.round(),
                rect.x.round(),
                rect.y.round()
            ));
            self.surfaces.world3d_status_pill_trace.insert(pill.surface_id.clone(), label);
        }
        self.surfaces.world3d_status_pill_trace.retain(|surface_id, _| live.contains(&surface_id.as_str()));
        pills
    }

    /// 🎛️ Paints ONE centre-cluster navbar control per grant, left-to-right from `cursor.x`, and
    /// answers `true` once the whole list is painted — the retained-chrome twin of React's navbar
    /// centre cluster (`exampleSelectElement` + `modeSwitcherElement` + `roleSwitcherElement`).
    /// An empty list paints nothing at all, which is exactly React's "no group" render gate.
    ///
    /// ⌨️ The inline hotkey badge belongs to EVERY bound chrome control, not only to the two role
    /// chips that used to append it themselves (audit W14 §B14) — and it has to be appended here,
    /// before `retained_chrome_group_item_width` measures the label, or the chip would lay out
    /// narrower than the text it paints.
    #[allow(clippy::too_many_arguments)]
    pub fn render_navbar_cluster_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        draw: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        controls: &[ShellNavbarControl],
        btn_y: f32,
        btn_h: f32,
    ) -> bool {
        if cursor.item == 0 && cursor.group_phase == 0 && cursor.shared_border_rects.is_empty() {
            let mut x = cursor.x;
            let mut frames = Vec::new();
            for control in controls {
                let Some(width) = self.navbar_control_width(atlas, theme, control) else { continue };
                let Some(rect) = shell_chrome_clip_horizontal(Rect::new(x, btn_y, width, btn_h), ShellChromeBandSpan { left: x, right: cursor.right }) else { break };
                let hovered = rect.contains(input.pointer_x, input.pointer_y);
                let state = if hovered { semio_renderer_wgpu_dock::shared_borders::BorderState::Hover } else if control.active { semio_renderer_wgpu_dock::shared_borders::BorderState::Active } else { semio_renderer_wgpu_dock::shared_borders::BorderState::Normal };
                frames.push(ChromeBorderFrame { owner: control.control_id.clone(), rect: [rect.x, rect.y, rect.w, rect.h], state });
                cursor.shared_border_rects.insert(control.control_id.clone(), rect);
                x = rect.x + rect.w;
            }
            match chrome_border_plan(&frames, theme.stroke_hairline) {
                Ok(plan) => cursor.shared_border_plan = plan,
                Err(error) => {
                    self.chrome.error = Some(error.into());
                }
            }
        }
        let Some(control) = controls.get(cursor.item) else {
            cursor.item = 0;
            cursor.rect = None;
            cursor.shared_border_plan.clear();
            cursor.shared_border_rects.clear();
            return true;
        };
        let label = self.chrome.chrome_control_label(&control.control_id, &control.label);
        let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
        if cursor.rect.is_none() {
            cursor.rect = cursor.shared_border_rects.get(&control.control_id).copied();
        }
        let Some(rect) = cursor.rect else {
            cursor.item += 1;
            return false;
        };
        let borders = cursor.shared_border_plan.get(&control.control_id).map(Vec::as_slice);
        match render_retained_chrome_group_item_step_with_borders(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true, borders) {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell navbar cluster item exceeded the retained glyph boundary".to_string()),
        }
        cursor.x = rect.x + rect.w;
        cursor.rect = None;
        cursor.item += 1;
        false
    }
}

//#endregion ShellChrome
