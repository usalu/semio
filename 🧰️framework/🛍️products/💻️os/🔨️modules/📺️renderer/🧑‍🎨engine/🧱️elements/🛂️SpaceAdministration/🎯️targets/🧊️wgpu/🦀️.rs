//! 🛂️ wgpu twin of the `🛂️SpaceAdministration` pane (`🟦️.tsx`, 459 lines) — the Shell-owned
//! administration sheet for exactly one space.
//!
//! 🧩️ Split of ownership, stated plainly: the OPERATION, its phases, its capability gating and its
//! whole control set already exist and are live on both targets since packet W1e
//! (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`'s `🏛️SpaceAdministration` region —
//! `shell_space_administration_controls` is the builder the native shell already renders from). What
//! did not exist on EITHER target was the CHROME: nothing painted a window, so
//! `open_space_administration` ran, fetched the hub's canonical page, and showed the human nothing
//! (`📓️w1e-space-administration-wasm.md` gap 3, `📓️audit-w14-transport-residual.md` row 5). This
//! module is that chrome, and only that: pure geometry and a flat paint program, no state of its own.
//!
//! 📐️ React mounts the pane as an OVERLAY SHEET, not a window: `absolute top-workbench left-1/2 z-50
//! max-h-[70vh] w-[36rem] -translate-x-1/2 overflow-auto rounded-sm border bg-base shadow-sm`
//! (`🏛️ShellHost/🟦️.tsx:10983`), mounted only while the retained operation is live. Every constant
//! below is that class list, resolved.

use ui_wgpu::wgpu::{Locale, Rect, Rgba, Theme};

//#region 🔖️Geometry
/// 📏️ `w-[36rem]` — 36 × the 16 px root font, in logical pixels.
pub const SPACE_ADMINISTRATION_WIDTH: f32 = 576.0;

/// 📏️ `max-h-[70vh]` — the sheet never grows past 70 % of the viewport, whatever the roster's size.
pub const SPACE_ADMINISTRATION_MAX_VIEWPORT_FRACTION: f32 = 0.7;

/// 📏️ One control row's height in the roster, matching the `gap-4 p-4` rhythm React lays the rows out
/// with: a label line and a value line, with the standard gap between rows.
pub fn space_administration_row_height(theme: &Theme) -> f32 {
    theme.font_size_small * 1.6 * 2.0 + theme.gap_standard
}

/// 🆔️ The close control — React's `labels.close` button, the one affordance the controls builder does
/// not mint (it describes the hub's page, not the chrome around it).
pub const SPACE_ADMINISTRATION_CLOSE_CONTROL_ID: &str = "os.space-administration.close";

/// 🆔️ The sheet's own id, the twin of React's `aria-labelledby` target
/// `os-space-administration-title`, so a probe addresses the pane by identity.
pub const SPACE_ADMINISTRATION_SHEET_ID: &str = "os.space-administration.sheet";

/// 📐️ Where the sheet sits: horizontally centred, its top at the workbench line (React's
/// `top-workbench`, i.e. immediately below the navbar), clamped so it can never leave the viewport on
/// a small window.
pub fn space_administration_sheet_rect(viewport_width: f32, viewport_height: f32, row_count: usize, theme: &Theme) -> Rect {
    let width = SPACE_ADMINISTRATION_WIDTH.min((viewport_width - theme.padding_standard * 2.0).max(1.0));
    let header = theme.padding_standard * 2.0 + theme.font_size_body * 1.6 + theme.font_size_small * 1.6 * 2.0;
    let body = row_count as f32 * space_administration_row_height(theme);
    let height = (header + body + theme.padding_standard).min((viewport_height * SPACE_ADMINISTRATION_MAX_VIEWPORT_FRACTION).max(1.0));
    let x = ((viewport_width - width) * 0.5).max(0.0);
    let y = theme.navbar_height.min((viewport_height - height).max(0.0));
    Rect::new(x, y, width, height)
}

