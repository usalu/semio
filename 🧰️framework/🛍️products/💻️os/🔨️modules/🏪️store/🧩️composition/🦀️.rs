//! 🧩️ The composition coordinator: multi-member composition sessions above the Store engine, their command transactions
//! and member factories.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
#[cfg(test)]
#[path = "🌱️initial/🪪️identity/🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
#[path = "🌳️closure/🦀️.rs"]
pub mod owned_document_closure;
#[path = "🌳️graph/🔗️links/🦀️.rs"]
pub mod composition_links;
#[path = "🌳️graph/🤝️group/🦀️.rs"]
pub mod composition_group;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_spr::command::ArtifactMessageSource;
use crate::owned_document_closure::{OWNED_DOCUMENT_MAXIMUM_MEMBERS, OwnedDocumentClosureSource};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::SchemaId;
use semio_framework_os_spr::{DiffRegions, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_child_id, mint_invocation_id, operation_list_fingerprint};
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::{Identified, Patchable};
use semio_framework_os_store_envelope::owner::OwnerRef;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_text_io::ArtifactCodecApplyFuture;
use semio_framework_os_pack::pack_rt;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_codec_registry::ComponentDocumentCodecFuture;
use semio_framework_os_store_command::COMMAND_BINARY_FORMAT;
use semio_framework_os_store_command::write_command_transaction;
use semio_framework_os_store_canonical_json::{ArtifactCanonicalJson, ArtifactCanonicalJsonTree};
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_backbone::resolve_backbone;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use semio_framework_os_store_backbone::local_storage_backbone_key;
use semio_framework_os_store_space_member::lifted_store::{ArtifactRefs, MemberFactory, SpaceMember, ReferenceSpaceMember};

// 🧩️ Atomic composite dispatch across a parent `SpaceMember` and N child `SpaceMember`s — the
// mechanism that makes ONE user gesture spanning a parent and several `🔖️Composition` children
// into ONE undo step, by orchestrating multi-store dispatch through the object-safe
// `SpaceMember::preview_wire`/`dispatch_wire`/`tail_group_id`/`merge_policy`/etc. seam (extended
// just above, `🔖️Space`'s `SpaceMember` region) rather than needing to know any member's concrete
// `P`/`Mutation`. Two-phase: `dispatch_group`'s phase 1 dry-runs every op on every member against
// its CURRENT snapshot with zero side effects, unions the resulting `MutationMessage`s (each
// prefixed with its originating member's path — `prefix_message_target`) into one set, and rejects
// the WHOLE group (`reject_if_policy_rejects`, ALL-OR-NOTHING — no member applies anything) if the
// parent-or-initiator's own `merge_policy()` rejects the union's worst `semio_framework_diagnostic::Severity`
// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6); a structural failure
// (unknown ownership, a graph cycle, a missing `ChildStoreFactory`) still rejects immediately,
// exactly as before. Phase 2 applies in the fixed order child geneses → child edits → parent ops,
// stamping every resulting edit's `MutationMeta.group_id` with the same minted `invocation_id` —
// the shared stamp a later `undo_group`/`redo_group` call recognizes — and the SAME unioned
// messages phase 1 computed travel through unchanged as `GroupReceipt.messages`. `CompositionGraph`
// tracks the ownership forest/link DAG this all leans on for cycle/ownership validation.

/// 📮️ One child's share of a composite dispatch: which child, its ops (each individually
/// `crate::os_spr::OpBinary`-encoded — the SAME per-op wire shape `ArtifactCommand::Apply.mutations`
/// bundles, see `SpaceMember::dispatch_wire`'s doc comment), the schema those ops decode against,
/// and human-readable labels (one per op, forward-compat metadata for a future audit/diagnostic
/// surface — `dispatch_group` does not interpret them itself, see this ticket's
/// `📓️wave1-reports/b2-store-composition-report.md` for the scoping note).
#[derive(Clone, Debug, PartialEq)]
pub struct ChildDispatch<'wire> {
    pub child: semio_framework_artifact_reference::ArtifactRef,
    pub ops: &'wire [Vec<u8>],
    pub op_schema: &'wire SchemaId,
    pub labels: &'wire [semio_framework_ui_locale::LocalizedLabel],
}

impl<'wire> ChildDispatch<'wire> {
    /// 🤲️ Borrows the exact retained child sources for one awaited composite publication.
    pub fn borrowed(child:semio_framework_artifact_reference::ArtifactRef,ops:&'wire [Vec<u8>],op_schema:&'wire SchemaId,labels:&'wire [semio_framework_ui_locale::LocalizedLabel])->Self {
        Self{child,ops,op_schema,labels}
    }
}

/// 🌱️ One new child to create in this same composite gesture: which parent-relative `slot`,
/// what dialect it materializes as, and its baked initial pack bytes (fed to the registered
/// `ChildStoreFactory::create`). No `ops` — a freshly-created child has nothing to replay yet, only
/// an initial snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct ChildGenesis {
    pub slot: String,
    pub dialect: semio_framework_artifact_reference::ArtifactDialect,
    pub initial_pack: Vec<u8>,
}

/// 🧾️ Receipt from a successful `CompositionCoordinator::dispatch_group`: the minted shared
/// `MutationMeta.group_id` every touched member's tail edit now carries, one `(member, edit_id)`
/// pair per member that actually got a new edit (parent included when `parent_ops` was non-empty;
/// a child with empty `ops` contributes none), and every `ChildGenesis`-created member's live
/// `Box<dyn SpaceMember>` — genesis has no pre-existing caller-held reference to hand back through
/// (unlike `children`, which the caller already owns), so this is the only way a freshly-created
/// child ever reaches the caller for registration (e.g. into a `SpaceHost`). No `Clone`/`Debug`/
/// `PartialEq`: `Box<dyn SpaceMember>` supports none of them.
///
/// 🎯️ `messages` (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): the
/// SAME union of every dispatched member's `preview_wire` output phase 1 already computed to decide
/// accept/reject — carried through unchanged rather than recomputed, so a successful receipt's
/// messages are byte-identical to what a `preview_wire` dry run of the same ops would have reported
/// (§C2 LAW 3, determinism). Each message's `target` is prefixed with the ORIGINATING member's own
/// `semio_framework_artifact_reference::ArtifactRef::to_uri()` (outermost segment), so a caller with several members in
/// flight can always tell which member a given message came from — the same discipline §C4 mandates
/// for composite step paths. Empty when every op's outcome was silent.
pub struct GroupReceipt<M> {
    pub invocation_id: String,
    pub member_edits: Vec<(semio_framework_artifact_reference::ArtifactRef, String)>,
    pub created_children: Vec<(semio_framework_artifact_reference::ArtifactRef, M)>,
    pub messages: Vec<semio_framework_os_spr::MutationMessage>,
}

