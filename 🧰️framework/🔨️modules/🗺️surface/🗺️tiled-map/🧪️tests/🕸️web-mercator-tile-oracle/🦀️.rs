//! 🦀️ Web-Mercator / slippy-tile case — Rust adapter, the SUBJECT half.
//!
//! The oracle half is `🐍️.py` beside this file, which answers every vector from `mercantile`. This adapter answers the
//! same vectors from this repository's `tiled_map` implementation — `projection::lonlat_to_world`, the tile the
//! production `tiles::visible_tiles` windowing selects at a point, `projection::tile_world_rect` read back through
//! `projection::world_to_lonlat`, and `map_lod_band` — and asserts the four camera invariants in role through the
//! production `MapHost`. The `floating-point-v1` profile then compares the two answers.
//!
//! @see 🥒️.feature
//! @see ../../../🧪️tests/🗺️tiled-map-mercator-oracle/🦀️.rs — the crate's own integration test over the same fixture

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_surface::tiled_map::canvas::camera::{screen_to_world, Camera, Viewport};
    use semio_framework_surface::tiled_map::projection::{lonlat_to_world, tile_world_rect, world_to_lonlat};
    use semio_framework_surface::tiled_map::tiles::visible_tiles;
    use semio_framework_surface::tiled_map::{map_lod_band, MapHost, Point, MAP_CAMERA_ZOOM_MIN, MAX_VISIBLE_TILE_REQUESTS};
    use semio_repo_test_host::{Context, Json, Outcome};

    const FIXTURE: &str = "shared://🕸️web-mercator-tile-oracle/🔣️.json";
    const WORLD_TOL: f64 = 1e-9;

    fn fixture(ctx: &Context) -> Result<Json, String> {
        ctx.fixture_json(FIXTURE)
    }

    fn number(value: f64) -> Json {
        Json::Number(value)
    }

    fn row(entries: Vec<(&str, Json)>) -> Json {
        Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn answer(key: &str, rows: Vec<Json>) -> Result<Outcome, String> {
        let projection = row(vec![(key, Json::Array(rows))]);
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    fn field(entry: &Json, key: &str) -> Result<f64, String> {
        match entry.get(key) {
            Some(Json::Number(value)) => Ok(*value),
            _ => Err(format!("fixture entry {} carries no numeric {key:?}", entry.to_string())),
        }
    }

    fn law(id: &str, holds: Result<(), String>) -> Result<Outcome, String> {
        holds?;
        let projection = row(vec![("law", Json::String(id.to_string())), ("holds", Json::Bool(true))]);
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    pub fn projection(ctx: &Context) -> Result<Outcome, String> {
        let rows = fixture(ctx)?
            .array("projection")
            .iter()
            .map(|entry| {
                let world = lonlat_to_world(field(entry, "lon")?, field(entry, "lat")?);
                Ok(row(vec![("id", Json::String(entry.str("id"))), ("worldX", number(world.x)), ("worldY", number(world.y))]))
            })
            .collect::<Result<Vec<Json>, String>>()?;
        answer("points", rows)
    }

    /// 🧭️ The tile this repository's windowing selects at a point: `visible_tiles` over a sub-pixel viewport centred
    /// there, narrowed to the one tile whose rectangle owns the point under the XYZ convention (west and north edges
    /// inclusive), which is how a point on a shared edge resolves to a single tile number.
    pub fn tile_numbering(ctx: &Context) -> Result<Outcome, String> {
        let rows = fixture(ctx)?
            .array("tileNumbering")
            .iter()
            .map(|entry| {
                let (lon, lat, z) = (field(entry, "lon")?, field(entry, "lat")?, field(entry, "z")? as u32);
                let world = lonlat_to_world(lon, lat);
                let candidates = visible_tiles(&Camera { x: world.x, y: -world.y, zoom: 1e15 }, &Viewport { width: 2, height: 2, dpr: 1.0 }, z);
                let owner = candidates
                    .iter()
                    .find(|(_, x, y)| {
                        let rect = tile_world_rect(z, *x, *y);
                        rect.x0() <= world.x && world.x < rect.x1() && rect.y0() < world.y && world.y <= rect.y1()
                    })
                    .ok_or_else(|| format!("{}: no selected tile of {candidates:?} owns the point", entry.str("id")))?;
                Ok(row(vec![("id", Json::String(entry.str("id"))), ("z", number(f64::from(z))), ("x", number(f64::from(owner.1))), ("y", number(f64::from(owner.2)))]))
            })
            .collect::<Result<Vec<Json>, String>>()?;
        answer("tiles", rows)
    }

    pub fn tile_bounds(ctx: &Context) -> Result<Outcome, String> {
        let rows = fixture(ctx)?
            .array("tileBounds")
            .iter()
            .map(|entry| {
                let rect = tile_world_rect(field(entry, "z")? as u32, field(entry, "x")? as u32, field(entry, "y")? as u32);
                let (west, south) = world_to_lonlat(rect.x0(), rect.y0());
                let (east, north) = world_to_lonlat(rect.x1(), rect.y1());
                Ok(row(vec![("id", Json::String(entry.str("id"))), ("west", number(west)), ("south", number(south)), ("east", number(east)), ("north", number(north))]))
            })
            .collect::<Result<Vec<Json>, String>>()?;
        answer("bounds", rows)
    }

    pub fn lod_bands(ctx: &Context) -> Result<Outcome, String> {
        let rows = fixture(ctx)?
            .array("lodBands")
            .iter()
            .map(|entry| {
                let span = field(entry, "spanDeg")?;
                let (index, tile_z) = map_lod_band(span);
                Ok(row(vec![("spanDeg", number(span)), ("lodIndex", number(index as f64)), ("tileZ", number(f64::from(tile_z)))]))
            })
            .collect::<Result<Vec<Json>, String>>()?;
        answer("bands", rows)
    }

    fn host(zoom: f64) -> MapHost {
        let mut host = MapHost::new();
        host.set_size(800, 600, 1.0);
        host.set_camera(0.02, -0.01, zoom);
        host
    }

    pub fn cursor_anchored_zoom(_ctx: &Context) -> Result<Outcome, String> {
        let mut host = host(5_000.0);
        let (cx, cy) = (300.0, 220.0);
        let before = screen_to_world(&host.camera, &host.viewport, Point::new(cx, cy));
        host.wheel_screen(cx, cy, -100.0);
        let after = screen_to_world(&host.camera, &host.viewport, Point::new(cx, cy));
        law("cursor-anchored-zoom-invariant", if (before.x - after.x).abs() < WORLD_TOL && (before.y - after.y).abs() < WORLD_TOL { Ok(()) } else { Err(format!("the world point under the cursor moved from ({}, {}) to ({}, {})", before.x, before.y, after.x, after.y)) })
    }

    pub fn pan_round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let mut host = host(5_000.0);
        let origin = (host.camera.x, host.camera.y);
        let (sx, sy, dx, dy) = (400.0, 300.0, 57.0, -33.0);
        host.pointer_down_screen(sx, sy, 1);
        host.pointer_move_screen(sx + dx, sy + dy);
        host.pointer_up_screen(sx + dx, sy + dy);
        let moved = (host.camera.x - origin.0).abs() > 1e-6;
        host.pointer_down_screen(sx + dx, sy + dy, 1);
        host.pointer_move_screen(sx, sy);
        host.pointer_up_screen(sx, sy);
        let returned = (host.camera.x - origin.0).abs() < WORLD_TOL && (host.camera.y - origin.1).abs() < WORLD_TOL;
        law("pan-round-trip-invariant", if moved && returned { Ok(()) } else { Err(format!("pan moved={moved}, camera ({}, {}) against the original ({}, {})", host.camera.x, host.camera.y, origin.0, origin.1)) })
    }

    pub fn zoom_round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let mut host = host(5_000.0);
        let original = host.camera.zoom;
        host.wheel_screen(400.0, 300.0, -100.0);
        let zoomed = (host.camera.zoom - original).abs() > 1e-6;
        host.wheel_screen(400.0, 300.0, 100.0);
        let relative = (host.camera.zoom - original).abs() / original;
        law("zoom-round-trip-invariant", if zoomed && relative < WORLD_TOL { Ok(()) } else { Err(format!("zoom changed={zoomed}, round trip left a relative error of {relative}")) })
    }

    pub fn visible_tile_budget(_ctx: &Context) -> Result<Outcome, String> {
        for zoom in [MAP_CAMERA_ZOOM_MIN, 5_000.0, 500_000.0, 100_000_000.0] {
            let mut host = MapHost::new();
            host.set_size(1920, 1080, 2.0);
            host.set_camera(0.0, 0.0, zoom);
            host.fit_world_camera();
            for z in [host.pick_raster_tile_zoom(), host.pick_vector_tile_zoom()] {
                let count = visible_tiles(&host.camera, &host.viewport, z).len();
                if count > MAX_VISIBLE_TILE_REQUESTS {
                    return law("visible-tile-count-bound", Err(format!("camera zoom {zoom} selects {count} tiles at z={z}, over the budget {MAX_VISIBLE_TILE_REQUESTS}")));
                }
            }
        }
        law("visible-tile-count-bound", Ok(()))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, one handler per scenario id. The subject half is
/// `sut`-gated so the oracle-only build never links the surface crate.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built
        .subject("lonlat-world-round-trip", subject::projection)
        .subject("tile-numbering", subject::tile_numbering)
        .subject("tile-bounds", subject::tile_bounds)
        .subject("lod-band-selection", subject::lod_bands)
        .subject("cursor-anchored-zoom-invariant", subject::cursor_anchored_zoom)
        .subject("pan-round-trip-invariant", subject::pan_round_trip)
        .subject("zoom-round-trip-invariant", subject::zoom_round_trip)
        .subject("visible-tile-count-bound", subject::visible_tile_budget);
    built
}
//#endregion 🔖️Registration
