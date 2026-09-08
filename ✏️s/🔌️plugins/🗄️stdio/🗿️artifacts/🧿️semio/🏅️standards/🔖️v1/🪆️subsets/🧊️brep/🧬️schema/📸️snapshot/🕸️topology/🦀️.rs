//! 🧱️ The B-Rep topology model: `Body` owns arenas of `Vertex/Edge/Coedge/Loop/Face/🐚️Shell/Solid`
//! plus geometry pools (`Curve3`/`Curve2`/`Surface`) that entities reference by id rather than
//! owning directly. Tier-(d) ephemeral working representation per doctrine — a `Body` is a local
//! variable inside a `🔺️diff` constructor or an `InferredField::{plan,dep_input,compute}` body,
//! never a durable struct field, `thread_local!`, or process-global singleton (the
//! `BrepEngineHost` anti-pattern this whole ticket exists to remove — see wave G4 phase 1).
//! Nests its own [`history`] submodule (label/provenance machinery) since no dedicated facet was
//! pre-mounted for it and every consumer already reaches it through `Body`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/🕸️topology` (topology) and
//! `🧰️framework/🔨️modules/🧊️3d/📐️brep/📜️history` (history, nested below) in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL3.

use std::collections::HashMap;

use crate::standards::v1::subsets::brep::schema::snapshot::arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, Store, SurfaceId, VertexId};
use crate::standards::v1::subsets::brep::schema::snapshot::curve::{Curve2, Curve3};
use crate::standards::v1::subsets::brep::schema::snapshot::surface::Surface;
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::{LabelSource, PersistentLabel};
use crate::standards::v1::subsets::brep::schema::snapshot::vector::Pnt3;

// #region 🔖️Entities

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Vertex {
    pub position: Pnt3,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ An edge's `curve` is shared geometry; `range` is *this edge's* portion of that curve's
/// parameter domain, so two edges split from one original edge share `curve` with disjoint ranges.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Edge {
    pub curve: Curve3Id,
    pub range: (f64, f64),
    pub v0: VertexId,
    pub v1: VertexId,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ One face's use of one edge within one loop. `forward` is this use's orientation relative to
/// the edge's own `v0 → v1` direction. `pcurve`/`prange` are the edge's curve reparametrized into
/// the owning face's `(u, v)` domain — `None` only ever transiently, before a producer has filled
/// it in; a face with a missing pcurve on a non-planar surface fails validation (see `validate.rs`).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Coedge {
    pub edge: EdgeId,
    pub forward: bool,
    pub pcurve: Option<Curve2Id>,
    pub prange: (f64, f64),
    pub loop_id: LoopId,
    pub next: CoedgeId,
    pub prev: CoedgeId,
}