/// 🎛️ Cross-member metadata for one `dispatch_group` call. No member edit carries a hand-written description (design
/// §20.6): history labels every row from its leaf. `actor` is operation attribution and preserves every member's opened actor authority.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GroupMeta {
    pub actor: Option<semio_framework_value::SharedUtf8>,
    /// 🪪️ The group identity every member's tail edit is stamped with when the caller already owns one
    /// (an agent transaction's `txn_id`, so the gateway's `TransactionUndo{group_id}` names this group);
    /// `None` mints the content-addressed invocation id from the parent edit and the child op fingerprints.
    pub group_id: Option<String>,
    /// 🧾️ The committed tool transaction the whole group belongs to: rides every member's `Apply` command,
    /// so parent and child operations carry the same [`protocol::TransactionRef`] in their ledgers and on the wire.
    pub transaction: Option<protocol::TransactionRef>,
}

/// 🧾️ Best-effort group undo/redo report: `undone` is every member that WAS rolled
/// back/reapplied (with the edit id touched), `skipped` is every member whose tail didn't belong to
/// the requested group, or whose own `undo()`/`redo()` itself failed — recorded as a diagnostic,
/// never aborting the rest of the group. Generalizes the existing benign `NothingToUndo`/
/// `ForeignEdit` collapse (`🔌️plugin/🦀️.rs`'s `UndoWithPolicy` dispatch) to the
/// multi-member case: one collaborator's foreign/failed child edit must never permanently freeze a
/// parent's undo stack.
#[derive(Debug, PartialEq)]
pub struct GroupUndoReport {
    pub undone: Vec<(semio_framework_artifact_reference::ArtifactRef, String)>,
    pub skipped: Vec<(semio_framework_artifact_reference::ArtifactRef, VcsError)>,
}

/// 🕸️ Ownership forest (`Owns`, each child has AT MOST one owner, no cycles) + reference DAG
/// (`Links`, acyclic) over artifact ids — the structure `CompositionCoordinator::dispatch_group`'s
/// phase 1 consults for ownership/cycle validation, incrementally maintained (`sync_member`) rather
/// than ever rebuilt from scratch on the hot path. A `CompositionCoordinator` owns one; a host that
/// needs the graph without dispatching anything (e.g. a UI computing "would this drag-and-drop
/// create a cycle") can also use a standalone `CompositionGraph` directly — every method here takes
/// `&self`/`&mut self`, none require a live `CompositionCoordinator`.
#[derive(Debug)]
pub struct CompositionGraph {
    pub owns: crate::composition_group::OwnsRoot,
    pub(crate) owns_group: Option<usize>,
    pub(crate) owns_retiring: Option<(String, (String, String))>,
    pub links: crate::composition_links::LinkRoot,
    pub owns_authority: Option<Arc<()>>,
    pub owns_generation: u64,
}

impl Default for CompositionGraph {
    fn default() -> Self { Self { owns: Default::default(), owns_group: None, owns_retiring: None, links: Default::default(), owns_authority: Some(Arc::new(())), owns_generation: 0 } }
}

/// 🎟️ Exact read-only ownership admission consumed by an exclusive graph commit.
pub struct OwnsAdmission {
    pub parent: String,
    pub slot: String,
    pub child: String,
    pub inserts: bool,
    pub authority: Arc<()>,
    pub generation: u64,
}

impl OwnsAdmission {
    pub fn inserts(&self) -> bool {
        self.inserts
    }
}

impl CompositionGraph {
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🌳️ Pre-admits the exact ownership-edge count for a closure already validated by
    /// [`OwnedDocumentClosure`]. No member payload or projection is retained here.
    pub fn try_for_owned_closure(member_count: usize) -> Result<Self, String> {
        if member_count > OWNED_DOCUMENT_MAXIMUM_MEMBERS {
            return Err("owned closure graph exceeds its fixed member limit".into());
        }
        let mut graph = Self::default();
        graph.owns.try_reserve(member_count).map_err(|_| "owned closure graph allocation was not admitted".to_string())?;
        Ok(graph)
    }

    /// 🔗️ Adds one exact ownership row from an already validated immutable closure source.
    pub fn insert_validated_owned(&mut self, parent_id: &str, slot: &str, child_id: &str) -> Result<(), String> {
        if self.owns_group.is_some() { return Err("composition ownership group retains its exclusive root preparation".into()); }
        if self.owns.contains_key(child_id) {
            return Err("validated owned closure attempted a duplicate child edge".into());
        }
        if self.owns.len() >= OWNED_DOCUMENT_MAXIMUM_MEMBERS || self.owns.len() == self.owns.capacity() {
            return Err("validated owned closure exceeded its pre-admitted graph capacity".into());
        }
        let generation = self.owns_generation.checked_add(1).ok_or_else(|| "composition ownership generation exhausted".to_string())?;
        self.owns.insert(child_id.to_string(), (parent_id.to_string(), slot.to_string()));
        self.owns_generation = generation;
        Ok(())
    }

