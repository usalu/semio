//! 👥️ Domain-neutral per-session presence palette contracts.
//#region 🔖️Palette
/// 🎨️ Resolved HSL triple for one presence palette entry — `h` in degrees `[0, 360)`, `s`/`l` in
/// `[0.0, 1.0]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresenceHsl {
    pub h: u16,
    pub s: f64,
    pub l: f64,
}

/// 🌓️ Selects which of `ui_styling::presence::LIGHT`/`DARK` [`presence_color`] resolves against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenceAppearance {
    Light,
    Dark,
}

/// 🎨️ Deterministic per-session palette color for a hub-assigned index (contract freeze §C7.5):
/// `index % 12` selects one of the 12 base hues (`ui_styling::presence::HUES`); `index / 12` (`k`)
/// desaturates by `0.25` once the roster wraps past two full cycles and alternates lightness by
/// `±0.14` every other cycle (lighter in `Light`, darker in `Dark`). Byte-identical to the TS twin
/// `presenceColor` in `🟦️.tsx`.
// 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
pub fn presence_color(index: u8, appearance: PresenceAppearance) -> PresenceHsl {
    use ui_styling::presence;
    let base = (index % 12) as usize;
    let k = index / 12;
    let h = presence::HUES[base];
    let (base_s, base_l) = match appearance {
        PresenceAppearance::Light => presence::LIGHT,
        PresenceAppearance::Dark => presence::DARK,
    };
    let s = base_s - if k >= 2 { 0.25 } else { 0.0 };
    let l_shift = if k % 2 == 1 { 0.14 } else { 0.0 };
    let l = match appearance {
        PresenceAppearance::Light => base_l + l_shift,
        PresenceAppearance::Dark => base_l - l_shift,
    };
    PresenceHsl { h, s, l }
}

