//! 🔳️ Subdivision, triangulation and loop cuts as row emitters: a split face keeps its first piece in place and appends the others;
//! the new vertices (face centres, edge cuts, grid points) are appended.

use super::{select_faces, triangulate_polygon, EdgeId, FaceId, MeshKernelError, MeshOverlay, MeshResult, Orphans, Vec3};
use std::collections::{BTreeMap, BTreeSet};

fn key(a: u32, b: u32) -> (u32, u32) {
    (a.min(b), a.max(b))
}

/// ✂️ The vertex `step` of `segments` along the edge from `a` to `b`: an end point or the cut vertex stored for the undirected edge.
fn on_edge(cuts: &BTreeMap<(u32, u32), Vec<u32>>, a: u32, b: u32, step: usize, segments: usize) -> u32 {
    if step == 0 {
        a
    } else if step == segments {
        b
    } else {
        cuts[&key(a, b)][if a < b { step - 1 } else { segments - step - 1 }]
    }
}

impl MeshOverlay<'_> {
    /// 🔳️ Splits the faces around their centre (a concave face first into triangles, each split around its own centre); see
    /// [`HalfedgeMesh::subdivide_faces`].
    pub fn subdivide_faces(&mut self, faces: &[FaceId]) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("subdivision")?;
        let soup = self.soup()?;
        let selected: BTreeSet<u32> = select_faces(&soup, faces, "duplicate face selection")?.into_iter().collect();
        for face in selected {
            let corners = soup.faces[face as usize].clone();
            let count = corners.len();
            let normal = soup.normals[face as usize];
            let point = |index: usize| Vec3(soup.positions[corners[index % count] as usize]);
            let convex = (0..count).all(|index| point(index + 1).sub(point(index)).cross(point(index + 2).sub(point(index + 1))).dot(normal) > 0.0);
            let mut pieces: Vec<Vec<u32>> = Vec::new();
            if convex {
                let centre = corners.iter().fold(Vec3::ZERO, |sum, vertex| sum.add(Vec3(soup.positions[*vertex as usize]))).scale(1.0 / count as f32);
                let centre = self.push_vertex(centre.0);
                pieces.extend((0..count).map(|index| vec![corners[index], corners[(index + 1) % count], centre]));
            } else {
                let points: Vec<Vec3> = (0..count).map(point).collect();
                for [a, b, c] in triangulate_polygon(&points) {
                    let ids = [corners[a], corners[b], corners[c]];
                    let centre = ids.iter().fold(Vec3::ZERO, |sum, vertex| sum.add(Vec3(soup.positions[*vertex as usize]))).scale(1.0 / 3.0);
                    let centre = self.push_vertex(centre.0);
                    pieces.extend((0..3).map(|index| vec![ids[index], ids[(index + 1) % 3], centre]));
                }
            }
            self.replace_with_pieces(face, pieces, soup.smooth[face as usize])?;
        }
        self.finish(Orphans::Keep)
    }

    /// 🔻️ Splits every polygon of more than three corners into triangles; see [`HalfedgeMesh::triangulate`].
    pub fn triangulate(&mut self) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("triangulation")?;
        let soup = self.soup()?;
        for (face, corners) in soup.faces.iter().enumerate() {
            if corners.len() <= 3 {
                continue;
            }
            let points: Vec<Vec3> = corners.iter().map(|vertex| Vec3(soup.positions[*vertex as usize])).collect();
            let pieces: Vec<Vec<u32>> = triangulate_polygon(&points).into_iter().map(|[a, b, c]| vec![corners[a], corners[b], corners[c]]).collect();
            self.replace_with_pieces(face as u32, pieces, soup.smooth[face])?;
        }
        self.finish(Orphans::Keep)
    }

    /// ✂️ Cuts the quad strips the named edges run through into `cuts + 1` rows; the cut vertices are shared by the neighbouring faces;
    /// see [`HalfedgeMesh::loop_cut`].
    pub fn loop_cut(&mut self, edges: &[EdgeId], cuts: u32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("loop cut")?;
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        if !(1..=256).contains(&cuts) {
            return Err(MeshKernelError::InvalidInput("cuts must be in 1..=256".into()));
        }
        let cuts = cuts as usize;
        let soup = self.soup()?;
        let mut incidence: BTreeMap<(u32, u32), Vec<(usize, usize)>> = BTreeMap::new();
        for (face, corners) in soup.faces.iter().enumerate() {
            for index in 0..corners.len() {
                incidence.entry(key(corners[index], corners[(index + 1) % corners.len()])).or_default().push((face, index));
            }
        }
        let mut marked: BTreeSet<(u32, u32)> = BTreeSet::new();
        let mut pending: Vec<(u32, u32)> = Vec::new();
        for edge in edges {
            let (a, b) = self.base.edge_endpoints(*edge)?;
            let edge = key(a.0, b.0);
            let owners = incidence.get(&edge).ok_or(MeshKernelError::InvalidHandle)?;
            if owners.len() > 2 {
                return Err(MeshKernelError::NonManifold);
            }
            if !owners.iter().any(|(face, _)| soup.faces[*face].len() == 4) {
                return Err(MeshKernelError::InvalidInput("selected edges must touch a quad".into()));
            }
            if marked.insert(edge) {
                pending.push(edge);
            }
        }
        let mut cursor = 0;
        while cursor < pending.len() {
            let owners = incidence.get(&pending[cursor]).ok_or(MeshKernelError::InvalidHandle)?;
            if owners.len() > 2 {
                return Err(MeshKernelError::NonManifold);
            }
            for (face, index) in owners {
                let corners = &soup.faces[*face];
                if corners.len() == 4 {
                    let opposite = key(corners[(index + 2) % 4], corners[(index + 3) % 4]);
                    if marked.insert(opposite) {
                        pending.push(opposite);
                    }
                }
            }
            cursor += 1;
        }
        let mut cut_vertices: BTreeMap<(u32, u32), Vec<u32>> = BTreeMap::new();
        for (a, b) in &marked {
            let (start, end) = (soup.positions[*a as usize], soup.positions[*b as usize]);
            let mut ids: Vec<u32> = Vec::with_capacity(cuts);
            let mut previous = start;
            for step in 1..=cuts {
                let t = step as f64 / (cuts + 1) as f64;
                let point = [0, 1, 2].map(|axis| (start[axis] as f64 * (1.0 - t) + end[axis] as f64 * t) as f32);
                if point.iter().any(|coordinate| !coordinate.is_finite()) || point == previous || point == end {
                    return Err(MeshKernelError::DegenerateOperation);
                }
                ids.push(self.push_vertex(point));
                previous = point;
            }
            cut_vertices.insert((*a, *b), ids);
        }
        for (face, corners) in soup.faces.iter().enumerate() {
            let smooth = soup.smooth[face];
            if corners.len() == 4 {
                let nx = if marked.contains(&key(corners[0], corners[1])) { cuts + 1 } else { 1 };
                let ny = if marked.contains(&key(corners[1], corners[2])) { cuts + 1 } else { 1 };
                if nx == 1 && ny == 1 {
                    continue;
                }
                let mut grid: Vec<u32> = Vec::with_capacity((nx + 1) * (ny + 1));
                for cursor in 0..(nx + 1) * (ny + 1) {
                    let (x, y) = (cursor % (nx + 1), cursor / (nx + 1));
                    let id = if y == 0 {
                        on_edge(&cut_vertices, corners[0], corners[1], x, nx)
                    } else if y == ny {
                        on_edge(&cut_vertices, corners[3], corners[2], x, nx)
                    } else if x == 0 {
                        on_edge(&cut_vertices, corners[0], corners[3], y, ny)
                    } else if x == nx {
                        on_edge(&cut_vertices, corners[1], corners[2], y, ny)
                    } else {
                        let (u, v) = (x as f64 / nx as f64, y as f64 / ny as f64);
                        let weights = [(1.0 - u) * (1.0 - v), u * (1.0 - v), u * v, (1.0 - u) * v];
                        let point = [0, 1, 2].map(|axis| corners.iter().zip(weights).map(|(vertex, weight)| soup.positions[*vertex as usize][axis] as f64 * weight).sum::<f64>() as f32);
                        if point.iter().any(|coordinate| !coordinate.is_finite()) {
                            return Err(MeshKernelError::DegenerateOperation);
                        }
                        self.push_vertex(point)
                    };
                    grid.push(id);
                }
                let pieces: Vec<Vec<u32>> = (0..nx * ny)
                    .map(|cursor| {
                        let (x, y) = (cursor % nx, cursor / nx);
                        let at = y * (nx + 1) + x;
                        vec![grid[at], grid[at + 1], grid[at + nx + 2], grid[at + nx + 1]]
                    })
                    .collect();
                self.replace_with_pieces(face as u32, pieces, smooth)?;
            } else if !corners.is_empty() {
                let count = corners.len();
                let mut boundary: Vec<u32> = Vec::with_capacity(count);
                let mut cut = false;
                for index in 0..count {
                    let (a, b) = (corners[index], corners[(index + 1) % count]);
                    boundary.push(a);
                    if marked.contains(&key(a, b)) {
                        cut = true;
                        boundary.extend((1..=cuts).map(|step| on_edge(&cut_vertices, a, b, step, cuts + 1)));
                    }
                }
                if cut {
                    self.replace_face(face as u32, &boundary)?;
                }
            }
        }
        self.finish(Orphans::Keep)
    }
}
