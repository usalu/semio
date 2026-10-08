//! 📶️ tui state, tick and paint functions for the Progress element: a determinate bar with a percentage, or an
//! indeterminate bar whose marker bounces while time passes. It takes no focus and no input. Wired as a
//! crate-root sibling module of `crate::tui::widget` (see that mod's `pub use crate::tui::progress::ProgressState;`).

use crate::tui::cell::{Cell, CellBuffer};
use crate::tui::geometry::{Pos, Rect};
use crate::tui::text::{display_width, elide, Elision};
use crate::tui::theme::{Glyph, GlyphSet, Role, Surface, Theme};

const FRAME_MS: u64 = 90;
const EIGHTHS: [&str; 8] = [" ", "\u{258f}", "\u{258e}", "\u{258d}", "\u{258c}", "\u{258b}", "\u{258a}", "\u{2589}"];

/// 📊 A label with a fraction between 0 and 1, or none while the amount of work is unknown.
pub struct ProgressState {
    pub label: String,
    value: Option<f32>,
    frame: u64,
}

impl ProgressState {
    pub fn determinate(label: impl Into<String>, fraction: f32) -> Self {
        Self { label: label.into(), value: Some(fraction.clamp(0.0, 1.0)), frame: 0 }
    }

    pub fn indeterminate(label: impl Into<String>) -> Self {
        Self { label: label.into(), value: None, frame: 0 }
    }

    pub fn value(&self) -> Option<f32> {
        self.value
    }

    /// 🎚️ Sets the fraction (clamped to 0..=1) or switches to indeterminate with `None`.
    pub fn set_value(&mut self, value: Option<f32>) {
        self.value = value.map(|fraction| fraction.clamp(0.0, 1.0));
    }

    /// 📐️ The width that fits the label, a 20 cell bar and the percentage.
    pub fn preferred_width(&self) -> u16 {
        display_width(&self.label).saturating_add(27)
    }
}

/// ⏱️ Advances the marker of an indeterminate bar; true when the frame changed and the widget must repaint.
pub(crate) fn progress_tick(p: &mut ProgressState, now_ms: u64) -> bool {
    if p.value.is_some() {
        return false;
    }
    let frame = now_ms / FRAME_MS;
    std::mem::replace(&mut p.frame, frame) != frame
}

fn percent(fraction: f32) -> u16 {
    (fraction * 100.0).round() as u16
}

pub(crate) fn paint_progress(p: &ProgressState, theme: &Theme, rect: Rect, buf: &mut CellBuffer) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let bg = theme.surface(Surface::Window);
    let line = Rect::new(rect.x, rect.y, rect.width, 1);
    buf.fill_rect(line, Cell::blank(theme.role(Role::Foreground), bg));
    let suffix = p.value.map_or_else(String::new, |fraction| format!(" {:>3}%", percent(fraction)));
    let suffix_cells = display_width(&suffix);
    let label_cells = display_width(&p.label).min(rect.width / 3);
    let label = elide(&p.label, label_cells, Elision::End, theme.glyphs.ellipsis());
    let label_used = display_width(&label);
    buf.put_str(Pos { x: rect.x, y: rect.y }, &label, theme.role(Role::Foreground), bg, 0, line);
    let bar_x = rect.x + label_used + u16::from(label_used > 0);
    let bar_width = (rect.x + rect.width).saturating_sub(bar_x + suffix_cells);
    if bar_width == 0 {
        return;
    }
    let (full, empty) = (theme.glyphs.glyph(Glyph::BarFull), theme.glyphs.glyph(Glyph::BarEmpty));
    let (fill, track) = (theme.role(Role::Accent), theme.role(Role::BorderNormal));
    let bar = Rect::new(bar_x, rect.y, bar_width, 1);
    match p.value {
        Some(fraction) => {
            let eighths = (fraction * f32::from(bar_width) * 8.0).round() as usize;
            let (whole, rest) = (eighths / 8, eighths % 8);
            for column in 0..usize::from(bar_width) {
                let x = bar_x + column as u16;
                let (glyph, color) = match column.cmp(&whole) {
                    std::cmp::Ordering::Less => (full, fill),
                    std::cmp::Ordering::Equal if rest > 0 && theme.glyphs == GlyphSet::Unicode => (EIGHTHS[rest], fill),
                    _ => (empty, track),
                };
                buf.put_str(Pos { x, y: rect.y }, glyph, color, bg, 0, bar);
            }
            buf.put_str(Pos { x: bar_x + bar_width, y: rect.y }, &suffix, theme.role(Role::MutedForeground), bg, 0, line);
        }
        None => {
            let marker = (usize::from(bar_width) / 4).max(1);
            let span = usize::from(bar_width).saturating_sub(marker).max(1);
            let cycle = span * 2;
            let step = (p.frame as usize) % cycle;
            let start = if step <= span { step } else { cycle - step };
            for column in 0..usize::from(bar_width) {
                let inside = (start..start + marker).contains(&column);
                let (glyph, color) = if inside { (full, fill) } else { (empty, track) };
                buf.put_str(Pos { x: bar_x + column as u16, y: rect.y }, glyph, color, bg, 0, bar);
            }
        }
    }
}
