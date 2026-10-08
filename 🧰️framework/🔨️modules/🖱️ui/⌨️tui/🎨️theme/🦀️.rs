use ui_styling::appearance::AppearanceName;
use ui_styling::color::linear_to_rgba8;
use ui_styling::{colors, ChromePalette};

/// 🌈️ An 8-bit truecolor triple.
pub type Rgb = [u8; 3];

/// 🪆️ The six nested semio chrome surfaces (base ⊂ window ⊂ pane ⊂ panel ⊂ dialog ⊂ menu).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface {
    Base,
    Window,
    Pane,
    Panel,
    Dialog,
    Menu,
}

/// 🎭️ A semantic foreground/border/state role, resolved against the active palette.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Foreground,
    MutedForeground,
    Accent,
    AccentForeground,
    ActiveBase,
    ActiveForeground,
    BorderNormal,
    BorderEmphasized,
    BorderElement,
    HoverInteractive,
    Success,
    Warning,
    Danger,
    Info,
}

/// 🚦️ What a task, row or tab reports about itself; each state has a colour role and a glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Waiting,
    Running,
    Success,
    Failure,
    Fault,
    Warning,
    Info,
}

impl Status {
    /// 🚥 The colour role the status is drawn in.
    pub fn role(self) -> Role {
        match self {
            Status::Waiting => Role::MutedForeground,
            Status::Running => Role::Accent,
            Status::Success => Role::Success,
            Status::Failure | Status::Fault => Role::Danger,
            Status::Warning => Role::Warning,
            Status::Info => Role::Info,
        }
    }

    /// 🌀️ The glyph of the status; `Running` cycles its spinner frame once per `tick`.
    pub fn glyph(self, set: GlyphSet, tick: u64) -> &'static str {
        const UNICODE_SPINNER: [&str; 4] = ["\u{25d0}", "\u{25d3}", "\u{25d1}", "\u{25d2}"];
        const ASCII_SPINNER: [&str; 4] = ["-", "\\", "|", "/"];
        let frame = (tick % 4) as usize;
        match (self, set) {
            (Status::Running, GlyphSet::Unicode) => UNICODE_SPINNER[frame],
            (Status::Running, GlyphSet::Ascii) => ASCII_SPINNER[frame],
            (Status::Waiting, _) => set.glyph(Glyph::Waiting),
            (Status::Success, _) => set.glyph(Glyph::Check),
            (Status::Failure, _) => set.glyph(Glyph::Cross),
            (Status::Fault, _) => set.glyph(Glyph::Fault),
            (Status::Warning, _) => set.glyph(Glyph::Warning),
            (Status::Info, _) => set.glyph(Glyph::Info),
        }
    }
}

/// 🔣️ Every symbol the chrome draws that a font may lack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glyph {
    Ellipsis,
    Close,
    Maximize,
    NewTab,
    Check,
    Cross,
    Fault,
    Waiting,
    Warning,
    Info,
    Collapsed,
    Expanded,
    Prompt,
    PreviousOption,
    NextOption,
    BarFull,
    BarEmpty,
    ScrollTrack,
    ScrollThumb,
    ToggleOn,
    ToggleOff,
}

/// 🔤️ The symbol repertoire: `Unicode` assumes a font with arrows, shapes and dingbats, `Ascii` draws every symbol with one printable ASCII cell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GlyphSet {
    #[default]
    Unicode,
    Ascii,
}

impl GlyphSet {
    /// 🔠 The glyph in this repertoire, always exactly one cell wide.
    pub fn glyph(self, glyph: Glyph) -> &'static str {
        let (unicode, ascii) = match glyph {
            Glyph::Ellipsis => ("\u{2026}", "~"),
            Glyph::Close => ("\u{2715}", "x"),
            Glyph::Maximize => ("\u{2922}", "^"),
            Glyph::NewTab => ("\u{29c9}", "+"),
            Glyph::Check => ("\u{2713}", "+"),
            Glyph::Cross => ("\u{2717}", "x"),
            Glyph::Fault => ("!", "!"),
            Glyph::Waiting => ("\u{25cc}", "."),
            Glyph::Warning => ("\u{25b2}", "*"),
            Glyph::Info => ("\u{25cf}", "i"),
            Glyph::Collapsed => ("\u{25b8}", ">"),
            Glyph::Expanded => ("\u{25be}", "v"),
            Glyph::Prompt => ("\u{203a}", ">"),
            Glyph::PreviousOption => ("\u{2039}", "<"),
            Glyph::NextOption => ("\u{203a}", ">"),
            Glyph::BarFull => ("\u{2588}", "#"),
            Glyph::BarEmpty => ("\u{2591}", "-"),
            Glyph::ScrollTrack => ("\u{2502}", "|"),
            Glyph::ScrollThumb => ("\u{2503}", "#"),
            Glyph::ToggleOn => ("\u{25c9}", "x"),
            Glyph::ToggleOff => ("\u{25cb}", " "),
        };
        match self {
            GlyphSet::Unicode => unicode,
            GlyphSet::Ascii => ascii,
        }
    }

    /// 🪧 The ellipsis of this repertoire for `text::elide`.
    pub fn ellipsis(self) -> &'static str {
        self.glyph(Glyph::Ellipsis)
    }
}

