//! 🗨️ `shell-view-overlays` staging of `navbar-footer-and-overlays`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> OverlaysViewCtx<'ctx> {
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
                if let Some(plan) = self.as_session().command_palette_plan(theme, width, height) {
                    overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
                    let rect = plan.dialog;
                    open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Dialog));
                    let title = shell_chrome_string(if plan.kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, self.prefs.locale_id == "de");
                    note_chrome_control_name(plan.kind.dialog_id(), Some(title));
                    input.register_hit(HitTarget { rect, event: None, control_id: Some(plan.kind.dialog_id().to_string()), kind: HitKind::Generic, drag_axis: None, drag_data: None });
                    cursor.rect = Some(rect);
                } else if matches!(self.input.overlay_state, OverlayState::Dropdown(_)) {
                    let rect = Rect::new(width * 0.5 - 200.0, theme.navbar_height + 8.0, 400.0, height * 0.55);
                    open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Menu));
                    cursor.rect = Some(rect);
                }
                cursor.phase = 1;
            }
            1 => {
                if let Some(plan) = self.as_session().command_palette_plan(theme, width, height) {
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
                                        self.chrome.error = Some("Shell palette input exceeded the retained glyph boundary".to_string());
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
                                    let title = shell_chrome_string(if plan.kind == ShellPaletteKind::Search { "search.title" } else { "find.title" }, self.prefs.locale_id == "de");
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
                                self.chrome.error = Some("Shell palette group exceeded the retained glyph boundary".to_string());
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
                                        self.chrome.error = Some("Shell palette row exceeded the retained glyph boundary".to_string());
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
                                        self.chrome.error = Some("Shell palette row description exceeded the retained glyph boundary".to_string());
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
                                    self.chrome.error = Some("Shell palette empty state exceeded the retained glyph boundary".to_string());
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
                let is_de = self.prefs.locale_id == "de";
                let label = match &self.input.overlay_state {
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
                                self.chrome.error = Some("Shell overlay text exceeded the retained glyph boundary".to_string());
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
                let rows = if matches!(&self.input.overlay_state, OverlayState::Dropdown(id) if id == "example") { self.docs.session_example_rows(&self.plugins, &self.prefs) } else { Vec::new() };
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
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell example row exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            3 => {
                if !self.as_chrome_view().render_context_menu_step(cursor, overlay, atlas, icons, input, theme, width, height) {
                    return false;
                }
                cursor.scalar = 0;
                cursor.item = 0;
                cursor.phase = 4;
            }
            4 => {
                if !self.as_chrome_view().render_chrome_tooltip_step(cursor, overlay, atlas, input, theme, width, height) {
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
                let pills = self.as_chrome_view().surface_status_pills(theme);
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
                        self.chrome.error = Some("Shell world3d status pill exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                close_chrome_overlay_glass_content(cursor, overlay);
                cursor.item += 1;
                cursor.scalar = 0;
                return false;
            }
            8 => {
                let items = self.directory.board_presence_paint_items(theme, &self.surfaces);
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
                        self.chrome.error = Some("Shell board presence chip exceeded the retained glyph boundary".to_string());
                        cursor.glyph.reset();
                    }
                }
                overlay.pop_scissor();
                cursor.item += 1;
                cursor.scalar = 0;
                return false;
            }
            9 => {
                let controls = self.as_chrome_view().surface_overlay_controls(theme);
                let Some((control, anchor)) = controls.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.phase = 10;
                    return false;
                };
                let label = self.chrome.chrome_control_label(&control.control_id, &control.label);
                let item = ChromeGroupItem { control_id: control.control_id.as_str(), icon_id: control.icon_id, label: Some(label.as_str()), active: control.active, disabled: false, kind: HitKind::NavbarItem };
                let Some(item_w) = retained_chrome_group_item_width(atlas, theme, &item) else {
                    self.chrome.error = Some("Shell surface control exceeded the retained chrome boundary".to_string());
                    cursor.item += 1;
                    return false;
                };
                let rect = Rect::new(anchor[0], anchor[1], item_w.max(theme.control_height), theme.control_height);
                match render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, overlay, atlas, icons, input, theme, rect, &item, true) {
                    RetainedChromeGroupStep::Pending => return false,
                    RetainedChromeGroupStep::Complete => {}
                    RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell surface control exceeded the retained glyph boundary".to_string()),
                }
                cursor.item += 1;
                return false;
            }
            10 => {
                if let Some(rect) = self.input.keyboard_focus_ring_rect(input) {
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
}

//#endregion ShellChrome

