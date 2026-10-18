//! 🧵️ Stitch and cleanup: the selected face stitching, the shell grouping and the removal of orphan solids, faces, edges and vertices. See [`super`].

use super::*;

// #region 🔖️Scoping
// #region 🔖️Stitch
/// 🔀 Groups `selected` faces into connected shells via shared edges, orients every group outward
/// (flipping the whole group when its net signed volume comes out negative), then assembles ONE
/// solid out of all of them: a group that a containment test ([`solid_wholly_inside`]) places
/// inside another becomes an inverted VOID shell, and every remaining group joins the outer shell.
/// 🐛 This used to return only the largest-volume group and leave the rest live-but-unreturned,
/// which silently dropped every other lump: two barely-tangent spheres united to the volume of ONE
/// sphere. Disjoint lumps sharing one shell is the same representation
/// [`trivial_topology_fast_path`]'s own disjoint-union branch already produces.
/// 🐛 The sign probe's own `chord_tol` used to be `1e-6`: for a full-circle imprint edge that made
/// `segments_for_chord_deviation` sample ~1700+ boundary points, and `ear_clip`'s O(n³) worst case
/// over that many points took minutes. Only the SIGN of this volume is used here (to decide
/// whether to flip a group's faces), so it now matches `validation_report`'s own sign-probe
/// tolerance (`PROBE_TOL = 1e-3`) instead of a precision this call never needed.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn stitch_selected_faces(body: &mut Body, selected: &[FaceId], tol: f64, rec: &mut OpRecorder) -> Result<SolidId, KernelError> {
    let groups = group_shells(body, selected);
    if groups.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("stitch produced no shells".into())));
    }
    let mut lumps: Vec<(Vec<FaceId>, ShellId, SolidId)> = Vec::with_capacity(groups.len());
    for group in groups {
        let shell = add_shell(body, group.clone(), rec);
        if shell_signed_volume(body, shell, 1e-3).unwrap_or(0.0) < 0.0 {
            for &f in &group {
                flip_face(body, f);
            }
        }
        let solid = add_solid(body, shell, Vec::new(), rec);
        lumps.push((group, shell, solid));
    }
    let mut void_of: Vec<Option<usize>> = vec![None; lumps.len()];
    for inner in 0..lumps.len() {
        for outer in 0..lumps.len() {
            if inner == outer || void_of[outer] == Some(inner) {
                continue;
            }
            if solid_wholly_inside(body, lumps[inner].2, lumps[outer].2, tol)? {
                void_of[inner] = Some(outer);
                break;
            }
        }
    }
    let mut outer_faces: Vec<FaceId> = Vec::new();
    let mut void_shells: Vec<ShellId> = Vec::new();
    for (index, (group, shell, _)) in lumps.iter().enumerate() {
        if void_of[index].is_some() {
            for &f in group {
                flip_face(body, f);
            }
            void_shells.push(*shell);
        } else {
            outer_faces.extend(group.iter().copied());
        }
    }
    for (index, (_, shell, solid)) in lumps.iter().enumerate() {
        if let Some(label) = body.solids.get(*solid).map(|s| s.label) {
            rec.record_deleted(label);
        }
        body.solids.remove(*solid);
        if void_of[index].is_none() {
            if let Some(label) = body.shells.get(*shell).map(|s| s.label) {
                rec.record_deleted(label);
            }
            body.shells.remove(*shell);
        }
    }
    if outer_faces.is_empty() {
        return Err(KernelError::Boolean(BooleanError::InvalidResult("stitch found only enclosed shells and no outer boundary".into())));
    }
    let outer = add_shell(body, outer_faces, rec);
    Ok(add_solid(body, outer, void_shells, rec))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn group_shells(body: &Body, selected: &[FaceId]) -> Vec<Vec<FaceId>> {
    let mut edge_to_faces: HashMap<EdgeId, Vec<FaceId>> = HashMap::new();
    for &f in selected {
        for cid in body.face_coedges(f) {
            if let Some(co) = body.coedges.get(cid) {
                edge_to_faces.entry(co.edge).or_default().push(f);
            }
        }
    }
    let mut adjacency: std::collections::BTreeMap<FaceId, std::collections::BTreeSet<FaceId>> = std::collections::BTreeMap::new();
    for &f in selected {
        adjacency.entry(f).or_default();
    }
    for faces in edge_to_faces.values() {
        for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                adjacency.entry(faces[i]).or_default().insert(faces[j]);
                adjacency.entry(faces[j]).or_default().insert(faces[i]);
            }
        }
    }
    let mut visited: HashSet<FaceId> = HashSet::new();
    let mut groups = Vec::new();
    for &f in selected {
        if visited.contains(&f) {
            continue;
        }
        let mut stack = vec![f];
        visited.insert(f);
        let mut comp = Vec::new();
        while let Some(cur) = stack.pop() {
            comp.push(cur);
            if let Some(neighbors) = adjacency.get(&cur) {
                for &n in neighbors {
                    if visited.insert(n) {
                        stack.push(n);
                    }
                }
            }
        }
        groups.push(comp);
    }
    groups
}
// #endregion 🔖️Stitch