fn rgb(channel: [f32; 4]) -> Rgb {
    let [r, g, b, _a] = linear_to_rgba8(channel[0], channel[1], channel[2], channel[3]);
    [r, g, b]
}

fn channel_luminance(value: u8) -> f32 {
    let c = f32::from(value) / 255.0;
    if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
}

/// 🌗️ WCAG relative luminance of a colour.
pub fn luminance(color: Rgb) -> f32 {
    0.2126 * channel_luminance(color[0]) + 0.7152 * channel_luminance(color[1]) + 0.0722 * channel_luminance(color[2])
}

/// 🔆️ WCAG contrast ratio of two colours, 1 to 21.
pub fn contrast(a: Rgb, b: Rgb) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

fn mix(from: Rgb, to: Rgb, amount: f32) -> Rgb {
    let blend = |a: u8, b: u8| (f32::from(a) + (f32::from(b) - f32::from(a)) * amount).round().clamp(0.0, 255.0) as u8;
    [blend(from[0], to[0]), blend(from[1], to[1]), blend(from[2], to[2])]
}

fn legible(color: Rgb, surfaces: &[Rgb], minimum: f32) -> Rgb {
    let dark_surfaces = surfaces.iter().map(|surface| luminance(*surface)).sum::<f32>() / surfaces.len() as f32 <= 0.18;
    let target = if dark_surfaces { [255, 255, 255] } else { [0, 0, 0] };
    (0..=50)
        .map(|step| mix(color, target, step as f32 / 50.0))
        .find(|candidate| surfaces.iter().all(|surface| contrast(*candidate, *surface) >= minimum))
        .unwrap_or(target)
}

/// 🎨️ A resolved semio theme: every chrome color precomputed once as 8-bit truecolor.
pub struct Theme {
    pub appearance: AppearanceName,
    pub glyphs: GlyphSet,
    level_base: Rgb,
    level_window: Rgb,
    level_pane: Rgb,
    level_panel: Rgb,
    level_dialog: Rgb,
    level_menu: Rgb,
    foreground: Rgb,
    muted_foreground: Rgb,
    accent: Rgb,
    accent_foreground: Rgb,
    active_base: Rgb,
    active_foreground: Rgb,
    border_normal: Rgb,
    border_emphasized: Rgb,
    border_element: Rgb,
    hover_interactive: Rgb,
    success: Rgb,
    warning: Rgb,
    danger: Rgb,
    info: Rgb,
}

impl Theme {
    pub fn new(appearance: AppearanceName) -> Self {
        let p: &ChromePalette = appearance.chrome();
        let surfaces = [rgb(p.level_base), rgb(p.level_window), rgb(p.level_pane), rgb(p.level_panel)];
        Self {
            appearance,
            glyphs: GlyphSet::default(),
            level_base: surfaces[0],
            level_window: surfaces[1],
            level_pane: surfaces[2],
            level_panel: surfaces[3],
            level_dialog: rgb(p.level_dialog),
            level_menu: rgb(p.level_menu),
            foreground: rgb(p.foreground),
            muted_foreground: rgb(p.muted_foreground),
            accent: rgb(p.accent),
            accent_foreground: rgb(p.accent_foreground),
            active_base: rgb(p.active_base),
            active_foreground: rgb(p.active_foreground),
            border_normal: rgb(p.border_normal),
            border_emphasized: rgb(p.border_emphasized),
            border_element: rgb(p.border_element),
            hover_interactive: rgb(p.hover_interactive_fill),
            success: legible(rgb(colors::SUCCESS), &surfaces, 4.5),
            warning: legible(rgb(colors::WARNING), &surfaces, 4.5),
            danger: legible(rgb(colors::DANGER), &surfaces, 4.5),
            info: legible(rgb(colors::INFO), &surfaces, 4.5),
        }
    }

    pub fn surface(&self, surface: Surface) -> Rgb {
        match surface {
            Surface::Base => self.level_base,
            Surface::Window => self.level_window,
            Surface::Pane => self.level_pane,
            Surface::Panel => self.level_panel,
            Surface::Dialog => self.level_dialog,
            Surface::Menu => self.level_menu,
        }
    }

    pub fn role(&self, role: Role) -> Rgb {
        match role {
            Role::Foreground => self.foreground,
            Role::MutedForeground => self.muted_foreground,
            Role::Accent => self.accent,
            Role::AccentForeground => self.accent_foreground,
            Role::ActiveBase => self.active_base,
            Role::ActiveForeground => self.active_foreground,
            Role::BorderNormal => self.border_normal,
            Role::BorderEmphasized => self.border_emphasized,
            Role::BorderElement => self.border_element,
            Role::HoverInteractive => self.hover_interactive,
            Role::Success => self.success,
            Role::Warning => self.warning,
            Role::Danger => self.danger,
            Role::Info => self.info,
        }
    }

    pub fn set_appearance(&mut self, appearance: AppearanceName) {
        let glyphs = self.glyphs;
        *self = Theme::new(appearance);
        self.glyphs = glyphs;
    }

    /// 🎛️ Selects the symbol repertoire every painter draws with.
    pub fn set_glyphs(&mut self, glyphs: GlyphSet) {
        self.glyphs = glyphs;
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🚦️status-roles/🦀️.rs"]
mod tests;
