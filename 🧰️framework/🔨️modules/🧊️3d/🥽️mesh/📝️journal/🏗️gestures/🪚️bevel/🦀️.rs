//! 🪚️ Bevelling as a row emitter: each selected edge contributes `segments` clip planes of a circular profile; every plane cuts the
//! convex solid. The gesture tracks, per face, whether a plane cut it (a cut face is replaced in place, a face cut away is removed)
//! and appends one cap face per plane; the intersection vertices are appended.

use super::{EdgeId, MeshKernelError, MeshOverlay, MeshResult, Orphans, Vec3};
use std::collections::{BTreeMap, BTreeSet};

/// 🧊️ The solid while the planes cut it: the faces with the original face each one stays (none for a cap), whether a plane cut it and
/// its shading.
struct Working {
    positions: Vec<[f32; 3]>,
    faces: Vec<Vec<u32>>,
    origin: Vec<Option<u32>>,
    cut: Vec<bool>,
    smooth: Vec<bool>,
}

/// ✂️ Clips the solid to the half space `normal · p <= distance`: corners beyond the plane drop out, edges that cross it gain an
/// intersection vertex and the cut closes with one cap face wound around the cap centre.
fn clip(work: &mut Working, normal: Vec3, distance: f32, epsilon: f32) -> MeshResult<()> {
    let mut intersections: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    let mut cap: BTreeSet<u32> = BTreeSet::new();
    let mut owners: BTreeMap<u32, usize> = BTreeMap::new();
    let mut kept: Vec<(Vec<u32>, Option<u32>, bool, bool)> = Vec::new();
    for face in 0..work.faces.len() {
        let corners_in = work.faces[face].clone();
        let count = corners_in.len();
        let mut corners: Vec<u32> = Vec::new();
        let mut touched = false;
        for index in 0..count {
            let (a, b) = (corners_in[index], corners_in[(index + 1) % count]);
            let da = Vec3(work.positions[a as usize]).dot(normal) - distance;
            let db = Vec3(work.positions[b as usize]).dot(normal) - distance;
            if da <= epsilon {
                if corners.last() != Some(&a) {
                    corners.push(a);
                }
                if da.abs() <= epsilon {
                    cap.insert(a);
                    owners.entry(a).or_insert(face);
                }
            } else {
                touched = true;
            }
            if (da > epsilon && db < -epsilon) || (da < -epsilon && db > epsilon) {
                let key = (a.min(b), a.max(b));
                let id = match intersections.get(&key) {
                    Some(id) => *id,
                    None => {
                        let point = Vec3(work.positions[a as usize]).lerp(Vec3(work.positions[b as usize]), da / (da - db));
                        let id = work.positions.len() as u32;
                        work.positions.push(point.0);
                        intersections.insert(key, id);
                        id
                    }
                };
                if corners.last() != Some(&id) {
                    corners.push(id);
                }
                cap.insert(id);
                owners.entry(id).or_insert(face);
                touched = true;
            }
        }
        if corners.first() == corners.last() {
            corners.pop();
        }
        if corners.len() >= 3 {
            kept.push((corners, work.origin[face], work.cut[face] || touched, work.smooth[face]));
        }
    }
    if cap.len() < 3 {
        return Err(MeshKernelError::InvalidInput("bevel width does not intersect the solid".into()));
    }
    let ids: Vec<u32> = cap.iter().copied().collect();
    let mut centre = Vec3::ZERO;
    for id in &ids {
        centre = centre.add(Vec3(work.positions[*id as usize]).scale(1.0 / ids.len() as f32));
    }
    let axis = if normal.x().abs() < 0.9 { Vec3::new(1.0, 0.0, 0.0) } else { Vec3::new(0.0, 1.0, 0.0) };
    let u = normal.cross(axis).normalize();
    let v = normal.cross(u);
    let mut ranked: Vec<(f32, u32)> = ids
        .iter()
        .map(|id| {
            let point = Vec3(work.positions[*id as usize]).sub(centre);
            (point.dot(v).atan2(point.dot(u)), *id)
        })
        .collect();
    ranked.sort_by(|left, right| left.0.total_cmp(&right.0).then(left.1.cmp(&right.1)));
    let cap_loop: Vec<u32> = ranked.iter().map(|entry| entry.1).collect();
    let cap_smooth = work.smooth[owners[&cap_loop[0]]];
    work.faces.clear();
    work.origin.clear();
    work.cut.clear();
    work.smooth.clear();
    for (corners, origin, cut, smooth) in kept {
        work.faces.push(corners);
        work.origin.push(origin);
        work.cut.push(cut);
        work.smooth.push(smooth);
    }
    work.faces.push(cap_loop);
    work.origin.push(None);
    work.cut.push(true);
    work.smooth.push(cap_smooth);
    Ok(())
}