// #region 🔖️Cleanup
/// 🔀 Removes every face of `solid`'s original shells that is NOT in `keep` (a face that WAS kept
/// survives as a floating face until [`stitch_selected_faces`]'s new shell picks it back up), then
/// drops the now-superseded shell(s)/solid wrapper itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn remove_solid_and_orphans(body: &mut Body, solid: SolidId, keep: &HashSet<FaceId>, rec: &mut OpRecorder) {
    let Some(data) = body.solids.get(solid).cloned() else { return };
    let shells: Vec<ShellId> = std::iter::once(data.outer).chain(data.inners.iter().copied()).collect();
    for shell_id in shells {
        let Some(shell) = body.shells.get(shell_id).cloned() else { continue };
        for f in shell.faces {
            if !keep.contains(&f) {
                remove_face(body, f, rec);
            }
        }
        if let Some(label) = body.shells.get(shell_id).map(|s| s.label) {
            rec.record_deleted(label);
        }
        body.shells.remove(shell_id);
    }
    if let Some(label) = body.solids.get(solid).map(|s| s.label) {
        rec.record_deleted(label);
    }
    body.solids.remove(solid);
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_face(body: &mut Body, face: FaceId, rec: &mut OpRecorder) {
    let Some(data) = body.faces.get(face).cloned() else { return };
    let mut loops = Vec::new();
    if let Some(o) = data.outer {
        loops.push(o);
    }
    loops.extend(data.inners.iter().copied());
    for loop_id in loops {
        for cid in body.loop_coedges(loop_id) {
            body.coedges.remove(cid);
        }
        body.loops.remove(loop_id);
    }
    rec.record_deleted(data.label);
    body.faces.remove(face);
}

/// 🔀 Drops every edge/vertex THIS boolean minted that no live coedge/edge references any more —
/// the imprint pipeline creates edges/vertices speculatively (both faces of a pair queue the same
/// shared edge even when only one side ends up selected) and `remove_face` only clears coedges, not
/// the shared geometry underneath them.
/// 🐛 `protected` is every edge/vertex that predates the job, and the sweep may never cross it: a
/// bare [`semio_framework_3d_brep_representation::topology::Wire`]'s member edges
/// carry no coedge at all, so an unscoped orphan sweep read every live wire handle's geometry as
/// garbage and freed it — the same "a boolean frees what it did not create" law its operands are
/// now protected by.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn gc_orphan_edges_and_vertices(body: &mut Body, (protected_edges, protected_vertices): (&HashSet<EdgeId>, &HashSet<VertexId>), rec: &mut OpRecorder) {
    let used_edges: HashSet<EdgeId> = body.coedges.iter().map(|(_, c)| c.edge).collect();
    let dead_edges: Vec<EdgeId> = body.edges.iter().map(|(id, _)| id).filter(|id| !used_edges.contains(id) && !protected_edges.contains(id)).collect();
    for e in dead_edges {
        if let Some(data) = body.edges.get(e) {
            rec.record_deleted(data.label);
        }
        body.edges.remove(e);
    }
    let mut used_verts: HashSet<VertexId> = HashSet::new();
    for (_, e) in body.edges.iter() {
        used_verts.insert(e.v0);
        used_verts.insert(e.v1);
    }
    let dead_verts: Vec<VertexId> = body.vertices.iter().map(|(id, _)| id).filter(|id| !used_verts.contains(id) && !protected_vertices.contains(id)).collect();
    for v in dead_verts {
        if let Some(data) = body.vertices.get(v) {
            rec.record_deleted(data.label);
        }
        body.vertices.remove(v);
    }
}
// #endregion 🔖️Cleanup
// #endregion 🔖️ExactImprintEngine
