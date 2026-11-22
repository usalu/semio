//! 🪧️ `shell-view-chrome-overlays` staging of `overlay-steps`: the tutorial bar, context menu, tooltip, agent chat header, transient notice, plugin install, document opening and status stack steps of the chrome, split from `shell-view-chrome` so that crate stays inside its source budget. See [`super`].

use super::*;

//#region ShellChrome
impl<'ctx> ChromeOverlaysCtx<'ctx> {
    /// 📖️ The chapter ticks of the running tutorial as `(id, atMs, localized title)`, ascending by `at`.
    fn tutorial_chapter_ticks(&self, duration_ms: u64) -> Vec<(String, u64, String)> {
        let (terminology, locale) = (self.prefs.active_terminology(), self.prefs.active_locale());
        self.tutorial.tutorial.as_ref().filter(|_| duration_ms > 0).map(|runtime| runtime.definition.chapters.iter().map(|chapter| (chapter.id.clone(), chapter.at, chapter.title.resolve(terminology, locale).to_string())).collect()).unwrap_or_default()
    }

    pub fn render_tutorial_bar_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(runtime) = self.tutorial.tutorial.as_ref() else { return true };
        let mode = runtime.mode;
        let playhead_ms = runtime.playhead_ms;
        let duration_ms = runtime.definition.duration_ms;
        let rate = runtime.rate;
        let muted = runtime.muted;
        let captions_on = runtime.captions_on;
        let is_de = self.prefs.locale_id == "de";
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
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell tutorial play item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.as_window_actions().tutorial_toggle_play_pause();
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
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell tutorial stop item exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    self.as_window_actions().tutorial_stop();
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
                        self.chrome.error = Some("Shell tutorial-bar text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.x += theme.font_size_small * 13.0 + theme.gap_standard * 2.0;
                cursor.phase = 22;
            }
            22 | 23 => {
                let add_chapter = cursor.phase == 22;
                let recording = mode == TutorialMode::Recording;
                let (control_id, icon_id, label_key, kind) = if add_chapter { ("shell.tutorial.addChapter", "plus", "tutorial.addChapter", HitKind::Button) } else { ("shell.tutorial.record", "circle-dot", "tutorial.record", HitKind::Toggle) };
                let item = ChromeGroupItem { control_id, icon_id: Some(icon_id), label: Some(shell_chrome_string(label_key, is_de)), active: !add_chapter && recording, disabled: false, kind };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        self.chrome.error = Some("Shell tutorial record item exceeded the retained chrome boundary".to_string());
                        cursor.phase = 5;
                        return false;
                    };
                    cursor.right -= item_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell tutorial record item exceeded the retained glyph boundary".to_string()),
                }
                if !add_chapter {
                    with_chrome_control_names(|names| {
                        if let Some(presentation) = names.get_mut(control_id) {
                            presentation.role = Some("button");
                            presentation.pressed = Some(recording);
                        }
                    });
                }
                if self.chrome.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    if add_chapter {
                        self.as_window_actions().tutorial_add_chapter();
                    } else {
                        self.as_window_actions().tutorial_toggle_recording();
                        cursor.rect = None;
                        cursor.phase = 9;
                        return false;
                    }
                }
                cursor.rect = None;
                cursor.phase = if add_chapter { 23 } else if recording { 24 } else { 20 };
            }
            24 => {
                let label = shell_chrome_string("tutorial.recording", is_de);
                let text_w = atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard;
                if cursor.rect.is_none() {
                    cursor.right -= text_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, text_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match chrome_text_complete_step(draw, atlas, label, rect.x, btn_y + (btn_h + theme.font_size_small) * 0.5 - 1.0, rect.w, theme.font_size_small, theme.error, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell tutorial recording indicator exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.rect = None;
                cursor.phase = 5;
            }
            20 | 21 => {
                let captions = cursor.phase == 20;
                let (control_id, icon_id, active, label_key) = if captions { ("shell.tutorial.captions", "message-square", captions_on, "tutorial.captions") } else { ("shell.tutorial.mute", "x", muted, "tutorial.mute") };
                let item = ChromeGroupItem { control_id, icon_id: Some(icon_id), label: Some(shell_chrome_string(label_key, is_de)), active, disabled: false, kind: HitKind::Toggle };
                if cursor.rect.is_none() {
                    let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                        self.chrome.error = Some("Shell tutorial toggle exceeded the retained chrome boundary".to_string());
                        cursor.phase = 5;
                        return false;
                    };
                    cursor.right -= item_w;
                    cursor.rect = Some(Rect::new(cursor.right, btn_y, item_w, btn_h));
                }
                let Some(rect) = cursor.rect else { return false };
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell tutorial toggle exceeded the retained glyph boundary".to_string()),
                }
                with_chrome_control_names(|names| {
                    if let Some(presentation) = names.get_mut(control_id) {
                        presentation.role = Some("button");
                        presentation.pressed = Some(active);
                    }
                });
                if self.chrome.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    if let Some(runtime) = self.tutorial.tutorial.as_mut() {
                        if captions {
                            runtime.captions_on = !runtime.captions_on;
                        } else {
                            runtime.muted = !runtime.muted;
                        }
                    }
                }
                cursor.rect = None;
                cursor.phase = if captions { 21 } else { 5 };
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
                        self.chrome.error = Some("Shell tutorial rate exceeded the retained chrome boundary".to_string());
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
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell tutorial rate exceeded the retained glyph boundary".to_string()),
                }
                if self.chrome.chrome_build.clicked_this_frame && rect.contains(input.pointer_x, input.pointer_y) {
                    if let Some(runtime) = self.tutorial.tutorial.as_mut() {
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
                for (_, at_ms, _) in self.tutorial_chapter_ticks(duration_ms) {
                    let tick = tutorial_chapter_tick_rect(rect, at_ms, duration_ms, theme);
                    let hovered = tutorial_chapter_tick_hit_rect(tick).contains(input.pointer_x, input.pointer_y);
                    draw.push_rounded([tick.x, tick.y, tick.w, tick.h], theme.text.with_alpha(if hovered { 1.0 } else { 0.5 }), tick.w * 0.5);
                }
                cursor.phase = 8;
            }
            8 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = u16::MAX;
                    return false;
                };
                input.register_hit(HitTarget { rect, event: None, control_id: Some("shell.tutorial.scrubber".into()), kind: HitKind::Slider, drag_axis: None, drag_data: None });
                let mut chapter_seek = None;
                for (id, at_ms, title) in self.tutorial_chapter_ticks(duration_ms) {
                    let hit = tutorial_chapter_tick_hit_rect(tutorial_chapter_tick_rect(rect, at_ms, duration_ms, theme));
                    let control_id = format!("ui.tutorial.chapter.{id}");
                    note_chrome_control_name(&control_id, Some(title.as_str()));
                    input.register_hit(HitTarget { rect: hit, event: None, control_id: Some(control_id.into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                    if hit.contains(input.pointer_x, input.pointer_y) {
                        chapter_seek = Some(at_ms as f64);
                    }
                }
                if mode != TutorialMode::Recording {
                    if let Some(target_ms) = chapter_seek {
                        if self.chrome.chrome_build.clicked_this_frame {
                            self.as_window_actions().tutorial_seek(target_ms);
                        }
                    } else if input.pointer_down && rect.contains(input.pointer_x, input.pointer_y) {
                        self.as_window_actions().tutorial_seek(tutorial_scrub_target_ms(input.pointer_x, rect, duration_ms));
                    }
                }
                cursor.phase = 9;
            }
            9 => return true,
            _ => return false,
        }
        false
    }

    pub fn render_context_menu_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, viewport_w: f32, viewport_h: f32) -> bool {
        let Some(menu) = self.input.context_menu.as_ref() else {
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
                    self.chrome.error = Some("Shell context-menu text exceeded the retained glyph boundary".to_string());
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
    pub fn render_chrome_tooltip_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if self.chrome.chrome_build.dialog_open() {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        if !self.chrome.chrome_build.tooltip_shows(false) {
            self.chrome.chrome_build.tooltip_hover = None;
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        match cursor.scalar {
            0 => {
                let Some(id) = input.hovered_id.as_deref() else {
                    self.chrome.chrome_build.tooltip_hover = None;
                    return true;
                };
                let Some(title) = self.chrome.chrome_build.tooltip_titles.get(id) else {
                    self.chrome.chrome_build.tooltip_hover = None;
                    return true;
                };
                let now = chrome_now_ms();
                let restart = self.chrome.chrome_build.tooltip_hover.as_ref().is_none_or(|hover| hover.control_id != id);
                if restart {
                    self.chrome.chrome_build.tooltip_hover = Some(ChromeTooltipHover { control_id: id.into(), anchor_x: input.pointer_x, anchor_y: input.pointer_y, started_ms: now });
                    return true;
                }
                let Some(hover) = self.chrome.chrome_build.tooltip_hover.as_ref() else { return true };
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
                            self.chrome.error = Some("Shell tooltip text exceeded the retained glyph boundary".to_string());
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
    pub fn render_agent_chat_header_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, body: Rect) -> bool {
                use semio_renderer_wgpu_agent_elements::agent_chat_panel as chat;
        let Some(anchor) = PanelAnchor::ALL.into_iter().find(|anchor| self.dock.anchor_open(*anchor) && self.dock.anchor_state(*anchor).active_tab() == Some(chat::AGENT_CHAT_PANEL_ID)) else {
            return true;
        };
        let panel = self.as_chrome_view().anchor_rect(anchor, body, theme);
        let content = self.dock.anchor_content_rect(anchor, panel, theme);
        let locale = self.prefs.active_locale();
        let plan = chat::plan_agent_chat_header(content, self.chrome.chrome_build.agent.status, &self.chrome.chrome_build.agent.presence, theme, locale);
        let baseline = plan.header.y + (plan.header.h + theme.font_size_small) * 0.5 - 1.0;
        match cursor.scalar {
            0 => overlay.push_solid([plan.header.x, plan.header.y + plan.header.h - theme.stroke_hairline, plan.header.w, theme.stroke_hairline], theme.border_normal),
            1 => overlay.push_rounded([plan.dot.x, plan.dot.y, plan.dot.w, plan.dot.h], plan.dot_color, plan.dot.w * 0.5),
            2..=4 => {
                let (value, x, max_w, size, color, y) = match cursor.scalar {
                    2 => (plan.title.clone(), plan.header.x + theme.padding_standard, (plan.dot.x - plan.header.x - theme.padding_standard * 2.0).max(1.0), theme.font_size_body, theme.text, baseline),
                    3 => {
                        let x = plan.dot.x + semio_renderer_wgpu_agent_elements::agent_presence::AGENT_PRESENCE_DOT_PX + semio_renderer_wgpu_agent_elements::agent_presence::AGENT_PRESENCE_GAP_PX;
                        (plan.status_text.clone(), x, (plan.header.x + plan.header.w - theme.padding_standard - x).max(1.0), theme.font_size_small, theme.text_muted, baseline)
                    }
                    _ => (
                        if self.chrome.chrome_build.agent.conversation.is_empty() { chat::agent_chat_empty_text(locale) } else { String::new() },
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
                        self.chrome.error = Some("Shell agent-chat header text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            5 => {
                let hit = Rect::new(plan.dot.x, plan.header.y, (plan.header.x + plan.header.w - plan.dot.x).max(1.0), plan.header.h);
                self.chrome.chrome_build.tooltip_titles.insert(semio_renderer_wgpu_agent_elements::agent_presence::AGENT_PRESENCE_CONTROL_ID.to_string(), semio_renderer_wgpu_agent_elements::agent_presence::agent_presence_status_label(locale));
                input.register_hit(HitTarget { rect: hit, event: None, control_id: Some(semio_renderer_wgpu_agent_elements::agent_presence::AGENT_PRESENCE_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None });
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🧯️ Paints the one live transient notice, top-centre at `navbar_height + 8px`, and expires it
    /// at the 4 s deadline. Packet W1j — React's only non-fatal surfacing channel, ported.
    pub fn render_transient_notice_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        if !self.chrome.chrome_build.expire_transient_notice(chrome_now_ms()) {
            return true;
        }
        let is_de = self.prefs.locale_id == "de";
        let close_label = shell_chrome_string("common.close", is_de);
        let Some(notice) = self.chrome.chrome_build.transient_notice.clone() else { return true };
        let layout = transient_notice_layout(&notice, close_label, width, theme);
        let banner = layout.rect;
        let close = transient_notice_close_rect(banner, close_label, theme);
        let (border, fill, text_color) = transient_notice_tone(notice.severity, theme);
        let hair = theme.stroke_hairline;
        match cursor.scalar {
            0 => {
                cursor.item = 0;
                overlay.push_rounded([banner.x, banner.y, banner.w, banner.h], fill, theme.border_radius)
            }
            1..=4 => {
                let edge = match cursor.scalar {
                    1 => [banner.x, banner.y, banner.w, hair],
                    2 => [banner.x, banner.y + banner.h - hair, banner.w, hair],
                    3 => [banner.x, banner.y, hair, banner.h],
                    _ => [banner.x + banner.w - hair, banner.y, hair, banner.h],
                };
                overlay.push_solid(edge, border);
            }
            5 => {
                let Some(line) = layout.lines.get(cursor.item) else {
                    cursor.scalar += 1;
                    return false;
                };
                let baseline = banner.y + theme.padding_standard + (cursor.item as f32 + 0.5) * layout.line_h + theme.font_size_small * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, line, banner.x + theme.padding_standard * 2.0, baseline, (close.x - banner.x - theme.padding_standard * 3.0).max(1.0), theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell transient notice text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                cursor.item += 1;
                return false;
            }
            6 => {
                let baseline = banner.y + theme.padding_standard + 0.5 * layout.line_h + theme.font_size_small * 0.5 - 1.0;
                match chrome_text_complete_step(overlay, atlas, close_label, close.x, baseline, close.w.max(1.0), theme.font_size_small, text_color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        self.chrome.error = Some("Shell transient notice text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: close, event: None, control_id: Some(TRANSIENT_NOTICE_CLOSE_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome.chrome_build.clicked_this_frame && close.contains(input.pointer_x, input.pointer_y) {
                    self.chrome.chrome_build.dismiss_transient_notice();
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
    pub fn render_plugin_install_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(install) = self.plugins.plugin_install.as_ref() else { return true };
        let is_de = self.prefs.locale_id == "de";
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
                        self.chrome.error = Some("Shell plugin install band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: action, event: None, control_id: Some(PLUGIN_INSTALL_CANCEL_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) {
                    self.plugins.cancel_plugin_install();
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
    pub fn render_document_opening_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32) -> bool {
        let Some(opening) = self.opening.document_opening.as_ref().filter(|opening| !matches!(opening.phase, ShellDocumentOpenPhase::Resolving)) else { return true };
        let is_de = self.prefs.locale_id == "de";
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
                        self.chrome.error = Some("Shell document open band text exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
            }
            7 => input.register_hit(HitTarget { rect: action, event: None, control_id: Some(DOCUMENT_OPEN_CANCEL_CONTROL_ID.to_string()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            8 => {
                if self.chrome.chrome_build.clicked_this_frame && action.contains(input.pointer_x, input.pointer_y) {
                    self.opening.cancel_document_opening();
                }
            }
            _ => return true,
        }
        cursor.scalar += 1;
        false
    }

    /// 🧱️ The cells of one status stack, read from the shell state each stack mirrors: bootstrap from the sync reseed a rebootstrapping hub document waits for, execution-target from the document open while it resolves its component (the `lease`/`component`/`descriptor`/`verified` stages), artifact-creation from the hub creation door, route-admission from the hub identity and the current space route.
    pub fn status_stack_cells(&self, kind: StatusStackKind) -> Vec<StatusCell> {
        let is_de = self.prefs.locale_id == "de";
        let locale = self.prefs.active_locale();
        match kind {
            StatusStackKind::Bootstrap => self.sync.sync_reseed.iter().map(|_| StatusCell { text: shell_chrome_string("status.bootstrap.rebootstrap", is_de).to_string(), action: None }).collect(),
            StatusStackKind::ExecutionTarget => self
                .opening
                .document_opening
                .as_ref()
                .filter(|opening| matches!(opening.phase, ShellDocumentOpenPhase::Resolving))
                .map(|opening| StatusCell { text: document_opening_banner_text(opening, chrome_now_ms(), is_de), action: Some(StatusAction { control_id: STATUS_STACK_EXECUTION_TARGET_CANCEL_CONTROL_ID.to_string(), label: shell_chrome_string("document.open.cancel", is_de).to_string(), event: None }) })
                .into_iter()
                .collect(),
            StatusStackKind::ArtifactCreation => {
                use semio_renderer_wgpu_hub_workspace::hub_connection::{HubArtifactCatalogPhase, HubArtifactCreationLabel, HubArtifactOpening, action as hub_action, hub_artifact_catalog_effective_phase, hub_artifact_creation_label, hub_artifact_creation_phase_str, hub_artifact_creation_phase_text, hub_artifact_creation_terminal};
                let creation = &self.directory.hub_workspace.creation;
                let mut cells = Vec::new();
                let catalog_label = match hub_artifact_catalog_effective_phase(creation) {
                    HubArtifactCatalogPhase::Idle => None,
                    HubArtifactCatalogPhase::Loading => Some(HubArtifactCreationLabel::CatalogLoading),
                    HubArtifactCatalogPhase::Ready => Some(HubArtifactCreationLabel::CatalogReady),
                    HubArtifactCatalogPhase::Unavailable => Some(HubArtifactCreationLabel::CatalogUnavailable),
                };
                if let Some(label) = catalog_label {
                    cells.push(StatusCell { text: hub_artifact_creation_label(label, locale).to_string(), action: None });
                }
                if let Some(operation) = creation.operation.as_ref() {
                    let terminal = hub_artifact_creation_terminal(operation.phase);
                    let ready = hub_artifact_creation_phase_str(operation.phase) == "ready";
                    let status = if ready && operation.opening == HubArtifactOpening::Opening {
                        hub_artifact_creation_label(HubArtifactCreationLabel::Opening, locale)
                    } else if ready && operation.opening == HubArtifactOpening::Failed {
                        hub_artifact_creation_label(HubArtifactCreationLabel::OpeningFailed, locale)
                    } else if operation.cancel_requested && !terminal {
                        hub_artifact_creation_label(HubArtifactCreationLabel::Cancelling, locale)
                    } else {
                        hub_artifact_creation_phase_text(operation.phase, locale)
                    };
                    let action = if ready && operation.opening == HubArtifactOpening::Failed {
                        Some((HubArtifactCreationLabel::OpenRetry, hub_action::OPEN_CREATED_ARTIFACT))
                    } else if !terminal {
                        Some((if operation.cancel_requested { HubArtifactCreationLabel::Cancelling } else { HubArtifactCreationLabel::Cancel }, hub_action::CANCEL_ARTIFACT_CREATION))
                    } else {
                        None
                    };
                    cells.push(StatusCell {
                        text: format!("{}\n{}\n{status}", hub_artifact_creation_label(HubArtifactCreationLabel::Heading, locale), operation.intent.name),
                        action: action.map(|(label, action)| StatusAction { control_id: format!("{STATUS_STACK_ARTIFACT_CREATION_ACTION_CONTROL_ID_PREFIX}{action}"), label: hub_artifact_creation_label(label, locale).to_string(), event: Some(ActionDescriptor { controller_id: "framework".into(), action: action.into(), args: None }) }),
                    });
                }
                cells
            }
            StatusStackKind::RouteAdmission => {
                let route = self.opening.uri_history.get(self.opening.uri_index).map_or("/", String::as_str);
                let identity = shell_identity_resolution(self.directory.identity_env.is_some(), self.directory.identity.is_some(), false, self.directory.verified_session_authority.is_some(), self.directory.identity_offline);
                let (key, offers_sign_in) = match shell_route_admission(route, identity) {
                    ShellRouteAdmission::Apply => return Vec::new(),
                    ShellRouteAdmission::AwaitIdentity => ("status.routeAdmission.awaitIdentity", false),
                    ShellRouteAdmission::AwaitSignIn => ("status.routeAdmission.awaitSignIn", true),
                    ShellRouteAdmission::ApplyOffline => ("status.routeAdmission.applyOffline", false),
                };
                let mut cells = vec![StatusCell { text: shell_chrome_string(key, is_de).to_string(), action: None }];
                if offers_sign_in {
                    cells.push(StatusCell { text: String::new(), action: Some(StatusAction { control_id: STATUS_STACK_ROUTE_ADMISSION_ACTION_CONTROL_ID.to_string(), label: shell_chrome_string("status.routeAdmission.signIn", is_de).to_string(), event: None }) });
                }
                cells
            }
        }
    }

    /// 🖱️ Answers the press of a status action that is no framework action: the execution-target's cancel stops the document open, the route-admission's sign-in opens the hub workspace.
    pub fn apply_status_action(&mut self, control_id: &str) {
        match control_id {
            STATUS_STACK_EXECUTION_TARGET_CANCEL_CONTROL_ID => self.opening.cancel_document_opening(),
            STATUS_STACK_ROUTE_ADMISSION_ACTION_CONTROL_ID => self.directory.hub_workspace_open = true,
            _ => {}
        }
    }

    /// 🧱️ Paints the four status stacks, one op per opportunity, and answers an action's press once every op has painted.
    pub fn render_status_stacks_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        let mut ops = Vec::new();
        for kind in StatusStackKind::ALL {
            if let Some(layout) = status_stack_layout(kind, &self.status_stack_cells(kind), (width, height), theme) {
                ops.extend(status_stack_ops(&layout, theme));
            }
        }
        let Some(op) = ops.get(cursor.scalar as usize) else {
            if self.chrome.chrome_build.clicked_this_frame {
                for op in &ops {
                    if let StatusOp::Hit { rect, action } = op {
                        if action.event.is_none() && rect.contains(input.pointer_x, input.pointer_y) {
                            self.apply_status_action(&action.control_id);
                        }
                    }
                }
            }
            return true;
        };
        match op {
            StatusOp::Fill(rect, color) => overlay.push_solid([rect.x, rect.y, rect.w, rect.h], *color),
            StatusOp::Text { text, x, baseline, max_w, color } => match chrome_text_complete_step(overlay, atlas, text, *x, *baseline, *max_w, theme.font_size_small, *color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    self.chrome.error = Some("Shell status stack text exceeded the retained glyph boundary".to_string());
                    cursor.glyph.reset();
                }
            },
            StatusOp::Hit { rect, action } => input.register_hit(HitTarget { rect: *rect, event: action.event.clone(), control_id: Some(action.control_id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
        }
        cursor.scalar += 1;
        false
    }
}

//#endregion ShellChrome
