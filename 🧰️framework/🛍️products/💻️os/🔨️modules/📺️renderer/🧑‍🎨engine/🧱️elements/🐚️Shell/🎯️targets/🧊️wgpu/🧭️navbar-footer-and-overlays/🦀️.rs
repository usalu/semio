//! 🧭️ Render steps for navbar, footer, overlays and notices.
//!
//! Second render half of `impl ShellState`: the navbar, overlay controls, navbar cluster, tutorial bar, footer,
//! overlay, context menu, tooltip, agent chat header, transient notice, plugin install and document opening
//! steps and the space administration plan.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellChrome
impl ShellState {
    pub(crate) fn render_navbar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let btn_h = theme.control_height;
        let btn_y = (theme.navbar_height - btn_h) * 0.5;
        let mobile = self.mobile_panel_active();
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, 0.0, width, theme.navbar_height], theme.navbar);
                cursor.x = theme.padding_standard;
                cursor.right = width - theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, 0.0, width, theme.navbar_height).contains(input.pointer_x, input.pointer_y);
                draw.push_solid([0.0, theme.navbar_height - theme.stroke_hairline, width, theme.stroke_hairline], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.item = 0;
                cursor.rect = None;
                cursor.phase = 10;
            }
            10 => {
                if mobile {
                    let fullscreen_width = self.fullscreen_item_width(atlas, theme);
                    let toggle = ChromeGroupItem { control_id: "ui.mobilePanel.toggle", icon_id: Some("panel-left"), label: None, active: self.mobile_panel_visible, disabled: false, kind: HitKind::Toggle };
                    let toggle_width = retained_chrome_group_item_width(atlas, theme, &toggle).unwrap_or(btn_h);
                    cursor.x = theme.padding_standard;
                    cursor.right = (width - theme.padding_standard - fullscreen_width - theme.gap_standard - toggle_width - theme.gap_standard).max(cursor.x);
                    cursor.item = 0;
                    cursor.phase = 2;
                    return false;
                }
                let Some((anchor, node, chip)) = self.navbar_leading_tab_row_rect(atlas, theme, cursor.item, cursor.x, btn_y, btn_h) else {
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
                    active: self.anchor_open(anchor) && self.anchor_state(anchor).path.first() == Some(&node.id),
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
                match render_retained_panel_chrome_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell navbar leading panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
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
                    let title = self.session.as_ref().map_or_else(|| SHELL_NAVBAR_SESSIONLESS_TITLE.to_string(), |session| shell_navbar_title(&session.app, &self.terminology_id));
                    let Ok(title) = UiText::try_from_string(title) else {
                        self.error = Some("Shell navbar title exceeded the retained text boundary".to_string());
                        cursor.phase = 11;
                        return false;
                    };
                    cursor.window = Some(title);
                }
                let role_width = self
                    .session
                    .as_ref()
                    .map(|session| {
                        let label = shell_surface_role_chip_text(session.app.role, self.locale_id == "de");
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
                            self.error = Some("Shell navbar text exceeded the retained glyph boundary".to_string());
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
                let Some(role) = self.session.as_ref().map(|session| session.app.role) else {
                    cursor.phase = if mobile { 7 } else { 4 };
                    return false;
                };
                let label = shell_surface_role_chip_text(role, self.locale_id == "de");
                let item = ChromeGroupItem { control_id: "", icon_id: None, label: Some(label), active: false, disabled: false, kind: HitKind::Button };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        self.error = Some("Shell navbar role chip exceeded the retained chrome boundary".to_string());
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
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell navbar role chip exceeded the retained glyph boundary".to_string()),
                }
                cursor.x = rect.x + rect.w + if mobile { 0.0 } else { theme.gap_standard * 2.0 };
                cursor.rect = None;
                cursor.phase = if mobile { 7 } else { 4 };
            }
            4 => {
                let rows = if crate::boot_locks().example_id.is_empty() { self.session_example_rows() } else { Vec::new() };
                let controls: Vec<ShellNavbarControl> = shell_example_control(&rows, matches!(&self.overlay_state, OverlayState::Dropdown(id) if id == "example"), self.locale_id == "de").into_iter().collect();
                if !self.render_navbar_cluster_step(cursor, draw, atlas, icons, input, theme, &controls, btn_y, btn_h) {
                    return false;
                }
                if !controls.is_empty() {
                    cursor.x += theme.gap_standard * 2.0;
                }
                cursor.phase = 5;
            }
            5 => {
                let controls = self.session.as_ref().map(|session| shell_mode_controls(&session.app, session.view_state.active_mode_id.as_deref(), self.active_terminology(), self.active_locale())).unwrap_or_default();
                if !self.render_navbar_cluster_step(cursor, draw, atlas, icons, input, theme, &controls, btn_y, btn_h) {
                    return false;
                }
                if !controls.is_empty() {
                    cursor.x += theme.gap_standard * 2.0;
                }
                cursor.phase = 6;
            }
            6 => {
                let controls = self
                    .session
                    .as_ref()
                    .and_then(|session| {
                        self.plugins.iter().find(|entry| entry.plugin_id == session.plugin_id).map(|entry| shell_role_controls(&entry.manifest.apps, &session.app, self.active_terminology(), self.active_locale(), &self.shortcut_table()))
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
                let Some((anchor, node)) = self.navbar_center_tab_row_item(cursor.item) else {
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
                    active: self.anchor_open(anchor) && self.anchor_state(anchor).path.first() == Some(&node.id),
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
                match render_retained_panel_chrome_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell navbar centered panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                cursor.x = rect.x + rect.w;
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            7 => {
                let is_de = self.locale_id == "de";
                let item = ChromeGroupItem {
                    control_id: "ui.fullscreen.toggle",
                    icon_id: Some(if self.fullscreen_active { "minimize-2" } else { "maximize-2" }),
                    label: Some(if self.fullscreen_active { shell_chrome_string("fullscreen.exit", is_de) } else { shell_chrome_string("fullscreen.toggle", is_de) }),
                    active: self.fullscreen_active,
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
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell fullscreen item exceeded the retained glyph boundary".to_string()),
                }
                cursor.rect = None;
                cursor.item = 0;
                cursor.phase = 8;
            }
            8 => {
                let row = (!mobile).then(|| self.navbar_trailing_tab_row_item(cursor.item)).flatten();
                if !mobile && row.is_none() {
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
                        active: self.anchor_open(*anchor) && self.anchor_state(*anchor).path.first() == Some(&node.id),
                        disabled: false,
                        kind: HitKind::Toggle,
                    },
                    None => ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some("panel-left"), label: None, active: self.mobile_panel_visible, disabled: false, kind: HitKind::Toggle },
                };
                if cursor.rect.is_none() {
                    let item_w = if row.is_some() { retained_panel_chrome_item_width(atlas, theme, &item).unwrap_or(btn_h) } else { retained_chrome_group_item_width(atlas, theme, &item).unwrap_or(btn_h) };
                    let right = if mobile {
                        width - theme.padding_standard - self.fullscreen_item_width(atlas, theme) - theme.gap_standard
                    } else {
                        self.navbar_chrome_layout(atlas, theme, width).trailing_right
                            - self
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
                    render_retained_panel_chrome_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true)
                } else {
                    render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true)
                };
                match step {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell panel toggle exceeded the retained glyph boundary".to_string()),
                }
                if row.is_some() {
                    self.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
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
    fn surface_overlay_controls(&self, theme: &Theme) -> Vec<(ShellNavbarControl, [f32; 2])> {
        let graphs: Vec<(&str, Rect)> = self.node_graph_states.iter().map(|(surface_id, surface)| (surface_id.as_str(), surface.bounds)).collect();
        let worlds = self.world3d_status_rows();
        let worlds: Vec<(&str, Rect, Option<&str>)> = worlds.iter().map(|(id, bounds, status)| (*id, *bounds, *status)).collect();
        surface_overlay_controls_for(&graphs, &worlds, &self.open_floating_panel_rects(theme), theme, self.locale_id == "de")
    }

    /// 🛟️ The boxes this frame's OPEN floating panels occupy, in the same screen space the dock lays its
    /// windows out in — the wgpu twin of React's `publishShellChromePanelBox`. A panel paints over the
    /// whole body (`render_panel_step` runs after `plan_dock_windows` over the same rect), so this is the
    /// one publication a surface's overlay row reserves against (`chrome_panel_safe_area`).
    fn open_floating_panel_rects(&self, theme: &Theme) -> Vec<Rect> {
        let body = self.body_rect(theme);
        self.open_anchors().into_iter().map(|anchor| self.anchor_rect(anchor, body, theme)).collect()
    }

    /// ⏳️🖼️ Each live World3d surface with the compute-status document it published — the one input
    /// both the status pill and the cancel control read.
    fn world3d_status_rows(&self) -> Vec<(&str, Rect, Option<&str>)> {
        self.world3d_states.iter().map(|(surface_id, world)| (surface_id.as_str(), world.bounds, self.world3d_status.get(surface_id).map(String::as_str))).collect()
    }

    /// ⏳️🖼️ The compute-status pills each LIVE World3d surface asks for — the wgpu twin of React's
    /// `WorldComputeStatusPane` (`🌐️World3dHost/🟦️.tsx`): the producer's own phase label in the
    /// reader's language, its `unitsDone/unitsTotal` count and the fraction the bar fills to, painted
    /// exactly while the scene's own status document is unsettled.
    fn surface_status_pills(&mut self, theme: &Theme) -> Vec<(World3dStatusPill, Rect)> {
        let worlds = self.world3d_status_rows();
        let worlds: Vec<(&str, Rect, Option<&str>)> = worlds.iter().map(|(id, bounds, status)| (*id, *bounds, *status)).collect();
        let pills = surface_status_pills_for(&worlds, &self.open_floating_panel_rects(theme), theme, self.locale_id == "de");
        let live: Vec<&str> = pills.iter().map(|(pill, _)| pill.surface_id.as_str()).collect();
        for (pill, rect) in &pills {
            let label = pill.label();
            if self.world3d_status_pill_trace.get(&pill.surface_id) == Some(&label) {
                continue;
            }
            Self::debug_log(&format!(
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
            self.world3d_status_pill_trace.insert(pill.surface_id.clone(), label);
        }
        self.world3d_status_pill_trace.retain(|surface_id, _| live.contains(&surface_id.as_str()));
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
    pub(crate) fn render_navbar_cluster_step(
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
        let Some(control) = controls.get(cursor.item) else {
            cursor.item = 0;
            cursor.rect = None;
            return true;
        };
        let label = self.chrome_control_label(&control.control_id, &control.label);
        let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
        if cursor.rect.is_none() {
            let Some(item_w) = self.navbar_control_width(atlas, theme, control) else {
                self.error = Some("Shell navbar cluster item exceeded the retained chrome boundary".to_string());
                cursor.item += 1;
                return false;
            };
            cursor.rect = shell_chrome_clip_horizontal(Rect::new(cursor.x, btn_y, item_w, btn_h), ShellChromeBandSpan { left: cursor.x, right: cursor.right });
        }
        let Some(rect) = cursor.rect else {
            cursor.item += 1;
            return false;
        };
        match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.error = Some("Shell navbar cluster item exceeded the retained glyph boundary".to_string()),
        }
        cursor.x = rect.x + rect.w;
        cursor.rect = None;
        cursor.item += 1;
        false
    }

    pub(crate) fn render_tutorial_bar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(runtime) = self.tutorial.as_ref() else { return true };
        let mode = runtime.mode;
        let playhead_ms = runtime.playhead_ms;
        let duration_ms = runtime.definition.duration_ms;
        let rate = runtime.rate;
        let bar_h = tutorial_bar_height(theme);
        let y = theme.navbar_height;
        let btn_h = theme.control_height;
        let btn_y = y + (bar_h - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, y, width, bar_h], theme.navbar);
                cursor.x = theme.padding_standard;
                cursor.right = width - theme.padding_standard;
                cursor.phase = 1;
            }
            1 => {
                draw.push_solid([0.0, y + bar_h - theme.stroke_hairline, width, theme.stroke_hairline], theme.border_normal);
                cursor.phase = 2;
            }
            2 if mode != TutorialMode::Recording => {
                let playing = mode == TutorialMode::Playing;
                let item = ChromeGroupItem { control_id: "shell.tutorial.playPause", icon_id: Some(if playing { "pause" } else { "play" }), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        cursor.phase = 9;
                        return false;
                    };
                    cursor.rect = Some(Rect::new(cursor.x, btn_y, item_w.max(btn_h), btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial play item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_toggle_play_pause();
                }
                cursor.x += rect.w + theme.gap_standard;
                cursor.rect = None;
                cursor.phase = 3;
            }
            2 => cursor.phase = 3,
            3 => {
                let item = ChromeGroupItem { control_id: "shell.tutorial.stop", icon_id: Some("square"), label: None, active: false, disabled: false, kind: HitKind::NavbarItem };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        cursor.phase = 9;
                        return false;
                    };
                    cursor.rect = Some(Rect::new(cursor.x, btn_y, item_w.max(btn_h), btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial stop item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_stop();
                    cursor.rect = None;
                    cursor.phase = 9;
                    return false;
                }
                cursor.x += rect.w + theme.gap_standard;
                cursor.rect = None;
                cursor.phase = 4;
            }
            4 => {
                let current = (playhead_ms / 1000.0).max(0.0) as u64;
                let label = if mode == TutorialMode::Recording {
                    format!("REC {}:{:02}", current / 60, current % 60)
                } else {
                    let total = duration_ms / 1000;
                    format!("{}:{:02} / {}:{:02}", current / 60, current % 60, total / 60, total % 60)
                };
                match chrome_text_complete_step(draw, atlas, &label, cursor.x, btn_y + (btn_h + theme.font_size_small) * 0.5 - 1.0, (cursor.right - cursor.x).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell tutorial-bar text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.x += theme.font_size_small * 13.0 + theme.gap_standard * 2.0;
                cursor.phase = 5;
            }
            5 if mode != TutorialMode::Recording => {
                const RATES: [(f32, &str, &str); 4] = [(2.0, "2x", "shell.tutorial.rate.2"), (1.5, "1.5x", "shell.tutorial.rate.1_5"), (1.0, "1x", "shell.tutorial.rate.1"), (0.5, "0.5x", "shell.tutorial.rate.0_5")];
                let Some((value, label, control_id)) = RATES.get(cursor.item).copied() else {
                    cursor.phase = 6;
                    return false;
                };
                let item = ChromeGroupItem { control_id, icon_id: None, label: Some(label), active: (rate - value).abs() < 0.01, disabled: false, kind: HitKind::Toggle };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        self.error = Some("Shell tutorial rate exceeded the retained chrome boundary".to_string());
                        cursor.phase = 6;
                        return false;
                    };
                    cursor.right -= item_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell tutorial rate exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    if let Some(runtime) = self.tutorial.as_mut() {
                        runtime.rate = value;
                    }
                }
                cursor.rect = None;
                cursor.item += 1;
            }
            5 => cursor.phase = 6,
            6 => {
                let rect = Rect::new(cursor.x, btn_y, (cursor.right - cursor.x).max(24.0), btn_h);
                cursor.rect = Some(rect);
                draw.push_rounded([rect.x, btn_y + btn_h * 0.5 - 2.0, rect.w, 4.0], theme.border_normal, 2.0);
                cursor.phase = 7;
            }
            7 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                let knob_x = rect.x + rect.w * tutorial_scrub_progress(playhead_ms, duration_ms);
                draw.push_rounded([knob_x - 6.0, btn_y + btn_h * 0.5 - 6.0, 12.0, 12.0], theme.selected, 6.0);
                cursor.phase = 8;
            }
            8 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                input.register_hit(HitTarget { rect, event: None, control_id: Some("shell.tutorial.scrubber".into()), kind: HitKind::Slider, drag_axis: None, drag_data: None });
                if mode != TutorialMode::Recording && input.pointer_down && rect.contains(input.pointer_x, input.pointer_y) {
                    self.tutorial_seek(tutorial_scrub_target_ms(input.pointer_x, rect, duration_ms));
                }
                cursor.phase = 9;
            }
            9 => return true,
            _ => return false,
        }
        false
    }

    pub(crate) fn render_footer_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        let y = height - theme.footer_height - self.subfooter_height(theme);
        let btn_h = theme.control_height;
        let btn_y = y + (theme.footer_height - btn_h) * 0.5;
        match cursor.phase {
            0 => {
                draw.push_solid([0.0, y, width, theme.footer_height], theme.navbar);
                cursor.phase = 1;
            }
            1 => {
                let hovered = Rect::new(0.0, y, width, theme.footer_height).contains(input.pointer_x, input.pointer_y);
                draw.push_solid([0.0, y, width, theme.stroke_hairline], if hovered { theme.border_emphasized } else { theme.border_normal });
                cursor.item = 0;
                cursor.phase = 2;
            }
            2 => {
                let Some((anchor, node, band_rect)) = self.footer_tab_row_rect(atlas, theme, cursor.item, width, btn_y, btn_h) else {
                    cursor.item = 0;
                    cursor.rect = None;
                    cursor.phase = 3;
                    return false;
                };
                let control_id = node.id.clone();
                let item = ChromeGroupItem {
                    control_id: control_id.as_str(),
                    icon_id: Some(node.icon_id.as_str()),
                    label: Some(node.label.as_str()),
                    active: self.anchor_open(anchor) && self.anchor_state(anchor).path.first() == Some(&node.id),
                    disabled: false,
                    kind: HitKind::Toggle,
                };
                if cursor.rect.is_none() {
                    cursor.rect = Some(band_rect);
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_panel_chrome_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell footer panel tab exceeded the retained glyph boundary".to_string()),
                }
                self.chrome_build.register_element_rect(panel_tab_introduction_element_id(&control_id), rect);
                cursor.rect = None;
                cursor.item += 1;
                return false;
            }
            3 => {
                if self.mobile_panel_active() {
                    cursor.phase = 4;
                    return false;
                }
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let Some(rect) = layout.presence else {
                    cursor.phase = 4;
                    return false;
                };
                let presence = self.footer_presence_rows();
                let label = ui_wgpu::wgpu::presence_bar_chip_text(&presence, None, self.active_locale());
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, rect, "s-presence-peers", None, &label, false) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 4,
                    Err(()) => {
                        self.error = Some("Shell presence footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 4;
                    }
                }
            }
            4 => {
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let hub = self.hub_connection_state();
                let label = self.hub_footer_label();
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, layout.hub, "s-hub-connection", Some(hub.icon_id()), &label, false) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 5,
                    Err(()) => {
                        self.error = Some("Shell hub footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 5;
                    }
                }
            }
            5 => {
                let layout = self.footer_chrome_layout(atlas, theme, width, btn_y, btn_h);
                let Some(rect) = layout.sign_in else {
                    cursor.phase = 6;
                    return false;
                };
                let label = shell_chrome_string("hub.signIn", self.locale_id == "de");
                match render_footer_status_step(cursor, draw, atlas, icons, input, theme, rect, "framework.hub.signIn", Some("user"), label, true) {
                    Ok(false) => return false,
                    Ok(true) => cursor.phase = 6,
                    Err(()) => {
                        self.error = Some("Shell hub sign-in footer exceeded the retained chrome boundary".to_string());
                        cursor.phase = 6;
                    }
                }
            }
            6 => return true,
            _ => return false,
        }
        false
    }

    /// 📚️ One hit-testable row per example the OPEN dialect authored, each carrying the
    /// `shell.example.<id>` control id `handle_shell_hit` already dispatches `setActiveExample`
    /// for. The handler predates the dialect refactor and was unreachable code until this
    /// emission existed (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// ⏳️🖼️ Per-surface World3d compute-status pills — the wgpu twin of React's
    /// `WorldComputeStatusPane`: a glass row carrying the producer's own phase label, its
    /// `unitsDone/unitsTotal` count and a bar filled to the published ratio. Painted BEFORE the
    /// overlay controls so the cancel control the same row carries registers its hit last and
    /// wins `InputState::hit_at`'s reverse-order resolution. Non-interactive by construction:
    /// a pill registers no hit at all, so it never steals a press from the surface beneath it.
    ///
    /// 👥️🖼️ Peer presence over every board window — the wgpu twin of React's
    /// `CanvasPresenceOverlayV1`: each other actor's viewport rectangle, cursor dot and name chip,
    /// then every peer mark as a corner chip of the peer's initials, clipped to the board and in
    /// the hub-assigned colour. Non-interactive by construction: nothing here registers a hit.
    ///
    /// 🛑️🖼️ Per-surface overlay controls — the World3d compute cancel. Painted last so they sit
    /// above the surface they annotate, and registered last so `InputState::hit_at` (reverse
    /// order) resolves them over the surface's own hit.
    /// Each grant recomputes its own rect from the live surface bounds, so nothing survives
    /// between grants and a surface that stopped painting stops offering its control.
    ///
    /// ⌨️🖼️ The keyboard focus ring around the chrome control the keyboard ring (or an assistive
    /// technology) focused — the wgpu twin of the browser's `:focus-visible` outline. Painted last,
    /// above every chrome layer; registers no hit.
    pub(crate) fn render_overlay_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        match cursor.phase {
            0 => {
                if let Some(plan) = self.command_palette_plan(theme, width, height) {
                    overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
                    let rect = plan.dialog;
                    open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Dialog));
                    let title = shell_chrome_string(if plan.kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, self.locale_id == "de");
                    note_chrome_control_name(plan.kind.dialog_id(), Some(title));
                    input.register_hit(HitTarget { rect, event: None, control_id: Some(plan.kind.dialog_id().to_string()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
                    cursor.rect = Some(rect);
                } else if matches!(self.overlay_state, OverlayState::Dropdown(_)) {
                    let rect = Rect::new(width * 0.5 - 200.0, theme.navbar_height + 8.0, 400.0, height * 0.55);
                    open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Menu));
                    cursor.rect = Some(rect);
                }
                cursor.phase = 1;
            }
            1 => {
                if let Some(plan) = self.command_palette_plan(theme, width, height) {
                    let Some(entry) = plan.entries.get(cursor.item) else {
                        close_chrome_overlay_glass_content(cursor, overlay);
                        cursor.item = 0;
                        cursor.group_phase = 0;
                        cursor.rect = None;
                        cursor.phase = 3;
                        return false;
                    };
                    let result = match entry {
                        ShellPalettePlanEntry::Input { rect, query, placeholder } => {
                            let text = if query.is_empty() { placeholder.as_str() } else { query.as_str() };
                            let icon_size = theme.gap_standard * 5.0;
                            let icon_x = rect.x + theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING;
                            let text_x = icon_x + icon_size + theme.gap_standard;
                            match cursor.group_phase {
                                0 => {
                                    chrome_icon(overlay, icons, "search", icon_x, rect.y + (rect.h - icon_size) * 0.5, icon_size, theme.text_muted);
                                    cursor.group_phase = 1;
                                    false
                                }
                                1 => match chrome_text_complete_step(
                                    overlay,
                                    atlas,
                                    text,
                                    text_x,
                                    rect.y + (rect.h + theme.font_size_body) * 0.5 - 1.0,
                                    (rect.x + rect.w - theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING - text_x).max(1.0),
                                    theme.font_size_body,
                                    if query.is_empty() { theme.text_muted } else { theme.text },
                                    &mut cursor.glyph,
                                ) {
                                    Ok(false) => false,
                                    Ok(true) => {
                                        cursor.group_phase = 2;
                                        false
                                    }
                                    Err(()) => {
                                        self.error = Some("Shell palette input exceeded the retained glyph boundary".to_string());
                                        cursor.group_phase = 2;
                                        cursor.glyph.reset();
                                        false
                                    }
                                },
                                2 => {
                                    overlay.push_solid([rect.x, rect.y + rect.h - theme.stroke_hairline, rect.w, theme.stroke_hairline], theme.border_normal);
                                    cursor.group_phase = 3;
                                    false
                                }
                                3 => {
                                    let title = shell_chrome_string(if plan.kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, self.locale_id == "de");
                                    note_chrome_control_name(plan.kind.input_id(), Some(title));
                                    input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(plan.kind.input_id().to_string()), kind: HitKind::Input, drag_axis: None, drag_data: None });
                                    true
                                }
                                _ => true,
                            }
                        }
                        ShellPalettePlanEntry::Group { rect, label } => match chrome_text_complete_step(
                            overlay,
                            atlas,
                            label,
                            rect.x + theme.padding_standard,
                            rect.y + theme.gap_standard * 2.0 + theme.font_size_small,
                            (rect.w - theme.padding_standard * 2.0).max(1.0),
                            theme.font_size_small,
                            theme.text_muted,
                            &mut cursor.glyph,
                        ) {
                            Ok(false) => false,
                            Ok(true) => true,
                            Err(()) => {
                                self.error = Some("Shell palette group exceeded the retained glyph boundary".to_string());
                                cursor.glyph.reset();
                                true
                            }
                        },
                        ShellPalettePlanEntry::Row { rect, index, item_id: _, label, description, selected } => {
                            let control_id = plan.kind.control_id(*index);
                            let hovered = input.hovered_id.as_deref() == Some(control_id.as_str());
                            match cursor.group_phase {
                                0 => {
                                    let background = chrome_item_bg(theme, *selected, hovered);
                                    if background.a > 0.0 {
                                        overlay.push_solid([rect.x, rect.y, rect.w, rect.h - theme.gap_standard], background);
                                    }
                                    cursor.group_phase = 1;
                                    false
                                }
                                1 => match chrome_text_complete_step(
                                    overlay,
                                    atlas,
                                    label,
                                    rect.x + theme.padding_standard,
                                    rect.y + theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING + theme.font_size_body,
                                    (rect.w - theme.padding_standard * 2.0).max(1.0),
                                    theme.font_size_body,
                                    chrome_item_text(theme, *selected, hovered),
                                    &mut cursor.glyph,
                                ) {
                                    Ok(false) => false,
                                    Ok(true) => {
                                        cursor.group_phase = 2;
                                        false
                                    }
                                    Err(()) => {
                                        self.error = Some("Shell palette row exceeded the retained glyph boundary".to_string());
                                        cursor.group_phase = 2;
                                        cursor.glyph.reset();
                                        false
                                    }
                                },
                                2 if description.is_some() => match chrome_text_complete_step(
                                    overlay,
                                    atlas,
                                    description.as_deref().unwrap_or_default(),
                                    rect.x + theme.padding_standard,
                                    rect.y + theme.gap_standard * SHELL_PALETTE_TINY_UI_SPACING + theme.font_size_body * 1.5 + theme.font_size_small,
                                    (rect.w - theme.padding_standard * 2.0).max(1.0),
                                    theme.font_size_small,
                                    theme.text_muted,
                                    &mut cursor.glyph,
                                ) {
                                    Ok(false) => false,
                                    Ok(true) => {
                                        cursor.group_phase = 3;
                                        false
                                    }
                                    Err(()) => {
                                        self.error = Some("Shell palette row description exceeded the retained glyph boundary".to_string());
                                        cursor.group_phase = 3;
                                        cursor.glyph.reset();
                                        false
                                    }
                                },
                                2 => {
                                    cursor.group_phase = 3;
                                    false
                                }
                                3 => {
                                    note_chrome_control_name(&control_id, Some(label));
                                    input.register_hit(HitTarget { rect: Rect::new(rect.x, rect.y, rect.w, rect.h - theme.gap_standard), event: None, control_id: Some(control_id), kind: HitKind::DropdownItem, drag_axis: None, drag_data: None });
                                    true
                                }
                                _ => true,
                            }
                        }
                        ShellPalettePlanEntry::Empty { rect, label } => {
                            let text_width = atlas.measure_text(label, theme.font_size_body).0;
                            match chrome_text_complete_step(
                                overlay,
                                atlas,
                                label,
                                rect.x + ((rect.w - text_width) * 0.5).max(theme.padding_standard),
                                rect.y + (rect.h + theme.font_size_body) * 0.5 - 1.0,
                                rect.w.max(1.0),
                                theme.font_size_body,
                                theme.text_muted,
                                &mut cursor.glyph,
                            ) {
                                Ok(false) => false,
                                Ok(true) => true,
                                Err(()) => {
                                    self.error = Some("Shell palette empty state exceeded the retained glyph boundary".to_string());
                                    cursor.glyph.reset();
                                    true
                                }
                            }
                        }
                        ShellPalettePlanEntry::Close { rect, label } => {
                            let icon_size = (rect.h - theme.gap_standard * 4.0).max(theme.font_size_small);
                            chrome_icon(overlay, icons, "close", rect.x + (rect.w - icon_size) * 0.5, rect.y + (rect.h - icon_size) * 0.5, icon_size, theme.text_muted);
                            note_chrome_control_name(plan.kind.close_id(), Some(label));
                            input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(plan.kind.close_id().to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                            true
                        }
                    };
                    if result {
                        cursor.item += 1;
                        cursor.group_phase = 0;
                        cursor.glyph.reset();
                    }
                    return false;
                }
                let is_de = self.locale_id == "de";
                let label = match &self.overlay_state {
                    OverlayState::Dropdown(id) if id == "example" => Some((shell_chrome_string("example.overlay.title", is_de), "")),
                    _ => None,
                };
                if let (Some(rect), Some((title, query))) = (cursor.rect, label) {
                    let text = match cursor.item {
                        0 => Some((title, rect.y + theme.padding_standard + theme.font_size_body, theme.font_size_body, theme.text)),
                        1 if !query.is_empty() => Some((query, rect.y + theme.padding_standard * 2.0 + theme.font_size_body * 2.0, theme.font_size_small, theme.text_muted)),
                        1 => {
                            cursor.item = 2;
                            return false;
                        }
                        _ => None,
                    };
                    if let Some((text, y, size, color)) = text {
                        match chrome_text_complete_step(overlay, atlas, text, rect.x + theme.padding_standard, y, (rect.w - theme.padding_standard * 2.0).max(1.0), size, color, &mut cursor.glyph) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                self.error = Some("Shell overlay text exceeded the retained glyph boundary".to_string());
                                cursor.glyph.reset();
                            }
                        }
                        cursor.item += 1;
                        return false;
                    }
                }
                cursor.item = 0;
                cursor.phase = 2;
            }
            2 => {
                let rows = if matches!(&self.overlay_state, OverlayState::Dropdown(id) if id == "example") { self.session_example_rows() } else { Vec::new() };
                let row_h = theme.control_height;
                let placed = cursor.rect.and_then(|panel| {
                    let row = rows.get(cursor.item)?;
                    let rect = Rect::new(panel.x + theme.gap_standard, panel.y + theme.padding_standard * 2.0 + theme.font_size_body * 2.0 + cursor.item as f32 * row_h, (panel.w - theme.gap_standard * 2.0).max(1.0), row_h);
                    (rect.y + rect.h <= panel.y + panel.h).then_some((row, rect))
                });
                let Some((row, rect)) = placed else {
                    close_chrome_overlay_glass_content(cursor, overlay);
                    cursor.item = 0;
                    cursor.rect = None;
                    cursor.phase = 3;
                    return false;
                };
                let item = ChromeGroupItem { control_id: row.control_id.as_str(), icon_id: Some("file"), label: Some(row.label.as_str()), active: row.selected, disabled: false, kind: HitKind::DropdownItem };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, overlay, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell example row exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            3 => {
                if !self.render_context_menu_step(cursor, overlay, atlas, icons, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 4;
            }
            4 => {
                if !self.render_chrome_tooltip_step(cursor, overlay, atlas, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 5;
            }
            5 => {
                if !self.render_chrome_dialog_step(cursor, overlay, atlas, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 6;
            }
            6 => {
                if !self.render_chrome_tour_step(cursor, overlay, atlas, icons, input, theme, width, height) {
                    return false;
                }
                cursor.item = 0;
                cursor.scalar = 0;
                cursor.phase = 7;
            }
            7 => {
                let pills = self.surface_status_pills(theme);
                let Some((pill, rect)) = pills.get(cursor.item) else {
                    close_chrome_overlay_glass_content(cursor, overlay);
                    cursor.item = 0;
                    cursor.scalar = 0;
                    cursor.phase = 8;
                    return false;
                };
                let (pill, rect) = (pill.clone(), *rect);
                match cursor.scalar {
                    0 => {
                        open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Pane));
                        cursor.scalar = 1;
                        return false;
                    }
                    1 => {
                        let track = Rect::new(rect.x + theme.padding_standard, rect.y + rect.h - theme.stroke_hairline * 3.0, (rect.w - theme.padding_standard * 2.0).max(0.0), theme.stroke_hairline * 2.0);
                        overlay.push_solid([track.x, track.y, track.w, track.h], theme.text_muted);
                        if let Some(ratio) = pill.ratio {
                            overlay.push_solid([track.x, track.y, track.w * ratio.clamp(0.0, 1.0), track.h], theme.accent);
                        }
                        cursor.scalar = 2;
                        return false;
                    }
                    _ => {}
                }
                let label = pill.label();
                match chrome_text_complete_step(
                    overlay,
                    atlas,
                    &label,
                    rect.x + theme.padding_standard,
                    rect.y + (rect.h + theme.font_size_small) * 0.5 - 2.0,
                    (rect.w - theme.padding_standard * 2.0).max(1.0),
                    theme.font_size_small,
                    if pill.computing { theme.text } else { theme.text_muted },
                    &mut cursor.glyph,
                ) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell world3d status pill exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                close_chrome_overlay_glass_content(cursor, overlay);
                cursor.item += 1;
                cursor.scalar = 0;
                return false;
            }
            8 => {
                let items = self.board_presence_paint_items(theme);
                let Some(item) = items.get(cursor.item).cloned() else {
                    cursor.item = 0;
                    cursor.scalar = 0;
                    cursor.phase = 9;
                    return false;
                };
                let (bounds, color, chip, chip_rect) = match &item {
                    ShellBoardPresencePaint::Cursor { bounds, peer } => {
                        let color = theme.presence_color(peer.color);
                        (*bounds, color, peer.chip.clone(), Rect::new(peer.at[0] + 10.0, peer.at[1] - 4.0, board_presence_chip_width(theme, &peer.chip), theme.font_size_small + 4.0))
                    }
                    ShellBoardPresencePaint::Mark { bounds, rect, mark } => (*bounds, theme.presence_color(mark.color), mark.chip.clone(), *rect),
                };
                if cursor.scalar == 0 {
                    overlay.push_scissor(bounds);
                    if let ShellBoardPresencePaint::Cursor { peer, .. } = &item {
                        let [x, y, w, h] = peer.viewport;
                        let edge = theme.stroke_hairline * 1.5;
                        let frame = color.with_alpha(0.55);
                        overlay.push_solid([x, y, w, edge], frame);
                        overlay.push_solid([x, y + h - edge, w, edge], frame);
                        overlay.push_solid([x, y, edge, h], frame);
                        overlay.push_solid([x + w - edge, y, edge, h], frame);
                        overlay.push_rounded([peer.at[0] - 2.0, peer.at[1] - 2.0, 10.0, 10.0], color, 5.0);
                    }
                    overlay.push_rounded([chip_rect.x, chip_rect.y, chip_rect.w, chip_rect.h], color, 3.0);
                    cursor.scalar = 1;
                    return false;
                }
                match chrome_text_complete_step(overlay, atlas, &chip, chip_rect.x + 4.0, chip_rect.y + chip_rect.h - 3.0, (chip_rect.w - 8.0).max(1.0), theme.font_size_small, theme.active_foreground, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell board presence chip exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                overlay.pop_scissor();
                cursor.item += 1;
                cursor.scalar = 0;
                return false;
            }
            9 => {
                let controls = self.surface_overlay_controls(theme);
                let Some((control, anchor)) = controls.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.phase = 10;
                    return false;
                };
                let label = self.chrome_control_label(&control.control_id, &control.label);
                let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
                let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                    self.error = Some("Shell surface control exceeded the retained chrome boundary".to_string());
                    cursor.item += 1;
                    return false;
                };
                let rect = Rect::new(anchor[0], anchor[1], item_w.max(theme.control_height), theme.control_height);
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, overlay, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.error = Some("Shell surface control exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            10 => {
                if let Some(rect) = self.keyboard_focus_ring_rect(input) {
                    let edge = theme.stroke_hairline * 2.0;
                    let [x, y, w, h] = [rect.x - edge, rect.y - edge, rect.w + edge * 2.0, rect.h + edge * 2.0];
                    overlay.push_solid([x, y, w, edge], theme.focus_ring);
                    overlay.push_solid([x, y + h - edge, w, edge], theme.focus_ring);
                    overlay.push_solid([x, y, edge, h], theme.focus_ring);
                    overlay.push_solid([x + w - edge, y, edge, h], theme.focus_ring);
                }
                cursor.phase = 11;
                return false;
            }
            11 => return true,
            _ => return false,
        }
        false
    }

    /// 👥️🖼️ The board presence this frame paints, in paint order: every cursor (viewport, dot, chip),
    /// then every mark chip, stacked down the board's right edge by entity row like React's canvas marks.
    fn board_presence_paint_items(&self, theme: &Theme) -> Vec<ShellBoardPresencePaint> {
        let mut items = Vec::new();
        for (bounds, overlays) in self.board_peer_overlays() {
            items.extend(overlays.cursors.into_iter().map(|peer| ShellBoardPresencePaint::Cursor { bounds, peer }));
            items.extend(overlays.marks.into_iter().map(|mark| {
                let width = board_presence_chip_width(theme, &mark.chip);
                let rect = Rect::new(bounds.x + bounds.w - 8.0 - width, bounds.y + 8.0 + mark.row as f32 * 18.0, width, theme.font_size_small + 4.0);
                ShellBoardPresencePaint::Mark { bounds, rect, mark }
            }));
        }
        items
    }

    pub(crate) fn render_context_menu_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, viewport_w: f32, viewport_h: f32) -> bool {
        let Some(menu) = self.context_menu.as_ref() else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let row_h = theme.control_height;
        let menu_w = 240.0_f32.min(viewport_w.max(0.0));
        let menu_h = (menu.items.len() as f32 * row_h + 8.0).min((viewport_h - 8.0).max(row_h + 8.0));
        let rect = Rect::new(menu.x.clamp(0.0, (viewport_w - menu_w).max(0.0)), menu.y.clamp(0.0, (viewport_h - menu_h).max(0.0)), menu_w, menu_h);
        match cursor.scalar {
            0 => {
                open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Menu));
                cursor.rect = Some(rect);
                cursor.scalar = 1;
            }
            1 => {
                overlay.push_scissor(rect);
                cursor.scalar = 2;
            }
            2 => {
                let Some(item) = menu.items.get(cursor.item) else {
                    cursor.scalar = 3;
                    return false;
                };
                let row = Rect::new(rect.x + 4.0, rect.y + 4.0 + cursor.item as f32 * row_h - menu.scroll_offset, rect.w - 8.0, row_h);
                if row.y + row.h > rect.y && row.y < rect.y + rect.h {
                    if item.separator {
                        overlay.push_solid([row.x + 4.0, row.y + row.h * 0.5, row.w - 8.0, theme.stroke_hairline], theme.text_muted);
                    } else {
                        if !cursor.flag {
                            let active = menu.active.first().copied() == Some(cursor.item);
                            overlay.push_rounded([row.x, row.y, row.w, row.h], if active { theme.accent } else { theme.button }, theme.border_radius);
                            let icon = item.icon.as_deref().unwrap_or("circle-dot");
                            chrome_icon(overlay, icons, icon, row.x + 8.0, row.y + (row.h - theme.font_size_body) * 0.5, theme.font_size_body, if item.disabled { theme.text_muted } else { theme.text });
                            cursor.flag = true;
                            return false;
                        }
                        match chrome_text_complete_step(
                            overlay,
                            atlas,
                            &item.painted_label(),
                            row.x + theme.font_size_body + theme.gap_standard * 2.0,
                            row.y + (row.h + theme.font_size_small) * 0.5 - 1.0,
                            (row.w - theme.font_size_body - theme.gap_standard * 3.0).max(1.0),
                            theme.font_size_small,
                            if item.disabled { theme.text_muted } else { theme.text },
                            &mut cursor.glyph,
                        ) {
                            Ok(false) => return false,
                            Ok(true) => {}
                            Err(()) => {
                                cursor.fault = true;
                                cursor.glyph.reset();
                            }
                        }
                        cursor.flag = false;
                        item.register_hit(row, input);
                    }
                }
                cursor.item += 1;
            }
            3 => {
                overlay.pop_scissor();
                cursor.scalar = 4;
            }
            4 => {
                close_chrome_overlay_glass_content(cursor, overlay);
                if cursor.fault {
                    self.error = Some("Shell context-menu text exceeded the retained glyph boundary".to_string());
                    cursor.fault = false;
                }
                return true;
            }
            _ => return false,
        }
        false
    }

    /// 💡️ `tooltips: "none"` (the shipped `compact` driver) shows no hover tooltip at all —
    /// React's `useControlTooltipText` returns `undefined` before it composes any text, so the
    /// hint never mounts. The armed hover is dropped too, so flipping the driver back does not
    /// reveal a tooltip whose dwell ran while it was suppressed.
    fn render_chrome_tooltip_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if self.chrome_build.dialog_open() {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        if !self.chrome_build.tooltip_shows(false) {
            self.chrome_build.tooltip_hover = None;
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        match cursor.scalar {
            0 => {
                let Some(id) = input.hovered_id.as_deref() else {
                    self.chrome_build.tooltip_hover = None;
                    return true;
                };
                let Some(title) = self.chrome_build.tooltip_titles.get(id) else {
                    self.chrome_build.tooltip_hover = None;
                    return true;
                };
                let now = chrome_now_ms();
                let restart = self.chrome_build.tooltip_hover.as_ref().is_none_or(|hover| hover.control_id != id);
                if restart {
                    self.chrome_build.tooltip_hover = Some(ChromeTooltipHover { control_id: id.into(), anchor_x: input.pointer_x, anchor_y: input.pointer_y, started_ms: now });
                    return true;
                }
                let Some(hover) = self.chrome_build.tooltip_hover.as_ref() else { return true };
                if !chrome_tooltip_ready(hover, now) {
                    return true;
                }
                let Some(text) = UiText::try_from_str(title) else { return true };
                let padding = theme.padding_standard * 0.5;
                let text_w = atlas.measure_text(text.as_str(), theme.font_size_small).0;
                let text_h = theme.font_size_small * 1.25;
                cursor.rect =
                    Some(Rect::new((hover.anchor_x + 12.0).clamp(0.0, (width - text_w - padding * 2.0).max(0.0)), (hover.anchor_y + 18.0).clamp(0.0, (height - text_h - padding * 2.0).max(0.0)), text_w + padding * 2.0, text_h + padding * 2.0));
                cursor.window = Some(text);
                cursor.scalar = 1;
            }
            1 => {
                let Some(rect) = cursor.rect else { return true };
                open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Menu));
                cursor.scalar = 2;
            }
            2 => {
                if let (Some(rect), Some(text)) = (cursor.rect, cursor.window.as_ref()) {
                    let padding = theme.padding_standard * 0.5;
                    match chrome_text_complete_step(overlay, atlas, text.as_str(), rect.x + padding, rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0, (rect.w - padding * 2.0).max(1.0), theme.font_size_small, theme.text, &mut cursor.glyph) {
                        Ok(false) => return false,
                        Ok(true) => {}
                        Err(()) => {
                            self.error = Some("Shell tooltip text exceeded the retained glyph boundary".to_string());
                            cursor.glyph.reset();
                        }
                    }
                }
                cursor.scalar = 3;
            }
            3 => {
                close_chrome_overlay_glass_content(cursor, overlay);
                return true;
            }
            _ => return false,
        }
        false
    }

    /// 💬️ Paints the agent chat panel's header — title on the left, `🚦️AgentPresence` dot + status on
    /// the right, a hairline under it — over whichever anchor currently shows the `framework.chat`
    /// tab. Packet W1j; React mounts the same pair as `AgentChatPanel`'s own header row. The
    /// transcript below it is still host-gapped (see `💬️AgentChatPanel/🎯️targets/🧊️wgpu`).
    pub(crate) fn render_agent_chat_header_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, body: Rect) -> bool {
        use crate::agent_chat_panel as chat;
        let Some(anchor) = PanelAnchor::ALL.into_iter().find(|anchor| self.anchor_open(*anchor) && self.anchor_state(*anchor).active_tab() == Some(chat::AGENT_CHAT_PANEL_ID)) else {
            return true;
        };
        let panel = self.anchor_rect(anchor, body, theme);
        let content = self.anchor_content_rect(anchor, panel, theme);
        let locale = self.active_locale();
        let plan = chat::plan_agent_chat_header(content, self.chrome_build.agent.status, &self.chrome_build.agent.presence, theme, locale);
        let baseline = plan.header.y + (plan.header.h + theme.font_size_small) * 0.5 - 1.0;
        match cursor.scalar {
            0 => overlay.push_solid([plan.header.x, plan.header.y + plan.header.h - theme.stroke_hairline, plan.header.w, theme.stroke_hairline], theme.border_normal),
            1 => overlay.push_rounded([plan.dot.x, plan.dot.y, plan.dot.w, plan.dot.h], plan.dot_color, plan.dot.w * 0.5),
            2..=4 => {
                let (value, x, max_w, size, color, y) = match cursor.scalar {
                    2 => (plan.title.clone(), plan.header.x + theme.padding_standard, (plan.dot.x - plan.header.x - theme.padding_standard * 2.0).max(1.0), theme.font_size_body, theme.text, baseline),
                    3 => {
                        let x = plan.dot.x + crate::agent_presence::AGENT_PRESENCE_DOT_PX + crate::agent_presence::AGENT_PRESENCE_GAP_PX;
                        (plan.status_text.clone(), x, (plan.header.x + plan.header.w - theme.padding_standard - x).max(1.0), theme.font_size_small, theme.text_muted, baseline)
                    }
                    _ => (
                        if self.chrome_build.agent.conversation.is_empty() { chat::agent_chat_empty_text(locale) } else { String::new() },
                        content.x + theme.padding_standard,
                        (content.w - theme.padding_standard * 2.0).max(1.0),
                        theme.font_size_small,
                        theme.text_muted,
                        plan.header.y + plan.header.h + theme.gap_standard + theme.font_size_small,
                    ),
                };
                match chrome_text_complete_step(overlay, atlas, &value, x, y, max_w, size, color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell agent-chat header text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            5 => {
                let hit = Rect::new(plan.dot.x, plan.header.y, (plan.header.x + plan.header.w - plan.dot.x).max(1.0), plan.header.h);
                self.chrome_build.tooltip_titles.insert(crate::agent_presence::AGENT_PRESENCE_CONTROL_ID.to_string(), crate::agent_presence::agent_presence_status_label(locale));
                input.register_hit(HitTarget { rect: hit, event: None, control_id: Some(crate::agent_presence::AGENT_PRESENCE_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🧯️ Paints the one live transient notice, top-centre at `navbar_height + 8px`, and expires it
    /// at the 4 s deadline. Packet W1j — React's only non-fatal surfacing channel, ported.
    pub(crate) fn render_transient_notice_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        if !self.chrome_build.expire_transient_notice(chrome_now_ms()) {
            return true;
        }
        let is_de = self.locale_id == "de";
        let close_label = shell_chrome_string("common.close", is_de);
        let Some(notice) = self.chrome_build.transient_notice.clone() else { return true };
        let banner = transient_notice_rect(&notice, close_label, width, theme);
        let close = transient_notice_close_rect(banner, close_label, theme);
        let (border, fill, text_color) = transient_notice_tone(notice.severity, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([banner.x, banner.y, banner.w, banner.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [banner.x, banner.y, banner.w, hair],
                    2 => [banner.x, banner.y + banner.h - hair, banner.w, hair],
                    3 => [banner.x, banner.y, hair, banner.h],
                    _ => [banner.x + banner.w - hair, banner.y, hair, banner.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_w, color) =
                    if cursor.scalar == 5 { (notice.message.as_str(), banner.x + theme.padding_standard, (close.x - banner.x - theme.padding_standard * 2.0).max(1.0), text_color) } else { (close_label, close.x, close.w.max(1.0), text_color) };
                let baseline = banner.y + (banner.h + theme.font_size_small) * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, value, x, baseline, max_w, theme.font_size_small, color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell transient notice text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: close, event: None, control_id: Some(TRANSIENT_NOTICE_CLOSE_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome_build.clicked_this_frame && close.contains(input.pointer_x, input.pointer_y) {
                    self.chrome_build.dismiss_transient_notice();
                }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🎬️ Paints the lazy plugin-install band: which plugin, which phase, and the control that stops
    /// it. A settled `Cancelled`/`Failed` record stays until the user clears it, because a refused
    /// open that vanished after 4 s is exactly the "explains itself" property
    /// [`ShellPluginInstallPhase::Failed`] carries its message for. A running install's control is a
    /// real cancel ([`ShellState::cancel_plugin_install`]), not a dismiss.
    pub(crate) fn render_plugin_install_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(install) = self.plugin_install.as_ref() else { return true };
        let is_de = self.locale_id == "de";
        let running = matches!(install.phase, ShellPluginInstallPhase::Resolving | ShellPluginInstallPhase::Loading);
        let action_label = shell_chrome_string(if running { "plugin.install.cancel" } else { "common.close" }, is_de);
        let message = plugin_install_banner_text(install, is_de);
        let (border, fill, text_color) = plugin_install_tone(&install.phase, theme);
        let band = plugin_install_rect(&message, action_label, width, theme);
        let action = plugin_install_action_rect(band, action_label, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_w) = if cursor.scalar == 5 { (message.as_str(), band.x + theme.padding_standard, (action.x - band.x - theme.padding_standard * 2.0).max(1.0)) } else { (action_label, action.x, action.w.max(1.0)) };
                let baseline = band.y + (band.h + theme.font_size_small) * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, value, x, baseline, max_w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell plugin install band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: action, event: None, control_id: Some(PLUGIN_INSTALL_CANCEL_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) {
                    self.cancel_plugin_install();
                }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🚪️ Paints the frame-pumped document open's band: which app, which phase, the step and the time
    /// it has taken, and the control that stops it. A settled `Cancelled`/`Failed` record stays until the
    /// user closes it, exactly like the plugin-install band above it.
    pub(crate) fn render_document_opening_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(opening) = self.document_opening.as_ref() else { return true };
        let is_de = self.locale_id == "de";
        let action_label = shell_chrome_string(if opening.running() { "document.open.cancel" } else { "common.close" }, is_de);
        let message = document_opening_banner_text(opening, chrome_now_ms(), is_de);
        let (border, fill, text_color) = document_opening_tone(&opening.phase, theme);
        let band = document_opening_rect(&message, action_label, width, theme);
        let action = plugin_install_action_rect(band, action_label, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => overlay.push_rounded([band.x, band.y, band.w, band.h], fill, theme.border_radius),
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [band.x, band.y, band.w, hair],
                    2 => [band.x, band.y + band.h - hair, band.w, hair],
                    3 => [band.x, band.y, hair, band.h],
                    _ => [band.x + band.w - hair, band.y, hair, band.h],
                };
                overlay.push_solid(edge, border);
            }
            5 | 6 => {
                let (value, x, max_w) = if cursor.scalar == 5 { (message.as_str(), band.x + theme.padding_standard, (action.x - band.x - theme.padding_standard * 2.0).max(1.0)) } else { (action_label, action.x, action.w.max(1.0)) };
                let baseline = band.y + (band.h + theme.font_size_small) * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, value, x, baseline, max_w, theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.error = Some("Shell document open band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: action, event: None, control_id: Some(DOCUMENT_OPEN_CANCEL_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) {
                    self.cancel_document_opening();
                }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🛂️ Resolves this frame's administration sheet from the retained operation alone — packet
    /// W15e. `None` means no operation is live, which is exactly when React mounts nothing.
    ///
    /// ⚖️ Every row comes from [`shell_space_administration_controls`], the SAME builder the native
    /// shell already renders from and the one whose `enabled` flags are derived solely from the hub's
    /// own capability answer. The chrome adds no authority of its own.
    ///
    /// 🧾️ The status row is the sheet's own status LINE, not a roster row — React paints it as the
    /// `role="status"` paragraph above the rosters, so it is lifted out of the control list here.
    ///
    /// 📋️ The invite-link control is deliberately WITHHELD by this packet, on both targets.
    /// Acknowledging the one-shot capability ERASES it in the same turn, so a control that
    /// acknowledges without delivering destroys the invitation outright — and neither target
    /// can deliver it yet: the frame Worker owns no `navigator.clipboard` (`web_sys::window()`
    /// is `None` there, the defect the file and preference doors exist for), and the native
    /// clipboard is an `InteractiveJob` (`ui_host::ClipboardIoJob`) with no shell-side runner.
    /// Offering a control that silently loses an invitation is strictly worse than not
    /// offering it; the clipboard door is named as a hand-off in this packet's report.
    pub(crate) fn space_administration_plan(&self) -> Option<crate::space_administration::SpaceAdministrationPlan> {
        use crate::space_administration as pane;
        let operation = self.space_administration.as_ref()?;
        let german = self.locale_id == "de";
        let locale = self.active_locale();
        let controls = shell_space_administration_controls(operation, german);
        let notice = match operation.page() {
            Some(DirectorySpaceAdministrationPageV1::Public { .. }) => Some(pane::space_administration_public_notice(locale).to_string()),
            Some(DirectorySpaceAdministrationPageV1::Member { .. }) => Some(pane::space_administration_spectator_notice(locale).to_string()),
            _ => None,
        };
        let mut rows = Vec::new();
        let mut status = shell_space_administration_status(operation.phase(), german).to_string();
        for control in controls {
            if control.control_id == "os.space-administration.status" {
                status = control.label.to_string();
                continue;
            }
            if control.control_id == "os.space-administration.invite.copy" {
                continue;
            }
            rows.push(pane::SpaceAdministrationRow { control_id: control.control_id, enabled: control.enabled, label: control.label.to_string(), value: control.value });
        }
        Some(pane::SpaceAdministrationPlan { notice, rows, space_id: operation.space_id().to_string(), status, title: shell_space_administration_label("title", german).to_string() })
    }
}
//#endregion ShellChrome
