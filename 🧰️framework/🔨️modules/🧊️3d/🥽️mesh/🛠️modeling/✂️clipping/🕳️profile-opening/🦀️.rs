//! 🕳️ Splits an annular cap along two certified support extensions.

use super::{MeshResult, invalid};

type VertexPair = [u32;2];
use std::collections::{BTreeMap,BTreeSet};

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct CapBridges {
    pub(super) ends: [[VertexPair; 2]; 2],
}

fn append_arc(ring: &[VertexPair], start: VertexPair, finish: VertexPair, path: &mut Vec<VertexPair>) -> MeshResult<()> {
    let cursor = ring.iter().position(|&corner| corner == start)
        .ok_or_else(|| invalid("cap bridge starts outside its boundary"))?;
    for &corner in ring.iter().cycle().skip(cursor).take(ring.len()) {
        path.push(corner);
        if corner == finish { return Ok(()); }
    }
    Err(invalid("cap bridge ends outside its boundary"))
}

impl CapBridges {
    pub(super) fn split(&self, outer: &[VertexPair], inner: &[VertexPair]) -> MeshResult<[Vec<VertexPair>; 2]> {
        let mut polygons = [Vec::new(), Vec::new()];
        for (index, polygon) in polygons.iter_mut().enumerate() {
            let [stock, opening] = self.ends[index];
            let [next_stock, next_opening] = self.ends[1 - index];
            append_arc(outer, stock, next_stock, polygon)?;
            append_arc(inner, next_opening, opening, polygon)?;
            let distinct: BTreeSet<_> = polygon.iter().collect();
            if distinct.len() < 3 || distinct.len() != polygon.len() {
                return Err(invalid("cap bridges do not separate two simple boundaries"));
            }
        }
        Ok(polygons)
    }
}

pub(super) fn certify_links<'a>(rings: impl IntoIterator<Item=&'a [u32]>) -> MeshResult<()> {
    let mut links = BTreeMap::<u32, BTreeMap<u32, u32>>::new();
    for vertices in rings {
        for i in 0..vertices.len() {
            let vertex = vertices[i];
            let previous = vertices[(i + vertices.len() - 1) % vertices.len()];
            let next = vertices[(i + 1) % vertices.len()];
            if vertex == next || links.entry(vertex).or_default().insert(previous, next).is_some() {
                return Err(invalid("joined clipping repeats a seam link"));
            }
        }
    }
    for link in links.values() {
        let first = *link.keys().next().unwrap();
        let mut current = first;
        for step in 0..link.len() {
            current = *link.get(&current).ok_or_else(|| invalid("joined clipping has an open seam link"))?;
            if (current == first) != (step + 1 == link.len()) {
                return Err(invalid("joined clipping has more than one seam fan"));
            }
        }
    }
    Ok(())
}

