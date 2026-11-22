//! 🔺️ Shared north-first triangle topology for authored and streamed terrain.

use super::missing_samples::is_missing;

/// 🧩 Bounded grid geometry with optional canonical south-first missing ordinals.
#[derive(Clone, Copy)]
pub struct TerrainGridTopology<'a> {
    width: u32,
    height: u32,
    missing_samples: Option<&'a str>,
}

/// 📐 One half of a cell and the three vertices in renderer winding order.
#[derive(Clone, Copy)]
pub struct TerrainGridFace {
    pub vertices: [u32; 3],
    row: u32,
    column: u32,
    half: usize,
}

impl TerrainGridFace {
    /// ⚖️ Interpolates in north-first grid coordinates on this face.
    pub fn weights(self, [x, y]: [f64; 2]) -> [f64; 3] {
        let u = x - f64::from(self.column);
        let v = y - f64::from(self.row);
        if self.half == 0 { [1.0 - u - v, v, u] } else { [1.0 - v, 1.0 - u, u + v - 1.0] }
    }
}

impl<'a> TerrainGridTopology<'a> {
    /// 🗺️ Uses admitted dimensions and a validated authored mask.
    pub fn new([width, height]: [u32; 2], missing_samples: Option<&'a str>) -> Self {
        assert!((2..=65).contains(&width) && (2..=65).contains(&height));
        Self { width, height, missing_samples }
    }

    /// 🕳️ Reports whether validity topology is present.
    pub fn has_missing(self) -> bool { self.missing_samples.is_some() }

    /// 🧭 Converts one north-first vertex ID to a canonical south-first sample.
    pub fn sample(self, vertex: u32) -> [u32; 2] {
        [vertex % self.width, self.height - 1 - vertex / self.width]
    }

    /// 🎯 Tests a north-first row and column for authored data.
    pub fn is_present(self, row: u32, column: u32) -> bool {
        row < self.height && column < self.width
            && !is_missing(self.missing_samples, ((self.height - 1 - row) * self.width + column) as usize)
    }

    /// 📏 Returns the fixed two faces of a cell in winding and emission order.
    pub(super) fn cell_vertices(width: u32, row: u32, column: u32) -> [[u32; 3]; 2] {
        let i = row * width + column;
        [[i, i + width, i + 1], [i + 1, i + width, i + width + 1]]
    }

    /// 🔺 Returns each face with three valid vertices.
    pub fn cell_faces(self, row: u32, column: u32) -> [Option<TerrainGridFace>; 2] {
        let vertices = Self::cell_vertices(self.width, row, column);
        std::array::from_fn(|half| {
            let vertices = vertices[half];
            (!self.has_missing() || vertices.into_iter().all(|vertex| self.is_present(vertex / self.width, vertex % self.width)))
                .then_some(TerrainGridFace { vertices, row, column, half })
        })
    }

    /// 🧱 Visits supported faces in the original renderer's row, cell and diagonal order.
    pub fn faces(self) -> impl Iterator<Item = TerrainGridFace> + 'a {
        (0..self.height - 1).flat_map(move |row| {
            (0..self.width - 1).flat_map(move |column| self.cell_faces(row, column).into_iter().flatten())
        })
    }

    /// 🔗 Checks whether this vertex participates in any supported face.
    pub fn vertex_has_face(self, row: u32, column: u32) -> bool {
        if !self.has_missing() { return true; }
        if !self.is_present(row, column) { return false; }
        let vertex = row * self.width + column;
        (row.saturating_sub(1)..=row.min(self.height - 2)).any(|cell_row| {
            (column.saturating_sub(1)..=column.min(self.width - 2)).any(|cell_column| {
                self.cell_faces(cell_row, cell_column).into_iter().flatten().any(|face| face.vertices.contains(&vertex))
            })
        })
    }

    /// 🧭 Finds a supported closed face, including either side of a grid line or cell diagonal.
    pub fn face_at(self, [x, y]: [f64; 2]) -> Option<TerrainGridFace> {
        const TOLERANCE: f64 = 1.0e-11;
        let cells = |value: f64, extent: u32| {
            let limit = f64::from(extent - 1);
            let index = |value: f64| value.floor().clamp(0.0, limit - 1.0) as u32;
            index(value - TOLERANCE)..=index(value + TOLERANCE)
        };
        if !(-TOLERANCE..=f64::from(self.width - 1) + TOLERANCE).contains(&x)
            || !(-TOLERANCE..=f64::from(self.height - 1) + TOLERANCE).contains(&y) { return None; }
        for row in cells(y, self.height) {
            for column in cells(x, self.width) {
                if let Some(face) = self.cell_faces(row, column).into_iter().flatten()
                    .find(|face| face.weights([x, y]).into_iter().all(|weight| weight >= -TOLERANCE)) {
                    return Some(face);
                }
            }
        }
        None
    }
}
