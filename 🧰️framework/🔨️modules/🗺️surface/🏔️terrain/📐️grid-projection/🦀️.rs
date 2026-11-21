use super::{normalize3, projection, TERRAIN_GRID_RESOLUTION};
/// 🎯️ Projects one sample of a rectangular elevation grid in the renderer's local coordinates.
#[derive(Clone, Copy)]
pub struct TerrainGridMeshProjection {
    bounds: [f64; 4],
    origin: [f64; 2],
    exaggeration: f64,
    cell_size: [f32; 2],
}

impl TerrainGridMeshProjection {
    pub fn from_bounds(bounds: [f64; 4], origin: [f64; 2], exaggeration: f64) -> Self {
        let mut projection = Self { bounds, origin, exaggeration, cell_size: [0.0; 2] };
        let [x, y] = projection.local_position(0, 0);
        projection.cell_size = [
            (projection.local_position(0, 1)[0] - x).abs().max(1e-3),
            (y - projection.local_position(1, 0)[1]).abs().max(1e-3),
        ];
        projection
    }

    pub fn local_position(&self, row: u32, column: u32) -> [f32; 2] {
        let [west, south, east, north] = self.bounds;
        let u = f64::from(column) / f64::from(TERRAIN_GRID_RESOLUTION - 1);
        let v = f64::from(row) / f64::from(TERRAIN_GRID_RESOLUTION - 1);
        let lon = west + (east - west) * u;
        let lat = north + (south - north) * v;
        let (x, y) = projection::lonlat_to_local_meters(lon, lat, self.origin[0], self.origin[1]);
        [x as f32, y as f32]
    }

    pub fn position(&self, row: u32, column: u32, height: f32) -> [f32; 3] {
        let [x, y] = self.local_position(row, column);
        [x, y, (f64::from(height) * self.exaggeration) as f32]
    }

    pub fn normal(&self, row: u32, column: u32, height: impl Fn(u32, u32) -> f32) -> [f32; 3] {
        let west = column.saturating_sub(1);
        let east = (column + 1).min(TERRAIN_GRID_RESOLUTION - 1);
        let north = row.saturating_sub(1);
        let south = (row + 1).min(TERRAIN_GRID_RESOLUTION - 1);
        let x = f64::from(height(row, east) - height(row, west)) * self.exaggeration
            / (f64::from(east - west) * f64::from(self.cell_size[0]));
        let y = f64::from(height(north, column) - height(south, column)) * self.exaggeration
            / (f64::from(south - north) * f64::from(self.cell_size[1]));
        let (x, y, z) = normalize3(-x, -y, 1.0);
        [x as f32, y as f32, z as f32]
    }

    pub fn uv(height: f32, minimum: f32, maximum: f32) -> [f32; 2] {
        let range = if maximum > minimum { maximum - minimum } else { 1.0 };
        [0.5, ((height - minimum) / range).clamp(0.0, 1.0)]
    }

    pub fn triangles(&self, row: u32, column: u32) -> [u32; 6] {
        let i = row * TERRAIN_GRID_RESOLUTION + column;
        [i, i + TERRAIN_GRID_RESOLUTION, i + 1, i + 1, i + TERRAIN_GRID_RESOLUTION, i + TERRAIN_GRID_RESOLUTION + 1]
    }
}