    /// 🔎️ The owning parent's artifact id, if `child_id` is currently tracked as owned.
    pub async fn owner_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(parent_id, _slot)| parent_id.as_str())
    }

    /// 🔎️ The slot `child_id` currently occupies under its owner, if tracked.
    pub async fn slot_of(&self, child_id: &str) -> Option<&str> {
        self.owns.get(child_id).map(|(_parent_id, slot)| slot.as_str())
    }

    /// ✅️ Whether owning `child_id` under `parent_id` would create a cycle — walks `parent_id`'s
    /// OWN ancestor chain looking for `child_id`; finding it means `child_id` would become both an
    /// ancestor and a (prospective) descendant of `parent_id`. Also true when `parent_id ==
    /// child_id` (an artifact cannot own itself).
    pub async fn would_cycle_owns(&self, parent_id: &str, child_id: &str) -> bool {
        if parent_id == child_id {
            return true;
        }
        let mut current = parent_id.to_string();
        let mut guard = 0usize;
        while let Some((owner, _slot)) = self.owns.get(&current) {
            if owner == child_id {
                return true;
            }
            current = owner.clone();
            guard += 1;
            if guard > self.owns.len() + 1 {
                break;
            }
        }
        false
    }

    /// 🌱️ Records `child_id` as owned by `parent_id` in `slot`. Rejects (a) adopting a child
    /// already owned by a DIFFERENT parent (single-ownership invariant) and (b) any edge that would
    /// cycle (see `would_cycle_owns`) — re-owning by the SAME parent/slot is idempotent, not an error.
    pub async fn insert_owns(&mut self, parent_id: &str, slot: &str, child_id: &str) -> Result<(), String> {
        let admission = self.admit_owns(parent_id, slot, child_id).await?;
        self.commit_owns_admitted(admission);
        Ok(())
    }

    pub async fn admit_owns(&self, parent_id: &str, slot: &str, child_id: &str) -> Result<OwnsAdmission, String> {
        if self.owns_group.is_some() { return Err("composition ownership group retains its exclusive root preparation".into()); }
        if let Some((existing_owner, existing_slot)) = self.owns.get(child_id) {
            if existing_owner != parent_id || existing_slot != slot {
                return Err(format!("{child_id} is already owned by {existing_owner} in {existing_slot}, cannot also be owned by {parent_id} in {slot}"));
            }
            return Ok(OwnsAdmission { parent: parent_id.into(), slot: slot.into(), child: child_id.into(), inserts: false, authority: Arc::clone(self.owns_authority.as_ref().ok_or_else(|| "composition graph is closed".to_owned())?), generation: self.owns_generation });
        }
        if self.would_cycle_owns(parent_id, child_id).await {
            return Err(format!("owning {child_id} under {parent_id} would create a composition cycle"));
        }
        self.owns_generation.checked_add(1).ok_or_else(|| "composition ownership generation exhausted".to_string())?;
        Ok(OwnsAdmission { parent: parent_id.into(), slot: slot.into(), child: child_id.into(), inserts: true, authority: Arc::clone(self.owns_authority.as_ref().ok_or_else(|| "composition graph is closed".to_owned())?), generation: self.owns_generation })
    }

    pub fn commit_owns_admitted(&mut self, admission: OwnsAdmission) {
        assert!(self.owns_group.is_none(), "ownership mutation waits for the retained group root");
        let OwnsAdmission { parent, slot, child, inserts, authority, generation } = admission;
        assert!(self.owns_authority.as_ref().is_some_and(|own| Arc::ptr_eq(&authority, own)) && generation == self.owns_generation, "ownership commit retains the exact graph and admitted generation");
        if inserts {
            assert!(!self.owns.contains_key(&child), "ownership commit retains its exclusive admitted child");
            self.owns.insert(child, (parent, slot));
            self.owns_generation = self.owns_generation.checked_add(1).expect("ownership generation preflighted before insertion");
        } else {
            assert_eq!(self.owns.get(&child), Some(&(parent, slot)), "idempotent ownership commit retains the exact prior edge");
        }
    }

    /// ✂️ Removes `child_id`'s ownership edge (e.g. on `extract`/`delete`), returning
    /// `(parent_id, slot)` if it was tracked.
    pub async fn remove_owns(&mut self, child_id: &str) -> Option<(String, String)> {
        assert!(self.owns_group.is_none(), "ownership mutation waits for the retained group root");
        let removed = self.owns.remove(child_id);
        if removed.is_some() {
            self.owns_generation = self.owns_generation.checked_add(1).expect("ownership generation exhausted");
        }
        removed
    }

    /// ✅️ Whether linking `from -> to` would create a cycle — true when `to` can already reach
    /// `from` (including `from == to`).
    pub async fn would_cycle_links(&self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        let mut stack = vec![to.to_string()];
        let mut seen: HashSet<String> = HashSet::new();
        while let Some(node) = stack.pop() {
            if node == from {
                return true;
            }
            if !seen.insert(node.clone()) {
                continue;
            }
            if let Some(targets) = self.links.get(&node) {
                stack.extend(targets.iter().cloned());
            }
        }
        false
    }

    /// 🔗️ Records a `from -> to` link edge. Rejects a cycle (see `would_cycle_links`); adding the
    /// same edge twice is idempotent.
    pub async fn insert_link(&mut self, from: &str, to: &str) -> Result<(), String> {
        if self.would_cycle_links(from, to).await {
            return Err(format!("linking {from} -> {to} would create a cycle"));
        }
        self.links.add_target(from, to);
        Ok(())
    }

    /// ✂️ Removes one `from -> to` link edge, if present.
    pub async fn remove_link(&mut self, from: &str, to: &str) {
        if let Some(targets) = self.links.get_mut(from) {
            if let Ok(index) = targets.binary_search_by(|target| target.as_str().cmp(to)) { targets.remove(index); }
        }
    }

    /// 🔎️ Every link target currently recorded FROM `from`.
    pub async fn links_from(&self, from: &str) -> Vec<String> {
        self.links.get(from).map(|targets| targets.iter().cloned().collect()).unwrap_or_default()
    }

    /// 🔄️ Rebuilds `artifact_id`'s OWN outgoing edges (both `Owns`-as-parent and `Links`-as-source)
    /// from its live `ArtifactRefs` projection — the incremental-maintenance seam a host (e.g.
    /// `SpaceHost`) calls after every dispatch that might have changed `artifact_id`'s
    /// children/links, instead of ever recomputing the whole graph from scratch. Never touches
    /// edges where `artifact_id` is the TARGET (another artifact's own `sync_member` call owns those).
    pub async fn sync_member<P: ArtifactRefs>(&mut self, artifact_id: &str, snapshot: &P) -> Result<(), String> {
        if self.owns_group.is_some() { return Err("composition ownership group retains its exclusive root preparation".into()); }
        let mut next_owns = HashMap::new();
        let mut member_ids = HashSet::new();
        for child in snapshot.child_refs().await {
            if !member_ids.insert(child.child_id) || next_owns.contains_key(&child.target.artifact_id) {
                return Err("composition parent declares a duplicate member identity or owned target".into());
            }
            if self.owns.get(&child.target.artifact_id).is_some_and(|(parent, _)| parent != artifact_id) {
                return Err(format!("{} is already owned by another parent", child.target.artifact_id));
            }
            if next_owns.get(&child.target.artifact_id).is_some_and(|(_, slot)| slot != &child.slot) {
                return Err(format!("{} appears in multiple owner slots", child.target.artifact_id));
            }
            if self.would_cycle_owns(artifact_id, &child.target.artifact_id).await {
                return Err(format!("owning {} under {artifact_id} would create a composition cycle", child.target.artifact_id));
            }
            next_owns.insert(child.target.artifact_id, (artifact_id.to_string(), child.slot));
            semio_framework_async::yield_once().await;
        }
        let mut next_links = HashSet::new();
        for link in snapshot.links().await {
            if self.would_cycle_links(artifact_id, &link.target.artifact_id).await {
                return Err(format!("linking {artifact_id} to {} would create a composition cycle", link.target.artifact_id));
            }
            next_links.insert(link.target.artifact_id);
            semio_framework_async::yield_once().await;
        }
        let generation = self.owns_generation.checked_add(1).ok_or_else(|| "composition ownership generation exhausted".to_string())?;
        self.owns.try_reserve(next_owns.len()).map_err(|_| "composition ownership allocation rejected".to_string())?;
        self.links.try_reserve(usize::from(!next_links.is_empty())).map_err(|_| "composition link allocation rejected".to_string())?;
        self.owns.retain(|_, (parent, _)| parent != artifact_id);
        self.owns.extend(next_owns);
        if next_links.is_empty() {
            self.links.remove(artifact_id);
        } else {
            self.links.insert(artifact_id.to_string(), next_links.into_iter().collect());
        }
        self.owns_generation = generation;
        Ok(())
    }

    /// 🧹️ Retires one owned forest row or its exact physical backing, retaining underfunded owners.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_depth < self.next_close_depth_demand()? || self.owns_group.is_some() { return Ok(RetainedCloneStep::Progress(empty)); }
        if let Some(step) = self.close_owned_root(grant)? { return Ok(step); }
        Ok(self.links.close_step(grant))
    }

    /// 🧺 Proves that no graph edge or adjacency owner remains.
    pub fn terminal_is_empty(&self) -> bool {
        self.owns_group.is_none() && self.owns.is_empty() && self.owns.capacity() == 0 && self.owns_retiring.is_none() && self.owns_authority.is_none() && self.links.capacity() == 0
    }
}

