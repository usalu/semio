
use super::*;

fn case_610_window(overhang: f64, offset: f64, fin: f64) -> WindowProjections {
    WindowProjections { width_m: 3.0, height_m: 2.0, overhang_depth_m: overhang, overhang_offset_m: offset, fin_depth_m: fin, fin_offset_m: 0.0 }
}

/// 🧪️ Hand-derived against the profile-angle construction, independently of this code:
/// `Ω = atan(tan α / cos γ)`, shadow `= d·tanΩ − offset`, sunlit `= (h − shadow)/h`.
/// ANSI/ASHRAE 140 case 610's 1 m overhang 0.5 m above a 2 m window head, due-south sun:
/// at 60° altitude the shadow is 1.732 − 0.5 = 1.232 m, leaving 0.384 sunlit; at 25° the
/// shadow is 0.466 m, entirely absorbed by the 0.5 m offset, so the window is fully sunlit.
#[test]
fn overhang_cuts_the_summer_sun_and_clears_the_winter_sun() {
    let projections = case_610_window(1.0, 0.5, 0.0);
    let summer = window_shading(&projections, 180.0, 60.0, 180.0);
    let winter = window_shading(&projections, 180.0, 25.0, 180.0);
    assert!((summer.beam_sunlit_fraction - 0.383_97).abs() < 1e-4, "summer sunlit fraction was {}", summer.beam_sunlit_fraction);
    assert!((winter.beam_sunlit_fraction - 1.0).abs() < 1e-9, "winter sunlit fraction was {}", winter.beam_sunlit_fraction);
}

/// 🧪️ Sky-diffuse survival for the same overhang: the outer edge sits `0.5 + 1.0 = 1.5` m above
/// the window centre at 1 m depth, so `1.5/√(1² + 1.5²) = 0.832` of the window's own sky view
/// survives.
#[test]
fn overhang_attenuates_the_sky_diffuse_by_its_view_factor() {
    let shading = window_shading(&case_610_window(1.0, 0.5, 0.0), 180.0, 60.0, 180.0);
    assert!((shading.diffuse_sky_fraction - 0.832_05).abs() < 1e-4, "diffuse fraction was {}", shading.diffuse_sky_fraction);
}

/// 🧪️ An unshaded window is untouched in both components — the property every 600-series case
/// depends on.
#[test]
fn an_unshaded_window_is_untouched() {
    let projections = case_610_window(0.0, 0.0, 0.0);
    assert!(projections.is_unshaded());
    let shading = window_shading(&projections, 180.0, 60.0, 180.0);
    assert!((shading.beam_sunlit_fraction - 1.0).abs() < 1e-9);
    assert!((shading.diffuse_sky_fraction - 1.0).abs() < 1e-9);
}

/// 🧪️ Case 630's east window with a 1 m overhang and 1 m fins at the jambs, sun 30° round from
/// the surface normal at 40° altitude: `tanΩ = tan40/cos30 = 0.969` gives a 0.969 m overhang
/// shadow, and `tan30 = 0.577` gives a 0.577 m fin shadow, so
/// `(2 − 0.969)/2 · (3 − 0.577)/3 = 0.416`.
#[test]
fn fins_and_overhang_multiply() {
    let shading = window_shading(&case_610_window(1.0, 0.0, 1.0), 90.0, 40.0, 120.0);
    assert!((shading.beam_sunlit_fraction - 0.416_33).abs() < 1e-4, "sunlit fraction was {}", shading.beam_sunlit_fraction);
    assert!((shading.diffuse_sky_fraction - 0.588_35).abs() < 1e-4, "diffuse fraction was {}", shading.diffuse_sky_fraction);
}

/// 🧪️ The sun behind the wall, and the sun below the horizon, both leave nothing to shade.
#[test]
fn no_beam_from_behind_the_wall_or_below_the_horizon() {
    let projections = case_610_window(1.0, 0.5, 0.0);
    assert_eq!(window_shading(&projections, 180.0, 40.0, 10.0).beam_sunlit_fraction, 0.0);
    assert_eq!(window_shading(&projections, 180.0, -5.0, 180.0).beam_sunlit_fraction, 0.0);
}
