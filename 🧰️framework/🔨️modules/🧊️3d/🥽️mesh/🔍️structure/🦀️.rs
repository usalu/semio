//! 🔍️ Read-only structural facts of a half-edge mesh: whether its rows hold together and its surface as a value that does not depend on
//! how the rows are numbered. Gesture tests use them to compare two meshes that show the same surface with different row numbering.

use super::HalfedgeMesh;

impl HalfedgeMesh {
    /// 🔍️ The first way the half-edge structure is inconsistent, if any: a face ring that does not close or has fewer than three corners,
    /// a halfedge in two rings or in none, a halfedge whose face is not its ring's face, a twin that is not symmetric or does not reverse
    /// its halfedge, a vertex reference out of range, a vertex link that does not name an outgoing halfedge.
    pub fn structure_violation(&self) -> Option<String> {
        let (vertices, halfedges, faces) = (&self.vertices, &self.halfedges, &self.faces);
        let mut owner: Vec<Option<usize>> = vec![None; halfedges.len()];
        for (face, row) in faces.iter().enumerate() {
            let mut halfedge = row.halfedge;
            let mut corners = 0;
            loop {
                let Some(held) = halfedges.get(halfedge as usize) else {
                    return Some(format!("face {face} ring leaves the halfedge arena at {halfedge}"));
                };
                if owner[halfedge as usize].replace(face).is_some() {
                    return Some(format!("halfedge {halfedge} is in two face rings"));
                }
                if held.face != Some(face as u32) {
                    return Some(format!("halfedge {halfedge} names face {:?}, its ring belongs to face {face}", held.face));
                }
                corners += 1;
                halfedge = held.next;
                if halfedge == row.halfedge {
                    break;
                }
                if corners > halfedges.len() {
                    return Some(format!("face {face} ring does not close"));
                }
            }
            if corners < 3 {
                return Some(format!("face {face} has {corners} corners"));
            }
        }
        if let Some(orphan) = owner.iter().position(Option::is_none) {
            return Some(format!("halfedge {orphan} belongs to no face"));
        }
        for (index, row) in halfedges.iter().enumerate() {
            if row.vertex as usize >= vertices.len() {
                return Some(format!("halfedge {index} starts at vertex {} past the vertex arena", row.vertex));
            }
            let destination = halfedges[row.next as usize].vertex;
            if let Some(twin) = row.twin {
                let Some(other) = halfedges.get(twin as usize) else {
                    return Some(format!("halfedge {index} has twin {twin} past the halfedge arena"));
                };
                if other.twin != Some(index as u32) {
                    return Some(format!("twin {twin} of halfedge {index} does not point back"));
                }
                if other.vertex != destination || halfedges[other.next as usize].vertex != row.vertex {
                    return Some(format!("twin {twin} of halfedge {index} does not reverse it"));
                }
            }
        }
        for (index, row) in vertices.iter().enumerate() {
            if let Some(halfedge) = row.halfedge {
                match halfedges.get(halfedge as usize) {
                    Some(held) if held.vertex as usize == index => {}
                    _ => return Some(format!("vertex {index} links halfedge {halfedge}, which is not outgoing from it")),
                }
            }
        }
        None
    }

    /// 🧭️ The surface as a value independent of row numbering: every face as its corner positions (as bit patterns, winding applied)
    /// rotated to the smallest corner, the faces sorted. Two meshes with equal signatures show the same polygons.
    pub fn surface_signature(&self) -> Vec<Vec<[u32; 3]>> {
        let positions = self.positions();
        let mut faces: Vec<Vec<[u32; 3]>> = self
            .polygons()
            .into_iter()
            .map(|corners| {
                let points: Vec<[u32; 3]> = corners.iter().map(|vertex| positions[*vertex as usize].map(f32::to_bits)).collect();
                let start = (0..points.len()).min_by_key(|index| points[*index]).unwrap_or(0);
                (0..points.len()).map(|step| points[(start + step) % points.len()]).collect()
            })
            .collect();
        faces.sort();
        faces
    }
}
