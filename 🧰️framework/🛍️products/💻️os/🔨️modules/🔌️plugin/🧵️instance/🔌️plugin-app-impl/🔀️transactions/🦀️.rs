//! 🔀️ Transactions of the mounted artifact instance: the prepare and commit legs of a composed transaction. See [`super`].

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + Send + 'static> VcsArtifactApp<A, M> {
    /// 🔀️ Decodes EITHER wire form (contract §2/§5.3): pre-planned (`prepared_ops`
    /// non-empty) carries its own `origin`; owner-mutation (`prepared_ops` empty)
    /// decodes the single `payload` op and has no origin on the wire — see the 🚧️ note below.
    ///
    /// 🚧️ The owner-mutation form carries no `origin` on the wire (contract §2/§5.3)
    /// — this member is by construction a foreign target of someone else's
    /// transaction, so the best available provenance is `Transaction` with an
    /// initiator identity the wire does not convey. Flagged in `📓️w1-b-report.md`
    /// as a contract gap for the coordinator/host lanes to close (e.g. by always
    /// preferring the pre-planned form, which does carry `origin`).
    pub(super) async fn transaction_prepare(&mut self, txn_id: &str, _mutation_id: &str, payload: &[u8], prepared_ops: &[Vec<u8>], prepared_child_ops: &[u8], origin: Option<protocol::MutationOrigin>) -> TransactionPrepareOutcome {
        if let Some(rejection) = InstanceExtensions::transaction_refusal(&self.time_travel, &self.tool_runs, txn_id) {
            return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(rejection) };
        }
        if self.txn.pending_transaction.is_some() {
            return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.instance-busy", "a transaction is already pending on this instance")) };
        }
        let children = match ChildEmit::decode_groups(prepared_child_ops) {
            Ok(children) => children,
            Err(fault) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(fault) },
        };
        if let Some(missing) = children.iter().find(|child| !child.owner.is_empty() || self.child_composition.children.get(&(child.slot.clone(), child.child_id.clone())).is_none()) {
            return TransactionPrepareOutcome {
                foreign: Vec::new(),
                rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("owned child {:?} in slot {:?} is not held by this instance", missing.child_id, missing.slot))),
            };
        }
        let (ops, resolved_origin): (Vec<A::Mutation>, protocol::MutationOrigin) = if !prepared_ops.is_empty() || !children.is_empty() {
            let mut decoded = Vec::with_capacity(prepared_ops.len());
            for op_bytes in prepared_ops {
                match <A::Mutation as ::protocol::OpBinary>::decode_op(op_bytes) {
                    Ok(op) => decoded.push(op),
                    Err(error) => {
                        return TransactionPrepareOutcome {
                            foreign: Vec::new(),
                            rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("prepared op did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                        };
                    }
                }
            }
            (decoded, origin.unwrap_or_default())
        } else {
            match <A::Mutation as ::protocol::OpBinary>::decode_op(payload) {
                Ok(op) => (vec![op], protocol::MutationOrigin::Transaction { initiator: protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None } }),
                Err(error) => {
                    return TransactionPrepareOutcome {
                        foreign: Vec::new(),
                        rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.unknown-mutation", format!("owner-mutation payload did not decode as a {} mutation: {error:?}", A::DOCUMENT_SCHEMA))),
                    };
                }
            }
        };
        let mut running = match self.stores.store.snapshot() {
            Ok(snapshot) => snapshot,
            Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.member-rejected", format!("{error:?}"))) },
        };
        let foreign = Vec::new();
        for op in &ops {
            match op.foreign_step_source(&running, 0) { Ok(None) => {}, Ok(Some(_)) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(plugin_sdk_fault("foreign transaction output awaits its original staged preparation recipient")) }, Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(error.into_fault()) } }
            let outcome = op.diff(&running);
            if !outcome.is_applicable(protocol::MergePolicy::default()) {
                return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", format!("mutation outcome was rejected: {:?}", outcome.messages()))) };
            }
            running = match protocol::apply_diff(outcome.diff(), &running) {
                Ok(next) => next,
                Err(error) => return TransactionPrepareOutcome { foreign: Vec::new(), rejection: Some(MediaTransfers::<A>::transaction_fault(FaultOrigin::App, "transaction.member-rejected", error.to_string())) },
            };
        }
        let base_generation = self.stores.store.generation();
        self.txn.pending_transaction = Some(PendingTransaction { txn_id: txn_id.to_string(), ops, children, origin: resolved_origin, base_generation });
        TransactionPrepareOutcome { foreign, rejection: None }
    }

    /// 🔀️ Contract §5.8: reject commit when the base generation observed at prepare time no
    /// longer matches the current one — the pending state is restored (not discarded) so an
    /// explicit `TransactionRollback`/retry can still act on it.
    ///
    /// 🔀️ Contract §5.6: this member's prepared ops land as exactly ONE `Edit` — the
    /// one-edit-per-member invariant `CompositionCoordinator::undo_group`'s tail-based group
    /// undo depends on — stamped with `group_id = txn_id` and every `MutationMeta.origin`.
    pub(super) async fn transaction_commit(&mut self, txn_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, Fault> {
        let pending = match self.txn.pending_transaction.take() {
            Some(pending) if pending.txn_id == txn_id => pending,
            Some(other) => {
                let message = format!("transaction_commit: txn_id {txn_id:?} does not match the pending transaction {:?}", other.txn_id);
                self.txn.pending_transaction = Some(other);
                return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", message));
            }
            None => return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("transaction_commit: no transaction is pending (requested txn_id {txn_id:?})"))),
        };
        if pending.base_generation != self.stores.store.generation() {
            let message = format!("transaction {txn_id:?}'s base generation {} no longer matches the current generation {}", pending.base_generation, self.stores.store.generation());
            self.txn.pending_transaction = Some(pending);
            return Err(MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.generation-mismatch", message));
        }
        let PendingTransaction { txn_id, ops, children, origin, .. } = pending;
        if !children.is_empty() {
            return self.dispatch_ctx().commit_transaction_group(&txn_id, ops, children, origin, meta, identity).await;
        }
        let label = ops.first().map(|op| protocol::SemanticMutation::<A::Snapshot>::label(op));
        self.stores.store.dispatch(ArtifactCommand::Apply { mutations: ops, transaction: None }, identity).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.history.cache = None;
        self.stores.store.stamp_tail_group_id(&txn_id).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        self.stores.store.stamp_tail_origin(origin).await.map_err(|error| MediaTransfers::<A>::transaction_fault(FaultOrigin::Plugin, "transaction.commit-failed", format!("{error:?}")))?;
        let edit_id = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()).unwrap_or_default();
        self.tools.record_command(&mut self.history, &format!("transaction:{txn_id}"), ActionKind::Mutation, label, Some(edit_id.clone()), Vec::new(), None);
        Ok(edit_id)
    }
}
