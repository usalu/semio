use super::{TerrainGridTopology, normalize3, projection};
/// 🎯️ Projects one sample of a rectangular elevation grid in the renderer's local coordinates.
#[derive(Clone, Copy)]
pub struct TerrainGridMeshProjection {
    bounds: [f64; 4],
    origin: [f64; 2],
    exaggeration: f64,
    cells: [u32; 2],
    cell_size: [f64; 2],
}

impl TerrainGridMeshProjection {
    pub fn from_bounds(bounds: [f64; 4], origin: [f64; 2], exaggeration: f64, [width, height]: [u32; 2]) -> Self {
        let [west, south, east, north] = bounds;
        let cells = [width - 1, height - 1];
        let (west_x, south_y) = projection::lonlat_to_local_meters(west, south, origin[0], origin[1]);
        let (east_x, north_y) = projection::lonlat_to_local_meters(east, north, origin[0], origin[1]);
        let (span_x, span_y) = (east_x - west_x, north_y - south_y);
        let cell_size = [(span_x / f64::from(cells[0])).abs().max(1e-3), (span_y / f64::from(cells[1])).abs().max(1e-3)];
        Self { bounds, origin, exaggeration, cells, cell_size }
    }

    pub fn local_position(&self, row: u32, column: u32) -> [f32; 2] {
        let [west, south, east, north] = self.bounds;
        let u = f64::from(column) / f64::from(self.cells[0]);
        let v = f64::from(row) / f64::from(self.cells[1]);
        let lon = west + (east - west) * u;
        let lat = north + (south - north) * v;
        let (x, y) = projection::lonlat_to_local_meters(lon, lat, self.origin[0], self.origin[1]);
        [x as f32, y as f32]
    }

    pub fn position(&self, row: u32, column: u32, height: f64) -> [f32; 3] {
        let [x, y] = self.local_position(row, column);
        [x, y, (height * self.exaggeration) as f32]
    }

    pub fn normal(&self, row: u32, column: u32, height: impl Fn(u32, u32) -> f64) -> [f32; 3] {
        self.normal_present(row, column, |row, column| Some(height(row, column)))
    }

    /// ⛰️ Uses a one-sided derivative when an adjacent sample is absent.
    pub fn normal_present(&self, row: u32, column: u32, height: impl Fn(u32, u32) -> Option<f64>) -> [f32; 3] {
        let Some(center) = height(row, column) else { return [0.0, 0.0, 1.0]; };
        let west = column.saturating_sub(1);
        let east = (column + 1).min(self.cells[0]);
        let north = row.saturating_sub(1);
        let south = (row + 1).min(self.cells[1]);
        let rise = |(lower, before): (u32, Option<f64>), (upper, after): (u32, Option<f64>), coordinate: u32, scale| {
            let before_distance = if before.is_some() { coordinate.abs_diff(lower) } else { 0 };
            let after_distance = if after.is_some() { coordinate.abs_diff(upper) } else { 0 };
            let distance = before_distance + after_distance;
            if distance == 0 { return 0.0; }
            (after.unwrap_or(center) - before.unwrap_or(center)) * self.exaggeration / (f64::from(distance) * scale)
        };
        let x = rise((west, height(row, west)), (east, height(row, east)), column, self.cell_size[0]);
        let y = rise((south, height(south, column)), (north, height(north, column)), row, self.cell_size[1]);
        let (x, y, z) = normalize3(-x, -y, 1.0);
        [x as f32, y as f32, z as f32]
    }

    pub fn uv(height: f64, minimum: f64, maximum: f64) -> [f32; 2] {
        let range = if maximum > minimum { maximum - minimum } else { 1.0 };
        [0.5, ((height - minimum) / range).clamp(0.0, 1.0) as f32]
    }

    pub fn triangles(&self, row: u32, column: u32) -> [u32; 6] {
        let [first, second] = TerrainGridTopology::cell_vertices(self.cells[0] + 1, row, column);
        [first[0], first[1], first[2], second[0], second[1], second[2]]
    }
}