/// 📐️ The scrolling roster band inside the sheet — React's `overflow-auto` region, below the title,
/// the close control and the status line.
pub fn space_administration_list_rect(sheet: Rect, theme: &Theme) -> Rect {
    let top = sheet.y + theme.padding_standard + theme.font_size_body * 1.6 + theme.font_size_small * 1.6 * 2.0;
    let height = (sheet.y + sheet.h - theme.padding_standard - top).max(0.0);
    Rect::new(sheet.x + theme.padding_standard, top, (sheet.w - theme.padding_standard * 2.0).max(1.0), height)
}

/// 📐️ The close control's rect — top-right of the sheet, the position React's `justify-between`
/// header row puts it in.
pub fn space_administration_close_rect(sheet: Rect, theme: &Theme, label_width: f32) -> Rect {
    let width = label_width.max(theme.font_size_small);
    Rect::new(sheet.x + sheet.w - theme.padding_standard - width, sheet.y + theme.padding_standard * 0.5, width, theme.font_size_small * 1.6)
}

/// 📐️ One roster row's rect inside the band.
pub fn space_administration_row_rect(list: Rect, index: usize, theme: &Theme) -> Rect {
    Rect::new(list.x, list.y + index as f32 * space_administration_row_height(theme), list.w, space_administration_row_height(theme))
}

/// 🔢️ How many rows fit the band — the sheet CLIPS rather than overflows, and the count says where
/// to stop emitting rather than leaving the painter to discover it.
pub fn space_administration_visible_rows(list: Rect, theme: &Theme) -> usize {
    let row = space_administration_row_height(theme);
    if row <= 0.0 {
        return 0;
    }
    (list.h / row).floor().max(0.0) as usize
}
//#endregion 🔖️Geometry

//#region 🌐️Labels
/// 🏷️ The sheet's own title line — React's `` `${labels.title} — ${spaceId}` ``, em dash included.
pub fn space_administration_title(title: &str, space_id: &str) -> String {
    format!("{title} — {space_id}")
}

/// 🏷️ The close affordance's label, in the pane's own two languages (`os.spaceAdministration.close`).
pub fn space_administration_close_label(locale: Locale) -> &'static str {
    if matches!(locale, Locale::De) {
        "Verwaltung schließen"
    } else {
        "Close administration"
    }
}

/// 🏷️ The two access notices React shows for a page that carries no capabilities at all — a
/// `public` page and a `member` page. Both are statements of fact about the hub's own answer, never a
/// locally derived role.
pub fn space_administration_public_notice(locale: Locale) -> &'static str {
    if matches!(locale, Locale::De) {
        "Dieser Space ist öffentlich sichtbar."
    } else {
        "This space is publicly visible."
    }
}

pub fn space_administration_spectator_notice(locale: Locale) -> &'static str {
    if matches!(locale, Locale::De) {
        "Du kannst diesen Space ansehen, aber nicht verwalten."
    } else {
        "You can view this space but not administer it."
    }
}
//#endregion 🌐️Labels

//#region 🔖️PaintProgram
/// 🎨️ One step of the sheet's flat paint program — the same shape the approvals modal uses, so the
/// retained chrome step only has to carry an index rather than a nested cursor.
#[derive(Clone, Debug, PartialEq)]
pub enum SpaceAdministrationPaintOp {
    /// 🕶️ The sheet's own surface (glass, like every other shell overlay).
    Sheet(Rect),
    /// 🎨️ One filled rect — a row's disabled tint or a control's button face.
    Fill {
        rect: Rect,
        color: Rgba,
    },
    Text {
        value: String,
        x: f32,
        y: f32,
        max_w: f32,
        size: f32,
        color: Rgba,
    },
    /// 🎯️ One hit-testable control. A DISABLED control registers nothing: React renders the button
    /// with `disabled`, and a hit target no click may act on is worse than none.
    Hit {
        rect: Rect,
        control_id: String,
    },
    /// 🖱️ The step that applies this frame's click, last, so every hit above it is registered first.
    Clicks,
}

