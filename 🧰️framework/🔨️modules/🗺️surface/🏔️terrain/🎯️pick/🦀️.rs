//! 🎯 Ray queries against an editable elevation grid.

use super::{projection, TerrainGridTopology, TerrainSessionCore, ELEVATION_GRID_TILE};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};

#[derive(semio_framework_value::FromValue)]
#[value(deny_unknown_fields)]
struct TerrainGridRay {
    origin: [f64; 3],
    direction: [f64; 3],
}

/// ⛰️ World position, WGS84 longitude and latitude, and nearest `[column from west, row from south]`.
#[derive(Debug, PartialEq, semio_framework_value::ToValue)]
pub struct TerrainGridHit {
    pub position: [f64; 3],
    pub coordinates: [f64; 2],
    pub sample: [u32; 2],
}

impl TerrainSessionCore {
    /// 🌄 Returns the nearest forward intersection with the embedded grid's rendered triangles.
    pub fn pick_elevation_grid(&self, origin: [f64; 3], direction: [f64; 3]) -> Option<TerrainGridHit> {
        let grid = self.elevation_grid.as_ref()?;
        if !origin.into_iter().chain(direction).all(f64::is_finite) {
            return None;
        }
        let scale = direction.into_iter().map(f64::abs).fold(0.0, f64::max);
        if scale == 0.0 {
            return None;
        }
        let direction = direction.map(|component| component / scale);
        let [west, south, east, north] = grid.bounds;
        let lower = projection::lonlat_to_local_meters(west, south, self.origin_lon, self.origin_lat);
        let upper = projection::lonlat_to_local_meters(east, north, self.origin_lon, self.origin_lat);
        let mut entrance: f64 = 0.0;
        let mut exit = f64::INFINITY;
        for (axis, (lower, upper)) in [(lower.0, upper.0), (lower.1, upper.1)].into_iter().enumerate() {
            let lower = f64::from(lower as f32);
            let upper = f64::from(upper as f32);
            if direction[axis] == 0.0 {
                if !(lower..=upper).contains(&origin[axis]) {
                    return None;
                }
            } else {
                let start = (lower - origin[axis]) / direction[axis];
                let end = (upper - origin[axis]) / direction[axis];
                entrance = entrance.max(start.min(end));
                exit = exit.min(start.max(end));
                if entrance > exit {
                    return None;
                }
            }
        }
        let (z, x, y) = ELEVATION_GRID_TILE;
        let mesh = self.tile_mesh(z, x, y)?;
        let vertex = |index: u32| {
            std::array::from_fn(|axis| f64::from(mesh.positions[index as usize * 3 + axis]))
        };
        let (distance, face) = mesh.indices.as_chunks::<3>().0.iter().filter_map(|&face| {
            let [a, b, c] = face.map(vertex);
            triangle_hit(origin, direction, a, b, c).map(|distance| (distance, face))
        }).min_by(|(first, _), (second, _)| first.total_cmp(second))?;
        let position = std::array::from_fn(|axis| direction[axis].mul_add(distance, origin[axis]));
        let (lon, lat) = projection::local_meters_to_lonlat(position[0], position[1], self.origin_lon, self.origin_lat);
        let [width, height] = grid.dimensions()?;
        let sample_index = |fraction: f64, extent| {
            let cells = f64::from(extent - 1);
            (fraction * cells).round().clamp(0.0, cells) as u32
        };
        let fractions = [(lon - west) / (east - west), (lat - south) / (north - south)];
        let sample = if grid.missing_samples.is_none() {
            [sample_index(fractions[0], width), sample_index(fractions[1], height)]
        } else {
            let topology = TerrainGridTopology::new([width, height], grid.missing_samples.as_deref());
            let x = fractions[0] * f64::from(width - 1);
            let y = fractions[1] * f64::from(height - 1);
            let distance = |[column, row]: [u32; 2]| {
                (x - f64::from(column)).powi(2) + (y - f64::from(row)).powi(2)
            };
            face.map(|vertex| topology.sample(vertex)).into_iter()
                .min_by(|&a, &b| distance(a).total_cmp(&distance(b)).then_with(|| a.cmp(&b)))?
        };
        Some(TerrainGridHit { position, coordinates: [lon.clamp(west, east), lat.clamp(south, north)], sample })
    }

    /// 🌐 Decodes a browser ray and publishes the same geographic hit as native hosts.
    pub fn pick_elevation_grid_json(&self, ray_json: &str) -> String {
        let hit = from_json_str::<TerrainGridRay>(ray_json, JsonMemberPolicy::Reject)
            .ok().and_then(|ray| self.pick_elevation_grid(ray.origin, ray.direction));
        hit.map_or_else(|| "null".to_owned(), |hit| to_json_string(&hit))
    }
}

fn triangle_hit(origin: [f64; 3], direction: [f64; 3], a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Option<f64> {
    let subtract = |left: [f64; 3], right: [f64; 3]| std::array::from_fn(|axis| left[axis] - right[axis]);
    let dot = |left: [f64; 3], right: [f64; 3]| left[0] * right[0] + left[1] * right[1] + left[2] * right[2];
    let cross = |left: [f64; 3], right: [f64; 3]| [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ];
    let ab = subtract(b, a);
    let ac = subtract(c, a);
    let perpendicular = cross(direction, ac);
    let determinant = dot(ab, perpendicular);
    if determinant == 0.0 {
        return None;
    }
    let offset = subtract(origin, a);
    let u = dot(offset, perpendicular) / determinant;
    let transverse = cross(offset, ab);
    let v = dot(direction, transverse) / determinant;
    let tolerance = 4.0 * f64::from(f32::EPSILON);
    if u < -tolerance || v < -tolerance || u + v > 1.0 + tolerance {
        return None;
    }
    let distance = dot(ac, transverse) / determinant;
    (distance.is_finite() && distance >= 0.0).then_some(distance)
}
