//! 🧹️ Checks the complete analytic boundary of a reloaded rounded box.

use super::*;
use semio_framework_3d_brep_representation::{
    contract::{RoundedBox, box_finish_layout::{EDGES, FACES, TOPOLOGY, VERTICES}},
    topology::Shell,
};
use semio_framework_3d_brep_solid_topology::rounded_box_boundary::{surface, trim};

pub(super) fn certify(body: &Body, shell: &Shell) -> Option<RoundedBox> {
    if shell.faces.len() != FACES { return None; }
    let sphere = |index: usize| {
        let face = body.faces.get(*shell.faces.get(index)?)?;
        let Surface::Sphere { frame, radius } = body.surfaces.get(face.surface)? else { return None; };
        Some((frame.origin, *radius))
    };
    let (low, radius) = sphere(18)?;
    let (high, _) = sphere(25)?;
    let (low, high) = (low.to_array(), high.to_array());
    let inset = std::array::from_fn(|axis| [low[axis], high[axis]]);
    let shape = RoundedBox::from_inset(inset, radius, Tol::DEFAULT.value())?;
    let mut edges = [None; EDGES];
    let mut vertices = [None; VERTICES];
    for (index, &face_id) in shell.faces.iter().enumerate() {
        let face = body.faces.get(face_id)?;
        if face.flipped || !face.inners.is_empty() || *body.surfaces.get(face.surface)? != surface(shape, index) {
            return None;
        }
        let ring_id = face.outer?;
        let ring = body.loops.get(ring_id)?;
        if ring.face != face_id { return None; }
        let mut cursor = ring.first;
        for member in 0..TOPOLOGY.ring_sizes[index] {
            if member > 0 && cursor == ring.first { return None; }
            let coedge = body.coedges.get(cursor)?;
            let expected = trim(shape, index, member);
            if coedge.loop_id != ring_id || coedge.forward != expected.forward || coedge.prange != expected.range
                || *body.curves2.get(coedge.pcurve?)? != expected.curve || body.coedges.get(coedge.next)?.prev != cursor {
                return None;
            }
            let edge = body.edges.get(coedge.edge)?;
            let (ends, curve, range) = shape.edge(expected.edge);
            if edge.range != range || *body.curves3.get(edge.curve)? != curve {
                return None;
            }
            bind(&mut edges, expected.edge, coedge.edge)?;
            for (ordinal, id) in ends.into_iter().zip([edge.v0, edge.v1]) {
                if body.vertices.get(id)?.position != shape.vertex(ordinal) { return None; }
                bind(&mut vertices, ordinal, id)?;
            }
            cursor = coedge.next;
        }
        if cursor != ring.first { return None; }
    }
    Some(shape)
}

fn bind<T: Copy + Eq>(slots: &mut [Option<T>], ordinal: usize, value: T) -> Option<()> {
    if let Some(existing) = slots[ordinal] { return (existing == value).then_some(()); }
    if slots.contains(&Some(value)) { return None; }
    slots[ordinal] = Some(value);
    Some(())
}
