/// 🧫️ Scene fixtures of the tiled map tests: the 1x1 PNG, the synthetic land tile, the land cover and overlay extensions, the overlay host and the Zurich pin constants.

fn test_png_1x1() -> Vec<u8> {
    use image::codecs::png::PngEncoder;
    use image::{ColorType, ImageEncoder};
    let mut buf = Vec::new();
    let enc = PngEncoder::new(&mut buf);
    enc.write_image(&[0, 0, 0, 255], 1, 1, ColorType::Rgba8.into()).expect("png");
    buf
}

/// 🧪️ A one-feature "landcover" vector tile — painted by [`land_cover_extension`], so it exercises the
/// real `tile_local_to_screen` → `world_to_screen` path the cache must translate.
fn synthetic_land_tile() -> super::vector_tiles::VectorTile {
    use super::vector_tiles::{GeomType, VectorFeature, VectorLayer, VectorTile};
    VectorTile {
        layers: vec![VectorLayer {
            name: "landcover".to_string(),
            extent: super::vector_tiles::DEFAULT_MVT_EXTENT,
            features: vec![VectorFeature { id: None, geom_type: GeomType::Polygon, rings: vec![vec![(500.0, 500.0), (3500.0, 500.0), (3500.0, 3500.0), (500.0, 3500.0)]], lines: vec![], points: vec![], properties: std::collections::BTreeMap::new() }],
        }],
    }
}

/// 🌱️ The neutral policy plus one source-layer role (`landcover` paints as land cover) and two style presets: `plain` (colored) and `silhouette` (figure-ground).
fn land_cover_extension() -> super::Arc<dyn super::MapExtension> {
    use super::{MapPaintTechnique, MapPaletteSlot, MapStylePreset};
    let layers = vec![super::MapSourceLayer { name: "landcover".into(), role: Some(super::MapLayerRole::LandCover), draw_rank: 0, label_rank: None }];
    let preset = |id: &str, technique| MapStylePreset { id: id.into(), technique, ink: MapPaletteSlot::LabelFill, paper: MapPaletteSlot::SurfaceClear, label_fill: MapPaletteSlot::LabelFill, label_halo: MapPaletteSlot::LabelHalo };
    let style_presets = vec![preset("plain", MapPaintTechnique::Colored), preset("silhouette", MapPaintTechnique::FigureGround)];
    super::Arc::new(super::DefaultMapExtension::with_policy(super::DescribedMapPolicy::new(super::MapPolicyDescriptor { layers, style_presets, ..super::MapPolicyDescriptor::neutral() })))
}

/// 📍️ A test policy with three overlay layers drawn in this order — `zones` (areas), `paths` (pickable lines, granularity `path`) and `pins` (pickable points, granularity `pin`, `accent` style filled from another slot) — each behind its own toggle, plus a `pinLabels` toggle for the pin captions.
fn overlay_extension() -> super::Arc<dyn super::MapExtension> {
    use super::{MapLayerTarget, MapLayerToggle, MapOverlayGeometry, MapOverlayLayer, MapOverlayStyleFill, MapPaletteSlot};
    let layer = |id: &str, geometry: MapOverlayGeometry, granularity: Option<&str>| MapOverlayLayer {
        id: id.into(),
        geometry,
        granularity: granularity.map(str::to_string),
        hit_radius_px: 14.0,
        line_width: 2.0,
        point_radius: 8.0,
        point_stroke: 2.0,
        label_offset_px: 6.0,
        fill: MapPaletteSlot::PositionFill,
        stroke: MapPaletteSlot::PositionStroke,
        style_fills: vec![MapOverlayStyleFill { style: "accent".into(), fill: MapPaletteSlot::RouteStroke }],
    };
    let toggle = |key: &str, target: MapLayerTarget| MapLayerToggle { key: key.into(), targets: vec![target] };
    let descriptor = super::MapPolicyDescriptor {
        overlays: vec![layer("zones", MapOverlayGeometry::Area, None), layer("paths", MapOverlayGeometry::Line, Some("path")), layer("pins", MapOverlayGeometry::Point, Some("pin"))],
        layer_toggles: vec![toggle("zones", MapLayerTarget::Overlay("zones".into())), toggle("paths", MapLayerTarget::Overlay("paths".into())), toggle("pins", MapLayerTarget::Overlay("pins".into())), toggle("pinLabels", MapLayerTarget::OverlayLabels("pins".into()))],
        ..super::MapPolicyDescriptor::neutral()
    };
    super::Arc::new(super::DefaultMapExtension::with_policy(super::DescribedMapPolicy::new(descriptor)))
}

fn overlay_host() -> super::MapHost {
    let mut host = super::MapHost::with_extension(overlay_extension());
    host.set_size(800, 600, 1.0);
    host
}

const ZURICH_PIN: &str = r#"{"overlays":{"pins":[{"id":"zurich","coordinates":[[8.54,47.37]],"label":"Zürich"}]}}"#;
const ZURICH_PIN_AND_PATH: &str = r#"{"overlays":{"pins":[{"id":"zurich","coordinates":[[8.54,47.37]],"label":"Zürich"}],"paths":[{"id":"path-a","coordinates":[[8.54,47.37],[8.55,47.38]],"width":2.0}]}}"#;
