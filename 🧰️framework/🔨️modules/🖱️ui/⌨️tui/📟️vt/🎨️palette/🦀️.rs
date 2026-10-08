use crate::tui::cell::Cell;
use crate::tui::theme::{Rgb, Role, Surface, Theme};

/// 🏴️ Cell attribute bits the screen keeps for itself: the colour is the theme default, not an RGB value.
pub mod flag {
    pub const DEFAULT_FG: u8 = 0x40;
    pub const DEFAULT_BG: u8 = 0x80;
    pub const MASK: u8 = DEFAULT_FG | DEFAULT_BG;
}

/// ⚪️ Default foreground a screen stores when no theme has been applied.
pub const XTERM_FG: Rgb = [192, 192, 192];
/// ⚫️ Default background a screen stores when no theme has been applied.
pub const XTERM_BG: Rgb = [0, 0, 0];

/// 🌈️ The 16 colours children address by number, as the child's terminal definition promises them.
pub const XTERM_ANSI: [Rgb; 16] = [
    [0, 0, 0],
    [205, 0, 0],
    [0, 205, 0],
    [205, 205, 0],
    [0, 0, 238],
    [205, 0, 205],
    [0, 205, 205],
    [229, 229, 229],
    [127, 127, 127],
    [255, 0, 0],
    [0, 255, 0],
    [255, 255, 0],
    [92, 92, 255],
    [255, 0, 255],
    [0, 255, 255],
    [255, 255, 255],
];

/// 🔟️ Maps a 256-colour index onto its truecolor RGB.
pub fn color_256(n: u8) -> Rgb {
    if n < 16 {
        XTERM_ANSI[usize::from(n)]
    } else if n < 232 {
        let i = n - 16;
        let level = |c: u8| if c == 0 { 0 } else { 55 + 40 * c };
        [level(i / 36), level((i % 36) / 6), level(i % 6)]
    } else {
        let v = 8 + 10 * (n - 232);
        [v, v, v]
    }
}

fn luma(c: Rgb) -> u32 {
    (u32::from(c[0]) * 299 + u32::from(c[1]) * 587 + u32::from(c[2]) * 114) / 1000
}

fn mix(from: Rgb, to: Rgb, percent: u32) -> Rgb {
    let m = |a: u8, b: u8| ((u32::from(a) * (100 - percent) + u32::from(b) * percent) / 100) as u8;
    [m(from[0], to[0]), m(from[1], to[1]), m(from[2], to[2])]
}

/// 🎨️ What the pane paints the child's default colours and numbered colours with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    pub fg: Rgb,
    pub bg: Rgb,
    pub ansi: [Rgb; 16],
}

impl Palette {
    /// 🗄️ The child's own definition of its colours, unthemed.
    pub const fn xterm() -> Self {
        Self { fg: XTERM_FG, bg: XTERM_BG, ansi: XTERM_ANSI }
    }

    /// 🪞️ Default colours from the theme's window surface; black, white and greys follow the theme's ink and paper.
    pub fn from_theme(theme: &Theme) -> Self {
        let fg = theme.role(Role::Foreground);
        let bg = theme.surface(Surface::Window);
        let muted = theme.role(Role::MutedForeground);
        let mut ansi = XTERM_ANSI;
        let dark = luma(bg) < luma(fg);
        ansi[0] = if dark { mix(bg, fg, 20) } else { fg };
        ansi[7] = if dark { mix(bg, fg, 85) } else { mix(bg, fg, 12) };
        ansi[8] = muted;
        ansi[15] = if dark { fg } else { bg };
        Self { fg, bg, ansi }
    }

    fn themed(&self, color: Rgb) -> Rgb {
        match XTERM_ANSI.iter().position(|candidate| *candidate == color) {
            Some(index) => self.ansi[index],
            None => color,
        }
    }

    /// 🧭️ The foreground and background `cell` shows under this palette.
    pub fn resolve(&self, cell: &Cell) -> (Rgb, Rgb) {
        let fg = if cell.attrs & flag::DEFAULT_FG != 0 { self.fg } else { self.themed(cell.fg) };
        let bg = if cell.attrs & flag::DEFAULT_BG != 0 { self.bg } else { self.themed(cell.bg) };
        (fg, bg)
    }
}
