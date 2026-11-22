//! 🌅️ Mesh shadings (types 4 to 7) decoded into Gouraud triangles.
//! [ISO 32000-1 §8.7.4.5.5 to §8.7.4.5.8](https://opensource.adobe.com/dc-acrobat-sdk-docs/pdfstandards/PDF32000_2008.pdf)

use super::{Control, RasterError};

const MAX_COMPONENTS: usize = 4;
const PATCH_DIVISIONS: usize = 12;
const MAX_TRIANGLES: usize = 4_000_000;

/// 🎛️ The mesh shading dictionary entries needed to read the vertex stream.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshParameters {
    pub shading_type: u32,
    pub bits_per_coordinate: u32,
    pub bits_per_component: u32,
    pub bits_per_flag: u32,
    pub vertices_per_row: u32,
    pub decode: Vec<f64>,
    pub components: usize,
    pub pad_vertices: bool,
}

/// 📍 A mesh vertex with its colour components (or the single function parameter).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub c: [f64; MAX_COMPONENTS],
}

struct Reader<'a> {
    data: &'a [u8],
    position: usize,
    parameters: &'a MeshParameters,
}

impl Reader<'_> {
    fn bits(&mut self, count: u32) -> Option<u64> {
        if count == 0 || count > 32 || self.position + count as usize > self.data.len() * 8 { return None; }
        let mut value = 0u64;
        for _ in 0..count {
            value = value << 1 | u64::from(self.data[self.position >> 3] >> (7 - (self.position & 7)) & 1);
            self.position += 1;
        }
        Some(value)
    }

    fn align(&mut self) { self.position = self.position.div_ceil(8) * 8; }

    fn flag(&mut self) -> Option<u32> { self.bits(self.parameters.bits_per_flag).map(|flag| flag as u32 & 3) }

    fn point(&mut self) -> Option<(f64, f64)> {
        let bits = self.parameters.bits_per_coordinate;
        let maximum = ((1u64 << bits) - 1) as f64;
        let d = &self.parameters.decode;
        let (x, y) = (self.bits(bits)? as f64, self.bits(bits)? as f64);
        Some((d[0] + x * (d[1] - d[0]) / maximum, d[2] + y * (d[3] - d[2]) / maximum))
    }

    fn colour(&mut self) -> Option<[f64; MAX_COMPONENTS]> {
        let bits = self.parameters.bits_per_component;
        let maximum = ((1u64 << bits) - 1) as f64;
        let mut out = [0.0; MAX_COMPONENTS];
        for (channel, slot) in out.iter_mut().enumerate().take(self.parameters.components) {
            let (low, high) = (self.parameters.decode[4 + 2 * channel], self.parameters.decode[5 + 2 * channel]);
            *slot = low + self.bits(bits)? as f64 * (high - low) / maximum;
        }
        Some(out)
    }

    fn vertex(&mut self) -> Option<Vertex> {
        let (x, y) = self.point()?;
        Some(Vertex { x, y, c: self.colour()? })
    }

    fn remaining(&self) -> bool { self.position + 8 <= self.data.len() * 8 }
}

/// 🌅️ Decodes the vertex or patch stream into triangles in shading space.
pub fn triangles(data: &[u8], parameters: &MeshParameters, control: Control<'_>) -> Result<Vec<[Vertex; 3]>, RasterError> {
    let bits_ok = (1..=32).contains(&parameters.bits_per_coordinate) && (1..=16).contains(&parameters.bits_per_component);
    if !bits_ok || parameters.components == 0 || parameters.components > MAX_COMPONENTS || parameters.decode.len() < 4 + 2 * parameters.components {
        return Err(RasterError::Malformed("invalid mesh shading parameters"));
    }
    let mut reader = Reader { data, position: 0, parameters };
    let mut out = Vec::new();
    let result = match parameters.shading_type {
        4 => free_form(&mut reader, &mut out, control),
        5 => lattice(&mut reader, &mut out, control),
        6 | 7 => patches(&mut reader, &mut out, parameters.shading_type == 7, control),
        _ => return Err(RasterError::Unsupported("not a mesh shading type")),
    };
    match result {
        Err(RasterError::Cancelled) => Err(RasterError::Cancelled),
        _ => Ok(out),
    }
}

fn free_form(reader: &mut Reader<'_>, out: &mut Vec<[Vertex; 3]>, control: Control<'_>) -> Result<(), RasterError> {
    let mut last: Option<[Vertex; 3]> = None;
    while reader.remaining() {
        if out.len() % 256 == 0 && !control(reader.position as f32 / (reader.data.len() * 8) as f32) { return Err(RasterError::Cancelled); }
        if out.len() > MAX_TRIANGLES { return Err(RasterError::TooLarge); }
        let flag = reader.flag().ok_or(RasterError::Malformed("truncated mesh"))?;
        let first = reader.vertex().ok_or(RasterError::Malformed("truncated mesh"))?;
        if reader.parameters.pad_vertices { reader.align(); }
        let triangle = match (flag, last) {
            (1, Some([_, b, c])) => [b, c, first],
            (2, Some([a, _, c])) => [a, c, first],
            _ => {
                let mut vertices = [first; 3];
                for slot in vertices.iter_mut().skip(1) {
                    reader.flag().ok_or(RasterError::Malformed("truncated mesh"))?;
                    *slot = reader.vertex().ok_or(RasterError::Malformed("truncated mesh"))?;
                    if reader.parameters.pad_vertices { reader.align(); }
                }
                vertices
            }
        };
        out.push(triangle);
        last = Some(triangle);
    }
    Ok(())
}

