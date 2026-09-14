use super::*;

const SOUTH_WINDOW: [[f64; 3]; 4] = [[1.0, 0.0, 0.5], [4.0, 0.0, 0.5], [4.0, 0.0, 2.5], [1.0, 0.0, 2.5]];
const SOUTH: [f64; 3] = [0.0, -1.0, 0.0];

fn sun_at(altitude_deg: f64, azimuth_deg: f64) -> [f64; 3] {
    let (altitude, azimuth) = (altitude_deg.to_radians(), azimuth_deg.to_radians());
    [altitude.cos() * azimuth.sin(), altitude.cos() * azimuth.cos(), altitude.sin()]
}

/// 🧪️ Isotropic-sky defect: on an unobstructed horizontal surface the Perez sky sums back to the
/// diffuse horizontal irradiance whatever the brightening, and receives no ground reflection.
#[test]
fn perez_sky_closes_on_a_horizontal_surface() {
    let sun = sun_at(40.0, 150.0);
    let sky = SkyState::new(sun, 800.0, 120.0, 1650.0, 0.2);
    assert!(sky.brightening.circumsolar > 0.0, "a clear sky must brighten the circumsolar region");
    let incident = sky.incident([0.0, 0.0, 1.0], 1.0, 1.0, 1.0);
    assert!((incident.sky_diffuse_w_m2 - 120.0).abs() < 1e-9);
    assert!((incident.beam_w_m2 - 800.0 * sun[2]).abs() < 1e-9);
    assert!(incident.ground_diffuse_w_m2.abs() < 1e-12);
    let wall = sky.incident(SOUTH, 1.0, 1.0, 1.0);
    assert!((wall.ground_diffuse_w_m2 - 0.1 * (800.0 * sun[2] + 120.0)).abs() < 1e-9);
    assert_eq!(SkyState::new(sun_at(-5.0, 90.0), 800.0, 120.0, 0.0, 0.2).incident(SOUTH, 1.0, 1.0, 1.0), IncidentSolar::default());
}

/// 🧪️ Overhang defect: a full-width 1 m overhang flush with the window head, sun due south at 60°,
/// shades `tan 60°` metres of the 2 m high window — exactly, by polygon projection.
#[test]
fn full_width_overhang_shades_by_projection() {
    let overhang = [[-10.0, 0.0, 2.5], [-10.0, -1.0, 2.5], [15.0, -1.0, 2.5], [15.0, 0.0, 2.5]];
    let sun = sun_at(60.0, 180.0);
    let fraction = sunlit_fraction(&SOUTH_WINDOW, SOUTH, std::iter::empty(), std::iter::once(&overhang[..]), sun);
    assert!((fraction - (2.0 - 3.0_f64.sqrt()) / 2.0).abs() < 1e-9, "sunlit fraction {fraction}");
    assert!((sunlit_fraction(&SOUTH_WINDOW, SOUTH, std::iter::empty(), std::iter::empty(), sun) - 1.0).abs() < 1e-12);
    assert_eq!(sunlit_fraction(&SOUTH_WINDOW, SOUTH, std::iter::empty(), std::iter::once(&overhang[..]), sun_at(30.0, 0.0)), 0.0);
    let (isotropic, horizon) = sky_diffuse_shading_ratios(&SOUTH_WINDOW, SOUTH, std::iter::empty(), std::iter::once(&overhang[..]));
    assert!(isotropic < 1.0 && isotropic > 0.5 && horizon <= 1.0 && horizon > isotropic);
    assert_eq!(sky_diffuse_shading_ratios(&SOUTH_WINDOW, SOUTH, std::iter::empty(), std::iter::empty()), (1.0, 1.0));
}

/// 🧪️ Interior solar defect: sunlight entering a south window of a convex box lands on the faces
/// that look back at the sun, and the patches add up to the whole window.
#[test]
fn beam_patches_partition_the_window() {
    let (w, d, h) = (8.0, 6.0, 2.7);
    let faces: [([[f64; 3]; 4], [f64; 3]); 6] = [
        ([[0.0, 0.0, 0.0], [0.0, d, 0.0], [w, d, 0.0], [w, 0.0, 0.0]], [0.0, 0.0, -1.0]),
        ([[0.0, 0.0, h], [w, 0.0, h], [w, d, h], [0.0, d, h]], [0.0, 0.0, 1.0]),
        ([[w, d, 0.0], [0.0, d, 0.0], [0.0, d, h], [w, d, h]], [0.0, 1.0, 0.0]),
        ([[w, 0.0, 0.0], [w, d, 0.0], [w, d, h], [w, 0.0, h]], [1.0, 0.0, 0.0]),
        ([[0.0, d, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, h], [0.0, d, h]], [-1.0, 0.0, 0.0]),
        ([[0.0, 0.0, 0.0], [w, 0.0, 0.0], [w, 0.0, h], [0.0, 0.0, h]], SOUTH),
    ];
    for sun in [sun_at(35.0, 160.0), sun_at(15.0, 230.0), sun_at(70.0, 180.0)] {
        let landed: f64 = faces.iter().map(|(polygon, normal)| beam_overlap_m2(&SOUTH_WINDOW, SOUTH, polygon, *normal, sun)).sum();
        assert!((landed - 6.0).abs() < 1e-9, "patches summed to {landed} m²");
    }
}