/// 🧱️ A closed cycle of coedges bounding one region of a face (the outer boundary, or one hole).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Face {
    pub surface: SurfaceId,
    pub outer: Option<LoopId>,
    pub inners: Vec<LoopId>,
    /// 🧱️ `true` when the face's outward normal is `-normal(surface)` (the surface's own natural
    /// normal, reversed) rather than matching it directly.
    pub flipped: bool,
    pub tol: Tol,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Shell {
    pub faces: Vec<FaceId>,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct Solid {
    pub outer: ShellId,
    pub inners: Vec<ShellId>,
    pub label: PersistentLabel,
}

// #endregion 🔖️Entities

// #region 🔖️Body

/// 🧱️ One B-Rep model: topology arenas + geometry pools + the label counter that stamps every
/// newly-born entity with a [`PersistentLabel`].
#[derive(Clone, Debug, Default, value_derive::ToValue, value_derive::FromValue)]
pub struct Body {
    pub vertices: Store<Vertex, VertexId>,
    pub edges: Store<Edge, EdgeId>,
    pub coedges: Store<Coedge, CoedgeId>,
    pub loops: Store<Loop, LoopId>,
    pub faces: Store<Face, FaceId>,
    pub shells: Store<Shell, ShellId>,
    pub solids: Store<Solid, SolidId>,
    pub curves3: Store<Curve3, Curve3Id>,
    pub curves2: Store<Curve2, Curve2Id>,
    pub surfaces: Store<Surface, SurfaceId>,
    pub labels: LabelSource,
}

impl Body {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Body::default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new_label(&mut self) -> PersistentLabel {
        self.labels.next_label()
    }
}

// #endregion 🔖️Body

// #region 🔖️Traverse

impl Body {
    /// 🧱️ Walks a loop's coedge ring starting from `Loop::first`, following `next` until it
    /// returns to the start. Panics via a debug assertion in the euler layer's invariant checks
    /// if the ring is malformed; callers here get a plain `Vec` (empty if the loop id is stale).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn loop_coedges(&self, loop_id: LoopId) -> Vec<CoedgeId> {
        let Some(lp) = self.loops.get(loop_id) else { return Vec::new() };
        let mut result = Vec::new();
        let mut current = lp.first;
        loop {
            result.push(current);
            let Some(coedge) = self.coedges.get(current) else { break };
            current = coedge.next;
            if current == lp.first {
                break;
            }
            if result.len() > self.coedges.len() {
                break; // malformed ring guard: never loop forever on corrupt data
            }
        }
        result
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_loops(&self, face_id: FaceId) -> Vec<LoopId> {
        let Some(face) = self.faces.get(face_id) else { return Vec::new() };
        let mut result: Vec<LoopId> = face.outer.into_iter().collect();
        result.extend(face.inners.iter().copied());
        result
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn face_coedges(&self, face_id: FaceId) -> Vec<CoedgeId> {
        self.face_loops(face_id).into_iter().flat_map(|l| self.loop_coedges(l)).collect()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn shell_faces(&self, shell_id: ShellId) -> Vec<FaceId> {
        self.shells.get(shell_id).map(|s| s.faces.clone()).unwrap_or_default()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_shells(&self, solid_id: SolidId) -> Vec<ShellId> {
        let Some(solid) = self.solids.get(solid_id) else { return Vec::new() };
        let mut result = vec![solid.outer];
        result.extend(solid.inners.iter().copied());
        result
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn solid_faces(&self, solid_id: SolidId) -> Vec<FaceId> {
        self.solid_shells(solid_id).into_iter().flat_map(|s| self.shell_faces(s)).collect()
    }
    /// 🧱️ The edge's endpoint vertices in `(start, end)` order as seen through `coedge`'s own
    /// orientation (i.e. respecting `forward`, not the underlying edge's raw `v0`/`v1`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn coedge_endpoints(&self, coedge_id: CoedgeId) -> Option<(VertexId, VertexId)> {
        let coedge = self.coedges.get(coedge_id)?;
        let edge = self.edges.get(coedge.edge)?;
        Some(if coedge.forward { (edge.v0, edge.v1) } else { (edge.v1, edge.v0) })
    }
    /// 🧱️ Every vertex incident to at least one edge that references it as `v0` or `v1`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn vertex_edges(&self, vertex_id: VertexId) -> Vec<EdgeId> {
        self.edges.iter().filter(|(_, e)| e.v0 == vertex_id || e.v1 == vertex_id).map(|(id, _)| id).collect()
    }
    /// 🧱️ Every coedge that uses `edge_id` (both orientations, both faces if the edge is shared).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn edge_coedges(&self, edge_id: EdgeId) -> Vec<CoedgeId> {
        self.coedges.iter().filter(|(_, c)| c.edge == edge_id).map(|(id, _)| id).collect()
    }
}

// #endregion 🔖️Traverse

// #region 🔖️Remap

impl Body {
    /// 🧱️ A deep copy of the entire body: every arena's entries are copied into a fresh `Body`
    /// with (generally) different arena indices, but *the same* [`PersistentLabel`]s — used
    /// wherever a caller needs an independent, mutable working copy without disturbing the
    /// original (e.g. undo snapshots, before the document layer's smarter delta-based history).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deep_copy(&self) -> Body {
        self.clone()
    }
}

// #endregion 🔖️Remap

// #region 🔖️Seed

/// 🌱 One restored vertex, keyed by its own [`PersistentLabel`] rather than a persisted string id
/// — translating a snapshot's own id convention into `PersistentLabel` is the caller's job (see
/// [`crate::standards::v1::subsets::brep::schema::snapshot::topology::history`]'s own docstring on why a label is never reused), done once per
/// diff-constructor call, not baked into this ephemeral seed's own shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
pub struct SeedVertex {
    pub label: PersistentLabel,
    pub position: Pnt3,
    pub tol: Tol,
}

/// 🌱 One restored edge; `v0`/`v1` reference [`SeedVertex::label`]s, not arena ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
/// [`crate::standards::v1::subsets::brep::schema::diff::euler::make_loop`]'s own docstring), so an ordinal index is the only address.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
pub struct SeedShell {
    pub label: PersistentLabel,
    pub faces: Vec<PersistentLabel>,
}

/// 🌱 One restored solid; `outer`/`inners` reference [`SeedShell::label`]s.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
    /// outside [`crate::standards::v1::subsets::brep::schema::diff::euler`] allowed to construct topology entities directly. This
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
                crate::standards::v1::subsets::brep::schema::diff::euler::make_loop(&mut body, placeholder, &members)
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

// #region 🔖️History

pub mod history {
    //! 📜️ Operation provenance: a [`PersistentLabel`] assigned once at an entity's birth and never
    //! reused, plus the [`OpDelta`] every mutating operation in [`crate::standards::v1::subsets::brep::schema::diff::euler`] returns.
    //! **Host authority:** `LabelSource` lives only inside a `Body` owned by engine compute or cache.

    // #region 🔖️Labels

    /// 📜️ A stable identity for one topological entity, assigned from a per-`Body` monotonically
    /// increasing counter at birth. Unlike an arena [`crate::standards::v1::subsets::brep::schema::snapshot::arena::ArenaId`] (which can be reused
    /// after removal once its generation increments), a label is never reused — it survives arena
    /// compaction and is the identity the document layer's persistent naming keys off of.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
    #[value(transparent)]
    pub struct PersistentLabel(pub u64);

    /// 📜️ Issues fresh, never-repeating labels for one `Body`.
    #[derive(Clone, Debug, Default, value_derive::ToValue, value_derive::FromValue)]
    pub struct LabelSource {
        next: u64,
    }

    impl LabelSource {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            LabelSource { next: 0 }
        }
        /// 📜️ Seeds the counter at an explicit high-water mark rather than restarting at 0 — used by
        /// [`crate::standards::v1::subsets::brep::schema::snapshot::topology::Body`]'s `from_seed` so a rebuild from a persisted seed carries
        /// the label numbering forward instead of colliding with the labels it is restoring.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn from_next(next: u64) -> Self {
            LabelSource { next }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn next_label(&mut self) -> PersistentLabel {
            let label = PersistentLabel(self.next);
            self.next += 1;
            label
        }
        /// 📜️ The next label this source would mint — the high-water mark a seed must carry forward
        /// (see [`Self::from_next`]) so a rebuilt `Body` never re-mints a label already in use.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn next(&self) -> u64 {
            self.next
        }
    }

    // #endregion 🔖️Labels

    // #region 🔖️Delta

    /// 📜️ The provenance of one mutating operation, in terms of stable [`PersistentLabel`]s rather
    /// than arena ids (which can be reused after removal): every entity the operation created, every
    /// entity it modified (paired with its label so the same entity's before/after states are
    /// linkable), and every entity it deleted.
    #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
    pub struct OpDelta {
        pub generated: Vec<PersistentLabel>,
        pub modified: Vec<PersistentLabel>,
        pub deleted: Vec<PersistentLabel>,
    }

    impl OpDelta {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn is_empty(&self) -> bool {
            self.generated.is_empty() && self.modified.is_empty() && self.deleted.is_empty()
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn merge(&mut self, other: OpDelta) {
            self.generated.extend(other.generated);
            self.modified.extend(other.modified);
            self.deleted.extend(other.deleted);
        }
    }

    /// 📜️ Accumulates an [`OpDelta`] as a checked editor runs; passed by every [`crate::standards::v1::subsets::brep::schema::diff::euler`]
    /// operator so no operation can forget to log what it touched. `record_deleted` and friends are
    /// idempotent against duplicate reporting within one operation, since some editors touch the same
    /// entity more than once (e.g. splitting an edge modifies the vertex on both sides).
    #[derive(Clone, Debug, Default)]
    pub struct OpRecorder {
        delta: OpDelta,
    }

    impl OpRecorder {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            OpRecorder::default()
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn record_generated(&mut self, label: PersistentLabel) {
            if !self.delta.generated.contains(&label) {
                self.delta.generated.push(label);
            }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn record_modified(&mut self, label: PersistentLabel) {
            if !self.delta.modified.contains(&label) && !self.delta.generated.contains(&label) {
                self.delta.modified.push(label);
            }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn record_deleted(&mut self, label: PersistentLabel) {
            self.delta.generated.retain(|l| *l != label);
            self.delta.modified.retain(|l| *l != label);
            if !self.delta.deleted.contains(&label) {
                self.delta.deleted.push(label);
            }
        }
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn into_delta(self) -> OpDelta {
            self.delta
        }
    }

    // #endregion 🔖️Delta

    // #region 🔖️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️history-unit/🦀️.rs");
    // #endregion 🔖️Tests
}

// #endregion 🔖️History

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests

// #region ♻️Reachability

/// ♻️ One arena entity a [`Body::reachable_from`] walk can start from or land on — the traversal
/// root type for garbage collection (see [`Body::compact`]) and for the set of entities a live
/// engine handle keeps alive across a compaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityRef {
    Vertex(VertexId),
    Edge(EdgeId),
    Coedge(CoedgeId),
    Loop(LoopId),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
    Curve3(Curve3Id),
    Curve2(Curve2Id),
    Surface(SurfaceId),
}

/// ♻️ The set of arena entities one [`Body::reachable_from`] walk visited, keyed by store — every
/// id NOT present here, for a store's current [`Store::ids`], is exactly what [`Body::compact`]
/// is safe to free.
#[derive(Clone, Debug, Default)]
pub struct ReachSet {
    pub vertices: std::collections::HashSet<VertexId>,
    pub edges: std::collections::HashSet<EdgeId>,
    pub coedges: std::collections::HashSet<CoedgeId>,
    pub loops: std::collections::HashSet<LoopId>,
    pub faces: std::collections::HashSet<FaceId>,
    pub shells: std::collections::HashSet<ShellId>,
    pub solids: std::collections::HashSet<SolidId>,
    pub curves3: std::collections::HashSet<Curve3Id>,
    pub curves2: std::collections::HashSet<Curve2Id>,
    pub surfaces: std::collections::HashSet<SurfaceId>,
}

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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn reachable_from(&self, roots: &[EntityRef]) -> ReachSet {
        let mut set = ReachSet::default();
        for &root in roots {
            self.mark_reachable(root, &mut set);
        }
        set
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn mark_reachable(&self, root: EntityRef, set: &mut ReachSet) {
        match root {
            EntityRef::Solid(id) => {
                if !set.solids.insert(id) {
                    return;
                }
                if let Some(solid) = self.solids.get(id) {
                    self.mark_reachable(EntityRef::Shell(solid.outer), set);
                    for &shell in &solid.inners {
                        self.mark_reachable(EntityRef::Shell(shell), set);
                    }
                }
            }
            EntityRef::Shell(id) => {
                if !set.shells.insert(id) {
                    return;
                }
                if let Some(shell) = self.shells.get(id) {
                    for &face in &shell.faces {
                        self.mark_reachable(EntityRef::Face(face), set);
                    }
                }
            }
            EntityRef::Face(id) => {
                if !set.faces.insert(id) {
                    return;
                }
                if let Some(face) = self.faces.get(id) {
                    set.surfaces.insert(face.surface);
                    if let Some(outer) = face.outer {
                        self.mark_reachable(EntityRef::Loop(outer), set);
                    }
                    for &inner in &face.inners {
                        self.mark_reachable(EntityRef::Loop(inner), set);
                    }
                }
            }
            EntityRef::Loop(id) => {
                if !set.loops.insert(id) {
                    return;
                }
                for coedge in self.loop_coedges(id) {
                    self.mark_reachable(EntityRef::Coedge(coedge), set);
                }
            }
            EntityRef::Coedge(id) => {
                if !set.coedges.insert(id) {
                    return;
                }
                if let Some(coedge) = self.coedges.get(id) {
                    if let Some(pcurve) = coedge.pcurve {
                        set.curves2.insert(pcurve);
                    }
                    self.mark_reachable(EntityRef::Edge(coedge.edge), set);
                }
            }
            EntityRef::Edge(id) => {
                if !set.edges.insert(id) {
                    return;
                }
                if let Some(edge) = self.edges.get(id) {
                    set.curves3.insert(edge.curve);
                    self.mark_reachable(EntityRef::Vertex(edge.v0), set);
                    self.mark_reachable(EntityRef::Vertex(edge.v1), set);
                }
            }
            EntityRef::Vertex(id) => {
                set.vertices.insert(id);
            }
            EntityRef::Curve3(id) => {
                set.curves3.insert(id);
            }
            EntityRef::Curve2(id) => {
                set.curves2.insert(id);
            }
            EntityRef::Surface(id) => {
                set.surfaces.insert(id);
            }
        }
    }

    /// ♻️ Frees every arena slot not in `keep` ([`Store::free`] bumps its generation) — ids for kept
    /// entities are left completely untouched (same index, same generation), so no caller ever
    /// needs to translate an id across a compaction; that is why the return value only reports
    /// counts, not a remap.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compact(&mut self, keep: &ReachSet) -> Remap {
        let mut freed = Remap::default();
        for id in self.vertices.ids().collect::<Vec<_>>() {
            if !keep.vertices.contains(&id) && self.vertices.free(id) {
                freed.freed_vertices += 1;
            }
        }
        for id in self.edges.ids().collect::<Vec<_>>() {
            if !keep.edges.contains(&id) && self.edges.free(id) {
                freed.freed_edges += 1;
            }
        }
        for id in self.coedges.ids().collect::<Vec<_>>() {
            if !keep.coedges.contains(&id) && self.coedges.free(id) {
                freed.freed_coedges += 1;
            }
        }
        for id in self.loops.ids().collect::<Vec<_>>() {
            if !keep.loops.contains(&id) && self.loops.free(id) {
                freed.freed_loops += 1;
            }
        }
        for id in self.faces.ids().collect::<Vec<_>>() {
            if !keep.faces.contains(&id) && self.faces.free(id) {
                freed.freed_faces += 1;
            }
        }
        for id in self.shells.ids().collect::<Vec<_>>() {
            if !keep.shells.contains(&id) && self.shells.free(id) {
                freed.freed_shells += 1;
            }
        }
        for id in self.solids.ids().collect::<Vec<_>>() {
            if !keep.solids.contains(&id) && self.solids.free(id) {
                freed.freed_solids += 1;
            }
        }
        for id in self.curves3.ids().collect::<Vec<_>>() {
            if !keep.curves3.contains(&id) && self.curves3.free(id) {
                freed.freed_curves3 += 1;
            }
        }
        for id in self.curves2.ids().collect::<Vec<_>>() {
            if !keep.curves2.contains(&id) && self.curves2.free(id) {
                freed.freed_curves2 += 1;
            }
        }
        for id in self.surfaces.ids().collect::<Vec<_>>() {
            if !keep.surfaces.contains(&id) && self.surfaces.free(id) {
                freed.freed_surfaces += 1;
            }
        }
        freed
    }

    /// ♻️ Live counts per store, e.g. to assert a compaction actually shrank the body.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn merge(&mut self, other: &Body) -> MergeMap {
        let offset = self.labels.next();
        self.labels = LabelSource::from_next(offset + other.labels.next());
        let relabel = |l: PersistentLabel| PersistentLabel(l.0 + offset);

        let mut curve3_map: HashMap<Curve3Id, Curve3Id> = HashMap::with_capacity(other.curves3.len());
        for (id, c) in other.curves3.iter() {
            curve3_map.insert(id, self.curves3.insert(c.clone()));
        }
        let mut curve2_map: HashMap<Curve2Id, Curve2Id> = HashMap::with_capacity(other.curves2.len());
        for (id, c) in other.curves2.iter() {
            curve2_map.insert(id, self.curves2.insert(c.clone()));
        }
        let mut surface_map: HashMap<SurfaceId, SurfaceId> = HashMap::with_capacity(other.surfaces.len());
        for (id, s) in other.surfaces.iter() {
            surface_map.insert(id, self.surfaces.insert(s.clone()));
        }

        let mut vertex_map: HashMap<VertexId, VertexId> = HashMap::with_capacity(other.vertices.len());
        for (id, v) in other.vertices.iter() {
            vertex_map.insert(id, self.vertices.insert(Vertex { position: v.position, tol: v.tol, label: relabel(v.label) }));
        }

        let mut edge_map: HashMap<EdgeId, EdgeId> = HashMap::with_capacity(other.edges.len());
        for (id, e) in other.edges.iter() {
            edge_map.insert(id, self.edges.insert(Edge { curve: curve3_map[&e.curve], range: e.range, v0: vertex_map[&e.v0], v1: vertex_map[&e.v1], tol: e.tol, label: relabel(e.label) }));
        }

        let mut coedge_map: HashMap<CoedgeId, CoedgeId> = HashMap::with_capacity(other.coedges.len());
        for (id, c) in other.coedges.iter() {
            let placeholder = Coedge { edge: edge_map[&c.edge], forward: c.forward, pcurve: c.pcurve.map(|p| curve2_map[&p]), prange: c.prange, loop_id: null_loop_for_merge(), next: null_coedge_for_merge(), prev: null_coedge_for_merge() };
            coedge_map.insert(id, self.coedges.insert(placeholder));
        }

        let mut loop_map: HashMap<LoopId, LoopId> = HashMap::with_capacity(other.loops.len());
        for (id, l) in other.loops.iter() {
            let placeholder = Loop { first: coedge_map[&l.first], face: null_face_for_merge() };
            loop_map.insert(id, self.loops.insert(placeholder));
        }

        for (id, c) in other.coedges.iter() {
            let new_id = coedge_map[&id];
            let patched = self.coedges.get_mut(new_id).expect("just inserted");
            patched.next = coedge_map[&c.next];
            patched.prev = coedge_map[&c.prev];
            patched.loop_id = loop_map[&c.loop_id];
        }

        let mut face_map: HashMap<FaceId, FaceId> = HashMap::with_capacity(other.faces.len());
        for (id, f) in other.faces.iter() {
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
        for (id, s) in other.shells.iter() {
            let faces = s.faces.iter().map(|f| face_map[f]).collect();
            shell_map.insert(id, self.shells.insert(Shell { faces, label: relabel(s.label) }));
        }

        let mut solid_map: HashMap<SolidId, SolidId> = HashMap::with_capacity(other.solids.len());
        for (id, s) in other.solids.iter() {
            let outer = shell_map[&s.outer];
            let inners = s.inners.iter().map(|sh| shell_map[sh]).collect();
            solid_map.insert(id, self.solids.insert(Solid { outer, inners, label: relabel(s.label) }));
        }

        MergeMap { vertices: vertex_map, edges: edge_map, faces: face_map, shells: shell_map, solids: solid_map }
    }
}

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️reachability/🦀️.rs"]
mod reachability_tests;
// #endregion 🔖️Tests

// #endregion ♻️Reachability
