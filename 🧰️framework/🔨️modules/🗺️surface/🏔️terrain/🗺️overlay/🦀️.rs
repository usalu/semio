//! 🗺️ Projects geographic annotations onto the triangles of a streamed elevation tile.

use super::{TerrainGridFace, TerrainGridTopology, TerrainSessionCore, ELEVATION_GRID_TILE, TERRAIN_GRID_RESOLUTION};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};
use semio_framework_value::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, FromValue, ToValue)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct TerrainOverlays {
    pub positions: Vec<TerrainOverlayPosition>,
    pub routes: Vec<TerrainOverlayRoute>,
    pub regions: Vec<TerrainOverlayRegion>,
}

#[derive(Clone, Debug, PartialEq, FromValue, ToValue)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct TerrainOverlayPosition {
    pub id: String,
    pub name: String,
    pub coordinates: [f64; 2],
    pub elevation: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, FromValue, ToValue)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct TerrainOverlayRoute {
    pub id: String,
    pub name: String,
    pub coordinates: Vec<[f64; 2]>,
}

#[derive(Clone, Debug, PartialEq, FromValue, ToValue)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct TerrainOverlayRegion {
    pub id: String,
    pub name: String,
    pub rings: Vec<Vec<[f64; 2]>>,
}

#[derive(ToValue)]
pub struct TerrainOverlayTile {
    pub positions: Vec<TerrainOverlayMarker>,
    pub routes: Vec<TerrainOverlayPath>,
    pub regions: Vec<TerrainOverlayPath>,
}

#[derive(ToValue)]
pub struct TerrainOverlayMarker {
    pub id: String,
    pub name: String,
    pub point: [f32; 3],
}

#[derive(ToValue)]
pub struct TerrainOverlayPath {
    pub id: String,
    pub name: String,
    pub segments: Vec<[[f32; 3]; 2]>,
}

#[cfg(test)]
const GRID: usize = TERRAIN_GRID_RESOLUTION as usize;

struct OverlayGrid<'a> {
    positions: &'a [f32],
    origin_lon: f64,
    origin_lat: f64,
    inline_bounds: Option<[f64; 4]>,
    topology: TerrainGridTopology<'a>,
    cells: [f64; 2],
}

impl OverlayGrid<'_> {
    /// 📐️ The authored grid's local-meter mesh is affine in WGS84 degrees. Normalize from its
    /// original bounds so a rounded mesh corner cannot move a feature across the grid boundary.
    fn coordinates(&self, [lon, lat]: [f64; 2]) -> [f64; 2] {
        let [columns, rows] = self.cells;
        if let Some([west, south, east, north]) = self.inline_bounds {
            return [(lon - west) / (east - west) * columns, (lat - north) / (south - north) * rows];
        }
        let (x, y) = super::projection::lonlat_to_local_meters(lon, lat, self.origin_lon, self.origin_lat);
        let first = self.positions.first_chunk::<3>().unwrap();
        let last = self.positions.last_chunk::<3>().unwrap();
        [
            (x - f64::from(first[0])) / (f64::from(last[0]) - f64::from(first[0])) * columns,
            (y - f64::from(first[1])) / (f64::from(last[1]) - f64::from(first[1])) * rows,
        ]
    }

    fn contains_position(&self, [lon, lat]: [f64; 2], coordinate: [f64; 2]) -> bool {
        if let Some([west, south, east, north]) = self.inline_bounds {
            return (west..=east).contains(&lon) && (south..=north).contains(&lat);
        }
        coordinate.into_iter().zip(self.cells).all(|(axis, extent)| (0.0..extent).contains(&axis))
    }

    fn point_on_face(&self, face: TerrainGridFace, coordinate: [f64; 2]) -> [f32; 3] {
        std::array::from_fn(|axis| {
            face.vertices.into_iter().zip(face.weights(coordinate)).map(|(vertex, weight)| {
                weight * f64::from(self.positions[3 * vertex as usize + axis])
            }).sum::<f64>() as f32
        })
    }

    fn point(&self, coordinate: [f64; 2]) -> Option<[f32; 3]> {
        let face = self.topology.face_at(coordinate)?;
        Some(self.point_on_face(face, coordinate))
    }

    fn segments(&self, path: &[[f64; 2]], closed: bool, output: &mut Vec<[[f32; 3]; 2]>) {
        let pairs = path.windows(2).map(|pair| (pair[0], pair[1]));
        let closing = path.first().zip(path.last()).filter(|(first, last)| closed && first != last).map(|(first, last)| (*last, *first));
        for (start, end) in pairs.chain(closing) {
            #[allow(clippy::float_cmp)]
            if start == end {
                continue;
            }
            let start = self.coordinates(start);
            let end = self.coordinates(end);
            let delta = [end[0] - start[0], end[1] - start[1]];
            let Some([enter, leave]) = clip_segment(start, delta, self.cells) else {
                continue;
            };
            let at = |t: f64| [start[0] + delta[0] * t, start[1] + delta[1] * t];
            let mut stops = vec![enter, leave];
            let [columns, rows] = self.cells;
            for (offset, slope, extent) in [(start[0], delta[0], columns), (start[1], delta[1], rows), (start[0] + start[1], delta[0] + delta[1], columns + rows)] {
                if !offset.is_finite() || !slope.is_finite() || slope == 0.0 {
                    continue;
                }
                let a = slope.mul_add(enter, offset);
                let b = slope.mul_add(leave, offset);
                let first = a.min(b).ceil().max(0.0) as i32;
                let last = a.max(b).floor().min(extent) as i32;
                stops.extend((first..=last).map(|k| (f64::from(k) - offset) / slope).filter(|&t| t > enter && t < leave));
            }
            stops.sort_unstable_by(f64::total_cmp);
            stops.dedup_by(|a, b| (*a - *b).abs() <= 1e-12);
            for pair in stops.windows(2) {
                let [begin, end] = [pair[0], pair[1]];
                let Some(face) = self.topology.face_at(at(begin.midpoint(end))) else { continue; };
                output.push([self.point_on_face(face, at(begin)), self.point_on_face(face, at(end))]);
            }
        }
    }
}

