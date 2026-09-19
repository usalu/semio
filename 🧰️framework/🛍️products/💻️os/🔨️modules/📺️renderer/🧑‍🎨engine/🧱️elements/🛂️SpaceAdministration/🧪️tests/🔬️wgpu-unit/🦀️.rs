//! 🧪️ Laws of the wgpu Space Administration sheet (packet W15e, audit item 3). The TRANSPORT and the
//! control set have been live on both targets since W1e; what is proven here is the chrome that was
//! missing — that the sheet lands where React's class list puts it, that it clips instead of
//! overflowing, and that a control the hub did not authorise mints no hit target.

use super::*;

fn theme() -> Theme {
    Theme::default()
}

fn row(control_id: &str, enabled: bool) -> SpaceAdministrationRow {
    SpaceAdministrationRow { control_id: control_id.to_string(), enabled, label: "Role".into(), value: "Author".into() }
}

fn plan(rows: Vec<SpaceAdministrationRow>) -> SpaceAdministrationPlan {
    SpaceAdministrationPlan { notice: None, rows, space_id: "space-1".into(), status: "Administration page is current.".into(), title: "Space administration".into() }
}

#[test]
fn the_sheet_is_horizontally_centred_at_reacts_own_width_below_the_navbar() {
    let theme = theme();
    let sheet = space_administration_sheet_rect(1600.0, 900.0, 4, &theme);
    assert_eq!(sheet.w, SPACE_ADMINISTRATION_WIDTH);
    assert!((sheet.x - (1600.0 - SPACE_ADMINISTRATION_WIDTH) * 0.5).abs() < 0.01, "left-1/2 -translate-x-1/2");
    assert_eq!(sheet.y, theme.navbar_height, "top-workbench");
}

#[test]
fn the_sheet_never_grows_past_seventy_percent_of_the_viewport() {
    let theme = theme();
    let sheet = space_administration_sheet_rect(1600.0, 900.0, 400, &theme);
    assert!(sheet.h <= 900.0 * SPACE_ADMINISTRATION_MAX_VIEWPORT_FRACTION + 0.01, "max-h-[70vh], got {}", sheet.h);
}

#[test]
fn the_sheet_stays_inside_a_window_narrower_than_itself() {
    let theme = theme();
    let sheet = space_administration_sheet_rect(320.0, 240.0, 8, &theme);
    assert!(sheet.x >= 0.0);
    assert!(sheet.x + sheet.w <= 320.0 + 0.01);
    assert!(sheet.y + sheet.h <= 240.0 + 0.01);
}

#[test]
fn the_title_line_is_reacts_own_title_em_dash_space_id() {
    assert_eq!(space_administration_title("Space administration", "space-1"), "Space administration — space-1");
}

#[test]
fn both_locales_answer_their_own_close_label_and_notices() {
    assert_eq!(space_administration_close_label(Locale::En), "Close administration");
    assert_ne!(space_administration_close_label(Locale::De), space_administration_close_label(Locale::En));
    assert_ne!(space_administration_public_notice(Locale::De), space_administration_public_notice(Locale::En));
    assert_ne!(space_administration_spectator_notice(Locale::De), space_administration_spectator_notice(Locale::En));
}

#[test]
fn the_close_control_is_the_one_affordance_the_chrome_adds_and_it_is_hit_testable() {
    let ops = space_administration_paint_ops(&plan(Vec::new()), 1600.0, 900.0, &theme(), Locale::En);
    let hits: Vec<&String> = ops
        .iter()
        .filter_map(|op| match op {
            SpaceAdministrationPaintOp::Hit { control_id, .. } => Some(control_id),
            _ => None,
        })
        .collect();
    assert_eq!(hits, vec![&SPACE_ADMINISTRATION_CLOSE_CONTROL_ID.to_string()]);
}

#[test]
fn a_control_the_hub_did_not_authorise_paints_but_mints_no_hit_target() {
    let ops = space_administration_paint_ops(&plan(vec![row("os.space-administration.remove.u1", false), row("os.space-administration.role.u2", true)]), 1600.0, 900.0, &theme(), Locale::En);
    let hits: Vec<String> = ops
        .iter()
        .filter_map(|op| match op {
            SpaceAdministrationPaintOp::Hit { control_id, .. } => Some(control_id.clone()),
            _ => None,
        })
        .collect();
    assert!(hits.contains(&"os.space-administration.role.u2".to_string()));
    assert!(!hits.contains(&"os.space-administration.remove.u1".to_string()), "a disabled control must not be clickable");
}

#[test]
fn a_roster_longer_than_the_band_is_clipped_rather_than_painted_under_the_sheets_edge() {
    let theme = theme();
    let rows: Vec<SpaceAdministrationRow> = (0..400).map(|index| row(&format!("os.space-administration.role.u{index}"), true)).collect();
    let ops = space_administration_paint_ops(&plan(rows), 1600.0, 900.0, &theme, Locale::En);
    let sheet = space_administration_sheet_rect(1600.0, 900.0, 400, &theme);
    for op in &ops {
        if let SpaceAdministrationPaintOp::Hit { rect, .. } = op {
            assert!(rect.y + rect.h <= sheet.y + sheet.h + 0.01, "a hit target escaped the sheet: {rect:?}");
        }
    }
    let visible = space_administration_visible_rows(space_administration_list_rect(sheet, &theme), &theme);
    assert!(visible > 0 && visible < 400, "the band shows some rows and clips the rest, got {visible}");
}

#[test]
fn the_status_line_is_painted_even_before_any_page_lands() {
    let ops = space_administration_paint_ops(&plan(Vec::new()), 1600.0, 900.0, &theme(), Locale::En);
    let texts: Vec<&String> = ops
        .iter()
        .filter_map(|op| match op {
            SpaceAdministrationPaintOp::Text { value, .. } => Some(value),
            _ => None,
        })
        .collect();
    assert!(texts.iter().any(|value| value.contains("Space administration — space-1")));
    assert!(texts.iter().any(|value| value.as_str() == "Administration page is current."));
}

#[test]
fn a_notice_is_painted_when_the_page_carries_one() {
    let mut with_notice = plan(Vec::new());
    with_notice.notice = Some(space_administration_public_notice(Locale::En).to_string());
    let ops = space_administration_paint_ops(&with_notice, 1600.0, 900.0, &theme(), Locale::En);
    assert!(ops.iter().any(|op| matches!(op, SpaceAdministrationPaintOp::Text { value, .. } if value == space_administration_public_notice(Locale::En))));
}

#[test]
fn the_click_step_is_last_so_every_hit_is_registered_before_it_runs() {
    let ops = space_administration_paint_ops(&plan(vec![row("os.space-administration.role.u1", true)]), 1600.0, 900.0, &theme(), Locale::En);
    assert_eq!(ops.last(), Some(&SpaceAdministrationPaintOp::Clicks));
    assert_eq!(ops.first(), Some(&SpaceAdministrationPaintOp::Sheet(space_administration_sheet_rect(1600.0, 900.0, 1, &theme()))));
}
