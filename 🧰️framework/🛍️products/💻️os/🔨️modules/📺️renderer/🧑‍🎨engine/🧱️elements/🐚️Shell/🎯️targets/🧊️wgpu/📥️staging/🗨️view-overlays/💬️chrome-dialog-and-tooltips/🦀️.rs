//! 🗨️ `shell-view-overlays` staging of `chrome-dialog-and-tooltips`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
impl<'ctx> OverlaysViewCtx<'ctx> {
    /// 🔐️ Paints the `/hub` workspace as the centered overlay React mounts outside `defaultDock`.
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's paint context is forwarded unchanged")]
    pub fn render_hub_workspace_step(
        &mut self,
        cursor: &mut ShellChromeChildCursor,
        overlay: &mut DrawList,
        atlas: &mut FontAtlas,
        icons: &IconAtlas,
        input: &mut InputState<ActionDescriptor>,
        theme: &Theme,
        width: f32,
        height: f32,
        world_resources: &mut infinite_world::world::World3dBuildContext,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> bool {
        if !self.directory.hub_workspace_open {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        let surface = semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::FRAMEWORK_HUB_PANEL_ID;
        match cursor.phase {
            0 => {
                overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
                input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, width, height), event: None, control_id: Some("framework.hub.scrim".into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                let card_w = (width - theme.padding_standard * 4.0).clamp(320.0, 960.0);
                let card_h = (height - theme.navbar_height - theme.footer_height - theme.padding_standard * 4.0).clamp(240.0, 760.0);
                let rect = Rect::new((width - card_w) * 0.5, (height - card_h) * 0.5, card_w, card_h);
                open_chrome_overlay_glass_content(cursor, overlay, rect, theme.border_radius, theme.glass(Level::Dialog));
                cursor.rect = Some(rect);
                cursor.phase = 1;
            }
            1 => {
                let Some(rect) = cursor.rect else {
                    close_chrome_overlay_glass_content(cursor, overlay);
                    return true;
                };
                let content = Rect::new(rect.x + theme.padding_standard, rect.y + theme.padding_standard, (rect.w - theme.padding_standard * 2.0).max(1.0), (rect.h - theme.padding_standard * 2.0).max(1.0));
                overlay.push_scissor(content);
                cursor.phase = 2;
            }
            2 => {
                let Some(rect) = cursor.rect else {
                    cursor.phase = 3;
                    return false;
                };
                let Some(document) = self.docs.panel_documents.remove(surface) else {
                    cursor.phase = 3;
                    return false;
                };
                let content = Rect::new(rect.x + theme.padding_standard, rect.y + theme.padding_standard, (rect.w - theme.padding_standard * 2.0).max(1.0), (rect.h - theme.padding_standard * 2.0).max(1.0));
                let controller = self.docs.document_controller_id(&self.plugins);
                semio_framework_os_renderer_wgpu_ui_engine::retained_document_consumer::set_ui_document_flow(surface, ui_contract::UiFlow::DEFAULT);
                let complete = {
                    let scroll_offsets = &mut self.input.scroll_offsets;
                    let collapsed_sections = &mut self.input.collapsed_sections;
                    let open_selects = &mut self.input.open_selects;
                    let widget_maps = &mut self.input.widget_maps_staging;
                    let mut hosts = semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts { chrome_labels: scene_chrome_labels(self.prefs.locale_id == "de"), world3d_states: &mut self.surfaces.world3d_states, world_resources, window_id: surface };
                    let viewport_height_for_widgets = overlay.screen_height();
                    let mut ctx = framework_widget_context(overlay, None, atlas, Some(icons), input, theme, scroll_offsets, collapsed_sections, open_selects, Some(widget_maps), viewport_height_for_widgets);
                    ctx.pick_clip = Some(content);
                    render_ui_document_step(&mut cursor.document, &document, content, &mut ctx, surface, controller.as_str(), self.chrome.chrome_build.driver.drag, &mut hosts, cx)
                };
                self.docs.panel_documents.insert(surface.to_string(), document);
                if !complete {
                    if !cursor.document.terminal_is_fault() {
                        return false;
                    }
                    self.as_session().record_document_paint_fault(surface);
                } else {
                    self.as_session().note_retained_body_painted(surface);
                }
                self.input.register_retained_body_hits(surface, content, input);
                cursor.phase = 3;
            }
            3 => {
                overlay.pop_scissor();
                close_chrome_overlay_glass_content(cursor, overlay);
                return true;
            }
            _ => return true,
        }
        false
    }

    /// 🛂️ Paints the Space Administration sheet — packet W15e, the chrome half of the lane W1e made
    /// transport-complete on both targets and nothing ever rendered.
    pub fn render_space_administration_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
                use semio_framework_os_renderer_wgpu_hub_workspace::space_administration::SpaceAdministrationPaintOp as Op;
        let Some(plan) = self.directory.space_administration_plan(&self.prefs) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let ops = semio_framework_os_renderer_wgpu_hub_workspace::space_administration::space_administration_paint_ops(&plan, width, height, theme, self.prefs.active_locale());
        let Some(op) = ops.get(cursor.scalar) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        match op {
            Op::Sheet(rect) => open_chrome_overlay_glass_content(cursor, overlay, *rect, theme.border_radius, theme.glass(Level::Dialog)),
            Op::Fill { rect, color } => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], *color, theme.border_radius),
            Op::Text { value, x, y, max_w, size, color } => match chrome_text_complete_step(overlay, atlas, value, *x, *y, *max_w, *size, *color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    self.chrome.error = Some("Shell space-administration text exceeded the retained glyph boundary".to_string());
                    cursor.glyph.reset();
                }
            },
            Op::Hit { rect, control_id } => input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(control_id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            Op::Clicks => {
                if self.chrome.chrome_build.clicked_this_frame {
                    let (x, y) = (input.pointer_x, input.pointer_y);
                    let hit = ops.iter().find_map(|op| match op {
                        Op::Hit { rect, control_id } if rect.contains(x, y) => Some(control_id.clone()),
                        _ => None,
                    });
                    if let Some(control_id) = hit {
                        self.directory.resolve_space_administration_control(&control_id, &self.prefs);
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// ✅️ Paints the agent approvals modal — opened purely by a non-empty queue, listing every
    /// parked request's capability/change-summary/requested-by/risk with the three decision buttons
    /// that emit an `Approval` frame back over the bridge. Packet W1j.
    pub fn render_agent_approvals_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        self.chrome.chrome_build.agent_approvals.observe(&self.chrome.chrome_build.agent.pending_approvals);
        if !self.chrome.chrome_build.agent_approvals.is_open(&self.chrome.chrome_build.agent.pending_approvals) {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        }
        let ops = self.agent_approvals_paint_ops(width, height, theme);
        let Some(op) = ops.get(cursor.scalar) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        match op {
            AgentApprovalPaintOp::Scrim => {
                overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
            }
            AgentApprovalPaintOp::Modal(rect) => open_chrome_overlay_glass_content(cursor, overlay, *rect, theme.border_radius, theme.glass(Level::Dialog)),
            AgentApprovalPaintOp::Fill { rect, color } => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], *color, theme.border_radius),
            AgentApprovalPaintOp::Text { value, x, y, max_w, size, color } => match chrome_text_complete_step(overlay, atlas, value, *x, *y, *max_w, *size, *color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    self.chrome.error = Some("Shell agent-approval text exceeded the retained glyph boundary".to_string());
                    cursor.glyph.reset();
                }
            },
            AgentApprovalPaintOp::Hit { rect, control_id } => input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(control_id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            AgentApprovalPaintOp::Clicks => {
                if self.chrome.chrome_build.clicked_this_frame {
                    let (x, y) = (input.pointer_x, input.pointer_y);
                    let hit = ops.iter().find_map(|op| match op {
                        AgentApprovalPaintOp::Hit { rect, control_id } if rect.contains(x, y) => Some(control_id.clone()),
                        _ => None,
                    });
                    if let Some(control_id) = hit {
                        self.chrome.resolve_agent_approval_control(&control_id);
                    }
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// ✅️ This frame's flat paint program for the approvals modal — rebuilt per opportunity (it is
    /// pure and small) so the retained step only has to carry an index, not a nested cursor.
    pub fn agent_approvals_paint_ops(&self, width: f32, height: f32, theme: &Theme) -> Vec<AgentApprovalPaintOp> {
                use semio_framework_os_renderer_wgpu_agent_elements::agent_approvals as approvals;
        let locale = self.prefs.active_locale();
        let pad = theme.padding_standard;
        let line_h = theme.font_size_small * 1.6;
        let now_ms = chrome_now_ms();
        let parsed: Vec<(String, approvals::ParsedApprovalSummary, f64)> =
            self.chrome.chrome_build.agent.pending_approvals.iter().map(|approval| (approval.approval_id.clone(), approvals::parse_approval_summary(&approval.summary), approval.requested_at_ms)).collect();
        let heights: Vec<f32> = parsed.iter().map(|(_, summary, _)| approvals::approval_row_height(summary, theme)).collect();
        let modal = approvals::approvals_modal_rect(width, height, &heights, theme);
        let list = approvals::approvals_list_rect(modal, theme);
        let close_label = shell_chrome_string("common.close", locale == Locale::De);
        let close_w = (close_label.chars().count() as f32 * theme.font_size_small * 0.6).max(theme.font_size_small);
        let close = Rect::new(modal.x + modal.w - pad - close_w, modal.y + pad * 0.5, close_w, theme.font_size_small * 1.6);

        let mut ops = vec![AgentApprovalPaintOp::Scrim, AgentApprovalPaintOp::Modal(modal)];
        let text = |value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba| AgentApprovalPaintOp::Text { value, x, y, max_w: max_w.max(1.0), size, color };
        ops.push(text(approvals::approvals_title(locale), modal.x + pad, modal.y + pad + theme.font_size_body, modal.w - pad * 2.0 - close_w, theme.font_size_body, theme.text));
        ops.push(text(approvals::approvals_description(locale), modal.x + pad, modal.y + pad + theme.font_size_body + line_h, modal.w - pad * 2.0, theme.font_size_small, theme.text_muted));
        ops.push(text(close_label.to_string(), close.x, close.y + theme.font_size_small, close.w, theme.font_size_small, theme.text_muted));
        ops.push(AgentApprovalPaintOp::Hit { rect: close, control_id: approvals::APPROVALS_CLOSE_CONTROL_ID.to_string() });

        if parsed.is_empty() {
            ops.push(text(approvals::approvals_empty(locale), list.x, list.y + theme.font_size_small, list.w, theme.font_size_small, theme.text_muted));
        }
        let mut row_y = list.y;
        for ((approval_id, summary, requested_at_ms), row_h) in parsed.iter().zip(heights.iter()) {
            if row_y + row_h > list.y + list.h {
                break;
            }
            let row = Rect::new(list.x, row_y, list.w, *row_h);
            let seconds_left = approvals::approval_seconds_remaining(summary.timeout_ms, *requested_at_ms, now_ms);
            for (index, approval_line) in approvals::approval_row_lines(summary, seconds_left, locale).into_iter().enumerate() {
                let color = match approval_line.tone {
                    approvals::ApprovalLineTone::Text => theme.text,
                    approvals::ApprovalLineTone::Muted => theme.text_muted,
                    approvals::ApprovalLineTone::Risk(risk) => risk.color(theme),
                };
                ops.push(AgentApprovalPaintOp::Text { value: approval_line.text, x: row.x, y: row.y + theme.font_size_small + index as f32 * line_h, max_w: row.w.max(1.0), size: theme.font_size_small, color });
            }
            for (decision, rect) in approvals::approval_decision_rects(row, theme) {
                let label_color = if matches!(decision, semio_framework_os_renderer_wgpu_agent_elements::agent_bridge::ApprovalDecision::Deny) { theme.error } else { theme.text };
                ops.push(AgentApprovalPaintOp::Fill { rect, color: theme.button });
                ops.push(AgentApprovalPaintOp::Text {
                    value: approvals::approvals_decision_label(decision, locale),
                    x: rect.x + theme.gap_standard * 2.0,
                    y: rect.y + (rect.h + theme.font_size_small) * 0.5 - 1.0,
                    max_w: (rect.w - theme.gap_standard * 4.0).max(1.0),
                    size: theme.font_size_small,
                    color: label_color,
                });
                ops.push(AgentApprovalPaintOp::Hit { rect, control_id: approvals::approval_decision_control_id(approval_id, decision) });
            }
            row_y += row_h;
        }
        ops.push(AgentApprovalPaintOp::Clicks);
        ops
    }

    pub fn render_chrome_dialog_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        let Some(request) = self.chrome.chrome_build.dialog_stack.last() else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let ops = chrome_dialog_paint_ops(request, width, height, theme, atlas);
        let Some(op) = ops.get(cursor.scalar) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        match op {
            ChromeDialogPaintOp::Scrim => {
                overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
            }
            ChromeDialogPaintOp::Modal(rect) => open_chrome_overlay_glass_content(cursor, overlay, *rect, theme.border_radius, theme.glass(Level::Dialog)),
            ChromeDialogPaintOp::Fill { rect, color } => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], *color, theme.border_radius),
            ChromeDialogPaintOp::Text { value, x, y, max_w, size, color } => match chrome_text_complete_step(overlay, atlas, value, *x, *y, *max_w, *size, *color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    self.chrome.error = Some("Shell dialog text exceeded the retained glyph boundary".to_string());
                    cursor.glyph.reset();
                }
            },
            ChromeDialogPaintOp::Hit { rect, control_id, event, kind, .. } => {
                note_chrome_dialog_control(op);
                input.register_hit(HitTarget { rect: *rect, event: event.clone(), control_id: Some(control_id.clone()), kind: *kind, drag_axis: None, drag_data: None });
            }
            ChromeDialogPaintOp::Segment { rect, control_id, .. } | ChromeDialogPaintOp::Nudge { rect, control_id, .. } => {
                note_chrome_dialog_control(op);
                input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(control_id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None });
            }
            ChromeDialogPaintOp::Clicks { dialog } => {
                if self.chrome.chrome_build.clicked_this_frame {
                    self.input.resolve_chrome_dialog_click(&ops, *dialog, input.pointer_x, input.pointer_y, &mut self.chrome, &mut self.settle);
                }
            }
        }
        cursor.scalar += 1;
        false
    }

