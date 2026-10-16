//! 🍱️ `shell-view-chrome` staging of `chrome-overlays-and-tour`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellChrome
//#region 🔖️ChromeOverlaysAndTour

/// ⏱️ Pure delay-threshold check, factored out of `render_chrome_tooltip` so it's unit-testable without
/// a `DrawList`/`FontAtlas` fixture.
pub(crate) fn chrome_tooltip_ready(hover: &ChromeTooltipHover, now_ms: f64) -> bool {
    now_ms - hover.started_ms >= CHROME_TOOLTIP_DELAY_MS
}

//#region 🧯️TransientNoticeAndAgentOverlays

/// 📐️ The banner's rect: top-centre, `navbar_height + 8px` down, sized to one line of the message
/// plus the close control. The width estimate is the same monospace approximation
/// `render_chrome_tooltip` already uses — this renderer measures glyphs only while painting them.
pub(crate) fn transient_notice_rect(notice: &ShellTransientNotice, close_label: &str, width: f32, theme: &Theme) -> Rect {
    let glyphs = notice.message.chars().count() + close_label.chars().count() + 2;
    let text_w = glyphs as f32 * theme.font_size_small * 0.6;
    let banner_w = (text_w + theme.padding_standard * 4.0).min(TRANSIENT_NOTICE_MAX_WIDTH).min((width - theme.padding_standard * 2.0).max(1.0));
    let banner_h = theme.font_size_small * 1.6 + theme.padding_standard * 2.0;
    Rect::new(((width - banner_w) * 0.5).max(0.0), theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP, banner_w, banner_h)
}

//#region 🎬️PluginInstallBand

/// 🎨️ `(border, fill, text)` for one phase, reusing [`transient_notice_tone`]'s own severity map so
/// the band tints exactly like every other shell banner: a running install is neutral, a cancelled
/// one a warning, a failed one destructive.
pub(crate) fn plugin_install_tone(phase: &ShellPluginInstallPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellPluginInstallPhase::Resolving | ShellPluginInstallPhase::Loading => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellPluginInstallPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellPluginInstallPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect — same top-centre geometry as the notice, one band height further down.
pub(crate) fn plugin_install_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let glyphs = message.chars().count() + action_label.chars().count() + 2;
    let text_w = glyphs as f32 * theme.font_size_small * 0.6;
    let band_w = (text_w + theme.padding_standard * 4.0).min(TRANSIENT_NOTICE_MAX_WIDTH).min((width - theme.padding_standard * 2.0).max(1.0));
    let band_h = theme.font_size_small * 1.6 + theme.padding_standard * 2.0;
    Rect::new(((width - band_w) * 0.5).max(0.0), theme.navbar_height + TRANSIENT_NOTICE_TOP_GAP + band_h + PLUGIN_INSTALL_TOP_GAP, band_w, band_h)
}

//#endregion 🎬️PluginInstallBand
//#region 🚪️DocumentOpenBand

/// 🗣️ The band's message: phase, which app, step and elapsed seconds while it runs; a failure appends
/// its own reason. English first, German through [`shell_chrome_string`].
pub(crate) fn document_opening_banner_text(opening: &ShellDocumentOpening, now_ms: f64, is_de: bool) -> String {
    let (key, step) = match opening.phase {
        ShellDocumentOpenPhase::Resolving => ("document.open.resolving", Some(1)),
        ShellDocumentOpenPhase::Instantiating => ("document.open.instantiating", Some(2)),
        ShellDocumentOpenPhase::Seeding => ("document.open.seeding", Some(DOCUMENT_OPEN_STEPS)),
        ShellDocumentOpenPhase::Cancelled => ("document.open.cancelled", None),
        ShellDocumentOpenPhase::Failed(_) => ("document.open.failed", None),
    };
    let head = match opening.phase {
        ShellDocumentOpenPhase::Resolving => {
            let detail =
                ["document.open.resolving.lease", "document.open.resolving.component", "document.open.resolving.descriptor", "document.open.resolving.verified"][usize::from(opening.resolve_step.load(std::sync::atomic::Ordering::Acquire).min(3))];
            format!("{} {} ({})", shell_chrome_string(key, is_de), opening.label, shell_chrome_string(detail, is_de))
        }
        _ => format!("{} {}", shell_chrome_string(key, is_de), opening.label),
    };
    match (&opening.phase, step) {
        (ShellDocumentOpenPhase::Failed(reason), _) => format!("{head}: {reason}"),
        (_, Some(step)) => format!("{head} · {step}/{DOCUMENT_OPEN_STEPS} · {} s", ((now_ms - opening.started_at_ms).max(0.0) / 1000.0).floor() as u64),
        _ => head,
    }
}

/// 🎨️ `(border, fill, text)` for one phase, the same severity map every shell banner uses.
pub(crate) fn document_opening_tone(phase: &ShellDocumentOpenPhase, theme: &Theme) -> (Rgba, Rgba, Rgba) {
    match phase {
        ShellDocumentOpenPhase::Resolving | ShellDocumentOpenPhase::Instantiating | ShellDocumentOpenPhase::Seeding => transient_notice_tone(semio_framework::Severity::Info, theme),
        ShellDocumentOpenPhase::Cancelled => transient_notice_tone(semio_framework::Severity::Warning, theme),
        ShellDocumentOpenPhase::Failed(_) => transient_notice_tone(semio_framework::Severity::Error, theme),
    }
}

/// 📐️ The band's rect: the plugin-install band's geometry, one band height and gap further down, so
/// an open that installs its plugin shows both.
pub(crate) fn document_opening_rect(message: &str, action_label: &str, width: f32, theme: &Theme) -> Rect {
    let band = plugin_install_rect(message, action_label, width, theme);
    Rect::new(band.x, band.y + band.h + PLUGIN_INSTALL_TOP_GAP, band.w, band.h)
}

//#endregion 🚪️DocumentOpenBand
//#endregion 🧯️TransientNoticeAndAgentOverlays
/// 🫧 Closes an overlay sheet's glass-content layer, if this walk opened one — the tour card and the
/// confirm dialog both. `cursor.depth` carries `region + 1` so `0` is "never opened": these ladders
/// are stepped ACROSS host opportunities and can be abandoned at a fault, a closed session, a
/// vanished step or a popped dialog, and an unbalanced `begin_glass_content` would classify every
/// LATER overlay child as this sheet's foreground.
pub(crate) fn open_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList, rect: Rect, radius: f32, style: ui_wgpu::wgpu::GlassStyle) {
    close_chrome_overlay_glass_content(cursor, overlay);
    let region = overlay.push_glass([rect.x, rect.y, rect.w, rect.h], radius, style);
    cursor.depth = region.saturating_add(1);
    overlay.begin_glass_content(region);
}

pub(crate) fn close_chrome_overlay_glass_content(cursor: &mut ShellChromeChildCursor, overlay: &mut DrawList) {
    if cursor.depth == 0 {
        return;
    }
    overlay.end_glass_content();
    cursor.depth = 0;
}

//#endregion 🔖️ChromeOverlaysAndTour
//#endregion ShellChrome

