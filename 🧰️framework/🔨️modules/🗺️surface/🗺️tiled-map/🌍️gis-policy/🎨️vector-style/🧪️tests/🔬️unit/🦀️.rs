//! 🔬️ Unit tests of the GIS vector style policy: water, waterway and country-hole level-of-detail gates and the weighted opaque fill.

#[test]
fn continent_water_filter_drops_inland_polygons() {
    let mut props = std::collections::BTreeMap::new();
    props.insert("class".to_string(), "lake".to_string());
    assert!(super::water_polygon_visible_for_lod(0, &props));
    assert!(!super::water_polygon_visible_for_lod(1, &props));
    assert!(super::water_polygon_visible_for_lod(2, &props));
    props.insert("class".to_string(), "ocean".to_string());
    assert!(super::water_polygon_visible_for_lod(1, &props));
    assert!(!super::waterway_visible_for_lod(1));
    assert!(super::waterway_visible_for_lod(2));
    assert!(super::country_polygon_holes_visible_for_lod(0));
    assert!(!super::country_polygon_holes_visible_for_lod(1));
    assert!(super::country_polygon_holes_visible_for_lod(2));
}

#[test]
fn weighted_opaque_fill_keeps_alpha_solid() {
    let c = super::weighted_opaque_fill(super::Color::from_rgba8(40, 50, 60, 128), 1.0);
    assert_eq!(c.to_rgba8().a, 255);
}
