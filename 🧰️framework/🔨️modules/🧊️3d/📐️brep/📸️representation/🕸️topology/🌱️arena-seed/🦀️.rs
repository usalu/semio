//! 🌱️ Arena seed: the seed records of every entity kind, the body arena seed and the body construction from a seed. See [`super`].

use super::*;

// #region 🔖️Seed
/// 🌱 One restored vertex, keyed by its own [`PersistentLabel`] rather than a persisted string id
/// — translating a snapshot's own id convention into `PersistentLabel` is the caller's job (see
/// [`crate::topology::history`]'s own docstring on why a label is never reused), done once per
/// diff-constructor call, not baked into this ephemeral seed's own shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeedVertex {
    pub label: PersistentLabel,
    pub position: Pnt3,
    pub tol: Tol,
}

/// 🌱 One restored edge; `v0`/`v1` reference [`SeedVertex::label`]s, not arena ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeedEdge {
    pub label: PersistentLabel,
    pub v0: PersistentLabel,
    pub v1: PersistentLabel,
    pub curve: Curve3,
    pub range: (f64, f64),
    pub tol: Tol,
}

/// 🌱 One restored face; `outer`/`inners` are indices into [`BrepArenaSeed::loops`] — loops carry
/// no [`PersistentLabel`] of their own (structural, not independently document-nameable, per
/// [`crate::topology::make_loop`]'s own docstring), so an ordinal index is the only address.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeedFace {
    pub label: PersistentLabel,
    pub surface: Surface,
    pub outer: Option<usize>,
    pub inners: Vec<usize>,
    pub flipped: bool,
    pub tol: Tol,
}

/// 🌱 One restored shell; `faces` references [`SeedFace::label`]s.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeedShell {
    pub label: PersistentLabel,
    pub faces: Vec<PersistentLabel>,
}

/// 🌱 One restored solid; `outer`/`inners` reference [`SeedShell::label`]s.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct SeedSolid {
    pub label: PersistentLabel,
    pub outer: PersistentLabel,
    pub inners: Vec<PersistentLabel>,
}

/// 🌱 A pure, ephemeral, tier-(d) working representation of a whole [`Body`] — the seed
/// [`Body::from_seed`]/[`Body::to_seed`] round-trip through. Never persisted, never registered as
/// an artifact schema, never a second `SemioBrepSnapshot`: it exists only for the span of one
/// diff-constructor call, built from whatever snapshot the caller (stdio's `🧊️brep` subset, once
/// its mutation triads land) owns.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
pub struct BrepArenaSeed {
    /// 🌱 The label high-water-mark to seed [`LabelSource::from_next`] with — MUST be carried
    /// forward from the persisted snapshot, never reset to 0, or two independent diff-constructor
    /// calls against the same `base` would mint colliding labels the instant both merge.
    pub next_label: u64,
    pub vertices: Vec<SeedVertex>,
    pub edges: Vec<SeedEdge>,
    pub loops: Vec<Vec<(PersistentLabel, bool)>>,
    pub faces: Vec<SeedFace>,
    pub shells: Vec<SeedShell>,
    pub solids: Vec<SeedSolid>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn placeholder_face_for_build() -> FaceId {
    ArenaId::from_raw(0, 0)
}

impl Body {
    /// 🌱 Reconstructs a `Body` from `seed`, inserting directly into each `Store` — the ONE place
    /// outside [`semio_framework_3d_brep_construction::euler`] allowed to construct topology entities directly. This
    /// mirrors euler's own "the *only* functions permitted to mutate a `Body`" docstring rather
    /// than violating it: `from_seed` constructs a *fresh* `Body`, it does not mutate an existing
    /// one. It must NOT call `euler::make_vertex`/`make_edge`/`add_face`/`add_shell`/`add_solid` —
    /// those mint a *fresh* label every time, which is correct for a genuine user-facing create but
    /// wrong here, where the whole point is restoring each entity's *existing* label from the seed;
    /// calling them would silently break the round-trip law below. `euler::make_loop` is the one
    /// euler function this DOES call, because loops carry no label to preserve or break.
    pub fn from_seed(seed: &BrepArenaSeed) -> Self {
        let mut body = Body::new();
        body.labels = LabelSource::from_next(seed.next_label);

        let mut vertex_ids: HashMap<PersistentLabel, VertexId> = HashMap::with_capacity(seed.vertices.len());
        for v in &seed.vertices {
            let id = body.vertices.insert(Vertex { position: v.position, tol: v.tol, label: v.label });
            vertex_ids.insert(v.label, id);
        }

        let mut edge_ids: HashMap<PersistentLabel, EdgeId> = HashMap::with_capacity(seed.edges.len());
        for e in &seed.edges {
            let curve_id = body.curves3.insert(e.curve.clone());
            let id = body.edges.insert(Edge { curve: curve_id, range: e.range, v0: vertex_ids[&e.v0], v1: vertex_ids[&e.v1], tol: e.tol, label: e.label });
            edge_ids.insert(e.label, id);
        }

        let placeholder = placeholder_face_for_build();
        let loop_ids: Vec<LoopId> = seed
            .loops
            .iter()
            .map(|ring| {
                let members: Vec<(EdgeId, bool)> = ring.iter().map(|(label, forward)| (edge_ids[label], *forward)).collect();
                make_loop(&mut body, placeholder, &members)
            })
            .collect();

        let mut face_ids: HashMap<PersistentLabel, FaceId> = HashMap::with_capacity(seed.faces.len());
        for f in &seed.faces {
            let surface_id = body.surfaces.insert(f.surface.clone());
            let outer = f.outer.map(|i| loop_ids[i]);
            let inners: Vec<LoopId> = f.inners.iter().map(|&i| loop_ids[i]).collect();
            let id = body.faces.insert(Face { surface: surface_id, outer, inners: inners.clone(), flipped: f.flipped, tol: f.tol, label: f.label });
            if let Some(outer_id) = outer {
                body.loops.get_mut(outer_id).expect("just inserted").face = id;
            }
            for inner_id in &inners {
                body.loops.get_mut(*inner_id).expect("just inserted").face = id;
            }
            face_ids.insert(f.label, id);
        }

        let mut shell_ids: HashMap<PersistentLabel, ShellId> = HashMap::with_capacity(seed.shells.len());
        for s in &seed.shells {
            let faces = s.faces.iter().map(|l| face_ids[l]).collect();
            let id = body.shells.insert(Shell { faces, label: s.label });
            shell_ids.insert(s.label, id);
        }

        for s in &seed.solids {
            let outer = shell_ids[&s.outer];
            let inners = s.inners.iter().map(|l| shell_ids[l]).collect();
            body.solids.insert(Solid { outer, inners, label: s.label });
        }

        body
    }

