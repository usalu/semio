//! 🗨️ `shell-view-overlays` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

/// ✍️ The WRAPPING twin of [`chrome_text_complete_step`], for the one piece of shell chrome whose copy
/// is prose rather than a chip label: the introduction card's body paragraphs, which React renders as
/// `<p class="whitespace-pre-line">` inside a `max-w-sm` box and therefore wraps. `rect` is the flow
/// box — its own top, not a baseline.
pub fn chrome_text_wrapped_complete_step(target: &mut DrawList, atlas: &mut FontAtlas, text: &str, rect: Rect, size: f32, color: Rgba, cursor: &mut RetainedGlyphCursor) -> Result<bool, ()> {
    match paint_retained_glyph_step_flowed(text, rect, size, color, RetainedTextFlow::Wrap, atlas, target, cursor) {
        RetainedGlyphStep::Pending => Ok(false),
        RetainedGlyphStep::Complete => {
            cursor.reset();
            Ok(true)
        }
        RetainedGlyphStep::Fault => Err(()),
    }
}

//#region ActionPanelAndUtilities
//#region 🪟️WindowPaneChrome
impl<'ctx> OverlaysViewCtx<'ctx> {
    /// 🖌️ One paint opportunity of ONE pane's overlay chip row. Steps chip by chip through
    /// `cursor.scalar` so a long label never blocks the chrome walk, and registers each chip's hit
    /// above the window body's own `ScrollRegion`. `true` once the whole row is off the critical path.
    ///
    /// 🫧️ ONE glass region per chip, pushed on its first opportunity and RE-ENTERED on every later
    /// one — pushing a region per paint opportunity would stack the same frosted band until the
    /// fixed layer filled, and painting the chip outside its own content layer is precisely the
    /// defect W3c §3 found (the glass samples the label away again).
    ///
    /// ⏪️ After the pane's own chips, an open history-edit session adds its time-travel indicator
    /// ([`Self::paint_window_time_travel_indicator_step`]).
    #[allow(clippy::too_many_arguments, reason = "the chrome walk's own paint context, forwarded unchanged")]
    pub fn paint_window_pane_chips_step(&mut self, cursor: &mut ShellChromeChildCursor, draw: &mut DrawList, atlas: &mut FontAtlas, icons: &IconAtlas, input: &mut InputState<ActionDescriptor>, theme: &Theme, window_id: &str, window_rect: Rect) -> bool {
        if !surface_fits_overlay(theme, window_rect) {
            return true;
        }
        let chips = self.as_session().window_pane_chips(window_id);
        let Some((chip, folded, disabled)) = chips.get(cursor.scalar).copied() else {
            return self.paint_window_time_travel_indicator_step(cursor, draw, atlas, icons, input, theme, window_id, window_rect, chips.len());
        };
        let is_de = self.prefs.locale_id == "de";
        let label = shell_chrome_string(chip.label_key(), is_de);
        let control_id = chip.control_id(window_id, folded);
        let item = ChromeGroupItem { control_id: control_id.as_str(), icon_id: Some(self.windows.window_pane_chip_icon_id(chip, window_id, &self.surfaces)), label: Some(label), active: !folded, disabled, kind: HitKind::Toggle };
        if cursor.rect.is_none() {
            let Some(width) = retained_chrome_group_item_width(atlas, theme, &item) else {
                self.chrome.error = Some("Shell pane chip exceeded the retained chrome boundary".to_string());
                return true;
            };
            cursor.rect = Some(window_pane_chip_rect(theme, window_rect, chip.anchor(), width));
        }
        let Some(rect) = cursor.rect else { return true };
        if cursor.depth == 0 {
            cursor.depth = draw.push_glass([rect.x, rect.y, rect.w, rect.h], 0.0, theme.glass(Level::Window)).saturating_add(1);
        }
        draw.begin_glass_content(cursor.depth - 1);
        let step = render_retained_chrome_group_item_step(&mut cursor.group_phase, &mut cursor.glyph, draw, atlas, icons, input, theme, rect, &item, false);
        draw.end_glass_content();
        match step {
            RetainedChromeGroupStep::Pending => return false,
            RetainedChromeGroupStep::Complete => {}
            RetainedChromeGroupStep::Fault => self.chrome.error = Some("Shell pane chip exceeded the retained glyph boundary".to_string()),
        }
        if !disabled {
            note_chrome_group_item(&item);
            with_chrome_control_names(|names| {
                if let Some(presentation) = names.get_mut(item.control_id) {
                    presentation.role = Some("button");
                }
            });
            self.input.pane_overlay_hits.push(HitTarget { rect, event: None, control_id: Some(control_id), kind: HitKind::Toggle, drag_axis: None, drag_data: None });
        }
        cursor.rect = None;
        cursor.depth = 0;
        cursor.scalar += 1;
        false
    }
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