/// 🧵️ Builds the exact `ArtifactCommand::<Mutation>::Apply` binary layout (see that impl's
/// `OpBinary::encode_op` in `🔖️CommandFormat` above, ordinal-0 arm) directly from already-
/// `OpBinary`-encoded op bytes, WITHOUT ever decoding them — this is how `CompositionCoordinator`
/// stays fully generic over every technology's concrete `Mutation` type while still producing bytes
/// `SpaceMember::dispatch_wire`'s blanket impl (which DOES know the concrete type, on the receiving
/// member) can decode correctly via the ordinary `dispatch_binary` path. Must be kept byte-for-byte
/// in sync with `write_command_ops`/`ArtifactCommand::encode_op`'s ordinal-0 (`Apply`) arm.
pub async fn build_apply_command_bytes(ops: &[Vec<u8>], transaction: Option<&protocol::TransactionRef>) -> Vec<u8> {
    let mut out = vec![COMMAND_BINARY_FORMAT];
    semio_framework_os_pack::write_varint_u64(&mut out, 0);
    out.push(u8::from(transaction.is_some()) << 1);
    write_command_transaction(&mut out, transaction);
    semio_framework_os_pack::write_varint_u64(&mut out, ops.len() as u64);
    for op in ops {
        semio_framework_os_pack::write_varint_u64(&mut out, op.len() as u64);
        out.extend_from_slice(op);
    }
    out
}

/// 🏷️ Prepends `member_path` (that member's own `semio_framework_artifact_reference::ArtifactRef::to_uri()`) as the
/// OUTERMOST segment of every message's `target` — the discipline §C4 mandates for composite step
/// paths, generalized to a composition group: a caller unioning messages from several members must
/// still be able to tell which member produced which message.
pub async fn prefix_message_target(messages: Vec<semio_framework_os_spr::MutationMessage>, member_path: &str) -> Vec<semio_framework_os_spr::MutationMessage> {
    messages
        .into_iter()
        .map(|mut message| {
            message.target.insert(0, member_path.to_string());
            message
        })
        .collect()
}

/// ⚖️ Phase 1's policy gate (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
/// CONFLICTS` §C6): folds every already-prefixed message `dispatch_relation_group` collected via
/// `SpaceMember::preview_wire` into ONE worst `semio_framework_diagnostic::Severity` and consults `policy` — the
/// parent-or-initiator's own `SpaceMember::merge_policy()`, since it is the member driving/owning
/// this atomic gesture (a policy is authority-local state per §C3, so SOME single member's policy
/// must govern one shared accept/reject decision; the entry point is the natural, and only
/// unambiguous, choice among a group that may otherwise mix policies). `Ok(())` when `messages` is
/// empty or `policy` accepts the worst level; `Err(VcsError::Rejected { policy, messages })`
/// otherwise — phase 2 never runs, so nothing has been dispatched anywhere yet (the same "zero side
/// effects" law every other phase-1 failure upholds).
///
/// 🎯️ This is the GROUP-level gate only. Each member's own real `dispatch_wire` (Phase 2) ALSO
/// independently enforces THAT member's own `merge_policy()` inside `ArtifactStore::dispatch`
/// (lane 1-A's C6 `replay_mutations`) — a member with a stricter policy than the parent's can still
/// reject its own op during Phase 2 even after this gate accepted the union under the parent's more
/// lenient policy. Both checks return the SAME `VcsError::Rejected` shape; a caller relying on
/// "accepted by the parent's policy" alone must also configure every other participating member's
/// own policy consistently if it wants Phase 2 to actually complete (see this file's own
/// `dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire` test, which sets
/// `LaissezFaire` on BOTH members for exactly this reason).
pub async fn reject_if_policy_rejects(policy: semio_framework_os_spr::MergePolicy, messages: &[semio_framework_os_spr::MutationMessage]) -> Result<(), VcsError> {
    let Some(worst) = semio_framework_os_spr::worst_level(messages) else {
        return Ok(());
    };
    if !policy.rejects(worst) {
        return Ok(());
    }
    Err(VcsError::Rejected { policy, messages: messages.to_vec() })
}

