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
pub use semio_framework_mesh_engine::HistoryFoldSet;

use crate::brep::representation::arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, Store, SurfaceId, VertexId};
use crate::brep::representation::curve::{Curve2, Curve3};
use crate::brep::representation::surface::Surface;
use crate::brep::representation::tolerance::Tol;
use crate::brep::representation::topology::history::{LabelSource, PersistentLabel};
use crate::brep::representation::vector::Pnt3;

#[cfg(test)]
std::thread_local! { pub(crate) static EDGE_USE_PROBES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

pub(crate) fn coedge_uses_edge(coedge: &Coedge, edge: EdgeId) -> bool {
    #[cfg(test)]
    EDGE_USE_PROBES.with(|probes| probes.set(probes.get() + 1));
    coedge.edge == edge
}

// #region 🔖️Entities

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Vertex {
    pub position: Pnt3,
    pub tol: Tol,
    pub label: PersistentLabel,
}

/// 🧱️ An edge's `curve` is shared geometry; `range` is *this edge's* portion of that curve's
/// parameter domain, so two edges split from one original edge share `curve` with disjoint ranges.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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
#[value(crate = "::protocol::value")]
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
#[value(crate = "::protocol::value")]
pub struct Loop {
    pub first: CoedgeId,
    pub face: FaceId,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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
#[value(crate = "::protocol::value")]
pub struct Shell {
    pub faces: Vec<FaceId>,
    pub label: PersistentLabel,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct Solid {
    pub outer: ShellId,
    pub inners: Vec<ShellId>,
    pub label: PersistentLabel,
}

// #endregion 🔖️Entities

// #region 🔖️Body

/// 🧱️ One B-Rep model: topology arenas + geometry pools + the label counter that stamps every
/// newly-born entity with a [`PersistentLabel`].
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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

semio_framework_value::artifact_retire_struct!(Vertex {position,tol,label});
semio_framework_value::artifact_retire_struct!(Edge {curve,range,v0,v1,tol,label});
semio_framework_value::artifact_retire_struct!(Coedge {edge,forward,pcurve,prange,loop_id,next,prev});
semio_framework_value::artifact_retire_struct!(Loop {first,face});
semio_framework_value::artifact_retire_struct!(Face {surface,outer,inners,flipped,tol,label});
semio_framework_value::artifact_retire_struct!(Shell {faces,label});
semio_framework_value::artifact_retire_struct!(Solid {outer,inners,label});
semio_framework_value::artifact_retire_struct!(Body {vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces,labels});

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
        self.coedges.iter().filter(|(_, c)| coedge_uses_edge(c, edge_id)).map(|(id, _)| id).collect()
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
/// [`crate::brep::representation::topology::history`]'s own docstring on why a label is never reused), done once per
/// diff-constructor call, not baked into this ephemeral seed's own shape.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
pub struct SeedVertex {
    pub label: PersistentLabel,
    pub position: Pnt3,
    pub tol: Tol,
}

/// 🌱 One restored edge; `v0`/`v1` reference [`SeedVertex::label`]s, not arena ids.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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
/// [`crate::brep::operations::euler::make_loop`]'s own docstring), so an ordinal index is the only address.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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
#[value(crate = "::protocol::value")]
pub struct SeedShell {
    pub label: PersistentLabel,
    pub faces: Vec<PersistentLabel>,
}

/// 🌱 One restored solid; `outer`/`inners` reference [`SeedShell::label`]s.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
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
#[value(crate = "::protocol::value")]
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
    /// outside [`crate::brep::operations::euler`] allowed to construct topology entities directly. This
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
                crate::brep::operations::euler::make_loop(&mut body, placeholder, &members)
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
    //! reused, plus the [`OpDelta`] every mutating operation in [`crate::brep::operations::euler`] returns.
    //! **Host authority:** `LabelSource` lives only inside a `Body` owned by engine compute or cache.

    // #region 🔖️Labels

    /// 📜️ A stable identity for one topological entity, assigned from a per-`Body` monotonically
    /// increasing counter at birth. Unlike an arena [`crate::brep::representation::arena::ArenaId`] (which can be reused
    /// after removal once its generation increments), a label is never reused — it survives arena
    /// compaction and is the identity the document layer's persistent naming keys off of.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
    #[value(transparent)]
    pub struct PersistentLabel(pub u64);

    /// 📜️ Issues fresh, never-repeating labels for one `Body`.
    #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::protocol::value")]
    pub struct LabelSource {
        next: u64,
    }

    semio_framework_value::artifact_retire_leaf!(PersistentLabel);
    semio_framework_value::artifact_retire_struct!(LabelSource {next});

    impl LabelSource {
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new() -> Self {
            LabelSource { next: 0 }
        }
        /// 📜️ Seeds the counter at an explicit high-water mark rather than restarting at 0 — used by
        /// [`crate::brep::representation::topology::Body`]'s `from_seed` so a rebuild from a persisted seed carries
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
#[value(crate = "::protocol::value")]
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

    /// 📜️ Accumulates an [`OpDelta`] as a checked editor runs; passed by every [`crate::brep::operations::euler`]
    /// operator so no operation can forget to log what it touched. `record_deleted` and friends are
    /// idempotent against duplicate reporting within one operation, since some editors touch the same
    /// entity more than once (e.g. splitting an edge modifies the vertex on both sides).
    #[derive(Clone, Debug, Default)]
    pub struct OpRecorder {
        delta: OpDelta,
        owned: Vec<super::EntityRef>,
    }

    semio_framework_value::artifact_retire_struct!(OpDelta {generated,modified,deleted});
    semio_framework_value::artifact_retire_struct!(OpRecorder {delta,owned});

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
/// 🧱️ Appends newly generated disjoint provenance without scanning older labels.
        pub fn append_disjoint(&mut self, mut other: Self) {
            assert!(other.delta.modified.is_empty() && other.delta.deleted.is_empty());
            self.delta.generated.append(&mut other.delta.generated);
            self.owned.append(&mut other.owned);
        }
        /// 🧱️ Retains exact private triangle entities for reverse bounded retirement.
        pub fn own_triangle(&mut self, body: &super::Body, face_id: super::FaceId) {
            use super::EntityRef;
            let face = body.faces.get(face_id).expect("created triangle");
            self.owned.push(EntityRef::Surface(face.surface));
            let loop_id = face.outer.expect("created triangle outer");
            let first = body.loops.get(loop_id).expect("created triangle loop").first;
            let mut next = first;
            for _ in 0..3 {
                let coedge = body.coedges.get(next).expect("created triangle coedge");
                let edge = body.edges.get(coedge.edge).expect("created triangle edge");
                self.owned.extend([EntityRef::Vertex(edge.v0), EntityRef::Vertex(edge.v1), EntityRef::Curve3(edge.curve), EntityRef::Edge(coedge.edge)]);
                if let Some(pcurve) = coedge.pcurve { self.owned.push(EntityRef::Curve2(pcurve)); }
                self.owned.push(EntityRef::Coedge(next));
                next = coedge.next;
            }
            assert_eq!(next,first);
            self.owned.extend([EntityRef::Loop(loop_id), EntityRef::Face(face_id)]);
        }
        /// 🧱️ Includes a newly created shell or solid in this operation's retirement authority.
        pub fn own_entity(&mut self, entity: super::EntityRef) { self.owned.push(entity); }
        /// 🎟️ Bytes of POD provenance and owned-ID allocations retained through final publication.
        pub fn retirement_bytes(&self)->usize {self.owned.capacity()*std::mem::size_of::<super::EntityRef>()+self.delta.generated.capacity()*std::mem::size_of::<PersistentLabel>()}
        /// 🎟️ Borrows the original rollback tail without removing its ownership.
        pub(crate) fn last_owned(&self)->Option<super::EntityRef> {self.owned.last().copied()}
        /// 🎟️ Removes the original rollback authority only after its entity transfer is admitted.
        pub(crate) fn pop_retired_owned(&mut self,entity:super::EntityRef) {assert_eq!(self.owned.pop(),Some(entity));}

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

semio_framework_value::artifact_retire_leaf!(EntityRef);

/// ♻️ The set of arena entities one [`Body::reachable_from`] walk visited, keyed by store — every
/// id NOT present here, for a store's current [`Store::ids`], is exactly what [`Body::compact`]
/// is safe to free.
#[derive(Clone, Debug, Default)]
pub struct ReachSet {
    pub vertices: HistoryFoldSet<VertexId>,
    pub edges: HistoryFoldSet<EdgeId>,
    pub coedges: HistoryFoldSet<CoedgeId>,
    pub loops: HistoryFoldSet<LoopId>,
    pub faces: HistoryFoldSet<FaceId>,
    pub shells: HistoryFoldSet<ShellId>,
    pub solids: HistoryFoldSet<SolidId>,
    pub curves3: HistoryFoldSet<Curve3Id>,
    pub curves2: HistoryFoldSet<Curve2Id>,
    pub surfaces: HistoryFoldSet<SurfaceId>,
}

semio_framework_value::artifact_retire_struct!(ReachSet {vertices,edges,coedges,loops,faces,shells,solids,curves3,curves2,surfaces});

#[path="♻️compaction/🦀️.rs"]
mod compaction;
pub use compaction::BodyCompactionJob;

#[cfg(test)]
#[path="🧪️tests/🔬️compaction/🦀️.rs"]
mod compaction_tests;


use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
impl ReachSet {
    fn mark_demand(&self,root:EntityRef,copy:usize)->Result<RetirementDemand,ValueError> {
        macro_rules! demand {($set:expr,$id:expr)=>{if $set.contains(&$id){Ok(RetirementDemand {depth:1,..Default::default()})}else{Ok(RetirementDemand {copy_bytes:$set.next_insert_copy_byte_demand(&$id)?,capacity_bytes:$set.next_insert_capacity_byte_demand(&$id,copy)?,release_bytes:$set.next_insert_release_byte_demand(&$id)?,depth:$set.next_insert_depth_demand(&$id)?})}}}
        match root {
            EntityRef::Vertex(id)=>demand!(self.vertices,id),
            EntityRef::Edge(id)=>demand!(self.edges,id),
            EntityRef::Coedge(id)=>demand!(self.coedges,id),
            EntityRef::Loop(id)=>demand!(self.loops,id),
            EntityRef::Face(id)=>demand!(self.faces,id),
            EntityRef::Shell(id)=>demand!(self.shells,id),
            EntityRef::Solid(id)=>demand!(self.solids,id),
            EntityRef::Curve3(id)=>demand!(self.curves3,id),
            EntityRef::Curve2(id)=>demand!(self.curves2,id),
            EntityRef::Surface(id)=>demand!(self.surfaces,id),
        }
    }
    fn mark_step(&mut self,root:EntityRef,grant:RetainedCloneGrant)->Result<(Option<bool>,RetainedCloneProgress),(ValueError,RetainedCloneProgress)> {
        macro_rules! mark {($set:expr,$id:expr)=>{if $set.contains(&$id){Ok((Some(false),RetainedCloneProgress{copied_items:1,..Default::default()}))}else if $set.next_insert_capacity_byte_demand(&$id,grant.maximum_copy_bytes).map_err(|error|(error,Default::default()))?!=0{$set.reserve_insert_step(&$id,grant).map(|receipt|(None,receipt))}else{$set.insert_reserved($id,grant).map(|(added,receipt)|(Some(added),receipt)).map_err(|(error,_)|(error,Default::default()))}}}
        match root {
            EntityRef::Vertex(id)=>mark!(self.vertices,id),
            EntityRef::Edge(id)=>mark!(self.edges,id),
            EntityRef::Coedge(id)=>mark!(self.coedges,id),
            EntityRef::Loop(id)=>mark!(self.loops,id),
            EntityRef::Face(id)=>mark!(self.faces,id),
            EntityRef::Shell(id)=>mark!(self.shells,id),
            EntityRef::Solid(id)=>mark!(self.solids,id),
            EntityRef::Curve3(id)=>mark!(self.curves3,id),
            EntityRef::Curve2(id)=>mark!(self.curves2,id),
            EntityRef::Surface(id)=>mark!(self.surfaces,id),
        }
    }
    fn clear_membership(&mut self) {
        self.vertices.clear();
        self.edges.clear();
        self.coedges.clear();
        self.loops.clear();
        self.faces.clear();
        self.shells.clear();
        self.solids.clear();
        self.curves3.clear();
        self.curves2.clear();
        self.surfaces.clear();
    }
}

#[derive(Clone,Copy)]
struct ReachFrame {root:EntityRef,marked:bool,cursor:usize,start:Option<CoedgeId>,next:Option<CoedgeId>,probes:usize}
semio_framework_value::artifact_retire_leaf!(ReachFrame);
enum ReachChild {Done,Skip,Entity(EntityRef)}
impl ReachFrame {
    fn new(root:EntityRef)->Self{Self{root,marked:false,cursor:0,start:None,next:None,probes:0}}
    fn child(&mut self,body:&Body)->ReachChild {
        let cursor=self.cursor;self.cursor+=1;
        let child=match self.root {
            EntityRef::Solid(id)=>body.solids.get(id).and_then(|solid|if cursor==0{Some(EntityRef::Shell(solid.outer))}else{solid.inners.get(cursor-1).copied().map(EntityRef::Shell)}),
            EntityRef::Shell(id)=>body.shells.get(id).and_then(|shell|shell.faces.get(cursor).copied().map(EntityRef::Face)),
            EntityRef::Face(id)=>{let Some(face)=body.faces.get(id)else{return ReachChild::Done};match cursor{0=>Some(EntityRef::Surface(face.surface)),1=>{let Some(outer)=face.outer else{return ReachChild::Skip};Some(EntityRef::Loop(outer))},_=>face.inners.get(cursor-2).copied().map(EntityRef::Loop)}},
            EntityRef::Loop(id)=>{if self.start.is_none(){let Some(loop_)=body.loops.get(id)else{return ReachChild::Done};self.start=Some(loop_.first);self.next=self.start;}if self.probes>=body.coedges.slot_count() || (self.probes!=0 && self.next==self.start){return ReachChild::Done;}let Some(next)=self.next else{return ReachChild::Done};self.next=body.coedges.get(next).map(|coedge|coedge.next);self.probes+=1;Some(EntityRef::Coedge(next))},
            EntityRef::Coedge(id)=>{let Some(coedge)=body.coedges.get(id)else{return ReachChild::Done};match cursor{0=>{let Some(pcurve)=coedge.pcurve else{return ReachChild::Skip};Some(EntityRef::Curve2(pcurve))},1=>Some(EntityRef::Edge(coedge.edge)),_=>None}},
            EntityRef::Edge(id)=>body.edges.get(id).and_then(|edge|match cursor{0=>Some(EntityRef::Curve3(edge.curve)),1=>Some(EntityRef::Vertex(edge.v0)),2=>Some(EntityRef::Vertex(edge.v1)),_=>None}),
            _=>None,
        };
        child.map_or(ReachChild::Done,ReachChild::Entity)
    }
}

/// 🧭️ Borrows one original topology member per turn while retaining the same physical membership arenas.
pub struct ReachabilityJob {keep:ReachSet,frames:[Option<ReachFrame>;7],depth:usize,cancelled:bool}
semio_framework_value::artifact_retire_struct!(ReachabilityJob {keep,frames,depth,cancelled});
/// 🪙️ Logical traversal readiness remains separate from the original membership allocation retirement.
pub enum ReachabilityStep {Working(RetainedCloneProgress),Ready(RetainedCloneProgress),Cancelled(RetainedCloneProgress),Failed {error:ValueError,progress:RetainedCloneProgress}}
impl ReachabilityStep {pub fn progress(&self)->RetainedCloneProgress {match self {Self::Working(progress)|Self::Ready(progress)|Self::Cancelled(progress)|Self::Failed{progress,..}=>*progress}}}
impl Default for ReachabilityJob {fn default()->Self{Self::new()}}
impl ReachabilityJob {
    pub fn new()->Self{Self{keep:ReachSet::default(),frames:[None;7],depth:0,cancelled:false}}
    pub fn begin_root(&mut self,root:EntityRef)->Result<(),EntityRef>{if self.depth!=0 || self.cancelled{return Err(root)}self.frames[0]=Some(ReachFrame::new(root));self.depth=1;Ok(())}
    pub fn keep(&self)->&ReachSet{&self.keep}
    pub fn walk_is_complete(&self)->bool{self.depth==0 && !self.cancelled}
    pub fn cancel(&mut self){self.cancelled=true;}
    pub fn restart(&mut self){self.keep.clear_membership();self.frames=[None;7];self.depth=0;self.cancelled=false;}
    pub fn into_set(self)->ReachSet{assert!(self.walk_is_complete(),"original reach walk requires logical completion");self.keep}
    fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.depth==0 || self.cancelled{return Ok(Default::default())}let frame=self.frames[self.depth-1].as_ref().unwrap();let mut demand=if frame.marked{RetirementDemand{depth:1,..Default::default()}}else{self.keep.mark_demand(frame.root,copy)?};demand.depth=demand.depth.checked_add(self.depth).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original reachability depth overflow"))?;Ok(demand)}
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demand(copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.release_bytes)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
    pub fn step(&mut self,body:&Body,grant:RetainedCloneGrant)->Result<ReachabilityStep,ValueError>{
        let empty=RetainedCloneProgress::default();if self.cancelled{return Ok(ReachabilityStep::Cancelled(empty))}if self.depth==0{return Ok(ReachabilityStep::Ready(empty))}
        let demand=self.demand(grant.maximum_copy_bytes)?;if grant.maximum_items==0 || grant.maximum_copy_bytes<demand.copy_bytes || grant.maximum_capacity_bytes<demand.capacity_bytes || grant.maximum_release_bytes<demand.release_bytes || grant.maximum_depth<demand.depth{return Ok(ReachabilityStep::Working(empty))}
        let slot=self.depth-1;let frame=self.frames[slot].unwrap();
        let progress=if !frame.marked {
            let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-self.depth,..grant};match self.keep.mark_step(frame.root,child){Ok((marked,progress))=>{if let Some(marked)=marked{if marked{self.frames[slot].as_mut().unwrap().marked=true;}else{self.frames[slot]=None;self.depth-=1;}}progress},Err((error,progress))=>return Ok(ReachabilityStep::Failed{error,progress})}
        }else{
            match self.frames[slot].as_mut().unwrap().child(body){ReachChild::Done=>{self.frames[slot]=None;self.depth-=1;},ReachChild::Skip=>{},ReachChild::Entity(root)=>{if self.depth==self.frames.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original topology exceeds its structural reachability depth"))}self.frames[self.depth]=Some(ReachFrame::new(root));self.depth+=1;}}
            RetainedCloneProgress{copied_items:1,..empty}
        };
        Ok(if self.depth==0{ReachabilityStep::Ready(progress)}else{ReachabilityStep::Working(progress)})
    }
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn compact(&mut self, keep: ReachSet) -> Remap {
        let mut job=BodyCompactionJob::new(keep).unwrap_or_else(|(error,_)|panic!("cold original compaction admission: {error}"));
        while !job.terminal_is_empty(){let copy=job.next_copy_byte_demand(self).expect("cold original compact copy");let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_capacity_byte_demand(self,copy).expect("cold original compact capacity"),maximum_release_bytes:job.next_release_byte_demand(self).expect("cold original compact release"),maximum_depth:job.next_depth_demand(self).expect("cold original compact depth")};job.step(self,grant).expect("cold original compact turn");}
        job.freed()
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
        self.merge_selected(other, None)
    }

    /// ♻️ [`Body::merge`] restricted to the entities of `other` inside `keep` (everything when
    /// `None`) — the extraction primitive behind a minimal standalone shape value. Labels shift by
    /// `self`'s high-water mark exactly like a full merge, so a fresh `self` preserves them verbatim.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn merge_selected(&mut self, other: &Body, keep: Option<&ReachSet>) -> MergeMap {
        let offset = self.labels.next();
        self.labels = LabelSource::from_next(offset + other.labels.next());
        self.copy_in(other, keep, |l: PersistentLabel| PersistentLabel(l.0 + offset))
    }

    /// ♻️ The one copy walk [`Body::merge_selected`], [`Body::extract`] and [`Body::absorb`] share:
    /// copies the entities of `other` inside `keep` (all when `None`), each carrying `relabel` of its
    /// label. Never touches `self`'s label counter — the caller owns that bookkeeping.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️reachability/🦀️.rs"]
mod reachability_tests;
// #endregion 🔖️Tests

// #endregion ♻️Reachability
