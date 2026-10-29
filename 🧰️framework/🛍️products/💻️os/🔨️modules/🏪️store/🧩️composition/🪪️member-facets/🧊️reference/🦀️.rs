//! ❄️ Explicit unscheduled reference member commands are independent of original retained publication.
use super::*;
use semio_framework_os_spr::ColdMutation;

/// 📖️ A caller explicitly selects wire-value reference arithmetic and whole reference projection capabilities.
pub trait ReferenceSpaceMember:SpaceMember {
    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>>;
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>;
    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;
    async fn create_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError>;
    async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;
    async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError>;

    /// 🧪️ Decodes `ops` as a sequence of individually-`OpBinary`-encoded `Mutation`s (the
    /// SAME wire shape `ArtifactCommand::Apply.mutations` bundles — see `dispatch_wire`'s doc
    /// comment) and dry-runs each against a snapshot threaded forward through the whole slice, so
    /// op `i` previews against the state ops `0..i` would produce, never a stale base. Mirrors the
    /// real apply algorithm (`📋️contract-freeze.md` §C6 `replay_mutations`: every op is diffed and
    /// folded in order, nothing stops early — a `Fatal`/`Error` message's diff is a no-op by
    /// construction (§C2 LAWS 1/2), so threading through it is always safe) and returns EVERY
    /// message every op raised, each stamped with its `op_index`; only a structural failure (a
    /// snapshot read error, or an op that fails to even decode) short-circuits the remaining ops,
    /// itself reported as one `mutation.invariant` `Fatal` message. Never applies anything — the
    /// live store is untouched no matter what this returns. `CompositionCoordinator` phase 1 unions
    /// this across every member, computes ONE worst `semio_framework_diagnostic::Severity`, and consults
    /// `merge_policy()` to decide accept/reject for the whole group.
    ///
    /// 🎯️ B2 `CompositionCoordinator` seam (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, `🔖️CompositionCoordinator`
    /// region below `🔖️Space`): nine object-safe methods, in two groups. `preview_wire`/
    /// `dispatch_wire`/`tail_group_id` are the three the task brief named explicitly (`preview_wire`
    /// itself renamed from `validate_wire` by `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6, once its `Result<(), String>` early-reject shape was replaced by the
    /// `Vec<MutationMessage>` dry run every `CompositionCoordinator` phase 1 now unions across
    /// members). The other six (`tail_edit_id`, `redo_tail`, `stamp_tail_group_id`, `set_owner`,
    /// `merge_policy`) are necessary, deliberate additions beyond that literal list — see
    /// `📓️wave1-reports/b2-store-composition-report.md`'s "Design decisions" for why each is
    /// unavoidable given the object-safety constraint: `GroupReceipt`/`GroupUndoReport` need real
    /// edit ids (not just group membership), `dispatch_group`'s phase 2 needs a way to stamp a shared
    /// `group_id` onto a member AFTER an ordinary `Apply` already hard-codes `group_id: None`,
    /// genesis needs a way to set `ArtifactEnvelope.owner` on a freshly-created child through the
    /// same type-erased interface, and `merge_policy` is what lets phase 1 fold every member's
    /// `preview_wire` messages against one authority-chosen `semio_framework_os_spr::MergePolicy` without
    /// downcasting.
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage>;
    /// 📡️ Decodes `cmd_bytes` as one binary `ArtifactCommand<Mutation>` and dispatches it via
    /// `dispatch_binary`. `CompositionCoordinator` builds `cmd_bytes` from a member's
    /// `ChildDispatch.ops`/`parent_ops` by replicating `write_command_ops`'s byte layout directly
    /// (length-prefixed already-encoded op bytes) WITHOUT ever decoding an individual op — the
    /// reason this whole family takes/returns bytes instead of a typed `ArtifactCommand<Mutation>`
    /// is that `SpaceMember` itself must stay object-safe (no generic method can appear on a trait
    /// object), so the coordinator stays fully agnostic of every member's concrete `Mutation` type.
    async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>;
    /// ⚖️ Applies a preflighted group command under the authority-selected policy without
    /// changing this member's local policy after dispatch.
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: semio_framework_os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError>;
    /// 📦️ This member's whole event log as ONE encoded envelope payload, for a composing
    /// parent to announce on the member's behalf: a composed child owns no transport of its own.
    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError>;
    /// 📤️ Seeds this member's TAIL edit into its causal dag and hands back the encoded
    /// envelopes a replica needs to fold it — the outbound half of child-lane replication.
    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError>;
    /// 📥️ Folds an encoded envelope payload a composing parent routed to this member's lane.
    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), VcsError>;
    /// 🖋️ Stamps `group_id` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — the mechanism `CompositionCoordinator::dispatch_group`'s phase 2 uses to give every
    /// member of one composite gesture the SAME `MutationMeta.group_id` after dispatching each
    /// member's own `Apply` independently (the ordinary `Apply` path has no way to accept an
    /// externally-supplied group id — see `ArtifactStore::replay_mutations`, which always stamps
    /// `group_id: None`). Errors with `VcsError::UnknownEdit` if this member has no applied edits at
    /// all — never true on the path `dispatch_group` actually calls it from, since it always calls
    /// this immediately after a successful `dispatch_wire`.
    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError>;
    /// 🔀️ Stamps `origin` onto every `MutationMeta` entry of this member's TAIL applied
    /// edit — `stamp_tail_group_id`'s provenance-direction twin, used by
    /// `TransactionCoordinator::dispatch_group`'s `Peer` relation to mark a foreign member's edit
    /// `semio_framework_os_spr::MutationOrigin::Transaction { initiator }` after dispatching it (the
    /// initiator's OWN edit is left `Owner` — it is not foreign to itself). Never called on the
    /// `Owned` relation path, which keeps every member's origin at its ordinary `Apply`-assigned
    /// default (`Owner`) — this is precisely what keeps `Owned` byte-identical to its pre-`Peer`
    /// behaviour. Same `VcsError::UnknownEdit` failure mode as `stamp_tail_group_id` (never true on
    /// the path `dispatch_group` calls it from).
    async fn stamp_tail_origin(&mut self, origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError>;
    /// 🏠️ Sets (or clears) this member's own envelope `owner` stamp — the mechanism
    /// `CompositionCoordinator::dispatch_group`'s phase 2 uses to record a freshly-`ChildGenesis`-
    /// created child's `OwnerRef` directly on the child's own envelope (see
    /// `ArtifactEnvelope.owner`'s doc comment for why ownership must be queryable from the child
    /// side, not only from the parent's `ArtifactChild` handle). Not part of the ordinary
    /// VCS/dispatch surface — no ordinary `Apply` mutation can reach envelope metadata — so it needs
    /// its own object-safe setter.
    async fn set_owner(&mut self, owner: Option<OwnerRef>);

    /// 📦️ This member's CURRENT materialized snapshot, pack-encoded — the live content a
    /// composition parent reads through `ArtifactView.children` and a `LinkPin::Head` resolves to.
    /// Reads through the live store, so it cannot go stale behind an undo/redo/checkout the way the
    /// `thread_local!` child caches this replaces did.
    ///
    /// 📖️ Object-safe READ surface (this ticket's CW1-1b). Everything above either mutates a member
    /// or reports a scalar about it; nothing could get a member's CONTENT back out without
    /// downcasting through `as_any_mut` to a concrete `ArtifactStore<P, Mutation>` the caller must
    /// already know the types of. That is exactly what a composition parent cannot do (its children
    /// are heterogeneous and plugin-defined), so `ArtifactView.children` and `LinkResolver` both
    /// dead-ended here. All three return PACK bytes rather than a typed value for the same reason
    /// `dispatch_wire` takes bytes: no generic method can appear on a trait object.
    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// 🗄️ This member's WHOLE envelope (initial snapshot pack + `.spr` op log, in
    /// `encode_document_pack_bytes` framing) — what gets persisted for a child and handed back to
    /// `ChildStoreFactory::open` on reload. The full history, not just the current content.
    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError>;
    /// 🪼️ Merges the event log of `envelope_pack` — another writer's persisted copy of THIS member — into it
    /// ([`ArtifactStore::merge_persisted_pair`]): the member keeps its head and its own edits. The default refuses: a member
    /// without an event log of its own has nothing to merge, and its owner falls back to a load.
    async fn merge_persisted_envelope(&mut self, _envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {
        Err(VcsError::ValidationFailed("this member holds no event log a persisted envelope merges into".into()))
    }
    /// ⏮️📦️ This member's snapshot AS OF `checkpoint_id`, pack-encoded, without disturbing the
    /// live cursor — replays exactly the edit ids that checkpoint's changes cover, the same set
    /// `checkout_checkpoint_internal` would install. This is what makes `LinkPin::Checkpoint` real:
    /// a pinned reference resolves to the target's historical content rather than silently
    /// degrading to its head.
    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError>;
}

impl<P, Mutation> ReferenceSpaceMember for ArtifactStore<P, Mutation>
where
 P: semio_framework_os_store_text_io::ArtifactStoreSnapshot + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + semio_framework_os_spr::StoreState,
 Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{

    fn begin_one_item_wire_publication(&self, request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        let Some(factory) = self.owners.one_item_wire_preparation_factory.as_ref().filter(|_| self.one_item_wire_publication_supported()) else {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member requires its exact retained wire preparation and retirement authority".into(), mutations: vec![request.wire] });
        };
        if request.wire.schema.len() > ARTIFACT_STORE_ONE_ITEM_ID_BYTES || request.wire.bytes.len() > ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES {
            return Err(ArtifactStoreBatchAdmissionRejected { reason: "member wire exceeds its fixed schema or byte admission".into(), mutations: vec![request.wire] });
        }
        let source = ArtifactStoreBatchSourceOf::new(Arc::clone(factory), vec![request.wire], None, self.log.operation_wire);
        let publication = self.begin_apply_batch_owned(request.operation, request.expected_generation, request.expected_revision, request.actor, HistoryLane::Document, request.group_id, source, false, None)?;
        Ok(Box::new(MemberStoreOneItemPublication { member: Some(SnapshotReadRegistryAliasRetirement::new(self.owners.snapshot_read_leases.clone())), publication, group_history: None, group_displaced: None }))
    }

    /// 🏁️ `self.current_checkpoint_id()` resolves to the inherent method (`Option<&str>`), not this
    /// trait method — Rust prefers inherent methods over trait methods of the same name.
    async fn commit_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CommitCheckpoint { message: Some(message), authors }, identity).await?;
        self.current_checkpoint_id().map(|id| id.to_string()).ok_or(VcsError::NoCheckpoint)
    }

    async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        if !alternative_id.is_empty() {
            let is_alternative_tip = self.envelope().vcs.alternatives.iter().find(|alternative| alternative.id == alternative_id).is_some_and(|alternative| alternative.checkpoint_ids.last().map(String::as_str) == Some(checkpoint_id));
            if is_alternative_tip {
                return self.dispatch(ArtifactCommand::SwitchAlternative { alternative_id: alternative_id.to_string() }, identity).await.map(|_| ());
            }
        }
        self.dispatch(ArtifactCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }, identity).await.map(|_| ())
    }

    async fn create_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        self.dispatch(ArtifactCommand::CreateAlternative { name }, identity).await?;
        self.envelope().active_alternative_id.clone().ok_or(VcsError::NoCheckpoint)
    }

    async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Undo, identity).await.map(|_| ())
    }

    async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        self.dispatch(ArtifactCommand::Redo, identity).await.map(|_| ())
    }

    /// 🌀️ `MutationMessage::at_op` is async (📡️replication); `Iterator::map`'s closure is
    /// sync (R10 shape 1), so it's hoisted into an explicit loop.
    async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage> {
        let mut running = match self.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return vec![semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.to_string())],
        };
        let mut all_messages = Vec::new();
        for (index, op) in ops.iter().enumerate() {
            let mutation = match <Mutation as OpBinary>::decode_op(op) {
                Ok(mutation) => mutation,
                Err(error) => {
                    all_messages.push(semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.to_string()).at_op(index as u32));
                    break;
                }
            };
            let (next, messages) = match apply_mutation(&running, &mutation) {
                Ok(applied) => applied,
                Err(error) => {
                    all_messages.push(semio_framework_os_spr::MutationMessage::fatal("mutation.invariant", error.message).at(error.target).at_op(index as u32));
                    break;
                }
            };
            for message in messages {
                all_messages.push(message.at_op(index as u32));
            }
            retire_replayed_projection::<P, Mutation>(std::mem::replace(&mut running, next));
        }
        all_messages
    }

    async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        self.dispatch_binary(cmd_bytes, identity).await
    }

    /// 🌀️ The unawaited future holds `&mut self`, so `self.log.merge_policy` cannot be reassigned
    /// while it's still live (E0506) — awaited immediately instead of deferred, matching the
    /// original synchronous "restore after dispatch completes" intent.
    async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: semio_framework_os_spr::MergePolicy, identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        let local_policy = self.log.merge_policy;
        self.log.merge_policy = policy;
        let result = self.dispatch_binary(cmd_bytes, identity).await;
        self.log.merge_policy = local_policy;
        result
    }

    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        ArtifactStore::event_log_payload(self)
    }

    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        ArtifactStore::announce_tail_edit_payload(self)
    }

    async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), VcsError> {
        ArtifactStore::ingest_remote_payload(self, envelopes).await.map(|_| ())
    }

    async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.group_id = Some(group_id.to_string());
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn stamp_tail_origin(&mut self, origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError> {
        let edit_id = self.applied_edit_ids().last().cloned().ok_or(VcsError::NothingToUndo)?;
        let edit = self.log.envelope.vcs.edits.iter_mut().find(|edit| edit.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.clone()))?;
        for meta in edit.mutation_meta.iter_mut() {
            meta.origin = origin.clone();
        }
        self.restamp_applied_tail();
        Ok(())
    }

    async fn set_owner(&mut self, owner: Option<OwnerRef>) {
        self.log.envelope.owner = owner;
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        Ok(self.snapshot()?.encode_pack())
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        let files = print_document_pack(self.envelope()).await?;
        Ok(encode_document_pack_bytes(&files.pack, &files.spr).await)
    }

    async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<PairMerge, VcsError> {
        let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;
        self.merge_persisted_pair(&pack, &spr).await
    }

    async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        let envelope = self.envelope();
        let checkpoint = envelope.vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or_else(|| VcsError::UnknownChange(checkpoint_id.to_string()))?;
        let edit_ids = edit_ids_for_changes(envelope, &checkpoint.change_ids).await;
        Ok(materialize_document_snapshot(envelope, &edit_ids).await?.encode_pack())
    }
}