impl MeshOverlay<'_> {
    /// 🪚️ Bevels the edges of a closed convex mesh with a circular profile of `segments` steps; see [`HalfedgeMesh::bevel_edges`].
    pub fn bevel_edges(&mut self, edges: &[EdgeId], amount: f32, segments: u32) -> MeshResult<()> {
        self.untouched()?;
        self.require_plain("bevel")?;
        if edges.is_empty() {
            return Err(MeshKernelError::EmptySelection);
        }
        if !amount.is_finite() || amount <= 0.0 || !(1..=64).contains(&segments) {
            return Err(MeshKernelError::InvalidInput("bevel requires positive finite width and 1..=64 segments".into()));
        }
        let soup = self.soup()?;
        let work = soup.positions.len().saturating_mul(soup.faces.len()).saturating_add(edges.len().saturating_mul(segments as usize).saturating_mul(self.halfedge_len()));
        if work > 4_000_000 {
            return Err(MeshKernelError::InvalidInput("bevel work budget exceeded".into()));
        }
        for edge in edges {
            if self.halfedge(edge.0).ok_or(MeshKernelError::InvalidHandle)?.twin.is_none() {
                return Err(MeshKernelError::NonManifold);
            }
        }
        let first = *soup.positions.first().ok_or(MeshKernelError::InvalidHandle)?;
        let extent = soup.positions.iter().fold(0.0_f32, |extent, position| extent.max(Vec3(*position).sub(Vec3(first)).length()));
        let epsilon = extent * 1e-6;
        if epsilon == 0.0 {
            return Err(MeshKernelError::DegenerateOperation);
        }
        for (face, corners) in soup.faces.iter().enumerate() {
            let normal = soup.normals[face];
            let origin = Vec3(soup.positions[corners[0] as usize]);
            if normal.length() == 0.0 || soup.positions.iter().any(|position| Vec3(*position).sub(origin).dot(normal) > epsilon) {
                return Err(MeshKernelError::InvalidInput("bevel requires an outward convex mesh".into()));
            }
        }
        if (0..self.halfedge_len() as u32).any(|halfedge| self.halfedge(halfedge).is_some_and(|row| row.twin.is_none())) {
            return Err(MeshKernelError::InvalidInput("bevel requires a closed manifold mesh".into()));
        }
        let mut selected: BTreeSet<u32> = BTreeSet::new();
        let mut planes: Vec<(Vec3, f32)> = Vec::new();
        for edge in edges {
            let row = self.halfedge(edge.0).ok_or(MeshKernelError::InvalidHandle)?.clone();
            let twin = row.twin.ok_or(MeshKernelError::NonManifold)?;
            if !selected.insert(edge.0.min(twin)) {
                continue;
            }
            let sides = [row.face.ok_or(MeshKernelError::InvalidHandle)? as usize, self.halfedge(twin).ok_or(MeshKernelError::InvalidHandle)?.face.ok_or(MeshKernelError::InvalidHandle)? as usize];
            let (n1, n2) = (soup.normals[sides[0]], soup.normals[sides[1]]);
            let cosine = n1.dot(n2).clamp(-1.0, 1.0);
            let theta = cosine.acos();
            let sine = theta.sin();
            if sine.abs() < 1e-5 {
                return Err(MeshKernelError::DegenerateOperation);
            }
            let origin = Vec3(soup.positions[row.vertex as usize]);
            let tangent = n2.sub(n1.scale(cosine)).scale(1.0 / sine);
            let directions = [tangent, n1.sub(n2.scale(cosine)).scale(1.0 / sine)];
            let centre = origin.sub(n1.add(n2).scale(amount / sine));
            let radius = amount * (1.0 + cosine) / sine;
            let mut clearance = f32::INFINITY;
            for (side, direction) in sides.iter().zip(directions) {
                for vertex in &soup.faces[*side] {
                    let distance = origin.sub(Vec3(soup.positions[*vertex as usize])).dot(direction);
                    if distance > epsilon {
                        clearance = clearance.min(distance);
                    }
                }
            }
            if amount >= clearance * 0.5 || amount <= epsilon {
                return Err(MeshKernelError::InvalidInput("bevel width must stay below half the neighboring face clearance".into()));
            }
            for segment in 0..segments {
                let angle = theta * (segment as f32 + 0.5) / segments as f32;
                let normal = n1.scale(angle.cos()).add(tangent.scale(angle.sin()));
                let distance = normal.dot(centre) + radius * (theta / (2.0 * segments as f32)).cos();
                planes.push((normal, distance));
            }
        }
        let original = soup.positions.len();
        let faces = soup.faces.len();
        let mut solid = Working { positions: soup.positions.clone(), faces: soup.faces.clone(), origin: (0..faces as u32).map(Some).collect(), cut: vec![false; faces], smooth: soup.smooth.clone() };
        for (normal, distance) in planes {
            clip(&mut solid, normal, distance, epsilon)?;
        }
        let mut directed: BTreeSet<(u32, u32)> = BTreeSet::new();
        for corners in &solid.faces {
            for index in 0..corners.len() {
                if !directed.insert((corners[index], corners[(index + 1) % corners.len()])) {
                    return Err(MeshKernelError::NonManifold);
                }
            }
        }
        if directed.iter().any(|(a, b)| !directed.contains(&(*b, *a))) {
            return Err(MeshKernelError::NonManifold);
        }
        if solid.faces.len() < 4 {
            return Err(MeshKernelError::InvalidInput("bevel removed the solid".into()));
        }
        let needed: BTreeSet<u32> = solid.faces.iter().flatten().copied().filter(|vertex| *vertex as usize >= original).collect();
        let mut mapped: BTreeMap<u32, u32> = BTreeMap::new();
        for vertex in needed {
            mapped.insert(vertex, self.push_vertex(solid.positions[vertex as usize]));
        }
        let mut survivors: BTreeMap<u32, (Vec<u32>, bool)> = BTreeMap::new();
        let mut caps: Vec<(Vec<u32>, bool)> = Vec::new();
        for (index, corners) in solid.faces.iter().enumerate() {
            let ids: Vec<u32> = corners.iter().map(|vertex| if (*vertex as usize) < original { *vertex } else { mapped[vertex] }).collect();
            match solid.origin[index] {
                Some(origin) => {
                    survivors.insert(origin, (ids, solid.cut[index]));
                }
                None => caps.push((ids, solid.smooth[index])),
            }
        }
        for face in 0..faces as u32 {
            if !survivors.contains_key(&face) {
                self.kill_face(face)?;
            }
        }
        for (face, (ids, cut)) in &survivors {
            if *cut {
                self.replace_face(*face, ids)?;
            }
        }
        for (ids, smooth) in caps {
            self.add_face(&ids, smooth)?;
        }
        self.finish(Orphans::Newly)
    }
}
