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

use crate::arena::{ArenaId, CoedgeId, Curve2Id, Curve3Id, EdgeId, FaceId, LoopId, ShellId, SolidId, Store, SurfaceId, VertexId};
use semio_framework_3d_brep_curve::{Curve2, Curve3};
use semio_framework_3d_brep_surface::Surface;
use semio_framework_3d_brep_tolerance::Tol;
use crate::topology::history::{LabelSource, PersistentLabel};
use semio_framework_3d_brep_vector::Pnt3;

#[path = "📒️entity-records/🦀️.rs"]
mod entity_records;
pub use entity_records::{Coedge, Edge, Face, Loop, Shell, Solid, Vertex, coedge_uses_edge, dummy_coedge, make_loop};

#[path = "🗄️wire-and-body-arenas/🦀️.rs"]
mod wire_and_body_arenas;
pub use wire_and_body_arenas::{Body, Wire};

#[path = "🌱️arena-seed/🦀️.rs"]
mod arena_seed;
pub use arena_seed::{BrepArenaSeed, SeedEdge, SeedFace, SeedShell, SeedSolid, SeedVertex};

#[path = "🧭️reachability-walk/🦀️.rs"]
mod reachability_walk;
pub use reachability_walk::{EntityRef, ReachSet, ReachabilityJob, ReachabilityStep};

#[path = "🔗️merge-and-remap/🦀️.rs"]
mod merge_and_remap;
pub use merge_and_remap::{EntityCounts, MergeMap, Remap};

// #region 🔖️History

pub mod history {
    //! 📜️ Operation provenance: a [`PersistentLabel`] assigned once at an entity's birth and never
    //! reused, plus the [`OpDelta`] every mutating operation in [`semio_framework_3d_brep_construction::euler`] returns.
    //! **Host authority:** `LabelSource` lives only inside a `Body` owned by engine compute or cache.

    // #region 🔖️Labels

    /// 📜️ A stable identity for one topological entity, assigned from a per-`Body` monotonically
    /// increasing counter at birth. Unlike an arena [`crate::arena::ArenaId`] (which can be reused
    /// after removal once its generation increments), a label is never reused — it survives arena
    /// compaction and is the identity the document layer's persistent naming keys off of.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
    #[value(transparent)]
    pub struct PersistentLabel(pub u64);

    /// 📜️ Issues fresh, never-repeating labels for one `Body`.
    #[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(crate = "::semio_framework_value")]
    pub struct LabelSource {
        next: u64,
    }

    semio_framework_value::artifact_retire_leaf!(PersistentLabel);
    semio_framework_value::artifact_retire_struct!(LabelSource {next});

    impl LabelSource {
        pub fn new() -> Self {
            LabelSource { next: 0 }
        }
        /// 📜️ Seeds the counter at an explicit high-water mark rather than restarting at 0 — used by
        /// [`crate::topology::Body`]'s `from_seed` so a rebuild from a persisted seed carries
        /// the label numbering forward instead of colliding with the labels it is restoring.
        pub fn from_next(next: u64) -> Self {
            LabelSource { next }
        }
        pub fn next_label(&mut self) -> PersistentLabel {
            let label = PersistentLabel(self.next);
            self.next += 1;
            label
        }
        /// 📜️ The next label this source would mint — the high-water mark a seed must carry forward
        /// (see [`Self::from_next`]) so a rebuilt `Body` never re-mints a label already in use.
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
#[value(crate = "::semio_framework_value")]
    pub struct OpDelta {
        pub generated: Vec<PersistentLabel>,
        pub modified: Vec<PersistentLabel>,
        pub deleted: Vec<PersistentLabel>,
    }

    impl OpDelta {
        pub fn is_empty(&self) -> bool {
            self.generated.is_empty() && self.modified.is_empty() && self.deleted.is_empty()
        }
        pub fn merge(&mut self, other: OpDelta) {
            self.generated.extend(other.generated);
            self.modified.extend(other.modified);
            self.deleted.extend(other.deleted);
        }
    }

    /// 📜️ Accumulates an [`OpDelta`] as a checked editor runs; passed by every [`semio_framework_3d_brep_construction::euler`]
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
        pub fn new() -> Self {
            OpRecorder::default()
        }
        pub fn record_generated(&mut self, label: PersistentLabel) {
            if !self.delta.generated.contains(&label) {
                self.delta.generated.push(label);
            }
        }
        pub fn record_modified(&mut self, label: PersistentLabel) {
            if !self.delta.modified.contains(&label) && !self.delta.generated.contains(&label) {
                self.delta.modified.push(label);
            }
        }
        pub fn record_deleted(&mut self, label: PersistentLabel) {
            self.delta.generated.retain(|l| *l != label);
            self.delta.modified.retain(|l| *l != label);
            if !self.delta.deleted.contains(&label) {
                self.delta.deleted.push(label);
            }
        }
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
        pub fn retirement_bytes(&self)->usize {self.owned.capacity()*size_of::<super::EntityRef>()+self.delta.generated.capacity()*size_of::<PersistentLabel>()}
        /// 🎟️ Borrows the original rollback tail without removing its ownership.
        pub fn last_owned(&self)->Option<super::EntityRef> {self.owned.last().copied()}
        /// 🎟️ Removes the original rollback authority only after its entity transfer is admitted.
        pub fn pop_retired_owned(&mut self,entity:super::EntityRef) {assert_eq!(self.owned.pop(),Some(entity));}

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

#[path="♻️compaction/🦀️.rs"]
mod compaction;
pub use compaction::BodyCompactionJob;

use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

// #region 🔖️Tests
// #endregion 🔖️Tests

// #endregion ♻️Reachability
