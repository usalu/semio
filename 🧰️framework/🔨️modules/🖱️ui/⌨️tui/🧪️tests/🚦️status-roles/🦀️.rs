use super::*;
use crate::tui::text::display_width;

const APPEARANCES: [AppearanceName; 2] = [AppearanceName::Dark, AppearanceName::Light];
const STATUSES: [Status; 7] = [Status::Waiting, Status::Running, Status::Success, Status::Failure, Status::Fault, Status::Warning, Status::Info];

#[test]
fn contrast_follows_the_wcag_reference_values() {
    assert!((contrast([0, 0, 0], [255, 255, 255]) - 21.0).abs() < 0.01);
    assert!((contrast([0x76, 0x76, 0x76], [255, 255, 255]) - 4.54).abs() < 0.01, "#767676 on white is the canonical 4.5:1 grey");
    assert!((luminance([255, 255, 255]) - 1.0).abs() < 1e-6);
    assert_eq!(luminance([0, 0, 0]), 0.0);
}

#[test]
fn status_colour_roles_are_legible_on_every_surface_in_both_appearances() {
    for appearance in APPEARANCES {
        let theme = Theme::new(appearance);
        for role in [Role::Success, Role::Warning, Role::Danger, Role::Info] {
            for surface in [Surface::Base, Surface::Window, Surface::Pane, Surface::Panel] {
                let ratio = contrast(theme.role(role), theme.surface(surface));
                assert!(ratio >= 4.5, "{role:?} on {surface:?} in {appearance:?} is {ratio:.2}:1");
            }
        }
    }
}

#[test]
fn status_roles_stay_distinct_from_each_other() {
    for appearance in APPEARANCES {
        let theme = Theme::new(appearance);
        let colours = [Role::Success, Role::Warning, Role::Danger, Role::Info].map(|role| theme.role(role));
        for (index, colour) in colours.iter().enumerate() {
            assert!(!colours[index + 1..].contains(colour), "{appearance:?}: two status roles share {colour:?}");
        }
    }
}

#[test]
fn every_status_has_one_cell_glyphs_in_both_repertoires_and_the_ascii_set_is_ascii() {
    for status in STATUSES {
        for tick in 0..8 {
            for set in [GlyphSet::Unicode, GlyphSet::Ascii] {
                let glyph = status.glyph(set, tick);
                assert_eq!(display_width(glyph), 1, "{status:?} {set:?} {glyph:?}");
                if set == GlyphSet::Ascii {
                    assert!(glyph.is_ascii(), "{status:?} ascii glyph {glyph:?}");
                }
            }
        }
    }
    for set in [GlyphSet::Unicode, GlyphSet::Ascii] {
        for glyph in [Glyph::Ellipsis, Glyph::Close, Glyph::Maximize, Glyph::NewTab, Glyph::Check, Glyph::Cross, Glyph::Fault, Glyph::Waiting, Glyph::Warning, Glyph::Info, Glyph::Collapsed, Glyph::Expanded, Glyph::Prompt, Glyph::PreviousOption, Glyph::NextOption, Glyph::BarFull, Glyph::BarEmpty, Glyph::ScrollTrack, Glyph::ScrollThumb, Glyph::ToggleOn, Glyph::ToggleOff] {
            assert_eq!(display_width(set.glyph(glyph)), 1, "{glyph:?} {set:?}");
        }
    }
}

#[test]
fn the_running_spinner_cycles_four_frames_and_other_statuses_hold_still() {
    let frames: Vec<&str> = (0..4).map(|tick| Status::Running.glyph(GlyphSet::Unicode, tick)).collect();
    assert_eq!(frames, ["\u{25d0}", "\u{25d3}", "\u{25d1}", "\u{25d2}"]);
    assert_eq!(Status::Running.glyph(GlyphSet::Unicode, 4), frames[0]);
    assert_eq!(Status::Running.glyph(GlyphSet::Ascii, 1), "\\");
    assert_eq!(Status::Success.glyph(GlyphSet::Unicode, 0), Status::Success.glyph(GlyphSet::Unicode, 3));
}

#[test]
fn statuses_map_to_the_shared_roles_and_glyphs() {
    let expected = [
        (Status::Waiting, Role::MutedForeground, "\u{25cc}"),
        (Status::Success, Role::Success, "\u{2713}"),
        (Status::Failure, Role::Danger, "\u{2717}"),
        (Status::Fault, Role::Danger, "!"),
        (Status::Warning, Role::Warning, "\u{25b2}"),
        (Status::Info, Role::Info, "\u{25cf}"),
    ];
    for (status, role, glyph) in expected {
        assert_eq!((status.role(), status.glyph(GlyphSet::Unicode, 0)), (role, glyph), "{status:?}");
    }
    assert_eq!(Status::Running.role(), Role::Accent);
}

#[test]
fn switching_the_appearance_keeps_the_glyph_repertoire() {
    let mut theme = Theme::new(AppearanceName::Dark);
    theme.set_glyphs(GlyphSet::Ascii);
    theme.set_appearance(AppearanceName::Light);
    assert_eq!(theme.glyphs, GlyphSet::Ascii);
    assert_eq!(theme.appearance, AppearanceName::Light);
    assert_eq!(theme.glyphs.ellipsis(), "~");
}
