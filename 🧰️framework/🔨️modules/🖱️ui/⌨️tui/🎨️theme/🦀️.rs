use ui_styling::appearance::AppearanceName;
use ui_styling::color::linear_to_rgba8;
use ui_styling::ChromePalette;

/// ??? An 8-bit truecolor triple.
pub type Rgb = [u8; 3];

/// ??? The six nested semio chrome surfaces (base ? window ? pane ? panel ? dialog ? menu).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Surface {
    Base,
    Window,
    Pane,
    Panel,
    Dialog,
    Menu,
}

/// ??? A semantic foreground/border/state role, resolved against the active palette.
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
}

fn rgb(channel: [f32; 4]) -> Rgb {
    let [r, g, b, _a] = linear_to_rgba8(channel[0], channel[1], channel[2], channel[3]);
    [r, g, b]
}

/// ??? A resolved semio theme: every chrome color precomputed once as 8-bit truecolor.
pub struct Theme {
    pub appearance: AppearanceName,
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
}

impl Theme {
    pub fn new(appearance: AppearanceName) -> Self {
        let p: &ChromePalette = appearance.chrome();
        Self {
            appearance,
            level_base: rgb(p.level_base),
            level_window: rgb(p.level_window),
            level_pane: rgb(p.level_pane),
            level_panel: rgb(p.level_panel),
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
        }
    }

    pub fn set_appearance(&mut self, appearance: AppearanceName) {
        *self = Theme::new(appearance);
    }
}
