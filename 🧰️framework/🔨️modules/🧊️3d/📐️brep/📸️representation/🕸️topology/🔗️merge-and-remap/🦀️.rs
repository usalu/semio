//! 🔗️ Merge and remap: the remap, entity count and merge map records with the body merge, compaction remap and counting operations. See [`super`].

use super::*;

// #region ♻️Reachability
/// ♻️ How many slots one [`Body::compact`] call actually freed, per store — informational only.
/// Kept ids never move (no index remap, only [`Store::free`] on the rest), so there is nothing
/// for a caller to translate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Remap {
    pub freed_vertices: usize,
    pub freed_edges: usize,
    pub freed_coedges: usize,
    pub freed_loops: usize,
    pub freed_faces: usize,
    pub freed_shells: usize,
    pub freed_solids: usize,
    pub freed_curves3: usize,
    pub freed_curves2: usize,
    pub freed_surfaces: usize,
}

/// ♻️ Per-store live counts — a cheap sanity probe for tests and diagnostics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EntityCounts {
    pub vertices: usize,
    pub edges: usize,
    pub coedges: usize,
    pub loops: usize,
    pub faces: usize,
    pub shells: usize,
    pub solids: usize,
    pub curves3: usize,
    pub curves2: usize,
    pub surfaces: usize,
}