/// 🧯️ Folds a post-validation dispatch failure with its compensation report: if every
/// already-applied member rolled back cleanly (`report.skipped` empty), the ORIGINAL error is
/// returned unchanged (compensation is a transparent implementation detail on the success path);
/// otherwise wraps both into `VcsError::CompensationFailed` so the caller sees the full picture —
/// what failed AND what could not be rolled back — rather than either fact silently.
///
/// 🌀️ `ArtifactRef::to_uri` is async (🚪️io, out of scope) — hoisted out of `Iterator::map`'s
/// sync closure into an explicit loop (R10 shape 1).
pub async fn fold_compensation_error(original: VcsError, report: GroupUndoReport) -> VcsError {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    if report.skipped.is_empty() {
        original
    } else {
        let mut skipped_desc: Vec<String> = Vec::with_capacity(report.skipped.len());
        for (reference, error) in &report.skipped {
            skipped_desc.push(format!("{}: {error}", reference.to_uri()));
        }
        VcsError::CompensationFailed(format!("original error: {original}; members that failed to roll back: [{}]", skipped_desc.join(", ")))
    }
}

/// 🔀️ Which structural relationship holds between a `TransactionCoordinator::dispatch_group`
/// (or `dispatch_peer_group`) call's `parent`/initiator and its `children`/peers —
/// `PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` W1-C's generalization of
/// the pre-existing composition machinery (contract-freeze §5, `📓️scout-2-group-undo-and-hosts.md`
/// §2). `Owned` is the pre-existing, byte-identical-unchanged behaviour: `CompositionGraph::
/// owner_of` is checked and `ChildGenesis` is allowed. `Peer` is new: every member is an equal
/// party to a cross-artifact transaction with NO ownership relation — no `owner_of` check, no
/// genesis — but the SAME cycle guard (`CompositionGraph::would_cycle_links` in place of
/// `would_cycle_owns`, persisting a `Links` edge per successfully-dispatched peer so a LATER
/// transaction that would close a cycle across separate calls is also caught), the same shared
/// `invocation_id`, the same one-edit-per-member rule, and the same reverse-order compensation.
/// Every peer's tail edit additionally gets `MutationMeta.origin` stamped
/// `crate::os_spr::MutationOrigin::Transaction { initiator }` (`SpaceMember::stamp_tail_origin`) —
/// the initiator's own edit is left at its ordinary `Owner` default, since it is not foreign to
/// itself. `Owned` never stamps `origin` at all, which is exactly what keeps it byte-identical.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemberRelation {
    Owned,
    Peer,
}

/// 🧩️ Atomic composite/transactional dispatch across a parent-or-initiator + N children-
/// or-peers — see this region's doc comment for the two-phase protocol and `MemberRelation` for
/// what differs between `Owned` (`dispatch_group`) and `Peer` (`dispatch_peer_group`). Holds a
/// `CompositionGraph` incrementally maintained across calls (`graph`/`graph_mut` for a host to
/// `sync_member` into, or to consult directly for UI-level "would this cycle" checks without
/// dispatching anything).
///
/// Named `TransactionCoordinator` as of W1-C — `CompositionCoordinator` just below is kept as an
/// exact type alias (same type, not a wrapper) so every existing call site across the tree,
/// including out-of-lease files under concurrent edit by sibling W1 lanes, keeps compiling and
/// behaving identically unchanged. Prefer the new name in code this ticket authors.
#[derive(Debug, Default)]
pub struct TransactionCoordinator {
    pub graph: CompositionGraph,
}

/// 🪪️ Source-compatible alias — see `TransactionCoordinator`'s own doc comment for why this
/// exists rather than a rename-in-place.
pub type CompositionCoordinator = TransactionCoordinator;

impl TransactionCoordinator {
    pub async fn new() -> Self {
        Self::default()
    }

    /// 🌳️ Creates an empty coordinator whose ownership map can receive the validated
    /// closure without allocating during retained preparation.
    pub fn try_for_owned_closure(member_count: usize) -> Result<Self, String> {
        Ok(Self { graph: CompositionGraph::try_for_owned_closure(member_count)? })
    }

    /// 🔗️ Adds one exact ownership row after the shared closure cursor accepted it.
    pub fn insert_validated_owned(&mut self, parent_id: &str, slot: &str, child_id: &str) -> Result<(), String> {
        self.graph.insert_validated_owned(parent_id, slot, child_id)
    }

    pub async fn graph(&self) -> &CompositionGraph {
        &self.graph
    }

    pub async fn graph_mut(&mut self) -> &mut CompositionGraph {
        &mut self.graph
    }

