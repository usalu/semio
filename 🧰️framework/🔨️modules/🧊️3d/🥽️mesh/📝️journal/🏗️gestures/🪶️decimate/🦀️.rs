//! 🪶️ Decimation as a row emitter: every step collapses the shortest safe edge, which moves one vertex to the edge midpoint, rewrites
//! the faces that touched the other endpoint (a face that collapses below three corners is removed) and removes the orphaned vertex.

use super::{MeshKernelError, MeshOverlay, MeshResult, Normal, Orphans, Soup, Vec3};
use std::collections::{BTreeMap, BTreeSet, HashSet};

/// 🪶️ The faces a collapse of `b` into `a` (which moves to the edge midpoint) rewrites — `None` for a face that vanishes — or `None`
/// when the collapse would fold a face over, repeat a vertex in a face, leave fewer than four faces or break the surface contract
/// (every edge shared by exactly two opposite faces, the link of the surviving vertex one cycle).
fn collapse(soup: &Soup, a: u32, b: u32, closed: bool) -> Option<Vec<(usize, Option<Vec<u32>>)>> {
    let middle = Vec3(soup.positions[a as usize]).lerp(Vec3(soup.positions[b as usize]), 0.5);
    let position = |vertex: u32| if vertex == a { middle } else { Vec3(soup.positions[vertex as usize]) };
    let mut rewrites: Vec<(usize, Option<Vec<u32>>)> = Vec::new();
    let mut surviving: Vec<Vec<u32>> = Vec::new();
    for (face, corners_in) in soup.faces.iter().enumerate() {
        let mut corners: Vec<u32> = Vec::new();
        for vertex in corners_in {
            let id = if *vertex == b { a } else { *vertex };
            if corners.last() != Some(&id) {
                corners.push(id);
            }
        }
        if corners.first() == corners.last() {
            corners.pop();
        }
        if corners.len() < 3 {
            rewrites.push((face, None));
            continue;
        }
        let mut seen: HashSet<u32> = HashSet::new();
        if corners.iter().any(|id| !seen.insert(*id)) {
            return None;
        }
        let mut normal = Normal::starting_at(position(corners[0]));
        for index in 0..corners.len() {
            normal.add(position(corners[index]), position(corners[(index + 1) % corners.len()]));
        }
        let after = normal.value();
        if after.length() == 0.0 || soup.normals[face].dot(after) <= 0.0 {
            return None;
        }
        rewrites.push((face, Some(corners.clone())));
        surviving.push(corners);
    }
    if surviving.len() < 4 {
        return None;
    }
    let mut incidence: BTreeMap<(u32, u32), (usize, i32)> = BTreeMap::new();
    let mut links: BTreeMap<u32, u32> = BTreeMap::new();
    let mut start: Option<u32> = None;
    for corners in &surviving {
        for index in 0..corners.len() {
            let (from, to) = (corners[index], corners[(index + 1) % corners.len()]);
            let entry = incidence.entry((from.min(to), from.max(to))).or_default();
            entry.0 += 1;
            entry.1 += if from < to { 1 } else { -1 };
            if from == a {
                let previous = corners[(index + corners.len() - 1) % corners.len()];
                if closed && links.insert(previous, to).is_some() {
                    return None;
                }
                start = Some(start.map_or(previous, |held| held.min(previous)));
            }
        }
    }
    if incidence.values().any(|edge| *edge != (2, 0) && (closed || edge.0 != 1)) {
        return None;
    }
    if closed {
        let start = start?;
        let mut cursor = start;
        loop {
            let next = links.remove(&cursor)?;
            cursor = next;
            if next == start {
                return links.is_empty().then_some(rewrites);
            }
        }
    }
    Some(rewrites)
}

impl MeshOverlay<'_> {
    /// 🪶️ Collapses the shortest safe edges until at most `ceil(vertices * ratio)` (and four) vertices remain; see
    /// [`HalfedgeMesh::decimate`].
    pub fn decimate(&mut self, target_ratio: f32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("decimation")?;
        if !target_ratio.is_finite() || target_ratio <= 0.0 || target_ratio > 1.0 {
            return Err(MeshKernelError::InvalidInput("decimation ratio must be in (0,1]".into()));
        }
        let initial = self.vertex_len();
        let target = ((initial as f32 * target_ratio).ceil() as usize).max(4);
        if initial.saturating_mul(self.halfedge_len()).saturating_mul(initial.saturating_sub(target)) > 32_000_000 {
            return Err(MeshKernelError::InvalidInput("decimation work budget exceeded".into()));
        }
        let mut identity: Vec<u32> = (0..initial as u32).collect();
        while self.vertex_len() > target {
            let soup = self.soup()?;
            let closed = (0..self.halfedge_len() as u32).all(|halfedge| self.halfedge(halfedge).is_some_and(|row| row.twin.is_some()));
            let mut seen: BTreeSet<(u32, u32)> = BTreeSet::new();
            let mut ranked: Vec<(f32, u32, u32, u32, u32)> = Vec::new();
            for corners in &soup.faces {
                for index in 0..corners.len() {
                    let (x, y) = (corners[index], corners[(index + 1) % corners.len()]);
                    if seen.insert((x.min(y), x.max(y))) {
                        let (a, b) = if identity[x as usize] < identity[y as usize] { (x, y) } else { (y, x) };
                        let rank = Vec3(soup.positions[x as usize]).sub(Vec3(soup.positions[y as usize])).length();
                        ranked.push((rank, identity[a as usize], identity[b as usize], a, b));
                    }
                }
            }
            ranked.sort_by(|left, right| left.0.total_cmp(&right.0).then((left.1, left.2).cmp(&(right.1, right.2))));
            let accepted = ranked.iter().find_map(|(_, _, _, a, b)| collapse(&soup, *a, *b, closed).map(|rewrites| (*a, *b, rewrites)));
            let Some((a, b, rewrites)) = accepted else {
                break;
            };
            let middle = Vec3(soup.positions[a as usize]).lerp(Vec3(soup.positions[b as usize]), 0.5);
            self.move_vertex(a, middle.0)?;
            for (face, corners) in rewrites {
                match corners {
                    None => self.kill_face(face as u32)?,
                    Some(corners) if soup.faces[face].contains(&b) => self.replace_face(face as u32, &corners)?,
                    Some(_) => {}
                }
            }
            self.finish(Orphans::Newly)?;
            for (hole, last) in std::mem::take(&mut self.vertex_moves) {
                identity[hole as usize] = identity[last as usize];
            }
            identity.truncate(self.vertex_len());
        }
        Ok(())
    }
}