    /// 🌱 The mirror-image half of [`Body::from_seed`] — extracts an equivalent [`BrepArenaSeed`]
    /// from `self`. Needed by the round-trip law (`Body::from_seed(&seed).to_seed() == seed`) and
    /// by a future diff constructor, which reads post-op state back out this way to translate into
    /// a `SemioBrepDiff` via the label↔snapshot-id map it owns.
    // 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
    pub fn to_seed(&self) -> BrepArenaSeed {
        let vertex_label = |id: VertexId| -> PersistentLabel { self.vertices.get(id).expect("live vertex").label };
        let edge_label = |id: EdgeId| -> PersistentLabel { self.edges.get(id).expect("live edge").label };
        let face_label = |id: FaceId| -> PersistentLabel { self.faces.get(id).expect("live face").label };
        let shell_label = |id: ShellId| -> PersistentLabel { self.shells.get(id).expect("live shell").label };

        let vertices: Vec<SeedVertex> = self.vertices.iter().map(|(_, v)| SeedVertex { label: v.label, position: v.position, tol: v.tol }).collect();

        let edges: Vec<SeedEdge> = self.edges.iter().map(|(_, e)| SeedEdge { label: e.label, v0: vertex_label(e.v0), v1: vertex_label(e.v1), curve: self.curves3.get(e.curve).expect("live curve").clone(), range: e.range, tol: e.tol }).collect();

        // One entry per distinct LoopId, in the order faces first reference it — the same order
        // `from_seed` assigns indices in, which is what the round-trip law needs to hold.
        let mut loops: Vec<Vec<(PersistentLabel, bool)>> = Vec::new();
        let mut loop_index: HashMap<LoopId, usize> = HashMap::new();
        let mut faces: Vec<SeedFace> = Vec::with_capacity(self.faces.len());
        for (_, f) in self.faces.iter() {
            let mut resolve_loop = |loop_id: LoopId| -> usize {
                if let Some(&i) = loop_index.get(&loop_id) {
                    return i;
                }
                let ring: Vec<(PersistentLabel, bool)> = self.loop_coedges(loop_id).into_iter().filter_map(|cid| self.coedges.get(cid).map(|c| (edge_label(c.edge), c.forward))).collect();
                let i = loops.len();
                loops.push(ring);
                loop_index.insert(loop_id, i);
                i
            };
            let outer = f.outer.map(&mut resolve_loop);
            let inners: Vec<usize> = f.inners.iter().map(|&l| resolve_loop(l)).collect();
            faces.push(SeedFace { label: f.label, surface: self.surfaces.get(f.surface).expect("live surface").clone(), outer, inners, flipped: f.flipped, tol: f.tol });
        }

        let shells: Vec<SeedShell> = self.shells.iter().map(|(_, s)| SeedShell { label: s.label, faces: s.faces.iter().map(|&f| face_label(f)).collect() }).collect();

        let solids: Vec<SeedSolid> = self.solids.iter().map(|(_, s)| SeedSolid { label: s.label, outer: shell_label(s.outer), inners: s.inners.iter().map(|&sh| shell_label(sh)).collect() }).collect();

        BrepArenaSeed { next_label: self.labels.next(), vertices, edges, loops, faces, shells, solids }
    }
}
// #endregion 🔖️Seed