/// 🧾️ One row as the sheet paints it, lifted out of the shell's own control vocabulary so this
/// module needs no dependency on the directory schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpaceAdministrationRow {
    pub control_id: String,
    pub label: String,
    pub value: String,
    pub enabled: bool,
}

/// 🧾️ Everything the sheet paints, resolved from the retained operation in one pure call.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SpaceAdministrationPlan {
    pub space_id: String,
    pub title: String,
    pub status: String,
    pub notice: Option<String>,
    pub rows: Vec<SpaceAdministrationRow>,
}

/// 🎨️ Builds this frame's flat paint program. Rows past the band are NOT emitted — the sheet clips
/// at `max-h-[70vh]` exactly as React's `overflow-auto` does, and a row nobody can see must not mint
/// a hit target under the sheet's own edge.
pub fn space_administration_paint_ops(plan: &SpaceAdministrationPlan, viewport_width: f32, viewport_height: f32, theme: &Theme, locale: Locale) -> Vec<SpaceAdministrationPaintOp> {
    let sheet = space_administration_sheet_rect(viewport_width, viewport_height, plan.rows.len(), theme);
    let list = space_administration_list_rect(sheet, theme);
    let pad = theme.padding_standard;
    let line = theme.font_size_small * 1.6;
    let close_label = space_administration_close_label(locale);
    let close_width = close_label.chars().count() as f32 * theme.font_size_small * 0.6;
    let close = space_administration_close_rect(sheet, theme, close_width);

    let mut ops = vec![SpaceAdministrationPaintOp::Sheet(sheet)];
    let mut text = |ops: &mut Vec<SpaceAdministrationPaintOp>, value: String, x: f32, y: f32, max_w: f32, size: f32, color: Rgba| {
        ops.push(SpaceAdministrationPaintOp::Text { color, max_w: max_w.max(1.0), size, value, x, y });
    };
    text(&mut ops, space_administration_title(&plan.title, &plan.space_id), sheet.x + pad, sheet.y + pad + theme.font_size_body, sheet.w - pad * 2.0 - close_width, theme.font_size_body, theme.text);
    text(&mut ops, close_label.to_string(), close.x, close.y + theme.font_size_small, close.w, theme.font_size_small, theme.text_muted);
    ops.push(SpaceAdministrationPaintOp::Hit { control_id: SPACE_ADMINISTRATION_CLOSE_CONTROL_ID.to_string(), rect: close });
    text(&mut ops, plan.status.clone(), sheet.x + pad, sheet.y + pad + theme.font_size_body + line, sheet.w - pad * 2.0, theme.font_size_small, theme.text_muted);
    if let Some(notice) = plan.notice.as_ref() {
        text(&mut ops, notice.clone(), sheet.x + pad, sheet.y + pad + theme.font_size_body + line * 2.0, sheet.w - pad * 2.0, theme.font_size_small, theme.text_muted);
    }

    let visible = space_administration_visible_rows(list, theme);
    for (index, row) in plan.rows.iter().take(visible).enumerate() {
        let rect = space_administration_row_rect(list, index, theme);
        let color = if row.enabled { theme.text } else { theme.text_muted };
        text(&mut ops, row.label.clone(), rect.x, rect.y + theme.font_size_small, rect.w, theme.font_size_small, color);
        if !row.value.is_empty() {
            text(&mut ops, row.value.clone(), rect.x, rect.y + line + theme.font_size_small, rect.w, theme.font_size_small, theme.text_muted);
        }
        if row.enabled {
            ops.push(SpaceAdministrationPaintOp::Fill { color: theme.button, rect: Rect::new(rect.x, rect.y, rect.w, line) });
            ops.push(SpaceAdministrationPaintOp::Hit { control_id: row.control_id.clone(), rect: Rect::new(rect.x, rect.y, rect.w, line) });
        }
    }
    ops.push(SpaceAdministrationPaintOp::Clicks);
    ops
}
//#endregion 🔖️PaintProgram

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
mod tests;