fn clip_segment(start: [f64; 2], delta: [f64; 2], cells: [f64; 2]) -> Option<[f64; 2]> {
    if !start.into_iter().chain(delta).all(f64::is_finite) {
        return None;
    }
    let (mut enter, mut leave) = (0.0_f64, 1.0_f64);
    for ((coordinate, movement), extent) in start.into_iter().zip(delta).zip(cells) {
        if movement == 0.0 {
            if !(0.0..=extent).contains(&coordinate) {
                return None;
            }
            continue;
        }
        let a = -coordinate / movement;
        let b = (extent - coordinate) / movement;
        enter = enter.max(a.min(b));
        leave = leave.min(a.max(b));
    }
    (enter < leave).then_some([enter, leave])
}

impl TerrainSessionCore {
    fn overlay_grid<'a>(&'a self, positions: &'a [f32]) -> Option<OverlayGrid<'a>> {
        let (inline_bounds, dimensions, missing) = match &self.elevation_grid {
            Some(grid) => (Some(grid.bounds), grid.dimensions()?, grid.missing_samples.as_deref()),
            None => (None, [TERRAIN_GRID_RESOLUTION; 2], None),
        };
        Some(OverlayGrid {
            positions,
            origin_lon: self.origin_lon,
            origin_lat: self.origin_lat,
            inline_bounds,
            topology: TerrainGridTopology::new(dimensions, missing),
            cells: dimensions.map(|samples| f64::from(samples - 1)),
        })
    }

    /// 🧭 Sample the project's origin if this tile contains it.
    pub fn terrain_origin_elevation(&self, z: u32, x: u32, y: u32) -> Option<f32> {
        if let Some(grid) = &self.elevation_grid {
            let [west, south, east, north] = grid.bounds;
            if (z, x, y) != ELEVATION_GRID_TILE || !(west..=east).contains(&self.origin_lon) || !(south..=north).contains(&self.origin_lat) {
                return None;
            }
        } else {
            let (tile_x, tile_y) = super::projection::lonlat_to_tile_xy(self.origin_lon, self.origin_lat, z);
            if tile_x.floor() != f64::from(x) || tile_y.floor() != f64::from(y) {
                return None;
            }
        }
        let mesh = self.tile_mesh(z, x, y)?;
        let grid = self.overlay_grid(&mesh.positions)?;
        grid.point(grid.coordinates([self.origin_lon, self.origin_lat])).map(|point| point[2])
    }

    /// 📍 Drape features on this DEM tile using the same triangles shown by the renderer.
    pub fn overlay_tile(&self, z: u32, x: u32, y: u32, overlays: &TerrainOverlays) -> Option<TerrainOverlayTile> {
        let mesh = self.tile_mesh(z, x, y)?;
        let grid = self.overlay_grid(&mesh.positions)?;
        let positions = overlays.positions.iter().filter_map(|feature| {
            let coordinate = grid.coordinates(feature.coordinates);
            if !grid.contains_position(feature.coordinates, coordinate) {
                return None;
            }
            let mut point = grid.point(coordinate)?;
            if let Some(elevation) = feature.elevation {
                point[2] = point[2].max((elevation * self.exaggeration) as f32);
            }
            Some(TerrainOverlayMarker { id: feature.id.clone(), name: feature.name.clone(), point })
        }).collect();
        let routes = overlays.routes.iter().filter_map(|feature| {
            let mut segments = Vec::new();
            grid.segments(&feature.coordinates, false, &mut segments);
            (!segments.is_empty()).then(|| TerrainOverlayPath { id: feature.id.clone(), name: feature.name.clone(), segments })
        }).collect();
        let regions = overlays.regions.iter().filter_map(|feature| {
            let mut segments = Vec::new();
            for ring in &feature.rings {
                grid.segments(ring, true, &mut segments);
            }
            (!segments.is_empty()).then(|| TerrainOverlayPath { id: feature.id.clone(), name: feature.name.clone(), segments })
        }).collect();
        Some(TerrainOverlayTile { positions, routes, regions })
    }

    /// 🌐 Compute a loaded tile's geographic overlays across the browser WASM boundary.
    pub fn terrain_overlay_tile_json(&self, z: u32, x: u32, y: u32, overlays_json: &str) -> String {
        from_json_str::<TerrainOverlays>(overlays_json, JsonMemberPolicy::Reject).ok()
            .and_then(|overlays| self.overlay_tile(z, x, y, &overlays))
            .map_or_else(|| "null".to_owned(), |overlays| to_json_string(&overlays))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