fn lattice(reader: &mut Reader<'_>, out: &mut Vec<[Vertex; 3]>, control: Control<'_>) -> Result<(), RasterError> {
    let columns = reader.parameters.vertices_per_row as usize;
    if columns < 2 { return Err(RasterError::Malformed("lattice mesh needs two vertices per row")); }
    let mut previous: Vec<Vertex> = Vec::new();
    loop {
        if !control(reader.position as f32 / (reader.data.len() * 8).max(1) as f32) { return Err(RasterError::Cancelled); }
        let mut row = Vec::with_capacity(columns);
        for _ in 0..columns {
            match reader.vertex() { Some(vertex) => { row.push(vertex); if reader.parameters.pad_vertices { reader.align(); } } None => return Ok(()) }
        }
        if !previous.is_empty() {
            for i in 0..columns - 1 {
                out.push([previous[i], previous[i + 1], row[i]]);
                out.push([previous[i + 1], row[i + 1], row[i]]);
            }
        }
        if out.len() > MAX_TRIANGLES { return Err(RasterError::TooLarge); }
        previous = row;
    }
}

type Grid = [[(f64, f64); 4]; 4];

fn patches(reader: &mut Reader<'_>, out: &mut Vec<[Vertex; 3]>, tensor: bool, control: Control<'_>) -> Result<(), RasterError> {
    let mut previous: Option<(Grid, [[f64; MAX_COMPONENTS]; 4])> = None;
    while reader.remaining() {
        if !control(reader.position as f32 / (reader.data.len() * 8) as f32) { return Err(RasterError::Cancelled); }
        if out.len() > MAX_TRIANGLES { return Err(RasterError::TooLarge); }
        let Some(flag) = reader.flag() else { break };
        let (mut edge, mut colours): ([(f64, f64); 12], [[f64; MAX_COMPONENTS]; 4]) = ([(0.0, 0.0); 12], [[0.0; MAX_COMPONENTS]; 4]);
        let carried = if flag == 0 { 0 } else { 4 };
        if flag != 0 {
            let Some((grid, old)) = previous else { return Ok(()) };
            let (shared, shared_colours) = match flag {
                1 => ([grid[0][3], grid[1][3], grid[2][3], grid[3][3]], [old[1], old[2]]),
                2 => ([grid[3][3], grid[3][2], grid[3][1], grid[3][0]], [old[2], old[3]]),
                _ => ([grid[3][0], grid[2][0], grid[1][0], grid[0][0]], [old[3], old[0]]),
            };
            edge[..4].copy_from_slice(&shared);
            colours[0] = shared_colours[0];
            colours[1] = shared_colours[1];
        }
        let Some(()) = (|| {
            for slot in edge.iter_mut().skip(carried) { *slot = reader.point()?; }
            Some(())
        })() else { break };
        let mut inner = [(0.0, 0.0); 4];
        if tensor {
            let Some(()) = (|| { for slot in inner.iter_mut() { *slot = reader.point()?; } Some(()) })() else { break };
        }
        let first_colour = if flag == 0 { 0 } else { 2 };
        let Some(()) = (|| { for slot in colours.iter_mut().skip(first_colour) { *slot = reader.colour()?; } Some(()) })() else { break };
        reader.align();
        let mut grid: Grid = [[(0.0, 0.0); 4]; 4];
        let order = [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (3, 0), (2, 0), (1, 0)];
        for (&(i, j), &point) in order.iter().zip(&edge) { grid[i][j] = point; }
        if tensor {
            grid[1][1] = inner[0];
            grid[1][2] = inner[1];
            grid[2][2] = inner[2];
            grid[2][1] = inner[3];
        } else {
            let g = grid;
            let combine = |terms: [(f64, usize, usize); 9]| {
                let sum = |select: fn((f64, f64)) -> f64| terms.iter().map(|&(weight, i, j)| weight * select(g[i][j])).sum::<f64>() / 9.0;
                (sum(|p| p.0), sum(|p| p.1))
            };
            grid[1][1] = combine([(-4.0, 0, 0), (6.0, 0, 1), (6.0, 1, 0), (-2.0, 0, 3), (-2.0, 3, 0), (3.0, 3, 1), (3.0, 1, 3), (-1.0, 3, 3), (0.0, 0, 0)]);
            grid[1][2] = combine([(-4.0, 0, 3), (6.0, 0, 2), (6.0, 1, 3), (-2.0, 0, 0), (-2.0, 3, 3), (3.0, 3, 2), (3.0, 1, 0), (-1.0, 3, 0), (0.0, 0, 0)]);
            grid[2][1] = combine([(-4.0, 3, 0), (6.0, 3, 1), (6.0, 2, 0), (-2.0, 3, 3), (-2.0, 0, 0), (3.0, 0, 1), (3.0, 2, 3), (-1.0, 0, 3), (0.0, 0, 0)]);
            grid[2][2] = combine([(-4.0, 3, 3), (6.0, 3, 2), (6.0, 2, 3), (-2.0, 3, 0), (-2.0, 0, 3), (3.0, 0, 2), (3.0, 2, 0), (-1.0, 0, 0), (0.0, 0, 0)]);
        }
        subdivide(&grid, &colours, out);
        previous = Some((grid, colours));
    }
    Ok(())
}