/// ♻️ Old→new id translation for every store [`Body::merge`] copied an entity into — needed by a
/// caller (the engine's handle registry) that must re-target ids it minted handles against before
/// the merge happened.
#[derive(Clone, Debug, Default)]
pub struct MergeMap {
    pub vertices: HashMap<VertexId, VertexId>,
    pub edges: HashMap<EdgeId, EdgeId>,
    pub faces: HashMap<FaceId, FaceId>,
    pub shells: HashMap<ShellId, ShellId>,
    pub solids: HashMap<SolidId, SolidId>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn null_coedge_for_merge() -> CoedgeId {
    ArenaId::from_raw(0, 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn null_loop_for_merge() -> LoopId {
    ArenaId::from_raw(0, 0)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn null_face_for_merge() -> FaceId {
    ArenaId::from_raw(0, 0)
}

impl Body {
    /// ♻️ Every entity transitively reachable from `roots`, walking solid→shell→face→(surface,
    /// loop)→coedge→(pcurve, edge)→(curve, vertex). The complement of this set is exactly what
    /// [`Body::compact`] is safe to free — see its own docstring.
    pub fn reachable_from(&self, roots: &[EntityRef]) -> ReachSet {
        let mut job=ReachabilityJob::new();
        for &root in roots {
            job.begin_root(root).expect("cold original reachability root");
            while !job.walk_is_complete() {
                let copy=job.next_copy_byte_demand().expect("cold original reachability copy");let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_capacity_byte_demand(copy).expect("cold original reachability capacity"),maximum_release_bytes:job.next_release_byte_demand().expect("cold original reachability release"),maximum_depth:job.next_depth_demand().expect("cold original reachability depth")};
                if let ReachabilityStep::Failed {error,..}=job.step(self,grant).expect("cold original reachability turn"){panic!("cold original reachability refused: {error}");}
            }
        }
        job.into_set()
    }

    /// ♻️ Frees every arena slot not in `keep` ([`Store::free`] bumps its generation) — ids for kept
    /// entities are left completely untouched (same index, same generation), so no caller ever
    /// needs to translate an id across a compaction; that is why the return value only reports
    /// counts, not a remap.
    pub fn compact(&mut self, keep: ReachSet) -> Remap {
        let mut job=BodyCompactionJob::new(keep).unwrap_or_else(|(error,_)|panic!("cold original compaction admission: {error}"));
        while !job.terminal_is_empty(){let copy=job.next_copy_byte_demand(self).expect("cold original compact copy");let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_capacity_byte_demand(self,copy).expect("cold original compact capacity"),maximum_release_bytes:job.next_release_byte_demand(self).expect("cold original compact release"),maximum_depth:job.next_depth_demand(self).expect("cold original compact depth")};job.step(self,grant).expect("cold original compact turn");}
        job.freed()
    }

    /// ♻️ Live counts per store, e.g. to assert a compaction actually shrank the body.
    pub fn entity_counts(&self) -> EntityCounts {
        EntityCounts {
            vertices: self.vertices.len(),
            edges: self.edges.len(),
            coedges: self.coedges.len(),
            loops: self.loops.len(),
            faces: self.faces.len(),
            shells: self.shells.len(),
            solids: self.solids.len(),
            curves3: self.curves3.len(),
            curves2: self.curves2.len(),
            surfaces: self.surfaces.len(),
        }
    }

    /// ♻️ Copies every entity of `other` into `self`, offsetting `other`'s [`PersistentLabel`]s
    /// above `self`'s current high-water mark so the two label spaces never collide, and leaving
    /// everything already in `self` completely untouched (same ids, same labels) — the merge
    /// lifecycle a re-import needs so handles minted before the import stay resolvable. Loses
    /// nothing `other` carried (pcurves, tolerances, flip flags all copy verbatim), unlike the
    /// lossy [`Body::from_seed`]/[`Body::to_seed`] round trip which drops pcurves.
    pub fn merge(&mut self, other: &Body) -> MergeMap {
        self.merge_selected(other, None)
    }

    /// ♻️ [`Body::merge`] restricted to the entities of `other` inside `keep` (everything when
    /// `None`) — the extraction primitive behind a minimal standalone shape value. Labels shift by
    /// `self`'s high-water mark exactly like a full merge, so a fresh `self` preserves them verbatim.
    pub fn merge_selected(&mut self, other: &Body, keep: Option<&ReachSet>) -> MergeMap {
        let offset = self.labels.next();
        self.labels = LabelSource::from_next(offset + other.labels.next());
        self.copy_in(other, keep, |l: PersistentLabel| PersistentLabel(l.0 + offset))
    }

    /// ♻️ The one copy walk [`Body::merge_selected`], [`Body::extract`] and [`Body::absorb`] share:
    /// copies the entities of `other` inside `keep` (all when `None`), each carrying `relabel` of its
    /// label. Never touches `self`'s label counter — the caller owns that bookkeeping.
    fn copy_in(&mut self, other: &Body, keep: Option<&ReachSet>, mut relabel: impl FnMut(PersistentLabel) -> PersistentLabel) -> MergeMap {
        let mut curve3_map: HashMap<Curve3Id, Curve3Id> = HashMap::with_capacity(other.curves3.len());
        for (id, c) in other.curves3.iter().filter(|(id, _)| keep.is_none_or(|set| set.curves3.contains(id))) {
            curve3_map.insert(id, self.curves3.insert(c.clone()));
        }
        let mut curve2_map: HashMap<Curve2Id, Curve2Id> = HashMap::with_capacity(other.curves2.len());
        for (id, c) in other.curves2.iter().filter(|(id, _)| keep.is_none_or(|set| set.curves2.contains(id))) {
            curve2_map.insert(id, self.curves2.insert(c.clone()));
        }
        let mut surface_map: HashMap<SurfaceId, SurfaceId> = HashMap::with_capacity(other.surfaces.len());
        for (id, s) in other.surfaces.iter().filter(|(id, _)| keep.is_none_or(|set| set.surfaces.contains(id))) {
            surface_map.insert(id, self.surfaces.insert(s.clone()));
        }

        let mut vertex_map: HashMap<VertexId, VertexId> = HashMap::with_capacity(other.vertices.len());
        for (id, v) in other.vertices.iter().filter(|(id, _)| keep.is_none_or(|set| set.vertices.contains(id))) {
            vertex_map.insert(id, self.vertices.insert(Vertex { position: v.position, tol: v.tol, label: relabel(v.label) }));
        }

        let mut edge_map: HashMap<EdgeId, EdgeId> = HashMap::with_capacity(other.edges.len());
        for (id, e) in other.edges.iter().filter(|(id, _)| keep.is_none_or(|set| set.edges.contains(id))) {
            edge_map.insert(id, self.edges.insert(Edge { curve: curve3_map[&e.curve], range: e.range, v0: vertex_map[&e.v0], v1: vertex_map[&e.v1], tol: e.tol, label: relabel(e.label) }));
        }

        let mut coedge_map: HashMap<CoedgeId, CoedgeId> = HashMap::with_capacity(other.coedges.len());
        for (id, c) in other.coedges.iter().filter(|(id, _)| keep.is_none_or(|set| set.coedges.contains(id))) {
            let placeholder = Coedge { edge: edge_map[&c.edge], forward: c.forward, pcurve: c.pcurve.map(|p| curve2_map[&p]), prange: c.prange, loop_id: null_loop_for_merge(), next: null_coedge_for_merge(), prev: null_coedge_for_merge() };
            coedge_map.insert(id, self.coedges.insert(placeholder));
        }

        let mut loop_map: HashMap<LoopId, LoopId> = HashMap::with_capacity(other.loops.len());
        for (id, l) in other.loops.iter().filter(|(id, _)| keep.is_none_or(|set| set.loops.contains(id))) {
            let placeholder = Loop { first: coedge_map[&l.first], face: null_face_for_merge() };
            loop_map.insert(id, self.loops.insert(placeholder));
        }

        for (id, c) in other.coedges.iter().filter(|(id, _)| keep.is_none_or(|set| set.coedges.contains(id))) {
            let new_id = coedge_map[&id];
            let patched = self.coedges.get_mut(new_id).expect("just inserted");
            patched.next = coedge_map[&c.next];
            patched.prev = coedge_map[&c.prev];
            patched.loop_id = loop_map[&c.loop_id];
        }

        let mut face_map: HashMap<FaceId, FaceId> = HashMap::with_capacity(other.faces.len());
        for (id, f) in other.faces.iter().filter(|(id, _)| keep.is_none_or(|set| set.faces.contains(id))) {
            let outer = f.outer.map(|l| loop_map[&l]);
            let inners: Vec<LoopId> = f.inners.iter().map(|l| loop_map[l]).collect();
            let new_id = self.faces.insert(Face { surface: surface_map[&f.surface], outer, inners: inners.clone(), flipped: f.flipped, tol: f.tol, label: relabel(f.label) });
            if let Some(outer_id) = outer {
                self.loops.get_mut(outer_id).expect("just inserted").face = new_id;
            }
            for inner_id in &inners {
                self.loops.get_mut(*inner_id).expect("just inserted").face = new_id;
            }
            face_map.insert(id, new_id);
        }

        let mut shell_map: HashMap<ShellId, ShellId> = HashMap::with_capacity(other.shells.len());
        for (id, s) in other.shells.iter().filter(|(id, _)| keep.is_none_or(|set| set.shells.contains(id))) {
            let faces = s.faces.iter().map(|f| face_map[f]).collect();
            shell_map.insert(id, self.shells.insert(Shell { faces, label: relabel(s.label) }));
        }

        let mut solid_map: HashMap<SolidId, SolidId> = HashMap::with_capacity(other.solids.len());
        for (id, s) in other.solids.iter().filter(|(id, _)| keep.is_none_or(|set| set.solids.contains(id))) {
            let outer = shell_map[&s.outer];
            let inners = s.inners.iter().map(|sh| shell_map[sh]).collect();
            solid_map.insert(id, self.solids.insert(Solid { outer, inners, label: relabel(s.label) }));
        }

        MergeMap { vertices: vertex_map, edges: edge_map, faces: face_map, shells: shell_map, solids: solid_map }
    }

    /// ✂️ A private working copy of exactly what `roots` reach, with every label preserved and the
    /// label counter continuing at `self`'s — the isolation an operation job runs in, so a cancelled
    /// or failed job leaves `self` untouched by construction. The [`MergeMap`] translates this body's
    /// ids into the copy's.
    pub fn extract(&self, roots: &[EntityRef]) -> (Body, MergeMap) {
        let keep = self.reachable_from(roots);
        let mut out = Body::new();
        out.labels = LabelSource::from_next(self.labels.next());
        let map = out.copy_in(self, Some(&keep), |label| label);
        (out, map)
    }

    /// 📥️ Appends everything `roots` reach in `scratch` — a working copy [`Body::extract`] made when
    /// this body's label counter stood at `scratch_base` — and leaves every entity already in `self`
    /// (same ids, same labels) completely untouched. A label the working copy minted keeps its offset
    /// from `scratch_base` above this body's counter *now* (identical to its own number when nothing
    /// else was minted meanwhile); a label that was copied in from this body and survived into the
    /// result gets a fresh one, so two live entities never share a label.
    pub fn absorb(&mut self, scratch: &Body, roots: &[EntityRef], scratch_base: u64) -> MergeMap {
        let keep = scratch.reachable_from(roots);
        let first = self.labels.next();
        let minted = scratch.labels.next().saturating_sub(scratch_base);
        let mut next_fresh = first + minted;
        let mut fresh: HashMap<u64, u64> = HashMap::new();
        let map = self.copy_in(scratch, Some(&keep), |label| {
            if label.0 >= scratch_base {
                return PersistentLabel(first + (label.0 - scratch_base));
            }
            PersistentLabel(*fresh.entry(label.0).or_insert_with(|| {
                next_fresh += 1;
                next_fresh - 1
            }))
        });
        self.labels = LabelSource::from_next(next_fresh);
        map
    }
}
// #endregion ♻️Reachability