    /// 📐️ Borrows the graph's current physical ownership-root retirement demand.
    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.graph.next_close_copy_byte_demand() }
    pub fn next_close_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.graph.next_close_capacity_byte_demand(copy) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { self.graph.next_close_release_byte_demand() }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { self.graph.next_close_depth_demand() }

    /// 🧭️ Borrows the retained ownership graph for synchronous publication phase queries.
    pub fn ownership_graph(&self) -> &CompositionGraph { &self.graph }

    /// 🛤️ Borrows the retained ownership graph for synchronous publication work and closure.
    pub fn ownership_graph_mut(&mut self) -> &mut CompositionGraph { &mut self.graph }

    /// 🧹 Releases one retained composition-graph owner within the caller's grant.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.graph.close_step(grant)
    }

    /// 🧺 Proves that the coordinator retains no composition owner.
    pub fn terminal_is_empty(&self) -> bool {
        self.graph.terminal_is_empty()
    }

    /// ✂️ Undoes `undo`/re-dispatches `undo` on the applied-so-far members in REVERSE application
    /// order (parent — if it was itself applied — first, then children in reverse dispatch order),
    /// collecting a `GroupUndoReport` rather than propagating the first failure — see
    /// `dispatch_group`'s doc comment for why every already-applied member must still get a
    /// best-effort rollback attempt even if an earlier one in this same pass failed. Relation-
    /// agnostic: undoing a member does not need to know WHY it was dispatched.
    pub async fn compensate<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember>(parent_ref: &semio_framework_artifact_reference::ArtifactRef, parent: &mut Mp, children: &mut [(&mut Mc, ChildDispatch<'_>)], applied_children: &[(usize, String)], parent_applied: Option<&str>, identity: &mut EntityIdentityAuthority<'_>) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        if let Some(edit_id) = parent_applied {
            match parent.undo(identity).await {
                Ok(()) => undone.push((parent_ref.clone(), edit_id.to_string())),
                Err(error) => skipped.push((parent_ref.clone(), error)),
            }
        }
        for (index, edit_id) in applied_children.iter().rev() {
            let (member, dispatch) = &mut children[*index];
            match member.undo(identity).await {
                Ok(()) => undone.push((dispatch.child.clone(), edit_id.clone())),
                Err(error) => skipped.push((dispatch.child.clone(), error)),
            }
        }
        GroupUndoReport { undone, skipped }
    }

    /// 🧩️ `Owned`-relation dispatch: `parent` truly owns every `children` entry (checked against
    /// `self.graph`) and may create brand-new `genesis` children. See `dispatch_relation_group` for
    /// the shared two-phase/compensation engine this delegates to unchanged — this method's own
    /// behaviour is byte-identical to before `MemberRelation` existed.
    pub async fn dispatch_group<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember + MemberFactory>(
        &mut self,
        parent_ref: &semio_framework_artifact_reference::ArtifactRef,
        parent: &mut Mp,
        children: &mut [(&mut Mc, ChildDispatch<'_>)],
        parent_ops: Vec<Vec<u8>>,
        genesis: Vec<ChildGenesis>,
        meta: GroupMeta,
     identity: &mut EntityIdentityAuthority<'_>) -> Result<GroupReceipt<Mc>, VcsError> {
        self.dispatch_relation_group(MemberRelation::Owned, parent_ref, parent, children, parent_ops, genesis, meta, identity).await
    }

    /// 🤝️ `Peer`-relation dispatch (contract-freeze §5): `initiator_ref`/`initiator` is the
    /// transaction's member #0, `peers` are every OTHER member with NO ownership relation to it.
    /// No `genesis` parameter — a peer transaction never creates a child, only touches existing
    /// artifacts. See `MemberRelation::Peer`'s doc comment for exactly what differs from
    /// `dispatch_group`, and `dispatch_relation_group` for the shared engine.
    pub async fn dispatch_peer_group<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember + MemberFactory>(
        &mut self,
        initiator_ref: &semio_framework_artifact_reference::ArtifactRef,
        initiator: &mut Mp,
        peers: &mut [(&mut Mc, ChildDispatch<'_>)],
        initiator_ops: Vec<Vec<u8>>,
        meta: GroupMeta,
     identity: &mut EntityIdentityAuthority<'_>) -> Result<GroupReceipt<Mc>, VcsError> {
        self.dispatch_relation_group(MemberRelation::Peer, initiator_ref, initiator, peers, initiator_ops, Vec::new(), meta, identity).await
    }

    /// 🧩️ Dispatches one composite/transactional gesture spanning `parent` + `children` (+ any
    /// brand-new `genesis` children, `Owned` only) as a single atomic unit. Shared by
    /// `dispatch_group`/`dispatch_peer_group` — `relation` is the ONLY thing that changes phase-1's
    /// per-child check and whether phase-2 stamps `MutationOrigin::Transaction`; everything else
    /// (order, `invocation_id` minting, `group_id` stamping, compensation) is identical, which is
    /// what "generalize, don't replace" means concretely in this method.
    ///
    /// **Phase 1 — preview-all, zero side effects.** Every non-empty op slice (`parent_ops`, each
    /// `ChildDispatch.ops`) is dry-run via `SpaceMember::preview_wire` against that member's CURRENT
    /// snapshot and its messages unioned into one `all_messages` list, each stamped with its
    /// originating member's own `semio_framework_artifact_reference::ArtifactRef::to_uri()` as the outermost `target`
    /// segment (`prefix_message_target`). Under `Owned`, every `children` entry's claimed ownership
    /// is ALSO checked against `self.graph` (`VcsError::OwnershipViolation` if the graph does not
    /// currently track `parent_ref` as that child's owner) — a STRUCTURAL failure, returned
    /// immediately, never folded into `all_messages`. Under `Peer`, no ownership is checked; instead
    /// `self.graph.would_cycle_links(parent_ref, child)` guards against a peer transaction that
    /// would close a link cycle (`VcsError::CompositionCycle`), including the trivial self-cycle of
    /// an artifact transacting with itself. Every `genesis` slot's (Owned-only) deterministic id is
    /// minted and checked for a cycle (`VcsError::CompositionCycle`) and a registered
    /// `ChildStoreFactory` (`VcsError::ValidationFailed` if none) — also structural, also immediate.
    /// Once every member's ops have been previewed and every structural check has passed,
    /// `reject_if_policy_rejects(parent.merge_policy(), &all_messages)` computes ONE worst
    /// `semio_framework_diagnostic::Severity` across the whole union and consults the parent-or-initiator's own
    /// `merge_policy()` (§C6: a `MergePolicy` is authority-local state, so ONE member's policy must
    /// govern the shared group decision — the entry point is the natural choice), returning
    /// `VcsError::Rejected { policy, messages }` on rejection. A reject here is ALL-OR-NOTHING
    /// exactly like every other phase-1 failure: NOTHING has been dispatched anywhere yet, for ANY
    /// member, no matter which member's ops raised the worst message. This is a GROUP-level gate
    /// only — Phase 2 below still runs each member's real `dispatch_wire`, which independently
    /// enforces THAT member's own `merge_policy()` too (lane 1-A's C6 `ArtifactStore::dispatch`), so
    /// a caller must configure every participating member's policy consistently to guarantee Phase 2
    /// completes even after this gate accepts.
    ///
    /// **Phase 2 — apply in fixed order: child geneses → child edits → parent ops.** This order
    /// guarantees a parent's own `Apply` (which typically ADDS the `ArtifactChild` handle pointing
    /// at a just-created genesis child) never references a child that does not exist locally yet.
    /// Every member that receives an `Apply` gets its tail edit's `MutationMeta.group_id` stamped
    /// with the same minted `invocation_id` right after dispatching it. Under `Peer`, each
    /// dispatched child ALSO gets `MutationMeta.origin` stamped `MutationOrigin::Transaction {
    /// initiator: parent_ref }` and a `self.graph.insert_link(parent_ref, child)` edge recorded (so
    /// a later, separate transaction's cycle guard sees it too).
    ///
    /// **Compensation.** A failure during phase 2 (a `dispatch_wire`/`stamp_tail_group_id`/
    /// `stamp_tail_origin` call that phase 1's validation did not catch — e.g. a `Mutation::
    /// validate` that is not fully exhaustive, or a genuinely unexpected `VcsError`) triggers
    /// `compensate`: `Undo` on every already-applied member in reverse order. This is sound under
    /// `&mut` on every member for the whole call (single-threaded per-app actor discipline) because
    /// each such member's group edit IS its tail — exact-base undo is mechanical, never a
    /// mid-history removal. If compensation itself fails to fully roll back, the returned error is
    /// `VcsError::CompensationFailed` (`fold_compensation_error`) carrying both the original
    /// failure and which members could not be rolled back, rather than silently leaving partial
    /// state unreported.
    ///
    /// 🌀️ `concat_ops_fingerprint` is async; `Iterator::map`'s closure is sync (R10 shape 1),
    /// so it's hoisted into an explicit loop.
    ///
    /// 🎯️ O1: no more `ChildStoreFactory` global registry lookup — `M::create` (the
    /// `space_members!`-generated `MemberFactory` impl) matches `kind` against M's OWN closed
    /// variant set directly. Nothing to compensate on a genesis failure: no `dispatch_wire` has
    /// run yet in this call, and any earlier-succeeding genesis member in this same loop was
    /// never registered/dispatched to anywhere — it simply gets dropped along with this `Err`.
    ///
    /// 🔗️ Best-effort: `would_cycle_links` already cleared this exact edge in phase 1, so
    /// this can only fail if a CONCURRENT graph mutation raced us between the two — not
    /// worth failing an already-applied transaction over; the cycle guard still holds for
    /// every edge that DID get recorded.
    async fn dispatch_relation_group<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember + MemberFactory>(
        &mut self,
        relation: MemberRelation,
        parent_ref: &semio_framework_artifact_reference::ArtifactRef,
        parent: &mut Mp,
        children: &mut [(&mut Mc, ChildDispatch<'_>)],
        parent_ops: Vec<Vec<u8>>,
        genesis: Vec<ChildGenesis>,
        meta: GroupMeta,
     identity: &mut EntityIdentityAuthority<'_>) -> Result<GroupReceipt<Mc>, VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _,DialectCoordinateText as _};

        let mut all_messages: Vec<semio_framework_os_spr::MutationMessage> = Vec::new();
        if !parent_ops.is_empty() {
            all_messages.extend(prefix_message_target(parent.preview_wire(&parent_ops).await, &parent_ref.to_uri()).await);
        }
        for (member, dispatch) in children.iter() {
            match relation {
                MemberRelation::Owned => match self.graph.owner_of(&dispatch.child.artifact_id).await {
                    Some(owner_id) if owner_id == parent_ref.artifact_id => {}
                    _ => return Err(VcsError::OwnershipViolation(format!("{} is not a currently-tracked owned child of {}", dispatch.child.artifact_id, parent_ref.artifact_id))),
                },
                MemberRelation::Peer => {
                    if self.graph.would_cycle_links(&parent_ref.artifact_id, &dispatch.child.artifact_id).await {
                        return Err(VcsError::CompositionCycle(format!("transacting {} with {} would create a peer cycle", parent_ref.artifact_id, dispatch.child.artifact_id)));
                    }
                }
            }
            if !dispatch.ops.is_empty() {
                all_messages.extend(prefix_message_target(member.preview_wire(dispatch.ops).await, &dispatch.child.to_uri()).await);
            }
        }
        let parent_edit_fingerprint = identity.encode(|control|operation_list_fingerprint(&parent_ops,control))?;
        let mut minted_child_ids: Vec<String> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.iter().enumerate() {
            let child_id = identity.encode(|control|mint_child_id(&parent_ref.artifact_id,&spec.slot,&parent_edit_fingerprint,ordinal as u32,control))?;
            if self.graph.would_cycle_owns(&parent_ref.artifact_id, &child_id).await {
                return Err(VcsError::CompositionCycle(format!("creating child {child_id} in slot {} under {} would cycle", spec.slot, parent_ref.artifact_id)));
            }
            minted_child_ids.push(child_id);
        }
        let group_policy = parent.merge_policy().await;
        reject_if_policy_rejects(group_policy, &all_messages).await?;

        let mut child_fingerprints = identity.encode(|control|control.allocate_vec::<(String,Vec<u8>)>(children.len()))?;
        for (_, dispatch) in children.iter() {
            child_fingerprints.push((identity.encode(|control|control.copy_text(&dispatch.child.artifact_id))?, identity.encode(|control|operation_list_fingerprint(dispatch.ops,control))?));
        }
        let invocation_id = match meta.group_id.clone() {
            Some(group_id) => group_id,
            None => identity.encode(|control|mint_invocation_id(&parent_ref.artifact_id,&parent_edit_fingerprint,&child_fingerprints,control))?,
        };

        let mut created_children: Vec<(semio_framework_artifact_reference::ArtifactRef, Mc)> = Vec::with_capacity(genesis.len());
        for (ordinal, spec) in genesis.into_iter().enumerate() {
            let child_id = minted_child_ids[ordinal].clone();
            let mut member = Mc::create(&child_id, &spec.dialect, &spec.initial_pack, parent.local_actor_id().clone()).await?;
            let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect: spec.dialect.clone() };
            member.set_owner(Some(OwnerRef { parent: parent_ref.clone(), slot: spec.slot.clone(), child_id: child_id.clone() })).await;
            self.graph.insert_owns(&parent_ref.artifact_id, &spec.slot, &child_id).await.map_err(VcsError::OwnershipViolation)?;
            created_children.push((target, member));
        }

        let mut applied_children: Vec<(usize, String)> = Vec::new();
        for index in 0..children.len() {
            if children[index].1.ops.is_empty() {
                continue;
            }
            let command_bytes = build_apply_command_bytes(children[index].1.ops, meta.transaction.as_ref()).await;
            let receipt = match children[index].0.dispatch_wire_with_policy(&command_bytes, group_policy, identity).await {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None, identity).await;
                    return Err(fold_compensation_error(error, report).await);
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            applied_children.push((index, edit_id));
            if let Err(error) = children[index].0.stamp_tail_group_id(&invocation_id).await {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, None, identity).await;
                return Err(fold_compensation_error(error, report).await);
            }
            if relation == MemberRelation::Peer {
                let initiator = semio_framework_os_spr::ForeignTarget { artifact_id: parent_ref.artifact_id.clone(), artifact_kind: parent_ref.dialect.artifact_kind.clone(), dialect: Some(parent_ref.dialect.to_coordinate()) };
                if let Err(error) = children[index].0.stamp_tail_origin(semio_framework_os_spr::MutationOrigin::Transaction { initiator }).await {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None, identity).await;
                    return Err(fold_compensation_error(error, report).await);
                }
                let _ = self.graph.insert_link(&parent_ref.artifact_id, &children[index].1.child.artifact_id).await;
            }
        }

        let mut parent_edit_id: Option<String> = None;
        if !parent_ops.is_empty() {
            let command_bytes = build_apply_command_bytes(&parent_ops, meta.transaction.as_ref()).await;
            let receipt = match parent.dispatch_wire_with_policy(&command_bytes, group_policy, identity).await {
                Ok(receipt) => receipt,
                Err(error) => {
                    let report = Self::compensate(parent_ref, parent, children, &applied_children, None, identity).await;
                    return Err(fold_compensation_error(error, report).await);
                }
            };
            let edit_id = receipt.edit_ids.last().cloned().unwrap_or_default();
            if let Err(error) = parent.stamp_tail_group_id(&invocation_id).await {
                let report = Self::compensate(parent_ref, parent, children, &applied_children, Some(&edit_id), identity).await;
                return Err(fold_compensation_error(error, report).await);
            }
            parent_edit_id = Some(edit_id);
        }

        let mut member_edits: Vec<(semio_framework_artifact_reference::ArtifactRef, String)> = applied_children.iter().map(|(index, edit_id)| (children[*index].1.child.clone(), edit_id.clone())).collect();
        if let Some(edit_id) = parent_edit_id {
            member_edits.push((parent_ref.clone(), edit_id));
        }
        Ok(GroupReceipt { invocation_id, member_edits, created_children, messages: all_messages })
    }

    /// 🔁️ Shared per-member undo attempt factored out of `undo_group` so its parent pass and its
    /// children pass (genuinely different types, `Mp` vs `Mc`, since `dispatch_group`'s split — see
    /// that fn's own doc comment) share one tail-group-check/undo/skip code path instead of two
    /// copies of it.
    async fn undo_one<M: ReferenceSpaceMember>(reference: &semio_framework_artifact_reference::ArtifactRef, member: &mut M, group_id: &str, undone: &mut Vec<(semio_framework_artifact_reference::ArtifactRef, String)>, skipped: &mut Vec<(semio_framework_artifact_reference::ArtifactRef, VcsError)>, identity: &mut EntityIdentityAuthority<'_>) {
        let tail_group = member.tail_group_id().await;
        if tail_group.as_deref() == Some(group_id) {
            let edit_id = member.tail_edit_id().await.unwrap_or_default();
            match member.undo(identity).await {
                Ok(()) => undone.push((reference.clone(), edit_id)),
                Err(error) => skipped.push((reference.clone(), error)),
            }
        } else {
            skipped.push((reference.clone(), VcsError::ForeignEdit(member.document_id().await.to_string())));
        }
    }

    /// ↩️ Best-effort group undo: `parent`/`children` are passed SEPARATELY (not one homogeneous
    /// list) because they are genuinely different types in the general case — the same `Mp`/`Mc`
    /// split `dispatch_group` uses, for the same reason (see its own doc comment). Parent is
    /// attempted first, then every child in `children` order, matching `dispatch_group`/
    /// `dispatch_peer_group`'s own "undo parent-first then children" fixed order — though, per
    /// `undo_one`, each member is checked against `group_id` independently, so this order only
    /// affects the RETURNED report's ordering, never correctness. Undoes a member if and only if
    /// `member.tail_group_id() == Some(group_id)`; a member whose tail belongs to a different (or
    /// no) group, or whose own `undo()` call errors, is SKIPPED and recorded in the returned report
    /// rather than aborting the rest — see `GroupUndoReport`'s doc comment for why abort-all would
    /// be actively harmful here. Relation-agnostic by construction: it never consults
    /// `self.graph`/ownership at all, only each member's own `tail_group_id()`, so a `Peer`-relation
    /// group reverses as one exactly the same way an `Owned`-relation group does — no separate code
    /// path needed.
    pub async fn undo_group<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember>(parent_ref: &semio_framework_artifact_reference::ArtifactRef, parent: &mut Mp, children: &mut [(&semio_framework_artifact_reference::ArtifactRef, &mut Mc)], group_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        Self::undo_one(parent_ref, parent, group_id, &mut undone, &mut skipped, identity).await;
        for (reference, member) in children.iter_mut() {
            Self::undo_one(reference, *member, group_id, &mut undone, &mut skipped, identity).await;
        }
        GroupUndoReport { undone, skipped }
    }

    /// ↪️ `undo_group`'s redo-direction mirror, split the same `parent`/`children` way and for the
    /// same reason. Every child in `children` is attempted first, then `parent` last, matching
    /// `dispatch_group`/`dispatch_peer_group`'s apply order (so redo re-establishes the group in
    /// the same order it was originally applied) — again only ordering the returned report, since
    /// each member's own `redo_tail()` group id is checked independently. Relation-agnostic for the
    /// same reason `undo_group` is.
    pub async fn redo_group<Mp: ReferenceSpaceMember, Mc: ReferenceSpaceMember>(parent_ref: &semio_framework_artifact_reference::ArtifactRef, parent: &mut Mp, children: &mut [(&semio_framework_artifact_reference::ArtifactRef, &mut Mc)], group_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> GroupUndoReport {
        let mut undone = Vec::new();
        let mut skipped = Vec::new();
        for (reference, member) in children.iter_mut() {
            match member.redo_tail().await {
                Some((edit_id, Some(tail_group))) if tail_group == group_id => match member.redo(identity).await {
                    Ok(()) => undone.push(((*reference).clone(), edit_id)),
                    Err(error) => skipped.push(((*reference).clone(), error)),
                },
                _ => skipped.push(((*reference).clone(), VcsError::NothingToRedo)),
            }
        }
        match parent.redo_tail().await {
            Some((edit_id, Some(tail_group))) if tail_group == group_id => match parent.redo(identity).await {
                Ok(()) => undone.push((parent_ref.clone(), edit_id)),
                Err(error) => skipped.push((parent_ref.clone(), error)),
            },
            _ => skipped.push((parent_ref.clone(), VcsError::NothingToRedo)),
        }
        GroupUndoReport { undone, skipped }
    }
}