    /// 🗨️ Paints the topmost dialog in one go — the test harness's synchronous twin of
    /// [`Self::render_chrome_dialog_step`], walking the same paint program.
    ///
    /// 🌫️ `Theme::veil` is the token-derived twin of React's `ui-veil` utility: the dialog level's
    /// own surface fill at `levels::VEIL_ALPHA` (`--veil-alpha`), not a theme-agnostic black.
    #[cfg(any(test, feature = "testkit"))]
    pub fn render_chrome_dialog(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let Some(request) = self.chrome.chrome_build.dialog_stack.last() else {
            return;
        };
        let ops = chrome_dialog_paint_ops(request, width, height, theme, atlas);
        for op in &ops {
            match op {
                ChromeDialogPaintOp::Scrim => {
                    overlay.push_glass([0.0, 0.0, width, height], 0.0, theme.veil_glass(Level::Dialog));
                }
                ChromeDialogPaintOp::Modal(rect) => {
                    let region = overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Dialog));
                    overlay.begin_glass_content(region);
                }
                ChromeDialogPaintOp::Fill { rect, color } => overlay.push_rounded([rect.x, rect.y, rect.w, rect.h], *color, theme.border_radius),
                ChromeDialogPaintOp::Text { value, x, y, size, color, .. } => chrome_text(overlay, atlas, input, theme, value, *x, *y, *size, *color),
                ChromeDialogPaintOp::Hit { rect, control_id, event, kind, .. } => {
                    note_chrome_dialog_control(op);
                    input.register_hit(HitTarget { rect: *rect, event: event.clone(), control_id: Some(control_id.clone()), kind: *kind, drag_axis: None, drag_data: None });
                }
                ChromeDialogPaintOp::Segment { rect, control_id, .. } | ChromeDialogPaintOp::Nudge { rect, control_id, .. } => {
                    note_chrome_dialog_control(op);
                    input.register_hit(HitTarget { rect: *rect, event: None, control_id: Some(control_id.clone()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                }
                ChromeDialogPaintOp::Clicks { dialog } => {
                    overlay.end_glass_content();
                    if self.chrome.chrome_build.clicked_this_frame {
                        self.input.resolve_chrome_dialog_click(&ops, *dialog, input.pointer_x, input.pointer_y, &mut self.chrome, &mut self.settle);
                    }
                }
            }
        }
    }

    /// 🎓️ Paints the armed introduction over the whole shell — the wgpu twin of `UIIntroduction`
    /// (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): a veil with the step's own targets cut out of it, a pulsing
    /// ring on the introduced element, and a card carrying the title, Skip, the body's paragraphs, the
    /// interaction checklist and a Back / `i / n` / Next-or-Done footer.
    ///
    /// 🫧 **A tour armed means its card's glyphs are encoded in the FOREGROUND pass.** The veil is a
    /// real `ui-veil` glass region (blur + saturate + tint, `Theme::veil_glass`), so everything behind
    /// it is blurred exactly as React's `backdrop-filter` blurs the page; the card then opens a
    /// `begin_glass_content` layer, which is what makes the prepared ladder re-encode the title, Skip,
    /// the body, the counter, Next/Back and the spotlight ring into the COMPOSITE target after the
    /// glass pass instead of into the scene it samples. Without that split the card painted its own
    /// copy and then blurred it away — the boot screenshot showed a smeared card under a crisp navbar,
    /// the exact inverse of the reference (`📓️w3c` §3 fixed the same defect for the window cap;
    /// ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W8a).
    ///
    /// 🧯️ The veil is a real hit target (`ui.introduction.veil`), not just paint. React's veil is
    /// `pointer-events-auto` while it blocks, so a press anywhere on it ends the tour and NOTHING under
    /// it ever sees the pointer; here the press used to fall straight through to the 3D surface below,
    /// which is how clicking Skip also orbited and picked in the scene
    /// (`📓️w5b-interaction-parity-probe.md` §4.3).
    ///
    /// 🎯️ React mounts the card as a `WindowChrome` with `borderKind="introduced"` until the
    /// user activates it, and pulses `data-introduced` on the introduced element itself; the
    /// instanced breathing ring is this renderer's own twin of those keyframes.
    ///
    /// 🧯️ The veil registers FIRST so the three card controls, registered after it, win
    /// `InputState::hit_at`'s reverse-order resolution over the sheet they sit on.
    pub fn render_chrome_tour_step(&mut self, cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) -> bool {
        if cursor.flag {
            close_chrome_overlay_glass_content(cursor, overlay);
            self.chrome.error = Some("Shell tour text exceeded the retained glyph boundary".to_string());
            cursor.flag = false;
            return true;
        }
        if self.docs.session.is_none() {
            close_chrome_overlay_glass_content(cursor, overlay);
            self.chrome.chrome_build.tour_state = None;
            return true;
        }
        let Some(step) = self.docs.chrome_tour_active_step(&self.chrome) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let Some(step_index) = self.chrome.chrome_build.tour_state.as_ref().map(|state| state.step_index) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let Some(step_count) = self.docs.session.as_ref().and_then(|session| session.app.introduction.as_ref()).map(|introduction| introduction.steps.len()) else {
            close_chrome_overlay_glass_content(cursor, overlay);
            return true;
        };
        let layout = chrome_tour_layout(self, atlas, theme, &step, step_index, step_count, width, height);
        let card = layout.card;
        let text = match cursor.scalar {
            4 => Some((layout.title.clone(), (layout.title_rect.x, layout.title_rect.y + (layout.title_rect.h + theme.font_size_body) * 0.5 - 1.0, layout.title_rect.w), theme.font_size_body, theme.text)),
            8 => Some((layout.skip.label.to_string(), chrome_tour_chip_label_box(&layout.skip, atlas, theme), theme.font_size_small, theme.text)),
            11 => Some((
                layout.counter_text.clone(),
                (layout.counter.x + theme.padding_standard * 2.0, layout.counter.y + (layout.counter.h + theme.font_size_small) * 0.5 - 1.0, (layout.counter.w - theme.padding_standard * 4.0).max(1.0)),
                theme.font_size_small,
                theme.text_muted,
            )),
            13 => layout.next.as_ref().map(|chip| (chip.label.to_string(), chrome_tour_chip_label_box(chip, atlas, theme), theme.font_size_small, theme.active_foreground)),
            14 => layout.next.as_ref().and_then(|chip| chrome_tour_chip_hotkey_box(chip, atlas, theme)).map(|(badge, box_)| (badge, box_, theme.font_size_small, theme.active_foreground)),
            17 => layout.back.as_ref().map(|chip| (chip.label.to_string(), chrome_tour_chip_label_box(chip, atlas, theme), theme.font_size_small, theme.text)),
            18 => layout.back.as_ref().and_then(|chip| chrome_tour_chip_hotkey_box(chip, atlas, theme)).map(|(badge, box_)| (badge, box_, theme.font_size_small, theme.text_muted)),
            _ => None,
        };
        if let Some((value, (x, y, text_width), size, color)) = text {
            match chrome_text_complete_step(overlay, atlas, &value, x, y, text_width, size, color, &mut cursor.glyph) {
                Ok(false) => return false,
                Ok(true) => {}
                Err(()) => {
                    cursor.glyph.reset();
                    cursor.flag = true;
                    return false;
                }
            }
            cursor.scalar += 1;
            return false;
        }
        match cursor.scalar {
            0 => {
                for band in introduction_veil_bands(width, height, &layout.cutouts) {
                    overlay.push_glass([band.x, band.y, band.w, band.h], 0.0, theme.veil_glass(Level::Dialog));
                }
            }
            1 => {
                cursor.depth = overlay.push_glass([card.x, card.y, card.w, card.h], theme.border_radius, theme.glass(Level::Dialog)).saturating_add(1);
                overlay.begin_glass_content(cursor.depth.saturating_sub(1));
            }
            2 => {
                overlay.push_introducing_border([card.x, card.y, card.w, card.h], theme.accent, theme.border_radius, theme.stroke_hairline * 2.0);
                if let Some(anchor) = layout.cutouts.first() {
                    overlay.push_introducing_border([anchor.x, anchor.y, anchor.w, anchor.h], theme.accent, theme.border_radius, theme.stroke_hairline * 2.0);
                }
            }
            3 => push_chrome_tour_chip_border(overlay, layout.title_chip, theme),
            5 => chrome_icon(overlay, icons, "grip-vertical", layout.grip.x, layout.grip.y, layout.grip.w, theme.text_muted),
            6 => push_chrome_tour_chip_border(overlay, layout.skip.rect, theme),
            7 => chrome_icon(overlay, icons, layout.skip.icon_id, chrome_tour_chip_icon_x(&layout.skip, theme), layout.skip.rect.y + (layout.skip.rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, theme.text),
            9 => overlay.push_solid([layout.footer_rule.x, layout.footer_rule.y, layout.footer_rule.w, layout.footer_rule.h], theme.border_normal),
            10 => push_chrome_tour_chip_border(overlay, layout.counter, theme),
            12 => {
                if let Some(chip) = layout.next.as_ref() {
                    overlay.push_rounded([chip.rect.x, chip.rect.y, chip.rect.w, chip.rect.h], theme.accent, theme.border_radius);
                }
            }
            15 => {
                if let Some(chip) = layout.next.as_ref() {
                    chrome_icon(overlay, icons, chip.icon_id, chrome_tour_chip_icon_x(chip, theme), chip.rect.y + (chip.rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, theme.active_foreground);
                }
            }
            16 => {
                if let Some(chip) = layout.back.as_ref() {
                    push_chrome_tour_chip_border(overlay, chip.rect, theme);
                }
            }
            19 => {
                if let Some(chip) = layout.back.as_ref() {
                    chrome_icon(overlay, icons, chip.icon_id, chrome_tour_chip_icon_x(chip, theme), chip.rect.y + (chip.rect.h - CHROME_ICON_TINY) * 0.5, CHROME_ICON_TINY, theme.text);
                }
            }
            20 => {
                let Some((paragraph, rect)) = layout.paragraphs.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.scalar = 21;
                    return false;
                };
                let (paragraph, rect) = (paragraph.clone(), *rect);
                match chrome_text_wrapped_complete_step(overlay, atlas, &paragraph, rect, theme.font_size_small, theme.text_muted, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        cursor.glyph.reset();
                        cursor.flag = true;
                        return false;
                    }
                }
                cursor.item += 1;
                return false;
            }
            21 => {
                let Some((label, done, row)) = layout.checklist.get(cursor.item) else {
                    cursor.item = 0;
                    cursor.group_phase = 0;
                    cursor.scalar = 22;
                    return false;
                };
                let (label, done, row) = (label.clone(), *done, *row);
                let color = if done { theme.text } else { theme.text_muted };
                if cursor.group_phase == 0 {
                    chrome_icon(overlay, icons, if done { "check" } else { "circle" }, row.x, row.y + (row.h - theme.font_size_small) * 0.5, theme.font_size_small, color);
                    cursor.group_phase = 1;
                    return false;
                }
                let text_x = row.x + theme.font_size_small + theme.gap_standard;
                match chrome_text_complete_step(overlay, atlas, &label, text_x, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, (row.w - theme.font_size_small - theme.gap_standard).max(1.0), theme.font_size_small, color, &mut cursor.glyph) {
                    Ok(false) => return false,
                    Ok(true) => {}
                    Err(()) => {
                        cursor.glyph.reset();
                        cursor.flag = true;
                        return false;
                    }
                }
                cursor.group_phase = 0;
                cursor.item += 1;
                return false;
            }
            22 if layout.veil_blocks_pointer => {
                input.register_hit(HitTarget { rect: Rect::new(0.0, 0.0, width, height), event: None, control_id: Some(UI_INTRODUCTION_VEIL_CONTROL_ID.into()), kind: HitKind::Button, drag_axis: None, drag_data: None })
            }
            23 => input.register_hit(HitTarget { rect: layout.skip.rect, event: None, control_id: Some(UI_INTRODUCTION_SKIP_CONTROL_ID.into()), kind: HitKind::Button, drag_axis: None, drag_data: None }),
            24 => {
                if let Some(chip) = layout.next.as_ref() {
                    input.register_hit(HitTarget { rect: chip.rect, event: None, control_id: Some(UI_INTRODUCTION_NEXT_CONTROL_ID.into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                }
            }
            25 => {
                if let Some(chip) = layout.back.as_ref() {
                    input.register_hit(HitTarget { rect: chip.rect, event: None, control_id: Some(UI_INTRODUCTION_BACK_CONTROL_ID.into()), kind: HitKind::Button, drag_axis: None, drag_data: None });
                }
            }
            26 => {
                close_chrome_overlay_glass_content(cursor, overlay);
                return true;
            }
            _ => {}
        }
        cursor.scalar += 1;
        false
    }

    //#region SilhouetteContent

    //#endregion SilhouetteContent
    /// 💬️ Paints the armed tooltip (item 1) — `AtPointer` placement/dismissal policy sourced from
    /// `ui_wgpu::wgpu::OverlayKind::Tooltip` via a scratch `UiTree` (empty; `Point` anchors never touch it).
    #[cfg(any(test, feature = "testkit"))]
    pub fn render_chrome_tooltip(&mut self, overlay: &mut DrawList, atlas: &mut FontAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, width: f32, height: f32) {
        let hovered_id = input.hovered_id.clone();
        let title = hovered_id.as_ref().and_then(|id| self.chrome.chrome_build.tooltip_titles.get(id).cloned());
        let now = chrome_now_ms();
        let armed = {
            let hover = &mut self.chrome.chrome_build.tooltip_hover;
            match (&title, hovered_id.as_ref()) {
                (Some(_), Some(id)) => {
                    let restart = hover.as_ref().map(|h| &h.control_id != id).unwrap_or(true);
                    if restart {
                        *hover = Some(ChromeTooltipHover { control_id: id.clone(), anchor_x: input.pointer_x, anchor_y: input.pointer_y, started_ms: now });
                    }
                }
                _ => *hover = None,
            }
            hover.clone()
        };
        if self.chrome.chrome_build.dialog_open() || !self.chrome.chrome_build.tooltip_shows(false) {
            return;
        }
        let Some(hover) = armed else { return };
        if !chrome_tooltip_ready(&hover, now) {
            return;
        }
        let Some(text) = title else { return };
        let padding = theme.padding_standard * 0.5;
        let (text_w, text_h) = atlas.measure_text(&text, theme.font_size_small);
        let content_w = text_w + padding * 2.0;
        let content_h = text_h + padding * 2.0;
        let scratch_tree = semio_framework_ui_retained_tree::tree::UiTree::new();
        let (x, y) = semio_framework_ui_wgpu_events::resolve_overlay_placement(
            &scratch_tree,
            semio_framework_ui_wgpu_events::OverlayAnchor::Point { x: hover.anchor_x, y: hover.anchor_y },
            (content_w, content_h),
            (width, height),
            semio_framework_ui_wgpu_events::OverlayKind::Tooltip.default_placement(),
            ui_contract::FlowInline::Ltr,
        );
        let region = overlay.push_glass([x, y, content_w, content_h], theme.border_radius, theme.glass(Level::Menu));
        overlay.begin_glass_content(region);
        chrome_text(overlay, atlas, input, theme, &text, x + padding, y + (content_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text);
        overlay.end_glass_content();
    }

}

/// 🗨️ This frame's flat paint program for `request` — rebuilt per opportunity with the shared font metrics
/// so the retained step only carries an index. Title, body, one labelled row per staged field (a
/// slider's rail with one tick per detent and its value, a segmented field's pressed segments, a
/// stepper's −/+ buttons, a reference field's removable chips and its "use current selection" button),
/// each choice followed by its wrapped consequence, then Cancel on the start edge and the submit on
/// the end edge. Action groups wrap within the dialog. A gated submit or choice
/// paints dimmed and registers no event; a destructive choice paints in the error colour.
pub fn chrome_dialog_paint_ops(request: &ChromeDialogRequest, width: f32, height: f32, theme: &Theme, atlas: &mut FontAtlas) -> Vec<ChromeDialogPaintOp> {
    let (pad, gap, small, body) = (theme.padding_standard, theme.gap_standard, theme.font_size_small, theme.font_size_body);
    let (line_small, line_body, control_h) = (small * 1.6, body * 1.6, theme.control_height);
    let dialog_w = (width - pad * 4.0).min(420.0).max(1.0);
    let inner_w = (dialog_w - pad * 2.0).max(1.0);
    let glyphs_w = |label: &str| label.chars().count() as f32 * small * 0.62;
    let button_w = |label: &str| (glyphs_w(label) + gap * 4.0).max(72.0).min(inner_w);
    let trailing: Vec<(ChromeDialogStop, &str)> = request.choices.iter().enumerate().map(|(index, choice)| (ChromeDialogStop::Choice(index), choice.label.as_str())).chain([(ChromeDialogStop::Confirm, request.confirm_label.as_str())]).collect();
    let mut action_rows: Vec<Vec<(ChromeDialogStop, &str, f32, Vec<String>)>> = vec![Vec::new()];
    let mut row_w = 0.0;
    for (stop, label) in trailing {
        let description = match stop {
            ChromeDialogStop::Choice(index) => request.choices[index].description.as_deref(),
            _ => None,
        };
        let lines = description.map(|value| atlas.wrap_lines(value, inner_w, small).into_iter().map(|range| value[range].trim().to_string()).filter(|line| !line.is_empty()).collect::<Vec<_>>()).unwrap_or_default();
        let w = if lines.is_empty() { (atlas.measure_text(label, small).0 + gap * 4.0).max(72.0).min(inner_w) } else { inner_w };
        if row_w > 0.0 && row_w + gap + w > inner_w {
            action_rows.push(Vec::new());
            row_w = 0.0;
        }
        let row = action_rows.last_mut().expect("an action row");
        row_w += w + if row.is_empty() { 0.0 } else { gap };
        row.push((stop, label, w, lines));
    }
    let row_heights: Vec<f32> = action_rows.iter().map(|row| control_h + row.iter().map(|(_, _, _, lines)| if lines.is_empty() { 0.0 } else { gap + lines.len() as f32 * line_small }).fold(0.0, f32::max)).collect();
    let cancel_w = (atlas.measure_text(&request.cancel_label, small).0 + gap * 4.0).max(72.0).min(inner_w);
    let cancel_footer = cancel_w + gap + row_w > inner_w;
    let body_h = if request.body.is_empty() { 0.0 } else { line_small };
    let actions_h = row_heights.iter().sum::<f32>() + gap * row_heights.len().saturating_sub(1) as f32 + if cancel_footer { gap + control_h } else { 0.0 };
    let dialog_h = pad * 2.0 + line_body + body_h + request.fields.len() as f32 * (line_small + control_h + gap) + gap + actions_h;
    let dialog = Rect::new((width - dialog_w) * 0.5, ((height - dialog_h) * 0.5).max(0.0), dialog_w, dialog_h);
    let x = dialog.x + pad;
    let focused = request.focused();
    let text = |value: String, x: f32, y: f32, size: f32, color: Rgba| ChromeDialogPaintOp::Text { value, x, y, max_w: inner_w.max(1.0), size, color };
    let clipped = |value: String, x: f32, y: f32, max_w: f32, color: Rgba| ChromeDialogPaintOp::Text { value, x, y, max_w: max_w.max(1.0), size: small, color };
    let ring = |rect: Rect| ChromeDialogPaintOp::Fill { rect: Rect::new(rect.x - 1.5, rect.y - 1.5, rect.w + 3.0, rect.h + 3.0), color: theme.accent };
    let mut ops = vec![ChromeDialogPaintOp::Scrim, ChromeDialogPaintOp::Modal(dialog)];
    let mut y = dialog.y + pad;
    ops.push(text(request.title.clone(), x, y + body, body, theme.text));
    y += line_body;
    if !request.body.is_empty() {
        ops.push(text(request.body.clone(), x, y + small, small, theme.text_muted));
        y += line_small;
    }
    for (index, field) in request.fields.iter().enumerate() {
        let label = field.label.clone();
        ops.push(text(if field.required { format!("{label} *") } else { label.clone() }, x, y + small, small, theme.text_muted));
        y += line_small;
        let rect = Rect::new(x, y, inner_w, control_h);
        let stop = ChromeDialogStop::Field(index);
        let control_id = request.control_id(stop);
        if focused == stop {
            ops.push(ring(rect));
        }
        let (value, placeholder) = request.field_text(index);
        let baseline = y + (control_h + small) * 0.5 - 1.0;
        let hit = |rect: Rect, label: String, kind: HitKind, semantics: ChromeDialogSemantics| ChromeDialogPaintOp::Hit { rect, stop, control_id: control_id.clone(), label, description: None, kind, event: None, semantics, disabled: false };
        match &field.kind {
            ChromeDialogFieldKind::Toggle => {
                let on = matches!(request.field_value(index), Some(DslValue::Bool(true)));
                let square = Rect::new(x, y + (control_h - small * 1.2) * 0.5, small * 1.2, small * 1.2);
                ops.push(ChromeDialogPaintOp::Fill { rect: square, color: if on { theme.accent } else { theme.button } });
                ops.push(text(value, square.x + square.w + gap * 2.0, baseline, small, theme.text));
                ops.push(hit(rect, label, HitKind::Toggle, ChromeDialogSemantics::default()));
            }
            ChromeDialogFieldKind::Segmented { options } => {
                let current = request.field_value(index).and_then(|value| value.as_str().map(str::to_string));
                let segment_w = rect.w / options.len().max(1) as f32;
                for (option, (option_value, option_label)) in options.iter().enumerate() {
                    let segment = Rect::new(rect.x + segment_w * option as f32, rect.y, (segment_w - 1.0).max(1.0), rect.h);
                    let selected = current.as_deref() == Some(option_value.as_str());
                    ops.push(ChromeDialogPaintOp::Fill { rect: segment, color: if selected { theme.accent } else { theme.button } });
                    ops.push(clipped(option_label.clone(), segment.x + gap * 2.0, baseline, segment.w - gap * 4.0, if selected { theme.active_foreground } else { theme.text }));
                    ops.push(ChromeDialogPaintOp::Segment { rect: segment, field: index, option, control_id: format!("{control_id}.option.{option_value}"), label: format!("{label}: {option_label}"), selected });
                }
            }
            ChromeDialogFieldKind::Slider { min, max, snaps, .. } => {
                let value_w = (glyphs_w(&value) + gap * 2.0).min(inner_w * 0.4);
                let rail = Rect::new(x + gap, y + control_h * 0.5 - 1.5, (inner_w - value_w - gap * 3.0).max(1.0), 3.0);
                let at = |value: f64| rail.x + rail.w * (((value - min) / (max - min).max(f64::EPSILON)) as f32).clamp(0.0, 1.0);
                ops.push(ChromeDialogPaintOp::Fill { rect: rail, color: theme.border_normal });
                for snap in snaps {
                    ops.push(ChromeDialogPaintOp::Fill { rect: Rect::new(at(*snap) - 1.0, rail.y - 4.0, 2.0, rail.h + 8.0), color: theme.text_muted });
                }
                let current = request.slider_value(index).unwrap_or(*min);
                ops.push(ChromeDialogPaintOp::Fill { rect: Rect::new(at(current) - small * 0.5, y + (control_h - small) * 0.5, small, small), color: theme.accent });
                ops.push(clipped(value.clone(), rail.x + rail.w + gap * 2.0, baseline, value_w, theme.text));
                let semantics = ChromeDialogSemantics { role: Some("slider"), pressed: None, value_text: Some(value), value_range: Some([*min, *max, current]) };
                ops.push(hit(Rect::new(rail.x, y, rail.w, control_h), label, HitKind::Slider, semantics));
            }
            ChromeDialogFieldKind::Reference { .. } => {
                let ids = request.reference_ids(index);
                let use_w = button_w(&request.labels.use_selection).min(inner_w * 0.5);
                let use_rect = Rect::new(rect.x + rect.w - use_w, rect.y, use_w, rect.h);
                let mut chip_x = rect.x;
                if ids.is_empty() {
                    ops.push(clipped(value, chip_x + gap * 2.0, baseline, use_rect.x - chip_x - gap * 4.0, theme.text_muted));
                }
                for (chip, id) in ids.iter().enumerate() {
                    let caption = format!("{id} ×");
                    let chip_rect = Rect::new(chip_x, rect.y, (glyphs_w(&caption) + gap * 4.0).min((use_rect.x - gap - chip_x).max(0.0)), rect.h);
                    if chip_rect.w < gap * 4.0 {
                        break;
                    }
                    let chip_stop = ChromeDialogStop::Chip(index, chip);
                    if focused == chip_stop {
                        ops.push(ring(chip_rect));
                    }
                    ops.push(ChromeDialogPaintOp::Fill { rect: chip_rect, color: theme.button });
                    ops.push(clipped(caption, chip_rect.x + gap * 2.0, baseline, chip_rect.w - gap * 4.0, theme.text));
                    ops.push(ChromeDialogPaintOp::Hit { rect: chip_rect, stop: chip_stop, control_id: request.control_id(chip_stop), label: request.labels.remove.replace("{item}", id), description: None, kind: HitKind::Button, event: None, semantics: ChromeDialogSemantics::default(), disabled: false });
                    chip_x += chip_rect.w + gap;
                }
                ops.push(ChromeDialogPaintOp::Fill { rect: use_rect, color: theme.button });
                ops.push(clipped(request.labels.use_selection.clone(), use_rect.x + gap * 2.0, baseline, use_rect.w - gap * 4.0, theme.text));
                ops.push(hit(use_rect, format!("{label}: {}", request.labels.use_selection), HitKind::Button, ChromeDialogSemantics::default()));
            }
            field_kind => {
                let stepper = matches!(field_kind, ChromeDialogFieldKind::Number { stepper: true, .. });
                let input = if stepper { Rect::new(rect.x, rect.y, (rect.w - control_h * 2.0 - gap * 2.0).max(1.0), rect.h) } else { rect };
                ops.push(ChromeDialogPaintOp::Fill { rect: input, color: theme.input_bg });
                let select = matches!(field_kind, ChromeDialogFieldKind::Select { .. });
                let caret = if focused == stop && !select { "|" } else { "" };
                let swatch_w = if let ChromeDialogFieldKind::Color { .. } = field_kind {
                    let components: Vec<f64> = request.field_value(index).and_then(|value| value.as_array().map(|components| components.iter().filter_map(semio_framework_value::DslValue::as_f64).collect())).unwrap_or_default();
                    let channel = |at: usize| (components.get(at).copied().unwrap_or(if at == 3 { 1.0 } else { 0.0 }).clamp(0.0, 1.0) * 255.0).round() as u8;
                    let side = (control_h - gap * 2.0).max(4.0);
                    ops.push(ChromeDialogPaintOp::Fill { rect: Rect::new(x + gap, y + (control_h - side) * 0.5, side, side), color: Rgba::from_srgb8(channel(0), channel(1), channel(2), channel(3)) });
                    side + gap
                } else {
                    0.0
                };
                ops.push(text(format!("{value}{caret}"), x + gap * 2.0 + swatch_w, baseline, small, if placeholder { theme.text_muted } else { theme.text }));
                let (kind, semantics) = match field_kind {
                    ChromeDialogFieldKind::Select { .. } => (HitKind::Select, ChromeDialogSemantics::default()),
                    ChromeDialogFieldKind::Number { min, max, .. } => {
                        let now = value.trim().parse::<f64>().ok();
                        (HitKind::Input, ChromeDialogSemantics { role: Some("spinbutton"), pressed: None, value_text: Some(value.clone()), value_range: now.zip((*min).zip(*max)).map(|(now, (min, max))| [min, max, now]) })
                    }
                    ChromeDialogFieldKind::Axis { .. } => (HitKind::Input, ChromeDialogSemantics { role: Some("spinbutton"), pressed: None, value_text: Some(value.clone()), value_range: None }),
                    _ => (HitKind::Input, ChromeDialogSemantics::default()),
                };
                ops.push(hit(input, label.clone(), kind, semantics));
                if stepper {
                    for (offset, sign, glyph, name) in [(0.0, -1.0, "−", request.labels.decrease.as_str()), (1.0, 1.0, "+", request.labels.increase.as_str())] {
                        let nudge = Rect::new(input.x + input.w + gap + offset * (control_h + gap), rect.y, control_h, rect.h);
                        ops.push(ChromeDialogPaintOp::Fill { rect: nudge, color: theme.button });
                        ops.push(clipped(glyph.to_string(), nudge.x + (control_h - small * 0.6) * 0.5, baseline, control_h, theme.text));
                        ops.push(ChromeDialogPaintOp::Nudge { rect: nudge, field: index, sign, control_id: format!("{control_id}.{}", if sign < 0.0 { "decrement" } else { "increment" }), label: format!("{name} {label}") });
                    }
                }
            }
        }
        y += control_h + gap;
    }
    y += gap;
    let button = |stop: ChromeDialogStop, label: &str, rect: Rect, ops: &mut Vec<ChromeDialogPaintOp>| {
        let event = request.action(stop);
        let gated = event.is_none() && stop != ChromeDialogStop::Cancel;
        let (fill, ink) = match stop {
            ChromeDialogStop::Choice(index) if request.choices[index].choice.destructive || request.choices[index].choice.tone == ui_contract::Tone::Danger => (theme.error, theme.active_foreground),
            ChromeDialogStop::Choice(index) if request.choices[index].choice.tone == ui_contract::Tone::Primary => (theme.accent, theme.active_foreground),
            ChromeDialogStop::Confirm => (theme.accent, theme.active_foreground),
            _ => (theme.button, theme.text),
        };
        let (fill, ink) = if gated { (fill.with_alpha(fill.a * 0.5), ink.with_alpha(ink.a * 0.5)) } else { (fill, ink) };
        if focused == stop {
            ops.push(ring(rect));
        }
        ops.push(ChromeDialogPaintOp::Fill { rect, color: fill });
        ops.push(ChromeDialogPaintOp::Text { value: label.to_string(), x: rect.x + gap * 2.0, y: rect.y + (rect.h + small) * 0.5 - 1.0, max_w: (rect.w - gap * 4.0).max(1.0), size: small, color: ink });
        let description = match stop {
            ChromeDialogStop::Choice(index) => request.choices[index].description.clone(),
            _ => None,
        };
        ops.push(ChromeDialogPaintOp::Hit { rect, stop, control_id: request.control_id(stop), label: label.to_string(), description, kind: HitKind::Button, event: if gated { None } else { event }, semantics: ChromeDialogSemantics::default(), disabled: gated });
    };
    let cancel_y = if cancel_footer { y + actions_h - control_h } else { y + row_heights.iter().take(row_heights.len().saturating_sub(1)).map(|h| h + gap).sum::<f32>() };
    button(ChromeDialogStop::Cancel, &request.cancel_label, Rect::new(x, cancel_y, cancel_w, control_h), &mut ops);
    for (row, row_h) in action_rows.into_iter().zip(row_heights) {
        let used_w = row.iter().map(|(_, _, w, _)| *w).sum::<f32>() + gap * row.len().saturating_sub(1) as f32;
        let mut left = x + inner_w - used_w;
        for (stop, label, w, lines) in row {
            let rect = Rect::new(left, y, w, control_h);
            button(stop, label, rect, &mut ops);
            for (index, line) in lines.into_iter().enumerate() {
                ops.push(clipped(line, left, y + control_h + gap + small + index as f32 * line_small, w, theme.text_muted));
            }
            left += w + gap;
        }
        y += row_h + gap;
    }
    ops.push(ChromeDialogPaintOp::Clicks { dialog });
    ops
}

#[cfg(any(test, feature = "testkit"))]
pub fn intersect_content_rect(left: Rect, right: Rect) -> Option<Rect> {
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    let x2 = (left.x + left.w).min(right.x + right.w);
    let y2 = (left.y + left.h).min(right.y + right.h);
    (x2 > x && y2 > y).then(|| Rect::new(x, y, x2 - x, y2 - y))
}

/// 📏️ Shared width pass for a menu level — also used to size a submenu BEFORE deciding which side of
/// its parent row it opens on (see `render_context_menu_level`'s flip-left check).
#[cfg(any(test, feature = "testkit"))]
pub fn context_menu_level_width(items: &[ContextMenuItem], theme: &Theme) -> f32 {
    let mut w = 180.0;
    for item in items.iter().filter(|item| !item.separator || !item.label.is_empty()) {
        let label_w = item.painted_label().chars().count() as f32 * theme.font_size_body * 0.55;
        let shortcut_w = item.shortcut.as_ref().map(|s| s.chars().count() as f32 * theme.font_size_small * 0.55 + 16.0).unwrap_or(0.0);
        w = f32::max(w, 56.0_f32 + label_w + shortcut_w);
    }
    w
}

/// 🖥️ Only the top-level menu is clamped on-screen — a submenu instead flips to the parent row's
/// left edge below when it would overflow the right edge (never repositioned vertically).
///
/// 🏷️ A separator carrying a `label` is a non-interactive header row — kept in place (never
/// dropped) and rendered as a labeled row instead of a bare rule.
#[cfg(any(test, feature = "testkit"))]
pub fn render_context_menu_level(
    overlay: &mut DrawList,
    atlas: &mut FontAtlas,
    icons: &IconAtlas,
    input: &mut InputState<ActionDescriptor>,
    theme: &Theme,
    menu: &ContextMenuState,
    items: &[ContextMenuItem],
    path_prefix: &[usize],
    origin_x: f32,
    origin_y: f32,
    viewport_w: f32,
    viewport_h: f32,
) {
    let row_h = theme.control_height;
    let w = context_menu_level_width(items, theme);
    let content_h = items.len() as f32 * row_h + 8.0;
    let available_h = (viewport_h - 8.0).max(row_h + 8.0);
    let h = content_h.min(available_h);
    let scrollable = content_h > h + 0.5;
    let scroll = if scrollable { menu.scroll_offset.clamp(0.0, content_h - h) } else { 0.0 };
    let (x, y) = if path_prefix.is_empty() { (origin_x.clamp(0.0, (viewport_w - w).max(0.0)), origin_y.clamp(0.0, (viewport_h - h).max(0.0))) } else { (origin_x, origin_y) };
    let rect = Rect::new(x, y, w, h);
    let glass_region = overlay.push_glass([rect.x, rect.y, rect.w, rect.h], theme.border_radius, theme.glass(Level::Menu));
    overlay.begin_glass_content(glass_region);
    if scrollable {
        input.register_hit(HitTarget { rect, event: None, control_id: Some("shell.context.menu.scroll".into()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    }
    let icon_size = theme.font_size_body;
    let ordinal_gap = theme.font_size_small * 0.85;
    overlay.push_scissor(rect);
    let mut visual_row = 0usize;
    let mut ordinal = 0usize;
    for (index, item) in items.iter().enumerate() {
        let row_top = rect.y + 4.0 + visual_row as f32 * row_h - scroll;
        let row_visible = row_top + row_h > rect.y && row_top < rect.y + rect.h;
        if item.separator {
            if row_visible {
                if item.label.is_empty() {
                    overlay.push_solid([rect.x + 8.0, row_top + row_h * 0.5, w - 16.0, 1.0], theme.text_muted);
                } else {
                    chrome_text(overlay, atlas, input, theme, &item.label, rect.x + 8.0, row_top + (row_h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
                }
            }
            visual_row += 1;
            continue;
        }
        ordinal += 1;
        let row_path: Vec<usize> = path_prefix.iter().copied().chain(std::iter::once(index)).collect();
        let is_active = context_menu_paths_equal(&menu.active, &row_path);
        let has_children = !item.children.is_empty();
        let submenu_collapsed = menu.submenu_collapsed_at.as_deref().is_some_and(|collapsed| context_menu_paths_equal(collapsed, &row_path));
        let submenu_open = (context_menu_submenu_open(&menu.active, &row_path, is_active, has_children) || (is_active && has_children)) && !submenu_collapsed;
        let row = Rect::new(rect.x + 4.0, row_top, w - 8.0, row_h);
        visual_row += 1;
        if row_visible {
            let (bg, fg) = if is_active { (theme.accent, theme.active_foreground) } else { (theme.button, theme.text) };
            overlay.push_rounded([row.x, row.y, row.w, row.h], bg, theme.border_radius);
            let mut text_x = row.x + 8.0;
            if ordinal <= 9 {
                let badge = format!("{ordinal}");
                chrome_text(overlay, atlas, input, theme, &badge, text_x, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
                text_x += ordinal_gap;
            }
            let icon_id = item.icon.as_deref().unwrap_or("circle-dot");
            chrome_icon(overlay, icons, icon_id, text_x, row.y + (row.h - icon_size) * 0.5, icon_size, if item.disabled { theme.text_muted } else { fg });
            text_x += icon_size + theme.gap_standard;
            chrome_text(overlay, atlas, input, theme, &item.painted_label(), text_x, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, if item.disabled { theme.text_muted } else { fg });
            if let Some(shortcut) = item.shortcut.as_deref() {
                let shortcut_w = shortcut.chars().count() as f32 * theme.font_size_small * 0.55;
                chrome_text(overlay, atlas, input, theme, shortcut, row.x + row.w - 8.0 - shortcut_w, row.y + (row.h + theme.font_size_small) * 0.5 - 1.0, theme.font_size_small, theme.text_muted);
            }
            item.register_hit(row, input);
        }
        if submenu_open {
            let child_w = context_menu_level_width(&item.children, theme);
            let opens_left = row.x + row.w + 4.0 + child_w > viewport_w;
            let child_x = if opens_left { (row.x - child_w - 4.0).max(0.0) } else { row.x + row.w + 4.0 };
            overlay.pop_scissor();
            overlay.end_glass_content();
            render_context_menu_level(overlay, atlas, icons, input, theme, menu, &item.children, &row_path, child_x, row.y, viewport_w, viewport_h);
            overlay.begin_glass_content(glass_region);
            overlay.push_scissor(rect);
        }
    }
    overlay.pop_scissor();
    overlay.end_glass_content();
}

//#endregion ShellChrome