fn bernstein(t: f64) -> [f64; 4] {
    let u = 1.0 - t;
    [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t]
}

fn subdivide(grid: &Grid, colours: &[[f64; MAX_COMPONENTS]; 4], out: &mut Vec<[Vertex; 3]>) {
    let n = PATCH_DIVISIONS;
    let sample = |iu: usize, iv: usize| {
        let (u, v) = (iu as f64 / n as f64, iv as f64 / n as f64);
        let (bu, bv) = (bernstein(u), bernstein(v));
        let (mut x, mut y) = (0.0, 0.0);
        for i in 0..4 { for j in 0..4 { x += grid[i][j].0 * bu[i] * bv[j]; y += grid[i][j].1 * bu[i] * bv[j]; } }
        let mut c = [0.0; MAX_COMPONENTS];
        for (channel, slot) in c.iter_mut().enumerate() {
            *slot = (1.0 - u) * (1.0 - v) * colours[0][channel] + (1.0 - u) * v * colours[1][channel] + u * v * colours[2][channel] + u * (1.0 - v) * colours[3][channel];
        }
        Vertex { x, y, c }
    };
    for iu in 0..n {
        for iv in 0..n {
            let (a, b, c, d) = (sample(iu, iv), sample(iu + 1, iv), sample(iu + 1, iv + 1), sample(iu, iv + 1));
            out.push([a, b, c]);
            out.push([a, c, d]);
        }
    }
}

/// 🖼️ Fills triangles into an RGBA raster, interpolating components at pixel centres and mapping them through `shade`.
/// `transform` maps shading space to raster pixels as `[a, b, c, d, e, f]`.
pub fn rasterize(triangles: &[[Vertex; 3]], transform: [f64; 6], width: u32, height: u32, shade: &dyn Fn(&[f64; MAX_COMPONENTS]) -> Option<[f64; 3]>, control: Control<'_>) -> Result<Vec<u8>, RasterError> {
    let mut pixels = vec![0u8; width as usize * height as usize * 4];
    let map = |vertex: &Vertex| (transform[0] * vertex.x + transform[2] * vertex.y + transform[4], transform[1] * vertex.x + transform[3] * vertex.y + transform[5]);
    for (index, triangle) in triangles.iter().enumerate() {
        if index % 4096 == 0 && !control(index as f32 / triangles.len() as f32) { return Err(RasterError::Cancelled); }
        let [p0, p1, p2] = [map(&triangle[0]), map(&triangle[1]), map(&triangle[2])];
        let area = (p1.0 - p0.0) * (p2.1 - p0.1) - (p2.0 - p0.0) * (p1.1 - p0.1);
        if area == 0.0 || !area.is_finite() { continue; }
        let (min_x, max_x) = (p0.0.min(p1.0).min(p2.0).floor().max(0.0) as i64, p0.0.max(p1.0).max(p2.0).ceil().min(f64::from(width)) as i64);
        let (min_y, max_y) = (p0.1.min(p1.1).min(p2.1).floor().max(0.0) as i64, p0.1.max(p1.1).max(p2.1).ceil().min(f64::from(height)) as i64);
        for y in min_y..max_y {
            for x in min_x..max_x {
                let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
                let w0 = ((p1.0 - px) * (p2.1 - py) - (p2.0 - px) * (p1.1 - py)) / area;
                let w1 = ((p2.0 - px) * (p0.1 - py) - (p0.0 - px) * (p2.1 - py)) / area;
                let w2 = 1.0 - w0 - w1;
                let epsilon = -1e-9;
                if w0 < epsilon || w1 < epsilon || w2 < epsilon { continue; }
                let mut components = [0.0; MAX_COMPONENTS];
                for (channel, slot) in components.iter_mut().enumerate() {
                    *slot = w0 * triangle[0].c[channel] + w1 * triangle[1].c[channel] + w2 * triangle[2].c[channel];
                }
                if let Some(rgb) = shade(&components) {
                    let at = (y as usize * width as usize + x as usize) * 4;
                    pixels[at..at + 4].copy_from_slice(&[rgb[0], rgb[1], rgb[2], 1.0].map(|value| (value.clamp(0.0, 1.0) * 255.0).round() as u8));
                }
            }
        }
    }
    Ok(pixels)
}