impl ReferenceSpaceMember for NoMembers {

    fn begin_one_item_wire_publication(&self, _request: MemberStoreOneItemWireRequest) -> Result<Box<dyn ErasedMemberStoreOneItemPublication>, ArtifactStoreBatchAdmissionRejected<MemberStoreOneItemWire>> {
        match *self {}
    }

    async fn commit_checkpoint(&mut self, _message: String, _authors: Vec<Author>, _identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        match *self {}
    }

    async fn checkout(&mut self, _checkpoint_id: &str, _alternative_id: &str, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn create_alternative(&mut self, _name: String, _identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        match *self {}
    }

    async fn undo(&mut self, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn redo(&mut self, _identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        match *self {}
    }

    async fn preview_wire(&self, _ops: &[Vec<u8>]) -> Vec<semio_framework_os_spr::MutationMessage> {
        match *self {}
    }

    async fn dispatch_wire(&mut self, _cmd_bytes: &[u8], _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn dispatch_wire_with_policy(&mut self, _cmd_bytes: &[u8], _policy: semio_framework_os_spr::MergePolicy, _identity: &mut EntityIdentityAuthority<'_>) -> Result<CommandReceipt, VcsError> {
        match *self {}
    }

    async fn event_log_payload(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn ingest_remote_payload(&mut self, _envelopes: &[u8]) -> Result<(), VcsError> {
        match *self {}
    }

    async fn stamp_tail_group_id(&mut self, _group_id: &str) -> Result<(), VcsError> {
        match *self {}
    }

    async fn stamp_tail_origin(&mut self, _origin: semio_framework_os_spr::MutationOrigin) -> Result<(), VcsError> {
        match *self {}
    }

    async fn set_owner(&mut self, _owner: Option<OwnerRef>) {
        match *self {}
    }

    async fn document_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }

    async fn pack_at_checkpoint(&self, _checkpoint_id: &str) -> Result<Vec<u8>, VcsError> {
        match *self {}
    }
}

/// 🪪️ Declares the separately selected reference facet only for explicitly cold member enums.
#[macro_export]
macro_rules! reference_space_members {
 ($enum_name:ident {$($variant:ident),+ $(,)?}) => {
  impl $crate::os_store::ReferenceSpaceMember for $enum_name {
            fn begin_one_item_wire_publication(&self, request: $crate::os_store::one_item_publication::MemberStoreOneItemWireRequest) -> Result<Box<dyn $crate::os_store::one_item_publication::ErasedMemberStoreOneItemPublication>, $crate::os_store::one_item_publication::ArtifactStoreBatchAdmissionRejected<$crate::os_store::one_item_publication::MemberStoreOneItemWire>> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::begin_one_item_wire_publication(m.as_ref(), request)),+ }
            }
            async fn commit_checkpoint(&mut self, message: String, authors: Vec<$crate::os_store::Author>, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::commit_checkpoint(m.as_mut(), message, authors, identity).await),+ }
            }
            async fn checkout(&mut self, checkpoint_id: &str, alternative_id: &str, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::checkout(m.as_mut(), checkpoint_id, alternative_id, identity).await),+ }
            }
            async fn create_alternative(&mut self, name: String, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::create_alternative(m.as_mut(), name, identity).await),+ }
            }
            async fn undo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::undo(m.as_mut(), identity).await),+ }
            }
            async fn redo(&mut self, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::redo(m.as_mut(), identity).await),+ }
            }
            async fn preview_wire(&self, ops: &[Vec<u8>]) -> Vec<$semio_framework_os_spr::MutationMessage> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::preview_wire(m.as_ref(), ops).await),+ }
            }
            async fn dispatch_wire(&mut self, cmd_bytes: &[u8], identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::dispatch_wire(m.as_mut(), cmd_bytes, identity).await),+ }
            }
            async fn dispatch_wire_with_policy(&mut self, cmd_bytes: &[u8], policy: $semio_framework_os_spr::MergePolicy, identity: &mut $crate::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<$crate::os_store::CommandReceipt, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::dispatch_wire_with_policy(m.as_mut(), cmd_bytes, policy, identity).await),+ }
            }
            async fn event_log_payload(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::event_log_payload(m.as_ref()).await),+ }
            }
            async fn announce_tail_edit_payload(&mut self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::announce_tail_edit_payload(m.as_mut()).await),+ }
            }
            async fn ingest_remote_payload(&mut self, envelopes: &[u8]) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::ingest_remote_payload(m.as_mut(), envelopes).await),+ }
            }
            async fn stamp_tail_group_id(&mut self, group_id: &str) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::stamp_tail_group_id(m.as_mut(), group_id).await),+ }
            }
            async fn stamp_tail_origin(&mut self, origin: $semio_framework_os_spr::MutationOrigin) -> Result<(), $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::stamp_tail_origin(m.as_mut(), origin).await),+ }
            }
            async fn set_owner(&mut self, owner: Option<$crate::os_store::OwnerRef>) {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::set_owner(m.as_mut(), owner).await),+ }
            }
            async fn document_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::document_pack_bytes(m.as_ref()).await),+ }
            }
            async fn envelope_pack_bytes(&self) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::envelope_pack_bytes(m.as_ref()).await),+ }
            }
            async fn merge_persisted_envelope(&mut self, envelope_pack: &[u8]) -> Result<$crate::os_store::replay::PairMerge, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::merge_persisted_envelope(m.as_mut(), envelope_pack).await),+ }
            }
            async fn pack_at_checkpoint(&self, checkpoint_id: &str) -> Result<Vec<u8>, $crate::os_store::VcsError> {
                match self { $(Self::$variant(m) => $crate::os_store::ReferenceSpaceMember::pack_at_checkpoint(m.as_ref(), checkpoint_id).await),+ }
            }
  }
 };
}
